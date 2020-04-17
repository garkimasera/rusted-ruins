use serde_cbor::ser::{IoWrite, Serializer};

pub fn to_writer_packed<W, T>(writer: W, value: &T) -> Result<(), serde_cbor::error::Error>
where
    W: std::io::Write,
    T: serde::Serialize,
{
    value.serialize(&mut Serializer::new(&mut IoWrite::new(writer)).packed_format())
}
