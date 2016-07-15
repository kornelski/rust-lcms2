use super::*;

extern crate lcms2_sys as ffi;

use std;
use std::os::raw::c_void;
use std::marker::PhantomData;

impl<F: Copy + Clone, T: Copy + Clone> Transform<F, T> {
    pub fn new(input: &Profile,
               in_format: PixelFormat,
               output: &Profile,
               out_format: PixelFormat,
               intent: Intent) -> Transform<F, T> {
        Self::new_flags(input, in_format, output, out_format, intent, 0)
    }

    pub fn new_flags(input: &Profile,
               in_format: PixelFormat,
               output: &Profile,
               out_format: PixelFormat,
               intent: Intent,
               flags: u32)
               -> Transform<F, T> {
        Transform {
            handle: unsafe {
                ffi::cmsCreateTransform(input.handle,
                                        in_format,
                                        output.handle,
                                        out_format,
                                        intent,
                                        flags)
            },
            _from: Self::check_format::<F>(in_format),
            _to: Self::check_format::<T>(out_format),
        }
    }

    fn check_format<Z>(format: PixelFormat) -> PhantomData<Z> {
        assert!(!format.planar(), "Planar not supported");
        assert_eq!(format.bytes_per_pixel(),
                   std::mem::size_of::<Z>(),
                   "PixelFormat {:?} has different size than the Rust data type",
                   format);
        PhantomData
    }

    pub fn transform_pixels(&self, src: &[F], dst: &mut [T]) {
        let size = src.len();
        assert_eq!(size, dst.len());
        assert!(size < std::u32::MAX as usize);
        unsafe {
            ffi::cmsDoTransform(self.handle,
                                src.as_ptr() as *const c_void,
                                dst.as_ptr() as *mut c_void,
                                size as u32);
        }
    }

    pub fn input_format(&self) -> PixelFormat {
        unsafe {
            ffi::cmsGetTransformInputFormat(self.handle) as PixelFormat
        }
    }

    pub fn output_format(&self) -> PixelFormat {
        unsafe {
            ffi::cmsGetTransformOutputFormat(self.handle) as PixelFormat
        }
    }
}

impl<T: Copy + Clone> Transform<T, T> {
      pub fn transform_in_place(&self, srcdst: &mut [T]) {
        let size = srcdst.len();
        assert!(size < std::u32::MAX as usize);
        unsafe {
            ffi::cmsDoTransform(self.handle,
                                srcdst.as_ptr() as *const c_void,
                                srcdst.as_ptr() as *mut c_void,
                                size as u32);
        }
    }
}

impl<F, T> Drop for Transform<F, T> {
    fn drop(&mut self) {
        unsafe {
            ffi::cmsDeleteTransform(self.handle);
        }
    }
}
