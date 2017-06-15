extern crate lcms2;
use lcms2::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct RGB(u8, u8, u8);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct RGBA {
    r: u16, g: u16, b: u16, a:u16
}

fn main() {
    // Standard profiles are built-in
    let srgb_profile = Profile::new_srgb();

    // Custom profiles need to be built from primaries (e.g. cHRM PNG chunk), white point, and gamma
    let custom_primaries = CIExyYTRIPLE{
        Red:   CIExyY{x:0.630, y:0.340, Y:1.0},
        Green: CIExyY{x:0.310, y:0.595, Y:1.0},
        Blue:  CIExyY{x:0.155, y:0.070, Y:1.0},
    };

    let custom_gamma = &ToneCurve::new(1./0.4545455);
    let custom_profile = Profile::new_rgb(CIExyY::d50(), &custom_primaries, &[custom_gamma, custom_gamma, custom_gamma]).unwrap();

    // Applies the profiles
    let t = Transform::new(&custom_profile, PixelFormat::RGB_8, &srgb_profile, PixelFormat::RGBA_16, Intent::Perceptual).unwrap();

    // Slices must contain pixels (not bytes), i.e. struct RGB, not Vec<u8>
    // and the pixels must have repr(C) layout compatible with the PixelFormat in Transform::new()
    let source_pixels = &[RGB(0,100,254)];
    let mut dest_pixels = vec![RGBA{r:0,g:0,b:0,a:0}];
    t.transform_pixels(source_pixels, &mut dest_pixels);

    assert_eq!(RGBA{r:0, g: 25996, b: 64510, a:0}, dest_pixels[0]);

    // Profile can be saved as ICC file
    let _ = custom_profile.icc();
}

