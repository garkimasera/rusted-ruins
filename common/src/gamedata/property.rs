/// Represents passive effect for character traits, items, etc.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum Property {
    CharaStr(i16),
    CharaVit(i16),
    CharaDex(i16),
    CharaInt(i16),
    CharaWil(i16),
    CharaCha(i16),
    CharaSpd(i16),
}
