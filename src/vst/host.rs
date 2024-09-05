use num_traits::ToPrimitive;
use vst3::{
    Interface,
    Steinberg::{char16, int32, int64, kNotImplemented, tresult, uint32, FIDString, Vst::*, TUID},
};
use IAttributeList_::AttrID;

use crate::TResult;

pub struct PluginHost {}

impl IHostApplicationTrait for PluginHost {
    unsafe fn getName(&self, name: *mut String128) -> tresult {
        *name = String::from("Host Application")
            .chars()
            .map(|c| c as char16)
            .collect::<Vec<char16>>()
            .try_into()
            .expect("name should always be less than 128 chars");
        TResult::KResultOk.to_i32().unwrap()
    }

    // TODO: fix this
    unsafe fn createInstance(
        &self,
        cid: *mut TUID,
        _iid: *mut TUID,
        obj: *mut *mut ::std::ffi::c_void,
    ) -> tresult {
        *obj = std::ptr::null_mut();
        TResult::KResultFalse.to_i32().unwrap()
    }
}

impl IAttributeListTrait for PluginHost {
    unsafe fn setInt(&self, id: AttrID, value: int64) -> tresult {
        todo!()
    }

    unsafe fn getInt(&self, id: AttrID, value: *mut int64) -> tresult {
        todo!()
    }

    unsafe fn setFloat(&self, id: AttrID, value: f64) -> tresult {
        todo!()
    }

    unsafe fn getFloat(&self, id: AttrID, value: *mut f64) -> tresult {
        todo!()
    }

    unsafe fn setString(&self, id: AttrID, string: *const TChar) -> tresult {
        todo!()
    }

    unsafe fn getString(
        &self,
        id: AttrID,
        string: *mut TChar,
        sizeInBytes: uint32,
    ) -> tresult {
        todo!()
    }

    unsafe fn setBinary(
        &self,
        id: AttrID,
        data: *const ::std::ffi::c_void,
        sizeInBytes: uint32,
    ) -> tresult {
        todo!()
    }

    unsafe fn getBinary(
        &self,
        id: AttrID,
        data: *mut *const ::std::ffi::c_void,
        sizeInBytes: *mut uint32,
    ) -> tresult {
        todo!()
    }
}

impl IComponentHandlerTrait for PluginHost {
    unsafe fn beginEdit(&self, id: ParamID) -> tresult {
        return kNotImplemented;
    }

    unsafe fn performEdit(&self, id: ParamID, valueNormalized: ParamValue) -> tresult {
        return kNotImplemented;
    }

    unsafe fn endEdit(&self, id: ParamID) -> tresult {
        return kNotImplemented;
    }

    unsafe fn restartComponent(&self, flags: int32) -> tresult {
        return kNotImplemented;
    }
}

impl IEventListTrait for PluginHost {
    unsafe fn getEventCount(&self) -> int32 {
        todo!()
    }

    unsafe fn getEvent(&self, index: int32, e: *mut Event) -> tresult {
        todo!()
    }

    unsafe fn addEvent(&self, e: *mut Event) -> tresult {
        todo!()
    }
}

impl IUnitHandlerTrait for PluginHost {
    unsafe fn notifyUnitSelection(&self, unitId: UnitID) -> tresult {
        todo!()
    }

    unsafe fn notifyProgramListChange(
        &self,
        listId: ProgramListID,
        programIndex: int32,
    ) -> tresult {
        todo!()
    }
}

impl IMessageTrait for PluginHost {
    unsafe fn getMessageID(&self) -> FIDString {
        todo!()
    }

    unsafe fn setMessageID(&self, id: FIDString) {
        todo!()
    }

    unsafe fn getAttributes(&self) -> *mut IAttributeList {
        todo!()
    }
}

impl IParamValueQueueTrait for PluginHost {
    unsafe fn getParameterId(&self) -> ParamID {
        todo!()
    }

    unsafe fn getPointCount(&self) -> int32 {
        todo!()
    }

    unsafe fn getPoint(
        &self,
        index: int32,
        sampleOffset: *mut int32,
        value: *mut ParamValue,
    ) -> tresult {
        todo!()
    }

    unsafe fn addPoint(
        &self,
        sampleOffset: int32,
        value: ParamValue,
        index: *mut int32,
    ) -> tresult {
        todo!()
    }
}

impl IParameterChangesTrait for PluginHost {
    unsafe fn getParameterCount(&self) -> int32 {
        todo!()
    }

    unsafe fn getParameterData(&self, index: int32) -> *mut IParamValueQueue {
        todo!()
    }

    unsafe fn addParameterData(
        &self,
        id: *const ParamID,
        index: *mut int32,
    ) -> *mut IParamValueQueue {
        todo!()
    }
}

// and IPlugFrame
