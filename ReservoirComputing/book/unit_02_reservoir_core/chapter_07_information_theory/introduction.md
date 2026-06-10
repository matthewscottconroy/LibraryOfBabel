# Chapter 7: Information Theory of Reservoirs

## What Does a Reservoir Actually Compute?

You have, by now, built reservoirs. You have watched them track chaotic attractors, classify speech, and predict the next symbol in a sequence. They work — often surprisingly well. But a nagging question should be forming: *why* do they work, and *how much* can they work? When a reservoir succeeds at a task, is it succeeding because the high-dimensional nonlinear expansion of the input signal happens to contain the right information? And when it fails, is it failing because the information was never there to begin with, or because the linear readout couldn't extract it?

These are not philosophical questions. They have quantitative, testable answers, and this chapter gives you the tools to find them.

The key insight is that a reservoir is, at its core, an information-processing device. Its $N$ recurrent neurons hold a finite-dimensional state vector at each timestep, and that state vector is the only channel through which past inputs can influence future outputs. Everything the readout can ever know about the input history must pass through this bottleneck. The question of reservoir capacity is, therefore, a question about how much information — about how many distinct functions of the input history — can be faithfully encoded in the reservoir state.

## The Capacity Framework as a Stress Test

In 2012, Dambre, Verstraeten, Schrauwen, and Massar published a framework that made this precise [Dambre2012]. Their central result is elegant: for any reservoir with $N$ neurons, the total information-processing capacity — measured as the sum of how well the reservoir can linearly reconstruct an orthogonal family of target functions of the input history — is bounded above by $N$. The bound is tight: a linear reservoir with an orthogonal weight matrix achieves it exactly.

This result does three things simultaneously. First, it gives you a *diagnostic tool*: by measuring capacity, you can determine whether a reservoir is using its $N$ degrees of freedom efficiently. A reservoir with 100 neurons but capacity of 20 is wasting 80 dimensions — and you can ask why. Second, it reveals a *fundamental tradeoff*: capacity spent on nonlinear transformations of the input is capacity not spent on memory of the input, and vice versa. You cannot have both simultaneously, to an arbitrary degree, with a fixed reservoir size. Third, it provides a *design principle*: if your task requires certain functions of the input history, you can engineer a reservoir to have high capacity for precisely those functions.

## Memory as a Special Case

The most natural and historically earliest version of capacity is *memory capacity*, introduced by Jaeger in 2002 [Jaeger2002memory]. A reservoir has memory if its current state encodes information about inputs from the past. The $k$-step memory capacity $MC_k$ measures how well the current state can be linearly decoded to recover the input from $k$ timesteps ago. Summing over all delays gives the total memory capacity $MC$, and — again — $MC \leq N$.

Memory capacity is the simplest case of the Dambre framework (using linear target functions), but it is worth studying on its own terms because it connects directly to the spectral properties of the reservoir weight matrix. You will see, in this chapter, that the memory profile $\{MC_k\}_{k=1}^\infty$ has a geometric envelope controlled by the spectral radius, with finer structure determined by the eigenspectrum. This connection bridges information theory and dynamical systems theory, the two pillars of reservoir computing analysis.

## What This Chapter Covers

**Section 7.1** develops the full Dambre capacity framework from first principles. We define the capacity of a reservoir for an arbitrary function $f$, construct the orthonormal basis expansion, and derive the bound $C_{total} \leq N$.

**Section 7.2** specializes to memory capacity. We prove Jaeger's original bound $MC \leq N$ using the rank of the state covariance matrix, derive the $\rho$-dependent decay formula for linear reservoirs, and work through a complete numerical example comparing reservoirs with different spectral radii.

The exercises and lab assignments at the end of the chapter will ask you to implement these measurements, plot capacity profiles as functions of reservoir hyperparameters, and use capacity as a diagnostic tool for reservoir designs you have built in earlier chapters.

By the end of this chapter, you will be able to answer the question in the title: not qualitatively, but in nats, bits, or dimensionless units of your choosing.

---

*Prerequisites: Chapters 1–6. Familiarity with linear algebra (SVD, eigendecompositions, covariance matrices), probability theory (variance, correlation), and basic signal processing (frequency response) will be assumed throughout.*
