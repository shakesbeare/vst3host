#include "GLFW/glfw3.h"
#include "pluginterfaces/base/funknown.h"
#include "pluginterfaces/base/smartpointer.h"
#include "pluginterfaces/gui/iplugview.h"
#include "pluginterfaces/vst/ivstaudioprocessor.h"
#include "public.sdk/source/vst/hosting/module.h"
#include "public.sdk/source/vst/hosting/plugprovider.h"
#include <iostream>
#include <print>

#include "app.h"
#include "component_handler.h"
#include "window.h"

namespace Host {
    static ComponentHandler gComponentHandler;

    static void error_callback(int error, const char *description) {
        std::println("GLFW Error: {}", description);
    }

    App::App() : m_windowManager{WindowManager()} {
        glfwSetErrorCallback(error_callback);
        if (!glfwInit()) {
        }
        glfwWindowHint(GLFW_RESIZABLE, GL_FALSE);
    }

    App::~App() {}

    void App::run() {
        std::string error;
        auto mod = VST3::Hosting::Module::create(
            "C:/Program Files/Common Files/VST3/OTT.vst3", error);
        Steinberg::IPtr<Steinberg::Vst::PlugProvider> plug_provider{nullptr};
        if (!mod) {
            std::println("{}", error);
            throw std::runtime_error("Failed to load module");
        }

        auto factory = mod->getFactory();

        for (auto &class_info : factory.classInfos()) {
            if (class_info.category() == kVstAudioEffectClass) {
                plug_provider =
                    Steinberg::owned(new Steinberg::Vst::PlugProvider(
                        factory, class_info, true));
                if (plug_provider->initialize() == false)
                    plug_provider = nullptr;
                break;
            }
        }

        if (!plug_provider) {
            throw std::runtime_error("No VST3 Audio Module Class");
        }

        auto edit_controller = plug_provider->getController();
        if (!edit_controller) {
            throw std::runtime_error("No EditController found");
        }
        edit_controller->release(); // plug_provider does an addRef, this is
                                    // important, I guess
        edit_controller->setComponentHandler(&gComponentHandler);

        std::println("");
        for (int i = 0; i < edit_controller->getParameterCount(); ++i) {
            Steinberg::Vst::ParameterInfo info;
            edit_controller->getParameterInfo(i, info);
            std::println("Param {}", info.id);
            std::wstring title(reinterpret_cast<wchar_t *>(info.title));
            std::wcout << "\tTitle: " << title << std::endl;
            std::wstring stitle(reinterpret_cast<wchar_t *>(info.shortTitle));
            std::wcout << "\tShort Title: " << stitle << std::endl;
            std::wstring units(reinterpret_cast<wchar_t *>(info.units));
            std::wcout << "\tUnits: " << stitle << std::endl;
            std::println("\tStep Count: {}", info.stepCount);
            std::println("\tDefault Normalized Value: {}",
                         info.defaultNormalizedValue);
            std::println("\tUnit ID: {}", info.unitId);
            std::println("\tFlags: {:b}", info.flags);
        }

        // create view
        auto view = owned(
            edit_controller->createView(Steinberg::Vst::ViewType::kEditor));
        Steinberg::ViewRect plug_view_size{};
        auto result = view->getSize(&plug_view_size);
        if (result != Steinberg::kResultTrue) {
            throw std::runtime_error("Could not get editor view size");
        }

        int id = m_windowManager.newWindow((char *)"Editor",
                                           plug_view_size.getWidth(),
                                           plug_view_size.getHeight());
        auto handle = m_windowManager.getWindow(id).getNativePtr();

        view->setFrame(&m_windowManager.getWindow(id));

        if (view->attached(handle.asPtr(), handle.windowType()) !=
            Steinberg::kResultTrue) {
            throw std::runtime_error("Attaching PlugView failed");
        }

        while (m_windowManager.hasActiveWindows()) {
            m_windowManager.updateWindows();
        }
    }

    void App::exit() { m_windowManager.removeAllWindows(); }
} // namespace Host
