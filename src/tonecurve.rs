use super::*;
use std::fmt;
use std::ptr;
use crate::eval::FloatOrU16;
use foreign_types::{ForeignType, ForeignTypeRef};

foreign_type! {
    /// Tone curves are powerful constructs that can contain curves specified in diverse ways.
    ///
    /// The curve is stored in segments, where each segment can be sampled or specified by parameters. A 16.bit simplification of the *whole* curve is kept for optimization purposes. For float operation, each segment is evaluated separately. Plug-ins may be used to define new parametric schemes.
    ///
    /// Owned version of `ToneCurveRef`
    pub unsafe type ToneCurve {
        type CType = ffi::ToneCurve;
        fn drop = ffi::cmsFreeToneCurve;
        fn clone = ffi::cmsDupToneCurve;
    }
}

impl ToneCurve {
    /// Simplified wrapper to `new_parametric`. Builds a parametric curve of type 1.
    pub fn new(gamma: f64) -> Self {
        unsafe { Self::from_ptr(ffi::cmsBuildGamma(ptr::null_mut(), gamma)) }
    }

    /// Builds a tone curve based on a table of 16-bit values. Tone curves built with this function are restricted to 0…1.0 domain.
    pub fn new_tabulated(values: &[u16]) -> Self {
        assert!(values.len() < std::i32::MAX as usize);
        unsafe { Self::new_handle(ffi::cmsBuildTabulatedToneCurve16(ptr::null_mut(), values.len() as _, values.as_ptr())) }
    }

    /// Builds a tone curve based on a table of floating point  values. Tone curves built with this function are **not** restricted to 0…1.0 domain.
    pub fn new_tabulated_float(values: &[f32]) -> Self {
        assert!(values.len() < std::i32::MAX as usize);
        unsafe { Self::new_handle(
            ffi::cmsBuildTabulatedToneCurveFloat(ptr::null_mut(), values.len() as u32, values.as_ptr())
        )}
    }

    /// See Table 52 in LCMS documentation for descriptino of the types.
    ///
    ///  1. Exponential
    ///  2. CIE 122-1966
    ///  3. IEC 61966-3
    ///  4. IEC 61966-2.1 (sRGB)
    ///  5. See PDF
    ///  6. Identical to 5, unbounded.
    ///  7. See PDF
    ///  8. See PDF
    ///  108. (108) S-Shaped sigmoidal
    ///
    /// Negative curve types will return the inverse curve of the corresponding positive type.
    /// 
    /// Always use 10-parameter slice for plug-in types.
    pub fn new_parametric(curve_type: i16, params: &[f64]) -> LCMSResult<Self> {
        let params_min_len = match curve_type.abs() {
            1 => 1,
            2 => 3,
            3 => 4,
            4 => 5,
            5 => 7,
            6 => 4,
            7 => 5,
            8 => 6,
            108 => 1,
            _ => 10,
        };
        if params.len() < params_min_len {
            return Err(Error::MissingData);
        }

        unsafe {
            Error::if_null(ffi::cmsBuildParametricToneCurve(ptr::null_mut(), curve_type.into(), params.as_ptr()))
        }
    }

    unsafe fn new_handle(handle: *mut ffi::ToneCurve) -> Self {
        assert!(!handle.is_null());
        Self::from_ptr(handle)
    }
}

impl ToneCurveRef {
    /// Creates a tone curve that is the inverse  of given tone curve.
    pub fn reversed(&self) -> ToneCurve {
        unsafe { ToneCurve::from_ptr(ffi::cmsReverseToneCurve(self.as_ptr())) }
    }

    /// Creates a tone curve that is the inverse  of given tone curve. In the case it couldn’t be analytically reversed, a tablulated curve of nResultSamples is created.
    pub fn reversed_samples(&self, samples: usize) -> ToneCurve {
        unsafe { ToneCurve::from_ptr(ffi::cmsReverseToneCurveEx(samples as _, self.as_ptr())) }
        }

    /// Composites two tone curves in the form Y^-1(X(t))
    /// (self is X, the argument is Y)
    pub fn join(&self, y: &ToneCurveRef, points: usize) -> ToneCurve {
        unsafe {
            ToneCurve::from_ptr(ffi::cmsJoinToneCurve(ptr::null_mut(), self.as_ptr(), y.as_ptr(), points as u32))
        }
    }

