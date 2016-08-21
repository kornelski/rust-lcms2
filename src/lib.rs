#![allow(dead_code)]

extern crate lcms2_sys as ffi;
mod profile;
mod tag;
mod transform;
mod tonecurve;
use std::marker::PhantomData;

#[doc(hidden)]
pub use ffi::CIEXYZ;
#[doc(hidden)]
pub use ffi::CIExyYTRIPLE;
#[doc(hidden)]
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

pub enum Tag<'a> {
    CIExyYTRIPLE(&'a ffi::CIExyYTRIPLE),
    CIEXYZ(&'a ffi::CIEXYZ),
    ICCData(&'a ffi::ICCData),
    ICCMeasurementConditions(&'a ffi::ICCMeasurementConditions),
    ICCViewingConditions(&'a ffi::ICCViewingConditions),
    MLU(&'a ffi::MLU),
    NAMEDCOLORLIST(&'a ffi::NAMEDCOLORLIST),
    Pipeline(&'a ffi::Pipeline),
    Screening(&'a ffi::Screening),
    SEQ(&'a ffi::SEQ),
    Signature(&'a ffi::Signature),
    ToneCurve(&'a ffi::ToneCurve),
    UcrBg(&'a ffi::UcrBg),
    None,
}

pub fn version() -> u32 {
    unsafe {
        ffi::cmsGetEncodedCMMversion() as u32
    }
}
