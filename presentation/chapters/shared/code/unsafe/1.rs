fn main() {
    let pointer_to_int = &mut i32;
    let raw = pointer_to_int as *mut i32;
    unsafe { deref_pointer(raw) };
}

unsafe deref_pointer<T: Debug>(p: *mut T) {
    unsafe { println("{:?}", *p }
}