# Perry OS 
Minimal OS written in Rust
## prerequisites
- `qemu` installed

## build
set bare metal environment with:
```bash
rustup target add thumbv7em-none-eabihf
```

add rust source with:
```bash
rustup component add rust-src
```

install `bootimage` tool: 
```bash
cargo install bootimage
```
install llvm tools:
```bash
rustup component add llvm-tools-preview
```

create bootable disk image:
```bash
cargo bootimage 
```

## run
```bash
cargo run
```
