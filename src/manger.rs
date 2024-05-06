/*
 * manger.rs, 2024-05-06 15:15 power by wujehy
 */

use std::collections::HashMap;
use std::future::Future;

use tracing::info;

use crate::error::TrekWasmError;
use crate::runtime::{TrekAbiEnv, TrekFunctionEnv};

// 管理 实例化的函数
pub struct WasmManager {
    instances: HashMap<usize, TrekFunctionEnv>,
}

impl WasmManager {
    pub fn new(pool_size: usize) -> Self {
        WasmManager {
            instances: HashMap::with_capacity(pool_size)
        }
    }

    pub fn new_env(&mut self, id: usize) -> anyhow::Result<(), TrekWasmError> {
        if self.instances.contains_key(&id) {
            return Err(TrekWasmError::CreateWasmEnvFail);
        }
        let trek_ctx = TrekAbiEnv::<()>::new(());
        let new_env = TrekFunctionEnv::new_with_store(trek_ctx);
        let _ret = self.instances.insert(id, new_env);
        Ok(())
    }
    pub fn del_env(&mut self, id: usize) -> anyhow::Result<(), TrekWasmError> {
        let ret = self.instances.remove(&id);
        info!("remove {:?}",ret.is_some());
        Ok(())
    }
    pub async fn get_env<'a, RETURN, F, Fut>(&'a self, id: usize, callback: F) -> anyhow::Result<RETURN, TrekWasmError>
        where F: FnOnce(&'a TrekFunctionEnv) -> Fut,
              Fut: Future<Output=anyhow::Result<RETURN, TrekWasmError>>
    {
        let env = self.instances.get(&id).ok_or(TrekWasmError::GetWasmEnvFail)?;
        let ret = callback(env).await;
        ret
    }
    pub async fn get_env_mut<'a, RETURN, F, Fut>(&'a mut self, id: usize, callback: F) -> anyhow::Result<RETURN, TrekWasmError>
        where F: FnOnce(&'a mut TrekFunctionEnv) -> Fut,
              Fut: Future<Output=anyhow::Result<RETURN, TrekWasmError>>
    {
        let get_target = self.instances.get_mut(&id);
        return match get_target {
            None => {
                Err(TrekWasmError::GetWasmEnvFail)
            }
            Some(env_clone) => {
                let ret = callback(env_clone).await;
                ret
            }
        };
    }
}