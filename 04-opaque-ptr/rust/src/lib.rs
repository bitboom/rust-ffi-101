#[repr(C)]
#[derive(Debug)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

#[no_mangle]
pub extern "C" fn create(c: &mut *mut Coordinate) {
    *c = Box::into_raw(Box::new(Coordinate { x: 3, y: 5 }));
}

#[no_mangle]
pub unsafe extern "C" fn destroy(c: *mut Coordinate) {
    Box::from_raw(c);
}

#[no_mangle]
pub unsafe extern "C" fn print(c: *const Coordinate) {
    println!("{:?}", *c);
}
