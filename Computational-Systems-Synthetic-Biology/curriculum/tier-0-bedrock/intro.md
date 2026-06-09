# Tier 0: Bedrock

The challenge of computational biology is not primarily technical. It is conceptual.

When you look at an ODE model of a genetic toggle switch, or a flux balance analysis of yeast metabolism, or a principal component analysis of single-cell RNA sequencing data, you are not encountering alien mathematics — you are encountering mathematics you already know (or soon will know), applied to questions that biology has made urgent. The barrier to entry is not intelligence; it is vocabulary. Tier 0 exists to give you that vocabulary.

Two things need to happen before you can do serious systems or computational biology. First, you need to be genuinely fluent with quantitative tools — calculus, linear algebra, probability, thermodynamics — not at the level of having once taken a course, but at the level of being able to reach for the right tool when you see a new biological problem and know immediately what it means. Second, you need a deep working knowledge of the biology and computation that the rest of the curriculum assumes: the molecular logic of the cell, the algorithms that process genomic data, the programming environment in which models are built and analyzed.

Unit 1 and Unit 2 of this tier address each of these needs in turn.

**Unit 1: Quantitative Foundations** builds the mathematical and chemical framework you need to do any quantitative biology at all. Chapter 0.1 covers the five mathematical areas that appear most frequently in the computational biology literature: calculus for modeling change over time, linear algebra for analyzing networks and high-dimensional data, probability and statistics for reasoning from noisy measurements, graph theory for representing biological networks, and information theory for quantifying biological organization. Chapter 0.2 covers the thermodynamics and kinetics that underlie all of biochemistry: why reactions run in one direction, how enzymes work, how proteins fold, and how free energy flows through metabolism. These are not abstract mathematical subjects — they are the specific quantitative tools you need to understand a genome-scale metabolic model, an RNA-seq analysis pipeline, or a signaling cascade.

**Unit 2: Life Sciences and Computing** provides the biological and computational context. Chapter 0.3 is not a standard cell biology review. It is a targeted account of the molecular and cellular biology that is actively referenced in computational work: gene regulation, signal transduction, the organization of chromatin, the logic of metabolic networks, the machinery of protein synthesis and degradation. Chapter 0.4 introduces the computational foundations: algorithm design, complexity, data structures, and the practical toolkit — Python, Unix, version control, scientific computing libraries — that is the working environment of every practicing computational biologist.

A word about how to use this tier.

Some readers will arrive having taken a year of calculus and a semester of physics. For them, parts of Unit 1 will feel like review. The temptation is to skim. Resist it. The value of Unit 1 is not merely the mathematics — it is the *biological framing* of that mathematics. Seeing a derivative appear inside an ODE model of a gene circuit, and understanding why that formulation captures the biology, is different from knowing how to differentiate $x^n$. The biological context is what makes the tools useful rather than decorative.

Other readers will arrive with strong biology backgrounds but uncertain about their mathematical preparation. For them, the goal of Unit 1 is to build genuine fluency rather than mere familiarity. Every concept is introduced in the context of a real biological problem. Every technique is practiced in that context. The mathematical details matter — you cannot fake fluency with the Jacobian matrix — but they are always motivated by the biological questions they help answer.

By the end of Tier 0, you should be able to:

- Write down and analyze a dynamical systems model of a biological process
- Interpret thermodynamic arguments about why biochemical reactions proceed as they do
- Navigate the Python scientific stack — NumPy, SciPy, Matplotlib, Pandas — with confidence
- Describe the molecular logic of gene regulation, signal transduction, and metabolism at a level sufficient to model them
- Apply basic algorithms for sequence comparison, graph analysis, and statistical inference

These are threshold competencies. They do not make you a computational biologist — they make you ready to become one. That work begins in Tier 1.
