# Chapter Notes — Chapter 43

**HoTT fundamentals.** The HoTT Book (Univalent Foundations Program, 2013) is free online at homotopytypetheory.org/book — read it. Voevodsky's original motivations are in *An experimental library of formalized mathematics based on univalent foundations* (Mathematical Structures in Computer Science, 2015). The Lean 4 / Mathlib project is the current primary implementation of HoTT-inspired formalized mathematics; see leanprover-community.github.io.

**Guarded recursion and coinduction.** Nakano's *A Modality for Recursion* (LICS, 2000) introduced the guarded modality $\triangleright$. Birkedal-Møgelberg-Schwinghammer-Støvring: *First steps in synthetic guarded domain theory* (LMCS, 2012). Atkey-McBride's *Productive Coprogramming with Guarded Recursion* (ICFP, 2013) is very readable. For the connection to coinductive streams in type theory, see the survey by Abel (2013).

**Cohesive HoTT.** Shulman's *Brouwer's fixed-point theorem in real-cohesive homotopy type theory* (Math. Structures in Comp. Sci., 2018). Schreiber's *Differential cohomology in a cohesive infinity-topos* (2013, available on arXiv) develops the full framework.

**Lawvere's fixed point theorem.** Lawvere's original: *Diagonal arguments and cartesian closed categories* (in *Category Theory, Homology Theory and their Applications*, 1969). The unification of Cantor, Gödel, Turing, and Rice is in Yanofsky's *A universal approach to self-referential paradoxes, incompleteness and fixed points* (Bull. Symb. Logic, 2003) — highly recommended for its clarity.

**Formalized ergodic theory.** Avigad-Hölzl-Serafin's formalization of the ergodic theorem in Isabelle/HOL: see Hölzl's Isabelle/HOL ergodic theory library. The Lean/Mathlib formalization of information theory is ongoing; the current state is visible in the Mathlib4 repository at github.com/leanprover-community/mathlib4. The Shannon AEP formalization by Dey-Neri-Scholer (2023) is the most recent substantial result.

**LTL and dynamics.** For temporal logic applied to dynamical systems verification, see Maler-Pnueli's work on model checking hybrid systems. The connection between LTL and coinductive type theory is explored in Bahr-Grathwohl-Møgelberg (2017).

**Where this is going.** The formalization of mathematics is accelerating rapidly. Within a decade, tools like Lean 4 with Mathlib will likely be able to formalize most of the results in this book — except possibly Ornstein's theorem, which remains a frontier. The interface between HoTT's constructive philosophy and the classical measure theory of ergodic systems will be where the interesting foundational questions live.
