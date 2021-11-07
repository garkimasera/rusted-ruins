macro_rules! misc_txt_format {
    ($id:expr; $($target:ident = $value:expr),*) => {{
        let mut table = fluent::FluentArgs::new();
        $(
            let value = fluent::FluentValue::String($value.to_text());
            table.add(stringify!($target), value);
        )*

        crate::text::misc_txt_with_args($id, Some(&table))
    }}
}

macro_rules! ui_txt_format {
    ($id:expr; $($target:ident = $value:expr),*) => {{
        use crate::text::ToText;
        let mut table = fluent::FluentArgs::new();
        $(
            let value = fluent::FluentValue::String($value.to_text());
            table.add(stringify!($target), value);
        )*

        crate::text::ui_txt_with_args($id, Some(&table))
    }}
}
