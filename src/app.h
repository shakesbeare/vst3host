#pragma once

#include "window.h"

namespace Host {
    class App {
    public:
        App();
        ~App();

        void run();
        void exit();

    private:
        WindowManager m_windowManager {};
    };
} // namespace Host
