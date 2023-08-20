PLATFORMS = x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu x86_64-pc-windows-msvc aarch64-pc-windows-msvc x86_64-apple-darwin aarch64-apple-darwin
ARTIFACTS_DIR_LINUX := Linux
ARTIFACTS_DIR_MACOS := macOS
ARTIFACTS_DIR_WINDOWS := Windows
ARTIFACTS_DIR_DIST := dist
PROJECT_PATH_PY := "flexible_inspect_py/Cargo.toml"
# Цель: all
# Описание: Собирает все указанные платформы
all-python-without-dist: $(PLATFORMS)

$(PLATFORMS):
	@echo "Building $@..."
	@if [ "$@" = "x86_64-unknown-linux-gnu" -o "$@" = "aarch64-unknown-linux-gnu" ]; then \
		mkdir -p $(ARTIFACTS_DIR_LINUX); \
		maturin build --release --target $@ --zig -o $(ARTIFACTS_DIR_LINUX) --manifest-path $(PROJECT_PATH_PY); \
	elif [ "$@" = "x86_64-pc-windows-msvc" -o "$@" = "aarch64-pc-windows-msvc" ]; then \
		mkdir -p $(ARTIFACTS_DIR_WINDOWS); \
		maturin build --release --target $@ --zig -o $(ARTIFACTS_DIR_WINDOWS) --manifest-path $(PROJECT_PATH_PY); \
	else \
		mkdir -p $(ARTIFACTS_DIR_MACOS); \
		maturin build --release --target $@ --zig -o $(ARTIFACTS_DIR_MACOS) --manifest-path $(PROJECT_PATH_PY); \
	fi

all-python: $(PLATFORMS)

$(PLATFORMS):
	@echo "Building $@..."
	@if [ "$@" = "x86_64-unknown-linux-gnu" -o "$@" = "aarch64-unknown-linux-gnu" ]; then \
		mkdir -p $(ARTIFACTS_DIR_DIST); \
		maturin build --release --target $@ --zig -o $(ARTIFACTS_DIR_DIST) --manifest-path $(PROJECT_PATH_PY); \
	elif [ "$@" = "x86_64-pc-windows-msvc" -o "$@" = "aarch64-pc-windows-msvc" ]; then \
		mkdir -p $(ARTIFACTS_DIR_DIST); \
		maturin build --release --target $@ --zig -o $(ARTIFACTS_DIR_DIST) --manifest-path $(PROJECT_PATH_PY); \
	else \
		mkdir -p $(ARTIFACTS_DIR_DIST); \
		maturin build --release --target $@ --zig -o $(ARTIFACTS_DIR_DIST) --manifest-path $(PROJECT_PATH_PY); \
	fi

clean:
	rm -rf $(ARTIFACTS_DIR_LINUX)
	rm -rf $(ARTIFACTS_DIR_MACOS)
	rm -rf $(ARTIFACTS_DIR_WINDOWS)
	rm -rf $(ARTIFACTS_DIR_DIST)