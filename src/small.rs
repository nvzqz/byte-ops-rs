use core::mem;

use super::*;

// Implementations that may fit within a register
macro_rules! impl_bytes_small_array {
    ($($n:expr => $s:ident $i:ident,)+) => { $(
        impl Bytes for [u8; $n] {
            #[inline]
            fn is(&self, byte: u8) -> bool {
                match $n {
                    // X xor Y equals 0 iff X equals Y
                    2 => (self[0] ^ byte) | (self[1] ^ byte) == 0,
                    _ => {
                        let value: $i = unsafe { mem::transmute(*self) };
                        value.is(byte)
                    },
                }
            }

            #[inline]
            fn is_zero(&self) -> bool {
                unsafe { mem::transmute::<_, $i>(*self) == 0 }
            }

            #[inline]
            fn contains(&self, byte: u8) -> bool {
                match $n {
                    2 => self[0] == byte || self[1] == byte,
                    _ => {
                        let value: $i = unsafe { mem::transmute(*self) };
                        value.contains(byte)
                    },
                }
            }
        }

        impl SizedBytes for [u8; $n] {
            #[inline]
            fn splat(byte: u8) -> Self { [byte; $n] }
        }

        #[cfg(feature = "simd")]
        impl Bytes for $s {
            #[inline]
            fn is(&self, byte: u8) -> bool {
                unsafe { mem::transmute::<_, [u8; $n]>(*self).is(byte) }
            }

            #[inline]
            fn is_zero(&self) -> bool {
                unsafe { mem::transmute::<_, $i>(*self) == 0 }
            }

            #[inline]
            fn contains(&self, byte: u8) -> bool {
                unsafe { mem::transmute::<_, [u8; $n]>(*self).contains(byte) }
            }
        }

        #[cfg(feature = "simd")]
        impl SizedBytes for $s {
            #[inline]
            fn splat(byte: u8) -> Self { Self::splat(byte) }
        }
    )+ };
}

impl_bytes_small_array! {
    2 => u8x2 u16,
    4 => u8x4 u32,
    8 => u8x8 u64,
}
