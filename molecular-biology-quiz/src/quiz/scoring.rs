use crate::models::{Difficulty, QuizResult};
use colored::*;
use std::collections::HashMap;
use std::time::Duration;

pub struct ScoreCalculator {
    base_points: HashMap<Difficulty, u32>,
    time_bonus_multiplier: f64,
    streak_bonus: u32,
}

impl ScoreCalculator {
    pub fn new() -> Self {
        let mut base_points = HashMap::new();
        base_points.insert(Difficulty::Easy, 10);
        base_points.insert(Difficulty::Medium, 20);
        base_points.insert(Difficulty::Hard, 30);
        base_points.insert(Difficulty::Expert, 50);
        
        Self {
            base_points,
            time_bonus_multiplier: 1.5,
            streak_bonus: 5,
        }
    }
    
    pub fn calculate_question_score(
        &self,
        difficulty: &Difficulty,
        time_taken: Duration,
        streak_count: u32,
    ) -> u32 {
        let base = *self.base_points.get(difficulty).unwrap_or(&10);
        
        // Time bonus: faster answers get more points
        let time_bonus = if time_taken.as_secs() < 10 {
            (base as f64 * 0.2) as u32
        } else if time_taken.as_secs() < 20 {
            (base as f64 * 0.1) as u32
        } else {
            0
        };
        
        // Streak bonus
        let streak_bonus = streak_count * self.streak_bonus;
        
        base + time_bonus + streak_bonus
    }
    
    pub fn format_score_display(&self, result: &QuizResult) -> String {
        let percentage = result.calculate_percentage();
        let grade = result.get_grade();
        
        let mut output = String::new();
        
        // Header
        output.push_str(&format!(
            "\n{}\n",
            "═══════════════════════════════════════════════════"
                .color(Color::BrightCyan)
        ));
        output.push_str(&format!(
            "{:^51}\n",
            "QUIZ COMPLETE! 🎉".color(Color::BrightYellow).bold()
        ));
        output.push_str(&format!(
            "{}\n\n",
            "═══════════════════════════════════════════════════"
                .color(Color::BrightCyan)
        ));
        
        // Main scores
        output.push_str(&format!(
            "📊 {} {}\n",
            "Final Score:".color(Color::BrightWhite).bold(),
            format!("{:.0} points", result.score)
                .color(Color::BrightGreen)
                .bold()
        ));
        
        output.push_str(&format!(
            "📈 {} {}\n",
            "Percentage:".color(Color::BrightWhite).bold(),
            self.format_percentage_with_color(percentage)
        ));
        
        output.push_str(&format!(
            "🏆 {} {}\n\n",
            "Grade:".color(Color::BrightWhite).bold(),
            self.format_grade_with_color(grade)
        ));
        
        // Statistics
        output.push_str(&format!(
            "{}\n",
            "─────────────────────────────────────────────────"
                .color(Color::BrightBlack)
        ));
        output.push_str(&format!(
            "📝 {}\n",
            "Statistics:".color(Color::BrightCyan).bold()
        ));
        
        output.push_str(&format!(
            "   {} {}/{}\n",
            "Questions:".color(Color::White),
            result.correct_answers.to_string().color(Color::BrightGreen),
            result.total_questions
        ));
        
        output.push_str(&format!(
            "   {} {}\n",
            "Correct:".color(Color::White),
            format!("{} ✓", result.correct_answers)
                .color(Color::BrightGreen)
        ));
        
        output.push_str(&format!(
            "   {} {}\n",
            "Incorrect:".color(Color::White),
            format!("{} ✗", result.incorrect_answers)
                .color(Color::BrightRed)
        ));
        
        if let Some(duration) = result.time_taken {
            let minutes = duration.as_secs() / 60;
            let seconds = duration.as_secs() % 60;
            output.push_str(&format!(
                "   {} {}m {}s\n",
                "Time:".color(Color::White),
                minutes,
                seconds
            ));
        }
        
        // Difficulty breakdown if available
        if !result.difficulty_breakdown.is_empty() {
            output.push_str(&format!(
                "\n{}\n",
                "Difficulty Breakdown:".color(Color::BrightCyan).bold()
            ));
            
            for (difficulty, correct, total) in &result.difficulty_breakdown {
                let percent = if *total > 0 {
                    (*correct as f64 / *total as f64) * 100.0
                } else {
                    0.0
                };
                
                output.push_str(&format!(
                    "   {} {}/{} ({:.0}%)\n",
                    format!("{:?}:", difficulty).color(Color::White),
                    correct.to_string().color(Color::BrightGreen),
                    total,
                    percent
                ));
            }
        }
        
        output.push_str(&format!(
            "{}\n",
            "═══════════════════════════════════════════════════"
                .color(Color::BrightCyan)
        ));
        
        // Performance message
        output.push_str(&format!("\n{}\n", self.get_performance_message(percentage)));
        
        output
    }
    
