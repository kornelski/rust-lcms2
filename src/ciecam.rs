use crate::*;
use std::mem::MaybeUninit;
use std::ptr;

/// CIE CAM02
#[repr(transparent)]
pub struct CIECAM02 {
    handle: ffi::HANDLE,
}

impl CIECAM02 {
    /// A CAM02 object based on given viewing conditions.
    ///
    /// Such object may be used as a color appearance model and evaluated in forward and reverse directions.
    /// Viewing conditions structure is detailed in Table 43. The surround member has to be one of the values enumerated in Table 44.
    /// Degree of chromatic adaptation (d), can be specified in 0...1.0 range, or the model can be instructed to calculate it by using `D_CALCULATE` constant (-1).
    ///
    ///  Viewing conditions.
    /// Please note those are CAM model viewing conditions, and not the ICC tag viewing conditions, which I'm naming `cmsICCViewingConditions` to make differences evident. Unfortunately, the tag cannot deal with surround La, Yb and D value so is basically useless to store CAM02 viewing conditions.
    pub fn new(conditions: ViewingConditions) -> LCMSResult<Self> {
        let handle = unsafe { ffi::cmsCIECAM02Init(ptr::null_mut(), &conditions) };
        if !handle.is_null() {
            Ok(Self { handle })
        } else {
            Err(Error::ObjectCreationError)
        }
    }

    /// Evaluates the CAM02 model in the forward direction
    pub fn forward(&mut self, input: &CIEXYZ) -> JCh {
        unsafe {
            let mut out = MaybeUninit::uninit();
            ffi::cmsCIECAM02Forward(self.handle, input, out.as_mut_ptr());
            out.assume_init()
        }
    }

    /// Evaluates the CAM02 model in the reverse direction
    pub fn reverse(&mut self, input: &JCh) -> CIEXYZ {
        unsafe {
            let mut out = MaybeUninit::uninit();
            ffi::cmsCIECAM02Reverse(self.handle, input, out.as_mut_ptr());
            out.assume_init()
        }
    }
}

impl Drop for CIECAM02 {
    fn drop(&mut self) {
        unsafe {
            ffi::cmsCIECAM02Done(self.handle);
        }
    }
}
