use std::ptr;

let error = ptr::null_mut();

let db = unsafe {
    leveldb_open(options,
                 name,
                 &mut error)
};

if error == ptr::null_mut() {
    Ok(Database::new(...))
} else {
    Err(Error::new_from_i8(error))
}