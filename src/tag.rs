use super::*;

extern crate lcms2_sys as ffi;

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
            TagSignature::SigBlueColorantTag => Tag::CIEXYZ(mem::transmute(data)),
            TagSignature::SigGreenColorantTag => Tag::CIEXYZ(mem::transmute(data)),
            TagSignature::SigLuminanceTag => Tag::CIEXYZ(mem::transmute(data)),
            TagSignature::SigMediaBlackPointTag => Tag::CIEXYZ(mem::transmute(data)),
            TagSignature::SigMediaWhitePointTag => Tag::CIEXYZ(mem::transmute(data)),
            TagSignature::SigRedColorantTag => Tag::CIEXYZ(mem::transmute(data)),
            TagSignature::SigCharTargetTag => Tag::MLU(mem::transmute(data)),
            TagSignature::SigCopyrightTag => Tag::MLU(mem::transmute(data)),
            TagSignature::SigDeviceMfgDescTag => Tag::MLU(mem::transmute(data)),
            TagSignature::SigDeviceModelDescTag => Tag::MLU(mem::transmute(data)),
            TagSignature::SigProfileDescriptionTag => Tag::MLU(mem::transmute(data)),
            TagSignature::SigScreeningDescTag => Tag::MLU(mem::transmute(data)),
            TagSignature::SigViewingCondDescTag => Tag::MLU(mem::transmute(data)),
            TagSignature::SigChromaticityTag => Tag::CIExyYTRIPLE(mem::transmute(data)),
            TagSignature::SigColorantTableTag => Tag::NAMEDCOLORLIST(mem::transmute(data)),
            TagSignature::SigColorantTableOutTag => Tag::NAMEDCOLORLIST(mem::transmute(data)),
            TagSignature::SigCrdInfoTag => Tag::NAMEDCOLORLIST(mem::transmute(data)),
            TagSignature::SigNamedColor2Tag => Tag::NAMEDCOLORLIST(mem::transmute(data)),
            TagSignature::SigDataTag => Tag::ICCData(mem::transmute(data)),
            TagSignature::SigPs2CRD0Tag => Tag::ICCData(mem::transmute(data)),
            TagSignature::SigPs2CRD1Tag => Tag::ICCData(mem::transmute(data)),
            TagSignature::SigPs2CRD2Tag => Tag::ICCData(mem::transmute(data)),
            TagSignature::SigPs2CRD3Tag => Tag::ICCData(mem::transmute(data)),
            TagSignature::SigPs2CSATag => Tag::ICCData(mem::transmute(data)),
            TagSignature::SigPs2RenderingIntentTag => Tag::ICCData(mem::transmute(data)),
            TagSignature::SigAToB0Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigAToB1Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigAToB2Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigBToA0Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigBToA1Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigBToA2Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigDToB0Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigDToB1Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigDToB2Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigDToB3Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigBToD0Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigBToD1Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigBToD2Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigBToD3Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigGamutTag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigPreview0Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigPreview1Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigPreview2Tag => Tag::Pipeline(mem::transmute(data)),
            TagSignature::SigBlueTRCTag => Tag::ToneCurve(mem::transmute(data)),
            TagSignature::SigGrayTRCTag => Tag::ToneCurve(mem::transmute(data)),
            TagSignature::SigGreenTRCTag => Tag::ToneCurve(mem::transmute(data)),
            TagSignature::SigRedTRCTag => Tag::ToneCurve(mem::transmute(data)),
            TagSignature::SigColorimetricIntentImageStateTag => {
                Tag::Signature(mem::transmute(data))
            }
            TagSignature::SigPerceptualRenderingIntentGamutTag => {
                Tag::Signature(mem::transmute(data))
            }
            TagSignature::SigSaturationRenderingIntentGamutTag => {
                Tag::Signature(mem::transmute(data))
            }
            TagSignature::SigTechnologyTag => Tag::Signature(mem::transmute(data)),
            TagSignature::SigMeasurementTag => Tag::ICCMeasurementConditions(mem::transmute(data)),
            TagSignature::SigProfileSequenceDescTag => Tag::SEQ(mem::transmute(data)),
            TagSignature::SigProfileSequenceIdTag => Tag::SEQ(mem::transmute(data)),
            TagSignature::SigScreeningTag => Tag::Screening(mem::transmute(data)),
            TagSignature::SigUcrBgTag => Tag::UcrBg(mem::transmute(data)),
            TagSignature::SigViewingConditionsTag => {
                Tag::ICCViewingConditions(mem::transmute(data))
            }
            _ => Tag::None,
        }
    }
}
