# -------------------- Config --------------------
CXX := g++
CXXFLAGS := -std=c++17 -Wall \
    $(shell pkg-config --cflags Qt6Core Qt6Gui Qt6Qml Qt6Quick Qt6QuickControls2 Qt6QuickLayouts)

MAIN_CPP := main.cpp
MAIN_OBJ := $(MAIN_CPP:.cpp=.o)
EXEC := hyprdm

QML_DIR := qml

# -------------------- Targets --------------------
all: backend $(EXEC)

backend:
	@bash ./autogen.sh
	sudo cp $$HYPRDM_LIB_DIR/libhyprdmbackend.so /usr/lib
	sudo ldconfig

# Uygulama hedefi
$(EXEC): $(MAIN_OBJ)
	$(CXX) $(MAIN_OBJ) \
    $(shell pkg-config --libs Qt6Core Qt6Gui Qt6Qml Qt6Quick Qt6QuickControls2 Qt6QuickLayouts) \
    -L$(HYPRDM_LIB_DIR) -lhyprdmbackend -o $@

# C++ derleme kuralı
%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Temizlik
clean:
	rm -f $(MAIN_OBJ) $(EXEC)
	cargo clean

# Çalıştırma
run: all
	./$(EXEC) -qmlimport $(QML_DIR)

.PHONY: all backend clean run
