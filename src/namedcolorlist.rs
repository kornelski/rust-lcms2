use super::*;
use std::fmt;

foreign_type! {
    type CType = ffi::NAMEDCOLORLIST;
    fn drop = ffi::cmsFreeNamedColorList;
    pub struct NamedColorList;
    pub struct NamedColorListRef;
}

impl<'a> fmt::Debug for NamedColorListRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NAMEDCOLORLIST")
    }
}
