#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


use crate::*;
use std::mem::ManuallyDrop;
use std::ffi::CStr;

#[repr(C)]
/// Doubly-linked list of dyn-info
pub struct unw_dyn_info_t {
    pub next: *mut unw_dyn_info_t,
    pub prev: *mut unw_dyn_info_t,
    /// First IP covered by this entry
    pub start_ip: usize,
    /// first IP NOT covered by this entry
    pub end_ip: usize,
    /// global-pointer in effect for this entry
    pub global_pointer: usize,
    /// real type: unw_dyn_info_format_t
    pub format: i32,
    pub pad: i32,
    pub u: unw_dyn_info_union
}

pub union unw_dyn_info_union {
    pi: ManuallyDrop<unw_dyn_proc_info_t>,
    ti: ManuallyDrop<unw_dyn_table_info_t>,
    rti: ManuallyDrop<unw_dyn_remote_table_info_t>,
}

#[repr(C)]
pub struct unw_dyn_proc_info_t {
    pub name_ptr: *mut CStr,
    pub handler: *mut (),
    pub flags: u32,
    pub pad0: i32,
    pub regions: *mut ()
}
#[repr(C)]
pub struct unw_dyn_table_info_t {
    /// addr. of table name (e.g., library name)
    pub name_pointer: *mut CStr,
    /// segment base
    pub segbase: usize,
    /// must be a multiple of sizeos(unw_word_t)
    pub table_len: usize,
    pub table_data: *mut usize
}
#[repr(C)]
pub struct unw_dyn_remote_table_info_t {
    /// addr. of table name (e.g., library name)
    pub name_ptr: *mut CStr,
    /// segment base
    pub segbase: usize,
    /// must be a multiple of sizeof(unw_word_t)!
    pub table_len: usize,
    pub table_data: usize,
}

// functions
extern "C" {
    #[link_name = "_U_dyn_register"]
    pub fn unw_dyn_register( dyn_info: *mut unw_dyn_info_t );

    #[link_name = "_U_dyn_cancel"]
    pub fn unw_dyn_cancel( dyn_info: *mut unw_dyn_info_t );
}
