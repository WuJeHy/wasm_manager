/*
 * demo_abi.rs, 2024-05-06 15:51 power by wujehy
 */


use tracing::info;
use wasmer::{AsStoreMut, Exports, Function, FunctionEnv, FunctionEnvMut, Imports, WasmPtr};

use crate::app::global;
use crate::error::TrekWasmError;
use crate::runtime::TrekAbiEnv;
use crate::tools::get_memory_view;

pub fn print(
    mut ctx: FunctionEnvMut<'_, TrekAbiEnv<()>>,
    ptr: WasmPtr<u8>,
    len: u32,
) -> Result<(), TrekWasmError> {
    let (get_trek_env, mut store) = ctx.data_and_store_mut();

    let memory_view = get_trek_env.ins
        .lock().ok()
        .ok_or(TrekWasmError::CreateWasmEnvFail)?
        .clone()
        .ok_or(TrekWasmError::UnknownErrFail)?
        .exports
        .get_memory("memory")?
        .view(&mut store);

    let get_str = ptr.read_utf8_string(&memory_view, len)?;

    info!("call print:{get_str} [{len}]" );
    Ok(())
}

pub fn exit(
    _ctx: FunctionEnvMut<'_, TrekAbiEnv<()>>,
    code: i32,
) {
    info!("inner exit {code}");
}

pub fn env_add_one(mut ctx: FunctionEnvMut<'_, TrekAbiEnv<()>>) -> Result<i32, TrekWasmError> {
    let (get_trek_env, _store) = ctx.data_and_store_mut();
    let my_id = match get_trek_env.get_id() {
        Ok(id) => { id }
        Err(err) => {
            info!("user not get id {err}");
            return Ok(-1);
        }
    };


    let ret = global().lock().as_mut().map(|store| {
        store.write_data(&my_id, |data| {
            data.add_one();
            Ok(0)
        })
    }).map_err(|_err| {
        TrekWasmError::UnknownErrFail
    })?;

    ret
}

pub fn env_get_val(mut ctx: FunctionEnvMut<'_, TrekAbiEnv<()>>) -> Result<i32, TrekWasmError> {
    let (get_trek_env, _store) = ctx.data_and_store_mut();
    let my_id = get_trek_env.get_id()?;

   let ret = global().lock().map(|app| {
        app.read_data(&my_id, |data| {
            Ok(data.get())
        })
    }).map_err(|_err|{
       TrekWasmError::UnknownErrFail
   })?;
    ret
}

pub fn get_env_id(mut ctx: FunctionEnvMut<'_, TrekAbiEnv<()>>,
) -> Result<u64, TrekWasmError> {
    let (get_trek_env, _store) = ctx.data_and_store_mut();
    let ret = get_trek_env.get_id();
    let id = match ret {
        Ok(id) => { id }
        Err(_err) => {
            0
        }
    };
    Ok(id)
}

pub fn env_register(
    mut ctx: FunctionEnvMut<'_, TrekAbiEnv<()>>,
    id: u64,
    ptr: WasmPtr<u8>,
    len: u32,
) -> Result<i32, TrekWasmError> {
    let (get_trek_env, mut store) = ctx.data_and_store_mut();

    let memory_view = get_memory_view(get_trek_env, &mut store)?;

    let token_str = ptr.read_utf8_string(&memory_view, len)?;

    if token_str.ne("user_token") {
        return Ok(-1);
    }

    // 注册
    if get_trek_env.update_id(&id).is_ok() {
        let _ = global().lock().as_mut().map(|app|{
            _ = app.init(&id);
        });

        return Ok(0);
    }
    Ok(-2)
}


pub fn register(store: &mut impl AsStoreMut, ctx: &FunctionEnv<TrekAbiEnv<()>>, imp: &mut Imports) {
    let mut export = Exports::new();

    export.insert("print", Function::new_typed_with_env(store, &ctx, print));
    export.insert("exit", Function::new_typed_with_env(store, &ctx, exit));
    export.insert("get_env_id", Function::new_typed_with_env(store, &ctx, get_env_id));
    export.insert("register", Function::new_typed_with_env(store, &ctx, env_register));
    export.insert("add_one", Function::new_typed_with_env(store, &ctx, env_add_one));
    export.insert("get_val", Function::new_typed_with_env(store, &ctx, env_get_val));

    imp.register_namespace("trek", export);
}