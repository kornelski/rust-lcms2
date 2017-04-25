# [Little CMS](http://www.littlecms.com) wrapper for [Rust](http://www.rust-lang.org/)

Convert and apply color profiles with a safe abstraction layer for the LCMS library.

```rust
    extern crate rgb;
    extern crate lcms2;
    use lcms2::*;

    fn main() {
        let icc_file = include_bytes!("custom_profile.icc"); // You can use File::open()….read_to_end(…), too
        let custom_profile = Profile::new_icc(icc_file).unwrap();

        let srgb_profile = Profile::new_srgb();

        let t = Transform::new(&custom_profile, PixelFormat::RGB_8, &srgb_profile, PixelFormat::RGB_8, Intent::Perceptual);

        // Pixel struct must have layout compatible with PixelFormat specified in new()
        let source_pixels: &[rgb::RGB<u8>] = …;
        t.transform_pixels(source_pixels, destination_pixels);

        // If input and output pixel formats are the same, you can overwrite them instead of copying
        t.transform_in_place(source_and_dest_pixels);
    }
```

See `examples` dir and [LCMS2 documentation PDF](http://www.littlecms.com/LittleCMS2.7%20API.pdf) for more info.

To apply ICC profile in JPEG:

```rust
if b"ICC_PROFILE\0" == &app2_marker_data[0..12] {
   let icc = &app2_marker_data[14..]; // Lazy assumption that the profile is smaller than 64KB
   let profile = Profile::new_icc(icc).unwrap();
   let t = Transform::new(&profile, PixelFormat::RGB_8,
       &Profile::new_srgb(), PixelFormat::RGB_8, Intent::Perceptual);
   t.transform_in_place(&mut rgb);
}
```
