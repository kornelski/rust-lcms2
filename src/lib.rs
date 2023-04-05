//! See [Little CMS full documentation](https://kornelski.github.io/rust-lcms2-sys/) for more in-depth information about LCMS functions.
//!
//! The main types you need to use in this crate are `Profile` and `Transform`
#![allow(dead_code)]
#![doc(html_logo_url = "https://kornelski.github.io/rust-lcms2/lcms_logo.png")]
#![doc(html_root_url = "https://kornelski.github.io/rust-lcms2")]

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

pub use crate::profile::*;
pub use crate::error::*;
pub use crate::ciecam::*;
pub use crate::context::{GlobalContext, ThreadContext};
pub use crate::mlu::*;
pub use crate::ext::*;
pub use crate::flags::*;
pub use crate::locale::*;
pub use crate::pipeline::*;
pub use crate::stage::*;
pub use crate::transform::*;
pub use crate::tonecurve::*;
pub use crate::namedcolorlist::*;

pub use crate::ffi::CIEXYZ;
pub use crate::ffi::CIELab;
#[doc(hidden)]
pub use crate::ffi::CIExyYTRIPLE;
#[doc(hidden)]
pub use crate::ffi::CIExyY;
#[doc(hidden)]
pub use crate::ffi::JCh;

pub use crate::ffi::PixelFormat;
pub use crate::ffi::InfoType;
pub use crate::ffi::TagSignature;
pub use crate::ffi::Intent;
pub use crate::ffi::ColorSpaceSignature;
pub use crate::ffi::ProfileClassSignature;
pub use crate::ffi::ViewingConditions;

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
    VcgtCurves([&'a ToneCurveRef; 3]),
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
