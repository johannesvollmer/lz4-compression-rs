This directory contains fuzzing targets to verify implementation correctness: 

- `roundtrip` target checks that we can successfully decode what we've encoded.
- `decompress` targets checks that we don't panic or abort on decoding a crafted file.
- targets with `reference` in the name check that we can interoperate with the
reference implementation of LZ4.

The command to run fuzzer is:

`cargo +nightly fuzz run --release -s none <fuzzing_target>`

For example,

`cargo +nightly fuzz run --release -s none roundtrip`

We use `-s none` because this crate does not contain unsafe code, so we don't
need sanitizers to detect memory or concurrency errors for us.

For more info see `cargo +nightly fuzz help`
