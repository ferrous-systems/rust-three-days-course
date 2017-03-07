pub fn create_comparator<T: Comparator>(x: Box<T>) -> *mut leveldb_comparator_t {
    unsafe {
        leveldb_comparator_create(Box::into_raw(x) as *mut c_void,
                                  <T as Comparator>::destructor,
                                  <T as Comparator>::compare,
                                  <T as Comparator>::name)
    }
}