# image2 - image processing library

<a href="https://crates.io/crates/image2">
    <img src="https://img.shields.io/crates/v/image2.svg">
</a>

A Rust crate focused on generic image processing for a wide range of image formats and data types. [OpenImageIO](https://github.com/OpenImageIO/io) is used to read/write images and supports "TIFF, JPEG/JFIF, OpenEXR, PNG, HDR/RGBE, ICO, BMP, Targa, JPEG-2000, RMan Zfile, FITS, DDS, Softimage PIC, PNM, DPX, Cineon, IFF, Field3D, Ptex, Photoshop PSD, Wavefront RLA, SGI, WebP, GIF, and a variety of RAW digital camera formats"  and `ImageMagick` can be used as a fallback.

- Supports a wide range of data types
- Easy to implement new color types
- Read/write images of any supported type
- Parallel pixel iterators
- Generic image processing across data types
- Composable operations using `Filter` with async support

## Features

- `oiio`
  * Enables I/O using OpenImageIO
- `parallel`:
  * Enables parallel image iterators

## External dependencies

- `libOpenImageIO` (optional)
    * `oiio` feature
    * Version >= 2.0
    * Debian-based distros: `apt install libopenimageio-dev`


