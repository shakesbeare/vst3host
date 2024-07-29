
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use main::vst::plugin::VstPlugin;
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
use main::vst::module::Module;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use anyhow::Result;
#[allow(deprecated)]
use winit::raw_window_handle::{
    HasRawWindowHandle, RawWindowHandle,
};

const VST_PATH: &str = "/Library/Audio/Plug-Ins/VST3/OTT.vst3";

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


fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    let event_loop = EventLoop::new().unwrap();
    let mut view = View::new()?;
    event_loop.run_app(&mut view)?;

    Ok(())
}

#[VST3(implements(IUnknown, IPlugFrame))]
struct View {
    window: Option<winit::window::Window>,
    plugin: VstPlugin,
}

impl View {
    pub(crate) fn new() -> Result<Box<View>> {
        let module = Module::new(VST_PATH);
        let plugin = VstPlugin::new(module)?;

        Ok(Self::allocate(None, plugin))
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
                    self.plugin.classes[0].plug_view.attached(ptr, platform_ui);
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
        unsafe {
            self.plugin.classes[0].plug_view.get_size(view_rect.as_mut_ptr());
        }
        let view_rect = unsafe {
            std::mem::transmute::<
                std::mem::MaybeUninit<vst3_sys::gui::ViewRect>,
                vst3_sys::gui::ViewRect,
            >(view_rect)
        };
        let self_ = unsafe { &mut *(self as *mut Self) } as *mut Self as *mut c_void;
        if unsafe { self.plugin.classes[0].plug_view.can_resize() } == kResultFalse {
            self.window.as_ref().unwrap().set_resizable(false);
        }
        unsafe {
            self.plugin.classes[0].plug_view.set_frame(self_);
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
                    self.plugin.classes[0].plug_view.removed();
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
