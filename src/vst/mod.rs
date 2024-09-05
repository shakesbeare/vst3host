use std::{
    ffi::CStr,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use module::Module;
use vst3::{com_scrape_types::SmartPtr, ComPtr, ComRef, Interface};

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

pub trait IntoVstPtr<'a, T>
where
    T: Interface,
{
    fn into_vstptr(self, module: &'a Module) -> VstPtr<T>;
}

impl<'a, T> IntoVstPtr<'a, T> for ComPtr<T>
where
    T: Interface,
{
    fn into_vstptr(self, module: &'a Module) -> VstPtr<T> {
        VstPtr::new(self, module)
    }
}

pub struct VstPtr<'a, T>
where
    T: Interface,
{
    ptr: ComPtr<T>,
    lifetime: PhantomData<&'a Module>,
}

impl<'a, T: Interface> VstPtr<'a, T> {
    pub fn new(com_ptr: ComPtr<T>, _module: &'a Module) -> Self {
        Self {
            ptr: com_ptr,
            lifetime: PhantomData,
        }
    }
}

impl<'a, T> Deref for VstPtr<'a, T>
where
    T: Interface,
{
    type Target = ComPtr<T>;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl<'a, T> DerefMut for VstPtr<'a, T>
where
    T: Interface,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ptr
    }
}

impl<'a, T: Interface> VstPtr<'a, T> {
    fn into_com_ptr(self) -> ComPtr<T> {
        self.ptr
    }

    fn as_com_ref(&'a self) -> ComRef<'a, T> {
        self.ptr.as_com_ref()
    }

    fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}

impl<'a, T: Interface> SmartPtr for VstPtr<'a, T> {
    type Target = T;

    fn ptr(&self) -> *mut Self::Target {
        self.as_ptr()
    }
}
