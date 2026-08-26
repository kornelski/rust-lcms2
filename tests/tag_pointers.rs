use lcms2::*;

#[test]
fn technology_tag_roundtrip() {
    let mut p = Profile::new_srgb();
    assert!(p.write_tag(
        TagSignature::TechnologyTag,
        Tag::Technology(lcms2_sys::ffi::TechnologySignature::DigitalCamera)
    ));
    let icc = p.icc().unwrap();
    let offset = find_tag(&icc, TagSignature::TechnologyTag);
    assert_eq!(&icc[offset..offset + 4], b"sig ", "type signature");
    assert_eq!(&icc[offset + 8..offset + 12], b"dcam", "technology value");
    match p.read_tag(TagSignature::TechnologyTag) {
        Tag::Technology(t) => assert_eq!(t, lcms2_sys::ffi::TechnologySignature::DigitalCamera),
        t => panic!("expected Tag::Technology, got {t:?}"),
    }
}

#[test]
fn colorimetric_intent_image_state_roundtrip() {
    let mut p = Profile::new_srgb();
    assert!(p.write_tag(
        TagSignature::ColorimetricIntentImageStateTag,
        Tag::ColorimetricIntentImageState(lcms2_sys::ffi::ColorimetricIntentImageState::SceneColorimetryEstimates)
    ));
    let icc = p.icc().unwrap();
    let offset = find_tag(&icc, TagSignature::ColorimetricIntentImageStateTag);
    assert_eq!(&icc[offset..offset + 4], b"sig ", "type signature");
    assert_eq!(&icc[offset + 8..offset + 12], b"scoe", "state value");
    match p.read_tag(TagSignature::ColorimetricIntentImageStateTag) {
        Tag::ColorimetricIntentImageState(s) => {
            assert_eq!(s, lcms2_sys::ffi::ColorimetricIntentImageState::SceneColorimetryEstimates)
        },
        t => panic!("expected Tag::ColorimetricIntentImageState, got {t:?}"),
    }
}

#[test]
fn perceptual_intent_gamut_roundtrip() {
    let mut p = Profile::new_srgb();
    assert!(p.write_tag(
        TagSignature::PerceptualRenderingIntentGamutTag,
        Tag::Intent(Intent::Saturation)
    ));
    let icc = p.icc().unwrap();
    let offset = find_tag(&icc, TagSignature::PerceptualRenderingIntentGamutTag);
    assert_eq!(&icc[offset..offset + 4], b"sig ", "type signature");
    assert_eq!(&icc[offset + 8..offset + 12], 2u32.to_be_bytes(), "intent value");
    match p.read_tag(TagSignature::PerceptualRenderingIntentGamutTag) {
        Tag::Intent(i) => assert_eq!(i, Intent::Saturation),
        t => panic!("expected Tag::Intent, got {t:?}"),
    }
}

#[test]
fn ps2_crd0_tag_roundtrip() {
    // `cmsSigDataTag` itself has no tag descriptor in lcms2 ("Ancient, unused",
    // cmstypes.c:6136) and can't be written. Ps2CRD0 uses the same
    // `cmsSigDataType` handler (Type_Data_Dup/Write) and is writable.
    let mut p = Profile::new_srgb();
    let mut data = lcms2_sys::ffi::ICCData::default();
    // `data` is a 1-byte flexible-array placeholder: a stack `ICCData` can only
    // safely hold 1 payload byte (lcms copies sizeof(cmsICCData)+len-1 bytes).
    data.len = 1;
    data.data[0] = b'h';
    assert!(p.write_tag(TagSignature::Ps2CRD0Tag, Tag::ICCData(&data)));
    let icc = p.icc().unwrap();
    // The serialized Data-type layout (`[sig][flag][payload]`, len implicit in
    // the tag size) is an lcms internal detail; the crate contract is that the
    // pointer passed to lcms is the `cmsICCData` struct base, so a write/read
    // round-trip must preserve len/flag/payload.
    let offset = find_tag(&icc, TagSignature::Ps2CRD0Tag);
    assert_eq!(&icc[offset..offset + 4], b"data", "type signature");
    match p.read_tag(TagSignature::Ps2CRD0Tag) {
        Tag::ICCData(d) => {
            assert_eq!(d.len, 1);
            assert_eq!(d.flag, 0);
            assert_eq!(d.data[0], b'h');
        },
        t => panic!("expected Tag::ICCData, got {t:?}"),
    }
}

#[test]
fn ps2_rendering_intent_roundtrip() {
    let mut p = Profile::new_srgb();
    let mut data = lcms2_sys::ffi::ICCData::default();
    data.len = 1;
    data.data[0] = b'0';
    assert!(p.write_tag(TagSignature::Ps2RenderingIntentTag, Tag::ICCData(&data)));
    let icc = p.icc().unwrap();
    let offset = find_tag(&icc, TagSignature::Ps2RenderingIntentTag);
    assert_eq!(&icc[offset..offset + 4], b"data", "type signature");
    match p.read_tag(TagSignature::Ps2RenderingIntentTag) {
        Tag::ICCData(d) => {
            assert_eq!(d.len, 1);
            assert_eq!(d.flag, 0);
            assert_eq!(d.data[0], b'0');
        },
        t => panic!("expected Tag::ICCData, got {t:?}"),
    }
}

#[test]
fn vcgt_tag_roundtrip() {
    let mut p = Profile::new_srgb();
    let tc = ToneCurve::new(2.2);
    let refs: [&ToneCurveRef; 3] = [&tc, &tc, &tc];
    assert!(p.write_tag(TagSignature::VcgtTag, Tag::VcgtCurves(refs)));
    match p.read_tag(TagSignature::VcgtTag) {
        Tag::VcgtCurves(gs) => {
            for g in gs {
                assert_eq!(g.estimated_entries(), tc.estimated_entries());
            }
        },
        t => panic!("expected Tag::VcgtCurves, got {t:?}"),
    }
}

/// Finds a tag entry in a serialized ICC profile and returns the offset of its
/// 4-byte type signature. Tag table entries are 12 bytes each (`TagBase`).
fn find_tag(icc: &[u8], sig: TagSignature) -> usize {
    let sig = (sig as u32).to_be_bytes();
    let count = u32::from_be_bytes(icc[128..132].try_into().unwrap()) as usize;
    let table = &icc[132..132 + count * 12];
    for i in 0..count {
        let entry = &table[i * 12..(i + 1) * 12];
        if entry[0..4] == sig {
            return u32::from_be_bytes(entry[4..8].try_into().unwrap()) as usize;
        }
    }
    panic!("tag {sig:?} not found in serialized profile");
}
