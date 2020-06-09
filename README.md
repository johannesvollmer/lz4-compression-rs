[![Crate](https://img.shields.io/crates/v/lz4-compression.svg)](https://crates.io/crates/lz4-compression)
[![Documentation](https://docs.rs/lz4-compression/badge.svg)](https://docs.rs/crate/lz4-compression/)


# LZ4-compression

A pure Rust implementation of LZ4 compression and decompression. Currently, this implementation is __not 100% compatible with the reference implementation__. Pull requests are welcome though!
This is based on [redox-os' LZ4 compression](https://github.com/redox-os/tfs/tree/master/lz4), but has been gradually improved since then. 

As this is only a passively maintained crate, consider using [lz-fear](https://github.com/main--/rust-lz-fear) instead,
which also aims to be compatible with the reference implementation of LZ4.

Usage: 
```rust
use lz4_compression::prelude::{ decompress, compress };

fn main(){
    let uncompressed_data: &[u8] = b"Hello world, what's up?";

    let compressed_data = compress(uncompressed_data);
    let decompressed_data = decompress(&compressed_data).unwrap();

    assert_eq!(uncompressed_data, decompressed_data.as_slice());
}
```


# Thanks to all contributors
- [Sergey Davidoff](https://github.com/Shnatsel)
- [Konrad Borowski](https://github.com/xfix)