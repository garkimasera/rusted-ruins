macro_rules! warn_continue {
    ($value:expr) => {
        match $value {
            Ok(o) => o,
            Err(e) => {
                warn!("{}", e);
                continue;
            }
        }
    };
}
