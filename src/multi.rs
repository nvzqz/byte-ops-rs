#[cfg(not(feature = "simd"))]
use core::mem;

use super::*;

// Implementations for multiples of `mem::size_of::<usize>()`
macro_rules! impl_bytes_multi {
    ($($n:expr => $s:ident,)+) => { $(
        impl SizedBytes for [u8; $n] {
            #[inline]
            fn splat(byte: u8) -> Self { [byte; $n] }
        }

        impl Bytes for [u8; $n] {
            #[inline]
            fn is(&self, byte: u8) -> bool {
                #[cfg(feature = "simd")]
                { $s::load_unaligned(self).is(byte) }

                #[cfg(not(feature = "simd"))]
                {
                    type Arr = [usize; $n / mem::size_of::<usize>()];
                    let arr: Arr = unsafe { mem::transmute(*self) };

                    for word in arr.iter() {
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
                { $s::load_unaligned(self).contains(byte) }

                #[cfg(not(feature = "simd"))]
                {
                    type Arr = [usize; $n / mem::size_of::<usize>()];
                    let arr: Arr = unsafe { mem::transmute(*self) };

                    for word in arr.iter() {
                        if word.contains(byte) {
                            return true;
                        }
                    }
                    false
                }
            }

            #[inline]
            fn first_eq(&self, byte: u8) -> Option<&u8> {
                unimplemented!()
            }

            #[inline]
            fn first_eq_mut(&mut self, byte: u8) -> Option<&mut u8> {
                unimplemented!()
            }

            #[inline]
            fn first_ne(&self, byte: u8) -> Option<&u8> {
                unimplemented!()
            }

            #[inline]
            fn first_ne_mut(&mut self, byte: u8) -> Option<&mut u8> {
                unimplemented!()
            }

            #[inline]
            fn last_eq(&self, byte: u8) -> Option<&u8> {
                unimplemented!()
            }

            #[inline]
            fn last_eq_mut(&mut self, byte: u8) -> Option<&mut u8> {
                unimplemented!()
            }

            #[inline]
            fn last_ne(&self, byte: u8) -> Option<&u8> {
                unimplemented!()
            }

            #[inline]
            fn last_ne_mut(&mut self, byte: u8) -> Option<&mut u8> {
                unimplemented!()
            }
        }

        #[cfg(feature = "simd")]
        impl SizedBytes for $s {
            #[inline]
            fn splat(byte: u8) -> Self { Self::splat(byte) }
        }

        #[cfg(feature = "simd")]
        impl Bytes for $s {
            #[inline]
            fn is(&self, byte: u8) -> bool {
                (*self).eq(Self::splat(byte)).all()
            }

            #[inline]
            fn contains(&self, byte: u8) -> bool {
                (*self).eq(Self::splat(byte)).any()
            }

            #[inline]
            fn first_eq(&self, byte: u8) -> Option<&u8> {
                unimplemented!()
            }

            #[inline]
            fn first_eq_mut(&mut self, byte: u8) -> Option<&mut u8> {
                unimplemented!()
            }

            #[inline]
            fn first_ne(&self, byte: u8) -> Option<&u8> {
                unimplemented!()
            }

            #[inline]
            fn first_ne_mut(&mut self, byte: u8) -> Option<&mut u8> {
                unimplemented!()
            }

            #[inline]
            fn last_eq(&self, byte: u8) -> Option<&u8> {
                unimplemented!()
            }

            #[inline]
            fn last_eq_mut(&mut self, byte: u8) -> Option<&mut u8> {
                unimplemented!()
            }

            #[inline]
            fn last_ne(&self, byte: u8) -> Option<&u8> {
                unimplemented!()
            }

            #[inline]
            fn last_ne_mut(&mut self, byte: u8) -> Option<&mut u8> {
                unimplemented!()
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
