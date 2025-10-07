#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl Theme {
    /// Get CSS variables for this theme
    pub fn to_css_vars(&self) -> &'static str {
        match self {
            Theme::Dark => r#"
                --bg-primary: #000000;
                --bg-secondary: #1a1a1a;
                --text-primary: #ffffff;
                --text-secondary: #a0a0a0;
                --accent-blue: #3b82f6;
                --accent-yellow: #fbbf24;
                --line-color: #ef4444;
                --preview-color: #fca5a5;
                --border-color: #333333;
            "#,
            Theme::Light => r#"
                --bg-primary: #ffffff;
                --bg-secondary: #f5f5f5;
                --text-primary: #1a1a1a;
                --text-secondary: #666666;
                --accent-blue: #3b82f6;
                --accent-yellow: #fbbf24;
                --line-color: #ef4444;
                --preview-color: #fca5a5;
                --border-color: #e5e5e5;
            "#,
        }
    }

    /// Toggle between dark and light themes
    pub fn toggle(&mut self) {
        *self = match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        };
    }

    /// Check if dark theme
    pub fn is_dark(&self) -> bool {
        matches!(self, Theme::Dark)
    }

    /// Check if light theme
    pub fn is_light(&self) -> bool {
        matches!(self, Theme::Light)
    }
    
    // Direct color getters for inline styles
    pub fn bg_primary(&self) -> &'static str {
        match self {
            Theme::Dark => "#000000",
            Theme::Light => "#ffffff",
        }
    }
    
    pub fn bg_secondary(&self) -> &'static str {
        match self {
            Theme::Dark => "#1a1a1a",
            Theme::Light => "#f5f5f5",
        }
    }
    
    pub fn text_primary(&self) -> &'static str {
        match self {
            Theme::Dark => "#ffffff",
            Theme::Light => "#1a1a1a",
        }
    }
    
    pub fn text_secondary(&self) -> &'static str {
        match self {
            Theme::Dark => "#a0a0a0",
            Theme::Light => "#666666",
        }
    }
    
    pub fn accent_blue(&self) -> &'static str {
        "#3b82f6"
    }
    
    pub fn accent_yellow(&self) -> &'static str {
        "#fbbf24"
    }
    
    pub fn border_color(&self) -> &'static str {
        match self {
            Theme::Dark => "#333333",
            Theme::Light => "#e5e5e5",
        }
    }
}
