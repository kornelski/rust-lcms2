use super::*;
use context::Context;
use std::path::Path;
use std::ptr;
use std::io;
use std::io::Read;
use std::fs::File;
use std::os::raw::c_void;
use std::default::Default;
use foreign_types::ForeignTypeRef;

/// An ICC color profile
pub struct Profile<Context = GlobalContext> {
    pub(crate) handle: ffi::HPROFILE,
    _context_ref: PhantomData<Context>,
}

unsafe impl<'a, C: Send> Send for Profile<C> {}

/// These are the basic functions on opening profiles.
/// For simpler operation, you must open two profiles using `new_file`, and then create a transform with these open profiles with `Transform`.
/// Using this transform you can color correct your bitmaps.
impl Profile<GlobalContext> {
    /// Parse ICC profile from the in-memory array
    pub fn new_icc(data: &[u8]) -> LCMSResult<Self> {
        Self::new_icc_context(GlobalContext::new(), data)
    }

    /// Load ICC profile file from disk
    pub fn new_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Self::new_file_context(GlobalContext::new(), path)
    }

    /// Create an ICC virtual profile for sRGB space. sRGB is a standard RGB color space created cooperatively by HP and Microsoft in 1996 for use on monitors, printers, and the Internet.
    pub fn new_srgb() -> Self {
        Self::new_srgb_context(GlobalContext::new())
    }

    /// This function creates a display RGB profile based on White point, primaries and transfer functions. It populates following tags; this conform a standard RGB Display Profile, and then I add (As per addendum II) chromaticity tag.
    ///
    ///   1. ProfileDescriptionTag
    ///   2. MediaWhitePointTag
    ///   3. RedColorantTag
    ///   4. GreenColorantTag
    ///   5. BlueColorantTag
    ///   6. RedTRCTag
    ///   7. GreenTRCTag
    ///   8. BlueTRCTag
    ///   9. Chromatic adaptation Tag
    ///   10. ChromaticityTag
    pub fn new_rgb(white_point: &CIExyY,
                   primaries: &CIExyYTRIPLE,
                   transfer_function: &[&ToneCurve])
                   -> LCMSResult<Self> {
        Self::new_rgb_context(GlobalContext::new(), white_point, primaries, transfer_function)
    }

    /// This function creates a gray profile based on White point and transfer function. It populates following tags; this conform a standard gray display profile.
    ///
    ///   1. ProfileDescriptionTag
    ///   2. MediaWhitePointTag
    ///   3. GrayTRCTag
    pub fn new_gray(white_point: &CIExyY, curve: &ToneCurve) -> LCMSResult<Self> {
        Self::new_gray_context(GlobalContext::new(), white_point, curve)
    }

    /// Creates a XYZ  XYZ identity, marking it as v4 ICC profile.  WhitePoint used in Absolute colorimetric intent  is D50.
    pub fn new_xyz() -> Self {
        Self::new_handle(unsafe { ffi::cmsCreateXYZProfile() }).unwrap()
    }

    /// Creates a fake NULL profile. This profile return 1 channel as always 0. Is useful only for gamut checking tricks.
    pub fn new_null() -> Self {
        Self::new_handle(unsafe { ffi::cmsCreateNULLProfile() }).unwrap()
    }

    /// Creates an empty profile object, ready to be populated by the programmer.
    ///
    /// WARNING: The obtained profile without adding any information is not directly useable.
    pub fn new_placeholder() -> Self {
        Self::new_handle(unsafe { ffi::cmsCreateProfilePlaceholder(ptr::null_mut()) }).unwrap()
    }

    /// This is a devicelink operating in CMYK for ink-limiting. Currently only cmsSigCmykData is supported.
    /// Limit: Amount of ink limiting in % (0..400%)
    pub fn ink_limiting(color_space: ColorSpaceSignature, limit: f64) -> LCMSResult<Self> {
        Self::new_handle(unsafe { ffi::cmsCreateInkLimitingDeviceLink(color_space, limit) })
    }

    /// Generates a device-link profile from a given color transform. This profile can then be used by any other function accepting profile handle.
    /// Depending on the specified version number, the implementation of the devicelink may vary. Accepted versions are in range 1.0…4.3
    pub fn new_device_link<F, T>(transform: &Transform<F, T>, version: f64, flags: Flags) -> LCMSResult<Self> {
        Self::new_handle(unsafe { ffi::cmsTransform2DeviceLink(transform.handle, version, flags.bits()) })
    }
}

