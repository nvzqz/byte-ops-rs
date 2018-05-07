use super::*;

macro_rules! multi_cast {
    ($r:expr, $base:expr, $n:expr) => { unsafe {
        &*($r as *const _ as *const [[u8; $base]; $n])
    } };
}

macro_rules! large_helper {
    ($base:expr => $($n:expr)+) => { $(
        impl SizedBytes for [u8; $n * $base] {
            #[inline]
            fn splat(byte: u8) -> Self { [byte; $n * $base] }
        }

        impl Bytes for [u8; $n * $base] {
            #[inline]
            fn contains(&self, byte: u8) -> bool {
                for val in multi_cast!(self, $base, $n).iter() {
                    if val.contains(byte) {
                        return true;
                    }
                }
                false
            }

            #[inline]
            fn is(&self, byte: u8) -> bool {
                for val in multi_cast!(self, $base, $n).iter() {
                    if !val.is(byte) {
                        return false;
                    }
                }
                true
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

macro_rules! large {
    ($($base:expr)+) => {
        $(large_helper! { $base => 2 3 4 5 6 7 })+
    };
}

large! {
    64 256 1024
}
