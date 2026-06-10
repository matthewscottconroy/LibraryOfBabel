# Countable Infinity and the Diagonal Argument

> "I see it, but I don't believe it."
> — Georg Cantor, in a letter to Dedekind, 1877 (on proving |[0,1]| = |[0,1]²|)

## The Question Nobody Had Thought to Ask

For most of human intellectual history, "infinity" was treated as a single, undifferentiated concept — the endless, the unbounded, the absolute. Aristotle distinguished *potential* infinity (the sequence of natural numbers, never completed) from *actual* infinity (a completed infinite whole), and for centuries mathematicians avoided the latter as philosophically dangerous.

Georg Cantor changed everything in the 1870s by asking a question that seems almost childishly simple: **are some infinities bigger than others?**

The answer is yes. Dramatically, provably, irrefutably yes. And the proof technique — **diagonalization** — became one of the most powerful tools in all of mathematics.

## Comparing Sizes: Bijections

How do we compare the "size" of infinite sets when we cannot count them in the ordinary sense?

Cantor's key insight was to use **bijections** — functions that are both injective (one-to-one) and surjective (onto). Two sets have the same size — the same **cardinality** — if and only if there exists a bijection between them.

This definition might seem strange but it agrees perfectly with our intuitions on finite sets: $\{a, b, c\}$ has the same size as $\{1, 2, 3\}$ because we can match them up one-to-one: $a \leftrightarrow 1$, $b \leftrightarrow 2$, $c \leftrightarrow 3$.

For infinite sets, this leads to counterintuitive conclusions:

**The natural numbers and the even natural numbers have the same cardinality.**

The bijection is $f(n) = 2n$: every natural number $n$ maps to the even number $2n$, and every even number $2k$ comes from $k$. So $|\mathbb{N}| = |\{0, 2, 4, 6, \ldots\}|$, even though the evens seem to be "half" of the naturals.

Galileo had noticed this in 1638 and concluded — correctly — that infinite sets behave differently from finite ones. He drew back from the conclusion, calling it a "paradox" best avoided. Cantor embraced it.

## Countably Infinite Sets

A set is **countably infinite** (or **countable**, or **denumerable**) if it has the same cardinality as $\mathbb{N}$ — that is, if its elements can be listed in a sequence $a_0, a_1, a_2, \ldots$ that hits every element exactly once.

Countable sets include:
- $\mathbb{N}$ itself: $0, 1, 2, 3, \ldots$
- $\mathbb{Z}$: $0, 1, -1, 2, -2, 3, -3, \ldots$ (interleaving positive and negative)
- $\mathbb{N} \times \mathbb{N}$: Use Cantor's pairing — enumerate by diagonals:

$$\begin{array}{ccccc}
(0,0) & (0,1) & (0,2) & (0,3) & \cdots \\
(1,0) & (1,1) & (1,2) & \cdots & \\
(2,0) & (2,1) & \cdots & & \\
(3,0) & \cdots & & &
\end{array}$$

Reading the diagonals: $(0,0), (1,0), (0,1), (2,0), (1,1), (0,2), (3,0), \ldots$

- $\mathbb{Q}$ (the rationals!): This is perhaps the most surprising. Between any two rationals there are infinitely many more rationals — they are "dense" in the reals. Yet we can list all of them. Write all fractions $p/q$ with $q > 0$ in a grid by numerator and denominator, then enumerate by diagonals (skipping duplicates). Every rational appears exactly once.

The technique of counting $\mathbb{N} \times \mathbb{N}$ by diagonals is itself called a **diagonalization** — but note this is different from Cantor's *other* diagonal argument below, which goes in the opposite direction.

## The Uncountability of the Reals

Now the dramatic result. Cantor (1874, refined 1891) proved:

**Theorem**: $\mathbb{R}$ — and even the interval $(0, 1)$ — is *not* countably infinite.