    fn format_percentage_with_color(&self, percentage: f64) -> String {
        let color = match percentage as u32 {
            90..=100 => Color::BrightGreen,
            80..=89 => Color::Green,
            70..=79 => Color::BrightYellow,
            60..=69 => Color::Yellow,
            _ => Color::Red,
        };
        
        format!("{:.1}%", percentage).color(color).bold().to_string()
    }
    
    fn format_grade_with_color(&self, grade: &str) -> String {
        let (color, stars) = match grade {
            "A" => (Color::BrightGreen, "⭐⭐⭐⭐⭐"),
            "B" => (Color::Green, "⭐⭐⭐⭐"),
            "C" => (Color::BrightYellow, "⭐⭐⭐"),
            "D" => (Color::Yellow, "⭐⭐"),
            _ => (Color::Red, "⭐"),
        };
        
        format!("{} {}", grade.color(color).bold(), stars)
    }
    
    fn get_performance_message(&self, percentage: f64) -> String {
        let message = match percentage as u32 {
            95..=100 => "🌟 EXCEPTIONAL! You're a molecular biology master! 🌟",
            90..=94 => "🎯 EXCELLENT! Outstanding knowledge demonstrated!",
            80..=89 => "💪 GREAT JOB! Strong understanding of the material!",
            70..=79 => "👍 GOOD WORK! Solid performance overall!",
            60..=69 => "📚 DECENT! Consider reviewing missed topics.",
            50..=59 => "💡 KEEP STUDYING! You're getting there!",
            _ => "📖 MORE PRACTICE NEEDED! Don't give up!",
        };
        
        let color = if percentage >= 70.0 {
            Color::BrightGreen
        } else if percentage >= 50.0 {
            Color::BrightYellow
        } else {
            Color::BrightRed
        };
        
        message.color(color).bold().to_string()
    }
}

pub struct StreakTracker {
    current_streak: u32,
    best_streak: u32,
    last_was_correct: bool,
}

impl StreakTracker {
    pub fn new() -> Self {
        Self {
            current_streak: 0,
            best_streak: 0,
            last_was_correct: false,
        }
    }
    
    pub fn update(&mut self, is_correct: bool) {
        if is_correct {
            if self.last_was_correct {
                self.current_streak += 1;
            } else {
                self.current_streak = 1;
            }
            
            if self.current_streak > self.best_streak {
                self.best_streak = self.current_streak;
            }
            
            self.last_was_correct = true;
        } else {
            self.current_streak = 0;
            self.last_was_correct = false;
        }
    }
    
    pub fn get_current_streak(&self) -> u32 {
        self.current_streak
    }
    
    pub fn get_best_streak(&self) -> u32 {
        self.best_streak
    }
    
    pub fn display_streak(&self) -> String {
        if self.current_streak > 0 {
            format!(
                "🔥 {} streak!",
                self.current_streak
            ).color(Color::BrightYellow).bold().to_string()
        } else {
            String::new()
        }
    }
}