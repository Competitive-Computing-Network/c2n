#[no_mangle]
pub extern "C" fn increment(value: i32) -> i32 {
    value + 1
}