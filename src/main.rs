use core_foundation::base::TCFType;
use core_foundation::bundle::CFBundle;
use core_foundation::string::CFString;
use core_foundation::url::{kCFURLPOSIXPathStyle, CFURL};
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::str::FromStr;
use vst3_sys::base::{
    kResultFalse, kResultTrue, tresult, FIDString, IPluginBase, IPluginFactory,
    IUnknown, PClassInfo, PFactoryInfo,
};
use vst3_sys::gui::{IPlugFrame, IPlugView, ViewRect};
use vst3_sys::sys::GUID;
use vst3_sys::utils::SharedVstPtr;
use vst3_sys::vst::{
    kVstAudioEffectClass, IEditController, IoModes, IID_IEDIT_CONTROLLER,
};
use vst3_sys::vst::{IComponent, IID_ICOMPONENT};
use vst3_sys::VstPtr;
use vst3_sys::{c_void, VST3};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
#[allow(deprecated)]
use winit::raw_window_handle::{
    HasRawWindowHandle, RawWindowHandle,
};

use libloading::{Library, Symbol};

const VST_PATH: &str = "/Library/Audio/Plug-Ins/VST3/OTT.vst3";

#[cfg(target_os = "macos")]
const OS_LABEL: &str = "MacOS";

#[allow(dead_code)]
trait ViewRectExt {
    fn to_logical_size(self) -> winit::dpi::LogicalSize<f32>;
    fn eq(&self, other: &ViewRect) -> bool;
}

#[allow(dead_code)]
trait ToViewRectExt {
    fn to_view_rect(self) -> ViewRect;
}

impl ToViewRectExt for PhysicalSize<u32> {
    fn to_view_rect(self) -> ViewRect {
        ViewRect {
            left: 0,
            top: 0,
            right: self.width as i32,
            bottom: self.height as i32,
        }
    }
}

impl ToViewRectExt for LogicalSize<u32> {
    fn to_view_rect(self) -> ViewRect {
        ViewRect {
            left: 0,
            top: 0,
            right: self.width as i32,
            bottom: self.height as i32,
        }
    }
}

impl ViewRectExt for ViewRect {
    fn to_logical_size(self) -> winit::dpi::LogicalSize<f32> {
        let width = self.right;
        let height = self.bottom;
        winit::dpi::LogicalSize {
            width: width as f32,
            height: height as f32,
        }
    }

    fn eq(&self, other: &ViewRect) -> bool {
        matches!((
            self.right == other.right,
            self.bottom == other.bottom,
            self.top == other.top,
            self.left == other.left,
        ), (true, true, true, true))
    }
}

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
        // let bundle_ref = unsafe { CFBundleGetBundleWithIdentifier("com.xfer.OTT.VST3".into()) };

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

    pub fn factory(&self) -> VstPtr<dyn IPluginFactory> {
        let get_plugin_factory: Symbol<fn() -> VstPtr<dyn IPluginFactory>> =
            unsafe { self.lib.get(b"GetPluginFactory").unwrap() };
        get_plugin_factory()
    }
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    let event_loop = EventLoop::new().unwrap();
    // let mut view = View {
    //     window: None,
    //     plug_view: plugview,
    //     reference_count: 1,
    //     __iunknownvptr: todo!(),
    //     __iplugframevptr: todo!(),
    //     __refcnt: 1.into(),
    //
    // };
    let mut view = View::new();
    event_loop.run_app(&mut view)?;

    Ok(())
}

#[VST3(implements(IUnknown, IPlugFrame))]
struct View {
    window: Option<winit::window::Window>,
    plug_view: VstPtr<dyn IPlugView>,
}

