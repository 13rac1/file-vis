SRCFILES   := $(wildcard files/*)
VISFILES   := $(patsubst files/%, build/png/%.png, $(SRCFILES))
SIZEDFILES := $(patsubst files/%, build/images/%.png, $(SRCFILES))

FILEVIS := target/release/file-vis

.PHONY: all images build-all clean clean-all dist

all: build-all

$(FILEVIS): src/main.rs
	cargo build --release

build/png/%.png: files/%| build/png
	$(FILEVIS) "$<" -o "$@"

build/images/%.png: build/png/%.png | build/images
	convert "$<" -sample 600x600 "$@"

build:
	mkdir build

build/png: | build
	mkdir build/png

build/images: | build
	mkdir build/images

dist: sigal.conf.py $(SIZEDFILES)
	$(FILEVIS) src/main.rs -o build/png/file-vis-main.rs.png
	convert build/png/file-vis-main.rs.png -sample 600x600 build/images/0file-vis-main.rs.png
	$(FILEVIS) $(FILEVIS) -o build/png/file-vis-compiled.png
	convert build/png/file-vis-compiled.png -sample 600x600 build/images/0file-vis-compiled.png

	cp src-site/index.md build/images/
	sigal build
	cp src-site/circle.yml dist/

build-all: $(FILEVIS) dist

deploy: build-all
	./deploy.sh

clean:
	rm -rf build
	rm -rf dist

clean-all: clean
	cargo clean
