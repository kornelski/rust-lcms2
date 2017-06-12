use std::cmp;
use std::fmt;
use std::fmt::Write;

/// Language code from ISO-639/2 and region code from ISO-3166.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Locale {
    language: [i8; 3],
    country: [i8; 3],
}

impl Locale {
    /// A string in format: 2-letter language name, separator, 2-letter country name, e.g. "en_US"
    pub fn new(locale_name: &str) -> Self {
        let (language_str, country_str) = locale_name.split_at(cmp::min(locale_name.len(), 3));

        let mut locale = Locale {
            language: [0; 3],
            country: [0; 3],
        };
        for (c,s) in locale.language.iter_mut().zip(language_str.bytes().take(2)) {
            *c = s as i8;
        }
        for (c,s) in locale.country.iter_mut().zip(country_str.bytes().take(2)) {
            *c = s as i8;
        }
        locale
    }

    /// Default/unspecified/any locale
    pub fn none() -> Self {
        Locale {
            language: [0; 3],
            country: [0; 3],
        }
    }

    pub(crate) fn language_ptr(&self) -> *const i8 {
        &self.language as _
    }

    pub(crate) fn country_ptr(&self) -> *const i8 {
        &self.country as _
    }

    pub(crate) fn language_ptr_mut(&mut self) -> *mut i8 {
        &self.language as *const i8 as _
    }

    pub(crate) fn country_ptr_mut(&mut self) -> *mut i8 {
        &self.country as *const i8 as _
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Locale as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    assert_eq!([0i8;3], l.language);
    assert_eq!([0i8;3], l.country);

    let l = Locale::none();
    assert_eq!([0i8;3], l.language);
    assert_eq!([0i8;3], l.country);

    let l = Locale::new("Ab");
    assert_eq!(['A' as i8,'b' as i8,0], l.language);
    assert_eq!([0i8;3], l.country);

    let l = Locale::new("Ab-X");
    assert_eq!(['A' as i8,'b' as i8,0], l.language);
    assert_eq!(['X' as i8,0,0], l.country);

    let l = Locale::new("overlong");
    assert_eq!(['o' as i8,'v' as i8,0], l.language);
    assert_eq!(['r' as i8,'l' as i8,0], l.country);
    unsafe {
        assert_eq!('o' as i8, *l.language_ptr());
    }
    unsafe {
        assert_eq!('r' as i8, *l.country_ptr());
    }
}
