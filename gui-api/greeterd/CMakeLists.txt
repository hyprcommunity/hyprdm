cmake_minimum_required(VERSION 3.20)
project(hyprdmgreeterd LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Qt6 paketleri
find_package(Qt6 REQUIRED COMPONENTS Core Gui Qml Quick QuickControls2 QuickLayouts)

# Rust kütüphane dizini (autogen.sh tarafından ayarlanmış)
if(NOT DEFINED ENV{HYPRDM_LIB_DIR})
    message(FATAL_ERROR "HYPRDM_LIB_DIR environment variable not set. Run autogen.sh first.")
endif()
set(RUST_LIB_DIR $ENV{HYPRDM_LIB_DIR})

# kaynak dosyalar
file(GLOB SRC_FILES "*.cpp" "backend/*.cpp")
file(GLOB MOC_HDRS "backend/*.h")

# otomatik MOC
qt6_wrap_cpp(MOC_SRC ${MOC_HDRS})

add_executable(${PROJECT_NAME} ${SRC_FILES} ${MOC_SRC})

# include dizinleri
target_include_directories(${PROJECT_NAME} PRIVATE ${CMAKE_CURRENT_SOURCE_DIR} "${RUST_LIB_DIR}")

# Qt linkleme
target_link_libraries(${PROJECT_NAME} PRIVATE Qt6::Core Qt6::Gui Qt6::Qml Qt6::Quick Qt6::QuickControls2 Qt6::QuickLayouts)

# Rust kütüphanesini linkleme (.so kullanmak için)
target_link_libraries(${PROJECT_NAME} PRIVATE "${RUST_LIB_DIR}/libhyprdmbackend.so")

# Install hedefi
install(TARGETS ${PROJECT_NAME}
        RUNTIME DESTINATION /usr/bin)

# Opsiyonel: Rust .so dosyasını da /usr/lib altına kopyala
install(FILES "${RUST_LIB_DIR}/libhyprdmbackend.so" DESTINATION /usr/lib)