    /// Returns TRUE if the tone curve contains more than one segment, FALSE if it has only one segment.
    pub fn is_multisegment(&self) -> bool {
        unsafe { ffi::cmsIsToneCurveMultisegment(self.as_ptr()) != 0 }
    }

    /// Returns an estimation of cube being an identity (1:1) in the [0..1] domain. Does not take unbounded parts into account. This is just a coarse approximation, with no mathematical validity.
    pub fn is_linear(&self) -> bool {
        unsafe { ffi::cmsIsToneCurveLinear(self.as_ptr()) != 0 }
    }

    /// Returns an estimation of monotonicity of curve in the [0..1] domain. Does not take unbounded parts into account. This is just a coarse approximation, with no mathematical validity.
    pub fn is_monotonic(&self) -> bool {
        unsafe { ffi::cmsIsToneCurveMonotonic(self.as_ptr()) != 0 }
    }

    /// Does not take unbounded parts into account.
    pub fn is_descending(&self) -> bool {
        unsafe { ffi::cmsIsToneCurveDescending(self.as_ptr()) != 0 }
    }

    pub fn parametric_type(&self) -> i32 {
        unsafe { ffi::cmsGetToneCurveParametricType(self.as_ptr()) }
    }

    /// Estimates the apparent gamma of the tone curve by using least squares fitting.
    /// Precision: The maximum standard deviation allowed on the residuals, 0.01 is a fair value, set it to a big number to fit any curve, mo matter how good is the fit.
    pub fn estimated_gamma(&self, precision: f64) -> Option<f64> {
        let g = unsafe { ffi::cmsEstimateGamma(self.as_ptr(), precision) };
        if g <= -1.0 { None } else { Some(g) }
    }

    /// Smoothes tone curve according to the lambda parameter. From: Eilers, P.H.C. (1994) Smoothing and interpolation with finite differences. in: Graphic Gems IV, Heckbert, P.S. (ed.), Academic press.
    pub fn smooth(&mut self, lambda: f64) -> bool {
        unsafe { ffi::cmsSmoothToneCurve(self.as_ptr(), lambda) != 0 }
    }

    /// Tone curves do maintain a shadow low-resolution tabulated representation of the curve. This function returns a pointer to this table.
    pub fn estimated_entries(&self) -> &[u16] {
        unsafe {
            let len = ffi::cmsGetToneCurveEstimatedTableEntries(self.as_ptr()) as usize;
            let data = ffi::cmsGetToneCurveEstimatedTable(self.as_ptr());
            std::slice::from_raw_parts(data, len)
        }
    }

    /// Evaluates the given number (u16 or f32) across the given tone curve.
    ///
    /// This function is significantly faster for u16, since it uses a pre-computed 16-bit lookup table.
    pub fn eval<ToneCurveValue: FloatOrU16>(&self, v: ToneCurveValue) -> ToneCurveValue {
        unsafe { v.eval_tone_curve(self.as_ptr()) }
    }
}

impl fmt::Debug for ToneCurveRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ToneCurve({} entries, gamma ~{:.1})", self.estimated_entries().len(), self.estimated_gamma(1.).unwrap_or(0.))
    }
}

#[test]
fn tones() {
    let _ = ToneCurve::new(0.);
    let _ = ToneCurve::new(1230.);
    let _ = ToneCurve::new(-10.);

    let g = ToneCurve::new(1./2.2);
    let r: &ToneCurveRef = &g;
    let mut z: ToneCurve = r.to_owned().clone();
    assert!(g.estimated_gamma(0.1).is_some());
    assert_eq!(1., g.eval(1.));
    assert_eq!(0, g.eval(0u16));
    assert!(!z.is_linear());
    assert!(z.is_monotonic());
    assert!(!z.is_descending());
    assert!(z.reversed().is_monotonic());
    assert!(z.smooth(0.5));

    assert_eq!(0, g.estimated_entries()[0]);
    assert_eq!(std::u16::MAX, *g.estimated_entries().last().unwrap());

    assert!(ToneCurve::new_parametric(7, &[0.]).is_err());
}
