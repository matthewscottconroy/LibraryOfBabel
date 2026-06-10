# Chapter 12: Higher Category Theory and the Homotopy Hypothesis

## Why Go Higher?

Ordinary category theory gives us a language for talking about mathematical objects and the maps between them. A category has objects and morphisms, and the morphisms compose. That's the story.

But sometimes you want to talk about *maps between maps*. In the category of categories, a functor $F : \mathcal{C} \to \mathcal{D}$ is a morphism, and a natural transformation $\alpha : F \Rightarrow G$ is a morphism between morphisms. You can go further: modifications between natural transformations, perturbations between modifications, and so on. This is the world of **higher category theory**.

For most of mathematics, ordinary categories are sufficient. But for homotopy theory — and hence for HoTT — you genuinely need the higher structure. And here's the key theorem that makes higher categories unavoidable for us:

**The Homotopy Hypothesis (Grothendieck, 1983):** *Homotopy types are the same as ∞-groupoids.*

This is not an analogy. It's an equivalence of mathematical structures. A homotopy type is a topological space considered up to weak equivalence — the kind of space you study in algebraic topology. An ∞-groupoid is a higher categorical structure where all morphisms at all levels are invertible. These turn out to be the same thing.

And now here's why this matters for HoTT: types in Martin-Löf type theory *are* ∞-groupoids, by virtue of their identity type structure. The type $A$ has terms; between any two terms, there are identity proofs (paths); between any two identity proofs, there are higher identity proofs (paths of paths); and this tower never stops. That's exactly the data of an ∞-groupoid.

So HoTT isn't just *inspired* by homotopy theory. It *is* homotopy theory, formalized as a type theory.

## The Hierarchy of Higher Categories

Before we dive in, let me sketch the big picture. We're going to climb a hierarchy:

**Ordinary categories (1-categories):** Objects, morphisms. Associative composition, units. The world of Chapter 10.

**2-Categories:** Objects, 1-morphisms, 2-morphisms. Two kinds of composition. Examples: Cat (categories, functors, natural transformations), Grpd (groupoids, functors, natural transformations).

**Strict n-categories:** Objects, 1-morphisms, ..., n-morphisms. All composition laws hold on the nose (strictly).

**Weak n-categories (bicategories for n=2):** Objects, 1-morphisms, ..., n-morphisms. Composition laws hold up to coherent isomorphism.

**∞-Categories:** Objects, morphisms at every level. The composition laws hold in the most general sense.

**∞-Groupoids:** An ∞-category where every morphism at every level is invertible. These model homotopy types.

**(∞,1)-Categories:** An ∞-category where all k-morphisms for $k \geq 2$ are invertible, but 1-morphisms may not be. These generalize ordinary categories to the homotopy-coherent setting.

The most important insight is that *strict* higher categories are much simpler to define but miss most examples that arise in nature. *Weak* higher categories are much harder to define correctly (the coherence conditions are complex) but capture the real mathematical phenomena.

## The Connection to HoTT

Here's the core diagram that this chapter is building toward:

```
Homotopy types ←→ ∞-groupoids ←→ Types in HoTT
```

- Left arrow: The homotopy hypothesis (Grothendieck's conjecture, proved in various senses)
- Right arrow: Types in MLTT/HoTT are ∞-groupoids via their iterated identity types

And zooming out:

```
HoTT = internal language of ∞-toposes
```

Just as intuitionistic higher-order logic is the internal language of elementary toposes (Chapter 11), HoTT is the internal language of ∞-toposes. This is the deepest connection in the whole curriculum.

## Chapter Roadmap

**Section 1 (2-Categories):** Start with the simplest non-trivial case. What does it mean to have morphisms between morphisms? How do you compose them? What are the coherence conditions?

**Section 2 (Groupoids):** Focus on groupoids — categories where every morphism is invertible. These are the 1-categorical version of homotopy types. The fundamental groupoid of a space captures its path structure. Types in MLTT are groupoids.

**Section 3 (The Homotopy Hypothesis):** State Grothendieck's conjecture precisely. Explain the simplicial set formulation (Kan complexes model homotopy types). Connect to MLTT: why types are ∞-groupoids.

**Section 4 ((∞,1)-Categories):** The most important higher categorical structure for HoTT. These generalize ordinary categories by allowing morphisms to form spaces rather than sets. The main model: quasi-categories (Joyal-Lurie).

**Section 5 (∞-Groupoids):** Multiple definitions and their equivalences. Why Kan complexes are the right model. Connection to the homotopy hypothesis.

## Prerequisites and Connections

This chapter builds on:
- Category theory (Chapter 10): the basics of categories, functors, natural transformations
- MLTT (Chapter 9): identity types and their groupoid structure
- Simplicial sets (Chapter 15): we preview the simplicial approach here and develop it fully there

This chapter connects forward to:
- Higher inductive types (Chapter 19): HITs construct specific homotopy types inside HoTT
- Synthetic homotopy theory (Chapter 20): computing with higher groupoid structure in HoTT
- Simplicial type theory (Chapter 24): formalizing (∞,1)-categories in HoTT

## A Note on Strictness vs. Weakness

One theme you'll encounter throughout this chapter: *strict is easy to define but wrong; weak is hard to define but right*.

For instance, you could define a "strict ∞-groupoid" as a globular set with strictly associative, strictly unital composition operations. This is easy to write down. But it turns out that strict ∞-groupoids only model a very restricted class of homotopy types — not all of them.

For full generality, you need weak ∞-groupoids, where composition is only associative and unital up to higher coherent cells. The combinatorics of "what does coherence mean at all levels" is genuinely complex, and there are multiple competing definitions.

The simplest approach (and the one that actually works) is to bypass the combinatorics entirely by using simplicial sets: a Kan complex is an ∞-groupoid by definition, and the horn-filling conditions encode all the coherence data implicitly. This is the approach we'll take.

The upshot: weakness and higher structure are not optional complications. They're forced on you by the mathematics itself. And they're exactly what makes HoTT interesting.
