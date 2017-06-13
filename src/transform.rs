use super::*;
use context::Context;
use std::os::raw::c_void;
use std::marker::PhantomData;

/// Conversion between two ICC profiles
///
/// Usually you don't need to specify `InputPixelFormat`/`OutputPixelFormat` parameters explicitly.
/// They will be inferred from the call to `transform_pixels` or `transform_in_place`.
/// You might get "cannot infer type for `InputPixelFormat`" error before you write calls to one of these functions.
pub struct Transform<InputPixelFormat, OutputPixelFormat, Context = GlobalContext> {
    pub(crate) handle: ffi::HTRANSFORM,
    _from: PhantomData<InputPixelFormat>,
    _to: PhantomData<OutputPixelFormat>,
    _context_ref: PhantomData<Context>,
}

impl<InputPixelFormat: Copy + Clone, OutputPixelFormat: Copy + Clone> Transform<InputPixelFormat, OutputPixelFormat, GlobalContext> {
    /// Creates a color transform for translating bitmaps.
    ///
    ///  * Input: Handle to a profile object capable to work in input direction
    ///  * InputFormat: A bit-field format specifier as described in Formatters section.
    ///  * Output: Handle to a profile object capable to work in output direction
    ///  * OutputFormat: A bit-field format specifier as described in Formatters section.
    ///  * Intent: Rendering intent
    pub fn new(input: &Profile,
               in_format: PixelFormat,
               output: &Profile,
               out_format: PixelFormat,
               intent: Intent) -> Result<Self, Error> {
        Self::new_flags(input, in_format, output, out_format, intent, 0)
    }

    pub fn new_flags(input: &Profile,
                     in_format: PixelFormat,
                     output: &Profile,
                     out_format: PixelFormat,
                     intent: Intent,
                     flags: u32)
                     -> Result<Self, Error> {
        Self::new_flags_context(GlobalContext::new(), input, in_format, output, out_format, intent, flags)
    }

    /// Adaptation state for absolute colorimetric intent, on all but cmsCreateExtendedTransform.
    pub fn global_adaptation_state() -> f64 {
        unsafe {
            ffi::cmsSetAdaptationState(-1.)
        }
    }

    /// Sets adaptation state for absolute colorimetric intent, on all but cmsCreateExtendedTransform.
    /// Little CMS can handle incomplete adaptation states.
    ///
    /// Degree on adaptation 0=Not adapted, 1=Complete adaptation,  in-between=Partial adaptation.
    pub fn set_global_adaptation_state(value: f64) {
        unsafe {
            ffi::cmsSetAdaptationState(value);
        }
    }

    /// Sets the global codes used to mark out-out-gamut on Proofing transforms. Values are meant to be encoded in 16 bits.
    /// AlarmCodes: Array [16] of codes. ALL 16 VALUES MUST BE SPECIFIED, set to zero unused channels.
    pub fn set_global_alarm_codes(codes: [u16; ffi::MAXCHANNELS]) {
        unsafe {
            ffi::cmsSetAlarmCodes(codes.as_ptr())
        }
    }

    /// Gets the current global codes used to mark out-out-gamut on Proofing transforms. Values are meant to be encoded in 16 bits.
    pub fn get_global_alarm_codes() -> [u16; ffi::MAXCHANNELS] {
        let mut tmp = [0u16; ffi::MAXCHANNELS];
        unsafe {
            ffi::cmsGetAlarmCodes(tmp.as_mut_ptr());
        }
        tmp
    }

