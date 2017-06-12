extern crate lcms2;
use lcms2::*;
use std::thread;

fn main() {
    thread::spawn(|| {
        // For each thread create its own context
        let context = ThreadContext::new();
        // And create profiles and transforms attached to that context
        let profile = Profile::new_srgb_context(&context);
        let tr = Transform::new_context(&context, &profile, PixelFormat::RGB_8, &profile, PixelFormat::RGB_8, Intent::Saturation).unwrap();
        let out = [0u8; 3];
        tr.transform_pixels(&[[1u8,2,3]], &mut [out]);
    }).join().unwrap();

    // Or each object can also own its context, which allows it to be sent to another thread
    let profile = Profile::new_srgb_context(ThreadContext::new());
    let tr = Transform::new_context(ThreadContext::new(), &profile, PixelFormat::RGB_8, &profile, PixelFormat::RGB_8, Intent::Saturation).unwrap();

    thread::spawn(move || {
        // For each thread create its own context
        // And create profiles and transforms attached to that context
        let out = [0u8; 3];
        tr.transform_pixels(&[[1u8,2,3]], &mut [out]);
    }).join().unwrap();
}
