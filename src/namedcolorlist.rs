use super::*;
use std::fmt;
use std::ptr;
use foreign_types::ForeignTypeRef;
use std::ffi::{CStr,CString};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColorInfo {
    pub name: String,
    pub prefix: String,
    pub suffix: String,
    pub pcs: [u16; 3],
    pub colorant: [u16; 16],
}

foreign_type! {
    type CType = ffi::NAMEDCOLORLIST;
    fn drop = ffi::cmsFreeNamedColorList;
    pub struct NamedColorList;
    pub struct NamedColorListRef;
}

impl NamedColorList {
    pub fn new(spot_colors: usize, colorant_count: usize, prefix: &str, suffix: &str) -> LCMSResult<Self> {
        let prefix = CString::new(prefix).unwrap();
        let suffix = CString::new(suffix).unwrap();
        unsafe {
            Error::if_null(ffi::cmsAllocNamedColorList(ptr::null_mut(),
                spot_colors as u32,
                colorant_count as u32,
                prefix.as_ptr(),
                suffix.as_ptr()))
        }
    }
}

impl NamedColorListRef {
    fn len(&self) -> usize {
        unsafe {
            ffi::cmsNamedColorCount(self.as_ptr()) as usize
        }
    }

    fn index_of(&self, color_name: &str) -> usize {
        let s = CString::new(color_name).unwrap();
        unsafe {
            ffi::cmsNamedColorIndex(self.as_ptr(), s.as_ptr()) as usize
        }
    }

    fn get(&self, index: usize) -> Option<ColorInfo> {
        let mut name = [0i8; 256];
        let mut prefix = [0i8; 33];
        let mut suffix = [0i8; 33];
        let mut pcs = [0u16; 3];
        let mut colorant = [0u16; 16];

        let ok = unsafe {
            0 != ffi::cmsNamedColorInfo(self.as_ptr(),
                index as u32,
                name.as_mut_ptr(),
                prefix.as_mut_ptr(),
                suffix.as_mut_ptr(),
                pcs.as_mut_ptr(),
                colorant.as_mut_ptr())
        };
        if ok {
            Some(unsafe {ColorInfo {
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

    fn colors(&self) -> Vec<ColorInfo> {
        (0..self.len()).filter_map(|i| self.get(i)).collect()
    }

    fn append(&mut self, color_name: &str, mut pcs: [u16; 3], mut colorant: [u16; ffi::MAXCHANNELS]) -> bool {
        let s = CString::new(color_name).unwrap();
        unsafe {
            0 != ffi::cmsAppendNamedColor(self.as_ptr(), s.as_ptr(), pcs.as_mut_ptr(), colorant.as_mut_ptr())
        }
    }
}

impl<'a> fmt::Debug for NamedColorListRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
