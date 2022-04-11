use std::convert::TryInto;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};


#[derive(Debug, Clone)]
pub struct ImInfo {
    pub width:  u64,
    pub height: u64,
    pub format: &'static str,
}

#[derive(Debug)]
pub enum ImError {
    IO(std::io::Error),
    UnknownFormat,
    ParserError(&'static str)
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
fn get_array<const LEN: usize>(slice: &[u8], format: &'static str) -> ImResult<[u8; LEN]> {
    match slice[..LEN].try_into() {
        Ok(array) => Ok(array),
        Err(_) => Err(ImError::ParserError(format)),
    }
}

pub fn imsz(fname: &str) -> ImResult<ImInfo> {
    let mut file = File::open(fname)?;
    let mut preamble = [0u8; 30];

    let size = file.read(&mut preamble)?;

    if size >= 6 && (&preamble[..6] == b"GIF87a" || &preamble[..6] == b"GIF89a") {
        // GIF
        if size < 10 {
            return Err(ImError::ParserError("gif"));
        }
        let w = u16::from_le_bytes(get_array(&preamble[6..], "gif")?);
        let h = u16::from_le_bytes(get_array(&preamble[8..], "gif")?);

        return Ok(ImInfo {
            format: "gif",
            width:  w.into(),
            height: h.into()
        });
    } else if size >= 16 && &preamble[..8] == b"\x89PNG\r\n\x1a\n" {
        // PNG
        let w;
        let h;
        if &preamble[12..16] == b"IHDR" {
            if size < 24 {
                return Err(ImError::ParserError("png"));
            }
            w = u32::from_be_bytes(get_array(&preamble[16..], "png")?);
            h = u32::from_be_bytes(get_array(&preamble[20..], "png")?);
        } else {
            w = u32::from_be_bytes(get_array(&preamble[ 8..], "png")?);
            h = u32::from_be_bytes(get_array(&preamble[12..], "png")?);
        }

        return Ok(ImInfo {
            format: "png",
            width:  w.into(),
            height: h.into()
        });
    } else if size >= 10 && (&preamble[..2] == b"BM" && &preamble[6..10] == b"\0\0\0\0") {
        // BMP
        if size < 22 {
            return Err(ImError::ParserError("bmp"));
        }
        let header_size = u32::from_le_bytes(get_array(&preamble[14..], "bmp")?);
        if header_size == 12 {
            let w = u16::from_le_bytes(get_array(&preamble[18..], "bmp")?);
            let h = u16::from_le_bytes(get_array(&preamble[20..], "bmp")?);

            return Ok(ImInfo {
                format: "bmp",
                width:  w.into(),
                height: h.into()
            });
        } else {
            if size < 24 {
                return Err(ImError::ParserError("bmp"));
            }
            let w = i32::from_le_bytes(get_array(&preamble[18..], "bmp")?);
            let h = i32::from_le_bytes(get_array(&preamble[22..], "bmp")?);

            return Ok(ImInfo {
                format: "bmp",
                width:  w as u64,
                // h is negative when stored upside down
                height: h.abs() as u64
            });
        }
    } else if size >= 3 && &preamble[..2] == b"\xff\xd8" {
        // JPEG
        let err_conv = |_| ImError::ParserError("jpeg");
        file.seek(SeekFrom::Start(3)).map_err(err_conv)?;
        let mut buf1: [u8; 1] = [ preamble[2] ];
        let mut buf2: [u8; 2] = [0; 2];
        let mut buf4: [u8; 4] = [0; 4];
        while buf1[0] != b'\xda' && buf1[0] != 0 {
            while buf1[0] != b'\xff' {
                file.read_exact(&mut buf1).map_err(err_conv)?;
            }
            while buf1[0] == b'\xff' {
                file.read_exact(&mut buf1).map_err(err_conv)?;
            }
            if buf1[0] >= 0xc0 && buf1[0] <= 0xc3 {
                file.seek(SeekFrom::Current(3)).map_err(err_conv)?;
                file.read_exact(&mut buf4).map_err(err_conv)?;
                let h = u16::from_be_bytes([ buf4[0], buf4[1] ]);
                let w = u16::from_be_bytes([ buf4[2], buf4[3] ]);

                return Ok(ImInfo {
                    format: "jpeg",
                    width:  w.into(),
                    height: h.into()
                });
            }
            file.read_exact(&mut buf2).map_err(err_conv)?;
            let b = u16::from_be_bytes(buf2);
            let offset: i64 = (b - 2).into();
            file.seek(SeekFrom::Current(offset)).map_err(err_conv)?;
            file.read_exact(&mut buf1).map_err(err_conv)?;
        }
        return Err(ImError::ParserError("jpeg"));
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
                format: "webp",
                width:  w as u64,
                height: h as u64,
            });
        } else if hdr == b"VP8 " {
            let b0 = preamble[23];
            let b1 = preamble[24];
            let b2 = preamble[25];
            if b0 != 0x9d || b1 != 0x01 || b2 != 0x2a {
                return Err(ImError::ParserError("webp"));
            }
            let w = u16::from_le_bytes(get_array(&preamble[26..], "webp")?);
            let h = u16::from_le_bytes(get_array(&preamble[28..], "webp")?);
            return Ok(ImInfo {
                format: "webp",
                width:  w as u64 & 0x3ffff,
                height: h as u64 & 0x3ffff,
            });
        }
        return Err(ImError::ParserError("webp"));
    } else if preamble.starts_with(b"qoif") && size >= 14 {
        // QOI
        let w = u32::from_be_bytes(get_array(&preamble[4..], "qoi")?);
        let h = u32::from_be_bytes(get_array(&preamble[8..], "qoi")?);

        return Ok(ImInfo {
            format: "qoi",
            width:  w as u64,
            height: h as u64,
        });
    } else if preamble.starts_with(b"8BPS\0\x01\0\0\0\0\0\0") && size >= 22 {
        // PSD
        let h = u32::from_be_bytes(get_array(&preamble[14..], "psd")?);
        let w = u32::from_be_bytes(get_array(&preamble[18..], "psd")?);

        return Ok(ImInfo {
            format: "psd",
            width:  w as u64,
            height: h as u64,
        });
    } else if preamble.starts_with(b"gimp xcf ") && size >= 22 && preamble[13] == 0 {
        // XCF
        let w = u32::from_be_bytes(get_array(&preamble[14..], "psd")?);
        let h = u32::from_be_bytes(get_array(&preamble[18..], "psd")?);

        return Ok(ImInfo {
            format: "xcf",
            width:  w as u64,
            height: h as u64,
        });
    } else if preamble.starts_with(b"\0\0\x01\0") && size >= 6 {
        // ICO
        let err_conv = |_| ImError::ParserError("ico");
        let count = u16::from_le_bytes(get_array(&preamble[4..], "ico")?);
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
            format: "ico",
            width:  width  as u64,
            height: height as u64,
        });
    }
    // TODO: AVIF and TIFF
    return Err(ImError::UnknownFormat);
}
