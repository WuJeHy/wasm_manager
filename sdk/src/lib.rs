/*
 * lib.rs, 2024-05-08 15:26 power by wujehy
 */

#[link(wasm_import_module = "trek")]
extern "C" {
    // 传输字符串测试
    fn print(ptr: *const u8, len: usize);
    // 测试接口
    fn exit(code: usize);
    // 获取环境id
    fn get_env_id() -> u64;
    // 注册环境
    fn register(id: u64, ptr: *const u8, len: usize) -> i32;
    // 值增加
    fn add_one() -> i32;
    // 获取当前值
    fn get_val() -> i32;
}


pub fn log_console(str: &str) {
    unsafe {
        print(str.as_ptr(), str.len())
    }
}

pub struct SystemApi {
    env_id: u64,
}

pub fn show_error_code(code: i32) {
    match code {
        0 => {
            log_console("操作成功")
        }
        -1 => {
            log_console("token 校验失败")
        }
        -2 => {
            log_console("宿主环境错误")
        }
        _ => {
            log_console("未知错误")
        }
    }
}

impl SystemApi {
    pub fn new() -> Self {
        let my_id = unsafe { get_env_id() };

        SystemApi {
            env_id: my_id
        }
    }

    pub fn register(&mut self, token: &str, id: u64) {
        if self.env_id != 0 {
            log_console("注册失败,已经注册过了");
            return;
        }


        let _ret = unsafe { register(id, token.as_ptr(), token.len()) };
        self.env_id = id
    }

    pub fn add_one(&self) {
        if self.env_id == 0 {
            log_console("实例未注册");
            return;
        }

        let ret = unsafe { add_one() };
        show_error_code(ret)
    }

    pub fn get_val(&self) -> i32 {
        if self.env_id == 0 {
            log_console("实例未注册");
            return 0;
        }
        unsafe { get_val() }
    }

    pub fn exit(code: usize) {
        unsafe { exit(code) }
    }
}
