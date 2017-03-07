unsafe trait Comparator where Self::Sized {
    /// Return the name of the Comparator
    extern "C" fn name(state: *mut c_void) -> *const c_char;
    /// compare two keys. This must implement a total ordering.
    extern "C" fn compare(state: *mut c_void,
                          a: *const i8,
                          a_len: size_t,
                          b: *const i8,
                          b_len: size_t)
                          -> i32;

    extern "C" fn destructor(state: *mut c_void) {
        let _x: Box<Self> = unsafe { Box::from_raw(state as *mut Self) };
         // let the Box fall out of scope and run the T's destructor
    }
}