impl<Ctx: Context> Profile<Ctx> {
    /// Create ICC file in memory buffer
    pub fn icc(&self) -> LCMSResult<Vec<u8>> {
        unsafe {
            let mut len = 0;
            if ffi::cmsSaveProfileToMem(self.handle, std::ptr::null_mut(), &mut len) == 0 {
                return Err(Error::ObjectCreationError);
            }
            let mut data = vec![0u8; len as usize];
            if len == 0 || ffi::cmsSaveProfileToMem(self.handle, data.as_mut_ptr() as *mut c_void, &mut len) == 0 {
                return Err(Error::ObjectCreationError);
            }
            Ok(data)
        }
    }

    /// Gets the device class signature from profile header.
    pub fn device_class(&self) -> ProfileClassSignature {
        unsafe { ffi::cmsGetDeviceClass(self.handle) }
    }

    /// Sets the device class signature in profile header.
    pub fn set_device_class(&mut self, cls: ProfileClassSignature) {
        unsafe { ffi::cmsSetDeviceClass(self.handle, cls) }
    }

    /// Returns the profile ICC version in the same format as it is stored in the header.
    pub fn encoded_icc_version(&self) -> u32 {
        unsafe { ffi::cmsGetEncodedICCversion(self.handle) }
    }

    pub fn set_encoded_icc_version(&self, v: u32) {
        unsafe { ffi::cmsSetEncodedICCversion(self.handle, v) }
    }

    /// Gets the attribute flags. Currently defined values correspond to the low 4 bytes of the 8 byte attribute quantity.
    ///
    ///  * `Reflective`
    ///  * `Transparency`
    ///  * `Glossy`
    ///  * `Matte`

    pub fn header_attributes(&self) -> u64 {
        let mut flags = 0;
        unsafe {
            ffi::cmsGetHeaderAttributes(self.handle, &mut flags);
        }
        flags
    }

    /// Sets the attribute flags in the profile header.
    pub fn set_header_attributes(&mut self, flags: u64) {
        unsafe {
            ffi::cmsSetHeaderAttributes(self.handle, flags);
        }
    }

    pub fn header_creator(&self) -> u32 {
        unsafe { ffi::cmsGetHeaderCreator(self.handle) }
    }

    /// Get header flags of given ICC profile object.
    ///
    /// The profile flags field does contain flags to indicate various hints for the CMM such as distributed processing and caching options.
    /// The least-significant 16 bits are reserved for the ICC. Flags in bit positions 0 and 1 shall be used as indicated in Table 7 of LCMS PDF.
    pub fn header_flags(&self) -> u32 {
        unsafe { ffi::cmsGetHeaderFlags(self.handle) }
    }

    /// Sets header flags of given ICC profile object. Valid flags are defined in Table 7 of LCMS PDF.
    pub fn set_header_flags(&mut self, flags: u32) {
        unsafe { ffi::cmsSetHeaderFlags(self.handle, flags); }
    }

    /// Returns the manufacturer signature as described in the header.
    ///
    /// This funcionality is widely superseded by the manufaturer tag. Of use only in elder profiles.
    pub fn header_manufacturer(&self) -> u32 {
        unsafe { ffi::cmsGetHeaderManufacturer(self.handle) }
    }

    /// Sets the manufacturer signature in the header.
    ///
    /// This funcionality is widely superseded by the manufaturer tag. Of use only in elder profiles.
    #[deprecated(note = "This funcionality is widely superseded by the manufaturer tag")]
    pub fn set_header_manufacturer(&mut self, m: u32) {
        unsafe { ffi::cmsSetHeaderManufacturer(self.handle, m) }
    }

    /// Returns the model signature as described in the header.
    ///
    /// This funcionality is widely superseded by the model tag. Of use only in elder profiles.
    pub fn header_model(&self) -> u32 {
        unsafe { ffi::cmsGetHeaderModel(self.handle) }
    }

    /// Sets the model signature in the profile header.
    ///
    /// This funcionality is widely superseded by the model tag. Of use only in elder profiles.
    #[deprecated(note = "This funcionality is widely superseded by the model tag")]
    pub fn set_header_model(&mut self, model: u32) {
        unsafe {
            ffi::cmsSetHeaderModel(self.handle, model);
        }
    }

    /// Gets the profile header rendering intent.
    ///
    /// From the ICC spec: “The rendering intent field shall specify the rendering intent which should be used
    /// (or, in the case of a Devicelink profile, was used) when this profile is (was) combined with another profile.
    /// In a sequence of more than two profiles, it applies to the combination of this profile and the next profile in the sequence and not to the entire sequence.
    /// Typically, the user or application will set the rendering intent dynamically at runtime or embedding time.
    /// Therefore, this flag may not have any meaning until the profile is used in some context, e.g. in a Devicelink or an embedded source profile.”
    pub fn header_rendering_intent(&self) -> Intent {
        unsafe { ffi::cmsGetHeaderRenderingIntent(self.handle) }
    }

