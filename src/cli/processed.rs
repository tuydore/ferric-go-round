use anyhow::{Context, anyhow};
use super::{auxiliary::{ImageSize, overlay_centered}, raw::Cli};

pub(crate) struct ProcessedCli {
    directory: std::path::PathBuf,
    root_name: String,
    image: image::RgbImage,
    num_panels: u32,
    panel_size: ImageSize,
    thumbnail_size: ImageSize,
    sigma: f32,
    thumbnail_border_radius: u32,
    thumbnail_border_color: [u8; 3],
}

impl TryFrom<Cli> for ProcessedCli {
    type Error = anyhow::Error;

    fn try_from(value: Cli) -> Result<Self, Self::Error> {
        let mut image = image::io::Reader::open(&value.filepath)?
            .decode()?
            .as_rgb8()
            .context("could not convert image to RGB8")?
            .to_owned();

        if let Some(height) = value.panel_height_px {
            image = image::imageops::resize(&image, image.width() * height / image.height(), height, image::imageops::FilterType::Lanczos3);
        }

        let panel_size = ImageSize { height: image.height(), width: image.width() / value.num_panels };

        let thumbnail_width = (panel_size.width as f32 * value.thumbnail_frac_panel_width) as u32;
        let thumbnail_size = ImageSize {
            height: thumbnail_width * (image.height() + 2 * value.thumbnail_border_radius) / (image.width() + 2 * value.thumbnail_border_radius),
            width: thumbnail_width,
        };

        let thumbnail_border_color = match value.thumbnail_border_color {
            Some(vec) => {
                if vec.len() != 3 {
                    return Err(anyhow!("border color RGB is not of length 3"));
                } else {
                    [vec[0], vec[1], vec[2]]
                }
            },
            None => [0; 3],
        };

        Ok(Self {
            image,
            num_panels: value.num_panels,
            panel_size,
            thumbnail_size,
            sigma: value.sigma,
            thumbnail_border_radius: value.thumbnail_border_radius,
            thumbnail_border_color,
            directory: value.filepath
                .parent()
                .context("image has no parent directory")?
                .to_owned(),
            root_name: value.filepath
                .file_name()
                .context("image has no name")?
                .to_str()
                .context("image name is not valid UTF-8")?
                .to_string(),
        })
    }
}

impl ProcessedCli {
    pub(crate) fn save_carousel(&self) -> Result<(), anyhow::Error> {
        for idx in 0..self.num_panels {
            image::imageops::crop_imm(&self.image, self.panel_size.width * idx, 0, self.panel_size.width, self.panel_size.height)
                .to_image()
                .save(self.directory.join(format!("{}-{}.jpg", self.root_name, idx + 1)))
                .context("could not save panel")?
        }
        Ok(())
    }

    fn outer_thumbnail_size(&self) -> ImageSize {
        self.thumbnail_size
    }

    fn inner_thumbnail_size(&self) -> ImageSize {
        ImageSize { 
            height: self.thumbnail_size.height - 2 * self.thumbnail_border_radius, 
            width: self.thumbnail_size.width - 2 * self.thumbnail_border_radius 
        }
    }

    fn cover(&self) -> image::RgbImage {
        let mut cover = image::imageops::crop_imm(&self.image, self.image.width() / 2 - self.panel_size.width, 0, self.panel_size.width, self.panel_size.height).to_image();
        cover = image::imageops::blur(&cover, self.sigma * self.panel_size.height as f32);

        let mut thumbnail_with_border = image::RgbImage::from_pixel(self.outer_thumbnail_size().width, self.outer_thumbnail_size().height, image::Rgb(self.thumbnail_border_color));
        let thumbnail = image::imageops::resize(&self.image, self.inner_thumbnail_size().width, self.inner_thumbnail_size().height, image::imageops::FilterType::Lanczos3);
        overlay_centered(&mut thumbnail_with_border, &thumbnail);
        
        overlay_centered(&mut cover, &thumbnail_with_border);
        cover
    }

    pub(crate) fn save_cover(&self) -> Result<(), anyhow::Error> {
        self.cover()
            .save(self.directory.join(format!("{}-cover.jpg", self.root_name)))
            .context("could not save cover")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Default for Cli {
        fn default() -> Self {
            Self { 
                filepath: std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test/panorama.jpg"), 
                num_panels: 4, 
                panel_height_px: None, 
                sigma: 0.01, 
                thumbnail_border_radius: 1, 
                thumbnail_border_color: Some(vec![0, 0, 0]), 
                thumbnail_frac_panel_width: 0.8
            }
        }
    }

    #[test]
    fn test_save_carousel() -> Result<(), anyhow::Error> {
        let tempdir = tempdir::TempDir::new("panorama")?;

        let mut processed: ProcessedCli = Cli::default().try_into()?;
        processed.directory = tempdir.into_path();
        processed.save_carousel()?;
        assert_eq!(processed.directory.read_dir()?.count(), 4);

        Ok(())
    }

    #[test]
    fn test_save_cover() -> Result<(), anyhow::Error> {
        let tempdir = tempdir::TempDir::new("panorama")?;

        let mut processed: ProcessedCli = Cli::default().try_into()?;
        processed.directory = tempdir.into_path();
        processed.save_cover()?;
        assert_eq!(processed.directory.read_dir()?.count(), 1);

        Ok(())
    }
}