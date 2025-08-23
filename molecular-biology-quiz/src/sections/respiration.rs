use crate::models::{Difficulty, Question};

pub fn get_questions() -> Vec<Question> {
    vec![
        Question::new(
            1,
            "Where does glycolysis occur in the cell?".to_string(),
            vec![
                "Mitochondrial matrix".to_string(),
                "Cytoplasm".to_string(),
                "Inner mitochondrial membrane".to_string(),
                "Nucleus".to_string(),
            ],
            1,
            "Glycolysis occurs in the cytoplasm and doesn't require oxygen, making it universal across cell types.".to_string(),
            Difficulty::Easy,
            "respiration".to_string(),
        ),
        
        Question::new(
            2,
            "How many ATP molecules are produced by substrate-level phosphorylation in glycolysis?".to_string(),
            vec![
                "2 ATP (net)".to_string(),
                "4 ATP (net)".to_string(),
                "32 ATP".to_string(),
                "36 ATP".to_string(),
            ],
            0,
            "Glycolysis produces 4 ATP total but consumes 2 ATP, yielding a net gain of 2 ATP through substrate-level phosphorylation.".to_string(),
            Difficulty::Medium,
            "respiration".to_string(),
        ),
        
        Question::new(
            3,
            "What is the final electron acceptor in the electron transport chain?".to_string(),
            vec![
                "NAD+".to_string(),
                "FAD".to_string(),
                "Oxygen".to_string(),
                "Water".to_string(),
            ],
            2,
            "Oxygen is the final electron acceptor in aerobic respiration, being reduced to form water.".to_string(),
            Difficulty::Easy,
            "respiration".to_string(),
        ),
        
        Question::new(
            4,
            "Which complex in the electron transport chain does NOT pump protons?".to_string(),
            vec![
                "Complex I".to_string(),
                "Complex II".to_string(),
                "Complex III".to_string(),
                "Complex IV".to_string(),
            ],
            1,
            "Complex II (succinate dehydrogenase) transfers electrons from FADH2 but doesn't pump protons across the membrane.".to_string(),
            Difficulty::Hard,
            "respiration".to_string(),
        ).with_hint("This complex is also part of the citric acid cycle.".to_string()),
        
        Question::new(
            5,
            "What molecule enters the citric acid cycle?".to_string(),
            vec![
                "Glucose".to_string(),
                "Pyruvate".to_string(),
                "Acetyl-CoA".to_string(),
                "Citrate".to_string(),
            ],
            2,
            "Acetyl-CoA, produced from pyruvate by pyruvate dehydrogenase, enters the citric acid cycle by combining with oxaloacetate.".to_string(),
            Difficulty::Medium,
            "respiration".to_string(),
        ),
        
        Question::new(
            6,
            "How many NADH molecules are produced per glucose in the citric acid cycle?".to_string(),
            vec![
                "2 NADH".to_string(),
                "4 NADH".to_string(),
                "6 NADH".to_string(),
                "8 NADH".to_string(),
            ],
            2,
            "The citric acid cycle produces 3 NADH per turn, and since one glucose yields 2 acetyl-CoA, total is 6 NADH.".to_string(),
            Difficulty::Medium,
            "respiration".to_string(),
        ),
        
        Question::new(
            7,
            "What is the proton-motive force?".to_string(),
            vec![
                "Force that moves protons into the nucleus".to_string(),
                "Electrochemical gradient across the inner mitochondrial membrane".to_string(),
                "Force that breaks peptide bonds".to_string(),
                "Movement of protons through DNA".to_string(),
            ],
            1,
            "The proton-motive force is the electrochemical gradient created by pumping protons across the inner mitochondrial membrane.".to_string(),
            Difficulty::Medium,
            "respiration".to_string(),
        ),
        
        Question::new(
            8,
            "Which enzyme synthesizes ATP using the proton gradient?".to_string(),
            vec![
                "Hexokinase".to_string(),
                "ATP synthase".to_string(),
                "Citrate synthase".to_string(),
                "Pyruvate kinase".to_string(),
            ],
            1,
            "ATP synthase uses the proton gradient to drive ATP synthesis from ADP and Pi through chemiosmosis.".to_string(),
            Difficulty::Easy,
            "respiration".to_string(),
        ),
        
        Question::new(
            9,
            "What is the P/O ratio?".to_string(),
            vec![
                "Ratio of protons to oxygen".to_string(),
                "Ratio of ATP produced per oxygen atom consumed".to_string(),
                "Ratio of pyruvate to oxaloacetate".to_string(),
                "Ratio of phosphate to oxygen".to_string(),
            ],
            1,
            "The P/O ratio indicates the number of ATP molecules produced per oxygen atom reduced in oxidative phosphorylation.".to_string(),
            Difficulty::Expert,
            "respiration".to_string(),
        ).with_hint("This ratio measures the efficiency of oxidative phosphorylation.".to_string()),
        
        Question::new(
            10,
            "What happens to pyruvate during fermentation in muscle cells?".to_string(),
            vec![
                "Converted to ethanol".to_string(),
                "Converted to lactate".to_string(),
                "Converted to acetyl-CoA".to_string(),
                "Converted to glucose".to_string(),
            ],
            1,
            "In anaerobic conditions, muscle cells convert pyruvate to lactate to regenerate NAD+ for glycolysis continuation.".to_string(),
            Difficulty::Easy,
            "respiration".to_string(),
        ),
        
        Question::new(
            11,
            "Which shuttle system transports NADH electrons into mitochondria in heart and liver?".to_string(),
            vec![
                "Glycerol-3-phosphate shuttle".to_string(),
                "Malate-aspartate shuttle".to_string(),
                "Carnitine shuttle".to_string(),
                "Citrate-pyruvate shuttle".to_string(),
            ],
            1,
            "The malate-aspartate shuttle efficiently transfers NADH electrons into mitochondria, preserving the energy as NADH.".to_string(),
            Difficulty::Hard,
            "respiration".to_string(),
        ),
        
        Question::new(
            12,
            "What is the rate-limiting enzyme of glycolysis?".to_string(),
            vec![
                "Hexokinase".to_string(),
                "Phosphofructokinase-1".to_string(),
                "Pyruvate kinase".to_string(),
                "Aldolase".to_string(),
            ],
            1,
            "Phosphofructokinase-1 (PFK-1) is the rate-limiting enzyme, catalyzing the committed step of glycolysis.".to_string(),
            Difficulty::Medium,
            "respiration".to_string(),
        ),
        
        Question::new(
            13,
            "How many CO2 molecules are released per glucose in cellular respiration?".to_string(),
            vec![
                "2 CO2".to_string(),
                "4 CO2".to_string(),
                "6 CO2".to_string(),
                "8 CO2".to_string(),
            ],
            2,
            "Six CO2 are released: 2 from pyruvate dehydrogenase and 4 from the citric acid cycle (2 per cycle × 2 cycles).".to_string(),
            Difficulty::Medium,
            "respiration".to_string(),
        ),
        
        Question::new(
            14,
            "What is the function of uncoupling proteins (UCPs)?".to_string(),
            vec![
                "To increase ATP production".to_string(),
                "To allow protons to bypass ATP synthase".to_string(),
                "To couple glycolysis to the citric acid cycle".to_string(),
                "To prevent oxygen consumption".to_string(),
            ],
            1,
            "UCPs allow protons to return to the matrix without making ATP, dissipating energy as heat (thermogenesis).".to_string(),
            Difficulty::Hard,
            "respiration".to_string(),
        ),
        
        Question::new(
            15,
            "Which molecule allosterically inhibits phosphofructokinase?".to_string(),
            vec![
                "AMP".to_string(),
                "ADP".to_string(),
                "ATP".to_string(),
                "NAD+".to_string(),
            ],
            2,
            "ATP allosterically inhibits PFK-1, providing negative feedback when energy levels are high.".to_string(),
            Difficulty::Medium,
            "respiration".to_string(),
        ),
        
        Question::new(
            16,
            "What is the energy yield from complete oxidation of one palmitic acid (16-carbon)?".to_string(),
            vec![
                "32 ATP".to_string(),
                "64 ATP".to_string(),
                "106 ATP".to_string(),
                "129 ATP".to_string(),
            ],
            3,
            "Beta-oxidation of palmitic acid yields approximately 129 ATP, demonstrating why fats are energy-dense.".to_string(),
            Difficulty::Expert,
            "respiration".to_string(),
        ).with_hint("Fats yield much more ATP per molecule than glucose.".to_string()),
        
        Question::new(
            17,
            "Where does the citric acid cycle occur?".to_string(),
            vec![
                "Cytoplasm".to_string(),
                "Mitochondrial matrix".to_string(),
                "Inner mitochondrial membrane".to_string(),
                "Intermembrane space".to_string(),
            ],
            1,
            "The citric acid cycle occurs in the mitochondrial matrix, where all necessary enzymes are located.".to_string(),
            Difficulty::Easy,
            "respiration".to_string(),
        ),
        
        Question::new(
            18,
            "What is substrate-level phosphorylation?".to_string(),
            vec![
                "ATP synthesis using the proton gradient".to_string(),
                "Direct transfer of phosphate from a substrate to ADP".to_string(),
                "Phosphorylation of glucose".to_string(),
                "Addition of phosphate to proteins".to_string(),
            ],
            1,
            "Substrate-level phosphorylation directly transfers a phosphate group from a high-energy substrate to ADP, forming ATP.".to_string(),
            Difficulty::Medium,
            "respiration".to_string(),
        ),
        
        Question::new(
            19,
            "Which complex contains cytochrome c oxidase?".to_string(),
            vec![
                "Complex I".to_string(),
                "Complex II".to_string(),
                "Complex III".to_string(),
                "Complex IV".to_string(),
            ],
            3,
            "Complex IV (cytochrome c oxidase) transfers electrons from cytochrome c to oxygen, the final step in the electron transport chain.".to_string(),
            Difficulty::Medium,
            "respiration".to_string(),
        ),
        
        Question::new(
            20,
            "What is the Warburg effect?".to_string(),
            vec![
                "Increased oxidative phosphorylation in cancer cells".to_string(),
                "Preference for glycolysis even in oxygen presence in cancer cells".to_string(),
                "Complete shutdown of metabolism in cancer".to_string(),
                "Enhanced fat oxidation in tumors".to_string(),
            ],
            1,
            "The Warburg effect describes cancer cells' preference for glycolysis over oxidative phosphorylation, even with adequate oxygen.".to_string(),
            Difficulty::Expert,
            "respiration".to_string(),
        ),
    ]
}