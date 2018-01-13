
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidEnumMemberError {
    pub invalid_member: String,
    pub enum_name: &'static str,
}

impl fmt::Display for InvalidEnumMemberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\" is not member for {}", self.invalid_member, self.enum_name)
    }
}

impl Error for InvalidEnumMemberError {
    fn description(&self) -> &str {
        "Invalid enum member error"
    }
}

#[macro_export]
macro_rules! impl_fromstr_for_enum {
    ($e:ident; $($m:ident),*) => {
        impl ::std::str::FromStr for $e {
            type Err = $crate::str2enum::InvalidEnumMemberError;
            fn from_str(s: &str) -> Result<$e, $crate::str2enum::InvalidEnumMemberError> {
                $(
                    if s.eq_ignore_ascii_case(stringify!($m)) {
                        return Ok($e::$m);
                    }
                )*
                    Err($crate::str2enum::InvalidEnumMemberError{
                        invalid_member: s.to_owned(),
                        enum_name: stringify!($e),
                    })
            }
        }
    }
}

