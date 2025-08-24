use crate::ui::colors::{rainbow_text, SectionThemes};
use crate::ui::display::{clear_screen, print_ascii_dna, print_centered, print_separator};
use colored::*;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal,
};
use std::io::{self, stdout, Write};
use std::time::Duration;

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
        print_centered("Use ↑↓ arrows or number keys (1-7), ENTER to select, Q to quit", Color::BrightBlack);
        println!("{}\n", rainbow_text("═══════════════════════════════════════════════════"));
        
        for (index, item) in self.sections.iter().enumerate() {
            self.display_item(index, item);
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
    
    fn display_item(&self, index: usize, item: &MenuItem) {
        let theme = self.theme.get_theme(&item.id);
        let is_selected = index == self.selected_index;
        let number = index + 1;
        
        if is_selected {
            // Highlighted selection with animated arrow
            println!(
                "  {} {} {} {} {}",
                "►".color(Color::BrightYellow).bold(),
                format!("{}.", number).color(Color::BrightWhite),
                item.icon,
                item.title.color(theme.primary).bold().underline(),
                format!("[{} questions]", item.question_count)
                    .color(Color::BrightGreen)
                    .italic()
            );
            println!(
                "         {}",
                item.description.color(theme.secondary).italic()
            );
        } else {
            // Normal display
            println!(
                "    {} {} {} {}",
                format!("{}.", number).color(Color::BrightBlack),
                item.icon,
                item.title.color(theme.primary),
                format!("[{} questions]", item.question_count)
                    .color(Color::BrightBlack)
                    .dimmed()
            );
            println!(
                "         {}",
                item.description.color(Color::BrightBlack).dimmed()
            );
        }
        println!();
    }
    
    pub fn run(&mut self) -> io::Result<Option<String>> {
        // Clear any pending input and add a small delay
        std::thread::sleep(Duration::from_millis(100));
        
        // Try raw mode first
        let use_raw_mode = terminal::enable_raw_mode().is_ok();
        
        if use_raw_mode {
            // Clear any pending events
            while event::poll(Duration::from_millis(0))? {
                event::read()?;
            }
            
            let result = self.run_with_arrows();
            terminal::disable_raw_mode()?;
            result
        } else {
            // Fallback to number selection
            self.run_with_numbers()
        }
    }
    
    fn run_with_arrows(&mut self) -> io::Result<Option<String>> {
        // Display initial menu
        self.display()?;
        
        loop {
            // Wait for a key press
            if let Event::Key(key_event) = event::read()? {
                // Only process key press events, not key release or repeat
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }
                
                let previous_index = self.selected_index;
                
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
                    KeyCode::Char('1') => {
                        return Ok(Some(self.sections[0].id.clone()));
                    }
                    KeyCode::Char('2') => {
                        return Ok(Some(self.sections[1].id.clone()));
                    }
                    KeyCode::Char('3') => {
                        return Ok(Some(self.sections[2].id.clone()));
                    }
                    KeyCode::Char('4') => {
                        return Ok(Some(self.sections[3].id.clone()));
                    }
                    KeyCode::Char('5') => {
                        return Ok(Some(self.sections[4].id.clone()));
                    }
                    KeyCode::Char('6') => {
                        return Ok(Some(self.sections[5].id.clone()));
                    }
                    KeyCode::Char('7') => {
                        return Ok(Some(self.sections[6].id.clone()));
                    }
                    KeyCode::Enter => {
                        let selected = self.sections[self.selected_index].id.clone();
                        return Ok(Some(selected));
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        return Ok(None);
                    }
                    _ => continue,
                }
                
                // Only redraw if selection changed
                if previous_index != self.selected_index {
                    self.update_display(previous_index)?;
                }
            }
        }
    }
    
    fn update_display(&self, previous_index: usize) -> io::Result<()> {
        // Calculate the line positions for each menu item
        // The header takes about 15 lines, then each item takes 3 lines
        let base_line = 15;
        
        // Move to and redraw the previous selection (unselected)
        let prev_line = base_line + (previous_index * 3);
        execute!(stdout(), MoveTo(0, prev_line as u16))?;
        
        // Clear the lines for the previous item
        for _ in 0..3 {
            print!("{}", " ".repeat(80));
            println!();
        }
        
        // Move back and redraw the previous item
        execute!(stdout(), MoveTo(0, prev_line as u16))?;
        self.display_item(previous_index, &self.sections[previous_index]);
        
        // Move to and redraw the current selection (selected)
        let curr_line = base_line + (self.selected_index * 3);
        execute!(stdout(), MoveTo(0, curr_line as u16))?;
        
        // Clear the lines for the current item
        for _ in 0..3 {
            print!("{}", " ".repeat(80));
            println!();
        }
        
        // Move back and redraw the current item
        execute!(stdout(), MoveTo(0, curr_line as u16))?;
        self.display_item(self.selected_index, &self.sections[self.selected_index]);
        
        // Move cursor to bottom
        execute!(stdout(), MoveTo(0, 40))?;
        stdout().flush()?;
        
        Ok(())
    }
    
    fn run_with_numbers(&mut self) -> io::Result<Option<String>> {
        loop {
            self.display()?;
            
            println!("\n{}", "Enter section number (1-7) or Q to quit: ".color(Color::BrightYellow));
            print!("> ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_lowercase();
            
            match input.as_str() {
                "1" => return Ok(Some(self.sections[0].id.clone())),
                "2" => return Ok(Some(self.sections[1].id.clone())),
                "3" => return Ok(Some(self.sections[2].id.clone())),
                "4" => return Ok(Some(self.sections[3].id.clone())),
                "5" => return Ok(Some(self.sections[4].id.clone())),
                "6" => return Ok(Some(self.sections[5].id.clone())),
                "7" => return Ok(Some(self.sections[6].id.clone())),
                "q" | "quit" => return Ok(None),
                _ => {
                    println!("{}", "Invalid selection. Please try again.".color(Color::BrightRed));
                    std::thread::sleep(Duration::from_secs(1));
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