**Proof** (Cantor's diagonal argument, 1891 version):

Suppose, for contradiction, that $(0, 1)$ *is* countable. Then its elements can be listed:
$$r_0 = 0.d_{00}d_{01}d_{02}d_{03}\ldots$$
$$r_1 = 0.d_{10}d_{11}d_{12}d_{13}\ldots$$
$$r_2 = 0.d_{20}d_{21}d_{22}d_{23}\ldots$$
$$r_3 = 0.d_{30}d_{31}d_{32}d_{33}\ldots$$
$$\vdots$$

where each $r_n \in (0,1)$ is given by its decimal expansion and $d_{ij}$ is the $j$-th digit of $r_i$.

Now construct a new number $d = 0.d_0d_1d_2d_3\ldots$ where:
$$d_n = \begin{cases} 1 & \text{if } d_{nn} \neq 1 \\ 2 & \text{if } d_{nn} = 1 \end{cases}$$

(We avoid 0 and 9 to sidestep the $0.\overline{9} = 1.0$ issue with decimal representations.)

**Claim**: $d$ is not in the list $r_0, r_1, r_2, \ldots$

Why? For each $n$:
- $d$ differs from $r_n$ in the $n$-th decimal place (by construction: $d_n \neq d_{nn}$)
- Therefore $d \neq r_n$

So $d$ is a real number in $(0, 1)$ that is not in our supposed complete list. But we assumed the list was complete — contradiction. Therefore no such list exists, and $(0, 1)$ is **uncountable**. $\square$

## The Diagonal Construction: A Meta-Pattern

What makes this argument so powerful is that it is completely general. The *diagonalization technique* appears everywhere in mathematics and theoretical computer science:

1. **Cantor's theorem**: $|\mathcal{P}(A)| > |A|$ for any set $A$ (the power set is strictly larger)
2. **Gödel's incompleteness theorem**: Any sufficiently powerful consistent formal system has true statements it cannot prove
3. **Halting problem**: No Turing machine can decide whether arbitrary programs halt
4. **Rice's theorem**: No algorithm can decide any non-trivial property of program behavior
5. **Russell's paradox**: No set contains all sets (a diagonal argument in disguise)

The common thread: whenever you try to list all objects of a certain type, you can always construct a new object that *differs* from every listed object in a systematic way. The diagonal construction ensures the new object escapes the list.

## Cardinality: ℵ₀ and Beyond

Cantor introduced **cardinals** to measure set sizes:
- $\aleph_0$ (aleph-naught): the cardinality of $\mathbb{N}$ — the "smallest" infinity
- $|\mathbb{R}| = 2^{\aleph_0}$: the cardinality of the reals (also written $\mathfrak{c}$, the "continuum")
- $|\mathcal{P}(\mathbb{N})| = 2^{\aleph_0} = |\mathbb{R}|$: the power set of $\mathbb{N}$ is equinumerous with $\mathbb{R}$

Cantor proved $2^{\aleph_0} > \aleph_0$ — the reals are *strictly* larger than the naturals. He asked: is there any cardinality strictly between $\aleph_0$ and $2^{\aleph_0}$? This is the **Continuum Hypothesis (CH)**:

$$\text{CH}: \text{There is no set } A \text{ with } \aleph_0 < |A| < 2^{\aleph_0}$$

In one of the most extraordinary results in 20th-century mathematics, Gödel (1938) showed CH cannot be *disproved* from ZFC, and Cohen (1963) showed it cannot be *proved* either. CH is **independent** of ZFC — neither CH nor ¬CH leads to contradiction. This means you can do mathematics in a universe where CH is true or in one where it is false, and neither choice creates inconsistency.

## Real-World Significance

The countable/uncountable divide has concrete implications:

**Computability**: Every computer program is a finite string of characters from a finite alphabet. So the set of all programs is countable ($|\text{Programs}| = \aleph_0$). But the set of all functions $f : \mathbb{N} \to \mathbb{N}$ has cardinality $2^{\aleph_0}$ — uncountably many. By a simple counting argument, most functions are **not computable** — no program computes them. And since "decidable languages" correspond to computable functions, most languages are undecidable. (See ch10.)

**Information theory**: Shannon's measure of information is defined over probability distributions on countable or uncountable sample spaces. The distinction matters for whether certain information-theoretic quantities are finite or infinite.

**Analysis**: The completeness of $\mathbb{R}$ — the property that every Cauchy sequence converges — is what distinguishes the reals from the rationals. It is deeply tied to the uncountability: $\mathbb{Q}$, being countable, cannot be complete.

## Lean 4 Sketch

```lean
-- Cantor's theorem: no surjection from A to Set A
theorem cantor (f : α → Set α) : ¬Surjective f := by
  intro h
  -- Define the diagonal set: d = {x | x ∉ f x}
  let d : Set α := {x | x ∉ f x}
  -- Since f is surjective, d = f a for some a
  obtain ⟨a, ha⟩ := h d
  -- Is a ∈ f a?
  by_cases h : a ∈ f a
  · -- If a ∈ f a, then a ∈ d means a ∉ f a — contradiction
    have : a ∈ d := ha ▸ h
    exact this h
  · -- If a ∉ f a, then a ∈ d, so a ∈ f a — contradiction
    have : a ∈ d := h
    exact h (ha ▸ this)
```

## Stop and Think

*Is the following argument valid?* "The natural numbers and the rationals both have cardinality $\aleph_0$. So the rationals 'are' the natural numbers."

No — equicardinality means there *exists* a bijection, not that the sets are identical. $\mathbb{N}$ and $\mathbb{Q}$ are very different objects mathematically (different operations, different order-theoretic properties, different arithmetic). Cardinality captures only *one* aspect of a set's structure — its "size" — not its algebraic or topological properties.

## Exercises
See [problems/ch06_set_theory/04_cardinality_challenges.md](../../../problems/ch06_set_theory/04_cardinality_challenges.md)
