use crate::ffi;
use std::ops;

#[derive(Debug, Copy, Clone)]
/// Flags for creating `Transform`. Can be OR-ed together with `|`.
///
/// There's a special `NO_CACHE` flag that enables sharing transform between threads.
pub struct Flags<T: CacheFlag = AllowCache>(pub u32, T);

impl Flags {
    /// Inhibit 1-pixel cache. This is required to make `Transform` implement `Sync`
    pub const NO_CACHE: Flags<DisallowCache> = Flags(ffi::FLAGS_NOCACHE, DisallowCache);
    /// Inhibit optimizations
    pub const NO_OPTIMIZE: Flags = Flags(ffi::FLAGS_NOOPTIMIZE, AllowCache);
    /// Don't transform anyway
    pub const NULL_TRANSFORM: Flags = Flags(ffi::FLAGS_NULLTRANSFORM, AllowCache);

    /// Proofing flags
    /// Out of Gamut alarm
    pub const GAMUT_CHECK: Flags = Flags(ffi::FLAGS_GAMUTCHECK, AllowCache);
    /// Do softproofing
    pub const SOFT_PROOFING: Flags = Flags(ffi::FLAGS_SOFTPROOFING, AllowCache);

    // Misc
    pub const BLACKPOINT_COMPENSATION: Flags = Flags(ffi::FLAGS_BLACKPOINTCOMPENSATION, AllowCache);
    /// Don't fix scum dot
    pub const NO_WHITE_ON_WHITE_FIXUP: Flags = Flags(ffi::FLAGS_NOWHITEONWHITEFIXUP, AllowCache);
    /// Use more memory to give better accurancy
    pub const HIGHRES_PRECALC: Flags = Flags(ffi::FLAGS_HIGHRESPRECALC, AllowCache);
    /// Use less memory to minimize resources
    pub const LOWRES_PRECALC: Flags = Flags(ffi::FLAGS_LOWRESPRECALC, AllowCache);

    /// For devicelink creation
    /// Create 8 bits devicelinks
    pub const DEVICELINK_8BITS: Flags = Flags(ffi::FLAGS_8BITS_DEVICELINK, AllowCache);
    /// Guess device class (for transform2devicelink)
    pub const GUESS_DEVICE_CLASS: Flags = Flags(ffi::FLAGS_GUESSDEVICECLASS, AllowCache);
    /// Keep profile sequence for devicelink creation
    pub const KEEP_SEQUENCE: Flags = Flags(ffi::FLAGS_KEEP_SEQUENCE, AllowCache);

    /// Specific to a particular optimizations
    /// Force CLUT optimization
    pub const FORCE_CLUT: Flags = Flags(ffi::FLAGS_FORCE_CLUT, AllowCache);
    /// create postlinearization tables if possible
    pub const CLUT_POST_LINEARIZATION: Flags = Flags(ffi::FLAGS_CLUT_POST_LINEARIZATION, AllowCache);
    /// create prelinearization tables if possible
    pub const CLUT_PRE_LINEARIZATION: Flags = Flags(ffi::FLAGS_CLUT_PRE_LINEARIZATION, AllowCache);

    /// Specific to unbounded mode
    /// Prevent negative numbers in floating point transforms
    pub const NO_NEGATIVES: Flags = Flags(ffi::FLAGS_NONEGATIVES, AllowCache);

    /// Alpha channels are copied on cmsDoTransform()
    pub const COPY_ALPHA: Flags = Flags(ffi::FLAGS_COPY_ALPHA, AllowCache);

    /// CRD special
    pub const NO_DEFAULT_RESOURCE_DEF: Flags = Flags(ffi::FLAGS_NODEFAULTRESOURCEDEF, AllowCache);
}

impl<T: CacheFlag> ops::BitOr<Flags<T>> for Flags<DisallowCache> {
    type Output = Flags<DisallowCache>;
    fn bitor(self, other: Flags<T>) -> Flags<DisallowCache> {
        Flags(self.0 | other.0, DisallowCache)
    }
}

impl<T: CacheFlag> ops::BitOr<Flags<T>> for Flags<AllowCache> {
    type Output = Flags<T>;
    fn bitor(self, other: Flags<T>) -> Flags<T> {
        Flags(self.0 | other.0, other.1)
    }
}

impl<T: CacheFlag> Flags<T> {
    pub(crate) fn bits(&self) -> u32 {
        self.0
    }

    pub(crate) fn allow_cache(&self) -> Flags {
        Flags(self.0, AllowCache)
    }

    pub fn has<F: CacheFlag>(&mut self, flag: Flags<F>) -> bool {
        0 != (self.0 & flag.0)
    }
}

impl Default for Flags {
    /// Default flags
    ///
    /// By default allows non-thread-safe cache, which improves performance, but limits transforms to use by one thread only.
    #[inline]
    fn default() -> Self {
        Flags(0, AllowCache)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct DisallowCache;
#[derive(Copy, Clone, Debug)]
pub struct AllowCache;

pub trait CacheFlag: Sized {}
impl CacheFlag for AllowCache {}
impl CacheFlag for DisallowCache {}

#[test]
fn flags() {
    let _ = Flags::default();
    let mut t = Flags::COPY_ALPHA | Flags::NO_OPTIMIZE;
    t = t | Flags::CLUT_PRE_LINEARIZATION;
    assert!(t.has(Flags::CLUT_PRE_LINEARIZATION));
    assert!(t.has(Flags::COPY_ALPHA));
    assert!(t.has(Flags::NO_OPTIMIZE));
    assert!(!t.has(Flags::DEVICELINK_8BITS));
    assert!(!t.has(Flags::GAMUT_CHECK));
    let _ = Flags::default() | Flags::NO_CACHE;
    let _ = Flags::NO_CACHE | Flags::CLUT_PRE_LINEARIZATION;
}
