macro_rules! find_attr {
    ($e:expr, $enum_type:ident::$enum_member:ident) => {
        $e.attrs
            .iter()
            .find(|attr| matches!(attr, $enum_type::$enum_member { .. }))
    };

    ($e:expr, $p:pat => $result:tt) => {
        $e.attrs
            .iter()
            .filter_map(|attr| match attr {
                $p => Some($result),
                _ => None,
            })
            .next()
    };

    ($e:expr, $enum_type:ident::$enum_member:ident($name:ident)) => {
        $e.attrs
            .iter()
            .filter_map(|attr| match attr {
                $enum_type::$enum_member($name) => Some($name),
                _ => None,
            })
            .next()
    };
}

macro_rules! has_attr {
    ($e:expr, $p:path) => {
        $e.attrs.iter().any(|attr| matches!(attr, $p { .. }))
    };
}
