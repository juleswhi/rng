PROJECT_NAME = rng
TARGET_DIR = target/release

# Rust compiler and flags
CARGO = cargo
CARGO_FLAGS = --release

# Installation path
INSTALL_DIR = /usr/local/bin

# Default target
all: build

# Build the project
build:
	@echo "Building $(PROJECT_NAME)..."
	$(CARGO) build $(CARGO_FLAGS)

install: build
	@echo "Installing $(PROJECT_NAME) to $(INSTALL_DIR)..."
	sudo install -m 755 $(TARGET_DIR)/$(PROJECT_NAME) $(INSTALL_DIR)/$(PROJECT_NAME)
	@echo "Installed."

uninstall:
	@echo "Removing $(PROJECT_NAME) from $(INSTALL_DIR)..."
	sudo rm -f $(INSTALL_DIR)/$(PROJECT_NAME)

.PHONY: all build run clean install uninstall test format lint
