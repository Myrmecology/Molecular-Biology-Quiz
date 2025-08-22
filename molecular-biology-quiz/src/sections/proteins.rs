use crate::models::{Difficulty, Question};

pub fn get_questions() -> Vec<Question> {
    vec![
        Question::new(
            1,
            "What type of bond links amino acids together in a protein?".to_string(),
            vec![
                "Hydrogen bond".to_string(),
                "Peptide bond".to_string(),
                "Ionic bond".to_string(),
                "Glycosidic bond".to_string(),
            ],
            1,
            "Peptide bonds form between the carboxyl group of one amino acid and the amino group of another, releasing water in a dehydration synthesis reaction.".to_string(),
            Difficulty::Easy,
            "proteins".to_string(),
        ),
        
        Question::new(
            2,
            "Which level of protein structure involves alpha helices and beta sheets?".to_string(),
            vec![
                "Primary structure".to_string(),
                "Secondary structure".to_string(),
                "Tertiary structure".to_string(),
                "Quaternary structure".to_string(),
            ],
            1,
            "Secondary structure includes alpha helices and beta sheets, formed by hydrogen bonds between backbone atoms of the polypeptide chain.".to_string(),
            Difficulty::Easy,
            "proteins".to_string(),
        ),
        
        Question::new(
            3,
            "What is the isoelectric point (pI) of a protein?".to_string(),
            vec![
                "The pH at which the protein denatures".to_string(),
                "The pH at which the protein has no net charge".to_string(),
                "The temperature at which the protein folds".to_string(),
                "The concentration at which the protein precipitates".to_string(),
            ],
            1,
            "The isoelectric point is the pH at which a protein has no net electrical charge, with equal numbers of positive and negative charges.".to_string(),
            Difficulty::Medium,
            "proteins".to_string(),
        ),
        
        Question::new(
            4,
            "Which amino acid can form disulfide bonds?".to_string(),
            vec![
                "Methionine".to_string(),
                "Cysteine".to_string(),
                "Serine".to_string(),
                "Threonine".to_string(),
            ],
            1,
            "Cysteine contains a sulfhydryl (-SH) group that can form disulfide bonds (S-S) with another cysteine, stabilizing protein structure.".to_string(),
            Difficulty::Medium,
            "proteins".to_string(),
        ).with_hint("Look for the amino acid with a sulfur-containing side chain that can form bridges.".to_string()),
        
        Question::new(
            5,
            "What is the Michaelis-Menten constant (Km)?".to_string(),
            vec![
                "Maximum reaction velocity".to_string(),
                "Substrate concentration at half-maximum velocity".to_string(),
                "Enzyme concentration at maximum velocity".to_string(),
                "Rate constant for product formation".to_string(),
            ],
            1,
            "Km is the substrate concentration at which the reaction velocity is half of Vmax, indicating enzyme-substrate affinity.".to_string(),
            Difficulty::Hard,
            "proteins".to_string(),
        ),
        
        Question::new(
            6,
            "Which type of enzyme inhibition can be overcome by increasing substrate concentration?".to_string(),
            vec![
                "Competitive inhibition".to_string(),
                "Non-competitive inhibition".to_string(),
                "Uncompetitive inhibition".to_string(),
                "Irreversible inhibition".to_string(),
            ],
            0,
            "Competitive inhibitors compete with substrate for the active site. Increasing substrate concentration can outcompete the inhibitor.".to_string(),
            Difficulty::Medium,
            "proteins".to_string(),
        ),
        
        Question::new(
            7,
            "What are molecular chaperones?".to_string(),
            vec![
                "Enzymes that break down proteins".to_string(),
                "Proteins that assist in proper protein folding".to_string(),
                "Molecules that transport proteins across membranes".to_string(),
                "Proteins that store amino acids".to_string(),
            ],
            1,
            "Molecular chaperones are proteins that help other proteins fold correctly and prevent aggregation of misfolded proteins.".to_string(),
            Difficulty::Medium,
            "proteins".to_string(),
        ),
        
        Question::new(
            8,
            "Which amino acid is achiral?".to_string(),
            vec![
                "Alanine".to_string(),
                "Valine".to_string(),
                "Glycine".to_string(),
                "Leucine".to_string(),
            ],
            2,
            "Glycine is the only achiral amino acid because its R group is a hydrogen atom, making it symmetrical.".to_string(),
            Difficulty::Medium,
            "proteins".to_string(),
        ),
        
        Question::new(
            9,
            "What is the primary driving force for protein folding?".to_string(),
            vec![
                "Hydrogen bonding".to_string(),
                "Hydrophobic effect".to_string(),
                "Ionic interactions".to_string(),
                "Van der Waals forces".to_string(),
            ],
            1,
            "The hydrophobic effect, where nonpolar amino acids cluster in the protein core away from water, is the main driving force for protein folding.".to_string(),
            Difficulty::Hard,
            "proteins".to_string(),
        ).with_hint("Think about what happens to nonpolar amino acids in aqueous solution.".to_string()),
        
        Question::new(
            10,
            "Which technique separates proteins based on their size?".to_string(),
            vec![
                "Ion exchange chromatography".to_string(),
                "SDS-PAGE".to_string(),
                "Isoelectric focusing".to_string(),
                "Affinity chromatography".to_string(),
            ],
            1,
            "SDS-PAGE (Sodium Dodecyl Sulfate Polyacrylamide Gel Electrophoresis) denatures proteins and coats them with negative charge, separating by size.".to_string(),
            Difficulty::Medium,
            "proteins".to_string(),
        ),
        
        Question::new(
            11,
            "What is an allosteric enzyme?".to_string(),
            vec![
                "An enzyme that works at high temperature".to_string(),
                "An enzyme regulated by binding at a site other than the active site".to_string(),
                "An enzyme that catalyzes multiple reactions".to_string(),
                "An enzyme that requires a cofactor".to_string(),
            ],
            1,
            "Allosteric enzymes are regulated by molecules binding to sites other than the active site, causing conformational changes that affect activity.".to_string(),
            Difficulty::Medium,
            "proteins".to_string(),
        ),
        
        Question::new(
            12,
            "Which amino acids are positively charged at physiological pH?".to_string(),
            vec![
                "Aspartate and Glutamate".to_string(),
                "Lysine and Arginine".to_string(),
                "Serine and Threonine".to_string(),
                "Phenylalanine and Tryptophan".to_string(),
            ],
            1,
            "Lysine and Arginine (along with Histidine) have positively charged side chains at physiological pH (~7.4).".to_string(),
            Difficulty::Medium,
            "proteins".to_string(),
        ),
        
        Question::new(
            13,
            "What is the function of ubiquitin in cells?".to_string(),
            vec![
                "To catalyze protein synthesis".to_string(),
                "To mark proteins for degradation".to_string(),
                "To transport proteins into the nucleus".to_string(),
                "To fold newly synthesized proteins".to_string(),
            ],
            1,
            "Ubiquitin tags proteins for degradation by the proteasome through a process called ubiquitination.".to_string(),
            Difficulty::Hard,
            "proteins".to_string(),
        ),
        
        Question::new(
            14,
            "Which statement about enzyme kinetics is true?".to_string(),
            vec![
                "Vmax increases with competitive inhibition".to_string(),
                "Km decreases with competitive inhibition".to_string(),
                "Vmax decreases with non-competitive inhibition".to_string(),
                "Km increases with non-competitive inhibition".to_string(),
            ],
            2,
            "Non-competitive inhibitors reduce Vmax by decreasing the number of functional enzymes, but don't affect Km since they don't compete for the active site.".to_string(),
            Difficulty::Expert,
            "proteins".to_string(),
        ).with_hint("Non-competitive inhibitors bind regardless of substrate presence.".to_string()),
        
        Question::new(
            15,
            "What is a zymogen?".to_string(),
            vec![
                "An active enzyme".to_string(),
                "An inactive enzyme precursor".to_string(),
                "An enzyme cofactor".to_string(),
                "An enzyme inhibitor".to_string(),
            ],
            1,
            "Zymogens are inactive enzyme precursors that require activation by cleavage, preventing unwanted enzyme activity.".to_string(),
            Difficulty::Medium,
            "proteins".to_string(),
        ),
        
        Question::new(
            16,
            "Which amino acid is most commonly found in turns and loops?".to_string(),
            vec![
                "Alanine".to_string(),
                "Leucine".to_string(),
                "Proline".to_string(),
                "Valine".to_string(),
            ],
            2,
            "Proline's rigid ring structure introduces kinks in the polypeptide chain, making it common in turns and loops.".to_string(),
            Difficulty::Medium,
            "proteins".to_string(),
        ),
        
        Question::new(
            17,
            "What is the Hill coefficient?".to_string(),
            vec![
                "A measure of pH sensitivity".to_string(),
                "A measure of cooperativity in binding".to_string(),
                "A measure of enzyme stability".to_string(),
                "A measure of protein size".to_string(),
            ],
            1,
            "The Hill coefficient indicates cooperativity in ligand binding. Values >1 indicate positive cooperativity, <1 negative cooperativity.".to_string(),
            Difficulty::Expert,
            "proteins".to_string(),
        ),
        
        Question::new(
            18,
            "Which post-translational modification involves adding sugar groups?".to_string(),
            vec![
                "Phosphorylation".to_string(),
                "Methylation".to_string(),
                "Glycosylation".to_string(),
                "Acetylation".to_string(),
            ],
            2,
            "Glycosylation is the addition of sugar groups to proteins, important for protein folding, stability, and cell recognition.".to_string(),
            Difficulty::Easy,
            "proteins".to_string(),
        ),
        
        Question::new(
            19,
            "What type of protein structure is hemoglobin an example of?".to_string(),
            vec![
                "Primary structure only".to_string(),
                "Secondary structure only".to_string(),
                "Tertiary structure only".to_string(),
                "Quaternary structure".to_string(),
            ],
            3,
            "Hemoglobin has quaternary structure, consisting of four polypeptide subunits (two alpha and two beta chains) working together.".to_string(),
            Difficulty::Easy,
            "proteins".to_string(),
        ),
        
        Question::new(
            20,
            "What is the Bohr effect in hemoglobin?".to_string(),
            vec![
                "Increased O2 binding at high pH".to_string(),
                "Decreased O2 binding at low pH".to_string(),
                "Increased CO2 binding at high pH".to_string(),
                "Both A and B".to_string(),
            ],
            3,
            "The Bohr effect describes how hemoglobin's oxygen affinity decreases at low pH (high CO2), facilitating oxygen release in tissues.".to_string(),
            Difficulty::Hard,
            "proteins".to_string(),
        ),
    ]
}