# rust-cross-compile-omega2

This project keeps some information and examples on how to cross compile rust code using Cargo to be executed on the Omega2

## Cross compile information

General rust cross compile information

https://github.com/japaric/rust-cross

Omega2 build system setup on ubuntu using LEDE

https://docs.onion.io/omega2-docs/cross-compiling.html 

```
Architecture -> mips
Vendor -> unknown
System -> Linux
ABI -> musl libc (mipsel-sf)
Version 1.1.16
Dynamic Program Loader
```

triple ->
mipsel-unknown-linux-musl

- This is to setup local rust to enable mispel / omega2 compile
```
rustup target add mipsel-unknown-linux-musl
```
- Example of the content of a 'config' file to be created in a .cargo folder that should be at the same level as the Cargo.toml

```
[target.mipsel-unknown-linux-musl]
linker = "/home/sam/cross_compile/source/staging_dir/toolchain-mipsel_24kc_gcc-5.4.0_musl-1.1.16/bin/mipsel-openwrt-linux-musl-gcc"
```

- This is how to ask from Cargo to produce a mispel binary when the linker / build system has been created / configured for this project + rust cross compiled crates are installed
```
cargo build --target=mipsel-unknown-linux-musl
```