use super::*;

extern crate lcms2_sys as ffi;

use std;
use std::os::raw::c_void;
use std::ffi::CString;
use std::default::Default;

impl Profile {
    pub fn new_icc(data: &[u8]) -> Profile {
        Self::new_handle(unsafe {
            ffi::cmsOpenProfileFromMem(data.as_ptr() as *const c_void, data.len() as u32)
        })
    }

    pub fn new_srgb() -> Profile {
        Self::new_handle(unsafe { ffi::cmsCreate_sRGBProfile() })
    }

    pub fn new_rgb(white_point: &CIExyY,
                   primaries: &CIExyYTRIPLE,
                   transfer_function: &[&ToneCurve])
                   -> Profile {
        assert_eq!(3, transfer_function.len());
        Self::new_handle(unsafe {
            ffi::cmsCreateRGBProfile(white_point,
                                     primaries,
                                     [transfer_function[0].handle as *const _,
                                      transfer_function[1].handle as *const _,
                                      transfer_function[2].handle as *const _]
                                         .as_ptr())
        })
    }

    pub fn new_gray(white_point: &CIExyY, curve: &ToneCurve) -> Profile {
        Self::new_handle(unsafe { ffi::cmsCreateGrayProfile(white_point, curve.handle) })
    }

    pub fn new_xyz() -> Profile {
        Self::new_handle(unsafe { ffi::cmsCreateXYZProfile() })
    }

    pub fn new_null() -> Profile {
        Self::new_handle(unsafe { ffi::cmsCreateNULLProfile() })
    }

    pub fn new_lab2(white_point: &CIExyY) -> Profile {
        Self::new_handle(unsafe { ffi::cmsCreateLab2Profile(white_point) })
    }

    pub fn new_lab4(white_point: &CIExyY) -> Profile {
        Self::new_handle(unsafe { ffi::cmsCreateLab4Profile(white_point) })
    }

    pub fn new_device_link<F, T>(transform: &Transform<F, T>, version: f64, flags: u32) -> Profile {
        Self::new_handle(unsafe { ffi::cmsTransform2DeviceLink(transform.handle, version, flags) })
    }

    fn new_handle(handle: ffi::HPROFILE) -> Profile {
        assert!(!handle.is_null());
        Profile { handle: handle }
    }

    pub fn icc(&self) -> Option<Vec<u8>> {
        unsafe {
            let mut len = 0;
            if ffi::cmsSaveProfileToMem(self.handle, std::ptr::null_mut(), &mut len) == 0 {
                return None;
            }
            let mut data = vec![0u8; len as usize];
            if len == 0 || ffi::cmsSaveProfileToMem(self.handle, data.as_mut_ptr() as *mut c_void, &mut len) == 0 {
                return None;
            }
            Some(data)
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

    pub fn info(&self, info: InfoType, languagecode: &str, countrycode: &str) -> Option<String> {
        let languagecode = CString::new(languagecode).unwrap();
        let countrycode = CString::new(countrycode).unwrap();

        let size = unsafe {
            ffi::cmsGetProfileInfo(self.handle,
                                   info,
                                   languagecode.as_ptr(),
                                   countrycode.as_ptr(),
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
                                             languagecode.as_ptr(),
                                             countrycode.as_ptr(),
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
        unsafe {
            ffi::cmsDetectTAC(self.handle)
        }
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
}

impl Drop for Profile {
    fn drop(&mut self) {
        unsafe {
            ffi::cmsCloseProfile(self.handle);
        }
    }
}


#[test]
fn icc() {
    let prof = Profile::new_xyz();
    assert!(prof.icc().unwrap().len() > 300);
}