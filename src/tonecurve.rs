use super::*;
use std::fmt;
use eval::FloatOrU16;
use foreign_types::{ForeignType, ForeignTypeRef};

foreign_type! {
    type CType = ffi::ToneCurve;
    fn drop = ffi::cmsFreeToneCurve;
    pub struct ToneCurve;
    pub struct ToneCurveRef;
}

impl ToneCurve {
    pub fn new(gamma: f64) -> Self {
        unsafe {
            Self::from_ptr(ffi::cmsBuildGamma(std::ptr::null_mut(), gamma))
        }
    }

    pub fn new_tabulated(values: &[u16]) -> Self {
        assert!(values.len() < std::i32::MAX as usize);
        unsafe { Self::new_handle(
            ffi::cmsBuildTabulatedToneCurve16(std::ptr::null_mut(), values.len() as i32, values.as_ptr())
        )}
    }

    pub fn new_tabulated_float(values: &[f32]) -> Self {
        assert!(values.len() < std::i32::MAX as usize);
        unsafe { Self::new_handle(
            ffi::cmsBuildTabulatedToneCurveFloat(std::ptr::null_mut(), values.len() as u32, values.as_ptr())
        )}
    }

    unsafe fn new_handle(handle: *mut ffi::ToneCurve) -> Self {
        assert!(!handle.is_null());
        Self::from_ptr(handle)
    }
}

impl ToneCurveRef {
    /// Creates a tone curve that is the inverse  of given tone curve.
    pub fn reversed(&self) -> ToneCurve {
        unsafe {
            ToneCurve::from_ptr(ffi::cmsReverseToneCurve(self.as_ptr()))
        }
    }

    /// Creates a tone curve that is the inverse  of given tone curve. In the case it couldn’t be analytically reversed, a tablulated curve of nResultSamples is created.
    pub fn reversed_samples(&self, samples: usize) -> ToneCurve {
        unsafe {
            ToneCurve::from_ptr(ffi::cmsReverseToneCurveEx(samples as i32, self.as_ptr()))
        }
    }

    /// Composites two tone curves in the form Y^-1(X(t))
    /// (self is X, the argument is Y)
    pub fn join(&self, y: &ToneCurveRef, points: usize) -> ToneCurve {
        unsafe {
            ToneCurve::from_ptr(ffi::cmsJoinToneCurve(std::ptr::null_mut(), self.as_ptr(), y.as_ptr(), points as u32))
        }
    }

    pub fn is_multisegment(&self) -> bool {
        unsafe { ffi::cmsIsToneCurveMultisegment(self.as_ptr()) != 0 }
    }

    pub fn is_linear(&self) -> bool {
        unsafe { ffi::cmsIsToneCurveLinear(self.as_ptr()) != 0 }
    }

    pub fn is_monotonic(&self) -> bool {
        unsafe { ffi::cmsIsToneCurveMonotonic(self.as_ptr()) != 0 }
    }

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
        if g <= -1.0 {None} else {Some(g)}
    }

    pub fn smooth(&mut self, lambda: f64) -> bool {
        unsafe { ffi::cmsSmoothToneCurve(self.as_ptr(), lambda) != 0 }
    }

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
        unsafe {
            v.eval_tone_curve(self.as_ptr())
        }
    }
}

impl Clone for ToneCurve {
    fn clone(&self) -> ToneCurve {
        unsafe {
            ToneCurve::from_ptr( ffi::cmsDupToneCurve(self.as_ptr()))
        }
    }
}

impl fmt::Debug for ToneCurveRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ToneCurve({} entries, gamma ~{:.1})", self.estimated_entries().len(), self.estimated_gamma(1.).unwrap_or(0.))
    }
}

#[test]
fn tones() {
    let _ = ToneCurve::new(0.);
    let _ = ToneCurve::new(1230.);
    let _ = ToneCurve::new(-10.);

    let g = ToneCurve::new(1./2.2);
    let mut z = g.clone();
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
}
