#pragma once
#include "pluginterfaces/gui/iplugview.h"
#include "public.sdk/source/vst/hosting/plugprovider.h"
#include "public.sdk/source/vst/hosting/plugprovider.h"
#include "public.sdk/source/vst/hosting/module.h"
#include <filesystem>

namespace Plugin {
    class Plugin {
    public:
        Plugin(std::filesystem::path path);
        ~Plugin();
        const std::string& getName();
        const Steinberg::ViewRect& getSize();
        Steinberg::IPtr<Steinberg::Vst::PlugProvider> getPlugProvider();
    private:
        std::string m_name;
        Steinberg::ViewRect m_size;
        Steinberg::Vst::IEditController* m_editController;
        Steinberg::IPtr<Steinberg::Vst::PlugProvider> m_plugProvider;
        std::shared_ptr<VST3::Hosting::Module> m_module;
    };
}
