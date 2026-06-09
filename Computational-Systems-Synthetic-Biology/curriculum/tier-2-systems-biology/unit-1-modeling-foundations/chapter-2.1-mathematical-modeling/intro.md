# Chapter 2.1: Mathematical Modeling

## Can a Cell Make a Decision?

Here is a question that would have seemed strange, even nonsensical, to a biologist of the 1950s: can a cell make a decision?

We mean this literally. Not metaphorically — not "the cell 'decides' to divide" as a shorthand for biochemistry. We mean: is there a molecular mechanism by which a cell can exist in one of two distinct stable states, commit to one of them, remain in that state for hours or days despite ongoing molecular noise and environmental fluctuation, and then switch to the other state when given the appropriate signal?

The answer is yes. And we know exactly how it works, because we can write it down as a pair of differential equations.

In 2000, Tim Gardner, Charles Cantor, and Jim Collins published a paper in *Nature* describing a synthetic genetic toggle switch — a two-gene circuit built from scratch by assembling known regulatory proteins in a carefully designed configuration. Gene A repressed gene B; gene B repressed gene A. Under the right conditions, this mutual repression produced bistability: two stable states, separated by an unstable saddle point. The circuit "remembered" which state it had been pushed into, long after the initial push had dissipated. It was a one-bit memory register, built from proteins.

What made this possible was not just clever engineering — it was a mathematical analysis that told the engineers what to build. Before a single gene was cloned, Collins and colleagues analyzed a model:

$$\frac{du}{dt} = \frac{\alpha_1}{1 + v^n} - u, \qquad \frac{dv}{dt} = \frac{\alpha_2}{1 + u^m} - v$$

This two-equation ODE system — with production terms shaped by Hill functions and simple linear degradation — predicted, through stability analysis and phase plane geometry, exactly the conditions under which bistability would occur. The Hill coefficient $n$ had to be greater than 1. The production strengths $\alpha_1$ and $\alpha_2$ had to be large enough. The nullclines had to intersect three times. All of this was visible from the mathematics before any experiment was run.

That is the power of mathematical modeling: it is not just a way of describing what we already know. It is a tool for predicting what we do not yet know, and for designing systems with specific desired behaviors.

## What This Chapter Is About

Chapter 2.1 builds the complete mathematical toolkit for modeling biological dynamics. The chapter is organized around the natural progression from deterministic to stochastic to spatial descriptions, ending with the philosophy of model construction and validation.

**Section 1: Why Models?** begins with the question of what a mathematical model actually is — a precisely stated hypothesis, not a description — and why biological complexity makes quantitative tools not just useful but necessary.

**Section 2: ODE-Based Deterministic Modeling** is the technical core of the chapter. You will learn to write ODEs from biochemical reaction schemes using mass action kinetics, derive the Michaelis-Menten equation by timescale separation, use Hill functions to describe cooperative and regulated gene expression, determine fixed-point stability from the Jacobian matrix and its eigenvalues, and understand how qualitative behavior changes through bifurcations. The toggle switch, the Goodwin oscillator, and the Lotka-Volterra system serve as running examples throughout.

**Section 3: Stochastic Modeling** addresses the regime where deterministic ODEs fail: when molecule copy numbers are so small that random fluctuations in individual molecular events matter. The Chemical Master Equation, the Gillespie algorithm, tau-leaping, and stochastic differential equations are all here, along with the biological phenomena that motivate them — transcriptional bursting, phenotypic heterogeneity, noise-driven switching.

**Section 4: Spatial Modeling** extends the framework to systems where position matters — morphogen gradients, pattern formation, and nanoscale receptor dynamics. Reaction-diffusion PDEs, Turing instability, numerical methods, and particle-based simulation are covered.

**Section 5: Parameter Estimation and Model Validation** addresses the practical challenge of connecting models to data — the inverse problem, identifiability analysis, and sensitivity analysis.

**Section 6: Model-Building Philosophy** synthesizes the technical material into principles and practices that distinguish productive modeling from sophisticated curve-fitting.

## Why This Chapter Is the Foundation

Every chapter that follows in this curriculum builds on the tools developed here. Gene network analysis (Chapter 2.3) requires stability analysis and bifurcation theory. Metabolic modeling (Chapter 2.2) requires quasi-steady-state approximations and sensitivity analysis. Signaling networks (Chapter 2.4) require both deterministic and stochastic frameworks. The mathematical modeling chapter is not a prerequisite for the interesting parts of systems biology — it is the point at which systems biology becomes quantitatively rigorous.

The stylistic model for this chapter is Steven Strogatz's *Nonlinear Dynamics and Chaos*, one of the most lucidly written mathematics texts of the past thirty years. Strogatz's approach — geometric intuition first, algebraic derivation in service of the geometry, biological application attached to every result — is the approach we aim for here. The goal is not to teach you to manipulate differential equations, but to teach you to see what differential equations are telling you about the behavior of living systems.

When you finish this chapter, you will be able to look at a pair of ODEs describing a gene circuit and ask: how many stable states does this have? Under what conditions does it oscillate? What happens when I perturb it? What experiment would distinguish this mechanism from an alternative? These are the questions that drive systems biology — and the tools to answer them are what this chapter provides.
