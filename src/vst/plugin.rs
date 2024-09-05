use anyhow::{bail, Result};
use num_traits::ToPrimitive;
use std::ffi::c_void;
use std::sync::Arc;
use std::{ffi::CStr, mem::MaybeUninit};
use thiserror::Error;
use vst3::Steinberg::Vst::ViewType::kEditor;
use vst3::Steinberg::Vst::{
    IComponent, IComponentTrait, IEditController, IEditControllerTrait,
    IHostApplication, IoModes_,
};
use vst3::Steinberg::{
    kResultTrue, FUnknown, IPlugView, IPluginBaseTrait, IPluginFactory,
    IPluginFactoryTrait, PClassInfo, PFactoryInfo,
};
use vst3::{ComPtr, ComRef, Interface};

use crate::TResult;

use super::module::{self, Module};
use super::{host, ClassCategory, IntoVstPtr, VstPtr};

pub struct VstPlugin<'a> {
    pub factory: VstPtr<'a, IPluginFactory>,
    pub metadata: PluginMetadata,
    pub classes: Vec<PluginClass<'a>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PluginMetadata {
    pub url: String,
    pub email: String,
    pub vendor: String,
}

pub struct PluginClass<'a> {
    pub metadata: ClassMetadata,
    pub component: VstPtr<'a, IComponent>,
    pub edit_controller: VstPtr<'a, IEditController>,
    pub plug_view: VstPtr<'a, IPlugView>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ClassMetadata {
    pub name: String,
    pub category: ClassCategory,
}

impl<'a> VstPlugin<'a> {
    pub fn new(
        module: &'a module::Module,
        host: &mut host::PluginHost,
    ) -> Result<Self> {
        if !module.entry() {
            bail!(PluginError::EntryFailed);
        }

        let mut factory = module.factory().into_vstptr(module);
        let num_classes = unsafe { module.factory().countClasses() };

        let mut factory_info: MaybeUninit<PFactoryInfo> = MaybeUninit::uninit();
        unsafe {
            factory.getFactoryInfo(factory_info.as_mut_ptr());
        };
        let factory_info: PFactoryInfo = unsafe { factory_info.assume_init() };

        let url = unsafe { CStr::from_ptr(factory_info.url.as_ptr()) }
            .to_str()?
            .to_string();
        let email = unsafe { CStr::from_ptr(factory_info.email.as_ptr()) }
            .to_str()?
            .to_string();
        let vendor = unsafe { CStr::from_ptr(factory_info.vendor.as_ptr()) }
            .to_str()?
            .to_string();

        let metadata = PluginMetadata { url, email, vendor };
        let mut classes = vec![];
        for i in 0..num_classes {
            classes.push(PluginClass::new(&mut factory, i, host, module)?);
        }

        Ok(VstPlugin {
            factory,
            metadata,
            classes,
        })
    }
}

impl<'a> PluginClass<'a> {
    pub fn new(
        factory: &mut VstPtr<IPluginFactory>,
        class_idx: i32,
        host: &mut host::PluginHost,
        module: &'a Module,
    ) -> Result<PluginClass<'a>> {
        let mut class_info: MaybeUninit<PClassInfo> = MaybeUninit::uninit();
        unsafe { factory.getClassInfo(class_idx, class_info.as_mut_ptr()) };
        let mut class_info: PClassInfo = unsafe { class_info.assume_init() };

        let category: ClassCategory = class_info.category.as_ptr().into();
        let name = unsafe { CStr::from_ptr(class_info.name.as_mut_ptr()) }
            .to_str()?
            .to_string();

        let mut component: MaybeUninit<*mut IComponent> = MaybeUninit::uninit();
        let create_instance_result = unsafe {
            factory.createInstance(
                class_info.cid.as_ptr(),
                IComponent::IID.as_ptr() as *const i8,
                component.as_mut_ptr() as *mut *mut c_void,
            )
        };

        if !create_instance_result
            == TResult::KResultOk
                .to_i32()
                .expect("This is guaranteed to be possible")
        {
            bail!(PluginClassError::ClassInstantiationFailed(
                name,
                create_instance_result
            ));
        }

        let component_ptr: *mut IComponent = unsafe { component.assume_init() };
        let component =
            unsafe { ComPtr::from_raw(component_ptr).unwrap() }.into_vstptr(&module);
        unsafe { component.setIoMode(IoModes_::kAdvanced as i32) };

        // let hostptr = unsafe { ComRef::from_raw(host as *mut host::PluginHost as *mut FUnknown).unwrap() };
        // unsafe { component.initialize(hostptr.as_ptr()) };
        unsafe { component.initialize(std::ptr::null_mut()) };

        let mut edit: MaybeUninit<*mut IEditController> = MaybeUninit::uninit();
        let create_instance_result = unsafe {
            factory.createInstance(
                class_info.cid.as_ptr(),
                IEditController::IID.as_ptr() as *const i8,
                edit.as_mut_ptr() as *mut *mut c_void,
            )
        };

        if create_instance_result != kResultTrue {
            panic!(
                "Failed to properly create IEditController instance, error code: {}",
                create_instance_result
            );
        }

        // Can now assume that edit points to valid memory
        let edit_ptr: *mut IEditController = unsafe { edit.assume_init() };
        let edit_controller =
            unsafe { ComPtr::from_raw(edit_ptr).unwrap() }.into_vstptr(module);

        // unsafe { component.initialize(host as *mut  host::PluginHost as *mut FUnknown) };
        unsafe { component.initialize(std::ptr::null_mut()) };

        let view_ptr = unsafe { edit_controller.createView(kEditor) };
        let plug_view = unsafe {
            std::mem::transmute::<*mut IPlugView, ComPtr<IPlugView>>(view_ptr)
        }
        .into_vstptr(module);

        Ok(PluginClass {
            metadata: ClassMetadata { name, category },
            component,
            edit_controller,
            plug_view,
        })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Error)]
pub enum PluginError {
    #[error("Failed to instantiate module")]
    EntryFailed,
    #[error("Failed to instantiate one or more classes: {0}")]
    #[from(ClassError)]
    PluginClassError(String),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Error)]
pub enum PluginClassError {
    #[error("Failed to instantiate class '{0}' with error code '{1}'")]
    ClassInstantiationFailed(String, i32),
}
// // instantiate the classes
//
