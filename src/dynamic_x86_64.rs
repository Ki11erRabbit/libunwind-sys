
use crate::*;



// functions
extern "C" {
    #[link_name = "_U_dyn_register"]
    pub fn unw_dyn_register( dyn_info: *mut unw_dyn_info_t );

    #[link_name = "_U_dyn_cancel"]
    pub fn unw_dyn_cancel( dyn_info: *mut unw_dyn_info_t );
}
