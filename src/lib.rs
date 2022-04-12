use std::convert::TryInto;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, BufReader};

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum ImFormat {
    GIF,
    PNG,
    BMP,
    JPEG,
    WEBP,
    QOI,
    PSD,
    XCF,
    ICO,
    AVIF,
    TIFF,
}

impl std::fmt::Display for ImFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GIF  => "gif",
            Self::PNG  => "png",
            Self::BMP  => "bmp",
            Self::JPEG => "jpeg",
            Self::WEBP => "webp",
            Self::QOI  => "qoi",
            Self::PSD  => "psd",
            Self::XCF  => "xcf",
            Self::ICO  => "ico",
            Self::AVIF => "avif",
            Self::TIFF => "tiff",
        }.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub struct ImInfo {
    pub width:  u64,
    pub height: u64,
    pub format: ImFormat,
}

#[derive(Debug)]
pub enum ImError {
    IO(std::io::Error),
    UnknownFormat,
    ParserError(ImFormat)
}

impl std::fmt::Display for ImError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(error) => error.fmt(f),
            Self::UnknownFormat => "Unknown Format".fmt(f),
            Self::ParserError(format) => write!(f, "Error parsing {} image", format)
        }
    }
}

impl From<std::io::Error> for ImError {
    fn from(error: std::io::Error) -> Self {
        ImError::IO(error)
    }
}

pub type ImResult<T> = std::result::Result<T, ImError>;

#[inline]
fn get_array<const LEN: usize>(slice: &[u8], format: ImFormat) -> ImResult<[u8; LEN]> {
    match slice[..LEN].try_into() {
        Ok(array) => Ok(array),
        Err(_) => Err(ImError::ParserError(format)),
    }
}

fn find_avif_chunk<R>(reader: &mut R, name: &[u8], chunk_size: u64) -> ImResult<u64>
where R: Read, R: Seek {
    let mut sub_chunk_size;
    let mut buf = [0u8; 8];
    let mut offset = 0;

    loop {
        if offset > chunk_size {
            return Err(ImError::ParserError(ImFormat::AVIF));
        }
        if let Err(_) = reader.read_exact(&mut buf) {
            return Err(ImError::ParserError(ImFormat::AVIF));
        }
        sub_chunk_size = u32::from_be_bytes(get_array(&buf, ImFormat::AVIF)?) as u64;
        if sub_chunk_size < 8 {
            return Err(ImError::ParserError(ImFormat::AVIF));
        }
        if buf.ends_with(name) {
            break;
        }
        offset += sub_chunk_size;
        if let Err(_) = reader.seek(SeekFrom::Current(sub_chunk_size as i64 - 8)) {
            return Err(ImError::ParserError(ImFormat::AVIF));
        }
    }

    return Ok(sub_chunk_size);
}

pub fn imsz(fname: impl AsRef<std::path::Path>) -> ImResult<ImInfo> {
    let mut file = File::open(fname)?;
    return imsz_file(&mut file);
}

