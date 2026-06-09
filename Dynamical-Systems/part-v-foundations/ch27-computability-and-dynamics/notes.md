# Chapter Notes — Chapter 27

## On the Literature

The connection between computation and dynamical systems is developed in Cristopher Moore's *Unpredictability and Undecidability in Dynamical Systems* (1990) and the subsequent Moore-Koiran (1999) work. If you've made it this far in the book, Moore's paper is worth reading in its original form — it's unusually clear about what is being claimed and why it matters.

The computational complexity perspective, especially for dynamics over the real numbers, is in Blum, Cucker, Shub, and Smale's *Complexity and Real Computation* (1998). This book develops a theory of computation directly over the reals (rather than encoding reals as sequences of rationals), and connects it to classical algebraic geometry. It's a different animal from Weihrauch's approach, and understanding both perspectives is valuable.

## On Computable Analysis

Weihrauch's *Computable Analysis* (2000) is the standard reference for Type-2 computability over real numbers. The key point to internalize is that the "right" notion of computability for real-valued functions forces continuity — this isn't a restriction, it's the content of the theory. If you find yourself wanting to compute a discontinuous function, you've left the computable realm.

The Braverman-Yampolsky results on Julia sets are in their book *Computability of Julia Sets* (Springer, 2009). The central theorem — that there exist parameter values $c$ for which $\mathcal{J}(f_c)$ is not computable — is one of the most striking results connecting complex dynamics and computability theory. The proof uses the concept of "computable topology" in a deep way.

## On Algorithmic Ergodic Theory

The arithmetic hierarchy of dynamical properties is surveyed in Hoyrup-Rojas-Weihrauch (2012). The key results classifying transitivity, minimality, and recurrence at specific levels of the hierarchy are more subtle than they look — the proofs require careful encoding arguments.

Galatolo, Hoyrup, and Rojas's work on computable ergodic theory appears in *Dynamics and abstract computability: computing invariant measures* (Discrete and Continuous Dynamical Systems, 2011). This paper is the entry point for understanding when and why ergodic averages are computable. The connection to Martin-Löf randomness — that ML-random initial conditions are exactly those for which Birkhoff averages are computable — is the paper's central insight, and it ties together three previously separate fields: computability theory, ergodic theory, and algorithmic randomness.

## Looking Ahead

Chapter 32 will revisit these computability questions from the descriptive set-theoretic angle. The arithmetic hierarchy is a "bottom-up" classification (built from Turing machine computations); the Borel hierarchy is a "top-down" classification (built from topological operations on Polish spaces). For dynamical systems, these two hierarchies align in a way that explains both the computability results here and the classification results there.
