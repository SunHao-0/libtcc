# libtcc
![CI (Linux)](https://github.com/SunHao-0/libtcc/workflows/CI%20(Linux)/badge.svg) 
![codecov](https://codecov.io/gh/SunHao-0/libtcc/branch/master/graph/badge.svg) 
[![crate.io](https://img.shields.io/crates/v/libtcc)](https://crates.io/crates/libtcc)

Rust binding for [tcc](https://github.com/TinyCC/tinycc).

* [API Documentation (Releases)](https://docs.rs/libtcc/)
* Cargo package: [libtcc](https://crates.io/crates/libtcc)

TinyCC (or tcc) is short for Tiny C Compiler. It's a SMALL, FAST, UNLIMITED,SAFE C language Compiler.
This crate provide a safe wrapper for libtcc, which supports jit compilation and low level control of 
code generation.

## Usage

To use `libtcc`, add this to your `Cargo.toml`:

```toml
[dependencies]
libtcc = "0.1.1"
```

### Install tcc
Although this crate take `tcc` as part of itself, you still need to install tcc on your env. 
The reasons are:
1. libtcc.a need small but necessary runtime library(such as libtcc1.a) and some header files defined 
by tcc(such as stddef.h)
2. The purpose of using tcc as part of this crate is to support cross compilation, you still need tcc to 
be installed in your target env and installation of tcc in target env should not change install prefix.

### Initialize Guard
Tcc uses global variable during one compilation, which means user can not compile programs simultaneously.
To prevent incorrect usage, we provide `Guard`. Only one guard can exist in a specific scope and every instance 
of tcc hold a mutable reference to a guard so that rust compiler can detects incorrect usage via borrow checker.
```rust,ignore
use libtcc::{Guard, Context};

fn main(){
    let mut g1 = Guard::new();
    assert!(g1.is_ok());
//  let mut g2 = Guard::new();
//  assert!(g2.is_err());
    let ctx1 = Context::new(&mut g1).unwrap();

//  compile error 
//  let ctx2 =  Context::new(&mut g1).unwrap();
}
```

### In memory compilation 
```rust,ignore
use libtcc::{Guard, Context, OutputType};
use std::ffi::CString;

fn main(){
    let p = CString::new(r#"
        #include<stdio.h>
        void greet(){
            printf("hello world\n");
        }
        "#.as_bytes()).unwrap();

    let mut g = Guard::new().unwrap();
    let mut ctx = Context::new(&mut g).unwrap();
    assert!(ctx.compile_string(&p).is_ok());  

    let mut relocated = ctx.relocate().unwrap();
    let addr = unsafe {
        relocated
            .get_symbol(CStr::from_bytes_with_nul_unchecked("greet\0".as_bytes()))
            .unwrap()
    };
    let greet: fn() = unsafe { transmute(addr) };
    greet();
}
```


### More example

There are 
[examples](https://github.com/SunHao-0/libtcc/tree/master/examples)
which provide more information.

## Contributing

All contributions are welcome, if you have a feature request don't hesitate to open an issue!

## License

This project is licensed under of
 * MIT license ([LICENSE-MIT](LICENSE) or
   https://opensource.org/licenses/MIT)