use crate::models::QuizResult;
use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedQuizResult {
    pub result: QuizResult,
    pub timestamp: DateTime<Local>,
    pub player_name: Option<String>,
}

pub struct FileHandler {
    save_directory: PathBuf,
}

impl FileHandler {
    pub fn new() -> Self {
        let save_directory = PathBuf::from("quiz_saves");
        Self { save_directory }
    }
    
    pub fn ensure_save_directory(&self) -> Result<()> {
        if !self.save_directory.exists() {
            fs::create_dir_all(&self.save_directory)
                .context("Failed to create save directory")?;
        }
        Ok(())
    }
    
    pub fn save_quiz_result(&self, result: &QuizResult, player_name: Option<String>) -> Result<()> {
        self.ensure_save_directory()?;
        
        let saved_result = SavedQuizResult {
            result: result.clone(),
            timestamp: Local::now(),
            player_name,
        };
        
        let filename = format!(
            "quiz_{}_{}.json",
            result.category.replace(' ', "_").to_lowercase(),
            Local::now().format("%Y%m%d_%H%M%S")
        );
        
        let filepath = self.save_directory.join(filename);
        let json = serde_json::to_string_pretty(&saved_result)
            .context("Failed to serialize quiz result")?;
        
        fs::write(&filepath, json)
            .context("Failed to write quiz result to file")?;
        
        println!(
            "\n{} Score saved to: {}",
            "✓".to_string(),
            filepath.display()
        );
        
        Ok(())
    }
    
    pub fn load_high_scores(&self) -> Result<Vec<SavedQuizResult>> {
        self.ensure_save_directory()?;
        
        let mut all_results = Vec::new();
        
        let entries = fs::read_dir(&self.save_directory)
            .context("Failed to read save directory")?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let contents = fs::read_to_string(&path)
                    .context("Failed to read save file")?;
                
                if let Ok(saved_result) = serde_json::from_str::<SavedQuizResult>(&contents) {
                    all_results.push(saved_result);
                }
            }
        }
        
        // Sort by score (highest first)
        all_results.sort_by(|a, b| {
            b.result.score.partial_cmp(&a.result.score).unwrap()
        });
        
        Ok(all_results)
    }
    
    pub fn load_section_high_scores(&self, section: &str) -> Result<Vec<SavedQuizResult>> {
        let all_scores = self.load_high_scores()?;
        
        let section_scores: Vec<SavedQuizResult> = all_scores
            .into_iter()
            .filter(|s| s.result.category.to_lowercase().contains(&section.to_lowercase()))
            .collect();
        
        Ok(section_scores)
    }
    
    pub fn display_high_scores(&self, limit: usize) -> Result<()> {
        let scores = self.load_high_scores()?;
        
        if scores.is_empty() {
            println!("\nNo saved scores yet!");
            return Ok(());
        }
        
        println!("\n🏆 HIGH SCORES 🏆");
        println!("{}", "═".repeat(60));
        
        for (i, saved) in scores.iter().take(limit).enumerate() {
            let name = saved.player_name.as_ref()
                .map(|n| n.as_str())
                .unwrap_or("Anonymous");
            
            println!(
                "{}. {} - {:.0} points ({}) - {}",
                i + 1,
                name,
                saved.result.score,
                saved.result.category,
                saved.timestamp.format("%Y-%m-%d %H:%M")
            );
            
            println!(
                "   {} correct, {}% accuracy",
                saved.result.correct_answers,
                saved.result.calculate_percentage() as u32
            );
        }
        
        println!("{}", "═".repeat(60));
        
        Ok(())
    }
    
    pub fn export_results_csv(&self, filepath: &Path) -> Result<()> {
        let scores = self.load_high_scores()?;
        
        let mut csv_content = String::from(
            "Timestamp,Player,Category,Score,Questions,Correct,Percentage,Grade\n"
        );
        
        for saved in scores {
            let name = saved.player_name.unwrap_or_else(|| "Anonymous".to_string());
            csv_content.push_str(&format!(
                "{},{},{},{:.0},{},{},{:.1},{}\n",
                saved.timestamp.format("%Y-%m-%d %H:%M:%S"),
                name,
                saved.result.category,
                saved.result.score,
                saved.result.total_questions,
                saved.result.correct_answers,
                saved.result.calculate_percentage(),
                saved.result.get_grade()
            ));
        }
        
        fs::write(filepath, csv_content)
            .context("Failed to write CSV file")?;
        
        println!("✓ Results exported to: {}", filepath.display());
        
        Ok(())
    }
    
    pub fn clear_all_saves(&self) -> Result<()> {
        if self.save_directory.exists() {
            let entries = fs::read_dir(&self.save_directory)?;
            
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    fs::remove_file(path)?;
                }
            }
            
            println!("✓ All saved scores have been cleared");
        }
        
        Ok(())
    }
}

impl Default for FileHandler {
    fn default() -> Self {
        Self::new()
    }
}