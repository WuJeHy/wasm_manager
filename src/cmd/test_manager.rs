/*
 * test_manager.rs, 2024-05-06 15:44 power by wujehy
 */


use std::time::Duration;

use wasm_manager::error::TrekWasmError;
use wasm_manager::manger::WasmManager;

#[tokio::main]
async fn main() -> anyhow::Result<(), TrekWasmError> {

    tracing_subscriber::fmt()
        .without_time()
        .init();
    let mut manager = WasmManager::new(10);

    let file = "./demo.wasm";

    let id = 1;
    let _ret = manager.new_env(id)?;


    let _ret = manager.get_env_mut(1, |env| async {
        env.load_wasm(file)?;
        env.call_init().await?;
        Ok(())
    }).await?;

    let _ret = manager.get_env(1, |env| async {
        let ins_get = env.get_ins()?;
        let arg = 11;

        let ret = env.call_store_async_with_instance(&ins_get, |env, ins, store| async {
            let ret = env.test_call_api::<i32, i32>(ins, store, "add", arg).await;
            ret
        }).await;
        ret
    }).await?;

    tokio::time::sleep(Duration::from_secs(3)).await;

    Ok(())
}