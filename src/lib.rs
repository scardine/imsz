use std::convert::TryInto;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, BufReader};

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum ImFormat {
    GIF     =  1,
    PNG     =  2,
    BMP     =  3,
    JPEG    =  4,
    WEBP    =  5,
    QOI     =  6,
    PSD     =  7,
    XCF     =  8,
    ICO     =  9,
    AVIF    = 10,
    TIFF    = 11,
    OpenEXR = 12,
}

impl ImFormat {
    pub fn name(&self) -> &'static str {
        match self {
            Self::GIF     => "gif",
            Self::PNG     => "png",
            Self::BMP     => "bmp",
            Self::JPEG    => "jpeg",
            Self::WEBP    => "webp",
            Self::QOI     => "qoi",
            Self::PSD     => "psd",
            Self::XCF     => "xcf",
            Self::ICO     => "ico",
            Self::AVIF    => "avif",
            Self::TIFF    => "tiff",
            Self::OpenEXR => "OpenEXR",
        }
    }
}

impl std::fmt::Display for ImFormat {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name().fmt(f)
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

trait Ratio<T: Sized> {
    fn value<R>(&self) -> R::Output
    where R: Sized, R: std::ops::Div, R: From<T>;
}

impl Ratio<u32> for (u32, u32) {
    fn value<R>(&self) -> R::Output
    where R: Sized, R: std::ops::Div, R: From<u32> {
        let (a, b) = *self;
        let x: R = a.into();
        let y: R = b.into();
        x / y
    }
}

impl Ratio<i32> for (i32, i32) {
    fn value<R>(&self) -> R::Output
    where R: Sized, R: std::ops::Div, R: From<i32> {
        let (a, b) = *self;
        let x: R = a.into();
        let y: R = b.into();
        x / y
    }
}

trait BinaryReader {
    #[inline]
    fn read_u8(reader: &mut impl Read) -> std::io::Result<u8> {
        let mut buf = [0u8];
        reader.read_exact(&mut buf)?;
        return Ok(buf[0]);
    }

    #[inline]
    fn read_uchar(reader: &mut impl Read) -> std::io::Result<u8> {
        let mut buf = [0u8];
        reader.read_exact(&mut buf)?;
        return Ok(buf[0]);
    }

    #[inline]
    fn read_i8(reader: &mut impl Read) -> std::io::Result<i8> {
        let mut buf = [0u8];
        reader.read_exact(&mut buf)?;
        return Ok(buf[0] as i8);
    }

    #[inline]
    fn read_ichar(reader: &mut impl Read) -> std::io::Result<i8> {
        let mut buf = [0u8];
        reader.read_exact(&mut buf)?;
        return Ok(buf[0] as i8);
    }

    fn get_u32(buf: [u8; 4]) -> u32;

    fn read_u16(reader: &mut impl Read) -> std::io::Result<u16>;
    fn read_u32(reader: &mut impl Read) -> std::io::Result<u32>;
    fn read_uratio(reader: &mut impl Read) -> std::io::Result<(u32, u32)>;

    fn read_i16(reader: &mut impl Read) -> std::io::Result<i16>;
    fn read_i32(reader: &mut impl Read) -> std::io::Result<i32>;
    fn read_iratio(reader: &mut impl Read) -> std::io::Result<(i32, i32)>;

    fn read_f32(reader: &mut impl Read) -> std::io::Result<f32>;
    fn read_f64(reader: &mut impl Read) -> std::io::Result<f64>;
}

struct LittleEndianReader;
struct BigEndianReader;

impl BinaryReader for LittleEndianReader {
    #[inline]
    fn get_u32(buf: [u8; 4]) -> u32 {
        return u32::from_le_bytes(buf);
    }

    #[inline]
    fn read_u16(reader: &mut impl Read) -> std::io::Result<u16> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        return Ok(u16::from_le_bytes(buf));
    }

