use crate::*;
use std::ptr;
use foreign_types::ForeignTypeRef;

unsafe fn cast<T>(ptr: *const u8) -> &'static T {
    unsafe {
        assert!(0 == ptr.align_offset(align_of::<T>()), "Tag data pointer must be aligned");
        &*ptr.cast::<T>()
    }
}

fn byte_ptr<T>(ptr: &T) -> *const u8 {
    ptr::from_ref(ptr).cast()
}

/// `from_ptr()` methods require mut for no good reason
unsafe fn aligned_mut<T>(ptr: *const u8) -> *mut T {
    assert!(0 == ptr.align_offset(align_of::<T>()), "Tag data pointer must be aligned");
    ptr.cast_mut().cast::<T>()
}

impl Tag<'_> {
    #[must_use]
    pub fn is_none(&self) -> bool {
        matches!(*self, Tag::None)
    }

    #[must_use]
    pub unsafe fn data_for_signature(&self, sig: TagSignature) -> *const u8 {
        unsafe {
            self.data_for_signature_opt(sig)
                .unwrap_or_else(|| panic!("Signature type {sig:?} does not support this tag data type"))
        }
    }

    #[must_use]
    pub unsafe fn data_for_signature_opt(&self, sig: TagSignature) -> Option<*const u8> {
        use crate::TagSignature::*;
        Some(match (sig, self) {
            (
                RedColorantTag | BlueColorantTag | GreenColorantTag | LuminanceTag | MediaBlackPointTag | MediaWhitePointTag,
                &Tag::CIEXYZ(data),
            ) => byte_ptr::<ffi::CIEXYZ>(data),
            (
                ViewingCondDescTag | CharTargetTag | CopyrightTag | DeviceMfgDescTag | DeviceModelDescTag | ProfileDescriptionTag | ProfileDescriptionMLTag | ScreeningDescTag,
                &Tag::MLU(data),
            ) => data.as_ptr().cast(),
            (ChromaticityTag | ChromaticAdaptationTag, &Tag::CIExyYTRIPLE(data)) => byte_ptr::<ffi::CIExyYTRIPLE>(data),
            (ColorantTableTag | ColorantTableOutTag | CrdInfoTag | NamedColor2Tag, &Tag::NamedColorList(data)) => {
                data.as_ptr().cast()
            },
            (DataTag | Ps2CRD0Tag | Ps2CRD1Tag | Ps2CRD2Tag | Ps2CRD3Tag | Ps2CSATag, &Tag::ICCData(data)) => {
                ptr::from_ref::<ffi::ICCData>(data).cast()
            },
            (Ps2RenderingIntentTag, &Tag::ICCData(data)) => ptr::from_ref::<ffi::ICCData>(data).cast(),
            (
                AToB0Tag | AToB1Tag | AToB2Tag | BToA0Tag | BToA1Tag | BToA2Tag | DToB0Tag | DToB1Tag | DToB2Tag | DToB3Tag
                | BToD0Tag | BToD1Tag | BToD2Tag | BToD3Tag | GamutTag | Preview0Tag | Preview1Tag | Preview2Tag,
                &Tag::Pipeline(data),
            ) => data.as_ptr().cast(),
            (BlueTRCTag | GrayTRCTag | GreenTRCTag | RedTRCTag, &Tag::ToneCurve(data)) => data.as_ptr().cast(),
            (ColorimetricIntentImageStateTag, Tag::ColorimetricIntentImageState(owned)) => byte_ptr::<ffi::ColorimetricIntentImageState>(owned),
            (PerceptualRenderingIntentGamutTag | SaturationRenderingIntentGamutTag, Tag::Intent(owned)) => byte_ptr::<ffi::Intent>(owned),
            (TechnologyTag, Tag::Technology(data)) => byte_ptr::<ffi::TechnologySignature>(data),
            (MeasurementTag, &Tag::ICCMeasurementConditions(data)) => byte_ptr::<ffi::ICCMeasurementConditions>(data),
            (ProfileSequenceDescTag | ProfileSequenceIdTag, &Tag::SEQ(data)) => byte_ptr::<ffi::SEQ>(data),
            (ScreeningTag, &Tag::Screening(data)) => byte_ptr::<ffi::Screening>(data),
            (UcrBgTag, &Tag::UcrBg(data)) => byte_ptr::<ffi::UcrBg>(data),
            (VcgtTag, Tag::VcgtCurves(arr)) => ptr::from_ref::<[_; 3]>(arr).cast(),
            (ViewingConditionsTag, &Tag::ICCViewingConditions(data)) => byte_ptr::<ffi::ICCViewingConditions>(data),
            (CicpTag, &Tag::VideoSignal(data)) => byte_ptr::<ffi::VideoSignalType>(data),
            (MHC2Tag, &Tag::MHC2(data)) => byte_ptr::<ffi::MHC2Type>(data),
            (_sig, _) => return None,
        })
    }

    #[must_use]
    pub unsafe fn new(sig: TagSignature, data: *const u8) -> Self {
        unsafe {
            if data.is_null() {
                return Tag::None;
            }
            use crate::TagSignature::*;
            match sig {
                BlueColorantTag |
                GreenColorantTag |
                LuminanceTag |
                MediaBlackPointTag |
                MediaWhitePointTag |
                RedColorantTag => Tag::CIEXYZ(cast(data)),
                CharTargetTag |
                CopyrightTag |
                DeviceMfgDescTag |
                DeviceModelDescTag |
                ProfileDescriptionTag |
                ProfileDescriptionMLTag |
                ScreeningDescTag |
                ViewingCondDescTag => Tag::MLU(MLURef::from_ptr(aligned_mut(data))),
                ChromaticityTag |
                ChromaticAdaptationTag => Tag::CIExyYTRIPLE(cast(data)),
                ColorantTableTag |
                ColorantTableOutTag |
                CrdInfoTag |
                NamedColor2Tag => Tag::NamedColorList(NamedColorListRef::from_ptr(aligned_mut(data))),
                DataTag |
                Ps2CRD0Tag |
                Ps2CRD1Tag |
                Ps2CRD2Tag |
                Ps2CRD3Tag |
                Ps2CSATag |
                Ps2RenderingIntentTag => Tag::ICCData(cast(data)),
                AToB0Tag |
                AToB1Tag |
                AToB2Tag |
                BToA0Tag |
                BToA1Tag |
                BToA2Tag |
                DToB0Tag |
                DToB1Tag |
                DToB2Tag |
                DToB3Tag |
                BToD0Tag |
                BToD1Tag |
                BToD2Tag |
                BToD3Tag |
                GamutTag |
                Preview0Tag |
                Preview1Tag |
                Preview2Tag => Tag::Pipeline(PipelineRef::from_ptr(aligned_mut(data))),
                BlueTRCTag |
                GrayTRCTag |
                GreenTRCTag |
                RedTRCTag => Tag::ToneCurve(ToneCurveRef::from_ptr(aligned_mut(data))),
                ColorimetricIntentImageStateTag  => {
                    Tag::ColorimetricIntentImageState(data.cast::<ffi::ColorimetricIntentImageState>().read_unaligned())
                },
                PerceptualRenderingIntentGamutTag | SaturationRenderingIntentGamutTag => {
                    Tag::Intent(data.cast::<Intent>().read_unaligned())
                },
                TechnologyTag => Tag::Technology(data.cast::<ffi::TechnologySignature>().read_unaligned()),
                MeasurementTag => Tag::ICCMeasurementConditions(cast(data)),
                ProfileSequenceDescTag | ProfileSequenceIdTag => Tag::SEQ(cast(data)),
                ScreeningTag => Tag::Screening(cast(data)),
                UcrBgTag => Tag::UcrBg(cast(data)),
                VcgtTag => {
                    let ptrs = data.cast::<*mut u8>(); // array of pointers
                    Tag::VcgtCurves([
                        ToneCurveRef::from_ptr(aligned_mut(*ptrs)),
                        ToneCurveRef::from_ptr(aligned_mut(*ptrs.add(1))),
                        ToneCurveRef::from_ptr(aligned_mut(*ptrs.add(2))),
                    ])
                },
                ViewingConditionsTag => Tag::ICCViewingConditions(cast(data)),
                CicpTag => Tag::VideoSignal(cast(data)),
                MHC2Tag => Tag::MHC2(cast(data)),
                _ => Tag::None,
            }
        }
    }
}

#[test]
fn tone_curves_tag() {
    let mut icc = Profile::new_srgb();
    let _ = icc.read_tag(TagSignature::VcgtTag);

    icc.remove_tag(TagSignature::ProfileDescriptionTag);

    let mut desc = MLU::new(1);
    desc.set_text("Test ICC", Locale::none());
    icc.write_tag(TagSignature::ProfileDescriptionTag, Tag::MLU(&desc));

    let tc = ToneCurve::new(2.0);
    let tc_refs: [&ToneCurveRef; 3] = [&tc, &tc, &tc];
    let before = Tag::VcgtCurves(tc_refs);
    icc.write_tag(TagSignature::VcgtTag, before);
    let before = Tag::VcgtCurves(tc_refs);
    let after = icc.read_tag(TagSignature::VcgtTag);

    match (before, after) {
        (Tag::VcgtCurves(a), Tag::VcgtCurves(b)) => {
            assert_eq!(a[0].estimated_entries(), b[0].estimated_entries());
            assert_eq!(a[1].estimated_entries(), b[1].estimated_entries());
            assert_eq!(a[2].estimated_entries(), b[2].estimated_entries());
        },
        _ => panic!("bad tags"),
    }
    icc.icc().unwrap();
}
