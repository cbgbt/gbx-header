# gbx-header

[![](https://docs.rs/gbx-header/badge.svg)](https://docs.rs/gbx-header)

Partial parser of .Gbx files as used in TrackMania Nations Forever.
The format of these files is not publicly documented, so this parser is made by reverse-engineering examples.

## gbx_header

Library to read the header of .Gbx files as used by TrackMania Nations Forever.
Parsing them into an easy to use [GBX](https://docs.rs/gbx-header/latest/gbx_header/gbx/struct.GBX.html) datastructure.

### TODO

- Parse map data

## gbx-info

Binary to dump information about a .Gbx file in a human readable format.

Example output:

```
GBX Info Dump (Size=10985B)
From file=examples/mtib-1-dirty-jumps.Challenge.Gbx
Header Infos
============
Map is Challenge/Race made in TMc6/2.11.16
UID: Zh7gt1dJfZbmCMhmqfnGC3EvBO3
Name: mtib-1-dirty-jumps
Author: mtibb
Setting: Stadium/Sunrise
Number of laps: 0
Price: 779
Times: Bronze: 46000, Silver: 37000, Gold: 33000, Authortime: 30630, Authorscore: 30630
Dependencies[0]: []
```

The command line flag `-t` allows exporting the thumbnail to jpg, note that the stored jpeg data is upside-down.