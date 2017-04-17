Title: Visualizing Binary File Formats
Author: Brad Erickson

Electronic files are made up of structured data including headers, delimiters
and keyframes. This consistency can create patterns.

A color palette is created from the first byte of the SHA256 hash of the file.

* 0x00 - Black
* Printable Characters - Primary hue is the byte scaled to the range (0->360)
* Control Characters + Space - The complementary color of the primary hue.
* Extended Characters - A darkened version of the primary hue.
