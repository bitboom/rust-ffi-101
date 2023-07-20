#[repr(C)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

#[no_mangle]
pub extern "C" fn flip(c: Coordinate) -> Coordinate {
    Coordinate { x: c.y, y: c.x }
}
