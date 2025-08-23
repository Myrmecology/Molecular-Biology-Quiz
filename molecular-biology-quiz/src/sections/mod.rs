pub mod cell_division;
pub mod dna_rna;
pub mod gene_expression;
pub mod proteins;
pub mod respiration;
pub mod signaling;
pub mod techniques;

use crate::models::Question;

pub fn get_questions_for_section(section_id: &str) -> Vec<Question> {
    match section_id {
        "dna_rna" => dna_rna::get_questions(),
        "proteins" => proteins::get_questions(),
        "cell_division" => cell_division::get_questions(),
        "gene_expression" => gene_expression::get_questions(),
        "respiration" => respiration::get_questions(),
        "techniques" => techniques::get_questions(),
        "signaling" => signaling::get_questions(),
        _ => Vec::new(),
    }
}

pub fn get_section_name(section_id: &str) -> String {
    match section_id {
        "dna_rna" => "DNA & RNA".to_string(),
        "proteins" => "Proteins & Enzymes".to_string(),
        "cell_division" => "Cell Division".to_string(),
        "gene_expression" => "Gene Expression".to_string(),
        "respiration" => "Cellular Respiration".to_string(),
        "techniques" => "Molecular Techniques".to_string(),
        "signaling" => "Cell Signaling".to_string(),
        _ => "Unknown Section".to_string(),
    }
}

pub fn get_all_sections() -> Vec<(String, String, usize)> {
    vec![
        ("dna_rna".to_string(), "DNA & RNA".to_string(), 20),
        ("proteins".to_string(), "Proteins & Enzymes".to_string(), 20),
        ("cell_division".to_string(), "Cell Division".to_string(), 20),
        ("gene_expression".to_string(), "Gene Expression".to_string(), 20),
        ("respiration".to_string(), "Cellular Respiration".to_string(), 20),
        ("techniques".to_string(), "Molecular Techniques".to_string(), 20),
        ("signaling".to_string(), "Cell Signaling".to_string(), 20),
    ]
}