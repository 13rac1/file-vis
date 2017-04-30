# File Visualizer

Visualizing binary file formats

[![Example images](example.png?raw=true)](https://eosrei.github.io/file-vis/)

## Intro

Electronic files are made up of structured data including headers, delimiters
and keyframes. This consistency can create patterns. Efficiently compressed
data removes all patterns and will appear as noise.

* Primary Goal: Learn some Rust.
* Secondary Goal: Make something interesting.

## File Types

* Text
* Audio
* Images
* Video
* 3D Models

## Build/Development Requirements

* Rust/Cargo - Compile file-vis.
* Imagemagick - Resize images.
* Python3 - Sigal, static site generator.
* Make.

```
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
make
```

## Other Projects

Far more complex projects than this exist if you want to do real work:

* https://github.com/wapiflapi/veles
* http://binvis.io/
