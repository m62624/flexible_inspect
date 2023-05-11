.PHONY: all clean

PLATFORMS = x86_64-pc-windows-msvc aarch64-apple-darwin x86_64-unknown-linux-gnu
WHEEL_DIR = WHEELS

all: $(PLATFORMS)

$(PLATFORMS):
	maturin build -i python3 --release --target $@ --zig -o $(WHEEL_DIR)/$@

clean:
	rm -rf $(WHEEL_DIR)
