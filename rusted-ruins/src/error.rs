
#![allow(unused_doc_comment)]

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Toml(::toml::de::Error);
    }
    errors {
        
    }
}

macro_rules! check_draw {
    ($rst:expr) => {
        match $rst {
            Ok(_) => (),
            Err(e) => { eprintln!("SDL drawing error : {}", e) },
        }
    }
}

