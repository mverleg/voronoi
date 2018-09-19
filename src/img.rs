use color::Color;
use dims::{Dim, X, Y};
use image::DynamicImage;
use image::ImageBuffer;
use std::io;
use std::ops::Index;
use std::ops::IndexMut;
use std::path::Path;

pub struct Img {
    data: ImageBuffer<Color, Vec<u8>>,
    width: X,
    height: Y,
}

impl Img {
    pub fn wrap(data: ImageBuffer<Color, Vec<u8>>) -> Self {
        let width = X::new(data.width() as usize);
        let height = Y::new(data.height() as usize);
        Img {
            data,
            width,
            height,
        }
    }

    pub fn empty(width: X, height: Y) -> Self {
        Img::wrap(ImageBuffer::new(
            width._expose() as u32,
            height._expose() as u32,
        ))
    }

    pub fn load(pth: &Path) -> Self {
        let dyn_img = image::open(pth).unwrap();
        if let DynamicImage::ImageRgb8(img) = dyn_img {
            // Get a random seed and generate points.
            Img::wrap(img)

        } else {
            panic!("Wrong image type (maybe there is an alpha channel?)");
        }
    }

    #[inline]
    pub fn width(&self) -> X {
        self.width
    }

    #[inline]
    pub fn height(&self) -> Y {
        self.height
    }

    #[inline]
    pub fn pixel_cnt(&self) -> usize {
        (self.width()._expose() * self.height()._expose()) as usize
    }

    pub fn save<Q>(&self, path: Q) -> io::Result<()>
    where
        Q: AsRef<Path>,
    {
        self.data.save(path)
    }
}

impl Index<(X, Y)> for Img {
    type Output = Color;

    fn index(&self, index: (X, Y)) -> &Self::Output {
        let (x, y) = index;
        &self.data[(x._expose() as u32, y._expose() as u32)]
    }
}

impl IndexMut<(X, Y)> for Img {
    fn index_mut(&mut self, index: (X, Y)) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.data[(x._expose() as u32, y._expose() as u32)]
    }
}

impl Clone for Img {
    fn clone(&self) -> Self {
        Img::wrap(self.data.clone())
    }
}