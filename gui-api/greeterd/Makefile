# C++ compiler
CXX := g++

# Qt5 include ve library dizinleri
QT5_INCLUDE_DIR := /usr/include/qt
QT5_LIB_DIR := /usr/lib/qt

# Rust library dizini
RUST_LIB_DIR ?= ../target/release

# C++ flags
CXXFLAGS := -std=c++17 -Wall \
        -I$(QT5_INCLUDE_DIR) \
        -I$(QT5_INCLUDE_DIR)/QtCore \
        -I$(QT5_INCLUDE_DIR)/QtGui \
        -I$(QT5_INCLUDE_DIR)/QtQml \
        -I$(QT5_INCLUDE_DIR)/QtQuick \
        -I$(QT5_INCLUDE_DIR)/QtQuickControls2

LDFLAGS := -L$(QT5_LIB_DIR) \
           -lQt5Core -lQt5Gui -lQt5Qml -lQt5Quick -lQt5QuickControls2 \
           -Wl,--whole-archive $(RUST_LIB_DIR)/libhyprdmbackend.a -Wl,--no-whole-archive \
           -lpthread -ldl -lm -lrt -lsystemd

# Qt moc tool
MOC := moc-qt5

# Source files
SRC := $(wildcard *.cpp) $(wildcard backend/*.cpp)
MOC_HDRS := $(wildcard backend/*.h)
MOC_SRC := $(MOC_HDRS:.h=.moc.cpp)
OBJ := $(SRC:.cpp=.o) $(MOC_SRC:.cpp=.o)

# Target executable
TARGET := hyprdmgreeterd

# Default build
all: $(TARGET)

# Link target
$(TARGET): $(OBJ)
	$(CXX) $(CXXFLAGS) -o $@ $^ $(LDFLAGS)

# Compile C++ sources
%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Generate moc files from headers
%.moc.cpp: %.h
	$(MOC) $< -o $@

# Install executable
install: $(TARGET)
	sudo cp $(TARGET) /usr/bin/
	sudo chmod +x /usr/bin/$(TARGET)

# Clean build artifacts
clean:
	rm -f $(OBJ) $(MOC_SRC) $(TARGET)
