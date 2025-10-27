# ===========================================
# HyprDM Greeterd — Final Arch Qt5 Makefile
# ===========================================

CXX       := g++
MOC       := moc-qt5
MODE ?= release

RUST_LIB_DIR ?= ../target/$(MODE)
RUST_LIB := $(RUST_LIB_DIR)/libhyprdmbackend.a

# Auto-detect Qt include root (Arch vs Debian)
QT_BASE := $(shell [ -d /usr/include/qt5 ] && echo /usr/include/qt5 || echo /usr/include/qt)

QT_INCLUDE := \
    -I$(QT_BASE) \
    -I$(QT_BASE)/QtCore \
    -I$(QT_BASE)/QtGui \
    -I$(QT_BASE)/QtQml \
    -I$(QT_BASE)/QtQuick \
    -I$(QT_BASE)/QtQuickControls2 \
    -I$(QT_BASE)/QtWidgets

CXXFLAGS := -std=c++17 -Wall -Wextra -fPIC -O3 -pipe \
    $(QT_INCLUDE) -Ibackend -Ibackend/adapters

LDFLAGS := \
    -L/usr/lib \
    -lQt5Core -lQt5Gui -lQt5Qml -lQt5Quick -lQt5QuickControls2 -lQt5Widgets \
    -Wl,--whole-archive $(RUST_LIB) -Wl,--no-whole-archive \
    -linput -ludev -lgbm -lpam -lsystemd -ldbus-1 -lxkbcommon \
    -lpthread -ldl -lm -lrt

SRC := $(wildcard backend/*.cpp *.cpp)
HDR := $(wildcard backend/*.h backend/*.hpp)
MOC_CPP := $(HDR:%.h=%.moc.cpp) $(HDR:%.hpp=%.moc.cpp)
OBJ := $(SRC:.cpp=.o) $(MOC_CPP:.cpp=.o)
TARGET := hyprdmgreeterd

all: $(TARGET)

# Generate MOC files with proper include dirs
%.moc.cpp: %.h
	@echo "[MOC] $<"
	@$(MOC) $(QT_INCLUDE) $< -o $@

%.moc.cpp: %.hpp
	@echo "[MOC] $<"
	@$(MOC) $(QT_INCLUDE) $< -o $@

# Compile C++ sources
%.o: %.cpp
	@echo "[C++] $<"
	@$(CXX) $(CXXFLAGS) -c $< -o $@

%.o: %.moc.cpp
	@echo "[C++] (moc) $<"
	@$(CXX) $(CXXFLAGS) -c $< -o $@

# Link final binary (include CXXFLAGS to ensure all -I paths)
$(TARGET): $(OBJ)
	@echo "[LD] $@"
	@$(CXX) $(CXXFLAGS) -o $@ $^ $(LDFLAGS)
	@echo "✅ Build complete: $(TARGET)"

install: $(TARGET)
	sudo install -Dm755 $(TARGET) /usr/bin/$(TARGET)
	@echo "✅ Installed to /usr/bin/$(TARGET)"

clean:
	rm -f $(OBJ) $(MOC_CPP) $(TARGET)
	@echo "🧹 Clean complete."

# ========================
# Extra helper targets
# ========================

build:
	@$(MAKE) -j$$(nproc)

debug:
	@echo "🔍 Building in debug mode with debug-symlink enabled..."
	@mkdir -p ../target/debug
	@ln -sf ../target/debug ../target/debug-symlink
	@$(MAKE) MODE=debug -j$$(nproc)

rebuild:
	@$(MAKE) clean
	@$(MAKE) -j$$(nproc)

.PHONY: all clean install build debug rebuild
