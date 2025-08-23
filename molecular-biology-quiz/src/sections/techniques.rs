use crate::models::{Difficulty, Question};

pub fn get_questions() -> Vec<Question> {
    vec![
        Question::new(
            1,
            "What does PCR stand for?".to_string(),
            vec![
                "Protein Chain Reaction".to_string(),
                "Polymerase Chain Reaction".to_string(),
                "Primer Chain Reaction".to_string(),
                "Phosphate Chain Reaction".to_string(),
            ],
            1,
            "PCR (Polymerase Chain Reaction) is a technique to amplify specific DNA sequences exponentially.".to_string(),
            Difficulty::Easy,
            "techniques".to_string(),
        ),
        
        Question::new(
            2,
            "What temperature is typically used for denaturation in PCR?".to_string(),
            vec![
                "55°C".to_string(),
                "72°C".to_string(),
                "94-95°C".to_string(),
                "37°C".to_string(),
            ],
            2,
            "Denaturation typically occurs at 94-95°C to separate the DNA strands by breaking hydrogen bonds.".to_string(),
            Difficulty::Easy,
            "techniques".to_string(),
        ),
        
        Question::new(
            3,
            "What does CRISPR-Cas9 use to target specific DNA sequences?".to_string(),
            vec![
                "Protein recognition".to_string(),
                "Guide RNA".to_string(),
                "Antibodies".to_string(),
                "Restriction enzymes".to_string(),
            ],
            1,
            "CRISPR-Cas9 uses a guide RNA (gRNA) that base-pairs with the target DNA sequence for specific targeting.".to_string(),
            Difficulty::Medium,
            "techniques".to_string(),
        ),
        
        Question::new(
            4,
            "In gel electrophoresis, DNA fragments move toward which electrode?".to_string(),
            vec![
                "Positive (anode)".to_string(),
                "Negative (cathode)".to_string(),
                "Both electrodes".to_string(),
                "Neither electrode".to_string(),
            ],
            0,
            "DNA is negatively charged due to phosphate groups, so it moves toward the positive electrode (anode).".to_string(),
            Difficulty::Easy,
            "techniques".to_string(),
        ),
        
        Question::new(
            5,
            "What is the purpose of SDS in SDS-PAGE?".to_string(),
            vec![
                "To preserve protein structure".to_string(),
                "To denature proteins and provide uniform negative charge".to_string(),
                "To stain proteins".to_string(),
                "To crosslink proteins".to_string(),
            ],
            1,
            "SDS denatures proteins and coats them with negative charge proportional to their length, allowing separation by size.".to_string(),
            Difficulty::Medium,
            "techniques".to_string(),
        ).with_hint("SDS is a detergent that disrupts protein structure.".to_string()),
        
        Question::new(
            6,
            "What type of antibody is used in Western blotting as the primary antibody?".to_string(),
            vec![
                "One that binds to the secondary antibody".to_string(),
                "One specific to the protein of interest".to_string(),
                "One that produces light".to_string(),
                "One that binds to the membrane".to_string(),
            ],
            1,
            "The primary antibody specifically recognizes and binds to the target protein in Western blotting.".to_string(),
            Difficulty::Medium,
            "techniques".to_string(),
        ),
        
        Question::new(
            7,
            "What is the purpose of reverse transcriptase in RT-PCR?".to_string(),
            vec![
                "To amplify DNA".to_string(),
                "To convert RNA to cDNA".to_string(),
                "To denature proteins".to_string(),
                "To sequence DNA".to_string(),
            ],
            1,
            "Reverse transcriptase synthesizes complementary DNA (cDNA) from an RNA template, allowing RNA amplification.".to_string(),
            Difficulty::Medium,
            "techniques".to_string(),
        ),
        
        Question::new(
            8,
            "Which technique would you use to determine protein-protein interactions?".to_string(),
            vec![
                "PCR".to_string(),
                "Co-immunoprecipitation".to_string(),
                "Northern blot".to_string(),
                "DNA sequencing".to_string(),
            ],
            1,
            "Co-immunoprecipitation uses antibodies to pull down a protein and its interacting partners from cell lysates.".to_string(),
            Difficulty::Medium,
            "techniques".to_string(),
        ),
        
        Question::new(
            9,
            "What does FACS stand for?".to_string(),
            vec![
                "Fluorescence-Activated Cell Sorting".to_string(),
                "Fast Antibody Cell Screening".to_string(),
                "Flow Analysis Cell System".to_string(),
                "Fluorescent Antibody Cellular Staining".to_string(),
            ],
            0,
            "FACS (Fluorescence-Activated Cell Sorting) separates cells based on fluorescent markers using flow cytometry.".to_string(),
            Difficulty::Hard,
            "techniques".to_string(),
        ).with_hint("This technique sorts cells based on fluorescent properties.".to_string()),
        
        Question::new(
            10,
            "What is the purpose of a Northern blot?".to_string(),
            vec![
                "To detect specific DNA sequences".to_string(),
                "To detect specific RNA sequences".to_string(),
                "To detect specific proteins".to_string(),
                "To detect specific lipids".to_string(),
            ],
            1,
            "Northern blotting detects specific RNA sequences, useful for studying gene expression at the RNA level.".to_string(),
            Difficulty::Medium,
            "techniques".to_string(),
        ),
        
        Question::new(
            11,
            "What does ChIP stand for in molecular biology?".to_string(),
            vec![
                "Chromatin Immunoprecipitation".to_string(),
                "Chemical Inhibition Protocol".to_string(),
                "Chromosome Insertion Procedure".to_string(),
                "Cell Hybridization In Plate".to_string(),
            ],
            0,
            "ChIP (Chromatin Immunoprecipitation) identifies DNA sequences bound by specific proteins in living cells.".to_string(),
            Difficulty::Hard,
            "techniques".to_string(),
        ),
        
        Question::new(
            12,
            "What is the principle behind ELISA?".to_string(),
            vec![
                "DNA-DNA hybridization".to_string(),
                "Antibody-antigen interaction".to_string(),
                "RNA interference".to_string(),
                "Protein synthesis".to_string(),
            ],
            1,
            "ELISA (Enzyme-Linked Immunosorbent Assay) uses antibody-antigen interactions to detect specific proteins or antibodies.".to_string(),
            Difficulty::Medium,
            "techniques".to_string(),
        ),
        
        Question::new(
            13,
            "What does qPCR measure?".to_string(),
            vec![
                "DNA quality".to_string(),
                "Real-time amplification of DNA".to_string(),
                "Protein quantity".to_string(),
                "RNA splicing".to_string(),
            ],
            1,
            "qPCR (quantitative PCR) measures DNA amplification in real-time, allowing quantification of initial template amount.".to_string(),
            Difficulty::Medium,
            "techniques".to_string(),
        ),
        
        Question::new(
            14,
            "What is the PAM sequence in CRISPR?".to_string(),
            vec![
                "Protospacer Adjacent Motif required for Cas9 binding".to_string(),
                "Protein Activation Module".to_string(),
                "Primary Antibody Marker".to_string(),
                "Polymerase Activation Mechanism".to_string(),
            ],
            0,
            "PAM (Protospacer Adjacent Motif) is a short DNA sequence required for Cas9 to bind and cut the target DNA.".to_string(),
            Difficulty::Hard,
            "techniques".to_string(),
        ),
        
        Question::new(
            15,
            "What is the purpose of DNase treatment in RNA extraction?".to_string(),
            vec![
                "To degrade RNA".to_string(),
                "To remove contaminating DNA".to_string(),
                "To stabilize RNA".to_string(),
                "To precipitate RNA".to_string(),
            ],
            1,
            "DNase treatment removes contaminating genomic DNA from RNA preparations, ensuring RNA purity.".to_string(),
            Difficulty::Medium,
            "techniques".to_string(),
        ),
        
        Question::new(
            16,
            "Which technique is used for genome-wide association studies?".to_string(),
            vec![
                "Western blot".to_string(),
                "SNP arrays".to_string(),
                "Southern blot".to_string(),
                "PCR".to_string(),
            ],
            1,
            "SNP arrays analyze single nucleotide polymorphisms across the genome to identify disease-associated variants.".to_string(),
            Difficulty::Hard,
            "techniques".to_string(),
        ).with_hint("GWAS looks for genetic variations associated with diseases.".to_string()),
        
        Question::new(
            17,
            "What is the principle of FISH technique?".to_string(),
            vec![
                "Fluorescent in situ hybridization using labeled DNA probes".to_string(),
                "Flow cytometry with antibodies".to_string(),
                "Protein fluorescence imaging".to_string(),
                "Cell sorting by size".to_string(),
            ],
            0,
            "FISH uses fluorescent probes to detect specific DNA sequences in chromosomes or RNA in tissues.".to_string(),
            Difficulty::Medium,
            "techniques".to_string(),
        ),
        
        Question::new(
            18,
            "What is 2D gel electrophoresis used for?".to_string(),
            vec![
                "DNA sequencing".to_string(),
                "Protein separation by pI and molecular weight".to_string(),
                "RNA splicing analysis".to_string(),
                "Chromosome separation".to_string(),
            ],
            1,
            "2D gel electrophoresis separates proteins first by isoelectric point, then by molecular weight, creating a protein map.".to_string(),
            Difficulty::Hard,
            "techniques".to_string(),
        ),
        
        Question::new(
            19,
            "What enzyme is commonly used in PCR?".to_string(),
            vec![
                "DNA ligase".to_string(),
                "Restriction enzymes".to_string(),
                "Taq polymerase".to_string(),
                "Reverse transcriptase".to_string(),
            ],
            2,
            "Taq polymerase, from Thermus aquaticus, is heat-stable and commonly used in PCR for DNA synthesis.".to_string(),
            Difficulty::Easy,
            "techniques".to_string(),
        ),
        
        Question::new(
            20,
            "What is next-generation sequencing (NGS)?".to_string(),
            vec![
                "Sanger sequencing method".to_string(),
                "High-throughput parallel sequencing technologies".to_string(),
                "Protein sequencing".to_string(),
                "RNA structure determination".to_string(),
            ],
            1,
            "NGS refers to high-throughput sequencing technologies that can sequence millions of DNA fragments simultaneously.".to_string(),
            Difficulty::Medium,
            "techniques".to_string(),
        ),
    ]
}