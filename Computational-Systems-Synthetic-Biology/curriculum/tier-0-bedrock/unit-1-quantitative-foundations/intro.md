# Unit 1: Quantitative Foundations

There is a moment that many biology students experience during their first real exposure to computational and systems biology — a moment of vertigo. They understand the biology. They can recite the central dogma, explain the logic of signal transduction, describe the thermodynamics of protein folding. But when they encounter an ODE model of gene expression, or a principal component analysis of RNA-seq data, or a flux balance analysis of a metabolic network, they find themselves unable to read the mathematics — not because they lack intelligence, but because no one taught them to.

This unit exists to prevent that moment of vertigo.

The Quantitative Foundations unit covers the mathematical, chemical, and biological bedrock that underpins everything else in this curriculum. It is not a math course and it is not a chemistry course — it is a course in the specific quantitative language of computational systems biology. The emphasis is relentlessly on *use*: every concept is introduced in the context of real biological problems, every formalism is motivated by the questions it enables you to answer.

**Chapter 0.1: Mathematics** covers calculus, linear algebra, probability and statistics, graph theory, and information theory. These are the five mathematical frameworks that appear most frequently in the research literature you will read, the models you will build, and the data you will analyze. 

**Chapter 0.2: Chemistry** covers the thermodynamics and kinetics that underlie molecular biology. Why does ATP hydrolysis drive biosynthesis? How do enzymes achieve their extraordinary specificity and speed? Why do proteins fold? The answers are chemical, and the quantitative framework is thermodynamics. You cannot build a genome-scale metabolic model without understanding free energy. You cannot analyze enzyme kinetics without understanding rate constants and transition state theory.

The two chapters in this unit are genuinely foundational — they set up the vocabulary and conceptual framework that every subsequent tier builds on. Students with a strong mathematics background will move through Chapter 0.1 quickly, perhaps treating it as review; for them, the value is in seeing the biological framing. Students who feel uncertain about their mathematical preparation should invest time here, because fluency with these tools pays dividends throughout the curriculum.

By the end of this unit, you should be able to:

- Write and analyze an ordinary differential equation model of a biological process
- Interpret a stability analysis using eigenvalues
- Apply Bayesian reasoning to parameter estimation from noisy data
- Compute and interpret a thermodynamic free energy change
- Describe enzyme kinetics using the Michaelis-Menten framework
- Connect molecular-level chemistry to cell-level behavior

These are not abstract skills. They are the daily tools of practicing computational biologists. Let's build them.
