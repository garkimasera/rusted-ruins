use crate::error::*;
use std::path::Path;

pub fn read_file_as_string<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    use std::fs::File;
    use std::io::Read;

    let mut f = File::open(path.as_ref())?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn replace_str<S0: AsRef<str>, S1: AsRef<str>>(s: &str, table: &[(S0, S1)]) -> String {
    enum State {
        Normal,
        DollarFound,
        Var,
    }
    let mut rst = String::new();
    let mut var_name = String::new();
    let mut state = State::Normal;

    for c in s.chars() {
        match state {
            State::Normal => {
                if c == '$' {
                    state = State::DollarFound;
                } else {
                    rst.push(c);
                }
            }
            State::DollarFound => {
                if c == '(' {
                    state = State::Var;
                } else {
                    rst.push('$');
                    rst.push(c);
                    state = State::Normal;
                }
            }
            State::Var => {
                if c == ')' {
                    {
                        let a = table
                            .iter()
                            .find(|t| t.0.as_ref() == var_name)
                            .map(|t| t.1.as_ref())
                            .unwrap_or(&var_name);
                        rst.push_str(a);
                    }
                    var_name.clear();
                    state = State::Normal;
                } else {
                    var_name.push(c);
                }
            }
        }
    }
    match state {
        State::Normal => (),
        State::DollarFound => {
            rst.push('$');
        }
        State::Var => {}
    }
    rst
}

macro_rules! replace_str {
    ($original_text:expr; $($target:ident = $value:expr),*) => {{
        use std::borrow::Cow;
        use crate::text::ToText;
        let text_raw: &str = $original_text.as_ref();
        let mut table: Vec<(&str, Cow<str>)> = Vec::new();
        $(
            table.push((stringify!($target), ToText::to_text($value)));
        )*;

        $crate::util::replace_str(text_raw, table.as_slice())
    }}
}
