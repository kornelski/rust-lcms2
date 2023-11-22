#[test]
fn test1() {
    let target_profile = lcms2::Profile::new_srgb();
    let src_profile = lcms2::Profile::new_srgb();
    let icc_pixel_format = lcms2::PixelFormat::RGBA_8;

    let transform = lcms2::Transform::<u8, u8>::new(
        &src_profile,
        icc_pixel_format,
        &target_profile,
        icc_pixel_format,
        lcms2::Intent::Perceptual,
    )
    .unwrap();

    let _ = transform.input_pixel_format();
}

#[test]
fn test2() {
    let src_profile =
        lcms2::Profile::new_gray(lcms2_sys::ffi::CIExyY::d50(), &lcms2::ToneCurve::new(2.2))
            .unwrap();

    let target_profile =
        lcms2::Profile::new_gray(lcms2_sys::ffi::CIExyY::d50(), &lcms2::ToneCurve::new(2.2))
            .unwrap();

    let icc_pixel_format = lcms2::PixelFormat::GRAY_16;

    let transform = lcms2::Transform::<u8, u8>::new(
        &src_profile,
        icc_pixel_format,
        &target_profile,
        icc_pixel_format,
        lcms2::Intent::Perceptual,
    )
    .unwrap();

    let _ = transform.input_pixel_format();
}
