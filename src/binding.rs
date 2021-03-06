/* automatically generated by rust-bindgen */

pub const TCC_OUTPUT_MEMORY: u32 = 1;
pub const TCC_OUTPUT_EXE: u32 = 2;
pub const TCC_OUTPUT_DLL: u32 = 3;
pub const TCC_OUTPUT_OBJ: u32 = 4;
pub const TCC_OUTPUT_PREPROCESS: u32 = 5;
#[repr(C)]
#[derive(Debug)]
pub struct TCCState {
    _unused: [u8; 0],
}
extern "C" {
    pub fn tcc_new() -> *mut TCCState;
}
extern "C" {
    pub fn tcc_delete(s: *mut TCCState);
}
extern "C" {
    pub fn tcc_set_lib_path(s: *mut TCCState, path: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn tcc_set_error_func(
        s: *mut TCCState,
        error_opaque: *mut ::std::os::raw::c_void,
        error_func: ::std::option::Option<
            unsafe extern "C" fn(
                opaque: *mut ::std::os::raw::c_void,
                msg: *const ::std::os::raw::c_char,
            ),
        >,
    );
}
extern "C" {
    pub fn tcc_set_options(s: *mut TCCState, str: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn tcc_add_include_path(
        s: *mut TCCState,
        pathname: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tcc_add_sysinclude_path(
        s: *mut TCCState,
        pathname: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tcc_define_symbol(
        s: *mut TCCState,
        sym: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn tcc_undefine_symbol(s: *mut TCCState, sym: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn tcc_add_file(
        s: *mut TCCState,
        filename: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tcc_compile_string(
        s: *mut TCCState,
        buf: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tcc_set_output_type(
        s: *mut TCCState,
        output_type: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tcc_add_library_path(
        s: *mut TCCState,
        pathname: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tcc_add_library(
        s: *mut TCCState,
        libraryname: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tcc_add_symbol(
        s: *mut TCCState,
        name: *const ::std::os::raw::c_char,
        val: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tcc_output_file(
        s: *mut TCCState,
        filename: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tcc_run(
        s: *mut TCCState,
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tcc_relocate(
        s1: *mut TCCState,
        ptr: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tcc_get_symbol(
        s: *mut TCCState,
        name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
