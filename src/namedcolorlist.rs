use crate::*;
use foreign_types::ForeignTypeRef;
use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::c_char;
use std::ptr;

#[derive(Clone, Debug, Eq, PartialEq)]
/// Color in the palette
pub struct NamedColorInfo {
    pub name: String,
    pub prefix: String,
    pub suffix: String,
    pub pcs: [u16; 3],
    pub colorant: [u16; 16],
}

foreign_type! {
    /// Palette of colors with names
    pub unsafe type NamedColorList {
        type CType = ffi::NAMEDCOLORLIST;
        fn drop = ffi::cmsFreeNamedColorList;
    }
}

impl NamedColorList {
    pub fn new(spot_colors: usize, colorant_count: usize, prefix: &str, suffix: &str) -> LCMSResult<Self> {
        let prefix = CString::new(prefix).unwrap();
        let suffix = CString::new(suffix).unwrap();
        unsafe {
            Error::if_null(ffi::cmsAllocNamedColorList(
                ptr::null_mut(),
                spot_colors as u32,
                colorant_count as u32,
                prefix.as_ptr().cast(),
                suffix.as_ptr().cast(),
            )) // char sign difference
        }
    }
}

impl NamedColorListRef {
    /// Number of colors in the palette
    #[inline]
    fn len(&self) -> usize {
        unsafe { ffi::cmsNamedColorCount(self.as_ptr()) as usize }
    }

    /// Find color by name
    fn index_of(&self, color_name: &str) -> usize {
        let s = CString::new(color_name).unwrap();
        unsafe { ffi::cmsNamedColorIndex(self.as_ptr(), s.as_ptr()) as usize }
    }

    /// Get color info
    fn get(&self, index: usize) -> Option<NamedColorInfo> {
        let mut name = [0 as c_char; 256];
        let mut prefix = [0 as c_char; 33];
        let mut suffix = [0 as c_char; 33];
        let mut pcs = [0u16; 3];
        let mut colorant = [0u16; 16];

        let ok = unsafe {
            0 != ffi::cmsNamedColorInfo(
                self.as_ptr(),
                index as u32,
                name.as_mut_ptr(),
                prefix.as_mut_ptr(),
                suffix.as_mut_ptr(),
                pcs.as_mut_ptr(),
                colorant.as_mut_ptr(),
            )
        };
        if ok {
            Some(unsafe {NamedColorInfo {
                name: CStr::from_ptr(name.as_ptr()).to_string_lossy().into_owned(),
                prefix: CStr::from_ptr(prefix.as_ptr()).to_string_lossy().into_owned(),
                suffix: CStr::from_ptr(suffix.as_ptr()).to_string_lossy().into_owned(),
                pcs,
                colorant,
            }})
        } else {
            None
        }
    }

    fn colors(&self) -> Vec<NamedColorInfo> {
        (0..self.len()).filter_map(|i| self.get(i)).collect()
    }

    /// Push a color at the end of the palette
    fn append(&mut self, color_name: &str, mut pcs: [u16; 3], mut colorant: [u16; ffi::MAXCHANNELS]) -> bool {
        let s = CString::new(color_name).unwrap();
        unsafe {
            0 != ffi::cmsAppendNamedColor(self.as_ptr(), s.as_ptr(), pcs.as_mut_ptr(), colorant.as_mut_ptr())
        }
    }
}

impl<'a> fmt::Debug for NamedColorListRef {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(c) = self.get(0) {
            write!(f, "NamedColorList({} colors: {}{}{}, etc.)", self.len(), c.prefix, c.name, c.suffix)
        } else {
            f.write_str("NamedColorList(0)")
        }
    }
}

#[test]
fn named() {
    let mut n = NamedColorList::new(10, 3, "hello", "world").unwrap();
    assert!(n.append("yellow", [1,2,3], [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]));
    assert_eq!(None, n.get(10000));

    let c = n.get(0).unwrap();
    assert_eq!("yellow", c.name);
    assert_eq!("hello", c.prefix);
    assert_eq!([1,2,3], c.pcs);
}
