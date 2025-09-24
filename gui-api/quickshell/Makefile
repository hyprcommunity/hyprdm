# -------------------- Config --------------------
CXX := g++
CXXFLAGS := -std=c++17 -Wall -I/usr/include/qt -I/usr/include/qt/QtQuick -I/usr/include/qt/QtQuickControls2 -I/usr/include/qt/QtQuickLayouts

MAIN_CPP := main.cpp backend.cpp
MAIN_OBJ := $(MAIN_CPP:.cpp=.o)
EXEC := hyprdm

QML_DIR := qml

# LDFLAGS artık autogen.sh tarafından belirlenen HYPRDM_LIB_DIR'ı kullanacak
LDFLAGS := -lQt5Quick -lQt5QuickControls2 -lQt5Qml -lQt5Core -lQt5Gui -lQt5Widgets -fPIC

# -------------------- Targets --------------------
all: backend $(EXEC)

backend: autogen.sh
	./autogen.sh

$(EXEC): $(MAIN_OBJ)
	$(CXX) $(MAIN_OBJ) -L$$HYPRDM_LIB_DIR -lhyprdm_backend $(LDFLAGS) -o $@

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -f $(MAIN_OBJ) $(EXEC)
	cargo clean

run: all
	./$(EXEC) -qmlimport $(QML_DIR)

.PHONY: all backend clean run
