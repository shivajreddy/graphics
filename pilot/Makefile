
#####################################
# Project: Graphics-Pilot
# Author: smpl
# OS: Linux
#####################################
# Compiler
CXX := g++
CXXFLAGS := -std=c++23 -Wall -Wextra -O2
DEBUGFLAGS := -g -DDEBUG
# Directories
BUILD_DIR := build
SRC_DIR := src
LIB_DIR := lib
# GLFW paths (using system-installed GLFW on Linux)
# GLAD paths
GLAD_PATH := $(LIB_DIR)/glad/linux
# I(include) & L(linking) flags
INCLUDES := -I$(GLAD_PATH)/include
LIBS := -lglfw -lGL -lX11 -lpthread -lXrandr -lXi -ldl
# Source files (automatically find all .cpp and .c files)
SOURCES := $(wildcard $(SRC_DIR)/*.cpp) $(wildcard $(SRC_DIR)/*.c)
# Target
TARGET := $(BUILD_DIR)/out
# Build targets
all: $(TARGET)
$(TARGET): $(SOURCES)
	@mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) $(INCLUDES) $(SOURCES) -o $(TARGET) $(LIBS)
	@echo Build complete!
# Debug build
debug: CXXFLAGS += $(DEBUGFLAGS)
debug: clean $(TARGET)
	@echo Debug complete!
run: clean all
	@./$(TARGET)
clean:
	@rm -rf $(BUILD_DIR)
	@echo Clean complete!
rebuild: clean all
# Show build configuration
info:
	@echo Target: $(TARGET)
	@echo Compiler: $(CXX)
	@echo Flags: $(CXXFLAGS)
	@echo GLFW Path: $(GLFW_PATH)
	@echo Sources: $(SOURCES)
# Help menu
help:
	@echo "Available targets:"
	@echo "  make          - Build the project (optimized)"
	@echo "  make debug    - Build with debug symbols"
	@echo "  make run      - Build and run the program"
	@echo "  make clean    - Remove build artifacts"
	@echo "  make rebuild  - Clean and build"
	@echo "  make info     - Show build configuration"
	@echo "  make help     - Show this help menu"
.PHONY: all debug run clean rebuild info help
