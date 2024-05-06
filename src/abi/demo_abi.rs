/*
 * demo_abi.rs, 2024-05-06 15:51 power by wujehy
 */


use tracing::info;
use wasmer::{AsStoreMut, Exports, Function, FunctionEnv, FunctionEnvMut, Imports, WasmPtr};

use crate::error::TrekWasmError;
use crate::runtime::TrekAbiEnv;

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

pub fn register(store: &mut impl AsStoreMut, ctx: &FunctionEnv<TrekAbiEnv<()>>, imp: &mut Imports) {
    let mut export = Exports::new();

    export.insert("print", Function::new_typed_with_env(store, &ctx, print));
    export.insert("exit", Function::new_typed_with_env(store, &ctx, exit));

    imp.register_namespace("trek", export);
}