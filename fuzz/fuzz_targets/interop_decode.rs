#![no_main]
use libfuzzer_sys::fuzz_target;
use lz4_compression::decompress::decompress;

fuzz_target!(|data: &[u8]| {
    let compression_result = reference_compress(data);
    if let Ok(compressed) = compression_result {
        let decompressed = decompress(&compressed).expect("Failed to decompress what reference implementation has compressed");
        assert!(decompressed.as_slice() == data, "Decompressed data is different from the original");
    }
});

// compress data using the reference lz4 implementation 
fn reference_compress(data: &[u8]) -> Result<Vec<u8>,()> {
    let mut input = std::io::Cursor::new(data);
    let output = std::io::Cursor::new(Vec::new());
    let mut encoder = lz4::EncoderBuilder::new()
        .level(4)
        .build(output).unwrap();
    std::io::copy(&mut input, &mut encoder).unwrap();
    let (output, result) = encoder.finish();
    if result.is_ok() {
        Ok(output.into_inner())
    } else {
        Err(())
    }
}
