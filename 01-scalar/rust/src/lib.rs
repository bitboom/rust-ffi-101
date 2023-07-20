#[no_mangle]
pub extern "C" fn print(a: i32, b: f32, c: bool) {
    println!("int: {}, float: {}, bool: {}", a, b, c);
}
