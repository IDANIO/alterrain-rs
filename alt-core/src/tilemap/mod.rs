//! An instance of a Tilemap
//!
//! Use whatever implementation you like internally, currently implementation is very naive:
//! simply using 2D array.

pub struct Tilemap {
    width: usize,
    height: usize,
}
