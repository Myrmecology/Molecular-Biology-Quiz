pub mod colors;
pub mod display;
pub mod menu;

pub use colors::{ColorTheme, SectionThemes, rainbow_text};
pub use display::{
    animated_loading, clear_screen, print_ascii_dna, print_centered,
    print_header, print_progress_bar, print_score_stars, print_section_banner,
    print_separator, wait_for_enter,
};
pub use menu::{display_welcome_screen, MainMenu};