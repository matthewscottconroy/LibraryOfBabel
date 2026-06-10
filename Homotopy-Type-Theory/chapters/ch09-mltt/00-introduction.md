# Chapter 9: Martin-Löf Type Theory

## The Central Engine

If you had to point to one formal system that makes HoTT possible, it would be Martin-Löf Type Theory (MLTT). Everything in the preceding chapters has been building toward it: the lambda calculus gave us the computational machinery, System F gave us polymorphism, dependent types gave us the ability to express mathematical properties, and Curry-Howard told us that proofs and programs are the same thing.

MLTT synthesizes all of this into a single coherent formal system, developed by Per Martin-Löf through a series of lectures and papers in the 1970s and 1980s. The theory has gone through several versions, but the version we'll study — intensional MLTT with a universe hierarchy — is the one that underlies Agda, that Lean 4 descends from, and that HoTT extends.

## What's Genuinely New

We've already seen Π types, Σ types, inductive types, and universes. What does MLTT add that we haven't seen?

The answer is the *identity type*, and it's the most important thing in this chapter.

The identity type $a =_A b$ is the type whose elements are *proofs that $a$ and $b$ are equal*. Its introduction rule (reflexivity) and elimination rule (the J-rule) are the formal system's primitive notion of equality.

This seems like a small technicality. It is not. Two crucial facts about the identity type are:

1. **The identity type can have multiple distinct elements.** Two different proofs of $a = b$ are, in general, different terms. This is UIP (Uniqueness of Identity Proofs) failing, and it's a feature, not a bug.

2. **Identity proofs form a groupoid.** The operations of reflexivity, symmetry (path inversion), and transitivity (path concatenation), derived from J, make every type into a groupoid with identity proofs as morphisms.

These two facts together are the seed of HoTT. If identity proofs can be non-trivial and multiple, then they can have their own identity proofs (homotopies between paths), which can have their own identity proofs, and so on — an infinite tower of higher-dimensional structure. Types become *homotopy types*, and HoTT is the study of this structure from the inside.

## MLTT vs. Earlier Type Theories

| System | Expressiveness | Identity |
|---|---|---|
| STLC | Propositional logic | No identity type |
| System F | Second-order logic | No identity type |
| System Fω | Higher-order logic | No identity type |
| Dependent types (Ch.8) | First-order logic + | Undefined in general |
| **MLTT** | **Full mathematics** | **Identity type with J** |

The transition from "dependent types" to MLTT is precisely the addition of the identity type as a first-class citizen with its own formation, introduction, elimination, and computation rules.

## The Role of Definitional vs. Propositional Equality

One of the most important (and initially confusing) aspects of MLTT is that it has *two* notions of equality:

- **Definitional equality** ($a \equiv b$, also called *judgmental equality*): holds when two terms reduce to the same normal form. This is checked by the type checker mechanically.

- **Propositional equality** ($a =_A b$): the *type* of proofs that $a$ equals $b$. This is something you prove inside the system.

Every definitional equality gives a propositional equality (by reflexivity), but not vice versa. In MLTT, $2 + 3$ and $5$ are definitionally equal (they both reduce to $\mathsf{succ}(\mathsf{succ}(\mathsf{succ}(\mathsf{succ}(\mathsf{succ}(\mathsf{zero})))))$). The commutativity $m + n = n + m$ is only propositionally equal (it requires a proof by induction).

This distinction is fundamental to the computational character of MLTT. Definitional equality is automatic (the type checker handles it); propositional equality requires explicit proof (a program of the appropriate type).

## Chapter Roadmap

**Section 1: The Four Judgments.** MLTT has precisely four primitive forms of assertion. We'll see what they are, what contexts look like, and how judgments build on each other.

**Section 2: Type Formers.** The FIEC framework (Formation, Introduction, Elimination, Computation) for each type former: Π, Σ, ℕ, and the universe. The rules are collected and presented systematically.

**Section 3: The Identity Type.** The central object of MLTT. Formation, introduction (reflexivity), elimination (J), computation, and the groupoid laws.

**Section 4: Transport and ap.** The two most important operations derived from J: transport (moving elements along identity proofs) and ap (applying functions to paths). These are the workhorses of MLTT.

**Section 5: Homotopies and Function Extensionality.** What it means for two functions to be homotopic (pointwise equal), and why function extensionality (homotopic functions are equal) is not provable in basic MLTT but follows from univalence.

**Section 6: Intensional vs. Extensional MLTT.** The two major variants of MLTT: one where identity proofs can be non-trivial (intensional, the foundation of HoTT) and one where identity implies definitional equality (extensional, which makes type checking undecidable).

**Section 7: Exercises.**

## Why This Chapter Matters for HoTT

The entire HoTT program is built on one observation: if you take MLTT (intensional version) seriously, and you interpret types as spaces and identity proofs as paths, then you get a consistent and useful theory of higher-dimensional structures.

The Univalence Axiom (Chapter 11) is an axiom *about* the identity type of the universe. Higher Inductive Types (Chapter 14) extend the identity type to allow non-trivial paths. Synthetic homotopy theory (Chapter 15+) proves theorems about spaces using only the type-theoretic language.

All of this starts here, with the four judgments and the J rule.
