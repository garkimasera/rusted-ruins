
#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct TownId(String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Town {
    id: String,
    name: Option<String>,
}


