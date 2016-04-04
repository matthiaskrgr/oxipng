use libz_sys;
use libc::c_int;
use std::cmp::max;

/// Decompress a data stream using the DEFLATE algorithm
pub fn inflate(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut input = data.to_owned();
    let mut stream = super::stream::Stream::new_decompress();
    let mut output = Vec::with_capacity(data.len());
    loop {
        match stream.decompress_vec(input.as_mut(), output.as_mut()) {
            libz_sys::Z_OK => output.reserve(data.len()),
            libz_sys::Z_STREAM_END => break,
            c => return Err(format!("Error code on decompress: {}", c)),
        }
    }
    output.shrink_to_fit();

    Ok(output)
}

/// Compress a data stream using the zlib implementation of the DEFLATE algorithm
pub fn deflate(data: &[u8], zc: u8, zm: u8, zs: u8, zw: u8) -> Result<Vec<u8>, String> {
    let mut input = data.to_owned();
    let mut stream = super::stream::Stream::new_compress(zc as c_int,
                                                         zw as c_int,
                                                         zm as c_int,
                                                         zs as c_int);
    // Compressed input should be smaller than decompressed, so allocate less than data.len()
    // However, it needs a minimum capacity in order to handle very small images
    let mut output = Vec::with_capacity(max(1024, data.len() / 20));
    loop {
        match stream.compress_vec(input.as_mut(), output.as_mut()) {
            libz_sys::Z_OK => output.reserve(max(1024, data.len() / 20)),
            libz_sys::Z_STREAM_END => break,
            c => return Err(format!("Error code on compress: {}", c)),
        }
    }
    output.shrink_to_fit();

    Ok(output)
}
