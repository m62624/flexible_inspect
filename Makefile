PLATFORMS = x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu x86_64-pc-windows-msvc aarch64-pc-windows-msvc x86_64-apple-darwin aarch64-apple-darwin

ARTIFACTS_DIR_LINUX := Linux
ARTIFACTS_DIR_MACOS := macOS
ARTIFACTS_DIR_WINDOWS := Windows
ARTIFACTS_DIR_DIST := dist

# Цель: all
# Описание: Собирает все указанные платформы
all: $(PLATFORMS)

$(PLATFORMS):
	@echo "Building $@..."
	@if [ "$@" = "x86_64-unknown-linux-gnu" -o "$@" = "aarch64-unknown-linux-gnu" ]; then \
		mkdir -p $(ARTIFACTS_DIR_LINUX); \
		maturin build -i python3 --release --target $@ --zig -o $(ARTIFACTS_DIR_LINUX); \
	elif [ "$@" = "x86_64-pc-windows-msvc" -o "$@" = "aarch64-pc-windows-msvc" ]; then \
		mkdir -p $(ARTIFACTS_DIR_WINDOWS); \
		maturin build -i python3 --release --target $@ --zig -o $(ARTIFACTS_DIR_WINDOWS); \
	else \
		mkdir -p $(ARTIFACTS_DIR_MACOS); \
		maturin build -i python3 --release --target $@ --zig -o $(ARTIFACTS_DIR_MACOS); \
	fi

dist: $(PLATFORMS)

$(PLATFORMS):
	@echo "Building $@..."
	@if [ "$@" = "x86_64-unknown-linux-gnu" -o "$@" = "aarch64-unknown-linux-gnu" ]; then \
		mkdir -p $(ARTIFACTS_DIR_DIST); \
		maturin build -i python3 --release --target $@ --zig -o $(ARTIFACTS_DIR_DIST); \
	elif [ "$@" = "x86_64-pc-windows-msvc" -o "$@" = "aarch64-pc-windows-msvc" ]; then \
		mkdir -p $(ARTIFACTS_DIR_DIST); \
		maturin build -i python3 --release --target $@ --zig -o $(ARTIFACTS_DIR_DIST); \
	else \
		mkdir -p $(ARTIFACTS_DIR_DIST); \
		maturin build -i python3 --release --target $@ --zig -o $(ARTIFACTS_DIR_DIST); \
	fi

# Шорткаты для каждой операционной системы

# Цель: linux-arm
# Описание: Собирает только для Linux ARM
linux-arm:
	@echo "Building $@..."
	mkdir -p $(ARTIFACTS_DIR_LINUX)
	maturin build -i python3 --release --target aarch64-unknown-linux-gnu --zig -o $(ARTIFACTS_DIR_LINUX)

# Цель: linux-amd
# Описание: Собирает только для Linux AMD64
linux-amd:
	@echo "Building $@..."
	mkdir -p $(ARTIFACTS_DIR_LINUX)
	maturin build -i python3 --release --target x86_64-unknown-linux-gnu --zig -o $(ARTIFACTS_DIR_LINUX)

# Цель: windows-amd
# Описание: Собирает только для Windows AMD64
windows-amd:
	@echo "Building $@..."
	mkdir -p $(ARTIFACTS_DIR_WINDOWS)
	maturin build -i python3 --release --target x86_64-pc-windows-msvc --zig -o $(ARTIFACTS_DIR_WINDOWS)

# Цель: windows-arm
# Описание: Собирает только для Windows ARM64
windows-arm:
	@echo "Building $@..."
	mkdir -p $(ARTIFACTS_DIR_WINDOWS)
	maturin build -i python3 --release --target aarch64-pc-windows-msvc --zig -o $(ARTIFACTS_DIR_WINDOWS)

# Цель: mac-amd
# Описание: Собирает только для macOS AMD64
mac-amd:
	@echo "Building $@..."
	mkdir -p $(ARTIFACTS_DIR_MACOS)
	maturin build -i python3 --release --target x86_64-apple-darwin --zig -o $(ARTIFACTS_DIR_MACOS)

# Цель: mac-arm
# Описание: Собирает только для macOS ARM
mac-arm:
	@echo "Building $@..."
	mkdir -p $(ARTIFACTS_DIR_MACOS)
	maturin build -i python3 --release --target aarch64-apple-darwin --zig -o $(ARTIFACTS_DIR_MACOS)

# Цель: clean
# Описание: Очищает созданные артефакты
clean:
	rm -rf $(ARTIFACTS_DIR_LINUX) $(ARTIFACTS_DIR_MACOS) $(ARTIFACTS_DIR_WINDOWS)