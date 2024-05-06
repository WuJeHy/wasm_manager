/*
 * error.rs, 2024-05-06 15:13 power by wujehy
 */
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrekWasmError {
    #[error("I/O error {0}")]
    IoError(#[from] io::Error),
    #[error("trek iot unknown err fail")]
    UnknownErrFail,
    #[error("create wasm env fail")]
    CreateWasmEnvFail,
    #[error("create wasm env ins fail")]
    CreateWasmEnvInsFail,
    #[error("get wasm env store fail")]
    GetWasmEnvStoreFail,
    #[error("error wasm export error {0}")]
    FromWasmExportFuncErr(#[from] wasmer::ExportError),
    #[error("error wasm RuntimeError error {0}")]
    FromWasmRuntimeErr(#[from] wasmer::RuntimeError),
    #[error("error wasm CompileError error {0}")]
    FromWasmCompileError(#[from] wasmer::CompileError),
    #[error("error wasm InstantiationError error {0}")]
    FromWasmInstantiationError(#[from] wasmer::InstantiationError),
    #[error("error wasm MemoryAccessError error {0}")]
    FromWasmMemoryAccessError(#[from] wasmer::MemoryAccessError),
    #[error("get wasm env fail")]
    GetWasmEnvFail,
    #[error("get wasm env ins fail")]
    GetWasmEnvInsFail,
    #[error("get wasm env lock fail")]
    GetWasmEnvLockFail,
}