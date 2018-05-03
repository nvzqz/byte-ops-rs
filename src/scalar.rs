use super::*;

const LO: u64 = ::core::u64::MAX / 0xFF;
const HI: u64 = LO << 7;

macro_rules! impl_bytes_scalar {
    ($($t:ident $u:ident)+) => { $(
        impl Bytes for $t {
            #[inline]
            fn is(&self, byte: u8) -> bool { *self == Self::splat(byte) }

            #[inline]
            fn contains(&self, byte: u8) -> bool {
                (self ^ Self::splat(byte)).contains_zero()
            }

            // From Matters Computational by J. Arndt (1.20)
            //
            // "The idea is to subtract one from each of the bytes and then look
            // for bytes where the borrow propagated all the way to the most
            // significant bit."
            #[inline]
            fn contains_zero(&self) -> bool {
                self.wrapping_sub(LO as Self) & !self & HI as Self != 0
            }
        }

        impl Bytes for $u {
            #[inline]
            fn is(&self, byte: u8) -> bool { (*self as $t).is(byte) }

            #[inline]
            fn contains(&self, byte: u8) -> bool { (*self as $t).contains(byte) }

            #[inline]
            fn contains_zero(&self) -> bool { (*self as $t).contains_zero() }
        }

        impl SizedBytes for $t {
            #[inline]
            fn splat(byte: u8) -> Self {
                // Spread `byte`'s bits across each starting bit in `LO`
                LO as Self * byte as Self
            }
        }

        impl SizedBytes for $u {
            #[inline]
            fn splat(byte: u8) -> Self { $t::splat(byte) as Self }
        }
    )+ }
}

impl_bytes_scalar! { u16 i16 u32 i32 u64 i64 usize isize }
