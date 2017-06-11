//! See [Little CMS full documentation](https://pornel.github.io/rust-lcms2-sys/) for more in-depth information about LCMS functions.
#![allow(dead_code)]

extern crate lcms2_sys as ffi;
#[macro_use]
extern crate foreign_types;

mod profile;
mod tag;
mod mlu;
mod namedcolorlist;
mod pipeline;
mod eval;
mod locale;
mod transform;
mod tonecurve;
mod error;
use std::marker::PhantomData;

pub use error::*;
pub use mlu::*;
pub use locale::*;
pub use pipeline::*;
pub use tonecurve::*;
pub use namedcolorlist::*;

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

#[derive(Debug)]
pub enum Tag<'a> {
    CIExyYTRIPLE(&'a ffi::CIExyYTRIPLE),
    CIEXYZ(&'a ffi::CIEXYZ),
    ICCData(&'a ffi::ICCData),
    ICCMeasurementConditions(&'a ffi::ICCMeasurementConditions),
    ICCViewingConditions(&'a ffi::ICCViewingConditions),
    MLU(&'a mlu::MLURef),
    NAMEDCOLORLIST(&'a NamedColorListRef),
    Pipeline(&'a PipelineRef),
    Screening(&'a ffi::Screening),
    SEQ(&'a ffi::SEQ),
    Signature(&'a ffi::Signature),
    Technology(ffi::TechnologySignature),
    ToneCurve(&'a ToneCurveRef),
    UcrBg(&'a ffi::UcrBg),
    None,
}

pub fn version() -> u32 {
    unsafe {
        ffi::cmsGetEncodedCMMversion() as u32
    }
}
