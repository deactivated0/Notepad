use iced::Font;
use serde::{Deserialize, Serialize};

use crate::theme::AppTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EditorFontFamily {
    #[default]
    Monospace,
    SansSerif,
}

impl EditorFontFamily {
    pub const ALL: [EditorFontFamily; 2] = [
        EditorFontFamily::Monospace,
        EditorFontFamily::SansSerif,
    ];

    pub fn label(self) -> &'static str {
        match self {
            EditorFontFamily::Monospace => "Consolas / Monospace",
            EditorFontFamily::SansSerif => "Segoe UI / Sans-Serif",
        }
    }

    pub fn to_iced_font(self) -> Font {
        match self {
            EditorFontFamily::Monospace => Font::MONOSPACE,
            EditorFontFamily::SansSerif => Font::DEFAULT,
        }
    }
}

impl std::fmt::Display for EditorFontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

/// Persistent user settings (saved to config dir on Native, localStorage on WASM).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub theme: AppTheme,
    pub font_family: EditorFontFamily,
    pub base_font_size: u16,
    pub word_wrap: bool,
    pub show_status_bar: bool,
    pub show_formatting_toolbar: bool,
    #[serde(default = "default_true")]
    pub syntax_highlighting: bool,
}

fn default_true() -> bool {
    true
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            theme: AppTheme::System,
            font_family: EditorFontFamily::Monospace,
            base_font_size: 14,
            word_wrap: true,
            show_status_bar: true,
            show_formatting_toolbar: true,
            syntax_highlighting: true,
        }
    }
}

impl UserSettings {
    #[cfg(not(target_arch = "wasm32"))]
    fn config_path() -> Option<std::path::PathBuf> {
        dirs::config_dir().map(|d| d.join("notepad-iced").join("settings.json"))
    }

    pub fn load() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = Self::config_path()
                && let Ok(raw) = std::fs::read_to_string(path)
                && let Ok(parsed) = serde_json::from_str::<Self>(&raw)
            {
                return parsed;
            }
            Self::default()
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window()
                && let Ok(Some(storage)) = window.local_storage()
                && let Ok(Some(raw)) = storage.get_item("notepad_iced_settings")
                && let Ok(parsed) = serde_json::from_str::<Self>(&raw)
            {
                return parsed;
            }
            Self::default()
        }
    }

    pub fn save(&self) {
        let Ok(json) = serde_json::to_string_pretty(self) else {
            return;
        };

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = Self::config_path() {
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::write(path, json);
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window()
                && let Ok(Some(storage)) = window.local_storage()
            {
                let _ = storage.set_item("notepad_iced_settings", &json);
            }
        }
    }
}