    #[inline]
    fn read_u32(reader: &mut impl Read) -> std::io::Result<u32> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        return Ok(u32::from_le_bytes(buf));
    }

    #[inline]
    fn read_uratio(reader: &mut impl Read) -> std::io::Result<(u32, u32)> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        return Ok((
            u32::from_le_bytes([ buf[0], buf[1], buf[2], buf[3] ]),
            u32::from_le_bytes([ buf[4], buf[5], buf[6], buf[7] ]),
        ));
    }

    #[inline]
    fn read_i16(reader: &mut impl Read) -> std::io::Result<i16> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        return Ok(i16::from_le_bytes(buf));
    }

    #[inline]
    fn read_i32(reader: &mut impl Read) -> std::io::Result<i32> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        return Ok(i32::from_le_bytes(buf));
    }

    #[inline]
    fn read_iratio(reader: &mut impl Read) -> std::io::Result<(i32, i32)> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        return Ok((
            i32::from_le_bytes([ buf[0], buf[1], buf[2], buf[3] ]),
            i32::from_le_bytes([ buf[4], buf[5], buf[6], buf[7] ]),
        ));
    }

    #[inline]
    fn read_f32(reader: &mut impl Read) -> std::io::Result<f32> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        return Ok(f32::from_le_bytes(buf));
    }

    #[inline]
    fn read_f64(reader: &mut impl Read) -> std::io::Result<f64> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        return Ok(f64::from_le_bytes(buf));
    }
}

impl BinaryReader for BigEndianReader {
    #[inline]
    fn get_u32(buf: [u8; 4]) -> u32 {
        return u32::from_be_bytes(buf);
    }

    #[inline]
    fn read_u16(reader: &mut impl Read) -> std::io::Result<u16> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        return Ok(u16::from_be_bytes(buf));
    }

    #[inline]
    fn read_u32(reader: &mut impl Read) -> std::io::Result<u32> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        return Ok(u32::from_be_bytes(buf));
    }

    #[inline]
    fn read_uratio(reader: &mut impl Read) -> std::io::Result<(u32, u32)> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        return Ok((
            u32::from_be_bytes([ buf[0], buf[1], buf[2], buf[3] ]),
            u32::from_be_bytes([ buf[4], buf[5], buf[6], buf[7] ]),
        ));
    }

    #[inline]
    fn read_i16(reader: &mut impl Read) -> std::io::Result<i16> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        return Ok(i16::from_be_bytes(buf));
    }

    #[inline]
    fn read_i32(reader: &mut impl Read) -> std::io::Result<i32> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        return Ok(i32::from_be_bytes(buf));
    }

    #[inline]
    fn read_iratio(reader: &mut impl Read) -> std::io::Result<(i32, i32)> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        return Ok((
            i32::from_be_bytes([ buf[0], buf[1], buf[2], buf[3] ]),
            i32::from_be_bytes([ buf[4], buf[5], buf[6], buf[7] ]),
        ));
    }

    #[inline]
    fn read_f32(reader: &mut impl Read) -> std::io::Result<f32> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        return Ok(f32::from_be_bytes(buf));
    }

    #[inline]
    fn read_f64(reader: &mut impl Read) -> std::io::Result<f64> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        return Ok(f64::from_be_bytes(buf));
    }
}

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

macro_rules! map_err {
    ($fmt:ident $expr:expr) => {
        if let Err(_) = $expr {
            return Err(ImError::ParserError(ImFormat::$fmt));
        }
    };
}

macro_rules! map_expr {
    ($fmt:ident $expr:expr) => {
        match $expr {
            Err(_) => return Err(ImError::ParserError(ImFormat::$fmt)),
            Ok(value) => value
        }
    };
}

