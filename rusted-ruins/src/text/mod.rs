
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;
use error::*;
use config;
use common::basic;

mod text_id_impl;

/// Initialize lazy static
pub fn init() {
    ::lazy_static::initialize(&OBJ_TXT_MAP);
    ::lazy_static::initialize(&LOG_TXT_MAP);
    ::lazy_static::initialize(&UI_TXT_MAP);
    ::lazy_static::initialize(&TALK_TXT_MAP);
    ::lazy_static::initialize(&MISC_TXT_MAP);
}

lazy_static! {
    static ref OBJ_TXT_MAP:  HashMap<String, String> = load_trans_txt(basic::OBJ_TXT_DIR);
    static ref LOG_TXT_MAP:  HashMap<String, String> = load_trans_txt(basic::LOG_TXT_DIR);
    static ref UI_TXT_MAP:   HashMap<String, String> = load_trans_txt(basic::UI_TXT_DIR);
    static ref TALK_TXT_MAP: HashMap<String, String> = load_trans_txt(basic::TALK_TXT_DIR);
    static ref MISC_TXT_MAP: HashMap<String, String> = load_trans_txt(basic::MISC_TXT_DIR);
}

pub fn obj_txt<'a>(id: &'a str) -> &'a str {
    if let Some(txt) = OBJ_TXT_MAP.get(id) { txt }else{ id }
}

#[allow(unused)]
pub fn obj_txt_checked(id: &str) -> Option<&'static str> {
    OBJ_TXT_MAP.get(id).map(|txt| txt.as_ref())
}

pub fn log_txt<'a>(id: &'a str) -> &'a str {
    if let Some(txt) = LOG_TXT_MAP.get(id) { txt }else{ id }
}

#[allow(unused)]
pub fn log_txt_checked(id: &str) -> Option<&'static str> {
    LOG_TXT_MAP.get(id).map(|txt| txt.as_ref())
}

pub fn ui_txt<'a>(id: &'a str) -> &'a str {
    if let Some(txt) = UI_TXT_MAP.get(id) { txt }else{ id }
}

#[allow(unused)]
pub fn ui_txt_checked(id: &str) -> Option<&'static str> {
    UI_TXT_MAP.get(id).map(|txt| txt.as_ref())
}

pub fn talk_txt<'a>(id: &'a str) -> &'a str {
    if let Some(txt) = TALK_TXT_MAP.get(id) { txt }else{ id }
}

#[allow(unused)]
pub fn talk_txt_checked(id: &str) -> Option<&'static str> {
    TALK_TXT_MAP.get(id).map(|txt| txt.as_ref())
}

pub fn misc_txt<'a>(id: &'a str) -> &'a str {
    if let Some(txt) = MISC_TXT_MAP.get(id) { txt }else{ id }
}

#[allow(unused)]
pub fn misc_txt_checked(id: &str) -> Option<&'static str> {
    MISC_TXT_MAP.get(id).map(|txt| txt.as_ref())
}

/// This is helper trait for some data objects that need to be printed in game.
/// Returned text id is translated to appropriate words in text module.
pub trait ToTextId {
    fn to_textid(&self) -> &'static str;
}

pub fn to_txt<T: ToTextId>(a: &T) -> &'static str {
    misc_txt(a.to_textid())
}

macro_rules! replace_str {
    ($original_text:expr; $($target:ident = $value:expr),*) => {{
        let text_raw: &str = $original_text.as_ref();
        let mut table: Vec<(&str, String)> = Vec::new();
        $(
            table.push((stringify!($target), $value.to_string()));
        )*;
        
        let t = $crate::util::replace_str(text_raw, table.as_slice());
        t
    }}
}

fn load_trans_txt(kind: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut textdirs: Vec<PathBuf> = Vec::new();

    // If the second language is specified, search (data_dir)/text/(second_lang)/(kind)/*.txt
    if config::CONFIG.second_lang != "" {
        let second_lang = &config::CONFIG.second_lang;
        let v = config::get_data_dirs();
        for mut dir in v.into_iter() {
            dir.push("text");
            dir.push(second_lang);
            textdirs.push(dir);
        }
    }

    // Pushes the first language's dirs
    let v = config::get_data_dirs();
    for mut dir in v.into_iter() {
        dir.push("text");
        dir.push(&config::CONFIG.lang);
        textdirs.push(dir);
    }

    for mut dir in textdirs {
        info!("Text file loading from directory : {:?}", dir);
        dir.push(kind);
        
        for f in WalkDir::new(dir).into_iter() {
            let f = match f {
                Ok(f) => f,
                Err(e) => { warn!("{}", e); continue; },
            };

            if !f.file_type().is_file() ||
                f.path().extension().is_none() ||
                f.path().extension().unwrap() != "txt" {
                    continue;
                }
            
            let _ = add_file(f.path(), &mut map, kind != "talk");
        }
    }
    
    map
}

fn add_file<P: AsRef<Path>>(p: P, map: &mut HashMap<String, String>, ignore_newline: bool) -> Result<()> {
    let p = p.as_ref();
    let file = fs::File::open(p)?;
    let file = BufReader::new(file);
    
    let mut key: Option<String> = None;
    let mut value = String::new();
    
    for line in file.lines() {
        let line = line?;
        let mut is_key = false;

        if let Some(first_char) =  line.chars().next() {
            if first_char == '#' { continue; } // Skip comment line
            if first_char == '%' { is_key = true; }
        }else{
            continue; // Skip empty line
        }

        if is_key {
            if key.is_some() {
                map.insert(::std::mem::replace(&mut key, None).unwrap(), value.clone());
                value.clear();
            }else{
                
            }
            key = Some(line[1..].trim_left().to_string());
        }else{
            value.push_str(&line);
            if !ignore_newline {
                value.push('\n');
            }
        }
    }
    
    if key.is_some() {
        map.insert(key.unwrap(), value);
    }

    Ok(())
}

