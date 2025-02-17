cmake_minimum_required(VERSION 3.21)
project(editorhost LANGUAGES CXX)

set(vst3sdk_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/vst3sdk")
list(APPEND CMAKE_MODULE_PATH "${vst3sdk_SOURCE_DIR}/cmake/modules")
include(SMTG_VST3_SDK)
smtg_setup_platform_toolset()
smtg_setup_symbol_visibility()

add_compile_definitions(DEVELOPMENT=true)

set(SMTG_ENABLE_VST3_PLUGIN_EXAMPLES FALSE)
set(SMTG_ENABLE_VST3_HOSTING_EXAMPLES FALSE)
set(SMTG_ENABLE_VSTGUI_SUPPORT FALSE)

set(editorhost_sources 
    src/main.cpp
)

add_subdirectory(${vst3sdk_SOURCE_DIR} ${PROJECT_BINARY_DIR}/vst3sdk)
set(target editorhost)
add_executable(${target} ${editorhost_sources})

target_include_directories(editorhost PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/vst3sdk>
)

target_compile_features(${target}
    PUBLIC
        cxx_std_17
)
target_link_libraries(${target}
    PRIVATE
        sdk_hosting
        ${editorhost_PLATFORM_LIBS}
)
# # # SDL2::SDL2main may or may not be available. It is e.g. required by Windows GUI applications
# # if(TARGET SDL2::SDL2main)
# #     # It has an implicit dependency on SDL2 functions, so it MUST be added before SDL2::SDL2 (or SDL2::SDL2-static)
# #     target_link_libraries(my_application PRIVATE SDL2::SDL2main)
# # endif()
#
smtg_target_setup_universal_binary(${target})
if (APP_PROPERTIES)
    set_target_properties(${target}
        PROPERTIES
            ${APP_PROPERTIES}
    )
endif(APP_PROPERTIES)
