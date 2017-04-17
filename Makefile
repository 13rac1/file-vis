SRCFILES  := $(wildcard files/*)
VISFILES  := $(patsubst files/%, build/png/%.png, $(SRCFILES))
SIZEDFILES:= $(patsubst files/%, build/images/%.png, $(SRCFILES))

.PHONY: all images build-all clean clean-all dist

all: build-all

target/release/file-vis: src/main.rs
	cargo build --release

build/png/%.png: files/%| build/png
	target/release/file-vis "$<" -o "$@"

build/images/%.png: build/png/%.png | build/images
	convert "$<" -sample 600x600 "$@"

build:
	mkdir build

build/png: | build
	mkdir build/png

build/images: | build
	mkdir build/images

dist: sigal.conf.py $(SIZEDFILES)
	cp text/* build/images/
	target/release/file-vis src/main.rs -o build/png/file-vis.rs.png
	convert build/png/file-vis.rs.png -sample 600x600 build/images/0file-vis.rs.png
	sigal build

build-all: target/release/file-vis dist

deploy: build-all
	./deploy.sh

clean:
	rm -rf build
	rm -rf dist

clean-all: clean
	cargo clean
