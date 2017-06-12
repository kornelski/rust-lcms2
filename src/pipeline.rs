use super::*;
use std::fmt;
use stage::*;
use std::ptr;
use eval::FloatOrU16;
use foreign_types::ForeignTypeRef;

foreign_type! {
    #[doc(hidden)]
    type CType = ffi::Pipeline;
    fn drop = ffi::cmsPipelineFree;
    fn clone = ffi::cmsPipelineDup;
    /// This is an owned version of `PipelineRef`.
    pub struct Pipeline;
    /// Pipelines are a convenient way to model complex operations on image data.
    ///
    /// Each pipeline may contain an arbitrary number of stages. Each stage performs a single operation.
    /// Pipelines may be optimized to be executed on a certain format (8 bits, for example) and can be saved as LUTs in ICC profiles.
    pub struct PipelineRef;
}

impl Pipeline {
    /// Allocates an empty pipeline. Final Input and output channels must be specified at creation time.
    pub fn new(input_channels: usize, output_channels: usize) -> LCMSResult<Self> {
        unsafe {
            Error::if_null(ffi::cmsPipelineAlloc(ptr::null_mut(), input_channels as u32, output_channels as u32))
        }
    }
}

impl PipelineRef {
    /// Appends pipeline given as argument at the end of this pipeline. Channel count must match.
    pub fn cat(&mut self, append: &PipelineRef) -> bool {
        if append.input_channels() != self.output_channels() {
            return false;
        }
        unsafe { ffi::cmsPipelineCat(self.as_ptr(), append.as_ptr()) != 0 }
    }

    pub fn stage_count(&self) -> usize {
        unsafe { ffi::cmsPipelineStageCount(self.as_ptr()) as usize }
    }

    pub fn first_stage(&self) -> Option<&StageRef> {
        unsafe {
            let f = ffi::cmsPipelineGetPtrToFirstStage(self.as_ptr());
            if !f.is_null() {Some(ForeignTypeRef::from_ptr(f))} else {None}
        }
    }

    pub fn last_stage(&self) -> Option<&StageRef> {
        unsafe {
            let f = ffi::cmsPipelineGetPtrToLastStage(self.as_ptr());
            if !f.is_null() {Some(ForeignTypeRef::from_ptr(f))} else {None}
        }
    }

    pub fn stages(&self) -> StagesIter {
        StagesIter(self.first_stage())
    }

    pub fn set_8bit(&mut self, on: bool) -> bool {
        unsafe { ffi::cmsPipelineSetSaveAs8bitsFlag(self.as_ptr(), on as i32) != 0 }
    }

    pub fn input_channels(&self) -> usize {
        unsafe { ffi::cmsPipelineInputChannels(self.as_ptr()) as usize }
    }

    pub fn output_channels(&self) -> usize {
        unsafe { ffi::cmsPipelineOutputChannels(self.as_ptr()) as usize }
    }

    // Evaluates a pipeline usin u16 of f32 numbers. With u16 it's optionally using the optimized path.
    pub fn eval<Value: FloatOrU16>(&self, input: &[Value], output: &mut [Value]) {
        assert_eq!(self.input_channels(), input.len());
        assert_eq!(self.output_channels(), output.len());
        unsafe {
            self.eval_unchecked(input, output);
        }
    }

    // You must ensure that input and output have length sufficient for channels
    #[inline]
    pub unsafe fn eval_unchecked<Value: FloatOrU16>(&self, input: &[Value], output: &mut [Value]) {
        Value::eval_pipeline(self.as_ptr(), input, output);
    }
}

impl fmt::Debug for PipelineRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pipeline({}->{}ch, {} stages)", self.input_channels(), self.output_channels(), self.stage_count())
    }
}

#[test]
fn pipeline() {
    let p = Pipeline::new(123, 12);
    assert!(p.is_err());

    let p = Pipeline::new(4, 3).unwrap();
    assert_eq!(0, p.stage_count());
    assert_eq!(4, p.input_channels());
    assert_eq!(3, p.output_channels());
}
