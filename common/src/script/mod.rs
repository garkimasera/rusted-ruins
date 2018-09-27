
mod parser;

use hashmap::HashMap;
use nom::Err;
use nom::types::CompleteStr;

/// Instructions are executed in Game.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Instruction {
    /// Jump to given section
    Jump(String),
    /// Talk instruction (textid, Vec<choice's textid, destination section>)
    Talk(String, Vec<(String, String)>),
    /// Special instruction to start buying at a shop
    ShopBuy,
    /// Special instruction to start selling at a shop
    ShopSell,
    /// Special instruction to get locations of dungeons
    GetDungeonLocation,
}

/// Script consists of one or more sections.
/// One section includes one or more instructions.
pub type Script = HashMap<String, Vec<Instruction>>;

/// Object that include script data.
#[derive(Serialize, Deserialize)]
pub struct ScriptObject {
    pub id: String,
    pub script: Script,
}

pub fn parse(input: &str) -> Result<Script, Err<CompleteStr, u32>> {
    self::parser::parse(CompleteStr(input)).map(|result| result.1)
}

