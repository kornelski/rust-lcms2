use std::cmp;
use std::fmt;
use std::fmt::Write;
use std::os::raw::c_char;

/// Language code from ISO-639/2 and region code from ISO-3166.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Locale {
    language: [c_char; 3],
    country: [c_char; 3],
}

impl Locale {
    /// A string in format: 2-letter language name, separator, 2-letter country name, e.g. "en_US"
    #[must_use]
    pub fn new(locale_name: &str) -> Self {
        let (language_str, country_str) = locale_name.split_at(cmp::min(locale_name.len(), 3));

        let mut locale = Locale {
            language: [0; 3],
            country: [0; 3],
        };
        for (c, s) in locale.language.iter_mut().zip(language_str.bytes().take(2)) {
            *c = s as c_char;
        }
        for (c, s) in locale.country.iter_mut().zip(country_str.bytes().take(2)) {
            *c = s as c_char;
        }
        locale
    }

    /// Default/unspecified/any locale
    #[must_use]
    #[inline]
    pub fn none() -> Self {
        Locale {
            language: [0; 3],
            country: [0; 3],
        }
    }

    pub(crate) fn language_ptr(&self) -> *const c_char {
        &self.language as _
    }

    pub(crate) fn country_ptr(&self) -> *const c_char {
        &self.country as _
    }

    pub(crate) fn language_ptr_mut(&mut self) -> *mut c_char {
        std::ptr::addr_of!(self.language) as _
    }

    pub(crate) fn country_ptr_mut(&mut self) -> *mut c_char {
        std::ptr::addr_of!(self.country) as _
    }
}

impl<'a> From<&'a str> for Locale {
    fn from(s: &'a str) -> Self {
        Locale::new(s)
    }
}

impl Default for Locale {
    fn default() -> Self {
        Locale::none()
    }
}

impl fmt::Debug for Locale {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Locale as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &c in self.language.iter().take_while(|&&c| c != 0) {
            f.write_char(c as u8 as char)?;
        }
        f.write_char('_')?;
        for &c in self.country.iter().take_while(|&&c| c != 0) {
            f.write_char(c as u8 as char)?;
        }
        Ok(())
    }
}

#[test]
fn locale() {
    let l = Locale::new("");
    assert_eq!([0i8; 3], l.language);
    assert_eq!([0i8; 3], l.country);

    let l = Locale::none();
    assert_eq!([0i8; 3], l.language);
    assert_eq!([0i8; 3], l.country);

    let l = Locale::new("Ab");
    assert_eq!(['A' as c_char, 'b' as c_char, 0], l.language);
    assert_eq!([0i8; 3], l.country);

    let l = Locale::new("Ab-X");
    assert_eq!(['A' as c_char, 'b' as c_char, 0], l.language);
    assert_eq!(['X' as c_char, 0, 0], l.country);

    let l = Locale::new("overlong");
    assert_eq!(['o' as c_char, 'v' as c_char, 0], l.language);
    assert_eq!(['r' as c_char, 'l' as c_char, 0], l.country);
    unsafe {
        assert_eq!('o' as c_char, *l.language_ptr());
    }
    unsafe {
        assert_eq!('r' as c_char, *l.country_ptr());
    }
}
