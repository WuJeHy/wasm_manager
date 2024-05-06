/*
 * tools.rs, 2024-05-06 15:33 power by wujehy
 */

// 用于计时的函数

use std::borrow::Borrow;

use tracing::debug;
use wasmer::{AsStoreMut, Memory, MemoryView};
use crate::error::TrekWasmError;
use crate::runtime::TrekAbiEnv;

fn trace_call_time<CTX, RETURN, ERR>(ctx: CTX, callback: impl Fn(CTX) -> anyhow::Result<RETURN, ERR>)
                                         -> anyhow::Result<RETURN, ERR> {
    let time_start = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let ret = callback(ctx);
    let time_end = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    debug!("test_init_time {time_start} {time_end} {}" , time_end - time_start);
    ret
}

pub fn get_memory_view<'a, T>(get_trek_env: &'a TrekAbiEnv<T> , store : &'a mut impl AsStoreMut) -> Result<MemoryView<'a >, TrekWasmError> {
    let memory_view = get_trek_env.ins
        .lock().ok()
        .ok_or(TrekWasmError::CreateWasmEnvFail)?
        .clone()
        .ok_or(TrekWasmError::UnknownErrFail)?
        .exports
        .get_memory("memory")?
        .view(store);
    Ok(memory_view)
}