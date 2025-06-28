#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


pub use crate::*;



// functions
extern "C" {
    #[link_name = "_U_dyn_register"]
    pub fn unw_dyn_register( dyn_info: *mut unw_dyn_info_t );

    #[link_name = "_U_dyn_cancel"]
    pub fn unw_dyn_cancel( dyn_info: *mut unw_dyn_info_t );
}
