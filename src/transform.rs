use super::*;
use crate::context::Context;
use std::os::raw::c_void;
use std::marker::PhantomData;
use std::fmt;

/// Conversion between two ICC profiles.
///
/// The transform ensures type safety and thread safety at compile time. To do this, it has a few generic types associated with it.
/// Usually, you don't need to specify any of the generic parameters (like `InputPixelFormat`/`OutputPixelFormat`) explicitly,
/// because they are inferred from calls to constructors and `transform_pixels` or `transform_in_place`.
///
/// If you get error such as:
///
/// > cannot infer type for `InputPixelFormat`
/// > type annotations required: cannot resolve `_: std::marker::Copy`
///
/// then don't worry! Write some code that calls `transform_pixels()`,
/// because this is the function that makes the type of the transform clear.
///
/// In case you need to store the transform in a struct or return from a function, the full type of `Transform` is:
///
/// ```rust,ignore
/// Transform<InputPixelFormat, OutputPixelFormat, Context = GlobalContext, Flags = AllowCache>
/// ```
///
///  * `InputPixelFormat` — e.g. `(u8,u8,u8)` or struct `RGB<u8>`, etc.
///     The type must have appropriate number of bytes per pixel (i.e. you can't just use `[u8]` for everything).
///  * `OutputPixelFormat` — similar to `InputPixelFormat`. If both are the same, then `transform_in_place()` function works.
///  * `Context` — it's `GlobalContext` for the default non-thread-safe version, or `ThreadContext` for thread-safe version.
///  * `Flags` — `AllowCache` or `DisallowCache`. If you disallow cache, then the transform will be accessible from multiple threads.
///
/// Thread-safety:
///
///  * Transform is `Send` if you create it with `ThreadContext` (use `new_*_context()` functions).
///  * Transform is `Sync` if you create it without cache. Set flags to `Flags::NO_CACHE`.
///
pub struct Transform<InputPixelFormat, OutputPixelFormat, Context = GlobalContext, Flags = AllowCache> {
    pub(crate) handle: ffi::HTRANSFORM,
    _from: PhantomData<InputPixelFormat>,
    _to: PhantomData<OutputPixelFormat>,
    _context_ref: PhantomData<Context>,
    _flags_ref: PhantomData<Flags>,
}

unsafe impl<'a, F, T, C: Send, Z> Send for Transform<F, T, C, Z> {}
unsafe impl<'a, F, T, C: Send> Sync for Transform<F, T, C, DisallowCache> {}

impl<InputPixelFormat: Copy + Clone, OutputPixelFormat: Copy + Clone> Transform<InputPixelFormat, OutputPixelFormat, GlobalContext, AllowCache> {
    /// Creates a color transform for translating bitmaps.
    ///
    /// Basic, non-tread-safe version.
    ///
    ///  * Input: Handle to a profile object capable to work in input direction
    ///  * InputFormat: A bit-field format specifier
    ///  * Output: Handle to a profile object capable to work in output direction
    ///  * OutputFormat: A bit-field format specifier
    ///  * Intent: Rendering intent
    ///
    ///  See documentation of these types for more detail.
    #[inline]
    pub fn new(input: &Profile,
               in_format: PixelFormat,
               output: &Profile,
               out_format: PixelFormat,
               intent: Intent) -> LCMSResult<Self> {
        Self::new_flags(input, in_format, output, out_format, intent, Flags::default())
    }

    /// Non-thread-safe
    #[inline]
    pub fn new_flags<Fl: CacheFlag>(input: &Profile,
                     in_format: PixelFormat,
                     output: &Profile,
                     out_format: PixelFormat,
                     intent: Intent,
                     flags: Flags<Fl>)
                     -> LCMSResult<Self> {
        Self::new_flags_context(GlobalContext::new(), input, in_format, output, out_format, intent, flags.allow_cache())
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
    #[inline]
    pub fn new_proofing(input: &Profile, in_format: PixelFormat,
                        output: &Profile, out_format: PixelFormat,
                        proofing: &Profile, intent: Intent, proofng_intent: Intent,
                        flags: Flags)
                        -> LCMSResult<Self> {
        Self::new_proofing_context(GlobalContext::new(), input, in_format, output, out_format, proofing, intent, proofng_intent, flags)
    }

    /// Multiprofile transforms
    ///
    /// User passes in an array of handles to open profiles. The returned color transform do "smelt" all profiles in a single devicelink.
    /// Color spaces must be paired with the exception of Lab/XYZ, which can be interchanged.
    #[inline]
    pub fn new_multiprofile(profiles: &[&Profile], in_format: PixelFormat, out_format: PixelFormat, intent: Intent, flags: Flags) -> LCMSResult<Self> {
        Self::new_multiprofile_context(GlobalContext::new(), profiles, in_format, out_format, intent, flags)
    }
}

impl<PixelFormat: Copy + Clone, Ctx: Context, C> Transform<PixelFormat, PixelFormat, Ctx, C> {
    #[inline]
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

impl<InputPixelFormat: Copy + Clone, OutputPixelFormat: Copy + Clone, Ctx: Context> Transform<InputPixelFormat, OutputPixelFormat, Ctx, AllowCache> {
    // Same as `new()`, but allows specifying thread-safe context (enables `Send`)
    //
    // For `Sync`, see `new_flags_context` and `Flags::NO_CACHE`
    #[inline]
    pub fn new_context(context: impl AsRef<Ctx>, input: &Profile<Ctx>, in_format: PixelFormat,
                       output: &Profile<Ctx>, out_format: PixelFormat, intent: Intent) -> LCMSResult<Self> {
        Self::new_flags_context(context, input, in_format, output, out_format, intent, Flags::default())
    }
}

impl<InputPixelFormat: Copy + Clone, OutputPixelFormat: Copy + Clone, Ctx: Context, Fl: CacheFlag> Transform<InputPixelFormat, OutputPixelFormat, Ctx, Fl> {
    #[inline]
    fn new_handle(handle: ffi::HTRANSFORM, in_format: PixelFormat, out_format: PixelFormat) -> LCMSResult<Self> {
        if handle.is_null() {
            Err(Error::ObjectCreationError)
        } else {
            Ok(Transform {
                handle,
                _from: Self::check_format::<InputPixelFormat>(in_format, true),
                _to: Self::check_format::<OutputPixelFormat>(out_format, false),
                _context_ref: PhantomData,
                _flags_ref: PhantomData,
            })
        }
    }

