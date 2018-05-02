use core::mem;

use super::*;

// Implementations for multiples of `mem::size_of::<usize>()`
macro_rules! impl_bytes_multi {
    ($($n:expr => $s:ident,)+) => { $(
        impl Bytes for [u8; $n] {
            #[inline]
            fn splat(byte: u8) -> Self {
                [byte; $n]
            }

            #[inline]
            fn is(&self, byte: u8) -> bool {
                #[cfg(feature = "simd")]
                {
                    let simd: $s = unsafe { mem::transmute(*self) };
                    simd.is(byte)
                }
                #[cfg(not(feature = "simd"))]
                {
                    type Arr = [usize; $n / mem::size_of::<usize>()];
                    let arr: Arr = unsafe { mem::transmute(*self) };

                    for &word in arr.iter() {
                        if !word.is(byte) {
                            return false;
                        }
                    }
                    true
                }
            }

            #[inline]
            fn contains(&self, byte: u8) -> bool {
                #[cfg(feature = "simd")]
                {
                    let simd: $s = unsafe { mem::transmute(*self) };
                    simd.contains(byte)
                }
                #[cfg(not(feature = "simd"))]
                { self[..].contains(&byte) }
            }

            #[inline]
            fn contains_zero(&self) -> bool {
                #[cfg(feature = "simd")]
                {
                    let simd: $s = unsafe { mem::transmute(*self) };
                    simd.contains_zero()
                }
                #[cfg(not(feature = "simd"))]
                {
                    const N: usize = $n / mem::size_of::<usize>();

                    let array: [usize; N] = unsafe {
                        mem::transmute_copy(&self)
                     };
                     for val in array.iter() {
                        if val.contains_zero() {
                            return true;
                        }
                    }
                     false
                }
            }
        }

        #[cfg(feature = "simd")]
        impl Bytes for $s {
            #[inline]
            fn splat(byte: u8) -> Self { Self::splat(byte) }

            #[inline]
            fn is(&self, byte: u8) -> bool {
                (*self).eq(Self::splat(byte)).all()
            }

            #[inline]
            fn contains(&self, byte: u8) -> bool {
                (*self).eq(Self::splat(byte)).any()
            }
         }
    )+ }
}

impl_bytes_multi! {
    // 2  => u8x2,
    // 4  => u8x4,
    // 8  => u8x8,
    16 => u8x16,
    32 => u8x32,
    64 => u8x64,
}
