use super::*;

use std::mem;

impl<'a> Tag<'a> {
    pub fn is_none(&self) -> bool {
        match *self {
            Tag::None => true,
            _ => false,
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
            TagSignature::CharTargetTag => Tag::MLU(mem::transmute(data)),
            TagSignature::CopyrightTag => Tag::MLU(mem::transmute(data)),
            TagSignature::DeviceMfgDescTag => Tag::MLU(mem::transmute(data)),
            TagSignature::DeviceModelDescTag => Tag::MLU(mem::transmute(data)),
            TagSignature::ProfileDescriptionTag => Tag::MLU(mem::transmute(data)),
            TagSignature::ScreeningDescTag => Tag::MLU(mem::transmute(data)),
            TagSignature::ViewingCondDescTag => Tag::MLU(mem::transmute(data)),
            TagSignature::ChromaticityTag => Tag::CIExyYTRIPLE(mem::transmute(data)),
            TagSignature::ColorantTableTag => Tag::NAMEDCOLORLIST(mem::transmute(data)),
            TagSignature::ColorantTableOutTag => Tag::NAMEDCOLORLIST(mem::transmute(data)),
            TagSignature::CrdInfoTag => Tag::NAMEDCOLORLIST(mem::transmute(data)),
            TagSignature::NamedColor2Tag => Tag::NAMEDCOLORLIST(mem::transmute(data)),
            TagSignature::DataTag => Tag::ICCData(mem::transmute(data)),
            TagSignature::Ps2CRD0Tag => Tag::ICCData(mem::transmute(data)),
            TagSignature::Ps2CRD1Tag => Tag::ICCData(mem::transmute(data)),
            TagSignature::Ps2CRD2Tag => Tag::ICCData(mem::transmute(data)),
            TagSignature::Ps2CRD3Tag => Tag::ICCData(mem::transmute(data)),
            TagSignature::Ps2CSATag => Tag::ICCData(mem::transmute(data)),
            TagSignature::Ps2RenderingIntentTag => Tag::ICCData(mem::transmute(data)),
            TagSignature::AToB0Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::AToB1Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::AToB2Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::BToA0Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::BToA1Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::BToA2Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::DToB0Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::DToB1Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::DToB2Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::DToB3Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::BToD0Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::BToD1Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::BToD2Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::BToD3Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::GamutTag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::Preview0Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::Preview1Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::Preview2Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::BlueTRCTag => Tag::ToneCurve(mem::transmute(data)),
            TagSignature::GrayTRCTag => Tag::ToneCurve(mem::transmute(data)),
            TagSignature::GreenTRCTag => Tag::ToneCurve(mem::transmute(data)),
            TagSignature::RedTRCTag => Tag::ToneCurve(mem::transmute(data)),
            TagSignature::ColorimetricIntentImageStateTag => {
                Tag::Signature(mem::transmute(data))
            }
            TagSignature::PerceptualRenderingIntentGamutTag => {
                Tag::Signature(mem::transmute(data))
            }
            TagSignature::SaturationRenderingIntentGamutTag => {
                Tag::Signature(mem::transmute(data))
            }
            TagSignature::TechnologyTag => Tag::Signature(mem::transmute(data)),
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
