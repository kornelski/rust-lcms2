use std::rc::Rc;
use std::sync::Arc;
use super::*;
use std::ptr;
use std::mem;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::panic::UnwindSafe;
use std::panic::RefUnwindSafe;

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
    fn as_ref(&self) -> &Self { self }
}

impl AsRef<ThreadContext> for ThreadContext {
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
pub struct ThreadContext {
    handle: ffi::Context,
}

impl GlobalContext {
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
    pub fn new() -> Self {
        unsafe {
            Self::new_handle(ffi::cmsCreateContext(ptr::null_mut(), ptr::null_mut()))
        }
    }

    unsafe fn new_handle(handle: ffi::Context) -> Self {
        assert!(!handle.is_null());
        Self { handle }
    }

    pub fn user_data(&self) -> *mut c_void {
        unsafe {
            ffi::cmsGetContextUserData(self.handle)
        }
    }

    pub unsafe fn install_plugin(&mut self, plugin: *mut c_void) -> bool {
        0 != ffi::cmsPluginTHR(self.handle, plugin)
    }

    pub fn unregister_plugins(&mut self) {
        unsafe {
            ffi::cmsUnregisterPluginsTHR(self.handle);
        }
    }

    pub fn supported_intents(&self) -> HashMap<Intent, &CStr> {
        let mut codes = [Intent::PreserveKOnlySaturation; 32];
        let mut descs = [ptr::null_mut(); 32];
        let len = unsafe {
            assert_eq!(mem::size_of::<Intent>(), mem::size_of::<u32>());
            ffi::cmsGetSupportedIntentsTHR(self.handle, 32, &mut codes as *mut _ as *mut u32, descs.as_mut_ptr())
        };
        assert!(len <= 32);
        codes.iter().zip(descs.iter()).take(len as usize).map(|(&code,&desc)|{
            (code, unsafe {CStr::from_ptr(desc)})
        }).collect()
    }

    /// Adaptation state for absolute colorimetric intent, on all but cmsCreateExtendedTransform.
    pub fn adaptation_state(&self) -> f64 {
        unsafe { ffi::cmsSetAdaptationStateTHR(self.handle, -1.) }
    }

    /// Sets adaptation state for absolute colorimetric intent in the given context.  Adaptation state applies on all but cmsCreateExtendedTransformTHR().
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
    /// AlarmCodes: Array [16] of codes. ALL 16 VALUES MUST BE SPECIFIED, set to zero unused channels.
    pub fn set_alarm_codes(&mut self, codes: [u16; ffi::MAXCHANNELS]) {
        unsafe { ffi::cmsSetAlarmCodesTHR(self.handle, codes.as_ptr()) }
    }

    /// Gets the current codes used to mark out-out-gamut on Proofing transforms for the given context. Values are meant to be encoded in 16 bits.
    pub fn alarm_codes(&self) -> [u16; ffi::MAXCHANNELS] {
        let mut tmp = [0u16; ffi::MAXCHANNELS];
        unsafe {
            ffi::cmsGetAlarmCodesTHR(self.handle, tmp.as_mut_ptr());
        }
        tmp
    }
}

impl Clone for ThreadContext {
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
    fn default() -> Self {
        Self::new()
        }
}

impl Default for ThreadContext {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn context() {
    let mut c = ThreadContext::new();
    assert!(c.user_data().is_null());
    c.unregister_plugins();
    assert!(Profile::new_icc_context(&c, &[]).is_err());

    assert!(c.supported_intents().contains_key(&Intent::RelativeColorimetric));

    let _ = GlobalContext::default();
}
