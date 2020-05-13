#![feature(ptr_internals)]
///! Implement a `std::Vec` from scratch.

use std::ptr::{Unique};

pub struct Vec<T> {
    ptr: Unique<T>,
    cap: usize,
    size: usize,
}
