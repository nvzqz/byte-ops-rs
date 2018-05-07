# Byte Ops

Common byte operations on arrays and slices in Rust.

## Supported Operations

The [`Bytes`](https://docs.rs/byte_ops/0.1.0/byte_ops/trait.Bytes.html) trait
contains all operations for this crate. As of this writing, they are:

- `is`: indicates whether _every byte_ in a value equals a certain byte.

- `contains`: indicates whether _any byte_ in a value equals a certain byte.

## SIMD Support

This crate contains [SIMD](https://en.wikipedia.org/wiki/SIMD)-accelerated
implementations, which can be enabled via the `simd` feature:

```toml
[dependencies.byte-ops]
version  = "0.1.0"
features = ["simd"]
```

Some extra performance can be gained by enabling `avx` when targeting x86 or
x86_64. This comes with a compatibility tradeoff. See [CPUs with
AVX](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#CPUs_with_AVX).

```sh
RUSTFLAGS="-C target-feature +avx"
```

## License

This library is licensed under either of

- [Apache License (Version 2.0)][license-apache]

- [MIT License][license-mit]

at your option.

[license-apache]: https://github.com/nvzqz/byte-ops-rs/blob/master/LICENSE-APACHE
[license-mit]:    https://github.com/nvzqz/byte-ops-rs/blob/master/LICENSE-MIT
