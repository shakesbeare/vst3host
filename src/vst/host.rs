use num_traits::ToPrimitive;
use vst3::Steinberg::{char16, tresult, Vst::*, TUID};

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
        _cid: *mut TUID,
        _iid: *mut TUID,
        obj: *mut *mut ::std::ffi::c_void,
    ) -> tresult {
        *obj = std::ptr::null_mut();
        TResult::KResultFalse.to_i32().unwrap()
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