impl View {
    pub(crate) fn new() -> Box<View> {
        let module = Module::new(VST_PATH);
        if module.entry() {
            println!("module loaded");
        }
        let factory = module.factory();
        println!("module has {:?} class(es)", unsafe {
            factory.count_classes()
        });

        let mut factory_info: MaybeUninit<PFactoryInfo> = MaybeUninit::uninit();
        unsafe { factory.get_factory_info(factory_info.as_mut_ptr()) };
        let mut factory_info: PFactoryInfo = unsafe { factory_info.assume_init() };

        let url = unsafe { CStr::from_ptr(factory_info.url.as_mut_ptr()) };
        let email = unsafe { CStr::from_ptr(factory_info.email.as_mut_ptr()) };
        let vendor = unsafe { CStr::from_ptr(factory_info.vendor.as_mut_ptr()) };

        println!("{:?}", url);
        println!("{:?}", email);
        println!("{:?}", vendor);

        let mut class_info: MaybeUninit<PClassInfo> = MaybeUninit::uninit();
        unsafe { factory.get_class_info(0, class_info.as_mut_ptr()) };
        let mut class_info: PClassInfo = unsafe { class_info.assume_init() };

        let category = unsafe { CStr::from_ptr(class_info.category.as_mut_ptr()) };
        let name = unsafe { CStr::from_ptr(class_info.name.as_mut_ptr()) };

        println!("{:?}", category);
        println!("{:?}", name);

        // instantiate the classes
        let mut info: MaybeUninit<PClassInfo> = MaybeUninit::uninit();
        unsafe { factory.get_class_info(0, info.as_mut_ptr()) };
        let mut info: PClassInfo = unsafe { info.assume_init() };

        let category = unsafe { CStr::from_ptr(info.category.as_mut_ptr()) };
        let expected = unsafe { CStr::from_ptr(kVstAudioEffectClass) };
        if category != expected {
            panic!("unexpected category on line 212");
        }

        let mut component: MaybeUninit<VstPtr<dyn IComponent>> = MaybeUninit::uninit();
        let create_instance_result = unsafe {
            factory.create_instance(
                &info.cid as *const GUID,
                &IID_ICOMPONENT as *const GUID,
                component.as_mut_ptr() as *mut *mut dyn IComponent as _,
            )
        };

        if create_instance_result != vst3_sys::base::kResultTrue {
            panic!(
                "Failed to properly create IComponent instance, error code: {}",
                create_instance_result
            );
        }

        // Can now assume that component points to valid memory
        let component: VstPtr<dyn IComponent> = unsafe { component.assume_init() };

        unsafe { component.set_io_mode(IoModes::kAdvanced as i32) };
        // TODO: pass the host context IHostApplication
        unsafe { component.initialize(std::ptr::null_mut()) };

        let mut edit: MaybeUninit<VstPtr<dyn IEditController>> = MaybeUninit::uninit();
        let create_instance_result = unsafe {
            factory.create_instance(
                &info.cid as *const GUID,
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
        let edit: VstPtr<dyn IEditController> = unsafe { edit.assume_init() };

        // TODO: pass the host context IHostApplication
        unsafe { edit.initialize(std::ptr::null_mut()) };

        let view_descriptor = CString::new("editor").unwrap();
        let view_ptr = unsafe { edit.create_view(view_descriptor.as_ptr()) };
        let plug_view = unsafe {
            std::mem::transmute::<
                *mut vst3_sys::c_void,
                vst3_sys::VstPtr<dyn vst3_sys::gui::IPlugView>,
            >(view_ptr)
        };

        return Self::allocate(None, plug_view);
    }
}

impl IPlugFrame for View {
    unsafe fn resize_view(
        &self,
        _view: SharedVstPtr<dyn IPlugView>,
        new_size: *mut ViewRect,
    ) -> tresult {
        let new_size = PhysicalSize::<u32> {
            width: (*new_size).right as u32,
            height: (*new_size).bottom as u32,
        };
        let _ = self.window.as_ref().unwrap().request_inner_size(new_size);
        kResultTrue
    }
}

impl View {
    #[allow(deprecated)]
    fn raw_handle(&self) -> Option<RawWindowHandle> {
        if let Some(win) = &self.window {
            let window_handle = win.raw_window_handle().unwrap();
            Some(window_handle)
        } else {
            None
        }
    }

    fn attach_plug_view(&mut self) -> anyhow::Result<()> {
        match self.raw_handle().unwrap() {
            RawWindowHandle::AppKit(ptr) => {
                let ptr = ptr.ns_view.as_ptr();
                let platform_ui =
                    FIDString::from(CString::new("NSView")?.as_ptr());
                unsafe {
                    self.plug_view.attached(ptr, platform_ui);
                }
            }
            _ => todo!(),
        };

        Ok(())
    }
}

impl winit::application::ApplicationHandler for View {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let attributes = winit::window::Window::default_attributes().with_inner_size(
            winit::dpi::LogicalSize {
                width: 340.0,
                height: 482.0,
            },
        );
        let window = event_loop
            .create_window(attributes)
            .expect("failed to create window");
        self.window = Some(window);
        self.attach_plug_view().unwrap();
        let mut view_rect: MaybeUninit<ViewRect> = MaybeUninit::uninit();
        // let mut info: MaybeUninit<PClassInfo> = MaybeUninit::uninit();
        // unsafe { factory.get_class_info(i, info.as_mut_ptr()) };
        unsafe {
            self.plug_view.get_size(view_rect.as_mut_ptr());
        }
        let view_rect = unsafe {
            std::mem::transmute::<
                std::mem::MaybeUninit<vst3_sys::gui::ViewRect>,
                vst3_sys::gui::ViewRect,
            >(view_rect)
        };
        let self_ = unsafe { &mut *(self as *mut Self) } as *mut Self as *mut c_void;
        if unsafe { self.plug_view.can_resize() } == kResultFalse {
            self.window.as_ref().unwrap().set_resizable(false);
        }
        unsafe {
            self.plug_view.set_frame(self_);
        }
        let _ = self
            .window
            .as_ref()
            .unwrap()
            .request_inner_size(view_rect.to_logical_size());
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::Focused(focused) => {
                if focused {
                    tracing::info!("Window={window_id:?} focused");
                } else {
                    tracing::info!("Window={window_id:?} unfocused");
                }
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                tracing::info!("Window={window_id:?} changed scale to {scale_factor}");
            }
            WindowEvent::RedrawRequested => {}
            WindowEvent::CloseRequested => {
                unsafe {
                    self.plug_view.removed();
                }
                std::process::exit(0);
            }
            WindowEvent::Resized(size) => {
                tracing::info!("Window resized to {:?}", size);
            }
            _ => {}
        }
    }
}
