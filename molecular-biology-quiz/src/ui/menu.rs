use crate::ui::colors::{rainbow_text, SectionThemes};
use crate::ui::display::{clear_screen, print_ascii_dna, print_centered, print_separator};
use colored::*;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal,
};
use std::io;

pub struct MainMenu {
    sections: Vec<MenuItem>,
    selected_index: usize,
    theme: SectionThemes,
}

pub struct MenuItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub question_count: usize,
}

impl MainMenu {
    pub fn new() -> Self {
        let sections = vec![
            MenuItem {
                id: "dna_rna".to_string(),
                title: "DNA & RNA".to_string(),
                description: "Structure, replication, transcription".to_string(),
                icon: "🧬".to_string(),
                question_count: 20,
            },
            MenuItem {
                id: "proteins".to_string(),
                title: "Proteins & Enzymes".to_string(),
                description: "Amino acids, folding, enzyme kinetics".to_string(),
                icon: "🔬".to_string(),
                question_count: 20,
            },
            MenuItem {
                id: "cell_division".to_string(),
                title: "Cell Division".to_string(),
                description: "Mitosis, meiosis, cell cycle".to_string(),
                icon: "🦠".to_string(),
                question_count: 20,
            },
            MenuItem {
                id: "gene_expression".to_string(),
                title: "Gene Expression".to_string(),
                description: "Regulation, operons, epigenetics".to_string(),
                icon: "🧫".to_string(),
                question_count: 20,
            },
            MenuItem {
                id: "respiration".to_string(),
                title: "Cellular Respiration".to_string(),
                description: "Glycolysis, Krebs cycle, ATP".to_string(),
                icon: "⚡".to_string(),
                question_count: 20,
            },
            MenuItem {
                id: "techniques".to_string(),
                title: "Molecular Techniques".to_string(),
                description: "PCR, CRISPR, Western blot".to_string(),
                icon: "🧪".to_string(),
                question_count: 20,
            },
            MenuItem {
                id: "signaling".to_string(),
                title: "Cell Signaling".to_string(),
                description: "Receptors, cascades, messengers".to_string(),
                icon: "📡".to_string(),
                question_count: 20,
            },
        ];
        
        Self {
            sections,
            selected_index: 0,
            theme: SectionThemes::new(),
        }
    }
    
    pub fn display(&self) -> io::Result<()> {
        clear_screen()?;
        print_ascii_dna();
        
        println!("\n{}", rainbow_text("═══════════════════════════════════════════════════"));
        print_centered("SELECT YOUR QUIZ SECTION", Color::BrightWhite);
        print_centered("Use ↑↓ arrows to navigate, ENTER to select, Q to quit", Color::BrightBlack);
        println!("{}\n", rainbow_text("═══════════════════════════════════════════════════"));
        
        for (index, item) in self.sections.iter().enumerate() {
            let theme = self.theme.get_theme(&item.id);
            let is_selected = index == self.selected_index;
            
            if is_selected {
                // Highlighted selection with animated arrow
                println!(
                    "  {} {} {} {}",
                    "►".color(Color::BrightYellow).bold(),
                    item.icon,
                    item.title.color(theme.primary).bold().underline(),
                    format!("[{} questions]", item.question_count)
                        .color(Color::BrightGreen)
                        .italic()
                );
                println!(
                    "       {}",
                    item.description.color(theme.secondary).italic()
                );
                println!();
            } else {
                // Normal display
                println!(
                    "    {} {} {}",
                    item.icon,
                    item.title.color(theme.primary),
                    format!("[{} questions]", item.question_count)
                        .color(Color::BrightBlack)
                        .dimmed()
                );
                println!(
                    "       {}",
                    item.description.color(Color::BrightBlack).dimmed()
                );
                println!();
            }
        }
        
        print_separator(Color::BrightBlack);
        println!(
            "\n{}",
            "🎯 Ready to test your molecular biology knowledge? 🎯"
                .color(Color::BrightCyan)
                .bold()
        );
        
        Ok(())
    }
    
    pub fn run(&mut self) -> io::Result<Option<String>> {
        terminal::enable_raw_mode()?;
        
        loop {
            self.display()?;
            
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => {
                        if self.selected_index > 0 {
                            self.selected_index -= 1;
                        } else {
                            self.selected_index = self.sections.len() - 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.selected_index < self.sections.len() - 1 {
                            self.selected_index += 1;
                        } else {
                            self.selected_index = 0;
                        }
                    }
                    KeyCode::Enter => {
                        let selected = self.sections[self.selected_index].id.clone();
                        terminal::disable_raw_mode()?;
                        return Ok(Some(selected));
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        terminal::disable_raw_mode()?;
                        return Ok(None);
                    }
                    _ => {}
                }
            }
        }
    }
    
    pub fn get_section_info(&self, section_id: &str) -> Option<&MenuItem> {
        self.sections.iter().find(|s| s.id == section_id)
    }
}

pub fn display_welcome_screen() -> io::Result<()> {
    clear_screen()?;
    
    let welcome_text = r#"
    ╔════════════════════════════════════════════════╗
    ║                                                ║
    ║     Welcome to MOLECULAR BIOLOGY QUIZ! 🧬     ║
    ║                                                ║
    ║      Test your knowledge across 7 topics      ║
    ║         with over 140 questions total!        ║
    ║                                                ║
    ╚════════════════════════════════════════════════╝
    "#;
    
    println!("{}", rainbow_text(welcome_text));
    
    println!("\n\n{}", "Features:".color(Color::BrightCyan).bold());
    println!("  {} Colorful, interactive terminal interface", "•".color(Color::BrightGreen));
    println!("  {} 7 comprehensive molecular biology sections", "•".color(Color::BrightGreen));
    println!("  {} Difficulty levels from Easy to Expert", "•".color(Color::BrightGreen));
    println!("  {} Detailed explanations for every answer", "•".color(Color::BrightGreen));
    println!("  {} Score tracking and performance analytics", "•".color(Color::BrightGreen));
    println!("  {} Hint system for challenging questions", "•".color(Color::BrightGreen));
    
    print_separator(Color::BrightBlack);
    
    Ok(())
}