use super::*;

pub trait FloatOrU16: Sized + Copy {
    unsafe fn eval_tone_curve(self, handle: *const ffi::ToneCurve) -> Self;
    unsafe fn eval_pipeline(handle: *const ffi::Pipeline, input: &[Self], out: &mut [Self]);
    unsafe fn stage_alloc_clut(contextid: ffi::Context, ngridpoints: u32, inputchan: u32, outputchan: u32, table: *const Self) -> *mut ffi::Stage;
}

impl FloatOrU16 for f32 {
    #[inline]
    unsafe fn eval_tone_curve(self, handle: *const ffi::ToneCurve) -> Self {
        ffi::cmsEvalToneCurveFloat(handle, self)
    }

    #[inline]
    unsafe fn eval_pipeline(handle: *const ffi::Pipeline, input: &[Self], out: &mut [Self]) {
        ffi::cmsPipelineEvalFloat(input.as_ptr(), out.as_mut_ptr(), handle)
    }

    #[inline]
    unsafe fn stage_alloc_clut(contextid: ffi::Context, ngridpoints: u32, inputchan: u32, outputchan: u32, table: *const Self) -> *mut ffi::Stage {
        ffi::cmsStageAllocCLutFloat(contextid, ngridpoints, inputchan, outputchan, table)
    }
}

impl FloatOrU16 for u16 {
    #[inline]
    unsafe fn eval_tone_curve(self, handle: *const ffi::ToneCurve) -> Self {
        ffi::cmsEvalToneCurve16(handle, self)
    }

    #[inline]
    unsafe fn eval_pipeline(handle: *const ffi::Pipeline, input: &[Self], out: &mut [Self]) {
        ffi::cmsPipelineEval16(input.as_ptr(), out.as_mut_ptr(), handle)
    }

    #[inline]
    unsafe fn stage_alloc_clut(contextid: ffi::Context, ngridpoints: u32, inputchan: u32, outputchan: u32, table: *const Self) -> *mut ffi::Stage {
        ffi::cmsStageAllocCLut16bit(contextid, ngridpoints, inputchan, outputchan, table)
    }
}
