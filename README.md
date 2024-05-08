# wasm manger 

## 概述
一种 wasm 管理器 的实现,  用于插件模式开发的项目的原型. 

这里只实现了基本能力, 仅供学习. 



完整项目用于 解决一些 , 不想重启主服务 , 想要动态扩展 功能的服务. 以及对第三方提供 有限的开发能力的云平台 / SaaS /Faas  等. 



提供两个例子, 具体实现 自己看源码. 



说到源码,  [src/app.rs](src/app.rs) 里面有说明写法. 看完这个 应该就好理解一些逻辑了.

```
 		// NOTE 这里 用 rust 的骚操作 写一个链式

        // 普通版本 这个可读性性比较强
        // let get_lock = self.data.get(&id).ok_or(TrekWasmError::UnknownErrFail)?;
        // let get_data = get_lock.lock().ok().ok_or(TrekWasmError::UnknownErrFail)?;
        // callback(&get_data)

        // 回调的写法
        // self.data.get(&id).map(|data| {
        //     let userdata = data.lock().ok().ok_or(TrekWasmError::UnknownErrFail)?;
        //     callback(&userdata)
        // }).ok_or(TrekWasmError::UnknownErrFail)?

        // 链式
        self.data
            .get(&id)
            .ok_or(TrekWasmError::UnknownErrFail)?
            .lock()
            .ok()
            .ok_or(TrekWasmError::UnknownErrFail)
            .map(|d| callback(&d))?

        // 这三个的效果是一样的
        // 比 go 的 err != nil 舒服多的原因
```

 

## 项目结构

```
├── Cargo.toml
├── sdk -- 提供的sdk 简单的封装
│   ├── app
│   │   ├── app1.rs -- 动态加载的实现 第一步
│   │   └── app2.rs -- 动态加载的实现 第二步
│   ├── Cargo.toml
│   └── src
│       └── lib.rs  -- abi 封装
├── src -- 核心源码
│   ├── abi
│   │   ├── demo_abi.rs -- demo 的abi 
│   │   └── mod.rs
│   ├── app.rs
│   ├── cmd
│   │   ├── demo_plugin.rs -- 最小测试插件 
│   │   ├── test_manager_app_1_and_2.rs -- 动态加载 控制器
│   │   └── test_manager.rs -- 最小案例 控制器
│   ├── error.rs 
│   ├── lib.rs
│   ├── manger.rs -- 管理
│   ├── runtime.rs -- 运行时
│   └── tools.rs
```



### 例子说明:

#### 最小案例 :


```shell
rustc --target=wasm32-unknown-unknown src/cmd/demo_plugin.rs -C opt-level='z' -o demo.wasm
```

```shell
cargo run --package wasm_manager --bin test_manager
```

输出:

```
 INFO wasm_manager::abi::demo_abi: call print:Trek World!  [12]
 INFO wasm_manager::abi::demo_abi: inner exit 0
 INFO wasm_manager::runtime: drop TrekFunctionEnv store_raw 1
```



#### 动态加载

业务A

```shell
cargo build --package sdk --bin app1 --target=wasm32-unknown-unknown
mv ./target/wasm32-unknown-unknown/debug/app1.wasm app1.wasm 
```
业务B

```shell
cargo build --package sdk --bin app2 --target=wasm32-unknown-unknown
mv ./target/wasm32-unknown-unknown/debug/app2.wasm app2.wasm 

```

主控

```shell

cargo run --package wasm_manager --bin test_manager_app_1_and_2

```

输出:

```
 INFO wasm_manager::abi::demo_abi: call print:start app 1 [11]
 INFO wasm_manager::abi::demo_abi: call print:操作成功 [12]
 INFO wasm_manager::abi::demo_abi: call print:data 1 [6]
 INFO wasm_manager::abi::demo_abi: inner exit 0
 INFO wasm_manager::abi::demo_abi: call print:操作成功 [12]
 INFO wasm_manager::abi::demo_abi: call print:操作成功 [12]
 INFO wasm_manager::abi::demo_abi: call print:start app 2 [11]
 INFO wasm_manager::app: 初始化过了
 INFO wasm_manager::abi::demo_abi: call print:data 3 [6]
 INFO wasm_manager::abi::demo_abi: inner exit 0
 INFO wasm_manager::runtime: drop TrekFunctionEnv store_raw 2
 INFO wasm_manager::runtime: drop TrekFunctionEnv store_raw 1
```

实例运行 二进制: 从release 下载 即可 
运行环境: ubuntu 22.04 x86_64



