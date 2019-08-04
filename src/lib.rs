#[macro_use]
extern crate structure;
use std::fs::File;
use std::io::{Read,Seek,SeekFrom};


#[derive(Debug)]
pub struct ImInfo {
    width: i64,
    height: i64,
    format: String,
}


pub fn imsz(fname: &str) -> ImInfo {
    let mut info = ImInfo { width: 0, height: 0, format: String::from("unknown")};

    let mut file = File::open(fname).unwrap();
    let mut preamble = [0u8; 26];

    file.read(&mut preamble).unwrap();

    if preamble[..6] == *b"GIF87a" || preamble[..6] == *b"GIF89a" {
        info.format = "gif".to_string();
        let s = structure!("<HH");
        let (w, h): (u16, u16) = s.unpack(&preamble[6..10]).unwrap();
        info.width = w.into(); 
        info.height = h.into(); 
    } else if preamble[..8] == *b"\x89PNG\r\n\x1a\n" {
        info.format = "png".to_string();
        let s = structure!(">II");
        if preamble[12..16] == *b"IHDR" {
            let (w, h): (u32, u32) = s.unpack(&preamble[16..24]).unwrap();
            info.width = w.into(); 
            info.height = h.into(); 
        } else {
            let (w, h): (u32, u32) = s.unpack(&preamble[8..16]).unwrap();
            info.width = w.into(); 
            info.height = h.into(); 
        }
    } else if preamble[..2] == *b"BM" {
        info.format = "bmp".to_string();
        let s = structure!("<I");
        let header_size: u32 = s.unpack(&preamble[14..18]).unwrap().0;
        if header_size == 12 {
            let s = structure!("<HH");
            let (w, h): (u16, u16) = s.unpack(&preamble[18..22]).unwrap();
            info.width = w.into(); 
            info.height = h.into(); 
        } else {
            let s = structure!("<ii");
            let (w, h): (i32, i32) = s.unpack(&preamble[18..26]).unwrap();
            info.width = w.into(); 
            // h is negative when stored upside down
            info.height = h.abs().into(); 
        }
    } else if preamble[..2] == *b"\xff\xd8" {
        info.format = "jpeg".to_string();
        let _ = file.seek(SeekFrom::Start(2));
        let mut buf1: [u8; 1] = [0]; 
        let mut buf2: [u8; 2] = [0; 2]; 
        let mut buf4: [u8; 4] = [0; 4]; 
        file.read(&mut buf1).unwrap();
        while buf1[0] != b'\xda' && buf1[0] != 0 {
            while buf1[0] != b'\xff' {
                file.read(&mut buf1).unwrap();
            }
            while buf1[0] == b'\xff' {
                file.read(&mut buf1).unwrap();
            }
            if buf1[0] >= 0xc0 && buf1[0] <= 0xc3 {
                let _ = file.seek(SeekFrom::Current(3));
                let s = structure!(">HH");
                file.read(&mut buf4).unwrap();
                let (w, h): (u16, u16) = s.unpack(&buf4).unwrap();
                info.width = w.into(); 
                info.height = h.into(); 
                break;
            }
            file.read(&mut buf2).unwrap();
            let s = structure!(">H");
            let b: u16 = s.unpack(&buf2).unwrap().0;
            let offset: i64 = (b - 2).into();
            let _ = file.seek(SeekFrom::Current(offset));
            file.read(&mut buf1).unwrap();
        }
    }
    return info
}