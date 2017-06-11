use super::*;

use std::fmt;

foreign_type! {
    type CType = ffi::Pipeline;
    fn drop = ffi::cmsPipelineFree;
    pub struct Pipeline;
    pub struct PipelineRef;
}

impl fmt::Debug for PipelineRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Pipeline")
    }
}

