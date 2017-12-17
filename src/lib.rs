//! See [Little CMS full documentation](https://pornel.github.io/rust-lcms2-sys/) for more in-depth information about LCMS functions.
//!
//! The main types you need to use in this crate are `Profile` and `Transform`
#![allow(dead_code)]
#![doc(html_logo_url = "https://pornel.github.io/rust-lcms2/lcms_logo.png")]
#![doc(html_root_url = "https://pornel.github.io/rust-lcms2")]

extern crate lcms2_sys as ffi;

#[macro_use]
extern crate foreign_types;

mod profile;
mod tag;
mod ciecam;
mod context;
mod mlu;
mod namedcolorlist;
mod pipeline;
mod eval;
mod ext;
mod flags;
mod locale;
mod stage;
mod transform;
mod tonecurve;
mod error;
use std::marker::PhantomData;

pub use profile::*;
pub use error::*;
pub use ciecam::*;
pub use context::{GlobalContext, ThreadContext};
pub use mlu::*;
pub use ext::*;
pub use flags::*;
pub use locale::*;
pub use pipeline::*;
pub use stage::*;
pub use transform::*;
pub use tonecurve::*;
pub use namedcolorlist::*;

pub use ffi::CIEXYZ;
pub use ffi::CIELab;
#[doc(hidden)]
pub use ffi::CIExyYTRIPLE;
#[doc(hidden)]
pub use ffi::CIExyY;
#[doc(hidden)]
pub use ffi::JCh;

pub use ffi::PixelFormat;
pub use ffi::InfoType;
pub use ffi::TagSignature;
pub use ffi::Intent;
pub use ffi::ColorSpaceSignature;
pub use ffi::ProfileClassSignature;
pub use ffi::ViewingConditions;

#[derive(Debug)]
/// Value of a tag in an ICC profile
pub enum Tag<'a> {
    CIExyYTRIPLE(&'a ffi::CIExyYTRIPLE),
    CIEXYZ(&'a ffi::CIEXYZ),
    ICCData(&'a ffi::ICCData),
    ICCMeasurementConditions(&'a ffi::ICCMeasurementConditions),
    ICCViewingConditions(&'a ffi::ICCViewingConditions),
    /// Unicode string
    MLU(&'a mlu::MLURef),
    /// A palette
    NAMEDCOLORLIST(&'a NamedColorListRef),
    Pipeline(&'a PipelineRef),
    Screening(&'a ffi::Screening),
    SEQ(&'a ffi::SEQ),
    Intent(Intent),
    ColorimetricIntentImageState(ffi::ColorimetricIntentImageState),
    Technology(ffi::TechnologySignature),
    ToneCurve(&'a ToneCurveRef),
    UcrBg(&'a ffi::UcrBg),
    /// Unknown format or missing data
    None,
}

/// LCMS version
pub fn version() -> u32 {
    unsafe {
        ffi::cmsGetEncodedCMMversion() as u32
    }
}

/// Temperature <-> Chromaticity (Black body)
/// Color temperature is a characteristic of visible light that has important applications.
///
/// The color temperature of a light source is determined by comparing its chromaticity with that of an ideal black-body radiator.
/// The temperature (usually measured in kelvin, K) is that source's color temperature at which the heated black-body radiator matches the color of the light source for a black body source.
/// Higher color temperatures (5,000 K or more) are cool (bluish white) colors, and lower color temperatures (2,700–3,000 K) warm (yellowish white through red) colors.
///
/// See `CIExzYExt::temp()`
pub fn white_point_from_temp(temp: f64) -> Option<CIExyY> {
    let mut res = CIExyY{x:0.,y:0.,Y:0.};
    let ok = unsafe {
        ffi::cmsWhitePointFromTemp(&mut res, temp) != 0
    };
    if ok { Some(res) } else { None }
}
