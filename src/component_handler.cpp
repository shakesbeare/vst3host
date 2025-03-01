#include <print>
#include "component_handler.h"

Steinberg::tresult ComponentHandler::beginEdit(Steinberg::Vst::ParamID id) {
    std::println("Begin Edit param {}", id);
    return Steinberg::kResultTrue;
}

Steinberg::tresult ComponentHandler::performEdit(Steinberg::Vst::ParamID id, Steinberg::Vst::ParamValue valueNormalized) {
    std::println("Edit performed on param {} with value {}", id, valueNormalized);
    return Steinberg::kResultTrue;
}

Steinberg::tresult ComponentHandler::endEdit(Steinberg::Vst::ParamID id) {
    std::println("End edit on param {}", id);
    return Steinberg::kResultTrue;
}

Steinberg::tresult ComponentHandler::restartComponent(Steinberg::int32 flags) {
    std::println("Component restarted");
    return Steinberg::kResultTrue;
}

Steinberg::tresult ComponentHandler::queryInterface(const Steinberg::TUID _iid, void **obj) {

    return Steinberg::kNoInterface;
}

Steinberg::uint32 ComponentHandler::addRef() {
    return 1000;
}

Steinberg::uint32 ComponentHandler::release() {
    return 1000;
}
