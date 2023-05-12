.PHONY: all clean

PLATFORMS = x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu x86_64-pc-windows-msvc aarch64-pc-windows-msvc x86_64-apple-darwin aarch64-apple-darwin
WHEEL_DIR = WHEELS

all: $(PLATFORMS)

$(PLATFORMS):
	maturin build -i python3 --release --target $@ --zig -o $(WHEEL_DIR)/$@

clean:
	rm -rf $(WHEEL_DIR)
