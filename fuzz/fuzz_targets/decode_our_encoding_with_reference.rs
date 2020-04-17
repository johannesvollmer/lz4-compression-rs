#![no_main]
use libfuzzer_sys::fuzz_target;
use lz4_compression::compress::compress;

fuzz_target!(|data: &[u8]| {
    let compressed = compress(data);
    let decompressed = reference_decompress(&compressed).expect("Reference implementation has failed to decompress what we've compressed");
    assert!(decompressed.as_slice() == data, "Decompressed data is different from the original");
});

// decompress data using the reference lz4 implementation 
fn reference_decompress(data: &[u8]) -> Result<Vec<u8>,std::io::Error> {
    let input = std::io::Cursor::new(data);
    let mut decoder = lz4::Decoder::new(input)?;
    let mut output = std::io::Cursor::new(Vec::new());
    std::io::copy(&mut decoder, &mut output)?;
    let (_reader, result) = decoder.finish();
    result?;
    Ok(output.into_inner())
}
