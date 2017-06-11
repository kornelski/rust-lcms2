use super::*;
use std::path::Path;
use std::ptr;
use std::io;
use std::io::Read;
use std::fs::File;
use std::os::raw::c_void;
use std::default::Default;
use foreign_types::ForeignTypeRef;

impl Profile {
    pub fn new_icc(data: &[u8]) -> Result<Self, Error> {
        Self::new_handle(unsafe {
            ffi::cmsOpenProfileFromMem(data.as_ptr() as *const c_void, data.len() as u32)
        })
    }

    pub fn new_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf)?;
        Self::new_icc(&buf).map_err(|_| io::ErrorKind::Other.into())
    }

    /// Create an ICC virtual profile for sRGB space. sRGB is a standard RGB color space created cooperatively by HP and Microsoft in 1996 for use on monitors, printers, and the Internet.
    pub fn new_srgb() -> Self {
        Self::new_handle(unsafe { ffi::cmsCreate_sRGBProfile() }).unwrap()
    }

    pub fn new_rgb(white_point: &CIExyY,
                   primaries: &CIExyYTRIPLE,
                   transfer_function: &[&ToneCurve])
                   -> Result<Self, Error> {
        assert_eq!(3, transfer_function.len());
        Self::new_handle(unsafe {
            ffi::cmsCreateRGBProfile(white_point,
                                     primaries,
                                     [transfer_function[0].as_ptr() as *const _,
                                      transfer_function[1].as_ptr() as *const _,
                                      transfer_function[2].as_ptr() as *const _]
                                         .as_ptr())
        })
    }

    pub fn new_gray(white_point: &CIExyY, curve: &ToneCurve) -> Result<Self, Error> {
        Self::new_handle(unsafe { ffi::cmsCreateGrayProfile(white_point, curve.as_ptr()) })
    }

    /// Number of tone curves must be sufficient for the color space
    pub unsafe fn new_linearization_device_link(color_space: ColorSpaceSignature, curves: &[ToneCurveRef]) -> LCMSResult<Self> {
        let v: Vec<_> = curves.iter().map(|c| c.as_ptr() as *const _).collect();
        Self::new_handle(ffi::cmsCreateLinearizationDeviceLink(color_space, v.as_ptr()))
    }

    /// This is a devicelink operating in CMYK for ink-limiting. Currently only cmsSigCmykData is supported.
    /// Limit: Amount of ink limiting in % (0..400%)
    pub fn ink_limiting(color_space: ColorSpaceSignature, limit: f64) -> LCMSResult<Self> {
        Self::new_handle(unsafe {ffi::cmsCreateInkLimitingDeviceLink(color_space, limit)})
    }

    pub fn new_xyz() -> Profile {
        Self::new_handle(unsafe { ffi::cmsCreateXYZProfile() }).unwrap()
    }

    pub fn new_null() -> Profile {
        Self::new_handle(unsafe { ffi::cmsCreateNULLProfile() }).unwrap()
    }

    pub fn new_placeholder() -> Self {
        Self::new_handle(unsafe { ffi::cmsCreateProfilePlaceholder(ptr::null_mut()) }).unwrap()
    }

    pub fn new_lab2(white_point: &CIExyY) -> Result<Self, Error> {
        Self::new_handle(unsafe { ffi::cmsCreateLab2Profile(white_point) })
    }

    pub fn new_lab4(white_point: &CIExyY) -> Result<Self, Error> {
        Self::new_handle(unsafe { ffi::cmsCreateLab4Profile(white_point) })
    }

    pub fn new_device_link<F, T>(transform: &Transform<F, T>, version: f64, flags: u32) -> Result<Self, Error> {
        Self::new_handle(unsafe { ffi::cmsTransform2DeviceLink(transform.handle, version, flags) })
    }

    fn new_handle(handle: ffi::HPROFILE) -> Result<Self, Error> {
        if handle.is_null() {
            return Err(Error::ObjectCreationError);
        }
        Ok(Profile { handle: handle })
    }

    pub fn icc(&self) -> Result<Vec<u8>, Error> {
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

    pub fn device_class(&self) -> ProfileClassSignature {
        unsafe { ffi::cmsGetDeviceClass(self.handle) }
    }
    pub fn encoded_icc_version(&self) -> u32 {
        unsafe { ffi::cmsGetEncodedICCversion(self.handle) }
    }
    pub fn header_attributes(&self) -> u64 {
        let mut flags = 0;
        unsafe {
            ffi::cmsGetHeaderAttributes(self.handle, &mut flags);
        }
        flags
    }

    pub fn header_creator(&self) -> u32 {
        unsafe { ffi::cmsGetHeaderCreator(self.handle) }
    }
    pub fn header_flags(&self) -> u32 {
        unsafe { ffi::cmsGetHeaderFlags(self.handle) }
    }
    pub fn header_manufacturer(&self) -> u32 {
        unsafe { ffi::cmsGetHeaderManufacturer(self.handle) }
    }
    pub fn header_model(&self) -> u32 {
        unsafe { ffi::cmsGetHeaderModel(self.handle) }
    }

    pub fn header_rendering_intent(&self) -> u32 {
        unsafe { ffi::cmsGetHeaderRenderingIntent(self.handle) }
    }
    pub fn pcs(&self) -> ColorSpaceSignature {
        unsafe { ffi::cmsGetPCS(self.handle) }
    }

    fn context_id(&self) -> Context {
        unsafe { ffi::cmsGetProfileContextID(self.handle) }
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

    pub fn version(&self) -> f64 {
        unsafe { ffi::cmsGetProfileVersion(self.handle) }
    }

    pub fn tag_signatures(&self) -> Vec<TagSignature> {
        unsafe {
            (0..ffi::cmsGetTagCount(self.handle)).map(|n| ffi::cmsGetTagSignature(self.handle, n as u32)).collect()
        }
    }

    pub fn detect_black_point(&self, intent: Intent, flags: u32) -> Option<CIEXYZ> {
        unsafe {
            let mut b = Default::default();
            if ffi::cmsDetectBlackPoint(&mut b, self.handle, intent, flags) != 0 {
                Some(b)
            } else {
                None
            }
        }
    }

    pub fn detect_destination_black_point(&self, intent: Intent, flags: u32) -> Option<CIEXYZ> {
        unsafe {
            let mut b = Default::default();
            if ffi::cmsDetectDestinationBlackPoint(&mut b, self.handle, intent, flags) != 0 {
                Some(b)
            } else {
                None
            }
        }
    }

    pub fn detect_tac(&self) -> f64 {
        unsafe { ffi::cmsDetectTAC(self.handle) }
    }

    pub fn color_space(&self) -> ColorSpaceSignature {
        unsafe { ffi::cmsGetColorSpace(self.handle) }
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

    pub fn read_tag<'a>(&'a self, sig: TagSignature) -> Tag<'a> {
        unsafe { Tag::new(sig, ffi::cmsReadTag(self.handle, sig) as *const u8) }
    }
}

impl Drop for Profile {
    fn drop(&mut self) {
        unsafe {
            ffi::cmsCloseProfile(self.handle);
        }
    }
}


#[test]
fn tags() {
    let prof = Profile::new_srgb();
    assert!(prof.read_tag(TagSignature::BToD0Tag).is_none());
    assert_eq!(CIEXYZ::d50().X, match prof.read_tag(TagSignature::MediaWhitePointTag) {
        Tag::CIEXYZ(xyz) => xyz.X,
        _ => panic!(),
    });
}

#[test]
fn icc() {
    let prof = Profile::new_xyz();
    assert!(prof.icc().unwrap().len() > 300);
}

#[test]
fn bad_icc() {
    let err = Profile::new_icc(&[1,2,3]);
    assert!(err.is_err());
}
