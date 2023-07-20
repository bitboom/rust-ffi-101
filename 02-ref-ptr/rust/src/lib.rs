#[no_mangle]
pub extern "C" fn pass_ref(i: &mut i32) {
    *i = 3;
}

#[no_mangle]
pub extern "C" fn pass_ptr(i: *mut i32) {
    unsafe {
        *i = 10;
    }
}
