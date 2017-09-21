
#[derive(Debug)]
pub struct InvalidEnumMember(pub String);

#[macro_export]
macro_rules! impl_fromstr_for_enum {
    ($e:ident; $($m:ident),*) => {
        impl FromStr for $e {
            type Err = $crate::str2enum::InvalidEnumMember;
            fn from_str(s: &str) -> Result<$e, $crate::str2enum::InvalidEnumMember> {
                use std::ascii::AsciiExt;
                
                $(
                    if s.eq_ignore_ascii_case(stringify!($m)) {
                        return Ok($e::$m);
                    }
                )*
                Err($crate::str2enum::InvalidEnumMember(s.to_owned()))
            }
        }
    }
}

