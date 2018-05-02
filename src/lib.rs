#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "simd", feature(stdsimd))]

#[cfg(test)]
extern crate rand;

#[cfg(test)]
#[macro_use]
extern crate static_assertions;

#[cfg(feature = "std")]
use std as core;

#[cfg(feature = "simd")]
use core::simd::*;

mod multi;
mod scalar;
mod small;

/// A type that can be treated as a sequence of bytes.
pub trait Bytes {
    /// Creates a new instance with `byte` duplicated over all bytes.
    fn splat(byte: u8) -> Self where Self: Sized;

    /// Returns whether every byte in `self` is `byte`.
    fn is(&self, byte: u8) -> bool;

    /// Returns whether every byte in `self` is zero.
    #[inline]
    fn is_zero(&self) -> bool {
        self.is(0)
    }

    /// Returns whether `self` contains `byte`.
    fn contains(&self, byte: u8) -> bool;

    /// Returns whether `self` contains a zero byte.
    #[inline]
    fn contains_zero(&self) -> bool {
        self.contains(0)
    }
}

impl Bytes for u8 {
    #[inline]
    fn splat(byte: u8) -> u8 { byte }

    #[inline]
    fn is(&self, byte: u8) -> bool { *self == byte }

    #[inline]
    fn contains(&self, byte: u8) -> bool { *self == byte }

    #[inline]
    fn is_zero(&self) -> bool { *self == 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    assert_obj_safe!(__; Bytes);
}
