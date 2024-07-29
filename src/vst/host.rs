use std::ffi::CString;

use num_traits::ToPrimitive;
use vst3_com::{c_void, IID};
use vst3_sys::{
    base::{tresult, IUnknown},
    vst::{IHostApplication, IMessage},
    VST3,
};

use crate::TResult;

#[VST3(implements(IUnknown))]
pub struct PluginHost {}

impl IHostApplication for PluginHost {
    unsafe fn get_name(&self, name: *mut u16) -> tresult {
        *name = String::from("Host Application")
            .encode_utf16()
            .by_ref()
            .next()
            .unwrap();
        TResult::KResultOk.to_i32().unwrap()
    }

    // TODO: fix this
    unsafe fn create_instance(
        &self,
        cid: *const IID,
        _iid: *const IID,
        obj: *mut *mut c_void,
    ) -> tresult {
        *obj = std::ptr::null_mut();
        TResult::KResultFalse.to_i32().unwrap()
    }
}

impl PluginHost {
    pub fn new() -> Box<Self> {
        Self::allocate()
    }
}

// sample implementation...
// tresult PLUGIN_API Validator::createInstance (TUID cid, TUID iid, void** obj)
// {
// 	FUID classID = FUID::fromTUID (cid);
// 	FUID interfaceID = FUID::fromTUID (iid);
// 	if (classID == IMessage::iid && interfaceID == IMessage::iid)
// 	{
// 		*obj = new HostMessage;
// 		return kResultTrue;
// 	}
// 	else if (classID == IAttributeList::iid && interfaceID == IAttributeList::iid)
// 	{
// 		if (auto al = HostAttributeList::make ())
// 		{
// 			*obj = al.take ();
// 			return kResultTrue;
// 		}
// 		return kOutOfMemory;
// 	}
// 	*obj = nullptr;
// 	return kResultFalse;
// }
