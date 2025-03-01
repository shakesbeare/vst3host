#ifndef EDITORHOST_MODULE
#define EDITORHOST_MODULE

#include "public.sdk/source/vst/hosting/module.h"

class Module {
public:
    Module();
private:
    VST3::Hosting::Module::Ptr module {nullptr};
};

#endif
