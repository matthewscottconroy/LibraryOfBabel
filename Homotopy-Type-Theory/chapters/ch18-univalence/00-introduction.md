# Chapter 18: Equivalences and the Univalence Axiom

## The Revolutionary Idea

There's a principle that mathematicians use constantly without thinking much about it: *isomorphic objects are interchangeable*. If two groups are isomorphic, any theorem about one holds for the other. If two topological spaces are homeomorphic, they have the same topological properties. Mathematicians routinely say "let $G$ be the cyclic group of order $p$" without specifying whether they mean $\mathbb{Z}/p\mathbb{Z}$ or some other isomorphic presentation.

This is so natural that it seems obvious. But in formal foundations, it's subtle. In ZFC set theory, the cyclic group $\mathbb{Z}/p\mathbb{Z}$ and another isomorphic group might be literally different sets (with different elements), even though they're isomorphic. Using them interchangeably is justified informally but not by the formal rules.

The Univalence Axiom, introduced by Vladimir Voevodsky, resolves this tension definitively:

**Equivalent types are literally equal.**

If $A \simeq B$ (the types are equivalent — there's a bijection respecting all the type-theoretic structure), then $A = B$ (they are equal as types). Not "morally equal" or "interchangeable for all practical purposes" — actually equal in the formal type theory.

This single axiom:
- Makes the informal mathematical practice of treating isomorphic objects as identical formally correct
- Implies function extensionality (homotopic functions are equal)
- Implies propositional extensionality (logically equivalent propositions are equal)
- Connects paths in the universe to equivalences between types
- Enables the program of *univalent foundations* — mathematics done in HoTT

## The Setup: What Is Equivalence?

Before stating Univalence, we need to know what "equivalent" means. The naive answer — "there's a bijection" — is not quite right in HoTT. A bijection in the naive sense (a function with a two-sided inverse) is called a *quasi-equivalence*, and it turns out that being a quasi-equivalence is not a *proposition* — there can be multiple distinct quasi-inverses.

For Univalence to work correctly (and for the type of equivalences to be "the right size"), we need a notion of equivalence that:
1. Captures the right mathematical content (the functions have inverses in the right sense)
2. Is a *proposition* — there's at most one way to be an equivalence

We'll see three equivalent definitions:
- **Bi-invertible maps**: having both a left and right inverse separately
- **Half-adjoint equivalences**: having one inverse plus a coherence condition
- **Contractible fibers**: every element of the codomain has a unique preimage

All three are logically equivalent and are propositions (unlike quasi-equivalences).

## Chapter Roadmap

**Section 1: Equivalences** — The three definitions of equivalence, their equivalence, and basic properties. The type of equivalences $A \simeq B$.

**Section 2: The Univalence Axiom** — Statement, consequences, and why it's not provable without an axiom. The computation rule. The inverse function $\mathsf{ua}$.

**Section 3: Consequences** — Function extensionality, propositional extensionality, structure invariance (the "Univalence Principle"). Why Univalence makes mathematics "work right."

**Section 4: Examples** — Paths between specific types. The two paths on $\mathsf{Bool}$. Groups and their automorphisms. The non-trivial path in the universe.

## Prerequisites and Connections

Builds on:
- Chapter 16 (Identity Types): Paths, transport, ap — the machinery Univalence extends to the universe
- Chapter 17 (H-Levels): Propositions and contractibility — needed to understand why equivalences must be propositions

Connects forward to:
- Chapter 19 (Higher Inductive Types): HITs use paths in the universe (via univalence) to define new types
- Chapter 20 (Synthetic Homotopy): Computing homotopy groups uses Univalence for the Hopf fibration
- Chapter 21–22 (Lean 4, Cubical Agda): Both implement Univalence, but in different ways

## A Note on Axioms vs. Theorems

Univalence is an *axiom* in the standard formulation of HoTT. It's not derivable from the basic rules of dependent type theory. Its consistency is established by the simplicial set model (Voevodsky's theorem), which provides a mathematical model where Univalence holds.

In *cubical type theory* (Chapter 23), Univalence is not an axiom but a *theorem* — it follows from the computational rules of the interval and Kan operations. This is a major advantage of cubical type theory: it turns a mysterious axiom into something computable.

For now, we add Univalence as an axiom, knowing it's consistent, and develop its consequences.
