/*
 * demo_plugin.rs, 2024-05-06 15:45 power by wujehy
 */


#![no_main]

#[link(wasm_import_module = "trek")]
extern "C" {
    fn print(ptr: *const u8, len: usize);
    fn exit(code: usize);
}

static HELLO: &'static str = "Trek World! ";

#[no_mangle]
fn start() {
    unsafe {
        print(HELLO.as_ptr(), HELLO.len());
        exit(0);
    }
}

#[no_mangle]
fn add(a: i32) -> i32 {
    a + 1
}