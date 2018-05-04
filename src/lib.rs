#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "simd", feature(stdsimd))]

#![deny(unused_variables)]

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

#[cfg(not(feature = "simd"))]
type Batch = usize;

#[cfg(feature = "simd")]
type Batch = u8x32;

/// A type that can be treated as a sequence of bytes.
pub trait Bytes {
    /// Returns whether every byte in `self` is `byte`.
    fn is(&self, byte: u8) -> bool;

    /// Returns whether every byte in `self` is zero.
    #[inline]
    fn is_zero(&self) -> bool { self.is(0) }

    /// Returns whether `self` contains `byte`.
    fn contains(&self, byte: u8) -> bool;

    /// Returns whether `self` contains a zero byte.
    #[inline]
    fn contains_zero(&self) -> bool { self.contains(0) }
}

impl Bytes for u8 {
    #[inline]
    fn is(&self, byte: u8) -> bool { *self == byte }

    #[inline]
    fn contains(&self, byte: u8) -> bool { *self == byte }

    #[inline]
    fn is_zero(&self) -> bool { *self == 0 }
}

// Alignment code used by the `bytecount` crate
fn batch_align(b: &[u8]) -> (&[u8], &[Batch], &[u8]) {
    use core::{cmp, mem, slice};

    const ALIGN: usize = mem::align_of::<Batch>();

    let address   = b.as_ptr() as usize;
    let align_rem = address % ALIGN;
    let align_end = (address + b.len()) % ALIGN;

    let d2 = b.len().saturating_sub(align_end);
    let d1 = cmp::min((ALIGN - align_rem) % ALIGN, d2);

    let (init, tail) = b.split_at(d2);
    let (init, mid) = init.split_at(d1);

    assert_eq!(mid.len() % ALIGN, 0);
    let mid = unsafe {
        slice::from_raw_parts(mid.as_ptr() as *const Batch, mid.len() / ALIGN)
    };

    (init, mid, tail)
}

macro_rules! batched {
    ($val:expr, $byte:expr, $f:expr, $b:expr) => {
        let (x, y, z) = batch_align($val);

        for batch in y {
            if $f(batch, $byte) == $b {
                return $b;
            }
        }

        for &slice in &[x, z] {
            for byte in slice {
                if $f(byte, $byte) == $b {
                    return $b;
                }
            }
        }

        !$b
    };
}

impl Bytes for [u8] {
    fn is(&self, byte: u8) -> bool {
        if self.is_empty() {
            return false;
        }
        batched! { self, byte, Bytes::is, false }
    }

    fn contains(&self, byte: u8) -> bool {
        if self.is_empty() {
            return false;
        }
        batched! { self, byte, Bytes::contains, true }
    }
}

/// A type that can be treated as a statically-sized sequence of bytes.
pub trait SizedBytes: Sized + Bytes {
    /// Creates a new instance with `byte` duplicated over all bytes.
    fn splat(byte: u8) -> Self where Self: Sized;
}

impl SizedBytes for u8 {
    #[inline]
    fn splat(byte: u8) -> u8 { byte }
}

#[cfg(test)]
mod tests {
    use super::*;

    assert_obj_safe!(__; Bytes);

    #[test]
    fn slice() {
        const UNALIGNED: usize = 27;

        let array: [u8; UNALIGNED] = rand::random();
        let slice = &array[..];

        for &byte in slice {
            assert!(Bytes::contains(slice, byte),
                    "{:?} does not contain {:?}",
                    slice,
                    byte);
            assert!(!Bytes::is(slice, !byte),
                    "{:?} all equals {:?}",
                    slice,
                    byte);
        }

        for f in &[Bytes::is, Bytes::contains] {
            let empty = &[0u8; 0][..];
            assert!(!f(empty, 0));
        }

        for f in &[Bytes::is, Bytes::contains] {
            const VALUE: u8 = 42;
            const SLICE: &[u8] = &[VALUE; UNALIGNED];
            assert!(f(SLICE, VALUE));
        }
    }
}
