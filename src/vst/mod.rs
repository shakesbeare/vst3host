use std::ffi::CStr;

pub mod host;
pub mod module;
pub mod plugin;

const K_VST_AUDIO_EFFECT_CLASS: &CStr = c"Audio Module Effect Class";
const K_VST_COMPONENT_CONTROLLER_CLASS: &CStr = c"Component Controller Class";

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ClassCategory {
    VstAudioEffectClass,
    VstComponentControllerClass,
    NotFound,
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
impl From<*const i8> for ClassCategory {
    fn from(value: *const i8) -> Self {
        let string = unsafe { CStr::from_ptr(value) };
        if string == K_VST_AUDIO_EFFECT_CLASS {
            Self::VstAudioEffectClass
        } else if string == K_VST_COMPONENT_CONTROLLER_CLASS {
            Self::VstComponentControllerClass
        } else {
            Self::NotFound
        }
    }
}
