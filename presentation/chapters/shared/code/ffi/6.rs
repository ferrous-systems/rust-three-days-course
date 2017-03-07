pub fn leveldb_comparator_create(
    state: *mut c_void,
    destructor: extern fn(*mut c_void),
    compare: extern fn(*mut c_void, *const c_char, size_t, *const c_char, size_t) -> c_int,
    name: extern fn(*mut c_void) -> *const c_char
) -> *mut leveldb_comparator_t;
pub fn leveldb_comparator_destroy(c: *mut leveldb_comparator_t);