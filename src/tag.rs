use super::*;
use std::mem;
use foreign_types::ForeignTypeRef;

impl<'a> Tag<'a> {
    pub fn is_none(&self) -> bool {
        match *self {
            Tag::None => true,
            _ => false,
        }
    }

    pub unsafe fn data_for_signature(&self, sig: TagSignature) -> *const u8 {
        match (sig, self) {
            (TagSignature::BlueColorantTag, &Tag::CIEXYZ(data)) => (data) as *const _ as *const u8,
            (TagSignature::GreenColorantTag, &Tag::CIEXYZ(data)) => data as *const _ as *const u8,
            (TagSignature::LuminanceTag, &Tag::CIEXYZ(data)) => data as *const _ as *const u8,
            (TagSignature::MediaBlackPointTag, &Tag::CIEXYZ(data)) => data as *const _ as *const u8,
            (TagSignature::MediaWhitePointTag, &Tag::CIEXYZ(data)) => data as *const _ as *const u8,
            (TagSignature::RedColorantTag, &Tag::CIEXYZ(data)) => data as *const _ as *const u8,
            (TagSignature::CharTargetTag, &Tag::MLU(data)) => data.as_ptr() as *const _,
            (TagSignature::CopyrightTag, &Tag::MLU(data)) => data.as_ptr() as *const _,
            (TagSignature::DeviceMfgDescTag, &Tag::MLU(data)) => data.as_ptr() as *const _,
            (TagSignature::DeviceModelDescTag, &Tag::MLU(data)) => data.as_ptr() as *const _,
            (TagSignature::ProfileDescriptionTag, &Tag::MLU(data)) => data.as_ptr() as *const _,
            (TagSignature::ScreeningDescTag, &Tag::MLU(data)) => data.as_ptr() as *const _,
            (TagSignature::ViewingCondDescTag, &Tag::MLU(data)) => data.as_ptr() as *const _,
            (TagSignature::ChromaticityTag, &Tag::CIExyYTRIPLE(data)) => data as *const _ as *const u8,
            (TagSignature::ChromaticAdaptationTag, &Tag::CIExyYTRIPLE(data)) => data as *const _ as *const u8,
            (TagSignature::ColorantTableTag, &Tag::NAMEDCOLORLIST(data)) => data as *const _ as *const u8,
            (TagSignature::ColorantTableOutTag, &Tag::NAMEDCOLORLIST(data)) => data as *const _ as *const u8,
            (TagSignature::CrdInfoTag, &Tag::NAMEDCOLORLIST(data)) => data as *const _ as *const u8,
            (TagSignature::NamedColor2Tag, &Tag::NAMEDCOLORLIST(data)) => data as *const _ as *const u8,
            (TagSignature::DataTag, &Tag::ICCData(data)) => data as *const _ as *const u8,
            (TagSignature::Ps2CRD0Tag, &Tag::ICCData(data)) => data as *const _ as *const u8,
            (TagSignature::Ps2CRD1Tag, &Tag::ICCData(data)) => data as *const _ as *const u8,
            (TagSignature::Ps2CRD2Tag, &Tag::ICCData(data)) => data as *const _ as *const u8,
            (TagSignature::Ps2CRD3Tag, &Tag::ICCData(data)) => data as *const _ as *const u8,
            (TagSignature::Ps2CSATag, &Tag::ICCData(data)) => data as *const _ as *const u8,
            (TagSignature::Ps2RenderingIntentTag, &Tag::ICCData(data)) => data as *const _ as *const u8,
            (TagSignature::AToB0Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::AToB1Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::AToB2Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::BToA0Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::BToA1Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::BToA2Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::DToB0Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::DToB1Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::DToB2Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::DToB3Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::BToD0Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::BToD1Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::BToD2Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::BToD3Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::GamutTag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::Preview0Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::Preview1Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::Preview2Tag, &Tag::Pipeline(data)) => data.as_ptr() as *const _,
            (TagSignature::BlueTRCTag, &Tag::ToneCurve(data)) => data.as_ptr() as *const _,
            (TagSignature::GrayTRCTag, &Tag::ToneCurve(data)) => data.as_ptr() as *const _,
            (TagSignature::GreenTRCTag, &Tag::ToneCurve(data)) => data.as_ptr() as *const _,
            (TagSignature::RedTRCTag, &Tag::ToneCurve(data)) => data.as_ptr() as *const _,
            (TagSignature::ColorimetricIntentImageStateTag, &Tag::Signature(data)) => data as *const _ as *const u8,
            (TagSignature::PerceptualRenderingIntentGamutTag, &Tag::Signature(data)) => data as *const _ as *const u8,
            (TagSignature::SaturationRenderingIntentGamutTag, &Tag::Signature(data)) => data as *const _ as *const u8,
            (TagSignature::TechnologyTag, &Tag::Technology(ref data)) => data as *const _ as *const u8,
            (TagSignature::MeasurementTag, &Tag::ICCMeasurementConditions(data)) => data as *const _ as *const u8,
            (TagSignature::ProfileSequenceDescTag, &Tag::SEQ(data)) => data as *const _ as *const u8,
            (TagSignature::ProfileSequenceIdTag, &Tag::SEQ(data)) => data as *const _ as *const u8,
            (TagSignature::ScreeningTag, &Tag::Screening(data)) => data as *const _ as *const u8,
            (TagSignature::UcrBgTag, &Tag::UcrBg(data)) => data as *const _ as *const u8,
            (TagSignature::ViewingConditionsTag, &Tag::ICCViewingConditions(data)) => data as *const _ as *const u8,
            (sig, _) => panic!("Signature type {:?} does not support this tag data type", sig),
        }
    }