    pub fn set_header_rendering_intent(&mut self, intent: Intent) {
        unsafe { ffi::cmsSetHeaderRenderingIntent(self.handle, intent) }
    }

    /// Gets the profile connection space used by the given profile, using the ICC convention.
    pub fn pcs(&self) -> ColorSpaceSignature {
        unsafe { ffi::cmsGetPCS(self.handle) }
    }

    /// Sets the profile connection space signature in profile header, using ICC convention.
    pub fn set_pcs(&mut self, pcs: ColorSpaceSignature) {
        unsafe { ffi::cmsSetPCS(self.handle, pcs) }
    }

    pub fn info(&self, info: InfoType, locale: Locale) -> Option<String> {
        let size = unsafe {
            ffi::cmsGetProfileInfo(self.handle,
                                   info,
                                   locale.language_ptr(),
                                   locale.country_ptr(),
                                   std::ptr::null_mut(),
                                   0)
        };
        if 0 == size {
            return None;
        }

        let wchar_bytes = std::mem::size_of::<ffi::wchar_t>();
        let mut data = vec![0; size as usize / wchar_bytes];
        unsafe {
            let len = data.len() * wchar_bytes;
            let res = ffi::cmsGetProfileInfo(self.handle,
                                             info,
                                             locale.language_ptr(),
                                             locale.country_ptr(),
                                             (&mut data).as_mut_ptr(),
                                             len as u32);
            if 0 == res {
                return None;
            }
        }
        Some(data.into_iter()
            .take_while(|&c| c > 0)
            .map(|c| std::char::from_u32(c as u32).unwrap())
            .collect())
    }

    /// Returns the profile ICC version. The version is decoded to readable floating point format.
    pub fn version(&self) -> f64 {
        unsafe { ffi::cmsGetProfileVersion(self.handle) }
    }

    /// Sets the ICC version in profile header. The version is given to this function as a float n.m
    pub fn set_version(&mut self, ver: f64) {
        unsafe {
            ffi::cmsSetProfileVersion(self.handle, ver);
        }
    }

    pub fn tag_signatures(&self) -> Vec<TagSignature> {
        unsafe {
            (0..ffi::cmsGetTagCount(self.handle)).map(|n| ffi::cmsGetTagSignature(self.handle, n as u32)).collect()
        }
    }

    pub fn detect_black_point(&self, intent: Intent) -> Option<CIEXYZ> {
        unsafe {
            let mut b = Default::default();
            if ffi::cmsDetectBlackPoint(&mut b, self.handle, intent, 0) != 0 {
                Some(b)
            } else {
                None
            }
        }
    }

    pub fn detect_destination_black_point(&self, intent: Intent) -> Option<CIEXYZ> {
        unsafe {
            let mut b = Default::default();
            if ffi::cmsDetectDestinationBlackPoint(&mut b, self.handle, intent, 0) != 0 {
                Some(b)
            } else {
                None
            }
        }
    }

    pub fn detect_tac(&self) -> f64 {
        unsafe { ffi::cmsDetectTAC(self.handle) }
    }

    /// Gets the color space used by the given profile, using the ICC convention.
    pub fn color_space(&self) -> ColorSpaceSignature {
        unsafe {
            let v = ffi::cmsGetColorSpace(self.handle);
            if 0 != v as u32 {v} else {ColorSpaceSignature::Sig1colorData}
        }
    }

    /// Sets the profile connection space signature in profile header, using ICC convention.
    pub fn set_color_space(&mut self, sig: ColorSpaceSignature) {
        unsafe { ffi::cmsSetColorSpace(self.handle, sig) }
    }

    pub fn is_clut(&self, intent: Intent, used_direction: u32) -> bool {
        unsafe { ffi::cmsIsCLUT(self.handle, intent, used_direction) != 0 }
    }

    pub fn is_intent_supported(&self, intent: Intent, used_direction: u32) -> bool {
        unsafe { ffi::cmsIsIntentSupported(self.handle, intent, used_direction) != 0 }
    }

    pub fn is_matrix_shaper(&self) -> bool {
        unsafe { ffi::cmsIsMatrixShaper(self.handle) != 0 }
    }

    pub fn has_tag(&self, sig: TagSignature) -> bool {
        unsafe { ffi::cmsIsTag(self.handle, sig) != 0 }
    }

