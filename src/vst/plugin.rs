use anyhow::{bail, Result};
use num_traits::ToPrimitive;
use std::ffi::CString;
use std::{ffi::CStr, mem::MaybeUninit};
use thiserror::Error;
use vst3_com::sys::GUID;
use vst3_com::VstPtr;
use vst3_sys::base::{IPluginBase, IPluginFactory, PClassInfo, PFactoryInfo};
use vst3_sys::gui::IPlugView;
use vst3_sys::vst::{
    IComponent, IEditController, IoModes, IID_ICOMPONENT, IID_IEDIT_CONTROLLER,
};

use crate::TResult;

use super::module;
use super::ClassCategory;

pub struct VstPlugin {
    pub metadata: PluginMetadata,
    pub classes: Vec<PluginClass>,
}

pub struct PluginMetadata {
    pub url: String,
    pub email: String,
    pub vendor: String,
}

pub struct PluginClass {
    pub metadata: ClassMetadata,
    pub component: VstPtr<dyn IComponent>,
    pub edit_controller: VstPtr<dyn IEditController>,
    pub plug_view: VstPtr<dyn IPlugView>,
}

pub struct ClassMetadata {
    pub name: String,
    pub category: ClassCategory,
}

impl VstPlugin {
    pub fn new(module: module::Module) -> Result<Self> {
        if !module.entry() {
            bail!(PluginError::EntryFailed);
        }

        let mut factory = module.factory();
        let num_classes = unsafe { factory.count_classes() };

        let mut factory_info: MaybeUninit<PFactoryInfo> = MaybeUninit::uninit();
        unsafe { factory.get_factory_info(factory_info.as_mut_ptr()) };
        let mut factory_info: PFactoryInfo = unsafe { factory_info.assume_init() };

        let url = unsafe { CStr::from_ptr(factory_info.url.as_mut_ptr()) }
            .to_str()?
            .to_string();
        let email = unsafe { CStr::from_ptr(factory_info.email.as_mut_ptr()) }
            .to_str()?
            .to_string();
        let vendor = unsafe { CStr::from_ptr(factory_info.vendor.as_mut_ptr()) }
            .to_str()?
            .to_string();

        let metadata = PluginMetadata { url, email, vendor };

        let mut classes = vec![];
        for i in 0..num_classes {
            classes.push(PluginClass::new(&mut factory, i)?);
        }

        Ok(VstPlugin { metadata, classes })
    }
}

impl PluginClass {
    pub fn new(
        factory: &mut VstPtr<dyn IPluginFactory>,
        class_idx: i32,
    ) -> Result<PluginClass> {
        let mut class_info: MaybeUninit<PClassInfo> = MaybeUninit::uninit();
        unsafe { factory.get_class_info(class_idx, class_info.as_mut_ptr()) };
        let mut class_info: PClassInfo = unsafe { class_info.assume_init() };

        let category: ClassCategory = class_info.category.as_ptr().into();
        let name = unsafe { CStr::from_ptr(class_info.name.as_mut_ptr()) }
            .to_str()?
            .to_string();

        let mut component: MaybeUninit<VstPtr<dyn IComponent>> = MaybeUninit::uninit();
        let create_instance_result = unsafe {
            factory.create_instance(
                &class_info.cid as *const GUID,
                &IID_ICOMPONENT as *const GUID,
                component.as_mut_ptr() as *mut *mut dyn IComponent as _,
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

        let component: VstPtr<dyn IComponent> = unsafe { component.assume_init() };
        unsafe { component.set_io_mode(IoModes::kAdvanced as i32) };
        // TODO: pass the host context IHostApplication
        unsafe { component.initialize(std::ptr::null_mut()) };

        let mut edit: MaybeUninit<VstPtr<dyn IEditController>> = MaybeUninit::uninit();
        let create_instance_result = unsafe {
            factory.create_instance(
                &class_info.cid as *const GUID,
                &IID_IEDIT_CONTROLLER as *const GUID,
                edit.as_mut_ptr() as *mut *mut dyn IComponent as _,
            )
        };

        if create_instance_result != vst3_sys::base::kResultTrue {
            panic!(
                "Failed to properly create IEditController instance, error code: {}",
                create_instance_result
            );
        }

        // Can now assume that edit points to valid memory
        let edit_controller: VstPtr<dyn IEditController> =
            unsafe { edit.assume_init() };

        // TODO: pass the host context IHostApplication
        unsafe { edit_controller.initialize(std::ptr::null_mut()) };

        let view_descriptor = CString::new("editor").unwrap();
        let view_ptr = unsafe { edit_controller.create_view(view_descriptor.as_ptr()) };
        let plug_view = unsafe {
            std::mem::transmute::<
                *mut vst3_sys::c_void,
                vst3_sys::VstPtr<dyn vst3_sys::gui::IPlugView>,
            >(view_ptr)
        };

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
