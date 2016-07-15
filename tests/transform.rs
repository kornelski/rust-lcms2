extern crate lcms2;

use lcms2::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct RGB16 {
    r: u16,
    g: u16,
    b: u16,
}

#[test]
fn transform() {
    const PROFILE: &'static [u8] = include_bytes!("tinysrgb.icc");
    let tiny = Profile::new_icc(PROFILE);
    assert_eq!(ColorSpaceSignature::SigRgbData, tiny.color_space());
    assert_eq!("c2", tiny.info(InfoType::Description, "en", "us").unwrap());
    assert_eq!(2.1, tiny.version());

    assert!(tiny.tag_signatures().contains(&TagSignature::SigGreenColorantTag));

    let srgb = Profile::new_srgb();
    assert_eq!(ColorSpaceSignature::SigRgbData, srgb.color_space());

    let tiny2 = tiny.icc().unwrap();
    let tiny2 = Profile::new_icc(&tiny2);

    let tr = Transform::new_flags(&tiny, PixelFormat::RGBA_8, &tiny2, PixelFormat::RGB_16, Intent::Perceptual, 0);
    let src = vec![0xFFFFFFFFu32,0,0x7F7F7F7F,0x10101010];
    let mut dst = vec![RGB16{r:0,g:1,b:2}; 4];
    tr.transform_pixels(&src, &mut dst);
    assert_eq!(vec![
        RGB16{r:0xFFFF,g:0xFFFF,b:0xFFFF},
        RGB16{r:0,g:0,b:0},
        RGB16{r:0x7F7F,g:0x7F7F,b:0x7F7F},
        RGB16{r:0x1010,g:0x1010,b:0x1010},
    ], dst);

    let tr = Transform::new(&tiny2, PixelFormat::RGB_16, &tiny, PixelFormat::RGB_16, Intent::Perceptual);
    tr.transform_in_place(&mut dst);
    assert_eq!(vec![
        RGB16{r:0xFFFF,g:0xFFFF,b:0xFFFF},
        RGB16{r:0,g:0,b:0},
        RGB16{r:0x7F7F,g:0x7F7F,b:0x7F7F},
        RGB16{r:0x1010,g:0x1010,b:0x1010},
    ], dst);
}