pub fn imsz_file(file: &mut File) -> ImResult<ImInfo> {
    let mut preamble = [0u8; 30];

    let size = file.read(&mut preamble)?;

    if size >= 6 && (&preamble[..6] == b"GIF87a" || &preamble[..6] == b"GIF89a") {
        // GIF
        if size < 10 {
            return Err(ImError::ParserError(ImFormat::GIF));
        }
        let w = u16::from_le_bytes(get_array(&preamble[6..], ImFormat::GIF)?);
        let h = u16::from_le_bytes(get_array(&preamble[8..], ImFormat::GIF)?);

        return Ok(ImInfo {
            format: ImFormat::GIF,
            width:  w as u64,
            height: h as u64,
        });
    } else if size >= 16 && &preamble[..8] == b"\x89PNG\r\n\x1a\n" {
        // PNG
        let w;
        let h;
        if &preamble[12..16] == b"IHDR" {
            if size < 24 {
                return Err(ImError::ParserError(ImFormat::PNG));
            }
            w = u32::from_be_bytes(get_array(&preamble[16..], ImFormat::PNG)?);
            h = u32::from_be_bytes(get_array(&preamble[20..], ImFormat::PNG)?);
        } else {
            w = u32::from_be_bytes(get_array(&preamble[ 8..], ImFormat::PNG)?);
            h = u32::from_be_bytes(get_array(&preamble[12..], ImFormat::PNG)?);
        }

        return Ok(ImInfo {
            format: ImFormat::PNG,
            width:  w as u64,
            height: h as u64,
        });
    } else if size >= 10 && (&preamble[..2] == b"BM" && &preamble[6..10] == b"\0\0\0\0") {
        // BMP
        if size < 22 {
            return Err(ImError::ParserError(ImFormat::BMP));
        }
        let header_size = u32::from_le_bytes(get_array(&preamble[14..], ImFormat::BMP)?);
        if header_size == 12 {
            let w = u16::from_le_bytes(get_array(&preamble[18..], ImFormat::BMP)?);
            let h = u16::from_le_bytes(get_array(&preamble[20..], ImFormat::BMP)?);

            return Ok(ImInfo {
                format: ImFormat::BMP,
                width:  w as u64,
                height: h as u64,
            });
        } else {
            if size < 24 {
                return Err(ImError::ParserError(ImFormat::BMP));
            }
            let w = i32::from_le_bytes(get_array(&preamble[18..], ImFormat::BMP)?);
            let h = i32::from_le_bytes(get_array(&preamble[22..], ImFormat::BMP)?);

            return Ok(ImInfo {
                format: ImFormat::BMP,
                width:  w as u64,
                // h is negative when stored upside down
                height: h.abs() as u64
            });
        }
    } else if size >= 3 && &preamble[..2] == b"\xff\xd8" {
        // JPEG
        let err_conv = |_| ImError::ParserError(ImFormat::JPEG);
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(3)).map_err(err_conv)?;
        let mut buf1: [u8; 1] = [ preamble[2] ];
        let mut buf2: [u8; 2] = [0; 2];
        let mut buf4: [u8; 4] = [0; 4];
        while buf1[0] != b'\xda' && buf1[0] != 0 {
            while buf1[0] != b'\xff' {
                reader.read_exact(&mut buf1).map_err(err_conv)?;
            }
            while buf1[0] == b'\xff' {
                reader.read_exact(&mut buf1).map_err(err_conv)?;
            }
            if buf1[0] >= 0xc0 && buf1[0] <= 0xc3 {
                reader.seek(SeekFrom::Current(3)).map_err(err_conv)?;
                reader.read_exact(&mut buf4).map_err(err_conv)?;
                let h = u16::from_be_bytes([ buf4[0], buf4[1] ]);
                let w = u16::from_be_bytes([ buf4[2], buf4[3] ]);

                return Ok(ImInfo {
                    format: ImFormat::JPEG,
                    width:  w as u64,
                    height: h as u64,
                });
            }
            reader.read_exact(&mut buf2).map_err(err_conv)?;
            let b = u16::from_be_bytes(buf2);
            let offset = (b - 2) as i64;
            reader.seek(SeekFrom::Current(offset)).map_err(err_conv)?;
            reader.read_exact(&mut buf1).map_err(err_conv)?;
        }
        return Err(ImError::ParserError(ImFormat::JPEG));
    } else if preamble.starts_with(b"RIFF") && size >= 30 && &preamble[8..12] == b"WEBP" {
        // WEBP
        let hdr = &preamble[12..16];
        if hdr == b"VP8L" {
            let b0 = preamble[21];
            let b1 = preamble[22];
            let b2 = preamble[23];
            let b3 = preamble[24];

            let w = 1u32 + ((((b1 & 0x3F) as u32) << 8) | b0 as u32);
            let h = 1u32 + ((((b3 & 0xF) as u32) << 10) | ((b2 as u32) << 2) | ((b1 & 0xC0) as u32 >> 6));

            return Ok(ImInfo {
                format: ImFormat::WEBP,
                width:  w as u64,
                height: h as u64,
            });
        } else if hdr == b"VP8 " {
            let b0 = preamble[23];
            let b1 = preamble[24];
            let b2 = preamble[25];
            if b0 != 0x9d || b1 != 0x01 || b2 != 0x2a {
                return Err(ImError::ParserError(ImFormat::WEBP));
            }
            let w = u16::from_le_bytes(get_array(&preamble[26..], ImFormat::WEBP)?);
            let h = u16::from_le_bytes(get_array(&preamble[28..], ImFormat::WEBP)?);
            return Ok(ImInfo {
                format: ImFormat::WEBP,
                width:  w as u64 & 0x3ffff,
                height: h as u64 & 0x3ffff,
            });
        }
        return Err(ImError::ParserError(ImFormat::WEBP));
    } else if size >= 12 && &preamble[4..12] == b"ftypavif" {
        // AVIF
        let err_conv = |_| ImError::ParserError(ImFormat::AVIF);

        let ftype_size = u32::from_be_bytes(get_array(&preamble, ImFormat::AVIF)?);
        if ftype_size < 12 {
            return Err(ImError::ParserError(ImFormat::AVIF));
        }
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(ftype_size as u64)).map_err(err_conv)?;

        // chunk nesting: meta > iprp > ipco > ispe
        let chunk_size = find_avif_chunk(&mut reader, b"meta", u64::MAX)?;
        if chunk_size < 12 {
            return Err(ImError::ParserError(ImFormat::AVIF));
        }
        reader.seek(SeekFrom::Current(4)).map_err(err_conv)?;
        let chunk_size = find_avif_chunk(&mut reader, b"iprp", chunk_size - 12)?;
        let chunk_size = find_avif_chunk(&mut reader, b"ipco", chunk_size - 8)?;
        let chunk_size = find_avif_chunk(&mut reader, b"ispe", chunk_size - 8)?;

        if chunk_size < 12 {
            return Err(ImError::ParserError(ImFormat::AVIF));
        }

        let mut buf = [0u8; 12];
        reader.read_exact(&mut buf).map_err(err_conv)?;

        let w = u32::from_be_bytes(get_array(&buf[4..], ImFormat::AVIF)?);
        let h = u32::from_be_bytes(get_array(&buf[8..], ImFormat::AVIF)?);

        return Ok(ImInfo {
            format: ImFormat::GIF,
            width:  w as u64,
            height: h as u64,
        });
    } else if preamble.starts_with(b"qoif") && size >= 14 {
        // QOI
        let w = u32::from_be_bytes(get_array(&preamble[4..], ImFormat::QOI)?);
        let h = u32::from_be_bytes(get_array(&preamble[8..], ImFormat::QOI)?);

        return Ok(ImInfo {
            format: ImFormat::QOI,
            width:  w as u64,
            height: h as u64,
        });
    } else if preamble.starts_with(b"8BPS\0\x01\0\0\0\0\0\0") && size >= 22 {
        // PSD
        let h = u32::from_be_bytes(get_array(&preamble[14..], ImFormat::PSD)?);
        let w = u32::from_be_bytes(get_array(&preamble[18..], ImFormat::PSD)?);

        return Ok(ImInfo {
            format: ImFormat::PSD,
            width:  w as u64,
            height: h as u64,
        });
    } else if preamble.starts_with(b"gimp xcf ") && size >= 22 && preamble[13] == 0 {
        // XCF
        let w = u32::from_be_bytes(get_array(&preamble[14..], ImFormat::XCF)?);
        let h = u32::from_be_bytes(get_array(&preamble[18..], ImFormat::XCF)?);

        return Ok(ImInfo {
            format: ImFormat::XCF,
            width:  w as u64,
            height: h as u64,
        });
    } else if preamble.starts_with(b"\0\0\x01\0") && size >= 6 {
        // ICO
        let err_conv = |_| ImError::ParserError(ImFormat::ICO);
        let count = u16::from_le_bytes(get_array(&preamble[4..], ImFormat::ICO)?);
        file.seek(SeekFrom::Start(6)).map_err(err_conv)?;

        let mut buf = [0u8; 16];
        let mut width:  u32 = 0;
        let mut height: u32 = 0;
        for _ in 0..count {
            file.read_exact(&mut buf).map_err(err_conv)?;
            let w = buf[0] as u32;
            let h = buf[1] as u32;
            if w >= width && h >= height {
                width  = w;
                height = h;
            }
        }

        return Ok(ImInfo {
            format: ImFormat::ICO,
            width:  width  as u64,
            height: height as u64,
        });
    }
    // TODO: AVIF and TIFF
    return Err(ImError::UnknownFormat);
}
