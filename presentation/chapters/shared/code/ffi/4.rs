#[repr(C)]
#[derive(Copy,Clone)]
pub enum Compression {
  No = 0,
  Snappy = 1
}