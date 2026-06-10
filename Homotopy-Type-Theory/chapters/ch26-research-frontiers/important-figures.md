# Important Figures

Chapter 26 maps the research frontier, so its important figures are a mix: foundational architects whose work made the current research possible, and active researchers working on the specific open problems the chapter describes. The section on the HoTT Book authors gestures toward the collaborative nature of the entire enterprise.

---

## Vladimir Voevodsky (1966–2017)
*Founder of the Univalent Foundations Program; Fields Medalist; mathematician at the Institute for Advanced Study*

Vladimir Voevodsky received the Fields Medal in 2002 for his work on motivic cohomology and the proof of the Milnor conjecture — a major result in algebraic K-theory that had been open for thirty years. This work required developing an entirely new framework (the motivic cohomology of algebraic varieties) and using it to relate algebraic K-theory to Galois cohomology. The proof was celebrated as one of the great achievements of late 20th-century mathematics.

In the years after his Fields Medal work, Voevodsky became concerned with a question that was unusual for a working mathematician: how do we know our proofs are correct? His concern was not philosophical but practical. A proof he had published contained an error that went undetected for several years. He found that even with careful refereeing, mathematical proofs of sufficient complexity are not reliably checked by humans. The solution he arrived at was formalization: writing proofs in a language that a computer can check. But the proof assistants available at the time (Coq, Agda) used foundations (ZFC-based set theory, or intensional type theory without univalence) that made it extremely difficult to formalize modern algebraic topology.

Voevodsky's univalence axiom was the breakthrough: by adding the axiom that equivalent types are equal, he made it possible to work with mathematical structures "up to isomorphism" in a way that is both foundationally correct and practically manageable. The insight that univalence is compatible with type theory — and that types have homotopy-theoretic content — came from his study of the simplicial set model of type theory. He presented the idea in a 2010 IAS lecture that electrified the type theory community. The subsequent Univalent Foundations Program (2012–2013 at IAS) assembled the community that produced the HoTT Book. Voevodsky continued to develop the UniMath library (in Coq) until his sudden death in 2017 at age 51.

The entire HoTT research enterprise is, in a direct sense, the continuation of Voevodsky's program. Every formalization library, every proof of a new theorem in HoTT, every application of univalence is part of what he started. His intellectual arc — from the most abstract algebraic geometry to the most foundational questions about the nature of mathematical proof — is the motivating story of the field.

---

## Guillaume Brunerie (1988–present)
*Author of the first HoTT proof of $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$; researcher at Stockholm University*

Guillaume Brunerie completed his PhD at the Université Nice Sophia Antipolis in 2016 under Carlos Simpson, producing a thesis that proved $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ entirely within homotopy type theory. The proof required constructing the Hopf fibration, defining the Hopf invariant, setting up the EHP long exact sequence, and computing a specific integer — the "Brunerie number" — whose value encodes the answer.

The Brunerie number is defined within the proof: it is the integer $n$ such that the composite map $S^3 \to S^2 \vee S^2 \to \Omega \Sigma S^2$ (constructed from the Hopf map and the James splitting) has Hopf invariant $n$. The proof shows that if $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ then $n = \pm 2$, but the proof also requires showing $n = 2$ (or $n = -2$) by an explicit computation. This computation — that a specific higher path in a specific type reduces to the standard generator of $\mathbb{Z}/2\mathbb{Z}$ — was so involved that it could not be verified by hand: the original proof relied on the Agda type-checker to confirm the computation.

Brunerie's work is the canonical "hard problem in synthetic homotopy theory." The computation of $\pi_4(S^3)$ by classical methods (using stable homotopy theory, the Adams spectral sequence, etc.) is a few pages once the machinery is set up. The HoTT proof is over 100 pages of dense synthetic homotopy theory. This gap between classical and synthetic proof lengths is the central challenge that the HoTT community is working to close — finding cleaner, more conceptual proofs of results that classical topology proves with powerful but non-synthetic machinery.

Since his PhD, Brunerie has continued to contribute to HoTT, including work on the formalization of the Brunerie number and the relationship between synthetic and classical homotopy theory. The "Brunerie number problem" — finding a cleaner proof that $n = 2$ — remains a famous open problem in the field.

---

## Axel Ljungström and Anders Mörtberg
*Authors of the 2022–2023 computational simplification of the Brunerie number*

