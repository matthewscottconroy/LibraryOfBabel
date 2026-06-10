# 93 — Exercises

## A Note on These Exercises

The exercises in every previous chapter of this curriculum had answers — in the back, in your head, or derivable from the material with sufficient thought. These exercises are different. Some are straightforward mathematical tasks at the edge of the material you have covered. Some are research problems that the field is actively working on. Some are genuinely open, and the "correct answer" is unknown.

The exercises are organized from most to least accessible. Do not skip to the hard ones. The early exercises build the habits and tools that the later ones require. A student who has completed Exercises 1–10 carefully is better positioned to make progress on Exercises 20–30 than a student who has skimmed the easy ones and rushed to the open problems.

For exercises marked **[OPEN PROBLEM]**: these are research problems. Working seriously on them, even if you do not solve them, is valuable — it develops your understanding of the problem and builds the technical machinery that a solution would require. Partial progress is real progress. Document what you find.

For exercises marked **[FORMALIZATION]**: these ask you to write actual Agda or Rzk code. Set up your environment before starting. Expect the formalization to take significantly longer than the mathematics suggests.

---

## Section A: Reading and Understanding (Exercises 1–8)

**Exercise 1.** Read the introduction and Chapter 2 of Brunerie's PhD thesis (arXiv:1606.05916, pages 1–30). After reading, answer the following:

(a) What is the Brunerie number n? State precisely: it is defined as the degree of what map?
(b) Why does showing n = ±2 imply π₄(S³) = ℤ/2ℤ? What is the structure of the argument?
(c) What are the three main ingredients of Brunerie's proof? (The thesis lists them explicitly in the introduction.)
(d) The EHP long exact sequence relates which three homotopy groups? Write the relevant part of the exact sequence.
(e) At which step does the proof require an explicit computation (rather than just a structural argument)? Describe the computation in one paragraph.

**Exercise 2.** Read the abstract and introduction of the Ljungström-Mörtberg LICS 2023 paper. Then answer:

(a) What does "symmetric monoidal smash product" mean informally? Why does it help reduce the size of the proof term?
(b) The paper says the computation is now "feasible" in Cubical Agda. What does "feasible" mean here — how is it measured (time? memory? number of reduction steps)?
(c) What is the difference between the computation being "feasible" and the proof being "conceptually transparent"? Give an example of a computation that is feasible but not transparent (from any area of mathematics or computer science).

**Exercise 3.** State the Freudenthal suspension theorem precisely: for an n-connected space X, the natural map X → Ω Σ X induces an isomorphism on πₖ for k < 2n and a surjection on π_{2n}.

(a) What does "n-connected" mean as a type-theoretic statement? (Use the language of h-levels and truncation.)
(b) The base case of the computation of π_k(Sᵏ) = ℤ is k = 1. State this case and indicate where it is proved (in the HoTT Book and in the Cubical Agda library).
(c) Write out the inductive step: assume π_n(Sⁿ) = ℤ and use Freudenthal to conclude π_{n+1}(Sⁿ⁺¹) = ℤ. What connectivity condition do you need on Sⁿ to apply Freudenthal?
(d) Is the full induction (showing π_n(Sⁿ) = ℤ for all n ≥ 1) formalized in the Cubical Agda library? If not, what is the specific obstacle?

**Exercise 4.** Canonicity and its failure. This exercise works through the canonicity problem for Book HoTT concretely.

(a) Define canonicity for a type theory precisely: what does it mean for every closed term of type ℕ to "normalize to a numeral"?
(b) In Cubical Agda, evaluate the term `transport (ua succ-equiv) 3`, where `succ-equiv : ℕ ≃ ℕ` is the equivalence given by the successor function. What does this reduce to, and why? (The answer is 4; trace through the computation rule for `transport` along a `Glue` type.)
(c) In Book HoTT (axiomatic univalence), why is `transport (ua succ-equiv) 3` stuck? What reduction rule is missing?
(d) State Shulman's homotopy canonicity result. How is it weaker than full canonicity? Give an example of a theorem that homotopy canonicity guarantees but full canonicity would additionally provide.

