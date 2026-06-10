# Applications: Chapter 9

The formal machinery of MLTT — the four judgments, the identity type, transport, ap, and the intensional/extensional distinction — is not just abstract logic. It is the engine of real mathematical discoveries and real engineering achievements. Here are six concrete applications.

---

## 1. The HoTT Book: A New Kind of Mathematical Collaboration

**The event:** In 2012–2013, Vladimir Voevodsky organized a special year at the Institute for Advanced Study in Princeton on Univalent Foundations. About 30 mathematicians and computer scientists gathered to develop HoTT. The result, published in 2013, was the HoTT Book — an unusual document: a mathematics textbook produced by collective authorship, written in a formal system (MLTT with univalence), with all major theorems simultaneously formalized in Coq and Agda.

**The MLTT machinery at work:** The entire book is written inside MLTT. Every definition uses Π and Σ types. Every theorem is a type, and every proof is a term. The identity type is the central object of study. Transport, ap, and path induction are the tools used in every chapter.

**The significance:** The HoTT Book demonstrated, concretely, that MLTT is capable of expressing all of modern mathematics — not just the toy examples of type theory textbooks, but genuine theorems from algebraic topology, category theory, and homotopy theory. The Seifert-Van Kampen theorem (computing fundamental groups), the Freudenthal suspension theorem (stable homotopy theory), and the calculation of π₁(S¹) = ℤ are all proved in full MLTT, machine-checkable.

This is the first instance of a mathematical text whose theorems are simultaneously stated in natural language and verified by a proof assistant — and where the formalization is in the same foundational system as the mathematics itself.

---

## 2. UniMath: Voevodsky's Formalization Project

**The project:** UniMath (Univalent Mathematics) is a Coq library, initiated by Voevodsky and continued by a group of collaborators, that formalizes a substantial portion of abstract mathematics in MLTT with univalence. It includes categories, functors, natural transformations, presheaves, sheaves, the Rezk completion, and foundations of algebraic geometry.

**Why MLTT specifically:** Voevodsky designed UniMath to work within a minimal, well-understood foundation — exactly the intensional MLTT of this chapter, plus univalence. No other axioms. No classical logic. This minimalism is deliberate: a smaller foundation means fewer assumptions, fewer places for inconsistency, and cleaner mathematical content.

**The identity type in action:** In UniMath, the notion of "equality of categories" is formalized using the identity type of the type of categories. By univalence, this identity type is equivalent to the type of categorical equivalences — so "two categories are equal" means "they are equivalent." This is the correct mathematical notion (as opposed to strict equality of definitions), and MLTT makes it precise.

UniMath demonstrates that MLTT is not just theoretically adequate for mathematics but practically usable for formalization at scale.

---

## 3. The Fundamental Group of the Circle: A Theorem in MLTT

**The mathematical statement:** π₁(S¹, base) = ℤ — the fundamental group of the circle is the integers.

**Why it matters for MLTT:** This theorem is proved entirely within MLTT with the Univalence Axiom and the Higher Inductive Type for S¹. The circle S¹ is defined as a HIT with one point constructor (base) and one path constructor (loop : base = base). The fundamental group π₁(S¹, base) = Σ(p:base=base). [equivalence class info] is computed using the identity type machinery of Chapter 9.

**The proof strategy:** The proof uses the encode-decode method. Define:
- An "encoding" function encode : base = base → ℤ that assigns a winding number to each loop
- A "decoding" function decode : ℤ → base = base that produces the loop winding n times

Then show encode and decode are inverse (using J and the induction principle for S¹). The full proof requires transport, ap, and path induction in an essential way — not as convenience but as the only tools available.

**Why this is remarkable:** In classical algebraic topology, π₁(S¹) = ℤ is a standard result, proved using covering space theory or the Seifert-Van Kampen theorem. In HoTT, it is proved synthetically, inside the type theory, without any reference to topology as a separate mathematical domain. The result *is* the type theory. MLTT is both the foundation and the tool.

