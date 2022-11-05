# file-format

![Build](https://img.shields.io/github/workflow/status/mmalecot/file-format/CI)
[![Crates.io](https://img.shields.io/crates/v/file-format.svg)](https://crates.io/crates/file-format)
[![Docs](https://docs.rs/file-format/badge.svg)](https://docs.rs/file-format)
![Rust](https://img.shields.io/badge/rust-1.60+-blueviolet.svg?logo=rust)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)

File format library for Rust.

This crate is for determining binary-based file formats.

## Examples

Determines from a file:

```rust
use file_format::FileFormat;

let format = FileFormat::from_file("fixtures/application/sample.zip").unwrap();
assert_eq!(format, FileFormat::Zip);
assert_eq!(format.name(), "ZIP");
assert_eq!(format.media_type(), "application/zip");
assert_eq!(format.extension(), "zip");
```

Determines from bytes:

```rust
use file_format::FileFormat;

let format = FileFormat::from_bytes(&[0xFF, 0xD8, 0xFF, 0xEE]);
assert_eq!(format, FileFormat::JointPhotographicExpertsGroup);
assert_eq!(format.name(), "Joint Photographic Experts Group");
assert_eq!(format.media_type(), "image/jpeg");
assert_eq!(format.extension(), "jpg");
```

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
file-format = "0.8"
```

If you want to enable Zip-based file formats:

```toml
[dependencies]
file-format = { version = "0.8", features = ["zip"] }
```

## Supported formats

- 3D Manufacturing Format - `3mf`
- 3rd Generation Partnership Project - `3gp`
- 3rd Generation Partnership Project 2 - `3g2`
- 7-Zip - `7z`
- ALZ - `alz`
- ANI - `ani`
- AV1 Image File Format - `avif`
- AV1 Image File Format Sequence - `avifs`
- Adaptive Multi-Rate - `amr`
- Adobe Flash Player Audio - `f4a`
- Adobe Flash Player Audiobook - `f4b`
- Adobe Flash Player Protected Video - `f4p`
- Adobe Flash Player Video - `f4v`
- Adobe InDesign Document - `indd`
- Adobe Photoshop Document - `psd`
- Advanced Audio Coding - `aac`
- Android Binary XML - `xml`
- Android Compiled Resources - `arsc`
- Android Package - `apk`
- Animated Portable Network Graphics - `apng`
- Apache Arrow Columnar - `arrow`
- Apple Disk Image - `dmg`
- Apple Icon Image - `icns`
- Apple QuickTime - `mov`
- Apple iTunes Audio - `m4a`
- Apple iTunes Audiobook - `m4b`
- Apple iTunes Protected Audio - `m4p`
- Apple iTunes Video - `m4v`
- Arbitrary Binary Data - `bin`
- Archived by Robert Jung - `arj`
- Au - `au`
- Audio Codec 3 - `ac3`
- Audio Interchange File Format - `aiff`
- Audio Video Interleave - `avi`
- Better Portable Graphics - `bpg`
- Blender - `blend`
- CUR - `cur`
- Cabinet - `cab`
- Canon Raw 2 - `cr2`
- Canon Raw 3 - `cr3`
- Cineon - `cin`
- Compound File Binary - `cfb`
- Dalvik Executable - `dex`
- Debian Binary Package - `deb`
- Design Web Format XPS - `dwfx`
- Digital Imaging and Communications in Medicine - `dcm`
- Digital Picture Exchange - `dpx`
- Electronic Publication - `epub`
- Embedded OpenType - `eot`
- Executable and Linkable Format - `elf`
- Experimental Computing Facility - `xcf`
- Extensible Archive - `xar`
- FastTracker 2 Extended Module - `xm`
- Flash Video - `flv`
- Flexible Image Transport System - `fits`
- Free Lossless Audio Codec - `flac`
- Free Lossless Image Format - `flif`
- Fujifilm Raw - `raf`
- GL Transmission Format Binary - `glb`
- Game Boy Advance ROM - `gba`
- Game Boy Color ROM - `gbc`
- Game Boy ROM - `gb`
- Google Chrome Extension - `crx`
- Graphics Interchange Format - `gif`
- High Efficiency Image Coding - `heic`
- High Efficiency Image Coding Sequence - `heics`
- High Efficiency Image File Format - `heif`
- High Efficiency Image File Format Sequence - `heifs`
- ICO - `ico`
- ISO 9660 - `iso`
- Impulse Tracker Module - `it`
- JPEG 2000 Part 1 - `jp2`
- JPEG 2000 Part 2 - `jpx`
- JPEG 2000 Part 3 - `mj2`
- JPEG 2000 Part 6 - `jpm`
- JPEG Extended Range - `jxr`
- JPEG XL - `jxl`
- Java Archive - `jar`
- Java Class - `class`
- Java KeyStore - `jks`
- Joint Photographic Experts Group - `jpg`
- Khronos Texture - `ktx`
- Khronos Texture 2 - `ktx2`
- LHA - `lzh`
- LZ4 - `lz4`
- Lempel–Ziv Finite State Entropy - `lzfse`
- Long Range ZIP - `lrz`
- MPEG-1 Video - `mpg`
- MPEG-1/2 Audio Layer III - `mp3`
- MPEG-4 Part 14 Video - `mp4`
- Material Exchange Format - `mxf`
- Matroska Video - `mkv`
- Microsoft Compiled HTML Help - `chm`
- Microsoft DirectDraw Surface - `dds`
- Microsoft Virtual Hard Disk - `vhd`
- Microsoft Virtual Hard Disk 2 - `vhdx`
- Microsoft Visio Drawing - `vsdx`
- Microsoft Visual Studio Extension - `vsix`
- Mobipocket - `mobi`
- Monkey’s Audio - `ape`
- Musepack - `mpc`
- Musical Instrument Digital Interface - `mid`
- Nikon Electronic File - `nef`
- Nintendo 64 ROM - `z64`
- Nintendo DS ROM - `nds`
- Nintendo Entertainment System ROM - `nes`
- Office Open XML Document - `docx`
- Office Open XML Presentation - `pptx`
- Office Open XML Workbook - `xlsx`
- Ogg FLAC - `oga`
- Ogg Media - `ogm`
- Ogg Multiplexed Media - `ogx`
- Ogg Opus - `opus`
- Ogg Speex - `spx`
- Ogg Theora - `ogv`
- Ogg Vorbis - `ogg`
- Olympus Raw Format - `orf`
- OpenDocument Graphics - `odg`
- OpenDocument Presentation - `odp`
- OpenDocument Spreadsheet - `ods`
- OpenDocument Text - `odt`
- OpenEXR - `exr`
- OpenType - `otf`
- Optimized Dalvik Executable - `dey`
- PCAP Dump - `pcap`
- PCAP Next Generation Dump - `pcapng`
- Panasonic Raw - `rw2`
- Portable Document Format - `pdf`
- Portable Network Graphics - `png`
- Qualcomm PureVoice - `qcp`
- Radiance HDR - `hdr`
- Red Hat Package Manager - `rpm`
- Roshal Archive - `rar`
- SQLite 3 - `sqlite`
- ScreamTracker 3 Module - `s3m`
- SeqBox - `sbx`
- Shapefile - `shp`
- SketchUp - `skp`
- Small Web Format - `swf`
- Snappy - `sz`
- Sony DSD Stream File - `dsf`
- Tag Image File Format - `tiff`
- Tape Archive - `tar`
- TrueType - `ttf`
- UNIX archiver - `ar`
- UNIX compress - `Z`
- VirtualBox Virtual Disk Image - `vdi`
- WavPack - `wv`
- Waveform Audio - `wav`
- Web Open Font Format - `woff`
- Web Open Font Format 2 - `woff2`
- WebAssembly Binary - `wasm`
- WebP - `webp`
- Windows Bitmap - `bmp`
- Windows Executable - `exe`
- Windows Media Video - `wmv`
- Windows Metafile - `wmf`
- Windows Shortcut - `lnk`
- XAP - `xap`
- XPInstall - `xpi`
- XZ - `xz`
- ZIP - `zip`
- Zstandard - `zst`
- bzip2 - `bz2`
- cpio - `cpio`
- gzip - `gz`
- lzip - `lz`
- lzop - `lzo`
- macOS Alias - `alias`
- zoo - `zoo`

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
