/*
 * test_manager_app_1_and_2.rs, 2024-05-08 15:53 power by wujehy
 */



use std::time::Duration;
use wasmer::TypedFunction;

use wasm_manager::error::TrekWasmError;
use wasm_manager::manger::WasmManager;

#[tokio::main]
async fn main() -> anyhow::Result<(), TrekWasmError> {
    tracing_subscriber::fmt()
        .without_time()
        .init();

    let mut manager = WasmManager::new(4);

    let file = "app1.wasm";
    let fil2 = "app2.wasm";

    let id = 1;
    let _ret = manager.new_env(id)?;

    let _ret = manager.get_env_mut(1, |env| async {
        env.load_wasm(file)?;
        env.call_init().await?;
        Ok(())
    }).await?;

    let _ret = manager.get_env(1, |env| async {
        let ins_get = env.get_ins()?;
        let ret= env.call_store_with_instance(&ins_get, |_env, ins, store|  {
            //   do something
            let call_func:TypedFunction<(),()>= ins.exports.get_typed_function(store , "test_api")?;
            let _ret = call_func.call(store)?;

            Ok(())
        });
        ret
    }).await;

    let id2= 2;
    let _ret = manager.new_env(id2)?;

    let _ret = manager.get_env_mut(2, |env| async {
        env.load_wasm(fil2)?;
        env.call_init().await?;
        Ok(())
    }).await?;

    tokio::time::sleep(Duration::from_secs(3)).await;


    Ok(())
}