**Exercise 5.** Read the abstract and Sections 1–2 of the Riehl-Shulman paper (arXiv:1705.07442). Then:

(a) What is a Segal type? Give the type-theoretic statement of the Segal condition.
(b) What is a Rezk type? What is the Rezk condition, and how does it relate to the "Rezk completeness" of an ∞-category?
(c) State the synthetic Yoneda lemma as it appears in the paper (or in the Kudasov-Riehl-Weinberger formalization). Be precise about the types involved.
(d) What is directed univalence, and why is it the "natural" analogue of univalence for ∞-categories? State the open problem precisely.

**Exercise 6.** The Blakers-Massey theorem. State the theorem precisely:

(a) In the language of pushouts and connectivity, using the HoTT Book's notation.
(b) In the language of homotopy excision, using the classical algebraic topology formulation (Blakers-Massey, 1951).
(c) What is the statement in an arbitrary ∞-topos (the Anel-Biedermann-Finster-Joyal version)? What additional generality does this give beyond the HoTT statement?
(d) What would it mean for the bound to be sharp? Give a specific example (from classical homotopy theory) that achieves the bound m + n - 1.

**Exercise 7.** Browse the open issues on the Cubical Agda library (github.com/agda/cubical/issues) for one hour. Then:

(a) Identify three issues you understand well enough (given this curriculum) to attempt. For each, write a precise statement of what theorem needs to be proved.
(b) For each of the three, identify: what library infrastructure already exists, what the key missing ingredient is, and how you would approach a proof.
(c) Rank the three by difficulty. Defend your ranking with specific mathematical reasoning.

**Exercise 8.** The HoTT Zulip. Register at hott.zulipchat.com. Read the "general" stream for two weeks. Then:

(a) Summarize two distinct research discussions you observed. What problems were being discussed, who was involved, and what progress (if any) was made?
(b) Identify one researcher active in the Zulip who is working on a problem related to something in Chapters 20–26 of this curriculum. Describe their research program in one paragraph, citing specific posts or papers.
(c) Find one question posted by someone stuck on a formalization problem. What was the question? Was it answered? If so, what was the answer?

---

## Section B: Mathematical Work (Exercises 9–18)

**Exercise 9.** [FORMALIZATION] Set up Cubical Agda and the Cubical library. Load the file `Cubical/HITs/S1/Base.agda` and read it carefully.

(a) Write a new Agda file that defines a map f : S¹ → S¹ by `f base = base` and `f loop = loop ∙ loop`. Prove that f is not homotopic to the identity map. (Hint: the winding number of f is 2, and the winding number of the identity is 1.)
(b) Define the n-fold cover of S¹ as a HIT or as a fiber bundle. Check that the fiber over `base` is the set {0, 1, ..., n-1}.
(c) Write out the type of the statement π₁(S¹) = ℤ as an Agda type. Verify that the statement compiles (even if you leave the proof as a hole).

**Exercise 10.** The EHP sequence. This is a long exact sequence of homotopy groups:

    ... → π_{n+1}(S²ⁿ⁻¹) →^H π_{n+1}(S^n) →^E π_n(S^{n-1}) →^P π_n(S²ⁿ⁻¹) → ...

where E is the suspension homomorphism, H is the Hopf invariant, and P is the "P-map" (a boundary map from the fibration Ω(S^n) → Ω(S^{2n-1})).

(a) For n = 2: specialize the EHP sequence to give a long exact sequence involving π_k(S³), π_k(S²), and π_k(S¹). Use this to compute π₄(S²) = ℤ/2ℤ, given π₄(S³) = ℤ/2ℤ and π₃(S²) = ℤ.
(b) For n = 3: specialize the EHP sequence to give a long exact sequence involving π_k(S⁵), π_k(S³), and π_k(S²). What would you need to know to compute π₅(S³) from this sequence?
(c) In what sense is the EHP sequence "already present" in the HoTT formalization of the Hopf fibration? What additional work would be needed to extract the full long exact sequence from the existing Cubical Agda code?

