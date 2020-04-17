#![no_main]
use libfuzzer_sys::fuzz_target;
use lz4_compression::decompress::decompress;

fuzz_target!(|data: &[u8]| {
    decompress(data);
});
