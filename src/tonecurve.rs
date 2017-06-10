use super::*;
use std;

impl ToneCurve {
    pub fn new(gamma: f64) -> ToneCurve {
        Self::new_handle(unsafe { ffi::cmsBuildGamma(std::ptr::null_mut(), gamma) })
    }

    pub fn new_tabulated(values: &[u16]) -> ToneCurve {
        assert!(values.len() < std::i32::MAX as usize);
        Self::new_handle(unsafe {
            ffi::cmsBuildTabulatedToneCurve16(std::ptr::null_mut(), values.len() as i32, values.as_ptr())
        })
    }

    pub fn new_tabulated_float(values: &[f32]) -> ToneCurve {
        assert!(values.len() < std::i32::MAX as usize);
        Self::new_handle(unsafe {
            ffi::cmsBuildTabulatedToneCurveFloat(std::ptr::null_mut(), values.len() as u32, values.as_ptr())
        })
    }

    fn new_handle(handle: *mut ffi::ToneCurve) -> ToneCurve {
        assert!(!handle.is_null());
        ToneCurve { handle: handle }
    }

    pub fn reversed(&self) -> ToneCurve {
        Self::new_handle(unsafe { ffi::cmsReverseToneCurve(self.handle) })
    }

    pub fn is_multisegment(&self) -> bool {
        unsafe { ffi::cmsIsToneCurveMultisegment(self.handle) != 0 }
    }

    pub fn is_linear(&self) -> bool {
        unsafe { ffi::cmsIsToneCurveLinear(self.handle) != 0 }
    }

    pub fn is_monotonic(&self) -> bool {
        unsafe { ffi::cmsIsToneCurveMonotonic(self.handle) != 0 }
    }

    pub fn is_descending(&self) -> bool {
        unsafe { ffi::cmsIsToneCurveDescending(self.handle) != 0 }
    }

    pub fn parametric_type(&self) -> i32 {
        unsafe { ffi::cmsGetToneCurveParametricType(self.handle) }
    }

    pub fn estimated_gamma(&self, precision: f64) -> f64 {
        unsafe { ffi::cmsEstimateGamma(self.handle, precision) }
    }

    pub fn smooth(&mut self, lambda: f64) -> bool {
        unsafe { ffi::cmsSmoothToneCurve(self.handle, lambda) != 0 }
    }

    pub fn estimated_entries(&self) -> &[u16] {
        unsafe {
            let len = ffi::cmsGetToneCurveEstimatedTableEntries(self.handle) as usize;
            let data = ffi::cmsGetToneCurveEstimatedTable(self.handle);
            std::slice::from_raw_parts(data, len)
        }
    }
}

impl Clone for ToneCurve {
    fn clone(&self) -> ToneCurve {
        Self::new_handle(unsafe { ffi::cmsDupToneCurve(self.handle) })
    }
}

impl Drop for ToneCurve {
    fn drop(&mut self) {
        unsafe {
            ffi::cmsFreeToneCurve(self.handle);
        }
    }
}

#[test]
fn tones() {
    let g = ToneCurve::new(1./2.2);
    let mut z = g.clone();
    assert!(!z.is_linear());
    assert!(z.is_monotonic());
    assert!(!z.is_descending());
    assert!(z.reversed().is_monotonic());
    assert!(z.smooth(0.5));

    assert_eq!(0, g.estimated_entries()[0]);
    assert_eq!(std::u16::MAX, *g.estimated_entries().last().unwrap());
}
