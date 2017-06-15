use super::*;
use eval::FloatOrU16;
use std::fmt;
use std::ptr;
use foreign_types::ForeignTypeRef;
use context::Context;

foreign_type! {
    type CType = ffi::Stage;
    fn drop = ffi::cmsStageFree;
    /// This is an owned version of `Stage`.
    pub struct Stage;
    /// Stage functions
    ///
    /// Stages are single-step operations that can be chained to create pipelines.
    /// Actual stage types does include matrices, tone curves, Look-up interpolation and user-defined.
    /// There are functions to create new stage types and a plug-in type to allow stages to be saved in multi profile elements tag types.
    /// See the plug-in API for further details.
    pub struct StageRef;
}

impl Stage {
    /// Creates an empty (identity) stage that does no operation.
    ///
    /// May be needed in order to save the pipeline as AToB/BToA tags in ICC profiles.
    pub fn new_identity(channels: u32) -> Stage {
        unsafe {Error::if_null(
            ffi::cmsStageAllocIdentity(GlobalContext::new().as_ptr(), channels)
        )}.unwrap()
    }

    /// Creates a stage that contains nChannels tone curves, one per channel.
    pub fn new_tone_curves(curves: &[&ToneCurveRef]) -> LCMSResult<Stage> {
        let ptrs: Vec<_> = curves.iter().map(|c| c.as_ptr() as *const _).collect();
        unsafe {Error::if_null(
            ffi::cmsStageAllocToneCurves(GlobalContext::new().as_ptr(), ptrs.len() as u32, ptrs.as_ptr())
        )}
    }

    /// Creates a stage that contains a matrix plus an optional offset.
    ///
    /// Note that Matrix is specified in double precision, whilst CLUT has only float precision.
    /// That is because an ICC profile can encode matrices with far more precision that CLUTS.
    pub fn new_matrix(matrix2d: &[f64], rows: usize, cols: usize, offsets: Option<&[f64]>) -> LCMSResult<Self> {
        if matrix2d.len() < rows * cols {
            return Err(Error::MissingData);
        }
        if let Some(offsets) = offsets {
            if offsets.len() < cols {
                return Err(Error::MissingData);
            }
        }
        unsafe {Error::if_null(
            ffi::cmsStageAllocMatrix(GlobalContext::new().as_ptr(), rows as u32, cols as u32, matrix2d.as_ptr(),
                offsets.map(|p|p.as_ptr()).unwrap_or(ptr::null()))
        )}
    }

    /// Creates a stage that contains a float or 16 bits multidimensional lookup table (CLUT).
    ///
    /// Each dimension has same resolution. The CLUT can be initialized by specifying values in Table parameter.
    /// The recommended way is to set Table to None and use sample_clut with a callback, because this way the implementation is independent of the selected number of grid points.
    pub fn new_clut<Value: FloatOrU16>(grid_point_nodes: usize, input_channels: u32, output_channels: u32, table: Option<&[Value]>) -> LCMSResult<Self> {
        if let Some(table) = table {
            if table.len() < grid_point_nodes {
                return Err(Error::MissingData)
            }
        }
        unsafe {Error::if_null(
            Value::stage_alloc_clut(GlobalContext::new().as_ptr(), grid_point_nodes as u32, input_channels, output_channels,
                table.map(|p|p.as_ptr()).unwrap_or(ptr::null()))
        )}
    }
}

impl StageRef {
    pub fn input_channels(&self) -> usize {
        unsafe { ffi::cmsStageInputChannels(self.as_ptr()) as usize }
    }

    pub fn output_channels(&self) -> usize {
        unsafe { ffi::cmsStageOutputChannels(self.as_ptr()) as usize }
    }

    pub fn stage_type(&self) -> ffi::StageSignature {
        unsafe { ffi::cmsStageType(self.as_ptr()) }
    }
}

pub struct StagesIter<'a>(pub Option<&'a StageRef>);

impl<'a> Iterator for StagesIter<'a> {
    type Item = &'a StageRef;
    fn next(&mut self) -> Option<Self::Item> {
        let it = self.0;
        if let Some(mpe) = it {
            self.0 = unsafe {
                let s = ffi::cmsStageNext(mpe.as_ptr());
                if s.is_null() {
                    None
                } else {
                    Some(ForeignTypeRef::from_ptr(s))
                }
            };
        }
        it
    }
}

impl fmt::Debug for StageRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stage({:?})", self.stage_type())
    }
}
