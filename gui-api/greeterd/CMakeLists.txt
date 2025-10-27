cmake_minimum_required(VERSION 3.16)
project(hyprdmgreeterd LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
endif()

if(CMAKE_BUILD_TYPE STREQUAL "Release")
    add_compile_options(-O3 -flto -fno-plt -Wall -Wextra -fPIC)
    add_link_options(-O3 -flto)
else()
    add_compile_options(-O0 -g3 -Wall -Wextra -fPIC)
endif()

find_program(CCACHE_PROGRAM ccache)
if(CCACHE_PROGRAM)
    set(CMAKE_CXX_COMPILER_LAUNCHER ${CCACHE_PROGRAM})
endif()

find_package(Qt5 5.15 REQUIRED COMPONENTS Core Gui Qml Quick QuickControls2)

if(DEFINED ENV{HYPRDM_LIB_DIR})
    set(RUST_LIB_DIR $ENV{HYPRDM_LIB_DIR})
else()
    set(RUST_LIB_DIR "${CMAKE_SOURCE_DIR}/../target/release")
endif()

set(RUST_STATIC_LIB "${RUST_LIB_DIR}/libhyprdmbackend.a")

if(NOT EXISTS ${RUST_STATIC_LIB})
    execute_process(
        COMMAND cargo build --release
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/..
        RESULT_VARIABLE RUST_BUILD_RESULT
    )
    if(NOT RUST_BUILD_RESULT EQUAL 0)
        message(FATAL_ERROR "Rust build failed! Check cargo output.")
    endif()
endif()

file(GLOB SRC_FILES
    "${CMAKE_SOURCE_DIR}/*.cpp"
    "${CMAKE_SOURCE_DIR}/backend/*.cpp"
)
file(GLOB HDR_FILES "${CMAKE_SOURCE_DIR}/backend/*.h")

qt5_wrap_cpp(MOC_SRC ${HDR_FILES})

add_executable(${PROJECT_NAME}
    ${SRC_FILES}
    ${MOC_SRC}
)

target_include_directories(${PROJECT_NAME} PRIVATE
    ${Qt5Core_INCLUDE_DIRS}
    ${Qt5Gui_INCLUDE_DIRS}
    ${Qt5Qml_INCLUDE_DIRS}
    ${Qt5Quick_INCLUDE_DIRS}
    ${Qt5QuickControls2_INCLUDE_DIRS}
    ${CMAKE_SOURCE_DIR}/backend
)

target_link_libraries(${PROJECT_NAME}
    Qt5::Core
    Qt5::Gui
    Qt5::Qml
    Qt5::Quick
    Qt5::QuickControls2
    -Wl,--whole-archive ${RUST_STATIC_LIB} -Wl,--no-whole-archive
    input
    udev
    gbm
    pam
    systemd
    dbus-1
    xkbcommon
    pthread
    dl
    m
    rt
)

set_target_properties(${PROJECT_NAME} PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
)

install(TARGETS ${PROJECT_NAME} RUNTIME DESTINATION /usr/bin)
