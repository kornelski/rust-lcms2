use crate::*;
use foreign_types::ForeignTypeRef;
use lcms2_sys::TechnologySignature;

unsafe fn cast<T>(ptr: *const u8) -> &'static T {
    assert!(0 == ptr.align_offset(align_of::<T>()), "Tag data pointer must be aligned");
    &*ptr.cast::<T>()
}

/// `from_ptr()` methods require mut for no good reason
unsafe fn aligned_mut<T>(ptr: *const u8) -> *mut T {
    assert!(0 == ptr.align_offset(align_of::<T>()), "Tag data pointer must be aligned");
    ptr as *mut T
}

impl Tag<'_> {
    #[must_use]
    pub fn is_none(&self) -> bool {
        match *self {
            Tag::None => true,
            _ => false,
        }
    }

    #[must_use]
    pub unsafe fn data_for_signature(&self, sig: TagSignature) -> *const u8 {
        use crate::TagSignature::*;
        match (sig, self) {
            (
                RedColorantTag | BlueColorantTag | GreenColorantTag | LuminanceTag | MediaBlackPointTag | MediaWhitePointTag,
                &Tag::CIEXYZ(data),
            ) => (data as *const CIEXYZ).cast::<u8>(),
            (
                ViewingCondDescTag
                | CharTargetTag
                | CopyrightTag
                | DeviceMfgDescTag
                | DeviceModelDescTag
                | ProfileDescriptionTag
                | ProfileDescriptionMLTag
                | ScreeningDescTag,
                &Tag::MLU(data),
            ) => data.as_ptr() as *const _,
            (ChromaticityTag | ChromaticAdaptationTag, &Tag::CIExyYTRIPLE(data)) => (data as *const CIExyYTRIPLE).cast::<u8>(),
            (ColorantTableTag | ColorantTableOutTag | CrdInfoTag | NamedColor2Tag, &Tag::NamedColorList(data)) => {
                data.as_ptr() as *const u8
            },
            (DataTag | Ps2CRD0Tag | Ps2CRD1Tag | Ps2CRD2Tag | Ps2CRD3Tag | Ps2CSATag, &Tag::ICCData(data)) => {
                (data as *const ffi::ICCData).cast::<u8>()
            },
            (Ps2RenderingIntentTag, &Tag::ICCData(data)) => (data as *const ffi::ICCData).cast::<u8>(),
            (
                AToB0Tag | AToB1Tag | AToB2Tag | BToA0Tag | BToA1Tag | BToA2Tag | DToB0Tag | DToB1Tag | DToB2Tag | DToB3Tag
                | BToD0Tag | BToD1Tag | BToD2Tag | BToD3Tag | GamutTag | Preview0Tag | Preview1Tag | Preview2Tag,
                &Tag::Pipeline(data),
            ) => data.as_ptr() as *const _,
            (BlueTRCTag | GrayTRCTag | GreenTRCTag | RedTRCTag, &Tag::ToneCurve(data)) => data.as_ptr() as *const _,
            (ColorimetricIntentImageStateTag, Tag::ColorimetricIntentImageState(owned)) => {
                (owned as *const ffi::ColorimetricIntentImageState).cast::<u8>()
            },
            (PerceptualRenderingIntentGamutTag | SaturationRenderingIntentGamutTag, Tag::Intent(owned)) => {
                (owned as *const Intent).cast::<u8>()
            },
            (TechnologyTag, Tag::Technology(data)) => (data as *const TechnologySignature).cast::<u8>(),
            (MeasurementTag, &Tag::ICCMeasurementConditions(data)) => (data as *const ffi::ICCMeasurementConditions).cast::<u8>(),
            (ProfileSequenceDescTag | ProfileSequenceIdTag, &Tag::SEQ(data)) => (data as *const ffi::SEQ).cast::<u8>(),
            (ScreeningTag, &Tag::Screening(data)) => (data as *const ffi::Screening).cast::<u8>(),
            (UcrBgTag, &Tag::UcrBg(data)) => (data as *const ffi::UcrBg).cast::<u8>(),
            (VcgtTag, Tag::VcgtCurves(arr)) => (arr as *const [_; 3]).cast(),
            (ViewingConditionsTag, &Tag::ICCViewingConditions(data)) => (data as *const ffi::ICCViewingConditions).cast::<u8>(),
            (CicpTag, &Tag::VideoSignal(data)) => (data as *const VideoSignalType).cast::<u8>(),
            (MHC2Tag, &Tag::MHC2(data)) => (data as *const ffi::MHC2Type).cast::<u8>(),
            (sig, _) => panic!("Signature type {sig:?} does not support this tag data type"),
        }
    }

    #[must_use] pub unsafe fn new(sig: TagSignature, data: *const u8) -> Self {
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
            PerceptualRenderingIntentGamutTag |
            SaturationRenderingIntentGamutTag => {
                Tag::Intent(data.cast::<Intent>().read_unaligned())
            },
            TechnologyTag => Tag::Technology(data.cast::<TechnologySignature>().read_unaligned()),
            MeasurementTag => Tag::ICCMeasurementConditions(cast(data)),
            ProfileSequenceDescTag |
            ProfileSequenceIdTag => Tag::SEQ(cast(data)),
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
