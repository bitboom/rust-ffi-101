#[repr(C)]
pub struct MyCallback {
    cb: fn(),
}
#[no_mangle]
pub extern "C" fn call(it: MyCallback) {
    (it.cb)()
}
