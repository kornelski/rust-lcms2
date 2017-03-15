extern crate lcms2;

use lcms2::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct RGB16 {
    r: u16,
    g: u16,
    b: u16,
}

#[test]
#[should_panic]
fn no_transform() {
    let srgb = Profile::new_srgb();
    let tr = Transform::new(&srgb, PixelFormat::RGB_16, &srgb, PixelFormat::RGB_16, Intent::Perceptual).unwrap();
    tr.transform_in_place(&mut [0u8; 6]);
}

#[test]
#[should_panic]
fn cmyk_rgb() {
    let srgb = Profile::new_srgb();
    let tr = Transform::new(&srgb, PixelFormat::CMYK_8, &srgb, PixelFormat::CMYK_8, Intent::Perceptual).unwrap();
    tr.transform_in_place(&mut [0u32; 1]);
}

#[test]
#[should_panic]
fn gray() {
    let srgb = Profile::new_srgb();
    let tr = Transform::new(&srgb, PixelFormat::GRAY_8, &srgb, PixelFormat::GRAY_8, Intent::Perceptual).unwrap();
    tr.transform_in_place(&mut [0u32; 1]);
}

const GRAY_PROFILE: &'static [u8] = include_bytes!("gray18.icc");
const SGRAY_PROFILE: &'static [u8] = include_bytes!("sGray.icc");

#[test]
fn transform_gray_to_rgb() {
    let gray = Profile::new_icc(GRAY_PROFILE).unwrap();
    assert_eq!(ColorSpaceSignature::SigGrayData, gray.color_space());
    assert_eq!("Gray Gamma 1.8", gray.info(InfoType::Description, "en", "us").unwrap());

    let srgb = Profile::new_srgb();

    let tr = Transform::new(&gray, PixelFormat::GRAY_8, &srgb, PixelFormat::RGB_8, Intent::Perceptual).unwrap();
    let mut dest = vec![(0u8,0u8,0u8); 3];
    tr.transform_pixels(&[0u8,100u8,255u8], &mut dest);
    assert_eq!(&dest, &[
        (0,0,0),
        (119,119,119),
        (255,255,255),
    ]);
}

#[test]
fn transform_gray_to_gray() {
    let gray = Profile::new_icc(GRAY_PROFILE).unwrap();
    assert_eq!(ColorSpaceSignature::SigGrayData, gray.color_space());
    assert_eq!("Gray Gamma 1.8", gray.info(InfoType::Description, "en", "us").unwrap());

    let sg = Profile::new_icc(SGRAY_PROFILE).unwrap();

    let tr = Transform::new(&gray, PixelFormat::GRAY_8, &sg, PixelFormat::GRAY_8, Intent::Perceptual).unwrap();
    let mut dest = vec![0u8; 3];
    tr.transform_pixels(&[0u8,100u8,255u8], &mut dest);
    assert_eq!(&dest, &[
        0,
        119,
        255,
    ]);
}

#[test]
fn transform() {
    const PROFILE: &'static [u8] = include_bytes!("tinysrgb.icc");
    let tiny = Profile::new_icc(PROFILE).unwrap();
    assert_eq!(ColorSpaceSignature::SigRgbData, tiny.color_space());
    assert_eq!("c2", tiny.info(InfoType::Description, "en", "us").unwrap());
    assert!((2.1 - tiny.version()).abs() < std::f64::EPSILON);

    assert!(tiny.tag_signatures().contains(&TagSignature::SigGreenColorantTag));

    let srgb = Profile::new_srgb();
    assert_eq!(ColorSpaceSignature::SigRgbData, srgb.color_space());

    let tiny2 = tiny.icc().unwrap();
    let tiny2 = Profile::new_icc(&tiny2).unwrap();

    let tr = Transform::new_flags(&tiny, PixelFormat::RGBA_8, &tiny2, PixelFormat::RGB_16, Intent::Perceptual, 0).unwrap();
    let src = vec![0xFFFFFFFFu32,0,0x7F7F7F7F,0x10101010];
    let mut dst = vec![RGB16{r:0,g:1,b:2}; 4];
    tr.transform_pixels(&src, &mut dst);
    assert_eq!(vec![
        RGB16{r:0xFFFF,g:0xFFFF,b:0xFFFF},
        RGB16{r:0,g:0,b:0},
        RGB16{r:0x7F7F,g:0x7F7F,b:0x7F7F},
        RGB16{r:0x1010,g:0x1010,b:0x1010},
    ], dst);

    let tr = Transform::new(&tiny2, PixelFormat::RGB_16, &tiny, PixelFormat::RGB_16, Intent::Perceptual).unwrap();
    tr.transform_in_place(&mut dst);
    assert_eq!(vec![
        RGB16{r:0xFFFF,g:0xFFFF,b:0xFFFF},
        RGB16{r:0,g:0,b:0},
        RGB16{r:0x7F7F,g:0x7F7F,b:0x7F7F},
        RGB16{r:0x1010,g:0x1010,b:0x1010},
    ], dst);
}
