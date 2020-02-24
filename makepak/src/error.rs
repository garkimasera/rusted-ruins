use thiserror::Error;

#[derive(Error, Debug)]
pub enum PakCompileError {
    #[error("missing field: {field_name}")]
    MissingField { field_name: String },
    #[error("unexpected value \"{field_name}\" for field \"{value}\"")]
    UnexpectedValue { field_name: String, value: String },
    #[error("image size error: expected size is ({input_x}, {input_y}), but png file size is ({image_x}, {image_y})")]
    ImageSizeError {
        input_x: u32,
        input_y: u32,
        image_x: u32,
        image_y: u32,
    },
    #[error("object writing error\n{description}")]
    ObjWriteError { description: String },
    #[error("script parse error\n{description}")]
    ScriptParseError { description: String },
}
