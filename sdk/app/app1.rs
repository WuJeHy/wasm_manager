/*
 * app1.rs, 2024-05-08 13:58 power by wujehy
 */
#![no_main]


use sdk::{log_console, SystemApi};

#[no_mangle]
fn start() {
    log_console("start app 1");

    let mut system_api = SystemApi::new();

    system_api.register("user_token", 1);

    system_api.add_one();

    let val = system_api.get_val();

    let show = format!("data {val}");

    log_console(show.as_str());

    SystemApi::exit(0)
}

#[no_mangle]
fn test_api() {
    let system_api = SystemApi::new();
    system_api.add_one();
    system_api.add_one();
}