    pub fn read_tag(&self, sig: TagSignature) -> Tag {
        unsafe { Tag::new(sig, ffi::cmsReadTag(self.handle, sig) as *const u8) }
    }

    pub fn write_tag(&mut self, sig: TagSignature, tag: Tag) -> bool {
        unsafe {
            ffi::cmsWriteTag(self.handle, sig, tag.data_for_signature(sig) as *const _) != 0
        }
    }

    /// Retrieves the Profile ID stored in the profile header.
    pub fn profile_id(&self) -> ffi::ProfileID {
        let mut id = ffi::ProfileID::default();
        unsafe {
            ffi::cmsGetHeaderProfileID(self.handle, &mut id as *mut ffi::ProfileID as *mut _);
        }
        id
    }

    /// Computes a MD5 checksum and stores it as Profile ID in the profile header.
    pub fn set_default_profile_id(&mut self) {
        unsafe {
            ffi::cmsMD5computeID(self.handle);
        }
    }

    pub fn set_profile_id(&mut self, id: ffi::ProfileID) {
        unsafe {
            ffi::cmsSetHeaderProfileID(self.handle, &id as *const ffi::ProfileID as *mut _);
        }
    }
}

/// Per-context functions that can be used with a `ThreadContext`
impl<Ctx: Context> Profile<Ctx> {
    pub fn new_icc_context(context: Ctx, data: &[u8]) -> LCMSResult<Self> {
        Self::new_handle(unsafe {
            ffi::cmsOpenProfileFromMemTHR(context.as_ptr(), data.as_ptr() as *const c_void, data.len() as u32)
        })
    }

    pub fn new_file_context<P: AsRef<Path>>(context: Ctx, path: P) -> io::Result<Self> {
        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf)?;
        Self::new_icc_context(context, &buf).map_err(|_| io::ErrorKind::Other.into())
    }

    pub fn new_srgb_context(context: Ctx) -> Self {
        Self::new_handle(unsafe { ffi::cmsCreate_sRGBProfileTHR(context.as_ptr()) }).unwrap()
    }

    pub fn new_rgb_context(context: Ctx, white_point: &CIExyY,
                   primaries: &CIExyYTRIPLE,
                   transfer_function: &[&ToneCurve])
                   -> LCMSResult<Self> {
        assert_eq!(3, transfer_function.len());
        Self::new_handle(unsafe {
            ffi::cmsCreateRGBProfileTHR(context.as_ptr(),
                                     white_point,
                                     primaries,
                                     [transfer_function[0].as_ptr() as *const _,
                                      transfer_function[1].as_ptr() as *const _,
                                      transfer_function[2].as_ptr() as *const _]
                                         .as_ptr())
        })
    }

    pub fn new_gray_context(context: Ctx, white_point: &CIExyY, curve: &ToneCurve) -> LCMSResult<Self> {
        Self::new_handle(unsafe { ffi::cmsCreateGrayProfileTHR(context.as_ptr(), white_point, curve.as_ptr()) })
    }

    /// This is a devicelink operating in the target colorspace with as many transfer functions as components.
    /// Number of tone curves must be sufficient for the color space.
    pub unsafe fn new_linearization_device_link_context(context: Ctx, color_space: ColorSpaceSignature, curves: &[ToneCurveRef]) -> LCMSResult<Self> {
        let v: Vec<_> = curves.iter().map(|c| c.as_ptr() as *const _).collect();
        Self::new_handle(ffi::cmsCreateLinearizationDeviceLinkTHR(context.as_ptr(), color_space, v.as_ptr()))
    }

    /// Creates an abstract devicelink operating in Lab for Bright/Contrast/Hue/Saturation and white point translation.
    /// White points are specified as temperatures ºK
    ///
    /// nLUTPoints : Resulting color map resolution
    /// Bright: Bright increment. May be negative
    /// Contrast : Contrast increment. May be negative.
    /// Hue : Hue displacement in degree.
    /// Saturation: Saturation increment. May be negative
    /// TempSrc: Source white point temperature
    /// TempDest: Destination white point temperature.
    /// To prevent white point adjustment, set Temp to None
    pub fn new_bchsw_abstract_context(context: Ctx, lut_points: usize, bright: f64, contrast: f64, hue: f64, saturation: f64,
                                      temp_src_dst: Option<(u32, u32)>) -> LCMSResult<Self> {
        let (temp_src, temp_dest) = temp_src_dst.unwrap_or((0,0));
        Self::new_handle(unsafe {
            ffi::cmsCreateBCHSWabstractProfileTHR(context.as_ptr(), lut_points as i32, bright, contrast, hue, saturation, temp_src as i32, temp_dest as i32)
        })
    }

    fn new_handle(handle: ffi::HPROFILE) -> LCMSResult<Self> {
        if handle.is_null() {
            return Err(Error::ObjectCreationError);
        }
        Ok(Profile {
            handle,
            _context_ref: PhantomData,
        })
    }

    /// This is a devicelink operating in CMYK for ink-limiting. Currently only cmsSigCmykData is supported.
    /// Limit: Amount of ink limiting in % (0..400%)
    pub fn ink_limiting_context(context: Ctx, color_space: ColorSpaceSignature, limit: f64) -> LCMSResult<Self> {
        Self::new_handle(unsafe { ffi::cmsCreateInkLimitingDeviceLinkTHR(context.as_ptr(), color_space, limit) })
    }

    /// Creates a XYZ  XYZ identity, marking it as v4 ICC profile.  WhitePoint used in Absolute colorimetric intent  is D50.
    pub fn new_xyz_context(context: Ctx) -> Self {
        Self::new_handle(unsafe { ffi::cmsCreateXYZProfileTHR(context.as_ptr()) }).unwrap()
    }

    /// Creates a fake NULL profile. This profile return 1 channel as always 0. Is useful only for gamut checking tricks.
    pub fn new_null_context(context: Ctx) -> Self {
        Self::new_handle(unsafe { ffi::cmsCreateNULLProfileTHR(context.as_ptr()) }).unwrap()
    }

    /// Creates a Lab  Lab identity, marking it as v2 ICC profile.
    ///
    /// Adjustments for accomodating PCS endoing shall be done by Little CMS when using this profile.
    pub fn new_lab2_context(context: Ctx, white_point: &CIExyY) -> LCMSResult<Self> {
        Self::new_handle(unsafe { ffi::cmsCreateLab2ProfileTHR(context.as_ptr(), white_point) })
    }

    /// Creates a Lab  Lab identity, marking it as v4 ICC profile.
    pub fn new_lab4_context(context: Ctx, white_point: &CIExyY) -> LCMSResult<Self> {
        Self::new_handle(unsafe { ffi::cmsCreateLab4ProfileTHR(context.as_ptr(), white_point) })
    }
}

