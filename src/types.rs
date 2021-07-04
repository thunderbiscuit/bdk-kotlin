use ::safer_ffi::prelude::*;
use safer_ffi::char_p::char_p_boxed;

#[derive_ReprC]
#[repr(C)]
#[derive(Debug)]
pub struct FfiResult<T> {
    pub ok: T,
    pub err: char_p_boxed,
}

#[ffi_export]
fn free_string_result(string_result: FfiResult<char_p_boxed>) {
    drop(string_result)
}

#[ffi_export]
fn free_int_result(int_result: FfiResult<i32>) {
    drop(int_result)
}

// TODO do we need this? remove?
/// Free a Rust-allocated string
#[ffi_export]
fn free_string(string: Option<char_p_boxed>) {
    drop(string)
}