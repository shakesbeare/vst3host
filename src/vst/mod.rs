pub mod module;
pub mod plugin;
pub mod host;

use vst3_sys::vst::{kVstAudioEffectClass, kVstComponentControllerClass};

pub enum ClassCategory {
    VstAudioEffectClass,
    VstComponentControllerClass,
    NotFound,
}

impl From<*const i8> for ClassCategory {
    fn from(value: *const i8) -> Self {
        if value == kVstAudioEffectClass {
            Self::VstAudioEffectClass
        } else if value == kVstComponentControllerClass { 
            Self::VstComponentControllerClass
        } else {
            Self::NotFound
        }
    }
}