fn parse_tiff<BR, R>(file: &mut R, preamble: &[u8]) -> ImResult<ImInfo>
where BR: BinaryReader, R: Read, R: Seek {
    let mut reader = BufReader::new(file);

    let ifd_offset = BR::get_u32(get_array(&preamble[4..], ImFormat::TIFF)?);
    map_err!(TIFF reader.seek(SeekFrom::Start(ifd_offset as u64)));

    let ifd_entry_count = map_expr!(TIFF BR::read_u16(&mut reader)) as u32;
    // 2 bytes: TagId + 2 bytes: type + 4 bytes: count of values + 4
    // bytes: value offset
    let mut width:  Option<u64> = None;
    let mut height: Option<u64> = None;

    for index in 0..ifd_entry_count {
        // sizeof ifd_entry_count = 2
        let entry_offset = ifd_offset + 2 + index * 12;
        map_err!(TIFF reader.seek(SeekFrom::Start(entry_offset as u64)));
        let tag = map_expr!(TIFF BR::read_u16(&mut reader));

        // 256 ... width
        // 257 ... height
        if tag == 256 || tag == 257 {
            // if type indicates that value fits into 4 bytes, value
            // offset is not an offset but value itself
            let ftype = map_expr!(TIFF BR::read_u16(&mut reader));
            map_err!(TIFF reader.seek(SeekFrom::Start(entry_offset as u64 + 8)));
            let value: u64 = match ftype {
                 1 => map_expr!(TIFF BR::read_u8(&mut reader)).into(),
                 2 => map_expr!(TIFF BR::read_uchar(&mut reader)).into(),
                 3 => map_expr!(TIFF BR::read_u16(&mut reader)).into(),
                 4 => map_expr!(TIFF BR::read_u32(&mut reader)).into(),
                 5 => map_expr!(TIFF BR::read_uratio(&mut reader)).value::<u64>(),
                 6 => map_expr!(TIFF BR::read_i8(&mut reader)).max(0) as u64,
                 7 => map_expr!(TIFF BR::read_ichar(&mut reader)).max(0) as u64,
                 8 => map_expr!(TIFF BR::read_i16(&mut reader)).max(0) as u64,
                 9 => map_expr!(TIFF BR::read_i32(&mut reader)).max(0) as u64,
                10 => map_expr!(TIFF BR::read_iratio(&mut reader)).value::<i64>().max(0) as u64,
                11 => map_expr!(TIFF BR::read_f32(&mut reader)) as u64,
                12 => map_expr!(TIFF BR::read_f64(&mut reader)) as u64,
                _ => return Err(ImError::ParserError(ImFormat::TIFF))
            };

            if tag == 256 {
                if let Some(height) = height {
                    return Ok(ImInfo {
                        format: ImFormat::TIFF,
                        width: value,
                        height,
                    });
                }
                width = Some(value);
            } else {
                if let Some(width) = width {
                    return Ok(ImInfo {
                        format: ImFormat::TIFF,
                        width,
                        height: value,
                    });
                }
                height = Some(value);
            }
        }
    }

    return Err(ImError::ParserError(ImFormat::TIFF));
}

#[inline]
pub fn imsz(fname: impl AsRef<std::path::Path>) -> ImResult<ImInfo> {
    let mut file = File::open(fname)?;
    return imsz_from_reader(&mut file);
}

