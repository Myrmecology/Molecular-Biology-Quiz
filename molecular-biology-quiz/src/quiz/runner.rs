use crate::models::{Question, QuizResult};
use crate::quiz::scoring::{ScoreCalculator, StreakTracker};
use crate::ui::colors::SectionThemes;
use crate::ui::display::{
    animated_loading, clear_screen, print_centered, print_header,
    print_progress_bar, print_section_banner, wait_for_enter,
};
use colored::*;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use rand::seq::SliceRandom;
use std::io;
use std::time::{Duration, Instant};

pub struct QuizRunner {
    questions: Vec<Question>,
    current_index: usize,
    result: QuizResult,
    score_calculator: ScoreCalculator,
    streak_tracker: StreakTracker,
    theme: SectionThemes,
    section_id: String,
    quiz_start_time: Instant,
    used_hints: Vec<usize>,
}

impl QuizRunner {
    pub fn new(mut questions: Vec<Question>, section_id: String, section_name: String) -> Self {
        // Shuffle questions for randomization
        let mut rng = rand::thread_rng();
        questions.shuffle(&mut rng);
        
        Self {
            questions,
            current_index: 0,
            result: QuizResult::new(section_name),
            score_calculator: ScoreCalculator::new(),
            streak_tracker: StreakTracker::new(),
            theme: SectionThemes::new(),
            section_id,
            quiz_start_time: Instant::now(),
            used_hints: Vec::new(),
        }
    }
    
    pub fn run(&mut self) -> io::Result<QuizResult> {
        terminal::enable_raw_mode()?;
        
        // Show loading animation
        terminal::disable_raw_mode()?;
        clear_screen()?;
        animated_loading("Loading quiz questions...", 1500);
        terminal::enable_raw_mode()?;
        
        while self.current_index < self.questions.len() {
            self.display_question()?;
            
            if let Some(answer) = self.get_user_answer()? {
                if answer == usize::MAX {
                    // User wants to quit
                    break;
                }
                
                let is_correct = self.process_answer(answer)?;
                self.display_feedback(is_correct, answer)?;
                
                terminal::disable_raw_mode()?;
                wait_for_enter();
                terminal::enable_raw_mode()?;
                
                self.current_index += 1;
            }
        }
        
        terminal::disable_raw_mode()?;
        
        // Set total time taken
        self.result.time_taken = Some(self.quiz_start_time.elapsed());
        
        // Display final results
        clear_screen()?;
        println!("{}", self.score_calculator.format_score_display(&self.result));
        
        Ok(self.result.clone())
    }
    
    fn display_question(&self) -> io::Result<()> {
        clear_screen()?;
        
        let question = &self.questions[self.current_index];
        let theme = self.theme.get_theme(&self.section_id);
        
        // Header with progress
        print_header(
            &format!("Question {} of {}", self.current_index + 1, self.questions.len()),
            theme.primary,
        );
        
        // Progress bar
        print_progress_bar(
            self.current_index + 1,
            self.questions.len(),
            50,
            theme.accent,
        );
        
        // Score and streak display
        println!("\n{} {} | {} | {}",
            "Score:".color(Color::White).bold(),
            format!("{:.0}", self.result.score).color(Color::BrightGreen).bold(),
            self.streak_tracker.display_streak(),
            format!("Difficulty: {}", question.difficulty).color(theme.info)
        );
        
        // Question text
        println!("\n{}", "━".repeat(60).color(theme.secondary));
        println!("\n{}\n", question.text.color(Color::BrightWhite).bold());
        println!("{}", "━".repeat(60).color(theme.secondary));
        
        // Options
        println!("\n{}", "Options:".color(theme.accent).bold());
        for (i, option) in question.options.iter().enumerate() {
            let letter = match i {
                0 => "A",
                1 => "B",
                2 => "C",
                3 => "D",
                _ => "?",
            };
            
            // Check if hint was used and this option was eliminated
            let is_eliminated = self.used_hints.contains(&self.current_index) 
                && i != question.correct_answer 
                && i == 0; // Eliminate first wrong answer as hint
            
            if is_eliminated {
                println!(
                    "  {} {}",
                    format!("[{}]", letter).color(Color::BrightBlack).dimmed().strikethrough(),
                    option.color(Color::BrightBlack).dimmed().strikethrough()
                );
            } else {
                println!(
                    "  {} {}",
                    format!("[{}]", letter).color(theme.primary).bold(),
                    option.color(Color::White)
                );
            }
        }
        
        // Instructions
        println!("\n{}", "─".repeat(60).color(Color::BrightBlack));
        println!(
            "{}",
            "Press A, B, C, or D to answer | H for hint | Q to quit"
                .color(Color::BrightBlack)
                .italic()
        );
        
        // Show if hint is available
        if question.hint.is_some() && !self.used_hints.contains(&self.current_index) {
            println!(
                "{}",
                "💡 Hint available (costs 5 points)".color(Color::BrightYellow).dimmed()
            );
        }
        
        Ok(())
    }
    
