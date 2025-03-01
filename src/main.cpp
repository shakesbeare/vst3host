#include <print>
#include "GLFW/glfw3.h"
#include "pluginterfaces/base/funknown.h"
#include "pluginterfaces/base/smartpointer.h"
#include "pluginterfaces/gui/iplugview.h"
#include "public.sdk/source/vst/hosting/plugprovider.h"
#include "public.sdk/source/vst/hosting/module.h"
#include "pluginterfaces/vst/ivstaudioprocessor.h"

#include "window.h"
#include "component_handler.h"


static void error_callback(int error, const char* description) {
    std::println("GLFW Error: {}", description);
}

static ComponentHandler gComponentHandler;

int main() {
    glfwSetErrorCallback(error_callback);
    if (!glfwInit()) {
    }
    glfwWindowHint(GLFW_RESIZABLE, GL_FALSE);


    WindowManager wm = WindowManager();

    std::string error;
    auto mod = VST3::Hosting::Module::create("C:/Program Files/Common Files/VST3/OTT.vst3", error);
    Steinberg::IPtr<Steinberg::Vst::PlugProvider> plug_provider {nullptr};
    if (!mod) {
        std::println("Failed to load module");
        std::println("{}", error);
        return -1;
    }

    auto factory = mod->getFactory();


    for (auto& class_info : factory.classInfos()) {
        if (class_info.category() == kVstAudioEffectClass) {
            plug_provider = Steinberg::owned(new Steinberg::Vst::PlugProvider(factory, class_info, true));
            if (plug_provider->initialize() == false)
                plug_provider = nullptr;
            break;
        }
    }

    if (!plug_provider) {
        std::println("No VST3 Audio Module Class");
        return -1;
    }

    auto edit_controller = plug_provider->getController();
    if (!edit_controller) {
        std::println("No EditController found");
        return -1;
    }
    edit_controller->release(); // plug_provider does an addRef, this is important, I guess
    edit_controller->setComponentHandler(&gComponentHandler);
    
    // create view
    auto view = owned(edit_controller->createView(Steinberg::Vst::ViewType::kEditor));
    Steinberg::ViewRect plug_view_size {};
    auto result = view->getSize(&plug_view_size);
    if (result != Steinberg::kResultTrue) {
        std::println("Could not get editor view size");
        return -1;
    }

    int id = wm.new_window((char*)"Editor", plug_view_size.getWidth(), plug_view_size.getHeight());
    auto handle = wm.get_window(id).get_native_ptr();

    view->setFrame(&wm.get_window(id));

    if (view->attached(handle.as_ptr(), handle.window_type()) != Steinberg::kResultTrue) {
        std::println("Attaching PlugView failed");
        return -1;
    }

    while (wm.has_active_windows()) {
        wm.update_windows();
    }

    return 0;
}
