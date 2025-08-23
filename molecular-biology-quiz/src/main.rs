mod models;
mod quiz;
mod sections;
mod ui;
mod utils;

use anyhow::Result;
use colored::*;
use quiz::QuizRunner;
use sections::{get_questions_for_section, get_section_name};
use std::io::{self, Write};
use ui::{
    clear_screen, display_welcome_screen, print_centered, print_separator, wait_for_enter,
    MainMenu,
};
use utils::FileHandler;

fn main() -> Result<()> {
    // Set up color support for Windows
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();
    
    // Initialize file handler
    let file_handler = FileHandler::new();
    
    // Main application loop
    loop {
        clear_screen()?;
        display_welcome_screen()?;
        
        println!("\n{}", "MAIN MENU".color(Color::BrightCyan).bold());
        print_separator(Color::BrightBlack);
        println!("1. 📚 Start Quiz");
        println!("2. 🏆 View High Scores");
        println!("3. 📊 Export Results to CSV");
        println!("4. 🗑️  Clear All Saved Scores");
        println!("5. ❌ Exit");
        print_separator(Color::BrightBlack);
        
        print!("\n{} ", "Select an option (1-5):".color(Color::BrightYellow));
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim() {
            "1" => {
                // Start quiz
                if let Err(e) = run_quiz(&file_handler) {
                    eprintln!("Error running quiz: {}", e);
                    wait_for_enter();
                }
            }
            "2" => {
                // View high scores
                clear_screen()?;
                if let Err(e) = file_handler.display_high_scores(10) {
                    eprintln!("Error displaying scores: {}", e);
                }
                wait_for_enter();
            }
            "3" => {
                // Export to CSV
                clear_screen()?;
                print!("Enter filename for CSV export (e.g., results.csv): ");
                io::stdout().flush()?;
                
                let mut filename = String::new();
                io::stdin().read_line(&mut filename)?;
                let filename = filename.trim();
                
                if !filename.is_empty() {
                    let path = std::path::Path::new(filename);
                    if let Err(e) = file_handler.export_results_csv(path) {
                        eprintln!("Error exporting results: {}", e);
                    }
                }
                wait_for_enter();
            }
            "4" => {
                // Clear all saves
                clear_screen()?;
                print!(
                    "{} {} ",
                    "⚠️".color(Color::BrightYellow),
                    "Are you sure you want to clear all saved scores? (y/N):".color(Color::BrightRed)
                );
                io::stdout().flush()?;
                
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm)?;
                
                if confirm.trim().to_lowercase() == "y" {
                    if let Err(e) = file_handler.clear_all_saves() {
                        eprintln!("Error clearing saves: {}", e);
                    }
                }
                wait_for_enter();
            }
            "5" | "exit" | "quit" | "q" => {
                // Exit
                clear_screen()?;
                print_centered("Thanks for playing Molecular Biology Quiz! 🧬", Color::BrightCyan);
                print_centered("Keep learning and exploring! 🚀", Color::BrightGreen);
                println!("\n");
                break;
            }
            _ => {
                println!(
                    "{}",
                    "Invalid option. Please select 1-5.".color(Color::BrightRed)
                );
                wait_for_enter();
            }
        }
    }
    
    Ok(())
}

fn run_quiz(file_handler: &FileHandler) -> Result<()> {
    let mut menu = MainMenu::new();
    
    // Show section selection menu
    if let Some(section_id) = menu.run()? {
        let questions = get_questions_for_section(&section_id);
        
        if questions.is_empty() {
            println!(
                "{}",
                "No questions available for this section yet!".color(Color::BrightRed)
            );
            wait_for_enter();
            return Ok(());
        }
        
        // Get player name (optional)
        clear_screen()?;
        print_centered("📝 PLAYER INFORMATION", Color::BrightCyan);
        print_separator(Color::BrightBlack);
        print!("\nEnter your name (or press Enter to skip): ");
        io::stdout().flush()?;
        
        let mut player_name = String::new();
        io::stdin().read_line(&mut player_name)?;
        let player_name = player_name.trim();
        let player_name = if player_name.is_empty() {
            None
        } else {
            Some(player_name.to_string())
        };
        
        // Create and run quiz
        let section_name = get_section_name(&section_id);
        let mut runner = QuizRunner::new(questions, section_id, section_name);
        
        // Run the quiz and get results
        let result = runner.run()?;
        
        // Save the results
        if let Err(e) = file_handler.save_quiz_result(&result, player_name) {
            eprintln!("Warning: Could not save quiz result: {}", e);
        }
        
        // Ask if player wants to play again
        println!("\n{}", "─".repeat(60).color(Color::BrightBlack));
        print!(
            "{} ",
            "Would you like to play another quiz? (Y/n):".color(Color::BrightGreen)
        );
        io::stdout().flush()?;
        
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again)?;
        
        if play_again.trim().to_lowercase() != "n" {
            return run_quiz(file_handler);
        }
    }
    
    Ok(())
}

// Custom panic handler for better error display
fn setup_panic_handler() {
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("\n{}", "═".repeat(60).color(Color::BrightRed));
        eprintln!(
            "{} {}",
            "⚠️".color(Color::BrightYellow),
            "An unexpected error occurred!".color(Color::BrightRed).bold()
        );
        
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("Error: {}", s);
        }
        
        if let Some(location) = panic_info.location() {
            eprintln!(
                "Location: {}:{}",
                location.file(),
                location.line()
            );
        }
        
        eprintln!("{}", "═".repeat(60).color(Color::BrightRed));
        eprintln!("\nPress Enter to exit...");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
    }));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sections_have_questions() {
        let sections = vec![
            "dna_rna",
            "proteins",
            "cell_division",
            "gene_expression",
            "respiration",
            "techniques",
            "signaling",
        ];
        
        for section in sections {
            let questions = get_questions_for_section(section);
            assert!(!questions.is_empty(), "Section {} has no questions", section);
            assert_eq!(questions.len(), 20, "Section {} should have 20 questions", section);
        }
    }
    
    #[test]
    fn test_section_names() {
        assert_eq!(get_section_name("dna_rna"), "DNA & RNA");
        assert_eq!(get_section_name("proteins"), "Proteins & Enzymes");
        assert_eq!(get_section_name("unknown"), "Unknown Section");
    }
}
