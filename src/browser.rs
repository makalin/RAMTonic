use std::collections::HashMap;

pub struct BrowserDetector {
    browser_patterns: HashMap<&'static str, Vec<&'static str>>,
}

impl BrowserDetector {
    pub fn new() -> Self {
        let mut browser_patterns = HashMap::new();

        // Chrome patterns
        browser_patterns.insert(
            "Chrome",
            vec!["chrome", "Google Chrome", "chrome.exe"],
        );

        // Firefox patterns
        browser_patterns.insert(
            "Firefox",
            vec!["firefox", "firefox.exe", "Firefox"],
        );

        // Edge patterns
        browser_patterns.insert(
            "Edge",
            vec!["msedge", "msedge.exe", "Microsoft Edge"],
        );

        // Safari patterns
        browser_patterns.insert(
            "Safari",
            vec!["Safari", "safari", "Safari.app"],
        );

        // Brave patterns
        browser_patterns.insert(
            "Brave",
            vec!["brave", "brave.exe", "Brave Browser"],
        );

        // Opera patterns
        browser_patterns.insert(
            "Opera",
            vec!["opera", "opera.exe", "Opera"],
        );

        Self { browser_patterns }
    }

    pub fn detect_browser(&self, process_name: &str) -> Option<String> {
        let process_name_lower = process_name.to_lowercase();

        for (browser, patterns) in &self.browser_patterns {
            for pattern in patterns {
                if process_name_lower.contains(&pattern.to_lowercase()) {
                    return Some(browser.to_string());
                }
            }
        }

        None
    }

    pub fn detect_process_type(&self, process_name: &str) -> String {
        let name_lower = process_name.to_lowercase();

        if name_lower.contains("gpu") || name_lower.contains("renderer") {
            "GPU/Renderer".to_string()
        } else if name_lower.contains("extension") || name_lower.contains("plugin") {
            "Extension".to_string()
        } else if name_lower.contains("tab") || name_lower.contains("renderer") {
            "Tab".to_string()
        } else if name_lower.contains("utility") || name_lower.contains("service") {
            "Utility".to_string()
        } else {
            "Main Process".to_string()
        }
    }
}

