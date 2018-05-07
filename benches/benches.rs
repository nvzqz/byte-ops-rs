#![feature(test)]

extern crate byte_ops;
extern crate test;

use byte_ops::*;
use test::{Bencher, black_box};

fn is_naive(bytes: &[u8], b: u8) -> bool {
    bytes.iter().all(|&byte| byte == b)
}

// Benchmarks worst case scenario where the entire array is searched
macro_rules! bench {
    ($($n:expr =>
        $is_array:ident $is_slice:ident $is_naive:ident
        $co_array:ident $co_slice:ident $co_std:ident
    ;)+) => { $(
        #[bench]
        fn $is_array(b: &mut Bencher) {
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
        fn $is_naive(b: &mut Bencher) {
            let array = [255u8; $n];
            let slice = &array[..];

            b.iter(|| {
                for _ in 0..1000 {
                    black_box(is_naive(black_box(slice), black_box(255)));
                }
            });
        }

        #[bench]
        fn $co_array(b: &mut Bencher) {
            let array = [255u8; $n];

            b.iter(|| {
                for _ in 0..1000 {
                    black_box(Bytes::contains(black_box(&array), black_box(0)));
                }
            });
        }

        #[bench]
        fn $co_slice(b: &mut Bencher) {
            let array = [255u8; $n];
            let slice = &array[..];

            b.iter(|| {
                for _ in 0..1000 {
                    black_box(Bytes::contains(black_box(slice), black_box(0)));
                }
            });
        }

        #[bench]
        fn $co_std(b: &mut Bencher) {
            let array = [255u8; $n];
            let slice = &array[..];

            b.iter(|| {
                for _ in 0..1000 {
                    black_box(black_box(slice).contains(black_box(&0)));
                }
            });
        }
    )+ }
}

bench! {
    0008 => is_array_0008 is_slice_0008 is_naive_0008 contains_array_0008 contains_slice_0008 contains_std_0008;
    0016 => is_array_0016 is_slice_0016 is_naive_0016 contains_array_0016 contains_slice_0016 contains_std_0016;
    0032 => is_array_0032 is_slice_0032 is_naive_0032 contains_array_0032 contains_slice_0032 contains_std_0032;
    0064 => is_array_0064 is_slice_0064 is_naive_0064 contains_array_0064 contains_slice_0064 contains_std_0064;
    0128 => is_array_0128 is_slice_0128 is_naive_0128 contains_array_0128 contains_slice_0128 contains_std_0128;
    0256 => is_array_0256 is_slice_0256 is_naive_0256 contains_array_0256 contains_slice_0256 contains_std_0256;
    0512 => is_array_0512 is_slice_0512 is_naive_0512 contains_array_0512 contains_slice_0512 contains_std_0512;
    1024 => is_array_1024 is_slice_1024 is_naive_1024 contains_array_1024 contains_slice_1024 contains_std_1024;
}