    #[inline]
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

    #[inline]
    pub fn new_flags_context(context: impl AsRef<Ctx>, input: &Profile<Ctx>, in_format: PixelFormat,
                             output: &Profile<Ctx>, out_format: PixelFormat,
                             intent: Intent, flags: Flags<Fl>)
                             -> LCMSResult<Self> {
        Self::new_handle(unsafe {
                             ffi::cmsCreateTransformTHR(context.as_ref().as_ptr(),
                                input.handle, in_format,
                                output.handle, out_format,
                                intent, flags.bits())
                         },
                         in_format, out_format)
    }

    #[inline]
    pub fn new_proofing_context(context: impl AsRef<Ctx>, input: &Profile<Ctx>, in_format: PixelFormat,
                        output: &Profile<Ctx>, out_format: PixelFormat,
                        proofing: &Profile<Ctx>, intent: Intent, proofng_intent: Intent,
                        flags: Flags<Fl>)
                        -> LCMSResult<Self> {
        Self::new_handle(unsafe {
                             ffi::cmsCreateProofingTransformTHR(context.as_ref().as_ptr(), input.handle, in_format,
                                output.handle, out_format,
                                proofing.handle, intent, proofng_intent, flags.bits())
                         },
                         in_format, out_format)
    }

    #[inline]
    pub fn new_multiprofile_context(context: impl AsRef<Ctx>, profiles: &[&Profile<Ctx>],
                                in_format: PixelFormat, out_format: PixelFormat, intent: Intent, flags: Flags<Fl>) -> LCMSResult<Self> {
        let mut handles: Vec<_> = profiles.iter().map(|p| p.handle).collect();
        unsafe {
            Self::new_handle(
                ffi::cmsCreateMultiprofileTransformTHR(context.as_ref().as_ptr(), handles.as_mut_ptr(), handles.len() as u32, in_format, out_format, intent, flags.bits()),
                in_format,
                out_format,
            )
        }
    }
}

impl<F, T, C, L> Transform<F, T, C, L> {
    #[inline]
    pub fn input_format(&self) -> PixelFormat {
        unsafe { ffi::cmsGetTransformInputFormat(self.handle) as PixelFormat }
    }

    #[inline]
    pub fn output_format(&self) -> PixelFormat {
        unsafe { ffi::cmsGetTransformOutputFormat(self.handle) as PixelFormat }
    }
}

impl<F, T, L> Transform<F, T, GlobalContext, L> {
    /// Adaptation state for absolute colorimetric intent, on all but cmsCreateExtendedTransform.
    ///
    /// See `ThreadContext::adaptation_state()`
    #[inline]
    pub fn global_adaptation_state() -> f64 {
        unsafe { ffi::cmsSetAdaptationState(-1.) }
    }

    /// Sets adaptation state for absolute colorimetric intent, on all but cmsCreateExtendedTransform.
    /// Little CMS can handle incomplete adaptation states.
    ///
    /// See `ThreadContext::set_adaptation_state()`
    ///
    /// Degree on adaptation 0=Not adapted, 1=Complete adaptation,  in-between=Partial adaptation.
    #[deprecated(note = "Use `ThreadContext::set_adaptation_state()`")]
    pub fn set_global_adaptation_state(value: f64) {
        unsafe {
            ffi::cmsSetAdaptationState(value);
        }
    }

    /// Sets the global codes used to mark out-out-gamut on Proofing transforms. Values are meant to be encoded in 16 bits.
    /// AlarmCodes: Array [16] of codes. ALL 16 VALUES MUST BE SPECIFIED, set to zero unused channels.
    ///
    /// See `ThreadContext::set_alarm_codes()`
    #[deprecated(note = "Use `ThreadContext::set_alarm_codes()`")]
    pub fn set_global_alarm_codes(codes: [u16; ffi::MAXCHANNELS]) {
        unsafe { ffi::cmsSetAlarmCodes(codes.as_ptr()) }
    }

    /// Gets the current global codes used to mark out-out-gamut on Proofing transforms. Values are meant to be encoded in 16 bits.
    ///
    /// See `ThreadContext::alarm_codes()`
    pub fn global_alarm_codes() -> [u16; ffi::MAXCHANNELS] {
        let mut tmp = [0u16; ffi::MAXCHANNELS];
        unsafe {
            ffi::cmsGetAlarmCodes(tmp.as_mut_ptr());
        }
        tmp
    }
}

impl<F, T, C, L> Drop for Transform<F, T, C, L> {
    fn drop(&mut self) {
        unsafe {
            ffi::cmsDeleteTransform(self.handle);
        }
    }
}


impl<F, T, C, L> fmt::Debug for Transform<F, T, C, L> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = f.debug_struct(&format!(
            "Transform<{}, {}, {}, {}>",
            std::any::type_name::<F>(),
            std::any::type_name::<T>(),
            std::any::type_name::<C>(),
            std::any::type_name::<L>(),
        ));
        s.field("input_format", &self.input_format());
        s.field("output_format", &self.output_format());
        s.finish()
    }
}
