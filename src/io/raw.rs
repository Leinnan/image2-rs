#![cfg(feature = "raw")]

use rawloader;

use crate::color::{Color, Gray, Rgb};
use crate::image::{Convert, Image};
use crate::image_buf::ImageBuf;
use crate::ty::Type;

use std::path::Path;

/// RAW image type
pub struct Raw {
    /// A rawloader image
    pub image: rawloader::RawImage,
}

impl Raw {
    /// Read a RAW image from a file
    pub fn read<P: AsRef<Path>>(path: &P) -> Option<Raw> {
        let filename = match path.as_ref().to_str() {
            Some(f) => f,
            None => return None,
        };

        let raw_image = match rawloader::decode_file(filename) {
            Ok(r) => r,
            Err(_) => return None,
        };

        Some(Raw { image: raw_image })
    }

    pub fn to_rgb_image<T: Type>(self) -> Option<ImageBuf<T, Rgb>> {
        if self.image.cpp == 1 {
            return None;
        }

        match self.image.data {
            rawloader::RawImageData::Integer(data) => {
                let im = ImageBuf::new_from(self.image.width, self.image.height, data);
                let mut dest = ImageBuf::new(self.image.width, self.image.height);
                im.convert_type(&mut dest);
                Some(dest)
            }
            rawloader::RawImageData::Float(data) => {
                let im = ImageBuf::new_from(self.image.width, self.image.height, data);
                let mut dest = ImageBuf::new(self.image.width, self.image.height);
                im.convert_type(&mut dest);
                Some(dest)
            }
        }
    }

    pub fn to_gray_image<T: Type>(self) -> Option<ImageBuf<T, Gray>> {
        if self.image.cpp != 1 {
            return None;
        }

        match self.image.data {
            rawloader::RawImageData::Integer(data) => {
                let im = ImageBuf::new_from(self.image.width, self.image.height, data);
                let mut dest = ImageBuf::new(self.image.width, self.image.height);
                im.convert_type(&mut dest);
                Some(dest)
            }
            rawloader::RawImageData::Float(data) => {
                let im = ImageBuf::new_from(self.image.width, self.image.height, data);
                let mut dest = ImageBuf::new(self.image.width, self.image.height);
                im.convert_type(&mut dest);
                Some(dest)
            }
        }
    }

    pub fn to_image<T: Type, C: Color + Convert<T, Rgb, T, C> + Convert<T, Gray, T, C>>(
        self,
    ) -> ImageBuf<T, C> {
        let mut dest = ImageBuf::new(self.image.width, self.image.height);
        match self.to_gray_image() {
            Some(im) => im.convert(&mut dest),
            None => {
                let im = self.to_rgb_image().unwrap();
                im.convert(&mut dest);
            }
        }
        dest
    }
}
