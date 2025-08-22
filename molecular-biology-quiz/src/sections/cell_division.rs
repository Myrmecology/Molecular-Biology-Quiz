use crate::models::{Difficulty, Question};

pub fn get_questions() -> Vec<Question> {
    vec![
        Question::new(
            1,
            "During which phase of mitosis do sister chromatids separate?".to_string(),
            vec![
                "Prophase".to_string(),
                "Metaphase".to_string(),
                "Anaphase".to_string(),
                "Telophase".to_string(),
            ],
            2,
            "During anaphase, sister chromatids separate and move to opposite poles of the cell, pulled by spindle fibers.".to_string(),
            Difficulty::Easy,
            "cell_division".to_string(),
        ),
        
        Question::new(
            2,
            "What is the main purpose of meiosis?".to_string(),
            vec![
                "Growth and repair".to_string(),
                "Production of gametes".to_string(),
                "DNA replication".to_string(),
                "Protein synthesis".to_string(),
            ],
            1,
            "Meiosis produces haploid gametes (sex cells) with half the chromosome number, essential for sexual reproduction.".to_string(),
            Difficulty::Easy,
            "cell_division".to_string(),
        ),
        
        Question::new(
            3,
            "What occurs during crossing over?".to_string(),
            vec![
                "Sister chromatids separate".to_string(),
                "Homologous chromosomes exchange genetic material".to_string(),
                "Chromosomes condense".to_string(),
                "Nuclear envelope breaks down".to_string(),
            ],
            1,
            "Crossing over involves the exchange of genetic material between homologous chromosomes during prophase I of meiosis, increasing genetic diversity.".to_string(),
            Difficulty::Medium,
            "cell_division".to_string(),
        ),
        
        Question::new(
            4,
            "Which protein complex holds sister chromatids together?".to_string(),
            vec![
                "Kinetochore".to_string(),
                "Cohesin".to_string(),
                "Condensin".to_string(),
                "Separase".to_string(),
            ],
            1,
            "Cohesin is a protein complex that holds sister chromatids together until separase cleaves it during anaphase.".to_string(),
            Difficulty::Hard,
            "cell_division".to_string(),
        ).with_hint("This protein's name relates to its adhesive function.".to_string()),
        
        Question::new(
            5,
            "What is the G0 phase?".to_string(),
            vec![
                "A phase of rapid cell division".to_string(),
                "A quiescent state where cells exit the cell cycle".to_string(),
                "The phase where DNA replicates".to_string(),
                "The phase where cells prepare for mitosis".to_string(),
            ],
            1,
            "G0 is a quiescent phase where cells exit the cell cycle and stop dividing, either temporarily or permanently.".to_string(),
            Difficulty::Medium,
            "cell_division".to_string(),
        ),
        
        Question::new(
            6,
            "Which checkpoint ensures all chromosomes are attached to spindle fibers?".to_string(),
            vec![
                "G1/S checkpoint".to_string(),
                "G2/M checkpoint".to_string(),
                "Spindle checkpoint".to_string(),
                "DNA damage checkpoint".to_string(),
            ],
            2,
            "The spindle checkpoint (SAC) prevents anaphase until all kinetochores are properly attached to spindle fibers.".to_string(),
            Difficulty::Medium,
            "cell_division".to_string(),
        ),
        
        Question::new(
            7,
            "How many cells result from one complete meiotic division?".to_string(),
            vec![
                "Two diploid cells".to_string(),
                "Two haploid cells".to_string(),
                "Four diploid cells".to_string(),
                "Four haploid cells".to_string(),
            ],
            3,
            "Meiosis produces four haploid cells from one diploid cell through two successive divisions (meiosis I and II).".to_string(),
            Difficulty::Easy,
            "cell_division".to_string(),
        ),
        
        Question::new(
            8,
            "What are cyclin-dependent kinases (CDKs)?".to_string(),
            vec![
                "Proteins that destroy cyclins".to_string(),
                "Enzymes that regulate cell cycle progression".to_string(),
                "Proteins that build the spindle apparatus".to_string(),
                "Enzymes that replicate DNA".to_string(),
            ],
            1,
            "CDKs are enzymes that, when bound to cyclins, phosphorylate target proteins to drive cell cycle progression.".to_string(),
            Difficulty::Medium,
            "cell_division".to_string(),
        ),
        
        Question::new(
            9,
            "During which phase does cytokinesis typically begin?".to_string(),
            vec![
                "Prophase".to_string(),
                "Metaphase".to_string(),
                "Anaphase".to_string(),
                "Telophase".to_string(),
            ],
            2,
            "Cytokinesis typically begins during anaphase and continues through telophase, dividing the cytoplasm to form two cells.".to_string(),
            Difficulty::Medium,
            "cell_division".to_string(),
        ),
        
        Question::new(
            10,
            "What is nondisjunction?".to_string(),
            vec![
                "Failure of DNA to replicate".to_string(),
                "Failure of chromosomes to separate properly".to_string(),
                "Failure of the nuclear envelope to reform".to_string(),
                "Failure of cytokinesis to occur".to_string(),
            ],
            1,
            "Nondisjunction is the failure of chromosomes to separate properly during cell division, leading to aneuploidy.".to_string(),
            Difficulty::Medium,
            "cell_division".to_string(),
        ),
        
        Question::new(
            11,
            "What protein is primarily responsible for chromosome condensation?".to_string(),
            vec![
                "Histone".to_string(),
                "Cohesin".to_string(),
                "Condensin".to_string(),
                "Topoisomerase".to_string(),
            ],
            2,
            "Condensin complexes help compact chromatin into the highly condensed chromosomes visible during mitosis.".to_string(),
            Difficulty::Hard,
            "cell_division".to_string(),
        ).with_hint("The name of this protein directly relates to its function.".to_string()),
        
        Question::new(
            12,
            "What is the synaptonemal complex?".to_string(),
            vec![
                "Structure that holds homologous chromosomes together during meiosis".to_string(),
                "Structure that attaches chromosomes to spindle fibers".to_string(),
                "Structure that separates sister chromatids".to_string(),
                "Structure that forms the cleavage furrow".to_string(),
            ],
            0,
            "The synaptonemal complex is a protein structure that forms between homologous chromosomes during prophase I of meiosis.".to_string(),
            Difficulty::Expert,
            "cell_division".to_string(),
        ),
        
        Question::new(
            13,
            "Which tumor suppressor protein is known as the 'guardian of the genome'?".to_string(),
            vec![
                "Rb".to_string(),
                "p53".to_string(),
                "BRCA1".to_string(),
                "APC".to_string(),
            ],
            1,
            "p53 monitors DNA integrity and can halt cell division or trigger apoptosis if damage is detected.".to_string(),
            Difficulty::Medium,
            "cell_division".to_string(),
        ),
        
        Question::new(
            14,
            "What structure forms in plant cells during cytokinesis?".to_string(),
            vec![
                "Cleavage furrow".to_string(),
                "Cell plate".to_string(),
                "Contractile ring".to_string(),
                "Spindle midzone".to_string(),
            ],
            1,
            "Plant cells form a cell plate during cytokinesis, which develops into a new cell wall separating daughter cells.".to_string(),
            Difficulty::Easy,
            "cell_division".to_string(),
        ),
        
        Question::new(
            15,
            "When do homologous chromosomes separate during meiosis?".to_string(),
            vec![
                "Anaphase I".to_string(),
                "Anaphase II".to_string(),
                "Metaphase I".to_string(),
                "Metaphase II".to_string(),
            ],
            0,
            "Homologous chromosomes separate during anaphase I, while sister chromatids separate during anaphase II.".to_string(),
            Difficulty::Medium,
            "cell_division".to_string(),
        ),
        
        Question::new(
            16,
            "What is the role of the anaphase promoting complex (APC)?".to_string(),
            vec![
                "To build spindle fibers".to_string(),
                "To trigger chromosome separation and exit from mitosis".to_string(),
                "To condense chromosomes".to_string(),
                "To replicate centrosomes".to_string(),
            ],
            1,
            "The APC ubiquitinates proteins like securin and cyclins, triggering sister chromatid separation and mitotic exit.".to_string(),
            Difficulty::Expert,
            "cell_division".to_string(),
        ),
        
        Question::new(
            17,
            "What happens during the G1 phase of the cell cycle?".to_string(),
            vec![
                "DNA replication".to_string(),
                "Cell growth and normal metabolic activities".to_string(),
                "Preparation for mitosis".to_string(),
                "Chromosome condensation".to_string(),
            ],
            1,
            "During G1, cells grow, accumulate nutrients, and synthesize proteins needed for DNA replication.".to_string(),
            Difficulty::Easy,
            "cell_division".to_string(),
        ),
        
        Question::new(
            18,
            "What is a bivalent?".to_string(),
            vec![
                "A cell with two nuclei".to_string(),
                "A pair of homologous chromosomes during meiosis".to_string(),
                "A chromosome with two chromatids".to_string(),
                "A cell in G2 phase".to_string(),
            ],
            1,
            "A bivalent (or tetrad) is a pair of homologous chromosomes held together during prophase I of meiosis.".to_string(),
            Difficulty::Hard,
            "cell_division".to_string(),
        ),
        
        Question::new(
            19,
            "Which phase of mitosis is longest?".to_string(),
            vec![
                "Prophase".to_string(),
                "Metaphase".to_string(),
                "Anaphase".to_string(),
                "Telophase".to_string(),
            ],
            0,
            "Prophase is typically the longest phase of mitosis, involving chromosome condensation and spindle formation.".to_string(),
            Difficulty::Medium,
            "cell_division".to_string(),
        ),
        
        Question::new(
            20,
            "What causes Down syndrome?".to_string(),
            vec![
                "Deletion of chromosome 21".to_string(),
                "Trisomy 21".to_string(),
                "Monosomy 21".to_string(),
                "Translocation of chromosome 22".to_string(),
            ],
            1,
            "Down syndrome is caused by trisomy 21 (three copies of chromosome 21), usually due to nondisjunction during meiosis.".to_string(),
            Difficulty::Easy,
            "cell_division".to_string(),
        ),
    ]
}