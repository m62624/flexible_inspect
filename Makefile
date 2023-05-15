PLATFORMS = x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu x86_64-pc-windows-msvc aarch64-pc-windows-msvc x86_64-apple-darwin aarch64-apple-darwin

ARTIFACTS_DIR_LINUX := Linux
ARTIFACTS_DIR_MACOS := macOS
ARTIFACTS_DIR_WINDOWS := Windows

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
clean:
	rm -rf $(ARTIFACTS_DIR_LINUX) $(ARTIFACTS_DIR_MACOS) $(ARTIFACTS_DIR_WINDOWS)
