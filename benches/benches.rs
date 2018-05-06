#![feature(test)]

extern crate byte_ops;
extern crate test;

use byte_ops::*;
use test::{Bencher, black_box};

// Benchmarks worst case scenario where the entire array is searched
macro_rules! bench {
    ($($n:expr => $is:ident $is_slice:ident $contains:ident $contains_slice:ident;)+) => { $(
        #[bench]
        fn $is(b: &mut Bencher) {
            let array = [255u8; $n];

            b.iter(|| {
                for _ in 0..1000 {
                    black_box(Bytes::is(black_box(&array), black_box(255)));
                }
            });
        }

        #[bench]
        fn $is_slice(b: &mut Bencher) {
            let array = [255u8; $n];
            let slice = &array[..];

            b.iter(|| {
                for _ in 0..1000 {
                    black_box(Bytes::is(black_box(slice), black_box(255)));
                }
            });
        }

        #[bench]
        fn $contains(b: &mut Bencher) {
            let array = [255u8; $n];

            b.iter(|| {
                for _ in 0..1000 {
                    black_box(Bytes::contains(black_box(&array), black_box(0)));
                }
            });
        }

        #[bench]
        fn $contains_slice(b: &mut Bencher) {
            let array = [255u8; $n];
            let slice = &array[..];

            b.iter(|| {
                for _ in 0..1000 {
                    black_box(Bytes::contains(black_box(slice), black_box(0)));
                }
            });
        }
    )+ }
}

bench! {
    0008 => is_0008 is_slice_0008 contains_0008 contains_slice_0008;
    0016 => is_0016 is_slice_0016 contains_0016 contains_slice_0016;
    0032 => is_0032 is_slice_0032 contains_0032 contains_slice_0032;
    0064 => is_0064 is_slice_0064 contains_0064 contains_slice_0064;
    0128 => is_0128 is_slice_0128 contains_0128 contains_slice_0128;
    0256 => is_0256 is_slice_0256 contains_0256 contains_slice_0256;
    0512 => is_0512 is_slice_0512 contains_0512 contains_slice_0512;
    1024 => is_1024 is_slice_1024 contains_1024 contains_slice_1024;
}
