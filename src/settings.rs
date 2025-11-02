use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub auto_refresh_interval: u64,
    pub memory_alert_threshold: f64,
    pub default_sort_by: String,
    pub default_sort_ascending: bool,
    pub show_system_stats: bool,
    pub refresh_on_startup: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            auto_refresh_interval: 2000, // 2 seconds
            memory_alert_threshold: 85.0, // 85%
            default_sort_by: "memory".to_string(),
            default_sort_ascending: false,
            show_system_stats: true,
            refresh_on_startup: true,
        }
    }
}

impl AppSettings {
    pub fn settings_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("RAMTonic");
        fs::create_dir_all(&path).ok();
        path.push("settings.json");
        path
    }

    pub fn load() -> Result<Self, String> {
        let path = Self::settings_path();
        
        if !path.exists() {
            let default = AppSettings::default();
            default.save()?;
            return Ok(default);
        }
        
        let contents = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read settings: {}", e))?;
        
        let settings: AppSettings = serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse settings: {}", e))?;
        
        Ok(settings)
    }

    pub fn save(&self) -> Result<bool, String> {
        let path = Self::settings_path();
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;
        
        fs::write(&path, json)
            .map_err(|e| format!("Failed to write settings: {}", e))?;
        
        Ok(true)
    }
}

