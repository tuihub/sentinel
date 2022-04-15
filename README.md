# Sentinel

## Get Started

```sh
git submodule init
git submodule update
rustup toolchain install stable
rustup default stable
cargo build --release
target/release/sentinel --help
```

## 功能列表

- [x] 单次扫描单个文件夹
- [x] 上报列表给单个服务端
- [x] dry run 模式
- [x] 日志
- [ ] 配置文件
- [ ] 增量上报
- [ ] 多文件夹支持
- [ ] 多服务端支持
- [ ] 守护进程模式
- [ ] 文件夹占用空间计算