    fn get_user_answer(&mut self) -> io::Result<Option<usize>> {
        loop {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('a') | KeyCode::Char('A') => return Ok(Some(0)),
                    KeyCode::Char('b') | KeyCode::Char('B') => return Ok(Some(1)),
                    KeyCode::Char('c') | KeyCode::Char('C') => return Ok(Some(2)),
                    KeyCode::Char('d') | KeyCode::Char('D') => return Ok(Some(3)),
                    KeyCode::Char('h') | KeyCode::Char('H') => {
                        self.use_hint()?;
                        return Ok(None);
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        return Ok(Some(usize::MAX))
                    }
                    _ => {}
                }
            }
        }
    }
    
    fn use_hint(&mut self) -> io::Result<()> {
        let question = &self.questions[self.current_index];
        
        if self.used_hints.contains(&self.current_index) {
            // Already used hint for this question
            return Ok(());
        }
        
        if let Some(hint_text) = &question.hint {
            self.used_hints.push(self.current_index);
            self.result.score = (self.result.score - 5.0).max(0.0);
            
            terminal::disable_raw_mode()?;
            println!("\n{}", "💡 HINT:".color(Color::BrightYellow).bold());
            println!("{}", hint_text.color(Color::BrightYellow));
            println!("{}", "(-5 points)".color(Color::Red).italic());
            wait_for_enter();
            terminal::enable_raw_mode()?;
            
            self.display_question()?;
        }
        
        Ok(())
    }
    
    fn process_answer(&mut self, answer_index: usize) -> io::Result<bool> {
        let question = &self.questions[self.current_index];
        let question_time = Duration::from_secs(10); // Placeholder for actual timing
        
        let is_correct = question.is_correct(answer_index);
        
        self.result.total_questions += 1;
        
        if is_correct {
            self.result.correct_answers += 1;
            self.streak_tracker.update(true);
            
            let points = self.score_calculator.calculate_question_score(
                &question.difficulty,
                question_time,
                self.streak_tracker.get_current_streak(),
            );
            
            self.result.score += points as f64;
        } else {
            self.result.incorrect_answers += 1;
            self.streak_tracker.update(false);
        }
        
        Ok(is_correct)
    }
    
    fn display_feedback(&self, is_correct: bool, user_answer: usize) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        
        let question = &self.questions[self.current_index];
        let theme = self.theme.get_theme(&self.section_id);
        
        println!("\n{}", "═".repeat(60).color(Color::BrightWhite));
        
        if is_correct {
            println!(
                "\n{} {}\n",
                "✓".color(Color::BrightGreen).bold(),
                "CORRECT!".color(Color::BrightGreen).bold()
            );
            
            if self.streak_tracker.get_current_streak() >= 3 {
                println!("{}", self.streak_tracker.display_streak());
            }
        } else {
            println!(
                "\n{} {}\n",
                "✗".color(Color::BrightRed).bold(),
                "INCORRECT".color(Color::BrightRed).bold()
            );
            
            println!(
                "{} {}",
                "Your answer:".color(Color::White),
                question.options[user_answer].color(Color::BrightRed)
            );
            
            println!(
                "{} {}",
                "Correct answer:".color(Color::White),
                question.get_correct_answer_text().color(Color::BrightGreen).bold()
            );
        }
        
        // Show explanation
        println!("\n{}", "Explanation:".color(theme.info).bold());
        println!("{}", question.explanation.color(Color::White));
        
        println!("\n{}", "═".repeat(60).color(Color::BrightWhite));
        
        terminal::enable_raw_mode()?;
        Ok(())
    }
}