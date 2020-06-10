use serde_cbor::ser::{IoWrite, Serializer};

pub fn to_writer_with_mode<W, T>(writer: W, value: &T) -> Result<(), serde_cbor::error::Error>
where
    W: std::io::Write,
    T: serde::Serialize,
{
    if packed_format() {
        value.serialize(&mut Serializer::new(&mut IoWrite::new(writer)).packed_format())
    } else {
        value.serialize(&mut Serializer::new(&mut IoWrite::new(writer)))
    }
}

fn packed_format() -> bool {
    if let Ok(v) = std::env::var("RUSTED_RUINS_OBJ_FIELD_MODE") {
        v != "NAMED"
    } else {
        true
    }
}
