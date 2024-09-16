use std::{ffi::c_void, str::FromStr};

use libloading::{Library, Symbol};

#[cfg(target_os = "macos")]
use core_foundation::{
    base::TCFType,
    bundle::CFBundle,
    string::CFString,
    url::{kCFURLPOSIXPathStyle, CFURL},
};
use vst3::{ComPtr, Steinberg::IPluginFactory3 as IPluginFactory};

#[cfg(target_os = "macos")]
const OS_LABEL: &str = "MacOS";

pub struct Module {
    lib: Library,
    bundle_ref: *const c_void,
}

impl Module {
    pub fn new(path: &str) -> Self {
        let path_buf = std::path::PathBuf::from(path);
        let exec_name = path_buf
            .components()
            .last()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .split_once('.')
            .unwrap()
            .0;

        let dylib_path = path_buf.join("Contents").join(OS_LABEL).join(exec_name);

        let lib = unsafe { Library::new(dylib_path).unwrap() };

        #[cfg(target_os = "macos")]
        {
            let cfstr_path = CFString::from_str(path).unwrap();
            let cfurl_path =
                CFURL::from_file_system_path(cfstr_path, kCFURLPOSIXPathStyle, true);
            let cf_bundle = CFBundle::new(cfurl_path)
                .expect("Plugin not present")
                .as_CFTypeRef();
            Module {
                lib,
                bundle_ref: cf_bundle,
            }
        }
    }

    pub fn entry(&self) -> bool {
        let bundle_entry: Symbol<fn(*const c_void) -> bool> =
            unsafe { self.lib.get(b"bundleEntry").unwrap() };
        bundle_entry(self.bundle_ref)
    }

    pub fn exit(&self) -> bool {
        let bundle_exit: Symbol<fn(*const c_void) -> bool> =
            unsafe { self.lib.get(b"bundleExit").unwrap() };
        bundle_exit(self.bundle_ref)
    }

    pub fn factory(&self) -> ComPtr<IPluginFactory> {
        let get_plugin_factory: Symbol<fn() -> *mut IPluginFactory> =
            unsafe { self.lib.get(b"GetPluginFactory").unwrap() };
        unsafe { ComPtr::from_raw(get_plugin_factory()).unwrap() }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        self.exit();
    }
}
