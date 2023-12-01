use crate::{ffi, Intent};
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::os::raw::c_void;
use std::panic::RefUnwindSafe;
use std::panic::UnwindSafe;
use std::ptr;
use std::rc::Rc;
use std::sync::Arc;

/// A special case for non-thread-aware functions.
///
/// This context is used by default and you don't need to create it manually.
#[doc(hidden)]
pub struct GlobalContext {
    _not_thread_safe: UnsafeCell<YouMustUseThreadContextToShareBetweenThreads>,
}

impl UnwindSafe for GlobalContext {}
impl RefUnwindSafe for GlobalContext {}

impl UnwindSafe for ThreadContext {}
impl RefUnwindSafe for ThreadContext {}

#[doc(hidden)]
pub trait Context {
    fn as_ptr(&self) -> ffi::Context;
}

impl AsRef<GlobalContext> for GlobalContext {
    #[inline]
    fn as_ref(&self) -> &Self { self }
}

impl AsRef<ThreadContext> for ThreadContext {
    #[inline]
    fn as_ref(&self) -> &Self { self }
}

impl<'a> Context for &'a GlobalContext {
    #[inline]
    fn as_ptr(&self) -> ffi::Context {
        ptr::null_mut()
    }
}

impl Context for GlobalContext {
    #[inline]
    fn as_ptr(&self) -> ffi::Context {
        ptr::null_mut()
    }
}

#[doc(hidden)]
struct YouMustUseThreadContextToShareBetweenThreads;

unsafe impl Send for ThreadContext {}

impl<'a> Context for &'a ThreadContext {
    #[inline]
    fn as_ptr(&self) -> ffi::Context {
        self.handle
    }
}

impl<'a> Context for Arc<ThreadContext> {
    #[inline]
    fn as_ptr(&self) -> ffi::Context {
        self.handle
    }
}

impl<'a> Context for Rc<ThreadContext> {
    #[inline]
    fn as_ptr(&self) -> ffi::Context {
        self.handle
    }
}

impl Context for ThreadContext {
    #[inline]
    fn as_ptr(&self) -> ffi::Context {
        self.handle
    }
}

/// Per-thread context for multi-threaded operation.
///
/// There are situations where several instances of Little CMS engine have to coexist but on different conditions.
/// For example, when the library is used as a DLL or a shared object, diverse applications may want to use different plug-ins.
/// Another example is when multiple threads are being used in same task and the user wants to pass thread-dependent information to the memory allocators or the logging system.
/// The context is a pointer to an internal structure that keeps track of all plug-ins and static data needed by the THR corresponding function.
///
/// A context-aware app could allocate a new context by calling new() or duplicate a yet-existing one by using clone().
/// Each context can hold different plug-ins, defined by the Plugin parameter. The context can also hold loggers.
///
/// Users may associate private data across a void pointer when creating the context, and can retrieve this pointer later.
///
/// When you see an error "expected reference, found struct `lcms2::GlobalContext`", it means you've mixed global and thread-context objects. They don't work together.
/// For example, if you create a `Transform` with a context (calling `new_*_context()`), then it will only support `Profile` with a context as well.
#[repr(transparent)]
pub struct ThreadContext {
    handle: ffi::Context,
}

impl GlobalContext {
    #[must_use]
    pub fn new() -> Self {
        Self {
            _not_thread_safe: UnsafeCell::new(YouMustUseThreadContextToShareBetweenThreads),
        }
    }

    pub fn unregister_plugins(&mut self) {
        unsafe {
            ffi::cmsUnregisterPlugins();
        }
    }
}

