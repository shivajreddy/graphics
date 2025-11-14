#####################################
# Project: Graphics-Pilot
# Author: smpl
# OS: macOS
#####################################

# Compiler
CXX := g++
CXXFLAGS := -std=c++23 -Wall -Wextra -O2
DEBUGFLAGS := -g -DDEBUG

# Directories
BUILD_DIR := build
SRC_DIR := src
LIB_DIR := lib

# GLFW paths
GLFW_PATH := $(LIB_DIR)/GLFW/macos

# GLAD paths
GLAD_PATH := $(LIB_DIR)/glad/macos

# I(include) & L(linking) flags
INCLUDES := -I$(GLFW_PATH)/include -I$(GLAD_PATH)/include
LIBS := -L$(GLFW_PATH)/lib -lglfw3 -framework OpenGL -framework Cocoa -framework IOKit -framework CoreVideo


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
	@echo Debug build complete!

run: clean all
    # $(TARGET)
	./$(TARGET)

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