**Exercise 11.** K-theory in HoTT. This exercise works through the definition of K₀(R) type-theoretically.

(a) Define, as a type in HoTT, the set of isomorphism classes of finitely generated projective R-modules, for R a commutative ring. Why does univalence make this definition cleaner than the classical definition?
(b) Define the addition operation on isomorphism classes (direct sum). Check that it satisfies commutativity and associativity.
(c) Define K₀(R) as the group completion of the monoid of isomorphism classes. (Use the group completion HIT: the type with generators [M] for each projective module M, and relations [M ⊕ N] = [M] + [N], and group axioms.)
(d) Compute K₀(ℤ): show that every finitely generated projective ℤ-module is free (of some rank n), so K₀(ℤ) = ℤ.

**Exercise 12.** Stable homotopy groups. The stable homotopy groups of spheres πₙˢ = colim_k π_{n+k}(Sᵏ) stabilize for k > n + 1 (by Freudenthal).

(a) Compute πₙˢ for n = 0 (trivial), n = 1 (ℤ/2ℤ, generated by the suspension of η), and n = 2 (ℤ/2ℤ, generated by the suspension of ν, where ν is related to the Hopf fibration).
(b) What is the "stable Hopf map" η : 𝕊 → 𝕊 in the sphere spectrum? How is it related to the Hopf fibration η : S³ → S²?
(c) In HoTT language: what would it mean to define the sphere spectrum 𝕊 as a type? What would its "elements" be? What would π_n(𝕊) mean?

**Exercise 13.** Directed type theory. This exercise explores the Segal condition and its consequences.

(a) State the Segal condition for a type A: for any x, y, z : A and composable arrows f : hom_A(x, y) and g : hom_A(y, z), there is a unique composite g ∘ f : hom_A(x, z). Write this as a type-theoretic statement using extension types.
(b) The universe U (the type of types in HoTT) is not itself a Segal type: the space of functors between two types is an ∞-groupoid, not an ∞-category. In STT, what modification of U would make it Segal?
(c) The Rezk condition for a Segal type A: every isomorphism in A (a morphism with a two-sided inverse) is equal (as an element of A) to an identity morphism. Why is this the "right" condition for A to be an ∞-category (not just an ∞-precategory)?
(d) State directed univalence precisely: for the conjectured Segal type Cat of ∞-categories, what does directed univalence assert? Why is this stronger than the Rezk condition for individual Segal types?

**Exercise 14.** Cohesive HoTT and the Brouwer fixed-point theorem. Work through the proof from Shulman's paper.

(a) State the Brouwer fixed-point theorem synthetically, using the language of cohesive HoTT: what is the type-theoretic statement, and what modalities does it use?
(b) The proof uses the shape modality ʃ. What does ʃ(Dⁿ) compute, and why? (Dⁿ is the n-dimensional disk in smooth cohesive HoTT.)
(c) The key step: Brouwer's theorem follows from the fact that there is no retraction of Dⁿ onto ∂Dⁿ = Sⁿ⁻¹. Prove this in cohesive HoTT, using the fact that ʃ(Dⁿ) is contractible and ʃ(Sⁿ⁻¹) = Sⁿ⁻¹ is not.
(d) Where does the proof use specifically cohesive structure (not just ordinary HoTT)? Could the proof be done in ordinary HoTT if Dⁿ were replaced by its underlying set?

**Exercise 15.** [FORMALIZATION] Find a `sorry` lemma in the Cubical Agda library (search for "sorry" or "postulate" in the Homotopy subdirectory). Read the context to understand what theorem it is a placeholder for.

(a) State the theorem precisely, both in Agda syntax and in mathematical prose.
(b) Write a sketch of the proof in mathematical prose. Identify the key steps.
(c) Attempt to fill in the `sorry` in Agda. If you succeed, submit a pull request. If you do not, document: (i) how far you got, (ii) where you got stuck, (iii) what additional library infrastructure would be needed.

**Exercise 16.** The Hopf invariant. The Hopf invariant of a map f : S²ⁿ⁻¹ → Sⁿ is an integer defined by the attaching map.

