use super::*;

pub trait ColorSpaceSignatureExt: Sized + Copy {
    fn channels(self) -> u32;
}

impl ColorSpaceSignatureExt for ColorSpaceSignature {
    fn channels(self) -> u32 {
        unsafe {
            ffi::cmsChannelsOf(self)
        }
    }
}

/// Chromatic adaptation
pub trait CIEXYZExt: Sized {
    /// Adapts a color to a given illuminant. Original color is expected to have
    /// a `source_white_point` white point.
    fn adapt_to_illuminant(&self, source_white_point: &CIEXYZ, illuminant: &CIEXYZ) -> Option<CIEXYZ>;
}

impl CIEXYZExt for CIEXYZ {
    fn adapt_to_illuminant(&self, source_white_point: &CIEXYZ, illuminant: &CIEXYZ) -> Option<CIEXYZ> {
        let mut res = CIEXYZ{X:0.,Y:0.,Z:0.};
        let ok = unsafe {
            ffi::cmsAdaptToIlluminant(&mut res, source_white_point, illuminant, self) != 0
        };
        if ok {
            Some(res)
        } else {
            None
        }
    }
}

/// Delta E
pub trait CIELabExt: Sized {
    ///  Delta-E 2000 is the first major revision of the dE94 equation.
    ///
    ///  Unlike dE94, which assumes that L\* correctly reflects the perceived differences in lightness, dE2000 varies the weighting of L\* depending on where in the lightness range the color falls.
    ///  dE2000 is still under consideration and does not seem to be widely supported in graphics arts applications.
    ///
    ///  Returns:
    ///  The CIE2000 dE metric value.
    fn cie2000_delta_e(&self, other: &CIELab, kl: f64, kc: f64, kh: f64) -> f64;

    /// A technical committee of the CIE (TC1-29) published an equation in 1995 called CIE94.
    /// The equation is similar to CMC but the weighting functions are largely based on RIT/DuPont tolerance data derived from automotive paint experiments where sample surfaces are smooth.
    /// It also has ratios, labeled kL (lightness) and Kc (chroma) and the commercial factor (cf) but these tend to be preset in software and are not often exposed for the user (as it is the case in Little CMS).
    /// Returns:
    /// The CIE94 dE metric value.
    fn cie94_delta_e(&self, other: &CIELab) -> f64;

    /// BFD delta E metric.
    fn bfd_delta_e(&self, other: &CIELab) -> f64;

    /// The dE76 metric value.
    ///
    /// The L\*a\*b\* color space was devised in 1976 and, at the same time delta-E 1976 (dE76) came into being.
    /// If you can imagine attaching a string to a color point in 3D Lab space, dE76 describes the sphere that is described by all the possible directions you could pull the string.
    /// If you hear people speak of just plain 'delta-E' they are probably referring to dE76. It is also known as dE-Lab and dE-ab.
    ///
    /// One problem with dE76 is that Lab itself is not 'perceptually uniform' as its creators had intended.
    /// So different amounts of visual color shift in different color areas of Lab might have the same dE76 number.
    /// Conversely, the same amount of color shift might result in different dE76 values.
    /// Another issue is that the eye is most sensitive to hue differences, then chroma and finally lightness and dE76 does not take this into account.
    fn delta_e(&self, other: &CIELab) -> f64;

    /// In 1984 the CMC (Colour Measurement Committee of the Society of Dyes and Colourists of Great Britain) developed and adopted an equation based on LCH numbers.
    ///
    /// Intended for the textiles industry, CMC l:c allows the setting of lightness (l) and chroma (c) factors. As the eye is more sensitive to chroma, the default ratio for l:c is 2:1 allowing for 2x the difference in lightness than chroma (numbers). There is also a 'commercial factor' (cf) which allows an overall varying of the size of the tolerance region according to accuracy requirements. A cf=1.0 means that a delta-E CMC value <1.0 is acceptable.
    /// CMC l:c is designed to be used with D65 and the CIE Supplementary Observer. Commonly-used values for l:c are 2:1 for acceptability and 1:1 for the threshold of imperceptibility.
    fn cmc_delta_e(&self, other: &CIELab, k: f64, c: f64) -> f64;

    /// amin, amax, bmin, bmax: boundaries of gamut rectangle
    fn desaturate(&mut self, amin: f64, amax: f64, bmin: f64, bmax: f64) -> bool;

    /// Encodes a Lab value, from a CIELab value to ICC v4 convention.
    fn encoded(&self) -> [u16; 3];

    /// Encodes a Lab value, from a CIELab value to ICC v2 convention.
    fn encoded_v2(&self) -> [u16; 3];
}

impl CIELabExt for CIELab {
    fn cie2000_delta_e(&self, other: &CIELab, kl: f64, kc: f64, kh: f64) -> f64 {
        unsafe {
            ffi::cmsCIE2000DeltaE(self, other, kl, kc, kh)
        }
    }

    fn cie94_delta_e(&self, other: &CIELab) -> f64 {
        unsafe {
            ffi::cmsCIE94DeltaE(self, other)
        }
    }

    fn bfd_delta_e(&self, other: &CIELab) -> f64 {
        unsafe {
            ffi::cmsBFDdeltaE(self, other)
        }
    }

    fn delta_e(&self, other: &CIELab) -> f64 {
        unsafe {
            ffi::cmsDeltaE(self, other)
        }
    }

    fn cmc_delta_e(&self, other: &CIELab, k: f64, c: f64) -> f64 {
        unsafe {
            ffi::cmsCMCdeltaE(self, other, k, c)
        }
    }

    fn desaturate(&mut self, amin: f64, amax: f64, bmin: f64, bmax: f64) -> bool {
        unsafe {
            0 != ffi::cmsDesaturateLab(self, amax, amin, bmax, bmin)
        }
    }

    fn encoded(&self) -> [u16; 3] {
        let mut out = [0u16; 3];
        unsafe {
            ffi::cmsFloat2LabEncoded(out.as_mut_ptr(), self)
        }
        out
    }

    fn encoded_v2(&self) -> [u16; 3] {
        let mut out = [0u16; 3];
        unsafe {
            ffi::cmsFloat2LabEncodedV2(out.as_mut_ptr(), self)
        }
        out
    }
}

#[test]
fn temp() {
    assert!(white_point_from_temp(4000.).is_some());
}