impl<Context> Drop for Profile<Context> {
    fn drop(&mut self) {
        unsafe {
            ffi::cmsCloseProfile(self.handle);
        }
    }
}


#[test]
fn tags_read() {
    let prof = Profile::new_srgb();
    assert!(prof.read_tag(TagSignature::BToD0Tag).is_none());
    assert_eq!(CIEXYZ::d50().X, match prof.read_tag(TagSignature::MediaWhitePointTag) {
        Tag::CIEXYZ(xyz) => xyz.X,
        _ => panic!(),
    });
}

#[test]
fn tags_write() {
    let mut p = Profile::new_placeholder();
    let mut mlu = MLU::new(1);
    mlu.set_text_ascii("Testing", Locale::new("en_GB"));
    assert!(p.write_tag(TagSignature::CopyrightTag, Tag::MLU(&mlu)));

    let xyz = CIEXYZ{X:1., Y:2., Z:3.};
    assert!(p.write_tag(TagSignature::RedColorantTag, Tag::CIEXYZ(&xyz)));

    assert!(p.has_tag(TagSignature::CopyrightTag));
    assert!(p.has_tag(TagSignature::RedColorantTag));
    assert!(!p.has_tag(TagSignature::BlueColorantTag));

    assert_eq!(&xyz, match p.read_tag(TagSignature::RedColorantTag) {
        Tag::CIEXYZ(d) => d,
        _ => panic!(),
    });

    assert_eq!(Ok("Testing".to_owned()), match p.read_tag(TagSignature::CopyrightTag) {
        Tag::MLU(mlu) => mlu.text(Locale::none()),
        _ => panic!(),
    });
}

#[test]
fn setters() {
    let mut p = Profile::new_placeholder();
    assert_eq!(ColorSpaceSignature::Sig1colorData, p.color_space());
    p.set_color_space(ColorSpaceSignature::RgbData);
    assert_eq!(ColorSpaceSignature::RgbData, p.color_space());
}

#[test]
fn icc() {
    let prof = Profile::new_xyz();
    assert!(prof.icc().unwrap().len() > 300);
}

#[test]
fn bad_icc() {
    let err = Profile::new_icc(&[1, 2, 3]);
    assert!(err.is_err());
}
