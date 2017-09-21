
use std::fs::File;
use std::io::Read;
use std::path::Path;
use image::{self, GenericImage};
use common::obj::*;
use dir;
use error::*;
use tomlinput::*;

pub fn build_img(input: ImgInput) -> Result<Img> {
    let path = Path::new(&input.path);
    let newpath = if path.is_relative() {
        dir::path_from_src_dir(&path)
    }else{
        path.to_owned()
    };
    
    let dimensions = get_dimensions(&newpath)?;
    let grid_w = input.grid_w.unwrap_or(1);
    let grid_h = input.grid_h.unwrap_or(1);
    ensure!(
        input.w * grid_w == dimensions.0 && input.h * grid_h == dimensions.1,
        ErrorKind::ImageSizeError((input.w * grid_w, input.h * grid_h), dimensions));

    Ok(Img {
        data: load_as_vec(&newpath)?,
        w: input.w,
        h: input.h,
        grid_w: grid_w,
        grid_h: grid_h,
    })
}

fn get_dimensions(filepath: &Path) -> Result<(u32, u32)> {
    let img = image::open(filepath).chain_err(|| "Error at image file loading")?;
    Ok(img.dimensions())
}

fn load_as_vec(filepath: &Path) -> Result<Vec<u8>> {
    let mut file = File::open(filepath)?;
    let mut v = Vec::new();

    file.read_to_end(&mut v)?;
    Ok(v)
}

pub fn build_icon(input: IconInput) -> Result<Icon> {
    Ok(Icon {
        n: input.n.unwrap_or(0)
    })
}