(a) Define the Hopf invariant of the Hopf map η : S³ → S² geometrically: H(η) = 1.
(b) Define the Hopf invariant type-theoretically, using the cup product in cohomology: if Cf = Sⁿ ∪_f e²ⁿ is the cofiber of f, then H(f) is the integer such that the cup product map α ∪ α = H(f) · β in H²ⁿ(Cf), where α ∈ Hⁿ(Cf) and β ∈ H²ⁿ(Cf) are generators.
(c) The Hopf invariant one problem (Adams 1960): for which n does there exist a map f : S²ⁿ⁻¹ → Sⁿ with H(f) = 1? The answer is n = 1, 2, 4, 8. Explain the connection to the existence of real division algebras (ℝ, ℂ, ℍ, 𝕆).
(d) In HoTT, how is the Hopf invariant defined? What cohomology theory does HoTT use, and how does the cup product arise?

**Exercise 17.** The Grothendieck construction in STT. The Grothendieck construction converts a functor F : C → Cat into a fibration ∫F → C.

(a) State the Grothendieck construction for functors between 1-categories precisely.
(b) In STT, functors between Segal types are morphisms in the Segal type of Segal types. State what a "left fibration" of Segal types should be (a morphism p : E → B such that...?).
(c) The ∞-categorical Grothendieck construction (Lurie's "straightening-unstraightening" equivalence) says: left fibrations over B are equivalent to functors B → Gpd_∞ (the ∞-category of ∞-groupoids). State this as a type-theoretic theorem in STT.
(d) This theorem is partially formalized in sHoTT. Look up the current state of the formalization (in the sHoTT repository). What is proved and what remains as a sorry?

**Exercise 18.** [OPEN PROBLEM] This is a research problem. Spend at least 5 hours on it and document your progress.

The inductive step for π_n(Sⁿ) = ℤ: the Freudenthal theorem is in the Cubical Agda library at `Cubical.Homotopy.FreudenthalSuspension`. The base case π₁(S¹) = ℤ is at `Cubical.HITs.S1.Properties`.

Attempt to formalize the inductive step: given `π_n(Sⁿ) ≃ ℤ`, deduce `π_{n+1}(Sⁿ⁺¹) ≃ ℤ` using Freudenthal.

Document: (a) how you set up the induction, (b) what connectivity conditions you need, (c) where the proof gets stuck (if it does), (d) what additional library lemmas you needed or wanted, (e) whether you succeeded, and if not, what the specific obstacle was.

---

## Section C: Open Research Problems (Exercises 19–30)

These exercises are genuine research problems. They are not "exercises" in the sense of having known solutions that you are expected to find. They are problems the community is working on. Working seriously on any of them — even if you do not solve them — builds exactly the skills and knowledge that research in HoTT requires.

**Exercise 19.** [OPEN PROBLEM] Write a research proposal for a new conceptual proof of the Brunerie number computation.

A "conceptual" proof would be one where, at each step, a human can check the reasoning without relying on the machine to handle computationally opaque steps. It need not avoid all computation, but the computation should be at a scale where a careful reader can verify it manually.

Your proposal should:
(a) Identify the specific step in Brunerie's proof (or the Ljungström-Mörtberg reformulation) where the computation becomes opaque.
(b) Suggest one alternative approach to this step — either a different definition of the Brunerie number, or a different computational strategy.
(c) Identify what new mathematics or new library infrastructure your alternative approach would require.
(d) Give an honest assessment: how likely is your approach to succeed, and why?

**Exercise 20.** [OPEN PROBLEM] Read de Jong and Escardó's LICS 2023 paper on small types in univalent foundations. Then:

(a) State the main theorem of the paper.
(b) The paper shows that certain cardinality assumptions (which hold in ZFC but are not provable in HoTT) have type-theoretic consequences. Identify one such assumption and its HoTT consequence.
(c) Does the paper have implications for canonicity for Book HoTT? If so, what are they?

**Exercise 21.** [OPEN PROBLEM] Directed univalence. This exercise is meant to help you understand exactly what the problem is asking.

(a) In ordinary HoTT, univalence is equivalent to the statement that the canonical map (A = B) → (A ≃ B) is an equivalence for all A, B : U. State the analogous statement for Segal types and their universe Cat.
(b) The "Rezk condition" for a specific Segal type C says: the map (a = b) → (a ≅ b) is an equivalence for all a, b : C, where a ≅ b means "there exist morphisms f : hom(a, b) and g : hom(b, a) that are mutual inverses." State the Rezk condition for Cat.
(c) Why is proving the Rezk condition for Cat harder than proving it for an individual Segal type? What self-referential difficulty arises?
(d) Propose one strategy for making directed univalence precise enough to attempt a proof, even if you cannot prove it.

**Exercise 22.** [OPEN PROBLEM] The general syntax for HITs. This exercise explores Problem 3.

(a) The circle S¹ has one point constructor and one path constructor. The torus T² has two point constructors and three path constructors (one for each generator of π₁(T²) and one relating them). Write the HIT specification for T² explicitly.
(b) The Eilenberg-MacLane space K(ℤ, 2) has infinitely many coherence conditions in any explicit presentation. Explain why: what goes wrong if you try to specify K(ℤ, 2) with finitely many constructors?
(c) The Lumsdaine-Shulman semantics (2020) handles HITs that can be specified as "cell monads." Can K(ℤ, 2) be specified as a cell monad? If not, what class of HITs includes it?
(d) Propose a definition of "valid HIT specification" that would include K(ℤ, n) for all n. What would need to be proved to show this definition gives a consistent type theory?

**Exercise 23.** [OPEN PROBLEM] Stable homotopy in HoTT. This exercise explores the infrastructure needed for stable homotopy theory.

(a) Define a "spectrum" in HoTT: a sequence of types (Xₙ)_{n:ℕ} together with equivalences σₙ : Xₙ ≃ ΩXₙ₊₁.
(b) The sphere spectrum 𝕊 has Xₙ = Sⁿ (the n-sphere) with σₙ : Sⁿ → Ω Sⁿ⁺¹ given by the Freudenthal map. Define 𝕊 as a type in HoTT (or sketch such a definition using an inductive type on ℕ).
(c) The stable homotopy groups πₙˢ(𝕊) = colim_k π_{n+k}(Sᵏ) are the homotopy groups of the sphere spectrum. How would this colimit be defined in HoTT? What HIT would represent a sequential colimit?
(d) What would be needed to compute π₁ˢ(𝕊) = ℤ/2ℤ synthetically in HoTT?

**Exercise 24.** Survey paper. Choose one of the open problems from Section A of Chapter 26 (Brunerie's number, canonicity, HIT syntax, directed univalence, π₅(S⁴), Blakers-Massey sharpness, stable homotopy). Write a 4–6 page survey:

(a) Precise statement of the problem.
(b) History: when was it first posed, what progress has been made, by whom.
(c) The main obstacle: what specifically makes the problem hard.
(d) Related problems: what other open problems are connected to this one, and how.
(e) Your assessment: where do you think progress is most likely to come from?

Write this as if presenting to a colleague who is a competent mathematician but not a HoTT specialist.

**Exercise 25.** [FORMALIZATION] [OPEN PROBLEM] Formalize the statement (not the proof) of the Blakers-Massey sharpness result in Cubical Agda.

That is: write an Agda type that expresses "for each m, n ≥ 1, there exist maps f : A → B and g : A → C such that f is exactly m-connected, g is exactly n-connected, and the comparison map A → B ×_{B ∪_A C} C is exactly (m+n-1)-connected (not more)."

Note: "exactly k-connected" means k-connected but not (k+1)-connected. Defining this precisely requires care: it means the k-th homotopy group of the homotopy fiber is non-trivial.

Submit your formalization attempt (even a partial one) as a comment on the relevant Cubical Agda library issue, if one exists.

**Exercise 26.** [OPEN PROBLEM] The Hopf fibration and associativity of joins. The Hopf fibration in Cubical Agda uses the join S¹ * S¹ to construct S³. But to verify that S¹ * S¹ is actually equivalent to S³ (and not just defined to be so), you need the associativity of joins: (X * Y) * Z ≃ X * (Y * Z).

(a) State join associativity as a type-theoretic statement.
(b) Identify where this statement appears (or does not appear) in the Cubical Agda library.
(c) Sketch a proof strategy: what is the natural equivalence, and what do you need to verify?
(d) Is this statement currently in the library? If not, attempt to prove at least the map (X * Y) * Z → X * (Y * Z) in Agda, even if you cannot verify it is an equivalence.

**Exercise 27.** [OPEN PROBLEM] Read the Finster-Mimram LICS 2017 paper "A Type-Theoretical Definition of Weak ω-Categories." Then:

(a) What is a "globular type" in their sense? How does it differ from a type in ordinary HoTT?
(b) How do they define weak ω-categories type-theoretically? What is the key inductive structure?
(c) The homotopy hypothesis would say: the weak ω-category defined by a type A (using its path algebra) is equivalent to the globular weak ω-category in Finster-Mimram's sense. Is this equivalence stated and proved in their paper? If not, is it known at all?
(d) What would be needed to prove the homotopy hypothesis *inside* a type theory that includes both Finster-Mimram's globular types and HoTT's path types?

**Exercise 28.** [OPEN PROBLEM] Chromatic homotopy and HoTT. This is a speculative research problem.

The chromatic filtration of the sphere spectrum is:

    ... → L₂𝕊 → L₁𝕊 → L₀𝕊

where Lₙ𝕊 is the Morava K(n)-localization. At the prime p = 2: L₀𝕊 = 𝕊_ℚ (rational sphere spectrum), L₁𝕊 = KO-theory (roughly), and Lₙ𝕊 for n ≥ 2 involves exotic arithmetic.

(a) Define the rationalization of a type as the p = 0 localization. How does the p-localization modality in cohesive HoTT relate to this?
(b) The rational sphere spectrum 𝕊_ℚ has stable homotopy groups πₙˢ(𝕊_ℚ) = ℚ for n = 0 and 0 otherwise. State this in HoTT language and identify what theorem would give this computation.
(c) Morava K-theory K(1) at the prime p is related to complex K-theory (KU) by K(1)-localization. In HoTT, KU should be defined using the classifying space BU(n) for the unitary groups. What are the ingredients needed to define K(1) in HoTT?
(d) Write a one-page research proposal for defining at least L₀𝕊 in Cubical Agda and computing its homotopy groups.

**Exercise 29.** [OPEN PROBLEM] Condensed mathematics and cohesive HoTT. Read Section 1 of Scholze's "Condensed Mathematics" notes (freely available) and Section 3 of Myers' "Simplicial, Divisorial, and Orientable Cohesion."

(a) What is a condensed set? What is a pyknotic set? How do they differ?
(b) In Myers' cohesive HoTT, what is the "pyknotic" version of cohesion? What axioms characterize it?
(c) The "liquid tensor experiment" formalized Scholze's theorem on liquid ℝ-modules in Lean 4. What is the statement of this theorem, informally?
(d) What would be needed to connect the Lean 4 liquid tensor formalization to cohesive HoTT? That is: what translation between the classical Lean 4 foundation and the cohesive HoTT foundation would be required?

**Exercise 30.** [OPEN PROBLEM] This is the final exercise, and the most open-ended.

Design a new type theory. It should:
(a) Extend HoTT in some direction not covered by cubical, simplicial, or cohesive HoTT.
(b) Have a clear mathematical motivation: there should be a specific class of mathematical structures that it handles more naturally than existing type theories.
(c) Be described precisely enough that its consistency could in principle be checked: state the new type constructors, their elimination principles, and the computation rules.
(d) Have at least one non-trivial theorem that would be easier to prove in your type theory than in existing ones.

This exercise has no correct answer. Its value is in the attempt: designing a type theory forces you to understand what type theories are, what they can do, and why the existing ones are shaped the way they are. Present your design and discuss its limitations honestly.