pub fn imsz_from_reader<R>(file: &mut R) -> ImResult<ImInfo>
where R: Read, R: Seek {
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
        let mut reader = BufReader::new(file);
        map_err!(JPEG reader.seek(SeekFrom::Start(3)));
        let mut buf1: [u8; 1] = [ preamble[2] ];
        let mut buf2: [u8; 2] = [0; 2];
        let mut buf4: [u8; 4] = [0; 4];
        while buf1[0] != b'\xda' && buf1[0] != 0 {
            while buf1[0] != b'\xff' {
                map_err!(JPEG reader.read_exact(&mut buf1));
            }
            while buf1[0] == b'\xff' {
                map_err!(JPEG reader.read_exact(&mut buf1));
            }
            if buf1[0] >= 0xc0 && buf1[0] <= 0xc3 {
                map_err!(JPEG reader.seek(SeekFrom::Current(3)));
                map_err!(JPEG reader.read_exact(&mut buf4));
                let h = u16::from_be_bytes([ buf4[0], buf4[1] ]);
                let w = u16::from_be_bytes([ buf4[2], buf4[3] ]);

                return Ok(ImInfo {
                    format: ImFormat::JPEG,
                    width:  w as u64,
                    height: h as u64,
                });
            }
            map_err!(JPEG reader.read_exact(&mut buf2));
            let b = u16::from_be_bytes(buf2);
            let offset = (b - 2) as i64;
            map_err!(JPEG reader.seek(SeekFrom::Current(offset)));
            map_err!(JPEG reader.read_exact(&mut buf1));
        }
        return Err(ImError::ParserError(ImFormat::JPEG));
    } else if size >= 30 && preamble.starts_with(b"RIFF") && &preamble[8..12] == b"WEBP" {
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
        } else if hdr == b"VP8X" {
            let w1 = preamble[24] as u32;
            let w2 = preamble[25] as u32;
            let w3 = preamble[26] as u32;
            let h1 = preamble[27] as u32;
            let h2 = preamble[28] as u32;
            let h3 = preamble[29] as u32;

            let width  = (w1 | w2 << 8 | w3 << 16) + 1;
            let height = (h1 | h2 << 8 | h3 << 16) + 1;

            return Ok(ImInfo {
                format: ImFormat::WEBP,
                width:  width  as u64,
                height: height as u64,
            });
        }
        return Err(ImError::ParserError(ImFormat::WEBP));
    } else if size >= 12 && &preamble[4..12] == b"ftypavif" {
        // AVIF
        let ftype_size = u32::from_be_bytes(get_array(&preamble, ImFormat::AVIF)?);
        if ftype_size < 12 {
            return Err(ImError::ParserError(ImFormat::AVIF));
        }
        let mut reader = BufReader::new(file);
        map_err!(AVIF reader.seek(SeekFrom::Start(ftype_size as u64)));

        // chunk nesting: meta > iprp > ipco > ispe
        let chunk_size = find_avif_chunk(&mut reader, b"meta", u64::MAX)?;
        if chunk_size < 12 {
            return Err(ImError::ParserError(ImFormat::AVIF));
        }
        map_err!(AVIF reader.seek(SeekFrom::Current(4)));
        let chunk_size = find_avif_chunk(&mut reader, b"iprp", chunk_size - 12)?;
        let chunk_size = find_avif_chunk(&mut reader, b"ipco", chunk_size - 8)?;
        let chunk_size = find_avif_chunk(&mut reader, b"ispe", chunk_size - 8)?;

        if chunk_size < 12 {
            return Err(ImError::ParserError(ImFormat::AVIF));
        }

        let mut buf = [0u8; 12];
        map_err!(AVIF reader.read_exact(&mut buf));

        let w = u32::from_be_bytes(get_array(&buf[4..], ImFormat::AVIF)?);
        let h = u32::from_be_bytes(get_array(&buf[8..], ImFormat::AVIF)?);

        return Ok(ImInfo {
            format: ImFormat::GIF,
            width:  w as u64,
            height: h as u64,
        });
    } else if size >= 8 && (preamble.starts_with(b"II*\0") || preamble.starts_with(b"MM\0*")) {
        // TIFF
        if preamble.starts_with(b"MM") {
            // big endian
            return parse_tiff::<BigEndianReader, R>(file, &preamble[..size]);
        } else {
            // little endian
            return parse_tiff::<LittleEndianReader, R>(file, &preamble[..size]);
        }
    } else if size >= 14 && preamble.starts_with(b"qoif") {
        // QOI
        let w = u32::from_be_bytes(get_array(&preamble[4..], ImFormat::QOI)?);
        let h = u32::from_be_bytes(get_array(&preamble[8..], ImFormat::QOI)?);

        return Ok(ImInfo {
            format: ImFormat::QOI,
            width:  w as u64,
            height: h as u64,
        });
    } else if size >= 22 && preamble.starts_with(b"8BPS\0\x01\0\0\0\0\0\0") {
        // PSD
        let h = u32::from_be_bytes(get_array(&preamble[14..], ImFormat::PSD)?);
        let w = u32::from_be_bytes(get_array(&preamble[18..], ImFormat::PSD)?);

        return Ok(ImInfo {
            format: ImFormat::PSD,
            width:  w as u64,
            height: h as u64,
        });
    } else if size >= 22 && preamble.starts_with(b"gimp xcf ") && preamble[13] == 0 {
        // XCF
        let w = u32::from_be_bytes(get_array(&preamble[14..], ImFormat::XCF)?);
        let h = u32::from_be_bytes(get_array(&preamble[18..], ImFormat::XCF)?);

        return Ok(ImInfo {
            format: ImFormat::XCF,
            width:  w as u64,
            height: h as u64,
        });
    } else if size >= 6 && preamble.starts_with(b"\0\0\x01\0") {
        // ICO
        let count = u16::from_le_bytes(get_array(&preamble[4..], ImFormat::ICO)?);
        map_err!(ICO file.seek(SeekFrom::Start(6)));

        let mut buf = [0u8; 16];
        let mut width:  u32 = 0;
        let mut height: u32 = 0;
        for _ in 0..count {
            map_err!(ICO file.read_exact(&mut buf));
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
    } else if size > 8 && preamble.starts_with(b"\x76\x2f\x31\x01") && (preamble[4] == 0x01 || preamble[4] == 0x02) {
        // OpenEXR
        let mut reader = BufReader::new(file);
        map_err!(OpenEXR reader.seek(SeekFrom::Start(8)));

        let mut name_buf = Vec::new();
        let mut type_buf = Vec::new();
        let mut buf1 = [0u8];
        let mut buf4 = [0u8; 4];

        loop {
            name_buf.clear();
            loop {
                map_err!(OpenEXR reader.read_exact(&mut buf1));
                let byte = buf1[0];
                if byte == 0 {
                    break;
                }
                name_buf.push(byte);
            }

            if name_buf.is_empty() {
                break;
            }

            type_buf.clear();
            loop {
                map_err!(OpenEXR reader.read_exact(&mut buf1));
                let byte = buf1[0];
                if byte == 0 {
                    break;
                }
                type_buf.push(byte);
            }

            map_err!(OpenEXR reader.read_exact(&mut buf4));
            let size = u32::from_le_bytes(buf4);

            if &name_buf == b"displayWindow" {
                if &type_buf != b"box2i" || size != 16 {
                    return Err(ImError::ParserError(ImFormat::OpenEXR));
                }

                let mut box_buf = [0u8; 16];
                map_err!(OpenEXR reader.read_exact(&mut box_buf));

                let x1 = i32::from_le_bytes(get_array(&box_buf,       ImFormat::OpenEXR)?) as i64;
                let y1 = i32::from_le_bytes(get_array(&box_buf[ 4..], ImFormat::OpenEXR)?) as i64;
                let x2 = i32::from_le_bytes(get_array(&box_buf[ 8..], ImFormat::OpenEXR)?) as i64;
                let y2 = i32::from_le_bytes(get_array(&box_buf[12..], ImFormat::OpenEXR)?) as i64;

                let width  = x2 - x1 + 1;
                let height = y2 - y1 + 1;

                if width <= 0 || height <= 0 {
                    map_err!(OpenEXR reader.seek(SeekFrom::Current(size as i64)));
                }

                return Ok(ImInfo {
                    format: ImFormat::OpenEXR,
                    width:  width  as u64,
                    height: height as u64,
                });
            } else {
                map_err!(OpenEXR reader.seek(SeekFrom::Current(size as i64)));
            }
        }

        return Err(ImError::ParserError(ImFormat::OpenEXR));
    }
    return Err(ImError::UnknownFormat);
}
