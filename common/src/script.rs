
use std::ops::Index;
use std::fmt;
use crate::hashmap::HashMap;
use crate::gamedata::Time;

/// Instructions are executed in Game.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Instruction {
    /// Jump to given section
    Jump(String),
    /// Jump if given expr is true
    JumpIf(String, Expr),
    /// Talk instruction (textid, Vec<choice's textid, destination section>)
    Talk(String, Vec<(String, String)>),
    /// Set global variable
    GSet(String, Expr),
    /// Player receive money
    ReceiveMoney(Expr),
    /// Remove item form player's inventory
    RemoveItem(String),
    /// Special Instruction
    Special(SpecialInstruction),
}

/// Special Instructions
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SpecialInstruction {
    /// Special instruction to start buying at a shop
    ShopBuy,
    /// Special instruction to start selling at a shop
    ShopSell,
    /// Special instruction to get locations of dungeons
    GetDungeonLocation,
    /// Special instruction to open quest window
    QuestWindow,
    /// Special instruction to receive quest rewards
    ReceiveQuestRewards,
}

impl std::str::FromStr for SpecialInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SpecialInstruction::*;
        match s {
            "shop_buy" => Ok(ShopBuy),
            "shop_sell" => Ok(ShopSell),
            "get_dungeon_location" => Ok(GetDungeonLocation),
            "quest_window" => Ok(QuestWindow),
            "receive_quest_rewards" => Ok(ReceiveQuestRewards),
            _ => Err(()),
        }
    }
}

/// Expression in script.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Expr {
    Value(Value),
    Term(Vec<(Operator, Expr)>),
    /// Reference to global variable
    GVar(String),
    IsGVarEmpty(String),
    CurrentTime,
    DurationHour(Box<Expr>, Box<Expr>),
    HasItem(String),
}

/// Value is the result of evaluation of Expr.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum Value {
    Bool(bool),
    Int(i32),
    Time(Time),
    /// Referenced for unknown variable. This can be changed to 0 or false.
    RefUnknownVar,
    Error(ExprErrorKind),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Operator {
    None,
    Or,
    And,
    Eq, NotEq,
    Less, LessEq, Greater, GreaterEq,
    Add, Sub,
    Mul, Div,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ExprErrorKind {
    InvalidType, UnknownIdRef, Other,
}

/// Script consists of one or more sections.
/// One section includes one or more instructions.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Script(HashMap<String, Vec<Instruction>>);

impl Script {
    pub fn from_map(map: HashMap<String, Vec<Instruction>>) -> Script {
        Script(map)
    }
    
    pub fn get(&self, pos: &ScriptPos) -> Option<&Instruction> {
        if let Some(v) = self.0.get(&pos.section) {
            v.get(pos.i)
        } else {
            warn!("script error: unknown section {}", pos.section);
            None
        }
    }
    
    pub fn section(&self, s: &str) -> &[Instruction] {
        self.0[s].as_ref()
    }
}

pub const QUIT_SECTION: &'static str = "quit";
pub const CONTINUE_SECTION: &'static str = "continue";

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ScriptPos {
    pub section: String,
    pub i: usize,
}

impl ScriptPos {
    pub fn advance(&mut self) {
        self.i += 1;
    }

    pub fn set_section<S: ToString>(&mut self, section: S) {
        let section = section.to_string();

        assert_ne!(section, QUIT_SECTION);
        assert_ne!(section, CONTINUE_SECTION);
        
        self.i = 0;
        self.section = section;
    }
}

impl<'a> Index<&'a ScriptPos> for Script {
    type Output = Instruction;

    fn index(&self, pos: &ScriptPos) -> &Instruction {
        &self.section(&pos.section)[pos.i]
    }
}

/// Object that include script data.
#[derive(Serialize, Deserialize)]
pub struct ScriptObject {
    pub id: String,
    pub script: Script,
}

#[derive(Clone, Debug)]
pub struct ScriptParseError {
    description: String,
}

impl std::error::Error for ScriptParseError {}

impl fmt::Display for ScriptParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "script parse error : {}", self.description)
    }
}

