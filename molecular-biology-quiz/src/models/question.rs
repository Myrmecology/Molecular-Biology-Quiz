use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "⭐"),
            Difficulty::Medium => write!(f, "⭐⭐"),
            Difficulty::Hard => write!(f, "⭐⭐⭐"),
            Difficulty::Expert => write!(f, "⭐⭐⭐⭐⭐"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: usize,
    pub text: String,
    pub options: Vec<String>,
    pub correct_answer: usize,  // Index of correct option
    pub explanation: String,
    pub difficulty: Difficulty,
    pub category: String,
    pub hint: Option<String>,
}

impl Question {
    pub fn new(
        id: usize,
        text: String,
        options: Vec<String>,
        correct_answer: usize,
        explanation: String,
        difficulty: Difficulty,
        category: String,
    ) -> Self {
        Self {
            id,
            text,
            options,
            correct_answer,
            explanation,
            difficulty,
            category,
            hint: None,
        }
    }

    pub fn with_hint(mut self, hint: String) -> Self {
        self.hint = Some(hint);
        self
    }

    pub fn is_correct(&self, answer: usize) -> bool {
        answer == self.correct_answer
    }

    pub fn get_correct_answer_text(&self) -> &str {
        &self.options[self.correct_answer]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizResult {
    pub total_questions: usize,
    pub correct_answers: usize,
    pub incorrect_answers: usize,
    pub score: f64,
    pub time_taken: Option<std::time::Duration>,
    pub category: String,
    pub difficulty_breakdown: Vec<(Difficulty, usize, usize)>, // (difficulty, correct, total)
}

impl QuizResult {
    pub fn new(category: String) -> Self {
        Self {
            total_questions: 0,
            correct_answers: 0,
            incorrect_answers: 0,
            score: 0.0,
            time_taken: None,
            category,
            difficulty_breakdown: Vec::new(),
        }
    }

    pub fn calculate_percentage(&self) -> f64 {
        if self.total_questions == 0 {
            0.0
        } else {
            (self.correct_answers as f64 / self.total_questions as f64) * 100.0
        }
    }

    pub fn get_grade(&self) -> &str {
        let percentage = self.calculate_percentage();
        match percentage as u32 {
            90..=100 => "A",
            80..=89 => "B",
            70..=79 => "C",
            60..=69 => "D",
            _ => "F",
        }
    }
}