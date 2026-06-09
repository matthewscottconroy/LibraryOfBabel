# Chapter 0.1: Mathematics

In 1952, Alan Turing published a paper titled "The Chemical Basis of Morphogenesis." In it, he asked a deceptively simple question: how does a uniform ball of cells know where to grow a head, a limb, a stripe? His answer was a system of two coupled partial differential equations describing how two chemical species — an activator and an inhibitor — diffuse and react across tissue. The mathematics predicted, astonishingly, that a spatially uniform system could spontaneously break its own symmetry and generate regular patterns: spots, stripes, spirals.

This was not a biology paper. It was a mathematics paper. And that is the first lesson of this chapter: mathematics is not the language that biologists learn so they can talk to physicists. It is the language in which the deepest questions in biology are asked and answered.

This chapter builds the mathematical foundation for everything that follows. It is organized around four major areas, each essential in its own right:

**Calculus and analysis** gives you the language of change — derivatives for instantaneous rates, integrals for accumulated quantities, Taylor series for principled approximation. You will use these tools in every ODE model, every stability analysis, every parameter estimation procedure in this curriculum.

**Linear algebra** gives you the language of systems — matrices as transformations, eigenvalues as the fundamental modes of a network, singular value decomposition as the key to understanding high-dimensional data. When you analyze the stability of a metabolic network or reduce a dataset of ten thousand gene expression measurements to its essential structure, you are doing linear algebra.

**Probability and statistics** gives you the language of uncertainty — distributions, inference, Bayesian reasoning. Biology is irreducibly stochastic: gene expression fluctuates, population sizes vary, measurements carry noise. Models that ignore this are wrong in ways that matter. Models that incorporate it properly can be learned from data.

**Discrete mathematics and graph theory** gives you the language of connectivity — networks as mathematical objects, graph properties that predict biological function, Boolean models of regulatory logic. The interactome is a graph. The metabolic network is a graph. Whether a network is robust to node deletion is a graph-theory question.

These four areas are not separate subjects that happen to appear in the same chapter — they are deeply interconnected. A signaling network is a graph (graph theory) with dynamics described by ODEs (calculus) whose parameters are estimated from noisy data (statistics) and whose steady-state behavior is analyzed via eigenvalues (linear algebra). The goal of this chapter is not merely to teach you each tool in isolation but to show you how they work together.

A note on depth: this chapter surveys topics that each have entire textbooks devoted to them. You will not emerge from it as a mathematician. What you will emerge with is fluency — the ability to recognize which mathematical framework applies to which biological problem, and sufficient facility with each framework to make real progress. The deeper mathematical treatments are worth reading; Strogatz's *Nonlinear Dynamics and Chaos* for ODEs, Lay's *Linear Algebra* for matrices, Jaynes' *Probability Theory* for Bayesian reasoning. But this chapter gives you the working vocabulary you need to engage productively with the biological material that follows.

Let's begin with change.