    /// A proofing transform does emulate the colors that would appear as  the image were rendered on a specific device.
    /// The obtained transform emulates the device described by the "Proofing" profile. Useful to preview final result without rendering to the physical medium.
    ///
    /// That is, for  example,  with a proofing transform I can see how will look a photo of my little daughter if rendered on my HP printer. Since most printer profiles   does include some sort of gamut-remapping, it is likely colors  will not look as the original. Using a proofing  transform, it can be done by using the appropriate function. Note that this is an important feature for final users, it is worth  of all color-management stuff if the final media is not cheap.
    ///
    /// To enable proofing and gamut check you need to include following flags:
    ///
    ///  * `FLAGS_GAMUTCHECK`: Color out of gamut are flagged to a fixed color defined by the function cmsSetAlarmCodes
    ///  * `FLAGS_SOFTPROOFING`: does emulate the Proofing device.
    pub fn new_proofing(input: &Profile, in_format: PixelFormat,
                        output: &Profile, out_format: PixelFormat,
                        proofing: &Profile, intent: Intent, proofng_intent: Intent,
                        flags: u32)
                        -> Result<Self, Error> {
        Self::new_proofing_context(GlobalContext::new(), input, in_format, output, out_format, proofing, intent, proofng_intent, flags)
    }
}

impl<InputPixelFormat: Copy + Clone, OutputPixelFormat: Copy + Clone, Ctx: Context> Transform<InputPixelFormat, OutputPixelFormat, Ctx> {
    fn new_handle(handle: ffi::HTRANSFORM, in_format: PixelFormat, out_format: PixelFormat) -> Result<Self, Error> {
        if handle.is_null() {
            Err(Error::ObjectCreationError)
        } else {
            Ok(Transform {
                handle: handle,
                _from: Self::check_format::<InputPixelFormat>(in_format, true),
                _to: Self::check_format::<OutputPixelFormat>(out_format, false),
                _context_ref: PhantomData,
            })
        }
    }

    fn check_format<Z>(format: PixelFormat, input: bool) -> PhantomData<Z> {
        assert!(!format.planar(), "Planar not supported");
        assert_eq!(format.bytes_per_pixel(),
                   std::mem::size_of::<Z>(),
                   "PixelFormat {:?} has {} bytes per pixel, but the {} format has {}",
                   format,
                   format.bytes_per_pixel(),
                   if input {"input"} else {"output"},
                   std::mem::size_of::<Z>());
        PhantomData
    }

    /// This function translates bitmaps according of parameters setup when creating the color transform.
    pub fn transform_pixels(&self, src: &[InputPixelFormat], dst: &mut [OutputPixelFormat]) {
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
        unsafe { ffi::cmsGetTransformInputFormat(self.handle) as PixelFormat }
    }

    pub fn output_format(&self) -> PixelFormat {
        unsafe { ffi::cmsGetTransformOutputFormat(self.handle) as PixelFormat }
    }

    pub fn new_context(context: Ctx, input: &Profile<Ctx>, in_format: PixelFormat,
                       output: &Profile<Ctx>, out_format: PixelFormat, intent: Intent) -> LCMSResult<Self> {
        Self::new_flags_context(context, input, in_format, output, out_format, intent, 0)
    }

    pub fn new_flags_context(context: Ctx, input: &Profile<Ctx>, in_format: PixelFormat,
                             output: &Profile<Ctx>, out_format: PixelFormat,
                             intent: Intent, flags: u32)
                             -> Result<Self, Error> {
        Self::new_handle(unsafe {
                             ffi::cmsCreateTransformTHR(context.as_ptr(),
                                input.handle, in_format,
                                output.handle, out_format,
                                intent, flags)
                         },
                         in_format, out_format)
    }

    pub fn new_proofing_context(context: Ctx, input: &Profile<Ctx>, in_format: PixelFormat,
                        output: &Profile<Ctx>, out_format: PixelFormat,
                        proofing: &Profile<Ctx>, intent: Intent, proofng_intent: Intent,
                        flags: u32)
                        -> Result<Self, Error> {
        Self::new_handle(unsafe {
                             ffi::cmsCreateProofingTransformTHR(context.as_ptr(), input.handle, in_format,
                                output.handle, out_format,
                                proofing.handle, intent, proofng_intent, flags)
                         },
                         in_format, out_format)
    }
}

impl<PixelFormat: Copy + Clone, Ctx: Context> Transform<PixelFormat, PixelFormat, Ctx> {
    pub fn transform_in_place(&self, srcdst: &mut [PixelFormat]) {
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

impl<F, T, C> Drop for Transform<F, T, C> {
    fn drop(&mut self) {
        unsafe {
            ffi::cmsDeleteTransform(self.handle);
        }
    }
}
