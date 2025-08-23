use crate::models::{Difficulty, Question};

pub fn get_questions() -> Vec<Question> {
    vec![
        Question::new(
            1,
            "What are G-protein coupled receptors (GPCRs)?".to_string(),
            vec![
                "Receptors that directly phosphorylate proteins".to_string(),
                "Seven-transmembrane receptors that activate G proteins".to_string(),
                "Nuclear receptors that bind DNA".to_string(),
                "Ion channels that open upon ligand binding".to_string(),
            ],
            1,
            "GPCRs are seven-transmembrane domain receptors that activate heterotrimeric G proteins upon ligand binding.".to_string(),
            Difficulty::Easy,
            "signaling".to_string(),
        ),
        
        Question::new(
            2,
            "What is the role of cAMP in cell signaling?".to_string(),
            vec![
                "Primary messenger".to_string(),
                "Second messenger".to_string(),
                "Neurotransmitter".to_string(),
                "Hormone".to_string(),
            ],
            1,
            "cAMP (cyclic AMP) is a second messenger that transmits signals from hormones and neurotransmitters inside cells.".to_string(),
            Difficulty::Easy,
            "signaling".to_string(),
        ),
        
        Question::new(
            3,
            "Which enzyme produces cAMP?".to_string(),
            vec![
                "Protein kinase A".to_string(),
                "Adenylyl cyclase".to_string(),
                "Phosphodiesterase".to_string(),
                "Guanylyl cyclase".to_string(),
            ],
            1,
            "Adenylyl cyclase converts ATP to cAMP in response to G protein activation.".to_string(),
            Difficulty::Medium,
            "signaling".to_string(),
        ),
        
        Question::new(
            4,
            "What type of receptor is the insulin receptor?".to_string(),
            vec![
                "GPCR".to_string(),
                "Receptor tyrosine kinase".to_string(),
                "Ion channel".to_string(),
                "Nuclear receptor".to_string(),
            ],
            1,
            "The insulin receptor is a receptor tyrosine kinase that phosphorylates tyrosine residues upon insulin binding.".to_string(),
            Difficulty::Medium,
            "signaling".to_string(),
        ).with_hint("This receptor type phosphorylates specific amino acids when activated.".to_string()),
        
        Question::new(
            5,
            "What is the function of protein kinase A (PKA)?".to_string(),
            vec![
                "To produce cAMP".to_string(),
                "To phosphorylate proteins in response to cAMP".to_string(),
                "To degrade cAMP".to_string(),
                "To activate G proteins".to_string(),
            ],
            1,
            "PKA is activated by cAMP and phosphorylates serine/threonine residues on target proteins.".to_string(),
            Difficulty::Medium,
            "signaling".to_string(),
        ),
        
        Question::new(
            6,
            "What does IP3 do in cell signaling?".to_string(),
            vec![
                "Activates protein kinase C".to_string(),
                "Triggers calcium release from intracellular stores".to_string(),
                "Phosphorylates proteins".to_string(),
                "Activates G proteins".to_string(),
            ],
            1,
            "IP3 (inositol 1,4,5-trisphosphate) binds to receptors on the ER, causing calcium release into the cytoplasm.".to_string(),
            Difficulty::Hard,
            "signaling".to_string(),
        ),
        
        Question::new(
            7,
            "What is the JAK-STAT pathway involved in?".to_string(),
            vec![
                "Cytokine signaling".to_string(),
                "Steroid hormone signaling".to_string(),
                "Neurotransmitter signaling".to_string(),
                "Ion channel regulation".to_string(),
            ],
            0,
            "The JAK-STAT pathway mediates cytokine signaling, with JAKs phosphorylating STATs for gene transcription.".to_string(),
            Difficulty::Hard,
            "signaling".to_string(),
        ).with_hint("This pathway is important in immune responses.".to_string()),
        
        Question::new(
            8,
            "What type of signaling occurs between adjacent cells?".to_string(),
            vec![
                "Endocrine".to_string(),
                "Paracrine".to_string(),
                "Juxtacrine".to_string(),
                "Autocrine".to_string(),
            ],
            2,
            "Juxtacrine signaling requires direct contact between adjacent cells through membrane-bound signals and receptors.".to_string(),
            Difficulty::Medium,
            "signaling".to_string(),
        ),
        
        Question::new(
            9,
            "What is the role of phosphodiesterase in signal transduction?".to_string(),
            vec![
                "To produce cAMP".to_string(),
                "To degrade cAMP and cGMP".to_string(),
                "To phosphorylate proteins".to_string(),
                "To activate G proteins".to_string(),
            ],
            1,
            "Phosphodiesterase terminates signaling by hydrolyzing cAMP to AMP and cGMP to GMP.".to_string(),
            Difficulty::Medium,
            "signaling".to_string(),
        ),
        
        Question::new(
            10,
            "Which molecules can pass directly through the cell membrane?".to_string(),
            vec![
                "Peptide hormones".to_string(),
                "Steroid hormones".to_string(),
                "Proteins".to_string(),
                "Ions".to_string(),
            ],
            1,
            "Steroid hormones are lipophilic and can pass through the lipid bilayer to bind intracellular receptors.".to_string(),
            Difficulty::Easy,
            "signaling".to_string(),
        ),
        
        Question::new(
            11,
            "What is the Ras protein?".to_string(),
            vec![
                "A transcription factor".to_string(),
                "A small GTPase involved in growth signaling".to_string(),
                "A second messenger".to_string(),
                "An ion channel".to_string(),
            ],
            1,
            "Ras is a small GTPase that acts as a molecular switch in growth factor signaling pathways.".to_string(),
            Difficulty::Medium,
            "signaling".to_string(),
        ),
        
        Question::new(
            12,
            "What does calcium bind to in smooth muscle contraction?".to_string(),
            vec![
                "Troponin".to_string(),
                "Calmodulin".to_string(),
                "Actin".to_string(),
                "Myosin".to_string(),
            ],
            1,
            "In smooth muscle, calcium binds to calmodulin, which then activates myosin light chain kinase.".to_string(),
            Difficulty::Hard,
            "signaling".to_string(),
        ),
        
        Question::new(
            13,
            "What is the MAPK cascade?".to_string(),
            vec![
                "A metabolic pathway".to_string(),
                "A protein phosphorylation cascade in cell signaling".to_string(),
                "A DNA repair mechanism".to_string(),
                "A protein degradation pathway".to_string(),
            ],
            1,
            "The MAPK (Mitogen-Activated Protein Kinase) cascade is a series of protein kinases that amplify signals.".to_string(),
            Difficulty::Medium,
            "signaling".to_string(),
        ),
        
        Question::new(
            14,
            "What type of receptor are nicotinic acetylcholine receptors?".to_string(),
            vec![
                "GPCRs".to_string(),
                "Receptor tyrosine kinases".to_string(),
                "Ligand-gated ion channels".to_string(),
                "Nuclear receptors".to_string(),
            ],
            2,
            "Nicotinic receptors are ligand-gated ion channels that open when acetylcholine binds.".to_string(),
            Difficulty::Medium,
            "signaling".to_string(),
        ),
        
        Question::new(
            15,
            "What is desensitization in cell signaling?".to_string(),
            vec![
                "Increased response to a signal".to_string(),
                "Decreased response to continued stimulation".to_string(),
                "Cell death".to_string(),
                "Receptor synthesis".to_string(),
            ],
            1,
            "Desensitization is the reduction in cellular response to continued or repeated stimulation.".to_string(),
            Difficulty::Medium,
            "signaling".to_string(),
        ),
        
        Question::new(
            16,
            "Which G protein subunit has GTPase activity?".to_string(),
            vec![
                "Gα".to_string(),
                "Gβ".to_string(),
                "Gγ".to_string(),
                "Gβγ complex".to_string(),
            ],
            0,
            "The Gα subunit has intrinsic GTPase activity that hydrolyzes GTP to GDP, terminating the signal.".to_string(),
            Difficulty::Hard,
            "signaling".to_string(),
        ).with_hint("This subunit switches between active and inactive states.".to_string()),
        
        Question::new(
            17,
            "What is the Wnt signaling pathway important for?".to_string(),
            vec![
                "Cell metabolism".to_string(),
                "Development and cell fate determination".to_string(),
                "DNA repair".to_string(),
                "Protein degradation".to_string(),
            ],
            1,
            "Wnt signaling regulates cell proliferation, differentiation, and fate determination during development.".to_string(),
            Difficulty::Hard,
            "signaling".to_string(),
        ),
        
        Question::new(
            18,
            "What does NO (nitric oxide) do as a signaling molecule?".to_string(),
            vec![
                "Binds to membrane receptors".to_string(),
                "Activates guanylyl cyclase to produce cGMP".to_string(),
                "Phosphorylates proteins".to_string(),
                "Acts as a transcription factor".to_string(),
            ],
            1,
            "NO diffuses across membranes and activates soluble guanylyl cyclase, increasing cGMP levels.".to_string(),
            Difficulty::Hard,
            "signaling".to_string(),
        ),
        
        Question::new(
            19,
            "What is apoptosis triggered by in the extrinsic pathway?".to_string(),
            vec![
                "Mitochondrial damage".to_string(),
                "Death receptor activation".to_string(),
                "DNA damage".to_string(),
                "ER stress".to_string(),
            ],
            1,
            "The extrinsic apoptotic pathway is initiated by death receptors like Fas or TNF receptors on the cell surface.".to_string(),
            Difficulty::Medium,
            "signaling".to_string(),
        ),
        
        Question::new(
            20,
            "What is the role of β-arrestin in GPCR signaling?".to_string(),
            vec![
                "To activate G proteins".to_string(),
                "To desensitize and internalize receptors".to_string(),
                "To produce second messengers".to_string(),
                "To phosphorylate proteins".to_string(),
            ],
            1,
            "β-arrestin binds to phosphorylated GPCRs, blocking G protein activation and promoting receptor internalization.".to_string(),
            Difficulty::Expert,
            "signaling".to_string(),
        ),
    ]
}