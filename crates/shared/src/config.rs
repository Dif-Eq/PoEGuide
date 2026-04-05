use serde::{Deserialize, Serialize};

/// A single hotkey: optional modifier + a key name string
/// Key names match global-hotkey's Code strings e.g. "F9", "F10", "F11"
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Hotkey {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub key: String, // e.g. "F9", "F10", "F11"
}

impl Hotkey {
    #[must_use]
    pub fn display(&self) -> String {
        let mut parts = Vec::new();
        if self.ctrl  { parts.push("Ctrl");  }
        if self.shift { parts.push("Shift"); }
        if self.alt   { parts.push("Alt");   }
        parts.push(&self.key);
        parts.join("+")
    }
}

impl Default for Hotkey {
    fn default() -> Self {
        Self { ctrl: false, shift: false, alt: false, key: "F9".to_string() }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub hotkey_advance:  Hotkey,
    pub hotkey_undo:     Hotkey,
    pub hotkey_toggle:   Hotkey,
    /// Overlay opacity 0.0–1.0
    pub opacity:         f32,
    /// Overlay position in absolute screen pixels
    pub overlay_x:       f32,
    pub overlay_y:       f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hotkey_advance: Hotkey { ctrl: true, shift: false, alt: false, key: "F9".to_string()  },
            hotkey_undo:    Hotkey { ctrl: true, shift: false, alt: false, key: "F10".to_string() },
            hotkey_toggle:  Hotkey { ctrl: true, shift: false, alt: false, key: "F11".to_string() },
            opacity:    0.85,
            overlay_x:  0.0,
            overlay_y:  0.0,
        }
    }
}

impl Config {
    fn path() -> Option<std::path::PathBuf> {
        dirs::data_local_dir().map(|d| d.join("poe2_guide").join("config.json"))
    }

    #[must_use]
    pub fn load() -> Self {
        if let Some(path) = Self::path() {
            if let Ok(data) = std::fs::read_to_string(&path) {
                if let Ok(cfg) = serde_json::from_str(&data) {
                    return cfg;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::path() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(data) = serde_json::to_string_pretty(self) {
                let _ = std::fs::write(path, data);
            }
        }
    }

    #[must_use]
    pub fn last_modified() -> Option<std::time::SystemTime> {
        Self::path().and_then(|p| p.metadata().ok()).and_then(|m| m.modified().ok())
    }
}
