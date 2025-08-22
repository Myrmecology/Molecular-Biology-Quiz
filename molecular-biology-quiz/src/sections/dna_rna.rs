use crate::models::{Difficulty, Question};

pub fn get_questions() -> Vec<Question> {
    vec![
        Question::new(
            1,
            "What type of bond holds the two strands of DNA together?".to_string(),
            vec![
                "Ionic bonds".to_string(),
                "Hydrogen bonds".to_string(),
                "Covalent bonds".to_string(),
                "Peptide bonds".to_string(),
            ],
            1,
            "Hydrogen bonds between complementary base pairs (A-T and G-C) hold the two DNA strands together. These bonds are weak enough to allow strand separation during replication and transcription.".to_string(),
            Difficulty::Easy,
            "dna_rna".to_string(),
        ).with_hint("Think about bonds that are strong enough to hold strands together but weak enough to separate when needed.".to_string()),
        
        Question::new(
            2,
            "Which enzyme is responsible for unwinding the DNA double helix during replication?".to_string(),
            vec![
                "DNA polymerase".to_string(),
                "RNA polymerase".to_string(),
                "Helicase".to_string(),
                "Ligase".to_string(),
            ],
            2,
            "Helicase unwinds the DNA double helix by breaking hydrogen bonds between base pairs, creating the replication fork.".to_string(),
            Difficulty::Medium,
            "dna_rna".to_string(),
        ),
        
        Question::new(
            3,
            "What is the complementary DNA sequence to 5'-ATCGGA-3'?".to_string(),
            vec![
                "3'-TAGCCT-5'".to_string(),
                "5'-TAGCCT-3'".to_string(),
                "3'-UAGCCU-5'".to_string(),
                "5'-ATCGGA-3'".to_string(),
            ],
            0,
            "DNA strands are antiparallel and complementary. A pairs with T, G pairs with C. The complement of 5'-ATCGGA-3' is 3'-TAGCCT-5'.".to_string(),
            Difficulty::Medium,
            "dna_rna".to_string(),
        ),
        
        Question::new(
            4,
            "Which sugar is found in RNA?".to_string(),
            vec![
                "Glucose".to_string(),
                "Deoxyribose".to_string(),
                "Ribose".to_string(),
                "Fructose".to_string(),
            ],
            2,
            "RNA contains ribose sugar, which has a hydroxyl (-OH) group at the 2' carbon. DNA contains deoxyribose, which lacks this hydroxyl group.".to_string(),
            Difficulty::Easy,
            "dna_rna".to_string(),
        ),
        
        Question::new(
            5,
            "What is the function of telomerase?".to_string(),
            vec![
                "To repair DNA mismatches".to_string(),
                "To add repetitive sequences to chromosome ends".to_string(),
                "To remove RNA primers".to_string(),
                "To synthesize RNA from DNA".to_string(),
            ],
            1,
            "Telomerase adds repetitive nucleotide sequences to the ends of chromosomes (telomeres), preventing chromosome shortening during replication.".to_string(),
            Difficulty::Hard,
            "dna_rna".to_string(),
        ).with_hint("This enzyme is particularly active in stem cells and cancer cells.".to_string()),
        
        Question::new(
            6,
            "Which type of RNA carries amino acids to the ribosome?".to_string(),
            vec![
                "mRNA".to_string(),
                "tRNA".to_string(),
                "rRNA".to_string(),
                "snRNA".to_string(),
            ],
            1,
            "Transfer RNA (tRNA) carries specific amino acids to the ribosome during protein synthesis. Each tRNA has an anticodon that pairs with mRNA codons.".to_string(),
            Difficulty::Easy,
            "dna_rna".to_string(),
        ),
        
        Question::new(
            7,
            "What is the name of the DNA segments that are removed during RNA splicing?".to_string(),
            vec![
                "Exons".to_string(),
                "Introns".to_string(),
                "Promoters".to_string(),
                "Enhancers".to_string(),
            ],
            1,
            "Introns are non-coding sequences that are removed during RNA splicing. Exons are the coding sequences that remain in the mature mRNA.".to_string(),
            Difficulty::Medium,
            "dna_rna".to_string(),
        ),
        
        Question::new(
            8,
            "Which direction does DNA polymerase synthesize new DNA?".to_string(),
            vec![
                "3' to 5'".to_string(),
                "5' to 3'".to_string(),
                "Both directions simultaneously".to_string(),
                "N-terminus to C-terminus".to_string(),
            ],
            1,
            "DNA polymerase synthesizes new DNA in the 5' to 3' direction, adding nucleotides to the 3' OH group of the growing strand.".to_string(),
            Difficulty::Medium,
            "dna_rna".to_string(),
        ),
        
        Question::new(
            9,
            "What is the approximate number of base pairs in the human genome?".to_string(),
            vec![
                "3 million".to_string(),
                "3 billion".to_string(),
                "30 billion".to_string(),
                "300 million".to_string(),
            ],
            1,
            "The human genome contains approximately 3 billion base pairs distributed across 23 pairs of chromosomes.".to_string(),
            Difficulty::Medium,
            "dna_rna".to_string(),
        ),
        
        Question::new(
            10,
            "Which repair mechanism fixes thymine dimers caused by UV radiation?".to_string(),
            vec![
                "Base excision repair".to_string(),
                "Nucleotide excision repair".to_string(),
                "Mismatch repair".to_string(),
                "Direct repair".to_string(),
            ],
            1,
            "Nucleotide excision repair removes bulky DNA lesions like thymine dimers by cutting out a segment of damaged DNA and resynthesizing it.".to_string(),
            Difficulty::Hard,
            "dna_rna".to_string(),
        ),
        
        Question::new(
            11,
            "What is the function of the 5' cap in eukaryotic mRNA?".to_string(),
            vec![
                "To terminate transcription".to_string(),
                "To protect mRNA and aid in translation initiation".to_string(),
                "To splice out introns".to_string(),
                "To add poly-A tail".to_string(),
            ],
            1,
            "The 5' cap (7-methylguanosine) protects mRNA from degradation and helps ribosomes recognize and bind to the mRNA for translation.".to_string(),
            Difficulty::Medium,
            "dna_rna".to_string(),
        ),
        
        Question::new(
            12,
            "Which statement about DNA replication is correct?".to_string(),
            vec![
                "It is conservative".to_string(),
                "It is semi-conservative".to_string(),
                "It is dispersive".to_string(),
                "It is non-conservative".to_string(),
            ],
            1,
            "DNA replication is semi-conservative: each new double helix contains one original strand and one newly synthesized strand.".to_string(),
            Difficulty::Easy,
            "dna_rna".to_string(),
        ),
        
        Question::new(
            13,
            "What are Okazaki fragments?".to_string(),
            vec![
                "Fragments of RNA primers".to_string(),
                "Short DNA segments on the lagging strand".to_string(),
                "Broken pieces of chromosomes".to_string(),
                "Degraded mRNA pieces".to_string(),
            ],
            1,
            "Okazaki fragments are short DNA segments synthesized discontinuously on the lagging strand during DNA replication, later joined by DNA ligase.".to_string(),
            Difficulty::Medium,
            "dna_rna".to_string(),
        ),
        
        Question::new(
            14,
            "Which base is found in RNA but not in DNA?".to_string(),
            vec![
                "Adenine".to_string(),
                "Guanine".to_string(),
                "Cytosine".to_string(),
                "Uracil".to_string(),
            ],
            3,
            "Uracil replaces thymine in RNA. Both are pyrimidines, but uracil lacks the methyl group present in thymine.".to_string(),
            Difficulty::Easy,
            "dna_rna".to_string(),
        ),
        
        Question::new(
            15,
            "What is the role of primase in DNA replication?".to_string(),
            vec![
                "To unwind DNA".to_string(),
                "To synthesize RNA primers".to_string(),
                "To join Okazaki fragments".to_string(),
                "To remove RNA primers".to_string(),
            ],
            1,
            "Primase synthesizes short RNA primers that provide the 3'-OH group needed for DNA polymerase to begin DNA synthesis.".to_string(),
            Difficulty::Medium,
            "dna_rna".to_string(),
        ),
        
        Question::new(
            16,
            "How many hydrogen bonds form between adenine and thymine?".to_string(),
            vec![
                "One".to_string(),
                "Two".to_string(),
                "Three".to_string(),
                "Four".to_string(),
            ],
            1,
            "Adenine and thymine form two hydrogen bonds, while guanine and cytosine form three hydrogen bonds, making G-C pairs more stable.".to_string(),
            Difficulty::Easy,
            "dna_rna".to_string(),
        ),
        
        Question::new(
            17,
            "What is the function of topoisomerase?".to_string(),
            vec![
                "To synthesize DNA".to_string(),
                "To relieve DNA supercoiling".to_string(),
                "To methylate DNA".to_string(),
                "To transcribe DNA to RNA".to_string(),
            ],
            1,
            "Topoisomerase relieves tension in the DNA double helix caused by unwinding during replication by temporarily cutting and rejoining DNA strands.".to_string(),
            Difficulty::Medium,
            "dna_rna".to_string(),
        ),
        
        Question::new(
            18,
            "Which RNA polymerase transcribes mRNA in eukaryotes?".to_string(),
            vec![
                "RNA polymerase I".to_string(),
                "RNA polymerase II".to_string(),
                "RNA polymerase III".to_string(),
                "RNA polymerase IV".to_string(),
            ],
            1,
            "RNA polymerase II transcribes all protein-coding genes to produce mRNA, as well as many non-coding RNAs.".to_string(),
            Difficulty::Hard,
            "dna_rna".to_string(),
        ).with_hint("RNA pol I makes rRNA, RNA pol III makes tRNA and other small RNAs.".to_string()),
        
        Question::new(
            19,
            "What modification occurs at the 3' end of eukaryotic mRNA?".to_string(),
            vec![
                "Addition of a 5' cap".to_string(),
                "Addition of a poly-A tail".to_string(),
                "Removal of introns".to_string(),
                "Addition of a poly-U tail".to_string(),
            ],
            1,
            "A poly-A tail (multiple adenine nucleotides) is added to the 3' end of eukaryotic mRNA, increasing stability and aiding in translation.".to_string(),
            Difficulty::Medium,
            "dna_rna".to_string(),
        ),
        
        Question::new(
            20,
            "In which phase of the cell cycle does DNA replication occur?".to_string(),
            vec![
                "G1 phase".to_string(),
                "S phase".to_string(),
                "G2 phase".to_string(),
                "M phase".to_string(),
            ],
            1,
            "DNA replication occurs during the S (synthesis) phase of the cell cycle, between the G1 and G2 phases.".to_string(),
            Difficulty::Easy,
            "dna_rna".to_string(),
        ),
    ]
}