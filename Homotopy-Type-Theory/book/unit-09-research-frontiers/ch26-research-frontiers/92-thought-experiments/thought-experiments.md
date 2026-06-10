# 92 — Thought Experiments

## What These Are

The thought experiments in this section are not exercises with answers at the back. They are genuine open questions — places where the mathematical situation is puzzling in ways that are not resolved by the existing theory. Some have partial answers. Some are completely open. Some connect to major foundational debates that have been running for decades.

The purpose of thinking through these questions is not to resolve them but to develop the kind of philosophical and mathematical intuition that good research requires. Frontier mathematics is full of situations where the technical tools are outrunning the conceptual understanding — where we can prove things we do not fully understand, or fail to prove things that seem obviously true. Sitting with these puzzles, taking them seriously, and developing your own positions on them is part of the work.

---

## Thought Experiment 1: If Brunerie's Number Were Provably Uncomputable

Suppose — this is not known to be the case — that someone proved the following theorem: in Book HoTT (with axiomatic univalence and no cubical computation rules), the Brunerie number n cannot be computed from the proof term in any formal sense. More precisely: suppose there is a proof that no algorithm, given the proof term of π₄(S³) = ℤ/nℤ, can output the value of n in finite time.

What would this mean?

**First interpretation: A failure of constructivity.** Type theory is supposed to be constructive — every proof should carry computable content. If the Brunerie number is provably uncomputable from the proof term, then Book HoTT admits proofs of existence that contain no computable content. This would be a serious foundational problem. It would mean that Book HoTT is, in this respect, more like classical mathematics than like constructive mathematics — you can prove that n exists and has a specific value, but you cannot find that value.

This is not a contradiction. Book HoTT is not purely constructive: the law of excluded middle is not assumed, but univalence (as a bare axiom with no computation rule) is not constructive either. The question is how far this non-constructivity extends.

**Second interpretation: A separation from Cubical HoTT.** In Cubical Agda, the Brunerie number can be computed (by running the type-checker). If it could be proved uncomputable in Book HoTT, this would establish a formal separation between Book HoTT and Cubical HoTT: they prove the same theorems, but Cubical HoTT has computational content that Book HoTT lacks.

This would vindicate the CCHM approach: not just "cubical is more convenient for computation" but "cubical has strictly more computational content than axiomatic HoTT."

**Third interpretation: A new notion of computation is needed.** Perhaps the Brunerie number is "uncomputable" only under a specific notion of computation — the one given by the type theory's reduction rules. But there might be a different notion of computation, using model-theoretic techniques or semantic normalization, under which the number is computable. This would suggest that the formal notion of "computation" in type theory is too narrow.

**What do you think?** If Book HoTT is genuinely less constructive than Cubical HoTT in this sense, does that make it a worse foundation, or just a different one? Is constructivity a prerequisite for a good foundation for mathematics, or is consistency enough?

---

## Thought Experiment 2: Can Every Classical Result Be Proved Synthetically?

The homotopy hypothesis says that ∞-groupoids and homotopy types are the same thing. In HoTT, this is very nearly a theorem: types are ∞-groupoids (by the path algebra), and every ∞-groupoid is presented by a Kan simplicial set (by the simplicial set model of type theory). 

But the following is not known: is every theorem of classical homotopy theory provable synthetically in HoTT?

**Arguments for yes.** Classical homotopy theory works in the category of Kan simplicial sets, which models HoTT. So every classical theorem has a model-theoretic statement that is true in HoTT. But having a model-theoretic truth is not the same as having a type-theoretic proof.

The more important "yes" argument: synthetic HoTT often proves results more cleanly than classical methods. The Seifert-van Kampen theorem in HoTT (as a statement about pushouts of HITs) is cleaner than the classical version (which requires open sets and path-connected spaces). Freudenthal is cleaner in HoTT than in classical homotopy. The EHP sequence structure is more transparent in the synthetic setting. If synthetics are cleaner, then perhaps every classical result admits a synthetic proof that is also cleaner — not just equivalent but better.

**Arguments for no.** Some classical results rely on specific model-category-theoretic techniques that have no obvious type-theoretic content. The Adams spectral sequence, for instance, uses the filtration of the stable homotopy groups by Adams filtration — a structure that depends on choosing resolutions in specific ways, which are not obviously type-theoretically natural. The Serre spectral sequence similarly. These are tools for computation, and it is not clear that synthetic HoTT has analogous computational tools.

More fundamentally: classical homotopy theory can use the axiom of choice and excluded middle freely, because it works in classical logic. HoTT's homotopy theory, if done constructively, cannot. Some classical theorems may require classical logic for their proofs — in which case they could not have constructive synthetic proofs, even in principle.

