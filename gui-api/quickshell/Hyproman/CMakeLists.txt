cmake_minimum_required(VERSION 3.16)
project(hyprdm CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# -------------------- Qt --------------------
find_package(Qt6 REQUIRED COMPONENTS Core Gui Qml Quick QuickControls2)

# -------------------- Source files --------------------
set(SRC_FILES
    main.cpp
)

# -------------------- Rust backend --------------------
# Rust library yolu environment variable ile
if(NOT DEFINED ENV{HYPRDM_LIB_DIR})
    message(FATAL_ERROR "HYPRDM_LIB_DIR environment variable not set. Run autogen.sh first.")
endif()
set(RUST_LIB_DIR $ENV{HYPRDM_LIB_DIR})
set(RUST_LIB ${RUST_LIB_DIR}/libhyprdmbackend.a)

# -------------------- Executable --------------------
add_executable(${PROJECT_NAME} ${SRC_FILES})

# Link Qt
target_link_libraries(${PROJECT_NAME} PRIVATE
    Qt6::Core
    Qt6::Gui
    Qt6::Qml
    Qt6::Quick
    Qt6::QuickControls2
)

# Link Rust static library
target_link_libraries(${PROJECT_NAME} PRIVATE ${RUST_LIB})

# Include directories for Qt headers
target_include_directories(${PROJECT_NAME} PRIVATE
    /usr/include/qt6
    /usr/include/qt6/QtCore
    /usr/include/qt6/QtGui
    /usr/include/qt6/QtQml
    /usr/include/qt6/QtQuick
    /usr/include/qt6/QtQuickControls2
    /usr/include/qt6/QtQuickLayouts
)
