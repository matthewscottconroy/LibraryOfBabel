# Unit I: The Prehistory — Dynamical Systems and the Problem of Time

---

> *"The present moment always will have been."* — a fact about time that has no analogue in feedforward computation.

---

## What This Unit Is About

Before we can understand what a reservoir computer is, we have to understand what problem it solves. And to understand the problem, we have to go deeper than machine learning — all the way down to a question that has troubled philosophers, physicists, and engineers alike: **what does it mean to compute with time?**

A feedforward neural network, for all its celebrated power, is a machine that lives entirely in the present. Feed it an image, and it tells you what it sees — now, from that image alone. The past does not exist for it. The future is not its concern. It is a function: one state of the world in, one answer out.

But most of what we actually care about computing is not like this. Speech is a sequence of sounds, each of which acquires meaning from what came before and what might come next. Music is pattern across time, not a frozen moment. The trajectory of a disease, the dynamics of a market, the evolution of a weather system — all of these are processes, not snapshots. They are stories, and stories require memory.

This unit is the story of how researchers came to understand this problem — and why the solutions they reached before reservoir computing were either mathematically beautiful but practically intractable, or practical but theoretically shallow. We will meet dynamical systems theory, which gives us the mathematical language of change. We will study recurrent neural networks, which are the most natural bridge between neural computation and temporal dynamics. And we will watch, in slow motion, the moment when a different and audacious idea became possible.

---

## The Central Problem

Here is the problem stated as sharply as possible.

A function $f: \mathbb{R}^n \to \mathbb{R}^m$ is a mapping from a finite-dimensional input to a finite-dimensional output. No matter how complex, no matter how deep the network that computes it, this function has no intrinsic relationship with time. If you run it twice on the same input, you get the same output. History is irrelevant.

Now consider a sequence of inputs: $u_1, u_2, u_3, \ldots, u_t$. We want to produce a sequence of outputs $y_1, y_2, \ldots, y_t$ where each $y_t$ depends not only on $u_t$ but on the entire history $u_1, \ldots, u_t$. This is a **functional** — a map from sequences to sequences — and it is a fundamentally different beast from a function.

The space of all possible sequences is infinite-dimensional. A feedforward network, however deep, lives in a finite-dimensional world. You cannot, in principle, design a static feedforward architecture that handles arbitrary-length sequences while maintaining full temporal dependence. Something has to give.

The classical answer was: **state**. Build a machine that maintains an internal state, update that state as new inputs arrive, and produce outputs from that state. This is the idea behind every stateful system from the humble delay line to the most sophisticated recurrent neural network. But as we will see, state comes with its own problems — problems of learning, of stability, of expressiveness — that drove the field toward increasingly clever (and increasingly complicated) solutions.

Reservoir computing is, in a sense, the simplest possible solution: use a randomly connected dynamical system as a state machine, and only train the mapping from state to output. The rest of this book is about why this works, when it works, and how far it can be pushed.

---

## The Three Chapters of Unit I

**Chapter 1** examines the problem from the top down: what can and cannot be computed without memory, and what properties a computational memory must have. We introduce the Volterra series as the mathematician's answer to temporal computation, and we prove rigorously that any input-output system with fading memory can be approximated by a dynamical system of sufficient richness.

**Chapter 2** is a full tutorial in dynamical systems theory. We develop fixed points, limit cycles, chaos, bifurcations, and attractors — all with complete mathematical derivations. We pay special attention to input-driven systems, because a reservoir is precisely such a system. The chapter culminates in generalized synchronization: the deep theorem that explains why a driven dynamical system can serve as a faithful record of its driver's history.

**Chapter 3** closes the pre-history by examining recurrent neural networks — the natural but troubled progenitor of reservoir computing. We derive backpropagation through time carefully, analyze the vanishing gradient problem in detail, and understand exactly why training recurrent weights is so hard. This chapter ends with a transitional section that poses the reservoir computing hypothesis as the natural response to that hardness.

---

## A Note on Mathematical Rigor

This book assumes you have studied calculus through multivariable calculus, linear algebra at the level of eigenvalues and matrix decompositions, and basic probability. It does not assume prior exposure to dynamical systems, functional analysis, or machine learning theory.

Every important result is derived, not merely stated. Proofs are stepped through line by line, with every algebraic manipulation explained. If you find a derivation too fast, write it out yourself — the discipline of reconstruction is worth more than passive reading.

When we state a theorem without full proof (because the proof requires mathematical machinery beyond our scope), we always say so explicitly, give an intuitive argument, and provide a reference for the reader who wants the full story.

---

*We begin with a question simple enough for a child and deep enough for a career: where does the past live?*
