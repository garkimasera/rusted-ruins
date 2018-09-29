
use std::fs::File;
use std::io::Read;
use std::path::Path;
use image::{self, GenericImageView};
use common::obj::*;
use dir;
use error::*;
use tomlinput::*;

pub fn build_img(input: ImgInput) -> Result<(Img, ImgData), Error> {
    let path = Path::new(&input.path);
    let newpath = if path.is_relative() {
        dir::path_from_src_dir(&path)
    }else{
        path.to_owned()
    };
    
    let imgdata = ImgData::load(&newpath)?;
    let w = input.w.unwrap_or(imgdata.dimensions.0);
    let h = input.h.unwrap_or(imgdata.dimensions.1);
    let grid_nx = input.grid_nx.unwrap_or(1);
    let grid_ny = input.grid_ny.unwrap_or(1);
    let n_pattern = input.n_pattern.unwrap_or(1);
    let n_anim_frame = input.n_anim_frame.unwrap_or(1);
    let n_frame = input.n_frame.unwrap_or(n_pattern * n_anim_frame);
    let duration = input.duration.unwrap_or(0);
    
    ensure!(
        w * grid_nx == imgdata.dimensions.0 && h * grid_ny == imgdata.dimensions.1,
        PakCompileError::ImageSizeError{
            input_x: w * grid_nx,
            input_y: h * grid_ny,
            image_x: imgdata.dimensions.0,
            image_y: imgdata.dimensions.1,
        });
    assert!(n_frame == n_pattern * n_anim_frame); // TODO: Make these asserts ensure!()
    assert!(n_frame > 0);
    assert!(n_pattern > 0);
    assert!(n_anim_frame > 0);

    Ok((
        Img {
            data: load_as_vec(&newpath)?,
            w: w,
            h: h,
            grid_nx: grid_nx,
            grid_ny: grid_ny,
            n_frame: n_frame,
            n_pattern: n_pattern,
            n_anim_frame: n_anim_frame,
            duration: duration,
        },
        imgdata))
}

pub struct ImgData {
    img: image::DynamicImage,
    pub dimensions: (u32, u32),
}

impl ImgData {
    fn load(filepath: &Path) -> Result<ImgData, Error> {
        let img = image::open(filepath).context("Error at image file loading")?;
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

fn load_as_vec(filepath: &Path) -> Result<Vec<u8>, Error> {
    let mut file = File::open(filepath)?;
    let mut v = Vec::new();

    file.read_to_end(&mut v)?;
    Ok(v)
}

