fn main() {
    let slice = &[1,2,3];
    let ptr = slice.as_ptr();
    let length = slice.len();
    let slice_again = unsafe {
        std::slice::from_raw_parts_mut(ptr as *mut i32, length)
    };
}