#![allow(dead_code)]

extern crate lcms2_sys as ffi;
mod profile;
mod transform;
mod tonecurve;
use std::marker::PhantomData;

pub use ffi::CIEXYZ;
pub use ffi::CIExyYTRIPLE;
pub use ffi::CIExyY;
pub use ffi::PixelFormat;
pub use ffi::InfoType;
pub use ffi::TagSignature;
pub use ffi::Intent;
pub use ffi::ColorSpaceSignature;
pub use ffi::ProfileClassSignature;
pub type Context = ffi::Context;

pub struct Profile {
    handle: ffi::HPROFILE,
}

pub struct Transform<F, T> {
    handle: ffi::HTRANSFORM,
    _from: PhantomData<F>,
    _to: PhantomData<T>,
}

pub struct ToneCurve {
    handle: *mut ffi::ToneCurve,
}
