
fn get_testdata(fname: &str) -> std::path::PathBuf {
    let mut path = std::env::current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().to_owned();
    path.push("testdata");
    path.push(fname);
    return path;
}


#[test]
fn image_avif() {
    let info = imsz::imsz(get_testdata("image.avif"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::AVIF);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_bmp() {
    let info = imsz::imsz(get_testdata("image.bmp"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::BMP);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_dds() {
    let info = imsz::imsz(get_testdata("image.dds"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::DDS);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_exr() {
    let info = imsz::imsz(get_testdata("image.exr"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::OpenEXR);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_gif() {
    let info = imsz::imsz(get_testdata("image.gif"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::GIF);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_heic() {
    let info = imsz::imsz(get_testdata("image.heic"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::HEIC);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_ico() {
    let info = imsz::imsz(get_testdata("image.ico"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::ICO);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_jpeg() {
    let info = imsz::imsz(get_testdata("image.jpeg"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::JPEG);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_pcx() {
    let info = imsz::imsz(get_testdata("image.pcx"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::PCX);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_png() {
    let info = imsz::imsz(get_testdata("image.png"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::PNG);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_psd() {
    let info = imsz::imsz(get_testdata("image.psd"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::PSD);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_qoi() {
    let info = imsz::imsz(get_testdata("image.qoi"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::QOI);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_tga() {
    let info = imsz::imsz(get_testdata("image.tga"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::TGA);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_tiff() {
    let info = imsz::imsz(get_testdata("image.tiff"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::TIFF);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_xcf() {
    let info = imsz::imsz(get_testdata("image.xcf"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::XCF);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_lossless_webp() {
    let info = imsz::imsz(get_testdata("image_lossless.webp"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::WEBP);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_lossless_vp8x_webp() {
    let info = imsz::imsz(get_testdata("image_lossless_vp8x.webp"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::WEBP);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_lossy_webp() {
    let info = imsz::imsz(get_testdata("image_lossy.webp"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::WEBP);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}


#[test]
fn image_lossy_vp8x_webp() {
    let info = imsz::imsz(get_testdata("image_lossy_vp8x.webp"));
    match info {
        Ok(info) => {
            assert_eq!(info.format, imsz::ImFormat::WEBP);
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }
        Err(error) => {
            assert!(false, "{}", error);
        }
    }
}
