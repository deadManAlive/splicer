use std::path::Path;

use image::{self, DynamicImage, GenericImageView, ImageError};

pub fn crop<P: AsRef<Path>>(path: P) -> Result<DynamicImage, ImageError> {
    let img = image::open(&path)?;

    let (width, height) = img.dimensions();
    let csize = std::cmp::min(width, height);
    let xstart = (width - csize) / 2;
    let ystart = (height - csize) / 2;

    Ok(img.crop_imm(xstart, ystart, csize, csize))
}