impl ThreadContext {
    #[track_caller]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        unsafe { Self::new_handle(ffi::cmsCreateContext(ptr::null_mut(), ptr::null_mut())) }
    }

    #[track_caller]
    #[inline]
    unsafe fn new_handle(handle: ffi::Context) -> Self {
        assert!(!handle.is_null());
        Self { handle }
    }

    #[must_use]
    pub fn user_data(&self) -> *mut c_void {
        unsafe { ffi::cmsGetContextUserData(self.handle) }
    }

    pub unsafe fn install_plugin(&mut self, plugin: *mut c_void) -> bool {
        0 != ffi::cmsPluginTHR(self.handle, plugin)
    }

    pub fn unregister_plugins(&mut self) {
        unsafe {
            ffi::cmsUnregisterPluginsTHR(self.handle);
        }
    }

    #[track_caller]
    #[inline]
    #[must_use]
    pub fn supported_intents(&self) -> HashMap<Intent, &'static CStr> {
        let mut codes = [0u32; 32];
        let mut descs = [ptr::null_mut(); 32];
        let len = unsafe {
            debug_assert_eq!(mem::size_of::<Intent>(), mem::size_of::<u32>());
            ffi::cmsGetSupportedIntentsTHR(self.handle, 32, codes.as_mut_ptr(), descs.as_mut_ptr())
        };
        debug_assert!(len <= 32);
        codes.iter().zip(descs.iter()).take(len as usize).filter_map(|(&code, &desc)|{
            use Intent::*;
            let code = match code {
                c if c == Perceptual as u32 => Perceptual,
                c if c == RelativeColorimetric as u32 => RelativeColorimetric,
                c if c == Saturation as u32 => Saturation,
                c if c == AbsoluteColorimetric as u32 => AbsoluteColorimetric,

                c if c == PreserveKOnlyPerceptual as u32 => PreserveKOnlyPerceptual,
                c if c == PreserveKOnlyRelativeColorimetric as u32 => PreserveKOnlyRelativeColorimetric,
                c if c == PreserveKOnlySaturation as u32 => PreserveKOnlySaturation,
                c if c == PreserveKPlanePerceptual as u32 => PreserveKPlanePerceptual,
                c if c == PreserveKPlaneRelativeColorimetric as u32 => PreserveKPlaneRelativeColorimetric,
                c if c == PreserveKPlaneSaturation as u32 => PreserveKPlaneSaturation,
                _ => return None,
            };
            Some((code, unsafe { CStr::from_ptr(desc) }))
        }).collect()
    }

    /// Adaptation state for absolute colorimetric intent, on all but `cmsCreateExtendedTransform`.
    #[must_use]
    pub fn adaptation_state(&self) -> f64 {
        unsafe { ffi::cmsSetAdaptationStateTHR(self.handle, -1.) }
    }

    /// Sets adaptation state for absolute colorimetric intent in the given context.  Adaptation state applies on all but `cmsCreateExtendedTransformTHR`().
    /// Little CMS can handle incomplete adaptation states.
    ///
    /// Degree on adaptation 0=Not adapted, 1=Complete adaptation,  in-between=Partial adaptation.
    pub fn set_adaptation_state(&mut self, value: f64) {
        unsafe {
            ffi::cmsSetAdaptationStateTHR(self.handle, value);
        }
    }

    /// Sets the codes used to mark out-out-gamut on Proofing transforms for a given context. Values are meant to be encoded in 16 bits.
    ///
    /// `AlarmCodes`: `Array [16]` of codes. ALL 16 VALUES MUST BE SPECIFIED, set to zero unused channels.
    #[inline]
    pub fn set_alarm_codes(&mut self, codes: [u16; ffi::MAXCHANNELS]) {
        unsafe { ffi::cmsSetAlarmCodesTHR(self.handle, codes.as_ptr()) }
    }

    /// Gets the current codes used to mark out-out-gamut on Proofing transforms for the given context. Values are meant to be encoded in 16 bits.
    #[must_use]
    #[inline]
    pub fn alarm_codes(&self) -> [u16; ffi::MAXCHANNELS] {
        let mut tmp = [0u16; ffi::MAXCHANNELS];
        unsafe {
            ffi::cmsGetAlarmCodesTHR(self.handle, tmp.as_mut_ptr());
        }
        tmp
    }

    /// Sets a function to be called if there is an error.
    pub fn set_error_logging_function(&mut self, handler: ffi::LogErrorHandlerFunction) {
        unsafe {
            ffi::cmsSetLogErrorHandlerTHR(self.handle, handler);
        }
    }
}

impl Clone for ThreadContext {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { Self::new_handle(ffi::cmsDupContext(self.handle, ptr::null_mut())) }
    }
}

impl Drop for ThreadContext {
    fn drop(&mut self) {
        unsafe { ffi::cmsDeleteContext(self.handle) }
    }
}

impl Default for GlobalContext {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ThreadContext {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for ThreadContext {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ThreadContext")
    }
}

impl fmt::Debug for GlobalContext {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GlobalContext")
    }
}

#[test]
fn context() {
    let mut c = ThreadContext::new();
    assert!(c.user_data().is_null());
    c.unregister_plugins();
    assert!(crate::Profile::new_icc_context(&c, &[]).is_err());

    assert!(c.supported_intents().contains_key(&Intent::RelativeColorimetric));

    let _ = GlobalContext::default();
}
