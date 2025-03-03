#pragma once
#include "pluginterfaces/base/ftypes.h"
#include "pluginterfaces/vst/ivsteditcontroller.h"

namespace Host {
    class ComponentHandler : public Steinberg::Vst::IComponentHandler { 
    public:
        ComponentHandler() {}
        ~ComponentHandler() {}

        virtual Steinberg::tresult beginEdit(Steinberg::Vst::ParamID id) override;
        virtual Steinberg::tresult performEdit(Steinberg::Vst::ParamID id, Steinberg::Vst::ParamValue valueNormalized) override;
        virtual Steinberg::tresult endEdit(Steinberg::Vst::ParamID id) override;
        virtual Steinberg::tresult restartComponent(Steinberg::int32 flags) override;
    private:
        virtual Steinberg::tresult queryInterface(const Steinberg::TUID _iid, void **obj) override;
        virtual Steinberg::uint32 addRef() override;
        virtual Steinberg::uint32 release() override;
    };
}