    pub unsafe fn new(sig: TagSignature, data: *const u8) -> Self {
        if data.is_null() {
            return Tag::None;
        }
        match sig {
            TagSignature::BlueColorantTag => Tag::CIEXYZ(mem::transmute(data)),
            TagSignature::GreenColorantTag => Tag::CIEXYZ(mem::transmute(data)),
            TagSignature::LuminanceTag => Tag::CIEXYZ(mem::transmute(data)),
            TagSignature::MediaBlackPointTag => Tag::CIEXYZ(mem::transmute(data)),
            TagSignature::MediaWhitePointTag => Tag::CIEXYZ(mem::transmute(data)),
            TagSignature::RedColorantTag => Tag::CIEXYZ(mem::transmute(data)),
            TagSignature::CharTargetTag => Tag::MLU(MLURef::from_ptr(data as *mut _)),
            TagSignature::CopyrightTag => Tag::MLU(MLURef::from_ptr(data as *mut _)),
            TagSignature::DeviceMfgDescTag => Tag::MLU(MLURef::from_ptr(data as *mut _)),
            TagSignature::DeviceModelDescTag => Tag::MLU(MLURef::from_ptr(data as *mut _)),
            TagSignature::ProfileDescriptionTag => Tag::MLU(MLURef::from_ptr(data as *mut _)),
            TagSignature::ScreeningDescTag => Tag::MLU(MLURef::from_ptr(data as *mut _)),
            TagSignature::ViewingCondDescTag => Tag::MLU(MLURef::from_ptr(data as *mut _)),
            TagSignature::ChromaticityTag => Tag::CIExyYTRIPLE(mem::transmute(data)),
            TagSignature::ChromaticAdaptationTag => Tag::CIExyYTRIPLE(mem::transmute(data)),
            TagSignature::ColorantTableTag => Tag::NAMEDCOLORLIST(NamedColorListRef::from_ptr(data as *mut _)),
            TagSignature::ColorantTableOutTag => Tag::NAMEDCOLORLIST(NamedColorListRef::from_ptr(data as *mut _)),
            TagSignature::CrdInfoTag => Tag::NAMEDCOLORLIST(NamedColorListRef::from_ptr(data as *mut _)),
            TagSignature::NamedColor2Tag => Tag::NAMEDCOLORLIST(NamedColorListRef::from_ptr(data as *mut _)),
            TagSignature::DataTag => Tag::ICCData(mem::transmute(data)),
            TagSignature::Ps2CRD0Tag => Tag::ICCData(mem::transmute(data)),
            TagSignature::Ps2CRD1Tag => Tag::ICCData(mem::transmute(data)),
            TagSignature::Ps2CRD2Tag => Tag::ICCData(mem::transmute(data)),
            TagSignature::Ps2CRD3Tag => Tag::ICCData(mem::transmute(data)),
            TagSignature::Ps2CSATag => Tag::ICCData(mem::transmute(data)),
            TagSignature::Ps2RenderingIntentTag => Tag::ICCData(mem::transmute(data)),
            TagSignature::AToB0Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::AToB1Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::AToB2Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::BToA0Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::BToA1Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::BToA2Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::DToB0Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::DToB1Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::DToB2Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::DToB3Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::BToD0Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::BToD1Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::BToD2Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::BToD3Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::GamutTag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::Preview0Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::Preview1Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::Preview2Tag => Tag::Pipeline(PipelineRef::from_ptr(data as *mut _)),
            TagSignature::BlueTRCTag => Tag::ToneCurve(ToneCurveRef::from_ptr(data as *mut _)),
            TagSignature::GrayTRCTag => Tag::ToneCurve(ToneCurveRef::from_ptr(data as *mut _)),
            TagSignature::GreenTRCTag => Tag::ToneCurve(ToneCurveRef::from_ptr(data as *mut _)),
            TagSignature::RedTRCTag => Tag::ToneCurve(ToneCurveRef::from_ptr(data as *mut _)),
            TagSignature::ColorimetricIntentImageStateTag => {
                Tag::Signature(mem::transmute(data))
            }
            TagSignature::PerceptualRenderingIntentGamutTag => {
                Tag::Signature(mem::transmute(data))
            }
            TagSignature::SaturationRenderingIntentGamutTag => {
                Tag::Signature(mem::transmute(data))
            }
            TagSignature::TechnologyTag => Tag::Technology(*(data as *const ffi::TechnologySignature)),
            TagSignature::MeasurementTag => Tag::ICCMeasurementConditions(mem::transmute(data)),
            TagSignature::ProfileSequenceDescTag => Tag::SEQ(mem::transmute(data)),
            TagSignature::ProfileSequenceIdTag => Tag::SEQ(mem::transmute(data)),
            TagSignature::ScreeningTag => Tag::Screening(mem::transmute(data)),
            TagSignature::UcrBgTag => Tag::UcrBg(mem::transmute(data)),
            TagSignature::ViewingConditionsTag => {
                Tag::ICCViewingConditions(mem::transmute(data))
            }
            _ => Tag::None,
        }
    }
}
