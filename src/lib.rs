extern crate lean_sys;
use lean_sys::{lean_mk_empty_array, lean_obj_res};

#[no_mangle]
pub extern "C" fn add_from_rust(a : i32, b : i32) -> i32 {
    return a + b
}

#[no_mangle]
pub extern "C" fn my_empty_array(_a : i32, _b : i32) -> lean_obj_res {
    let x = unsafe { lean_mk_empty_array() };
    // let y = unsafe { lean_array_push(x, lean_mk_string("NamedNode".as_ptr())) };
    // unsafe { lean_array_push(x, lean_mk_string("http://example.org/test".as_ptr())) };
    // unsafe { lean_array_push(x, lean_mk_string("NamedNode".as_ptr())) };
    // unsafe { lean_array_push(x, lean_mk_string("http://example.org/predicate".as_ptr())) };
    // unsafe { lean_array_push(x, lean_mk_string("Literal".as_ptr())) };
    // unsafe { lean_array_push(x, lean_mk_string("belting".as_ptr())) };
    return x;
}
