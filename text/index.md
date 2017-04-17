Title: Visualizing Binary File Formats
Author: Brad Erickson

Electronic files are made up of structured data including headers, delimiters
and keyframes. This consistency can create patterns.

A color palette is created from the first byte of the SHA256 hash of the file.

* 0x00 - Black
* Printable Characters - Primary hue is the byte scaled to the range (0->360)
* Control Characters + Space - The complementary color of the primary hue.
* Extended Characters - A darkened version of the primary hue.

<a href="https://github.com/eosrei/file-vis/"><img style="position: fixed; top: 0; right: 0; border: 0;" src="https://camo.githubusercontent.com/e7bbb0521b397edbd5fe43e7f760759336b5e05f/68747470733a2f2f73332e616d617a6f6e6177732e636f6d2f6769746875622f726962626f6e732f666f726b6d655f72696768745f677265656e5f3030373230302e706e67" alt="Fork me on GitHub" data-canonical-src="https://s3.amazonaws.com/github/ribbons/forkme_right_green_007200.png"></a>