**A specific hard case.** The Nilpotence theorem (Devinatz-Hopkins-Smith 1988, building on Nishida 1973): the sphere spectrum has no nilpotent elements in dimensions other than 0. This is a deep result about stable homotopy theory that uses chromatic techniques. Is there a synthetic proof in HoTT? Nobody knows.

---

## Thought Experiment 3: What Would "Complete" Algebraic Topology Look Like?

Suppose someone wanted to formalize all of algebraic topology — all the theorems in, say, Hatcher's book, plus the theorems in Spanier, plus the theorems in May's "Concise Course," plus the current research in stable homotopy theory and chromatic homotopy.

How far away is this? What would it require?

**The current state.** The Cubical Agda library (2025) contains: π₁(S¹) = ℤ, the Hopf fibration, Freudenthal, Blakers-Massey, pushouts, van Kampen, covering spaces (partly), Eilenberg-MacLane spaces (partly), truncations, π₄(S³) = ℤ/2ℤ. This is maybe 10% of an undergraduate algebraic topology course, in terms of theorems.

**The missing pieces.** Homology and cohomology (beyond the definitions), the Künneth formula, the universal coefficient theorem, Poincaré duality, the Hurewicz theorem in full generality, characteristic classes, the Adams spectral sequence, the chromatic filtration — none of these are in any formalization library in any form close to the classical treatments.

**The fundamental difficulty.** Classical algebraic topology is both computational (you want to calculate specific groups) and structural (you want to know which spaces are equivalent). HoTT handles the structural questions beautifully (equivalences are identities, fiber sequences are exact sequences of types). But the computational questions — "what is the homology of RP²?" — require machinery that translates between the synthetic and the computational.

