#![no_main]
use libfuzzer_sys::fuzz_target;
use lz4_compression::decompress::decompress;
use lz4_compression::compress::compress;

fuzz_target!(|data: &[u8]| {
    let compressed = compress(data);
    let decompressed = decompress(&compressed).expect("Failed to decompress what we've compressed");
    assert!(decompressed.as_slice() == data, "Decompressed data is different from the original");
});
