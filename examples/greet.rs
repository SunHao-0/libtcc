use libtcc::*;
use std::ffi::{CStr, CString};
use std::mem::transmute;
use std::process::exit;

static GREET: &str = r#"
#include <stdio.h>
void greet(){
    printf("hello, rust\n");
}
"#;

fn main() {
    let c_program = CString::new(GREET.as_bytes()).unwrap();
    let mut err_warn = None;

    let mut g = Guard::new().unwrap();
    let mut ctx = Context::new(&mut g).unwrap();

    let compile_ret = ctx
        .set_output_type(OutputType::Memory)
        .set_call_back(|msg| err_warn = Some(String::from(msg.to_str().unwrap())))
        .compile_string(&c_program);
    if compile_ret.is_err() {
        drop(ctx);
        eprintln!("{:?}", err_warn);
        exit(1);
    }

    let mut relocated = ctx.relocate().unwrap();
    let addr = unsafe {
        relocated
            .get_symbol(CStr::from_bytes_with_nul_unchecked("greet\0".as_bytes()))
            .unwrap()
    };
    let greet: fn() = unsafe { transmute(addr) };
    greet();
}
