CXX := g++
CXXFLAGS := -std=c++17 -Wall $(shell pkg-config --cflags Qt6Core Qt6Gui Qt6Qml Qt6Quick Qt6QuickControls2 Qt6QuickLayouts)
LDFLAGS := $(shell pkg-config --libs Qt6Core Qt6Gui Qt6Qml Qt6Quick Qt6QuickControls2 Qt6QuickLayouts)

# Rust kütüphane dizini, autogen.sh ile set edilmiş
RUST_LIB_DIR ?= $(HYPRDM_LIB_DIR)

# kaynak dosyalar
SRC := $(wildcard *.cpp) $(wildcard backend/*.cpp)
MOC_HDRS := $(wildcard backend/*.h)
MOC_SRC := $(MOC_HDRS:.h=.moc.cpp)

# objeler
OBJ := $(SRC:.cpp=.o) $(MOC_SRC:.cpp=.o)
TARGET := hyprdmgreeterd

all: $(TARGET)

# main target
$(TARGET): $(OBJ)
	$(CXX) $(CXXFLAGS) -o $@ $^ $(LDFLAGS) -L$(RUST_LIB_DIR) -lhyprdmbackend

# normal cpp compilation
%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

# generate moc files from headers
%.moc.cpp: %.h
	moc $< -o $@

install: $(TARGET)
	sudo cp $(TARGET) /usr/bin/
	sudo chmod +x /usr/bin/$(TARGET)

# temizleme
clean:
	rm -f $(OBJ) $(MOC_SRC) $(TARGET)
