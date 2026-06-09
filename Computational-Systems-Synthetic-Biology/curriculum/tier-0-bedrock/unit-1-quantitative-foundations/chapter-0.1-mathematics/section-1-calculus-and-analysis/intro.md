# Section 1: Calculus and Analysis

Biology is, at its core, a science of *process*. Molecules are made and destroyed. Cells grow, divide, and die. Populations rise and fall. Signals propagate across membranes and through signaling cascades. None of these phenomena are captured by a static description — they are inherently *dynamic*, and to reason about them quantitatively you need a mathematical language for change. That language is calculus.

This section builds that language from the ground up. We begin with **single-variable calculus** — derivatives, integrals, and the Taylor series — because these are the tools you will reach for first when writing down even the simplest model. Every ODE you will ever encounter is a statement about a derivative. Every solution you will ever compute is an integral. Every stability analysis you will ever perform begins with a Taylor expansion around a fixed point.

From there we move to **multivariable calculus**, where single quantities give way to systems. Biological systems do not have one variable — they have dozens. Metabolite concentrations, protein abundances, gene expression levels all vary simultaneously, and their interplay is captured by partial derivatives and the Jacobian matrix. Gradient descent — the engine behind modern parameter estimation and machine learning — lives here.

Then come **ordinary differential equations** (ODEs), where calculus meets dynamics. This is the chapter's climax: the moment when all the individual tools combine into the framework you will use to model everything from enzyme kinetics to genetic oscillators. You will learn not just how to write down and solve ODEs, but how to understand their behavior qualitatively — what fixed points mean, how stability is assessed, what bifurcations signal.

Finally, we touch on **partial differential equations** (PDEs), which generalize the ODE framework to systems varying in both time and space. Turing patterns, morphogen gradients, diffusion of transcription factors across a nucleus — these phenomena require PDEs to describe properly.

A word of advice before you begin: do not rush. The concepts here are genuinely foundational, and the temptation to skim toward the biological applications is real. Resist it. The students who struggle with systems biology modeling later are almost always the ones who treated this section as a review rather than a foundation. Every tool you encounter in the rest of this curriculum connects back here.

The payoff comes quickly. By the end of this section, you will have the mathematical vocabulary to write down a model of almost any biological dynamical system, analyze its steady states, and begin asking what happens when things go wrong.
