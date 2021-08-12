use std::{fs::File, io::{BufReader, Read}, path::PathBuf};
use flate2::read::GzDecoder;
use anyhow::{bail, anyhow, Context, Result};
use grep_cli::DecompressionReader;

pub fn open_comp_file(filename: &PathBuf) -> Result<Box<dyn Read>> {
    let ext = {
        match filename.to_str().unwrap().rfind('.') {
            None => String::from(""),
            Some(i) => String::from(&filename.to_str().unwrap()[i..]),
        }
    };
    let io_block_size = 64*1024;
    let mut rdr: Box<dyn Read> = match &ext[..] {
        ".gz" | ".tgz" => {
            match File::open(&filename) {
                Ok(f) => if io_block_size != 0 {
                    Box::new(GzDecoder::new(BufReader::with_capacity(io_block_size,f)))
                } else {
                    Box::new(GzDecoder::new(f))
                },
                Err(err) => {
                    return Err(anyhow!("skipping gz file \"{}\", due to error: {}", filename.display(), err));
                }
            }
        }
        ".zst" | ".zstd" => {
            match File::open(&filename) {
                Ok(f) => {
                    match zstd::stream::read::Decoder::new({
                        if io_block_size != 0 {
                            BufReader::with_capacity(io_block_size, f)
                        } else {
                            BufReader::new(f)
                        }
                    }) {
                        Ok(br) => Box::new(br),
                        Err(err) => {
                            return Err(anyhow!("skipping file \"{}\", zstd decoder error: {}", filename.display(), err));
                        },
                    }
                }
                Err(err) => {
                    return Err(anyhow!("skipping zst file \"{}\", due to error: {}", filename.display(), err));
                }
            }
        }
        ".bz2" | ".tbz2" | ".txz" | ".xz" | ".lz4" | ".lzma" | ".br" | ".Z" => {
            if io_block_size != 0 {
                return Err(anyhow!("file {} cannot override default IO block size as it is opened via a different method", filename.display()));
            }
            match DecompressionReader::new(&filename) {
                Ok(rdr) => Box::new(rdr),
                Err(err) => {
                    return Err(anyhow!("skipping general de-comp file \"{}\", due to error: {}", filename.display(), err));
                }
            }
        }
        _ => {
            match File::open(&filename) {
                Ok(f) => if io_block_size != 0 {
                    Box::new(BufReader::with_capacity(io_block_size, f))
                } else {
                    Box::new(BufReader::new(f))
                },
                Err(err) => {
                    return Err(anyhow!("skipping regular file \"{}\", due to error: {}", filename.display(), err));
                }
            }
        },
    };
    Ok(rdr)
}

