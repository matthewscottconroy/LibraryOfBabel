# Non-Standard Models

> "The existence of non-standard models is a permanent reminder that our formal systems do not fully characterize the mathematical structures we think we are describing."
> — Model theorist's observation

## The Gap Between Syntax and Semantics

When we write down the Peano axioms for arithmetic, we have something specific in mind: the natural numbers $\{0, 1, 2, 3, \ldots\}$ with the usual addition and multiplication. The axioms are *intended* to describe this unique structure.

But they don't — not uniquely. The Peano axioms (in first-order logic) have models radically different from the intended one. These are **non-standard models**, and they reveal a fundamental limitation of first-order logic: it cannot pin down infinite structures up to isomorphism.

## What is a Non-Standard Model?

A non-standard model of arithmetic is a structure $\mathcal{M}$ satisfying all the first-order consequences of Peano Arithmetic (PA), but not isomorphic to the standard natural numbers $\mathbb{N}$.

Every non-standard model contains:
1. **A standard part**: elements corresponding to $0, 1, 2, 3, \ldots$, satisfying all the standard facts ($1 + 1 = 2$, etc.)
2. **Non-standard elements**: "infinite" elements larger than any standard natural number

The non-standard elements form what looks (from inside) like a dense linear order with no endpoints — they come in $\mathbb{Z}$-blocks (a copy of the integers) arranged in the order type of $\mathbb{Q}$.

## Existence via Compactness

As shown in the previous section, the Compactness theorem guarantees non-standard models exist. The key construction:

Add a constant $c$ to the language and add axioms $c \neq \bar{n}$ for every standard numeral $\bar{n}$. Every finite subset is satisfiable (by a large enough standard number), so by Compactness, the whole theory is satisfiable — and any model contains the non-standard $c$.

## What PA Can and Cannot See

Here is the deep point: **non-standard models satisfy every first-order theorem of PA**. This means:
- In a non-standard model, every even number is the sum of two primes if Goldbach's conjecture is true in $\mathbb{N}$ (because Goldbach is a first-order sentence)
- Fermat's Last Theorem holds in non-standard models (it's a first-order statement)
- The fundamental theorem of arithmetic holds in non-standard models

Yet non-standard models differ from $\mathbb{N}$ in ways visible from *outside* the model:
- They are not well-ordered from an external perspective (there are infinite descending sequences, just none that are sets in the model)
- Induction in non-standard models only holds for *first-order definable* properties — not for arbitrary external properties
- The non-standard elements cannot be "named" by any standard term of PA

**True arithmetic vs. provable arithmetic**: Gödel's incompleteness theorem tells us there are sentences $G$ true in $\mathbb{N}$ but unprovable in PA. Since $G$ is not provable, $\neg G$ is consistent with PA, so there are models of PA where $G$ is *false* — these are necessarily non-standard models.

## Non-Standard Analysis

The most famous application of non-standard models is Abraham Robinson's **non-standard analysis** (1961), which provides rigorous foundations for infinitesimals.

Consider the hyperreal number system ${}^*\mathbb{R}$, a non-standard model of the first-order theory of the real numbers. It contains:
- All standard reals
- **Infinitesimals** $\varepsilon$ with $0 < |\varepsilon| < r$ for every positive real $r$
- **Infinite hyperreals** $H$ with $|H| > r$ for every real $r$

The **standard part function** $\text{st}: {}^*\mathbb{R}_{\text{finite}} \to \mathbb{R}$ maps every finite hyperreal to the unique real infinitely close to it.

**Calculus with infinitesimals**:
$$f'(a) = \text{st}\!\left(\frac{f(a + \varepsilon) - f(a)}{\varepsilon}\right)$$

for any nonzero infinitesimal $\varepsilon$. This is rigorous — not just "intuition."

**Example**: Let $f(x) = x^2$. Then:
$$\frac{f(a+\varepsilon) - f(a)}{\varepsilon} = \frac{(a+\varepsilon)^2 - a^2}{\varepsilon} = \frac{2a\varepsilon + \varepsilon^2}{\varepsilon} = 2a + \varepsilon$$

Since $\varepsilon$ is infinitesimal, $\text{st}(2a + \varepsilon) = 2a$. So $f'(a) = 2a$. This is the same answer as the $\epsilon$-$\delta$ approach, but the reasoning is closer to Newton's original "fluxion" intuition.

The **Transfer Principle** makes this rigorous: any first-order statement true in $\mathbb{R}$ is true in ${}^*\mathbb{R}$, and vice versa. So the non-standard reals satisfy all the same axioms as the reals — they are just a larger model.

## Skolem's Paradox

Here is a striking consequence of the Löwenheim-Skolem theorem:

ZFC set theory (which proves the existence of uncountable sets) has a **countable model** $\mathcal{M}$. But wait — ZFC proves $\mathbb{R}$ is uncountable. Isn't that a contradiction?

No. "Uncountable" in the model $\mathcal{M}$ means: there is no bijection **inside $\mathcal{M}$** between the real numbers (as interpreted in $\mathcal{M}$) and $\omega^{\mathcal{M}}$ (the integers as interpreted in $\mathcal{M}$). From *outside* $\mathcal{M}$, we can see that the domain of $\mathcal{M}$ is countable — but the bijection witnessing this is not *in* $\mathcal{M}$.

This is Skolem's paradox (1922): the same set can be "uncountable" relative to one model and "countable" relative to another. Countability is not an absolute property; it is *model-relative*. What matters is not the cardinality of the set itself, but what bijections exist within the ambient model.

This shows that model-theoretic notions like "finite," "countable," and "infinite" are sensitive to the ambient model — a startling realization that continues to generate philosophical debate about mathematical realism.

## Category-Theoretic Perspective

From a category-theoretic viewpoint, the existence of non-standard models is exactly the statement that the category of models of a first-order theory is not **rigid** — it has non-trivial automorphisms and multiple non-isomorphic objects.

**Categoricity** — a theory having a unique model up to isomorphism at some cardinality — is the exception rather than the rule in first-order logic. Morley's theorem (1965) shows that if a countable theory is categorical in one uncountable cardinality, it is categorical in all uncountable cardinalities. But no infinite theory can be categorical in $\aleph_0$ — countable models come in multiple non-isomorphic copies (or in a single rigid copy, but the Löwenheim-Skolem theorem gives more at uncountable cardinalities).

## Philosophical Implications

Non-standard models raise deep questions about mathematical realism:

**Structuralism**: If what we care about is the *structure* of the natural numbers, and first-order logic cannot characterize this structure uniquely, what *does* characterize it? Second-order logic, with its ability to quantify over sets, *can* characterize $\mathbb{N}$ up to isomorphism (Dedekind's theorem). But second-order logic has its own complications — it is not complete (no effective deductive system can derive all second-order consequences).

**Intended model**: When mathematicians work with "the natural numbers," do they mean the first-order structure (satisfying PA) or the second-order structure (the unique Dedekind-complete structure)? Most practicing mathematicians work informally in a way that implicitly uses the intended model — but formal systems cannot fully capture this intention at the first-order level.

**Indeterminacy**: Some philosophers (following Putnam) argue that the existence of non-standard models shows that mathematical reference is indeterminate — we cannot "pin down" which model we are talking about, even with unlimited formal resources. Others (following Shapiro and others) see this as a feature of the formal language, not of the mathematical objects themselves.

## Exercises
See [problems/ch09_model_theory/04_nonstandard_exercises.md](../../../problems/ch09_model_theory/04_nonstandard_exercises.md)
