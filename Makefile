SRCFILES  := $(wildcard files/*)
VISFILES  := $(patsubst files/%, build/%.png, $(SRCFILES))
SIZEDFILES:= $(patsubst files/%, build/images/%.png, $(SRCFILES))

.PHONY: all images build-all clean clean-all

all: build-all

target/release/file-vis: src/main.rs
	cargo build --release

build/%.png: files/% | build
	target/release/file-vis "$<" -o "$@"

build/images/%.png: build/%.png | build/images
	convert "$<" -sample 600x600 "$@"

build:
	mkdir build

build/images: | build
	mkdir build/images

build-all: target/release/file-vis $(SIZEDFILES)

clean:
	rm -rf build

clean-all: clean
	cargo clean
