use super::READABLE_BUNDLE;

const NEW_PAGE_LINE: &str = "<newpage>";

pub fn readable_title_txt(id: &str) -> Option<String> {
    let id = format!("{id}-title");
    READABLE_BUNDLE.format(&id, None)
}

pub fn readable_txt(id: &str) -> Vec<String> {
    let text = if let Some(text) = READABLE_BUNDLE.format(id, None) {
        text
    } else {
        return vec!["(empty)".to_owned()];
    };

    let mut v = Vec::new();
    let mut s = String::new();

    for line in text.lines() {
        if line != NEW_PAGE_LINE {
            s.push_str(line);
            s.push('\n');
        } else {
            v.push(std::mem::take(&mut s));
        }
    }

    if !s.is_empty() {
        v.push(s);
    }

    v
}
