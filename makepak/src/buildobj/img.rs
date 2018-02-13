
use std::fs::File;
use std::io::Read;
use std::path::Path;
use image::{self, GenericImage};
use common::obj::*;
use dir;
use error::*;
use tomlinput::*;

pub fn build_img(input: ImgInput) -> Result<(Img, ImgData)> {
    let path = Path::new(&input.path);
    let newpath = if path.is_relative() {
        dir::path_from_src_dir(&path)
    }else{
        path.to_owned()
    };
    
    let imgdata = ImgData::load(&newpath)?;
    let w = input.w.unwrap_or(imgdata.dimensions.0);
    let h = input.h.unwrap_or(imgdata.dimensions.1);
    let grid_w = input.grid_w.unwrap_or(1);
    let grid_h = input.grid_h.unwrap_or(1);
    let n_frame = input.n_frame.unwrap_or(1);
    let duration = input.duration.unwrap_or(0);
    
    ensure!(
        w * grid_w == imgdata.dimensions.0 && h * grid_h == imgdata.dimensions.1,
        ErrorKind::ImageSizeError((w * grid_w, h * grid_h), imgdata.dimensions));

    Ok((
        Img {
            data: load_as_vec(&newpath)?,
            w: w,
            h: h,
            grid_w: grid_w,
            grid_h: grid_h,
            n_frame: n_frame,
            duration: duration,
        },
        imgdata))
}

pub struct ImgData {
    img: image::DynamicImage,
    pub dimensions: (u32, u32),
}

impl ImgData {
    fn load(filepath: &Path) -> Result<ImgData> {
        let img = image::open(filepath).chain_err(|| "Error at image file loading")?;
        let dimensions = img.dimensions();

        Ok(ImgData {
            img, dimensions,
        })
    }

    pub fn calc_average_color(&self) -> (u8, u8, u8) {
        let mut n_pixel_count = 0u32;
        let mut rgb = (0u32, 0u32, 0u32);
        
        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                let pixel = self.img.get_pixel(x, y);
                if pixel.data[3] != 0 { // Not transparent pixel
                    rgb.0 += pixel.data[0] as u32;
                    rgb.1 += pixel.data[1] as u32;
                    rgb.2 += pixel.data[2] as u32;
                    n_pixel_count += 1;
                }
            }
        }
        ((rgb.0 / n_pixel_count) as u8, (rgb.1 / n_pixel_count) as u8, (rgb.2 / n_pixel_count) as u8)
    }
}

fn load_as_vec(filepath: &Path) -> Result<Vec<u8>> {
    let mut file = File::open(filepath)?;
    let mut v = Vec::new();

    file.read_to_end(&mut v)?;
    Ok(v)
}

