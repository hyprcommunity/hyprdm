cmake_minimum_required(VERSION 3.16)
project(hyprdmgreeterd CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Qt5 path (Arch Linux default)
set(Qt5Core_DIR /usr/lib/cmake/Qt5Core)
set(Qt5Gui_DIR /usr/lib/cmake/Qt5Gui)
set(Qt5Qml_DIR /usr/lib/cmake/Qt5Qml)
set(Qt5Quick_DIR /usr/lib/cmake/Qt5Quick)
set(Qt5QuickControls2_DIR /usr/lib/cmake/Qt5QuickControls2)

find_package(Qt5Core REQUIRED)
find_package(Qt5Gui REQUIRED)
find_package(Qt5Qml REQUIRED)
find_package(Qt5Quick REQUIRED)
find_package(Qt5QuickControls2 REQUIRED)

# Rust backend library dizini
# Eğer ortam değişkeni varsa onu kullan, yoksa default ../target/release
if(DEFINED ENV{HYPRDM_LIB_DIR})
    set(RUST_LIB_DIR $ENV{HYPRDM_LIB_DIR})
else()
    set(RUST_LIB_DIR "${CMAKE_SOURCE_DIR}/../target/release")
endif()

# Include directories
include_directories(
    ${Qt5Core_INCLUDE_DIRS}
    ${Qt5Gui_INCLUDE_DIRS}
    ${Qt5Qml_INCLUDE_DIRS}
    ${Qt5Quick_INCLUDE_DIRS}
    ${Qt5QuickControls2_INCLUDE_DIRS}
)

# Source files
file(GLOB SRC_FILES
    "*.cpp"
    "backend/*.cpp"
)

file(GLOB MOC_HEADERS
    "backend/*.h"
)

# MOC generation
qt5_wrap_cpp(MOC_SRC ${MOC_HEADERS})

# Executable
add_executable(${PROJECT_NAME} ${SRC_FILES} ${MOC_SRC})

# Link libraries
target_link_libraries(${PROJECT_NAME}
    Qt5::Core
    Qt5::Gui
    Qt5::Qml
    Qt5::Quick
    Qt5::QuickControls2
    ${RUST_LIB_DIR}/libhyprdmbackend.a
)
