/*
 * app1.rs, 2024-05-08 13:58 power by wujehy
 */

#![no_main]


use sdk::{log_console, SystemApi};

#[no_mangle]
fn start() {
    log_console("start app 2");
    let mut system_api = SystemApi::new();

    system_api.register("user_token" , 1 );

    // 注意这里直接读取 系统环境的数据
    let val = system_api.get_val();

    let show = format!("data {val}");

    log_console(show.as_str());

    SystemApi::exit(0)

}

