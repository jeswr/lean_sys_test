// extern crate lean_sys;
// use lean_sys::{lean_mk_empty_array, lean_obj_res};

#[no_mangle]
pub extern "C" fn add_from_rust(a : i32, b : i32) -> i32 {
    return a + b
}

// #[no_mangle]
// pub extern "C" fn empty_array() -> lean_obj_res {
//     return unsafe { lean_mk_empty_array() };
// }
