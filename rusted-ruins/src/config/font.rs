use super::CONFIG;
use std::collections::HashMap;

/// Font name for each language
#[derive(Debug, Deserialize)]
pub struct FontConfig {
    pub font_names: HashMap<String, String>,
    pub mono_font: String,
}

impl FontConfig {
    /// Get font_name by the first language
    pub fn font_name(&self) -> &str {
        self.font_name_by_lang(&CONFIG.lang)
    }

    fn font_name_by_lang(&self, lang: &str) -> &str {
        if let Some(f) = self.font_names.get(lang) {
            f
        } else {
            warn!("Font for language \"{}\" is not set in the config", lang);
            if let Some(f) = self.font_names.get("en") {
                warn!("Use default font \"{}\"", f);
                f
            } else {
                error!("Cannot find defalut font");
                panic!();
            }
        }
    }
}
