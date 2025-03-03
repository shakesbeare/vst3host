#include "pluginterfaces/base/funknown.h"
#include "pluginterfaces/base/smartpointer.h"
#include "pluginterfaces/gui/iplugview.h"
#include "pluginterfaces/vst/ivstaudioprocessor.h"
#include "public.sdk/source/vst/hosting/module.h"
#include "public.sdk/source/vst/hosting/plugprovider.h"
#include <iostream>
#include <print>

#include "app.h"
#include "plugin.h"

namespace Plugin {
    Plugin::Plugin(std::filesystem::path path) {
        std::string error;
        m_module = VST3::Hosting::Module::create(path.string(), error);
        if (!m_module) {
            std::println("{}", error);
            throw std::runtime_error("Failed to load module");
        }

        m_name = m_module->getName();
        auto factory = m_module->getFactory();

        for (auto &class_info : factory.classInfos()) {
            if (class_info.category() == kVstAudioEffectClass) {
                m_plugProvider =
                    Steinberg::owned(new Steinberg::Vst::PlugProvider(
                        factory, class_info, true));
                if (m_plugProvider->initialize() == false)
                    m_plugProvider = nullptr;
                break;
            }
        }

        if (!m_plugProvider) {
            throw std::runtime_error("No VST3 Audio Module Class");
        }

        /* std::println(""); */
        /* for (int i = 0; i < m_editController->getParameterCount(); ++i) { */
        /*     Steinberg::Vst::ParameterInfo info; */
        /*     m_editController->getParameterInfo(i, info); */
        /*     std::println("Param {}", info.id); */
        /*     std::wstring title(reinterpret_cast<wchar_t *>(info.title)); */
        /*     std::wcout << "\tTitle: " << title << std::endl; */
        /*     std::wstring stitle(reinterpret_cast<wchar_t
         * *>(info.shortTitle)); */
        /*     std::wcout << "\tShort Title: " << stitle << std::endl; */
        /*     std::wstring units(reinterpret_cast<wchar_t *>(info.units)); */
        /*     std::wcout << "\tUnits: " << stitle << std::endl; */
        /*     std::println("\tStep Count: {}", info.stepCount); */
        /*     std::println("\tDefault Normalized Value: {}", */
        /*                  info.defaultNormalizedValue); */
        /*     std::println("\tUnit ID: {}", info.unitId); */
        /*     std::println("\tFlags: {:b}", info.flags); */
        /* } */

    }

    Plugin::~Plugin() {}

    const std::string &Plugin::getName() { return m_name; }

    const Steinberg::ViewRect &Plugin::getSize() { return m_size; }

    Steinberg::IPtr<Steinberg::Vst::PlugProvider> Plugin::getPlugProvider() {
        return m_plugProvider;
    }

} // namespace Plugin
