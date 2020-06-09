///! Implement a `std::Vec` from scratch.


pub struct Vector<T> {
    ptr: *mut T,
    cap: usize,
    len: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self { ptr: 0, cap: 0, len: 0 }
    }

}