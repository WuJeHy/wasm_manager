/*
 * app.rs, 2024-05-07 21:12 power by wujehy
 */

// 这里实现的是应用层的数据

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use tracing::info;

use crate::error::TrekWasmError;

// 用户环境数据
#[derive(Default)]
pub struct UserData {
    counter: i32,
}

// 用于存储用户的 环境数据
#[derive(Clone)]
pub struct AppStore {
    data: HashMap<u64, Arc<Mutex<UserData>>>,
}

static INSTANCE: OnceLock<Arc<Mutex<AppStore>>> = OnceLock::new();

pub fn global() -> Arc<Mutex<AppStore>> {
    let ins = INSTANCE.get_or_init(|| {
        let data = AppStore {
            // data: Arc::new(RwLock::new(HashMap::with_capacity(64)))
            data: HashMap::with_capacity(64)
        };
        Arc::new(Mutex::new(data))
    }).clone();
    ins
}


impl AppStore {
    pub fn init(&mut self, id: &u64) -> anyhow::Result<(), TrekWasmError> {

        if self.data.contains_key(id) {
            info!("初始化过了");
            return Ok(());
        }

        self.data.insert(id.clone(), Arc::new(Mutex::new(Default::default()))).
            ok_or(TrekWasmError::UnknownErrFail)?;

        Ok(())
    }
    pub fn read_data<RETURN, FT>
    (&self, id: &u64, callback: FT)
     -> Result<RETURN, TrekWasmError>
        where FT: Fn(&UserData) -> Result<RETURN, TrekWasmError> {
        // NOTE 这里 用 rust 的骚操作 写一个链式

        // 普通版本 这个可读性性比较强
        // let get_lock = self.data.get(&id).ok_or(TrekWasmError::UnknownErrFail)?;
        // let get_data = get_lock.lock().ok().ok_or(TrekWasmError::UnknownErrFail)?;
        // callback(&get_data)

        // 回调的写法
        // self.data.get(&id).map(|data| {
        //     let userdata = data.lock().ok().ok_or(TrekWasmError::UnknownErrFail)?;
        //     callback(&userdata)
        // }).ok_or(TrekWasmError::UnknownErrFail)?

        // 链式
        self.data
            .get(&id)
            .ok_or(TrekWasmError::UnknownErrFail)?
            .lock()
            .ok()
            .ok_or(TrekWasmError::UnknownErrFail)
            .map(|d| callback(&d))?

        // 这三个的效果是一样的
        // 比 go 的 err != nil 舒服多的原因
    }
    pub fn write_data<RETURN, FT>
    (&mut self, id: &u64, callback: FT)
     -> Result<RETURN, TrekWasmError>
        where FT: Fn(&mut UserData) -> Result<RETURN, TrekWasmError> {
        self.data
            .get(&id)
            .ok_or(TrekWasmError::UnknownErrFail)?
            .clone()
            .lock()
            .ok()
            .ok_or(TrekWasmError::UnknownErrFail)
            .map(|mut d| callback(&mut d))?
    }
}

impl UserData {
    pub fn add_one(&mut self) {
        self.counter += 1;
    }

    pub fn get(&self) -> i32 {
        self.counter
    }
}

#[cfg(test)]
mod test {
    use crate::app::global;
    use crate::error::TrekWasmError;

    #[test]
    fn test_global() -> anyhow::Result<(), TrekWasmError> {
        let binding = global();
        let mut binding = binding.lock();
        let g = binding.as_mut().unwrap();
        let id = 1;
        let ret = g.read_data(&id, |data| {
            let ret = data.get();
            Ok(ret)
        });

        assert_eq!(ret.is_err(), true);

        let _ = g.init(&id);
        let _ = g.write_data(&id, |data| {
            data.add_one();
            data.add_one();
            data.add_one();
            data.add_one();
            Ok(())
        })?;
        let ret = g.read_data(&id, |data| {
            let ret = data.get();
            Ok(ret)
        })?;
        assert_eq!(ret, 4);
        Ok(())
    }
}