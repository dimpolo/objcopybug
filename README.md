# Steps
* `rustup component add llvm-tools-preview`
* `cargo run`
* Compare generated binaries

# Info
```shell
> rust-objcopy -V 
llvm-objcopy, compatible with GNU objcopy
LLVM (http://llvm.org/):
  LLVM version 15.0.2-rust-1.66.0-nightly
  Optimized build.
  Default target: x86_64-pc-windows-msvc
  Host CPU: znver1

```
.cargo/config
```text
[target.thumbv6m-none-eabi]

[target.'cfg(all(target_arch = "arm", target_os = "none"))']

rustflags = [
  "-C", "link-arg=--nmagic",
  "-C", "link-arg=-Tlink.x",
]

[build]
target = "thumbv6m-none-eabi"

[unstable]
build-std = ["core"]
```

memory.x
```text
MEMORY
{
  FLASH : ORIGIN = 0x08004000, LENGTH = 64K - 16K - 2K
  RAM : ORIGIN = 0x20000000, LENGTH = 18K
}
```