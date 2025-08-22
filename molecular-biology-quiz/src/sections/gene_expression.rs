use crate::models::{Difficulty, Question};

pub fn get_questions() -> Vec<Question> {
    vec![
        Question::new(
            1,
            "What is an operon?".to_string(),
            vec![
                "A single gene that codes for multiple proteins".to_string(),
                "A cluster of genes under the control of a single promoter".to_string(),
                "A type of RNA polymerase".to_string(),
                "A protein that regulates transcription".to_string(),
            ],
            1,
            "An operon is a cluster of genes transcribed together under the control of a single promoter, common in prokaryotes.".to_string(),
            Difficulty::Easy,
            "gene_expression".to_string(),
        ),
        
        Question::new(
            2,
            "In the lac operon, what happens when lactose is present?".to_string(),
            vec![
                "The repressor binds more tightly to the operator".to_string(),
                "The repressor is inactivated and transcription occurs".to_string(),
                "The operon is permanently turned off".to_string(),
                "CAP-cAMP cannot bind".to_string(),
            ],
            1,
            "Lactose (as allolactose) binds to the lac repressor, causing it to release from the operator, allowing transcription.".to_string(),
            Difficulty::Medium,
            "gene_expression".to_string(),
        ),
        
        Question::new(
            3,
            "What is the role of microRNAs (miRNAs)?".to_string(),
            vec![
                "To code for small proteins".to_string(),
                "To regulate gene expression post-transcriptionally".to_string(),
                "To splice introns from mRNA".to_string(),
                "To replicate DNA".to_string(),
            ],
            1,
            "miRNAs bind to complementary mRNA sequences, typically leading to translational repression or mRNA degradation.".to_string(),
            Difficulty::Medium,
            "gene_expression".to_string(),
        ),
        
        Question::new(
            4,
            "What is DNA methylation typically associated with?".to_string(),
            vec![
                "Gene activation".to_string(),
                "Gene silencing".to_string(),
                "DNA replication".to_string(),
                "DNA repair".to_string(),
            ],
            1,
            "DNA methylation, particularly at CpG islands in promoters, is generally associated with gene silencing and heterochromatin formation.".to_string(),
            Difficulty::Medium,
            "gene_expression".to_string(),
        ).with_hint("Methylation adds methyl groups to cytosines, affecting chromatin structure.".to_string()),
        
        Question::new(
            5,
            "What are enhancers?".to_string(),
            vec![
                "Proteins that increase transcription".to_string(),
                "DNA sequences that increase transcription when bound by proteins".to_string(),
                "RNA molecules that enhance translation".to_string(),
                "Enzymes that modify histones".to_string(),
            ],
            1,
            "Enhancers are DNA regulatory sequences that increase transcription when bound by transcription factors, regardless of orientation or distance.".to_string(),
            Difficulty::Medium,
            "gene_expression".to_string(),
        ),
        
        Question::new(
            6,
            "Which histone modification is associated with active transcription?".to_string(),
            vec![
                "H3K9 methylation".to_string(),
                "H3K27 methylation".to_string(),
                "H3K4 methylation".to_string(),
                "H3K9 deacetylation".to_string(),
            ],
            2,
            "H3K4 methylation (histone H3 lysine 4 methylation) is a mark of active chromatin and transcriptional activation.".to_string(),
            Difficulty::Hard,
            "gene_expression".to_string(),
        ),
        
        Question::new(
            7,
            "What is the function of the TATA box?".to_string(),
            vec![
                "To terminate transcription".to_string(),
                "To serve as a core promoter element for RNA polymerase II binding".to_string(),
                "To splice RNA".to_string(),
                "To methylate DNA".to_string(),
            ],
            1,
            "The TATA box is a core promoter element that helps position RNA polymerase II for transcription initiation.".to_string(),
            Difficulty::Easy,
            "gene_expression".to_string(),
        ),
        
        Question::new(
            8,
            "What is alternative splicing?".to_string(),
            vec![
                "Removal of all introns from pre-mRNA".to_string(),
                "Production of different mRNAs from the same pre-mRNA".to_string(),
                "Joining of exons from different genes".to_string(),
                "Splicing that occurs in the cytoplasm".to_string(),
            ],
            1,
            "Alternative splicing produces multiple protein isoforms from a single gene by including or excluding different exons.".to_string(),
            Difficulty::Medium,
            "gene_expression".to_string(),
        ),
        
        Question::new(
            9,
            "What is the role of CAP-cAMP in the lac operon?".to_string(),
            vec![
                "To repress transcription".to_string(),
                "To enhance transcription when glucose is low".to_string(),
                "To bind lactose".to_string(),
                "To degrade mRNA".to_string(),
            ],
            1,
            "CAP-cAMP complex enhances lac operon transcription when glucose is low, ensuring lactose metabolism when preferred sugar is absent.".to_string(),
            Difficulty::Hard,
            "gene_expression".to_string(),
        ).with_hint("This complex is involved in catabolite repression.".to_string()),
        
        Question::new(
            10,
            "What is genomic imprinting?".to_string(),
            vec![
                "Copying of the entire genome".to_string(),
                "Expression of genes based on parent of origin".to_string(),
                "Silencing of all genes on one chromosome".to_string(),
                "Random inactivation of genes".to_string(),
            ],
            1,
            "Genomic imprinting is epigenetic marking causing monoallelic expression based on whether the allele came from mother or father.".to_string(),
            Difficulty::Hard,
            "gene_expression".to_string(),
        ),
        
        Question::new(
            11,
            "What is X-inactivation?".to_string(),
            vec![
                "Death of cells with X chromosomes".to_string(),
                "Random silencing of one X chromosome in female mammals".to_string(),
                "Activation of X-linked genes".to_string(),
                "Removal of X chromosomes during meiosis".to_string(),
            ],
            1,
            "X-inactivation randomly silences one X chromosome in female mammals for dosage compensation, creating a Barr body.".to_string(),
            Difficulty::Medium,
            "gene_expression".to_string(),
        ),
        
        Question::new(
            12,
            "What are transcription factors?".to_string(),
            vec![
                "Enzymes that synthesize RNA".to_string(),
                "Proteins that bind DNA and regulate transcription".to_string(),
                "RNA molecules that regulate translation".to_string(),
                "Factors that transport mRNA".to_string(),
            ],
            1,
            "Transcription factors are proteins that bind to specific DNA sequences to regulate gene transcription.".to_string(),
            Difficulty::Easy,
            "gene_expression".to_string(),
        ),
        
        Question::new(
            13,
            "What is the trp operon an example of?".to_string(),
            vec![
                "Positive regulation".to_string(),
                "Negative repressible regulation".to_string(),
                "Negative inducible regulation".to_string(),
                "Constitutive expression".to_string(),
            ],
            1,
            "The trp operon is repressible - it's normally on but is repressed when tryptophan is abundant.".to_string(),
            Difficulty::Medium,
            "gene_expression".to_string(),
        ),
        
        Question::new(
            14,
            "What is RNAi (RNA interference)?".to_string(),
            vec![
                "A method of RNA synthesis".to_string(),
                "A gene silencing mechanism using small RNAs".to_string(),
                "A type of RNA splicing".to_string(),
                "A method of RNA amplification".to_string(),
            ],
            1,
            "RNAi is a biological process where small RNA molecules inhibit gene expression by neutralizing targeted mRNA molecules.".to_string(),
            Difficulty::Medium,
            "gene_expression".to_string(),
        ),
        
        Question::new(
            15,
            "What is a promoter?".to_string(),
            vec![
                "A protein that promotes transcription".to_string(),
                "A DNA sequence where RNA polymerase binds".to_string(),
                "An RNA sequence that promotes translation".to_string(),
                "An enzyme that promotes DNA replication".to_string(),
            ],
            1,
            "A promoter is a DNA sequence where RNA polymerase binds to initiate transcription of a gene.".to_string(),
            Difficulty::Easy,
            "gene_expression".to_string(),
        ),
        
        Question::new(
            16,
            "What is heterochromatin?".to_string(),
            vec![
                "Loosely packed, transcriptionally active chromatin".to_string(),
                "Tightly packed, transcriptionally inactive chromatin".to_string(),
                "Chromatin found only in prokaryotes".to_string(),
                "Chromatin without histones".to_string(),
            ],
            1,
            "Heterochromatin is densely packed chromatin that is generally transcriptionally inactive.".to_string(),
            Difficulty::Medium,
            "gene_expression".to_string(),
        ),
        
        Question::new(
            17,
            "What are CpG islands?".to_string(),
            vec![
                "Regions rich in cytosine-guanine dinucleotides".to_string(),
                "Islands of DNA in the cytoplasm".to_string(),
                "Regions where replication begins".to_string(),
                "Sites of DNA damage".to_string(),
            ],
            0,
            "CpG islands are regions with high frequency of CG dinucleotides, often found in promoters and usually unmethylated in normal cells.".to_string(),
            Difficulty::Hard,
            "gene_expression".to_string(),
        ),
        
        Question::new(
            18,
            "What is the mediator complex?".to_string(),
            vec![
                "A protein complex that mediates DNA repair".to_string(),
                "A complex that bridges enhancers and promoters".to_string(),
                "A complex that mediates RNA splicing".to_string(),
                "A complex that mediates protein folding".to_string(),
            ],
            1,
            "The mediator complex transmits signals from transcription factors to RNA polymerase II, integrating regulatory signals.".to_string(),
            Difficulty::Expert,
            "gene_expression".to_string(),
        ).with_hint("This complex acts as a bridge in transcriptional regulation.".to_string()),
        
        Question::new(
            19,
            "What is a ribozyme?".to_string(),
            vec![
                "An enzyme that breaks down RNA".to_string(),
                "An RNA molecule with catalytic activity".to_string(),
                "A protein that binds to ribosomes".to_string(),
                "An enzyme that synthesizes ribose".to_string(),
            ],
            1,
            "Ribozymes are RNA molecules that can catalyze chemical reactions, demonstrating that RNA can have enzymatic activity.".to_string(),
            Difficulty::Medium,
            "gene_expression".to_string(),
        ),
        
        Question::new(
            20,
            "What is the function of insulator elements?".to_string(),
            vec![
                "To insulate DNA from temperature changes".to_string(),
                "To block enhancer-promoter interactions between domains".to_string(),
                "To protect DNA from nucleases".to_string(),
                "To prevent DNA methylation".to_string(),
            ],
            1,
            "Insulator elements (boundary elements) prevent enhancers from activating wrong promoters and maintain chromatin domain boundaries.".to_string(),
            Difficulty::Expert,
            "gene_expression".to_string(),
        ),
    ]
}