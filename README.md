# 环境配置
1. Install WebAssembly target
```
rustup target add wasm32-unknown-unknown
```

2. Install Trunk
```
cargo install --locked trunk
```
(note that this might take a while to install because it compiles everything from scratch)

reference: https://yew.rs/zh-Hans/docs/getting-started/introduction

# 项目运行
```
trunk serve
```
(使用`trunk`来运行项目，注意不是`cargo run`!)