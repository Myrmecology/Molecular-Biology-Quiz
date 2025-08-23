# 🧬 Molecular Biology Quiz

An interactive, colorful terminal-based quiz application for testing molecular biology knowledge. Features 7 comprehensive sections with over 140 questions covering fundamental concepts in molecular biology.

## ✨ Features

- **🎨 Colorful Terminal Interface**: Vibrant, engaging UI with section-specific color themes
- **📚 7 Comprehensive Sections**: 
  - DNA & RNA
  - Proteins & Enzymes
  - Cell Division
  - Gene Expression
  - Cellular Respiration
  - Molecular Techniques
  - Cell Signaling
- **⭐ Difficulty Levels**: Questions range from Easy to Expert
- **📖 Detailed Explanations**: Learn from every answer with comprehensive explanations
- **💡 Hint System**: Get help on challenging questions (costs points)
- **🔥 Streak Tracking**: Bonus points for consecutive correct answers
- **📊 Score Analytics**: Track performance with detailed statistics
- **🏆 High Score System**: Save and view your best performances
- **📁 Export Results**: Export all results to CSV for analysis

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/Myrmecology/Molecular-Biology-Quiz.git
cd molecular-biology-quiz

Build the project:
cargo build --release

Run the quiz:
cargo run --release

🎮 How to Play

Start the Application: Run cargo run from the project directory
Main Menu Options:

Start Quiz: Begin a new quiz session
View High Scores: See top performances
Export Results: Save all results to CSV
Clear Scores: Reset all saved scores


Select a Section: Use arrow keys to navigate, Enter to select
Answer Questions: Press A, B, C, or D to answer
Use Hints: Press H for a hint (costs 5 points)
View Results: See detailed performance statistics after completing the quiz

🎯 Scoring System

Base Points by Difficulty:

Easy: 10 points
Medium: 20 points
Hard: 30 points
Expert: 50 points


Time Bonus: Answer quickly for extra points
Streak Bonus: 5 points per consecutive correct answer
Hint Penalty: -5 points per hint used

molecular-biology-quiz/
├── src/
│   ├── main.rs           # Application entry point
│   ├── models/           # Data structures
│   │   ├── mod.rs
│   │   └── question.rs   # Question and result models
│   ├── quiz/             # Quiz logic
│   │   ├── mod.rs
│   │   ├── runner.rs     # Quiz execution
│   │   └── scoring.rs    # Scoring calculations
│   ├── sections/         # Question databases
│   │   ├── mod.rs
│   │   ├── dna_rna.rs
│   │   ├── proteins.rs
│   │   ├── cell_division.rs
│   │   ├── gene_expression.rs
│   │   ├── respiration.rs
│   │   ├── techniques.rs
│   │   └── signaling.rs
│   ├── ui/               # User interface
│   │   ├── mod.rs
│   │   ├── colors.rs     # Color themes
│   │   ├── display.rs    # Display utilities
│   │   └── menu.rs       # Menu system
│   └── utils/            # Utilities
│       ├── mod.rs
│       └── file_handler.rs # Save/load functionality
├── Cargo.toml            # Dependencies
├── README.md             # This file
└── .gitignore           # Git ignore rules

📦 Dependencies

crossterm: Terminal manipulation
colored: Colored terminal output
termcolor: Additional color support
serde/serde_json: Serialization for save files
rand: Random question selection
anyhow: Error handling
chrono: Timestamp handling
unicode-width: Unicode support

📝 License
This project is licensed under the MIT License.

🐛 Known Issues

Terminal colors may not display correctly on some older Windows terminals
Unicode characters may not render properly without proper font support

Happy Learning! 🧬🔬🎓 and as always happy coding

MORE THINGS TO KNOW
Checking Code
in the terminal run the following 
cargo check
cargo clippy
cargo fmt

Running Tests
cargo test

Running with Debug Output
cargo run

Building for Release
cargo build --release
The optimized binary will be in target/release/molecular-biology-quiz

🙏 Acknowledgments

Questions based on standard molecular biology curriculum 🧬
Inspired by the need for interactive learning tools 🧬
Built with Rust for performance and reliability 🧬
Dedicated to my love for molecular biology 🔬


