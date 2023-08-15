PLATFORMS = x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu x86_64-pc-windows-msvc aarch64-pc-windows-msvc x86_64-apple-darwin aarch64-apple-darwin

ARTIFACTS_DIR_LINUX := Linux
ARTIFACTS_DIR_MACOS := macOS
ARTIFACTS_DIR_WINDOWS := Windows
ARTIFACTS_DIR_DIST := dist
PROJECT_PATH := "flexible_inspect_py/Cargo.toml"

# Цель: all
# Описание: Собирает все указанные платформы
all-python: $(PLATFORMS)

$(PLATFORMS):
	@echo "Building $@..."
	@if [ "$@" = "x86_64-unknown-linux-gnu" -o "$@" = "aarch64-unknown-linux-gnu" ]; then \
		mkdir -p $(ARTIFACTS_DIR_LINUX); \
		maturin build --release --target $@ --zig -o $(ARTIFACTS_DIR_LINUX) --manifest-path $(PROJECT_PATH); \
	elif [ "$@" = "x86_64-pc-windows-msvc" -o "$@" = "aarch64-pc-windows-msvc" ]; then \
		mkdir -p $(ARTIFACTS_DIR_WINDOWS); \
		maturin build --release --target $@ --zig -o $(ARTIFACTS_DIR_WINDOWS) --manifest-path $(PROJECT_PATH); \
	else \
		mkdir -p $(ARTIFACTS_DIR_MACOS); \
		maturin build --release --target $@ --zig -o $(ARTIFACTS_DIR_MACOS) --manifest-path $(PROJECT_PATH); \
	fi

dist-python: $(PLATFORMS)

$(PLATFORMS):
	@echo "Building $@..."
	@if [ "$@" = "x86_64-unknown-linux-gnu" -o "$@" = "aarch64-unknown-linux-gnu" ]; then \
		mkdir -p $(ARTIFACTS_DIR_DIST); \
		maturin build --release --target $@ --zig -o $(ARTIFACTS_DIR_DIST) --manifest-path $(PROJECT_PATH); \
	elif [ "$@" = "x86_64-pc-windows-msvc" -o "$@" = "aarch64-pc-windows-msvc" ]; then \
		mkdir -p $(ARTIFACTS_DIR_DIST); \
		maturin build --release --target $@ --zig -o $(ARTIFACTS_DIR_DIST) --manifest-path $(PROJECT_PATH); \
	else \
		mkdir -p $(ARTIFACTS_DIR_DIST); \
		maturin build --release --target $@ --zig -o $(ARTIFACTS_DIR_DIST) --manifest-path $(PROJECT_PATH); \
	fi

# Шорткаты для каждой операционной системы

# Цель: linux-arm
# Описание: Собирает только для Linux ARM
linux-arm-python:
	@echo "Building $@..."
	mkdir -p $(ARTIFACTS_DIR_LINUX)
	maturin build --release --target aarch64-unknown-linux-gnu --zig -o $(ARTIFACTS_DIR_LINUX) --manifest-path $(PROJECT_PATH);

# Цель: linux-amd
# Описание: Собирает только для Linux AMD64
linux-amd-python:
	@echo "Building $@..."
	mkdir -p $(ARTIFACTS_DIR_LINUX)
	maturin build --release --target x86_64-unknown-linux-gnu --zig -o $(ARTIFACTS_DIR_LINUX) --manifest-path $(PROJECT_PATH);

# Цель: windows-amd
# Описание: Собирает только для Windows AMD64
windows-amd-python:
	@echo "Building $@..."
	mkdir -p $(ARTIFACTS_DIR_WINDOWS)
	maturin build --release --target x86_64-pc-windows-msvc --zig -o $(ARTIFACTS_DIR_WINDOWS) --manifest-path $(PROJECT_PATH);

# Цель: windows-arm
# Описание: Собирает только для Windows ARM64
windows-arm-python:
	@echo "Building $@..."
	mkdir -p $(ARTIFACTS_DIR_WINDOWS)
	maturin build --release --target aarch64-pc-windows-msvc --zig -o $(ARTIFACTS_DIR_WINDOWS) --manifest-path $(PROJECT_PATH);

# Цель: mac-amd
# Описание: Собирает только для macOS AMD64
mac-amd-python:
	@echo "Building $@..."
	mkdir -p $(ARTIFACTS_DIR_MACOS)
	maturin build --release --target x86_64-apple-darwin --zig -o $(ARTIFACTS_DIR_MACOS) --manifest-path $(PROJECT_PATH);

# Цель: mac-arm
# Описание: Собирает только для macOS ARM
mac-arm-python:
	@echo "Building $@..."
	mkdir -p $(ARTIFACTS_DIR_MACOS)
	maturin build --release --target aarch64-apple-darwin --zig -o $(ARTIFACTS_DIR_MACOS) --manifest-path $(PROJECT_PATH);


clean:
	rm -rf $(ARTIFACTS_DIR_LINUX)
	rm -rf $(ARTIFACTS_DIR_MACOS)
	rm -rf $(ARTIFACTS_DIR_WINDOWS)
	rm -rf $(ARTIFACTS_DIR_DIST)