**An honest estimate.** If current rates of progress continue and a significant team of researchers directed their efforts at this goal, a "complete" formalization of undergraduate algebraic topology (Hatcher's book) would take somewhere between 10 and 30 years. Research-level algebraic topology (stable homotopy, chromatic) would take longer.

This is not a discouraging estimate. It means there is decades of interesting work to do, and the person who formalizes the Hurewicz theorem or the Adams spectral sequence in Cubical Agda will have done something genuinely significant.

---

## Thought Experiment 4: What Else Could Become Primitive?

Simplicial type theory made a radical move: it treated the morphisms of ∞-categories as primitive, adding a directed interval 2 to the type theory alongside the undirected interval I. The result is a type theory where "morphism" is a basic concept, not a derived one.

What other mathematical structures could be made primitive in a future type theory?

**Spectra and stability.** Classical homotopy theory is dominated by stable homotopy theory — the study of spectra rather than spaces. A "stable HoTT" with the sphere spectrum as a primitive type (not as a HIT, not as a limit construction, but as a primitive) might make stable homotopy theory cleaner. The spectrum 𝕊 would be a primitive type, and stable maps would be primitive inhabitants. What would the elimination principle for 𝕊 look like?

**Metrics and geometry.** Cohesive HoTT makes shape (the homotopy type of a space) primitive. But it does not make metric structure primitive. A type theory with a primitive notion of distance — where types come equipped with a metric and morphisms are required to be non-expansive — might give a synthetic foundation for metric geometry and analysis. The metric would need to satisfy the triangle inequality by a rule, not an axiom.

**Probability and measure.** A probabilistic type theory would have types whose elements are probability distributions, with a primitive notion of random variable and measurable function. The Giry monad (the probability monad on measurable spaces) would be a primitive type constructor. This connects to the active field of probabilistic programming languages.

**Quantum mechanics.** A quantum type theory would have types that are Hilbert spaces, with primitive notions of quantum state (vector in a Hilbert space) and measurement (projection-valued measure). The type theory would enforce the superposition principle (states can be combined linearly) and the no-cloning theorem (quantum states cannot be freely copied, which corresponds to linearity in the type system).

**The philosophical question.** HoTT's move was: instead of building homotopy theory out of set theory, make homotopy theory primitive. This turned out to be philosophically clarifying and mathematically powerful. Is there a general principle at work? Is there a class of mathematical structures that become cleaner when made primitive rather than derived? What is the criterion for a structure being "primitive-worthy"?

---

## Thought Experiment 5: Cohesive HoTT and the Physics Program

Urs Schreiber's program (ongoing since approximately 2012) proposes to formalize M-theory — the conjectured 11-dimensional theory that unifies the various string theories — in cohesive HoTT. The program is detailed in a series of papers and an extensive nLab development. The claim is that cohesive HoTT, with appropriate axioms, provides the natural language for higher gauge theory, supergravity, and the geometry of M-branes.

Is this a realistic research program or a beautiful dream?

**Arguments for realistic.** Cohesive HoTT does handle differential geometry synthetically: smooth manifolds, differential forms, de Rham cohomology, and gauge connections all have synthetic formulations in cohesive HoTT. The connection to physics is not analogical but formal: the gauge connection in Yang-Mills theory is literally a morphism in a ∞-groupoid of connections, and this is literally what Schreiber's formulation describes.

Schreiber's concrete claim: the equations of 11-dimensional supergravity (the low-energy limit of M-theory) can be derived from the cofiber sequence of a specific map in cohesive HoTT, using the "rational homotopy theory" approximation. This is a specific mathematical claim that could be verified or refuted.

**Arguments for dream.** M-theory itself is not a well-defined theory: it is a conjectured unification whose precise formulation is unknown even in physics. You cannot formalize a theory that has not been specified. Schreiber's program formalizes specific structures that *should* appear in M-theory (higher gauge fields, branes, anomaly cancellation conditions), but the claim that these constitute M-theory is physicists' intuition, not mathematics.

Additionally, the connection between cohesive HoTT and the actual physics (quantum field theories with path integrals, operator algebras, renormalization) is currently at the level of the classical (non-quantum) geometry. The quantum mechanics is missing.

**A more honest version of the claim.** Cohesive HoTT gives the right language for the *geometry* of M-theory. It does not currently formalize the *physics* of M-theory. The geometry without the physics is like describing the arena without the game.

Whether the full physics can be brought into the type-theoretic setting — with quantum mechanics as a primitive, not just as a metaphor — is the deep open question.

---

## Thought Experiment 6: The Homotopy Hypothesis as a Theorem *In* HoTT

The homotopy hypothesis says: ∞-groupoids and homotopy types are the same thing. This is a theorem *about* HoTT (it is a theorem about the models of HoTT — in particular, the simplicial set model). But what would it mean to prove the homotopy hypothesis *inside* HoTT?

**The problem.** Inside HoTT, types are already ∞-groupoids by definition (the path algebra gives them ∞-groupoid structure). So there is nothing to prove: every type is an ∞-groupoid, and every ∞-groupoid can be... what? Represented as a type? But HoTT has no external notion of ∞-groupoid to compare types to.

This seems to make the homotopy hypothesis trivially true in HoTT (types are ∞-groupoids by definition) and at the same time unprovable (there is no external standard against which to check the equivalence).

**One resolution.** You can state the homotopy hypothesis inside HoTT if you have a way to define ∞-groupoids without referring to types. The Finster-Mimram definition of weak ω-categories (LICS 2017) uses globular type theory — a type theory where the basic shapes are globes rather than paths. In this setting, ∞-groupoids are defined independently of types, and you can ask whether the ∞-groupoid structure on a type (given by its path algebra) is equivalent to the ∞-groupoid described by the globular structure.

**The deeper question.** Is "homotopy hypothesis" even a well-formed statement inside HoTT, or does it require a meta-level perspective? This connects to the general question of whether foundational statements can be expressed within the foundation they describe, or whether they are inherently meta-theoretic.

---

## Thought Experiment 7: Beyond ∞-Groupoids

HoTT works with ∞-groupoids (types with paths in all dimensions, all invertible). Simplicial type theory works with ∞-categories (types with morphisms in all dimensions, not necessarily invertible). What comes next?

**∞-categories with duals.** An ∞-groupoid is an ∞-category where every morphism is invertible. An ∞-category with duals is an ∞-category where every morphism has a dual (adjoint) but the dual need not be an inverse. These appear naturally in topological field theories (via the cobordism hypothesis: fully dualizable objects classify fully extended TFTs). A type theory for ∞-categories with duals would be the natural foundation for TFT classification.

**(∞,2)-categories.** An (∞,2)-category has objects, morphisms, 2-morphisms, ..., with k-morphisms invertible for k > 2. The category of ∞-categories (Cat_∞) is an (∞,2)-category: ∞-categories, functors between them, and natural transformations between functors. A type theory for (∞,2)-categories would need a second directed interval (not just the simplicial interval 2 of STT, but a 2-dimensional directed shape) and corresponding extension types.

**ω-categories (strict).** Strict ω-categories have associativity and unitality as equalities (not just equivalences), at every level. The category of strict ω-categories is much more tractable computationally than the weak case — there is a Quillen model structure on strict ω-categories (Ara-Maltsiniotis 2020) — but the strict setting is not sufficient for most applications in topology (since homotopy theory is fundamentally weak).

**The fundamental question.** HoTT's power comes from making a specific mathematical structure primitive. The structures above each have a claim to being the "next" level: they generalize ∞-groupoids in specific ways that capture important mathematical phenomena. Which of these generalizations (if any) should be the next type theory? What would be the analogue of univalence for (∞,2)-categories? What would the analogue of the Brunerie problem look like at the (∞,2)-categorical level?

These are not questions with answers yet. They are the shape of the problems that will define HoTT research a decade from now.