The full proof was formalized in Agda (HoTT Agda library) and Coq (the HoTT Coq library) and verified machine-checked.

---

## 4. The Blakers-Massey Theorem: Homotopy Theory in Type Theory

**The mathematical statement:** The Blakers-Massey theorem is a connectivity theorem from classical homotopy theory: if f : A → B is k-connected and g : A → C is l-connected, then the induced map from the pushout B ∪_A C into the "fat wedge" is (k+l)-connected.

**Why it matters:** This is a non-trivial theorem of algebraic topology, proved by Blakers and Massey in 1951. It has consequences including the Freudenthal suspension theorem (which gives the stable homotopy groups of spheres in the stable range).

**The MLTT proof:** Peter Lumsdaine, Michael Shulman, and others proved Blakers-Massey in HoTT in 2013. The proof uses the identity type, transport, path induction, and the notion of truncation (h-level). Crucially, the proof was later shown to give a *new proof* of Blakers-Massey in classical topology, discovered via the synthetic setting — the type-theoretic formulation revealed a simpler argument than the classical one.

**The significance:** MLTT, extended with HoTT's axioms, is not just a language for translating known mathematics. It is a tool for discovering new proofs. The synthetic setting, by working directly with paths and homotopies as first-class types, exposes structure that is hidden in the classical point-set formulation.

---

## 5. Cubical Agda: Computation from Proofs via the Identity Type

**The problem:** In standard intensional MLTT (the version in this chapter), the Univalence Axiom has no computation rule. Proofs using univalence are not "runnable" in the sense that there is no canonical normal form for terms that use the univalence proof.

**The solution:** Cubical type theory, implemented in Cubical Agda (from the Agda team at Chalmers, including Thierry Coquand, Anders Mörtberg, and Andrea Vezzosi), gives the J rule and transport full computation rules by interpreting paths as functions from an interval type.

**The MLTT connection:** The four judgments of Chapter 9 are still the foundation of Cubical Agda. The difference is that the identity type is *defined* as a path type (functions from the interval [0,1]) rather than as an inductive type with J as an axiom. Transport then computes by evaluating a path at an endpoint. J is derived (not axiomatized) and has a full computation rule.

**Practical impact:** In Cubical Agda, the computation

```agda
transport (funExt (λ n → refl)) v ≡ v
```

holds *definitionally* (not just propositionally). Programs extracted from proofs that use function extensionality actually run and compute the expected output. This makes Cubical Agda practically usable for verified computing in a way that standard MLTT-with-univalence-as-axiom is not.

---

## 6. The Proof of the Milnor Conjecture: MLTT as Research Tool

**The conjecture (proved):** Voevodsky proved the Milnor conjecture in 1996 and the more general Bloch-Kato conjecture (Voevodsky's theorem) in 2009. These are deep results in algebraic K-theory relating Milnor K-theory and Galois cohomology.

**The MLTT connection:** Voevodsky's engagement with MLTT came, in part, from his experience verifying his own proofs of these theorems. The proofs are long, complex, and had errors — errors not caught for years after publication. He found that the informal mathematical proof community lacked the tools to verify complex arguments reliably.

MLTT, formalized in Coq or Agda, is the proposed solution. The identity type machinery — the careful distinction between definitional and propositional equality, the requirement that every equality proof be explicitly constructed, the machine verification of type-checking — prevents the class of errors that plagued informal mathematics.

**The ongoing impact:** The formalization of Voevodsky's own mathematical work (the motivic cohomology library in Coq, the UniMath library) is the direct consequence. More broadly, the realization that MLTT's strict account of equality is *necessary* for reliable formalization — that the casual use of equality in informal mathematics hides real complexity — has influenced how proof assistants are designed and how mathematicians think about proof reliability.

MLTT is not just a foundation. It is a research tool that makes certain classes of mathematical errors impossible — by requiring that every equality be witnessed by an explicit path.
