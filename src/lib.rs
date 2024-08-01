pub mod vst;

#[macro_use]
extern crate num_derive;

use num_derive::FromPrimitive;

#[cfg(not(target_os = "windows"))]
#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Eq, PartialEq)]
pub enum TResult {
    KNoInterface = -1,
    /// This value also used for KResultTrue
    KResultOk = 0,
    KResultFalse = 1,
    KInvalidArgument = 2,
    KNotImplemented = 3,
    KInternalError = 4,
    KNotInititalized = 5,
    KOutOfMemory = 6,
}

impl TResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::KResultOk)
    }
}

#[cfg(target_os = "windows")]
#[derive(FromPrimitive)]
pub enum KResult {
    KNoInterface = -2_147_467_262,
    /// This value also used for KResultTrue
    KResultOk = 0,
    KResultFalse = 1,
    KInvalidArgument = -2_147_467_809,
    KNotImplemented = -2_147_467_263,
    KInternalError = -2_147_467_259,
    KNotInititalized = -2_147_418_113,
    KOutOfMemory = -2_147_024_882,
}