**Axel Ljungström** is a PhD student at Stockholm University, and **Anders Mörtberg** is a professor at Stockholm University who was previously at Carnegie Mellon and IAS. Mörtberg is one of the four authors of CCHM cubical type theory (the "M" in CCHM) and a core developer of Cubical Agda. His contributions to cubical type theory — the construction of the Glue type, the implementation of the type-checker, the design of the library — have shaped modern HoTT formalization.

Their 2022–2023 work on the Brunerie number is the current state of the art. The key achievement: they found a formulation of the computation (the evaluation of the Brunerie number) that could actually be run by the Cubical Agda type-checker in a reasonable time. Brunerie's original proof involved a term so large that checking it would have required impractical amounts of time or memory. Ljungström and Mörtberg found "shortcuts" — reformulations of intermediate steps — that reduced the computation to something machine-feasible.

This work is significant for two reasons. First, it actually resolves the computational question: we now have a machine-verified proof that $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$. Second, the techniques they developed — especially the "symmetric monoidal smash products" framework — gave new mathematical insight into why the computation works, pointing toward the "conceptual" proof that is still being sought. Their work exemplifies the symbiotic relationship between formalization and mathematical insight: the formalization forced a clearer understanding of the mathematics, which led to a cleaner formalization.

---

## Emily Riehl and Michael Shulman (the Directed Univalence Problem)
*Formulators of the central open problem in STT*

Emily Riehl and Michael Shulman are discussed in depth in the Chapter 24 Important Figures section, where their roles as co-creators of simplicial type theory are described. Their relevance to Chapter 26 is specific: they are the originators of the open problem of directed univalence.

Directed univalence asks: is there a Segal type $\mathsf{Cat}$ of ∞-categories in STT such that two ∞-categories are equal in $\mathsf{Cat}$ if and only if they are equivalent as ∞-categories (fully faithful and essentially surjective)? This is the ∞-categorical version of the univalence axiom for the universe of types. Riehl and Shulman stated this as an open problem in their 2017 paper and have returned to it repeatedly in subsequent work and lectures. As of 2025, it remains unresolved.

The difficulty: in ordinary HoTT, univalence is an axiom (in Book HoTT) or a theorem (in cubical HoTT, from the Glue type). In STT, the analogue would require defining the universe of ∞-categories as a Segal type and proving the Rezk condition for it. The Rezk condition for a specific Segal type $\mathcal{C}$ (that isomorphisms and paths agree in $\mathcal{C}$) is proved for each $\mathcal{C}$ individually. But the Rezk condition for the universe of all Segal types would be a global statement about the entire type theory, analogous to univalence. Whether this is consistent, what its computational content would be, and how it relates to the classical theory of ∞-category equivalences are all open.

---

## The HoTT Book Authors (Collective, 2013)
*The collaborative foundation of the field*

The HoTT Book was written by over fifty authors over the course of the 2012–2013 academic year at the Institute for Advanced Study. The list includes Thorsten Altenkirch, Steve Awodey, Marc Bezem, Ulrik Buchholtz, Thierry Coquand, Eric Finster, Dan Grayson, Martin Hofmann, André Joyal, Nicolai Kraus, Chris Kapulkin, Peter Lumsdaine, Per Martin-Löf (contributor), Urs Schreiber, Michael Shulman, Bas Spitters, Thomas Streicher, Vladimir Voevodsky, and many others.

This collective authorship is worth noting not merely as a sociological fact but as a reflection of the field's intellectual culture. Mathematics is typically written by individuals or small groups; the HoTT Book represents a genuinely collaborative synthesis. The practical effect was a single, coherent reference text covering the entire foundation in consistent notation — something rare in a new field. The text has shaped the notation and conventions of every subsequent HoTT paper.

The HoTT Book community also established the norm of open access: the book was posted freely online on the day of publication, which was unusual for a Princeton University Press book. The Cubical Agda library, the Rzk library, and the sHoTT library have all maintained this norm. The HoTT Zulip is open to anyone who registers. The HoTTEST seminar videos are freely available. This culture of openness means that a motivated person, regardless of their institutional affiliation, can engage with current research at a level of depth that would have been unavailable in earlier decades of mathematics.

The names on the HoTT Book are a partial snapshot of the research community as of 2013. The field has grown since, with a new generation of researchers — Brunerie, Ljungström, Kudasov, Weinberger, Myers, de Jong, and many others — whose work has extended the foundations in directions the 2013 book could not anticipate. The ongoing development is documented in papers, in the Zulip, and in the formalization libraries: it is a living research program, not a completed edifice.
