use colored::*;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Print, ResetColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, stdout, Write};

pub fn clear_screen() -> io::Result<()> {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
    Ok(())
}

pub fn hide_cursor() -> io::Result<()> {
    execute!(stdout(), Hide)?;
    Ok(())
}

pub fn show_cursor() -> io::Result<()> {
    execute!(stdout(), Show)?;
    Ok(())
}

pub fn print_header(text: &str, color: Color) {
    let width = terminal::size().unwrap_or((80, 24)).0 as usize;
    let text_len = text.len();
    let padding = (width.saturating_sub(text_len)) / 2;
    
    println!("\n{}", "═".repeat(width).color(color));
    println!("{:padding$}{}", "", text.color(color).bold(), padding = padding);
    println!("{}\n", "═".repeat(width).color(color));
}

pub fn print_progress_bar(current: usize, total: usize, width: usize, color: Color) {
    let percentage = (current as f32 / total as f32 * 100.0) as u32;
    let filled = (current as f32 / total as f32 * width as f32) as usize;
    let empty = width.saturating_sub(filled);
    
    let bar = format!(
        "[{}{}] {}/{}  ({}%)",
        "█".repeat(filled).color(color),
        "░".repeat(empty),
        current,
        total,
        percentage
    );
    
    println!("{}", bar);
}

pub fn print_ascii_dna() {
    let dna = r#"
        ╔══════════════════════════════════════╗
        ║     🧬 MOLECULAR BIOLOGY QUIZ 🧬     ║
        ╚══════════════════════════════════════╝
              
             ∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙
            ╱◉────◉╲╱◉────◉╲╱◉────◉╲
           ╱────────╲────────╲────────╲
          ╱◉────────◉────────◉────────◉╲
         ╱────◉╲╱────◉╲╱────◉╲╱────◉╲╱─╲
        ╱──────╲──────╲──────╲──────╲───╲
       ╱◉──────◉──────◉──────◉──────◉────╲
      ╱──◉╲╱────◉╲╱────◉╲╱────◉╲╱────◉╲╱──╲
     ∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙
    "#;
    
    // Print with gradient colors
    let lines: Vec<&str> = dna.lines().collect();
    let colors = vec![
        Color::Blue,
        Color::Cyan,
        Color::BrightBlue,
        Color::BrightCyan,
    ];
    
    for (i, line) in lines.iter().enumerate() {
        let color_index = i % colors.len();
        println!("{}", line.color(colors[color_index]));
    }
}

pub fn print_section_banner(section_name: &str, icon: &str, color: Color) {
    let width = 50;
    println!("\n{}", "▀".repeat(width).color(color));
    println!(
        "{}  {}  {}  {}",
        icon.color(color).bold(),
        section_name.to_uppercase().color(color).bold(),
        icon.color(color).bold(),
        " ".repeat(width.saturating_sub(section_name.len() + 6))
    );
    println!("{}\n", "▄".repeat(width).color(color));
}

pub fn animated_loading(message: &str, duration_ms: u64) {
    let frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let colors = vec![
        Color::Red,
        Color::Yellow,
        Color::Green,
        Color::Cyan,
        Color::Blue,
        Color::Magenta,
    ];
    
    hide_cursor().ok();
    
    for i in 0..(duration_ms / 100) {
        let frame = frames[(i as usize) % frames.len()];
        let color = colors[(i as usize) % colors.len()];
        print!("\r{}  {}", frame.color(color), message);
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    print!("\r{}\n", " ".repeat(message.len() + 4));
    show_cursor().ok();
}

pub fn print_score_stars(score: f64) {
    let stars = match score as u32 {
        90..=100 => "⭐⭐⭐⭐⭐",
        80..=89 => "⭐⭐⭐⭐",
        70..=79 => "⭐⭐⭐",
        60..=69 => "⭐⭐",
        50..=59 => "⭐",
        _ => "☆",
    };
    
    println!("{}", stars);
}

pub fn wait_for_enter() {
    println!("\n{}", "Press ENTER to continue...".italic().dimmed());
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
}

pub fn print_separator(color: Color) {
    let width = terminal::size().unwrap_or((80, 24)).0 as usize;
    println!("{}", "─".repeat(width).color(color));
}

pub fn print_centered(text: &str, color: Color) {
    let width = terminal::size().unwrap_or((80, 24)).0 as usize;
    let padding = (width.saturating_sub(text.len())) / 2;
    println!("{:padding$}{}", "", text.color(color), padding = padding);
}