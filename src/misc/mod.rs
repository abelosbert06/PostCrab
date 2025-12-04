use sourceview5::prelude::*;

pub fn syntax_highlighter(buffer: &sourceview5::Buffer, lang: &str) {
    let language_manager = sourceview5::LanguageManager::default();

    if let Some(language) = language_manager.language(lang) {
        buffer.set_language(Some(&language));
    }

    //theme
    let scheme_manager = sourceview5::StyleSchemeManager::default();

    if let Some(scheme) = scheme_manager.scheme("Adwaita-dark") {
        buffer.set_style_scheme(Some(&scheme));
    }
}

pub fn auto_detect_lang(resp_text: &str) -> &str {
    if resp_text.starts_with("{") || resp_text.starts_with("[") {
        "json"
    } else if resp_text.starts_with("<") {
        "xml"
    } else {
        "text"
    }
}
