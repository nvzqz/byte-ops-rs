use core::mem;

use super::*;

// Implementations that may fit within a register
macro_rules! impl_bytes_small_array {
    ($($n:expr => $s:ident $i:ident,)+) => { $(
        impl Bytes for [u8; $n] {
            #[inline]
            fn splat(byte: u8) -> Self { [byte; $n] }

            #[inline]
            fn is(self, byte: u8) -> bool {
                for &b in self.iter() {
                    if b != byte {
                        return false;
                    }
                }
                true
            }

            #[inline]
            fn is_zero(self) -> bool {
                unsafe { mem::transmute::<_, $i>(self) == 0 }
            }

            #[inline]
            fn contains(self, byte: u8) -> bool {
                match $n {
                    2 => {
                        for &b in self.iter() {
                            if b == byte {
                                return true;
                            }
                        }
                        false
                    },
                    _ => {
                        let value: $i = unsafe { mem::transmute(self) };
                        value.contains(byte)
                    },
                }
            }
        }
    )+ };
}

impl_bytes_small_array! {
    2 => u8x2 u16,
    4 => u8x4 u32,
    8 => u8x8 u64,
}
