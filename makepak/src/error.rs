
pub use failure::{Error, ResultExt};

#[derive(Debug, Fail)]
pub enum PakCompileError {
    #[fail(display = "missing field: {}", field_name)]
    MissingField {
        field_name: String,
    },
    #[fail(display = "unexpected value \"{}\" for field \"{}\"", value, field_name)]
    UnexpectedValue {
        field_name: String,
        value: String
    },
    #[fail(display = "image size error: expected size is ({}, {}), but png file size is ({}, {})",
           input_x, input_y, image_x, image_y)]
    ImageSizeError {
        input_x: u32,
        input_y: u32,
        image_x: u32,
        image_y: u32,
    },
    #[fail(display = "object writing error\n{}", description)]
    ObjWriteError {
        description: String,
    }
}

