<h1 align="center">Sentinel</h1>

## Get Started

```sh
git submodule init
git submodule update
rustup toolchain install stable
rustup default stable
cargo build --release
target/release/sentinel --help
```

## TODO

- [x] scan folder
  - mode
    - [x] fixed depth
    - [ ] every file
    - [ ] until file
  - [ ] multi root folder
  - [ ] folder size calculate
- [x] report to server
  - [ ] multi server
  - [ ] incremental report
- [x] dry-run mode
- [x] log
- [ ] config file
- [ ] daemon mode