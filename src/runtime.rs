/*
 * runtime.rs, 2024-05-06 15:15 power by wujehy
 */

use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::UNIX_EPOCH;
use tracing::{debug, info};

use wasmer::{AsStoreMut, Function, FunctionEnv, Imports, imports, Instance, Module, Store, TypedFunction};
use crate::abi;
use crate::abi::demo_abi;
use crate::error::TrekWasmError;

#[derive(Clone , Default)]
pub struct TrekAbiEnv<T> {
    pub ins: Arc<Mutex<Option<Instance>>>,
    // 这个是后期用来储存 上下文的 具体这里并没有使用
    ctx: T,
}

impl<T> TrekAbiEnv<T> {
    pub fn new(t: T) -> Self {
        TrekAbiEnv {
            ctx: t,
            ins: Arc::new(Mutex::new(None)),
        }
    }
}


// 注意 为了解决 store 的问题不得不引入 unsafe 为了确保数据安全与内存问题 需要自行托管 TrekFunctionEnv 的生命周期
#[derive(Clone)]
pub struct TrekFunctionEnv {
    pub env: FunctionEnv<TrekAbiEnv<()>>,
    ins: Arc<Mutex<Option<Instance>>>,
    store_raw: Arc<*mut Store>,
}

impl Drop for TrekFunctionEnv {
    fn drop(&mut self) {
        let arc_count = Arc::strong_count(&self.store_raw);

        if arc_count == 1 {
            let _from_box = unsafe { Box::from_raw(*self.store_raw) };
            info!("drop TrekFunctionEnv store_raw ");
        }
    }
}

impl TrekFunctionEnv {
    pub fn new(store: Store, env: TrekAbiEnv<()>) -> Self {
        let ins_point = env.ins.clone();

        let store_box = Box::new(store);
        let store_raw = Box::into_raw(store_box);

        let from_raw_box = unsafe { store_raw.as_mut() }.unwrap();

        Self {
            env: FunctionEnv::new(from_raw_box, env),
            ins: ins_point,
            store_raw: Arc::new(store_raw),
        }
    }

    pub fn load_wasm(&mut self, file: &str) -> anyhow::Result<(), TrekWasmError> {
        self.call_store(|_ctx, mut store| {
            let wasm_bytes = std::fs::read(file)?;
            let module = Module::new(&store, wasm_bytes)?;

            let wasi_import = self.get_imports(&mut store)?;
            let instance = Instance::new(&mut store, &module, &wasi_import)?;
            self.update_ins(instance);
            Ok(())
        })
    }
    pub fn new_with_store(env: TrekAbiEnv<()>) -> Self {
        let store = Store::default();
        let store_box = Box::new(store);
        let store_raw = Box::into_raw(store_box);
        let ins_point = env.ins.clone();
        let from_raw_box = unsafe { store_raw.as_mut() }.unwrap();

        Self {
            env: FunctionEnv::new(from_raw_box, env),
            ins: ins_point,
            store_raw: Arc::new(store_raw),
        }
    }

    pub fn get_ins(&self) -> Result<Instance, TrekWasmError> {
        self.ins.lock()
            .ok()
            .ok_or(TrekWasmError::GetWasmEnvLockFail)?
            .clone()
            .ok_or(TrekWasmError::GetWasmEnvInsFail)
    }

    pub fn call_store_with_instance<RETURN>(&self, ins: &Instance, callback: impl Fn(&Self, &Instance, &mut Store)
        -> anyhow::Result<RETURN, TrekWasmError>)
                                            -> anyhow::Result<RETURN, TrekWasmError> {
        let store = unsafe { self.store_raw.as_mut() };
        let store = store
            .ok_or(TrekWasmError::GetWasmEnvStoreFail)?;
        callback(self, ins, store)
    }
    pub fn call_store<RETURN>(&self, callback: impl Fn(&Self, &mut Store)
        -> anyhow::Result<RETURN, TrekWasmError>)
                              -> anyhow::Result<RETURN, TrekWasmError> {
        let store = unsafe { self.store_raw.as_mut() };
        store.ok_or(TrekWasmError::GetWasmEnvStoreFail)
            .map(|store| {
                callback(self, store)
            })?
    }

    pub async fn call_store_async<'a, RETURN, F, Fut>(&'a self, callback: F)
                                                      -> anyhow::Result<RETURN, TrekWasmError>
        where F: FnOnce(&'a Self, &'a mut Store) -> Fut,
              Fut: Future<Output=anyhow::Result<RETURN, TrekWasmError>> + 'a,
              RETURN: Send

    {
        let self_ctx: &'a TrekFunctionEnv = &self;
        let store = unsafe { self.store_raw.as_mut() };

        let store = store
            .ok_or(TrekWasmError::CreateWasmEnvFail)?;
        callback(self_ctx, store).await
    }

    pub async fn call_store_async_with_instance<'a, RETURN, F, Fut>(&'a self, ins: &'a Instance, callback: F)
                                                                    -> anyhow::Result<RETURN, TrekWasmError>
        where F: FnOnce(&'a Self, &'a Instance, &'a mut Store) -> Fut,
              Fut: Future<Output=anyhow::Result<RETURN, TrekWasmError>> + 'a,
              RETURN: Send

    {
        let self_ctx: &'a TrekFunctionEnv = &self;
        let store = unsafe { self.store_raw.as_mut() };
        let store = store
            .ok_or(TrekWasmError::GetWasmEnvStoreFail)?;
        callback(self_ctx, ins, store).await
    }
    pub async fn call_init(&mut self) -> anyhow::Result<(), TrekWasmError> {
        let get_ins = self.get_ins()?;

        self.call_store(move |_ctx, store| {
            let get_func: TypedFunction<(), ()> = get_ins.exports.get_typed_function(store, "start")?;
            let time_start = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
            get_func.call(store)?;
            let time_end = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
            debug!("test_init_time {time_start} {time_end} {}" , time_end - time_start);
            Ok(())
        })
    }




    pub async fn test_call_api<ARGS, RETURN>(&self, instance: &Instance, store: &mut impl AsStoreMut, api_path: &str, args: ARGS)
                                        -> anyhow::Result<RETURN, TrekWasmError>
        where ARGS: wasmer::FromToNativeWasmType,
              RETURN: wasmer::FromToNativeWasmType
    {
        let get_func: TypedFunction<ARGS, RETURN> = instance.exports.get_typed_function(store, api_path)?;
        let time_start = std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let result = get_func.call(store, args)?;
        let time_end = std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        debug!("test_call_time {time_start} {time_end} {}" , time_end - time_start);
        Ok(result)
    }

    pub fn update_ins(&self, instance: Instance) {
        let ins_point = self.ins.clone();
        *ins_point.lock().unwrap() = Some(instance);
    }

    pub fn get_imports(&self, store: &mut impl AsStoreMut) -> anyhow::Result<Imports, TrekWasmError> {
        let self_ctx = &self.env;

        let mut wasi_import = imports! {};


        demo_abi::register(store, self_ctx, &mut wasi_import);

        Ok(wasi_import)
    }
}


