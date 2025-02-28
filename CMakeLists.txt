cmake_minimum_required(VERSION 3.21)
project(editorhost LANGUAGES CXX)
set(target editorhost)

set(vst3sdk_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/deps/vst3sdk")
list(APPEND CMAKE_MODULE_PATH "${vst3sdk_SOURCE_DIR}/cmake/modules")

include(SMTG_VST3_SDK)
smtg_setup_platform_toolset()
smtg_setup_symbol_visibility()

add_compile_definitions(DEVELOPMENT=true)

set(SMTG_ENABLE_VST3_PLUGIN_EXAMPLES OFF)
set(SMTG_ENABLE_VST3_HOSTING_EXAMPLES OFF)
set(SMTG_ENABLE_VSTGUI_SUPPORT OFF)
set(SMTG_RUN_VST_VALIDATOR OFF)

set(editorhost_sources 
    src/main.cpp
)

add_subdirectory(${vst3sdk_SOURCE_DIR} ${PROJECT_BINARY_DIR}/vst3sdk)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/$<CONFIGURATION>")
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/$<CONFIGURATION>")

add_subdirectory(deps/glfw EXCLUDE_FROM_ALL)

add_executable(${target} ${editorhost_sources})

target_link_libraries(${target} PRIVATE glfw)
find_package(OpenGL REQUIRED)
target_link_libraries(${target} PRIVATE OpenGL::GL)

target_include_directories(${target} PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/vst3sdk>
)


target_compile_features(${target}
    PUBLIC
        cxx_std_23
)
target_link_libraries(${target}
    PRIVATE
        sdk_hosting
        ${editorhost_PLATFORM_LIBS}
)

smtg_target_setup_universal_binary(${target})
if (APP_PROPERTIES)
    set_target_properties(${target}
        PROPERTIES
            ${APP_PROPERTIES}
    )
endif(APP_PROPERTIES)

