use std::path::PathBuf;
use std::sync::Once;
use sourceview5::prelude::*;

static INIT_PATHS: Once = Once::new();

fn candidate_data_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            // macOS .app bundle: PostCrab.app/Contents/MacOS/postcrab-r4 -> PostCrab.app/Contents/Resources/share
            if let Some(contents_dir) = exe_dir.parent() {
                let bundle_share = contents_dir.join("Resources").join("share");
                if bundle_share.exists() {
                    dirs.push(bundle_share);
                }
            }
            // Portable / next to executable: ./share
            let portable_share = exe_dir.join("share");
            if portable_share.exists() {
                dirs.push(portable_share);
            }
        }
    }

    // Common Homebrew paths on macOS
    #[cfg(target_os = "macos")]
    {
        dirs.push(PathBuf::from("/opt/homebrew/share"));
        dirs.push(PathBuf::from("/usr/local/share"));
    }

    // Common MSYS2 / MinGW paths on Windows
    #[cfg(target_os = "windows")]
    {
        if let Ok(msys_prefix) = std::env::var("MSYSTEM_PREFIX") {
            dirs.push(PathBuf::from(format!("{}/share", msys_prefix)));
        }
    }

    dirs
}

fn init_sourceview_search_paths() {
    INIT_PATHS.call_once(|| {
        let lm = sourceview5::LanguageManager::default();
        let mut lm_paths: Vec<String> = lm.search_path().iter().map(|s| s.to_string()).collect();

        let sm = sourceview5::StyleSchemeManager::default();
        let mut sm_paths: Vec<String> = sm.search_path().iter().map(|s| s.to_string()).collect();

        for base_dir in candidate_data_dirs() {
            let lang_dir = base_dir.join("gtksourceview-5").join("language-specs");
            if lang_dir.exists() {
                let s = lang_dir.to_string_lossy().to_string();
                if !lm_paths.contains(&s) {
                    lm_paths.push(s);
                }
            }

            let style_dir = base_dir.join("gtksourceview-5").join("styles");
            if style_dir.exists() {
                let s = style_dir.to_string_lossy().to_string();
                if !sm_paths.contains(&s) {
                    sm_paths.push(s);
                }
            }
        }

        let lm_slice: Vec<&str> = lm_paths.iter().map(|s| s.as_str()).collect();
        lm.set_search_path(&lm_slice);

        let sm_slice: Vec<&str> = sm_paths.iter().map(|s| s.as_str()).collect();
        sm.set_search_path(&sm_slice);
    });
}

pub fn syntax_highlighter(buffer: &sourceview5::Buffer, lang: &str) {
    init_sourceview_search_paths();

    let language_manager = sourceview5::LanguageManager::default();
    if let Some(language) = language_manager.language(lang) {
        buffer.set_language(Some(&language));
    }

    // Fallback theme resolution
    let scheme_manager = sourceview5::StyleSchemeManager::default();
    let scheme = scheme_manager
        .scheme("Adwaita-dark")
        .or_else(|| scheme_manager.scheme("classic-dark"))
        .or_else(|| scheme_manager.scheme("solarized-dark"));

    if let Some(scheme) = scheme {
        buffer.set_style_scheme(Some(&scheme));
    }
}

pub fn auto_detect_lang(resp_text: &str) -> &str {
    let trimmed = resp_text.trim_start();
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        "json"
    } else if trimmed.starts_with('<') {
        "xml"
    } else {
        "text"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_detect_json() {
        assert_eq!(auto_detect_lang("{\"key\": \"value\"}"), "json");
        assert_eq!(auto_detect_lang("  [1, 2, 3]"), "json");
    }

    #[test]
    fn test_auto_detect_xml() {
        assert_eq!(auto_detect_lang("<root><child/></root>"), "xml");
        assert_eq!(auto_detect_lang("   <?xml version=\"1.0\"?>"), "xml");
    }

    #[test]
    fn test_auto_detect_text() {
        assert_eq!(auto_detect_lang("Hello, World!"), "text");
        assert_eq!(auto_detect_lang("plain text response"), "text");
    }

    #[test]
    fn test_candidate_data_dirs_not_empty_on_mac() {
        let dirs = candidate_data_dirs();
        #[cfg(target_os = "macos")]
        assert!(!dirs.is_empty());
    }
}
