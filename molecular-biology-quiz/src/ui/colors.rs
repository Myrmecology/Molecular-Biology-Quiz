use colored::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ColorTheme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub success: Color,
    pub error: Color,
    pub warning: Color,
    pub info: Color,
}

impl ColorTheme {
    pub fn default() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Magenta,
            accent: Color::Yellow,
            success: Color::Green,
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::Blue,
        }
    }
}

pub struct SectionThemes {
    themes: HashMap<String, ColorTheme>,
}

impl SectionThemes {
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        
        // DNA & RNA Theme - Blues and Cyans
        themes.insert(
            "dna_rna".to_string(),
            ColorTheme {
                primary: Color::Blue,
                secondary: Color::Cyan,
                accent: Color::BrightBlue,
                success: Color::Green,
                error: Color::Red,
                warning: Color::Yellow,
                info: Color::BrightCyan,
            },
        );
        
        // Proteins Theme - Purples and Magentas
        themes.insert(
            "proteins".to_string(),
            ColorTheme {
                primary: Color::Magenta,
                secondary: Color::BrightMagenta,
                accent: Color::Yellow,
                success: Color::Green,
                error: Color::Red,
                warning: Color::Yellow,
                info: Color::BrightWhite,
            },
        );
        
        // Cell Division Theme - Oranges and Reds
        themes.insert(
            "cell_division".to_string(),
            ColorTheme {
                primary: Color::Red,
                secondary: Color::BrightRed,
                accent: Color::Yellow,
                success: Color::Green,
                error: Color::BrightRed,
                warning: Color::Yellow,
                info: Color::White,
            },
        );
        
        // Gene Expression Theme - Greens and Yellows
        themes.insert(
            "gene_expression".to_string(),
            ColorTheme {
                primary: Color::Green,
                secondary: Color::BrightGreen,
                accent: Color::Yellow,
                success: Color::BrightGreen,
                error: Color::Red,
                warning: Color::BrightYellow,
                info: Color::Cyan,
            },
        );
        
        // Cellular Respiration Theme - Reds and Pinks
        themes.insert(
            "respiration".to_string(),
            ColorTheme {
                primary: Color::BrightRed,
                secondary: Color::Magenta,
                accent: Color::BrightYellow,
                success: Color::Green,
                error: Color::Red,
                warning: Color::Yellow,
                info: Color::BrightMagenta,
            },
        );
        
        // Molecular Techniques Theme - Cyans and Whites
        themes.insert(
            "techniques".to_string(),
            ColorTheme {
                primary: Color::Cyan,
                secondary: Color::White,
                accent: Color::BrightCyan,
                success: Color::Green,
                error: Color::Red,
                warning: Color::Yellow,
                info: Color::BrightWhite,
            },
        );
        
        // Cell Signaling Theme - Yellows and Golds
        themes.insert(
            "signaling".to_string(),
            ColorTheme {
                primary: Color::Yellow,
                secondary: Color::BrightYellow,
                accent: Color::White,
                success: Color::Green,
                error: Color::Red,
                warning: Color::BrightYellow,
                info: Color::Cyan,
            },
        );
        
        Self { themes }
    }
    
    pub fn get_theme(&self, section: &str) -> ColorTheme {
        self.themes
            .get(section)
            .cloned()
            .unwrap_or_else(ColorTheme::default)
    }
}

pub fn apply_gradient(text: &str, colors: &[Color]) -> String {
    if colors.is_empty() {
        return text.to_string();
    }
    
    let chars: Vec<char> = text.chars().collect();
    let mut result = String::new();
    let color_step = chars.len() / colors.len().max(1);
    
    for (i, ch) in chars.iter().enumerate() {
        let color_index = (i / color_step.max(1)).min(colors.len() - 1);
        let colored_char = ch.to_string().color(colors[color_index]);
        result.push_str(&colored_char.to_string());
    }
    
    result
}

pub fn rainbow_text(text: &str) -> String {
    let rainbow_colors = vec![
        Color::Red,
        Color::BrightRed,
        Color::Yellow,
        Color::BrightYellow,
        Color::Green,
        Color::BrightGreen,
        Color::Cyan,
        Color::BrightCyan,
        Color::Blue,
        Color::BrightBlue,
        Color::Magenta,
        Color::BrightMagenta,
    ];
    
    apply_gradient(text, &rainbow_colors)
}

pub fn print_colored_box(text: &str, color: Color, width: usize) {
    let horizontal = "─".repeat(width);
    let top = format!("┌{}┐", horizontal);
    let bottom = format!("└{}┘", horizontal);
    
    println!("{}", top.color(color));
    
    let _padding = (width - text.len()) / 2;  // Prefixed with underscore to fix warning
    let padded_text = format!("│{:^width$}│", text, width = width);
    println!("{}", padded_text.color(color));
    
    println!("{}", bottom.color(color));
}