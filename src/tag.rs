use super::*;
use foreign_types::ForeignTypeRef;

unsafe fn cast<T>(ptr: *const u8) -> &'static T {
    assert!(0 == ptr.align_offset(std::mem::align_of::<T>()), "Tag data pointer must be aligned");
    &*(ptr as *const T)
}

/// from_ptr() methods require mut for no good reason
unsafe fn aligned_mut<T>(ptr: *const u8) -> *mut T {
    assert!(0 == ptr.align_offset(std::mem::align_of::<T>()), "Tag data pointer must be aligned");
    ptr as *mut T
}

impl<'a> Tag<'a> {
    pub fn is_none(&self) -> bool {
        match *self {
            Tag::None => true,
            _ => false,
        }
    }

    pub unsafe fn data_for_signature(&self, sig: TagSignature) -> *const u8 {
        use crate::TagSignature::*;
        match (sig, self) {
            (RedColorantTag, &Tag::CIEXYZ(data)) |
            (BlueColorantTag, &Tag::CIEXYZ(data)) |
            (GreenColorantTag, &Tag::CIEXYZ(data)) |
            (LuminanceTag, &Tag::CIEXYZ(data)) |
            (MediaBlackPointTag, &Tag::CIEXYZ(data)) |
            (MediaWhitePointTag, &Tag::CIEXYZ(data)) => {
                data as *const _ as *const u8
            },
            (ViewingCondDescTag, &Tag::MLU(data)) |
            (CharTargetTag, &Tag::MLU(data)) |
            (CopyrightTag, &Tag::MLU(data)) |
            (DeviceMfgDescTag, &Tag::MLU(data)) |
            (DeviceModelDescTag, &Tag::MLU(data)) |
            (ProfileDescriptionTag, &Tag::MLU(data)) |
            (ProfileDescriptionMLTag, &Tag::MLU(data)) |
            (ScreeningDescTag, &Tag::MLU(data)) => {
                data.as_ptr() as *const _
            },
            (ChromaticityTag, &Tag::CIExyYTRIPLE(data)) |
            (ChromaticAdaptationTag, &Tag::CIExyYTRIPLE(data)) => {
                data as *const _ as *const u8
            },
            (ColorantTableTag, &Tag::NAMEDCOLORLIST(data)) |
            (ColorantTableOutTag, &Tag::NAMEDCOLORLIST(data)) |
            (CrdInfoTag, &Tag::NAMEDCOLORLIST(data)) |
            (NamedColor2Tag, &Tag::NAMEDCOLORLIST(data)) => data as *const _ as *const u8,
            (DataTag, &Tag::ICCData(data)) |
            (Ps2CRD0Tag, &Tag::ICCData(data)) |
            (Ps2CRD1Tag, &Tag::ICCData(data)) |
            (Ps2CRD2Tag, &Tag::ICCData(data)) |
            (Ps2CRD3Tag, &Tag::ICCData(data)) |
            (Ps2CSATag, &Tag::ICCData(data)) => data as *const _ as *const u8,
            (Ps2RenderingIntentTag, &Tag::ICCData(data)) => data as *const _ as *const u8,
            (AToB0Tag, &Tag::Pipeline(data)) |
            (AToB1Tag, &Tag::Pipeline(data)) |
            (AToB2Tag, &Tag::Pipeline(data)) |
            (BToA0Tag, &Tag::Pipeline(data)) |
            (BToA1Tag, &Tag::Pipeline(data)) |
            (BToA2Tag, &Tag::Pipeline(data)) |
            (DToB0Tag, &Tag::Pipeline(data)) |
            (DToB1Tag, &Tag::Pipeline(data)) |
            (DToB2Tag, &Tag::Pipeline(data)) |
            (DToB3Tag, &Tag::Pipeline(data)) |
            (BToD0Tag, &Tag::Pipeline(data)) |
            (BToD1Tag, &Tag::Pipeline(data)) |
            (BToD2Tag, &Tag::Pipeline(data)) |
            (BToD3Tag, &Tag::Pipeline(data)) |
            (GamutTag, &Tag::Pipeline(data)) |
            (Preview0Tag, &Tag::Pipeline(data)) |
            (Preview1Tag, &Tag::Pipeline(data)) |
            (Preview2Tag, &Tag::Pipeline(data)) => {
                data.as_ptr() as *const _
            },
            (BlueTRCTag, &Tag::ToneCurve(data)) |
            (GrayTRCTag, &Tag::ToneCurve(data)) |
            (GreenTRCTag, &Tag::ToneCurve(data)) |
            (RedTRCTag, &Tag::ToneCurve(data)) => {
                data.as_ptr() as *const _
            },
            (ColorimetricIntentImageStateTag, &Tag::ColorimetricIntentImageState(ref data)) => {
                data as *const ffi::ColorimetricIntentImageState as *const u8
            },
            (PerceptualRenderingIntentGamutTag, &Tag::Intent(ref data)) |
            (SaturationRenderingIntentGamutTag, &Tag::Intent(ref data)) => {
                data as *const ffi::Intent as *const u8
            },
            (TechnologyTag, &Tag::Technology(ref data)) => data as *const _ as *const u8,
            (MeasurementTag, &Tag::ICCMeasurementConditions(data)) => {
                data as *const _ as *const u8
            },
            (ProfileSequenceDescTag, &Tag::SEQ(data)) |
            (ProfileSequenceIdTag, &Tag::SEQ(data)) => {
                data as *const _ as *const u8
            },
            (ScreeningTag, &Tag::Screening(data)) => data as *const _ as *const u8,
            (UcrBgTag, &Tag::UcrBg(data)) => data as *const _ as *const u8,
            (VcgtTag, &Tag::VcgtCurves(arr)) => arr[0].as_ptr() as *const _,
            (ViewingConditionsTag, &Tag::ICCViewingConditions(data)) => {
                data as *const _ as *const u8
            },
            (sig, _) => panic!("Signature type {:?} does not support this tag data type", sig),
        }
    }

    pub unsafe fn new(sig: TagSignature, data: *const u8) -> Self {
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
            NamedColor2Tag => Tag::NAMEDCOLORLIST(NamedColorListRef::from_ptr(aligned_mut(data))),
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
                Tag::ColorimetricIntentImageState((data as *const ffi::ColorimetricIntentImageState).read_unaligned())
            },
            PerceptualRenderingIntentGamutTag |
            SaturationRenderingIntentGamutTag => {
                Tag::Intent((data as *const ffi::Intent).read_unaligned())
            },
            TechnologyTag => Tag::Technology((data as *const ffi::TechnologySignature).read_unaligned()),
            MeasurementTag => Tag::ICCMeasurementConditions(cast(data)),
            ProfileSequenceDescTag |
            ProfileSequenceIdTag => Tag::SEQ(cast(data)),
            ScreeningTag => Tag::Screening(cast(data)),
            UcrBgTag => Tag::UcrBg(cast(data)),
            VcgtTag => Tag::VcgtCurves([
                ToneCurveRef::from_ptr(aligned_mut(data)),
                ToneCurveRef::from_ptr(*(aligned_mut::<*mut ffi::ToneCurve>(data).offset(1))),
                ToneCurveRef::from_ptr(*(aligned_mut::<*mut ffi::ToneCurve>(data).offset(2))),
            ]),
            ViewingConditionsTag => Tag::ICCViewingConditions(cast(data)),
            _ => Tag::None,
        }
    }
}
