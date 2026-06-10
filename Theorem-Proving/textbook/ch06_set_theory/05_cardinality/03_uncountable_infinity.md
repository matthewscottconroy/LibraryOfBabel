# Uncountable Infinity

Cantor's most shocking discovery: not all infinities are equal. There is an infinity so vast that no list can exhaust it — the real numbers cannot be counted.

## Countability

A set A is *countable* (or *countably infinite*) if there is a bijection ℕ → A — we can list its elements: a₀, a₁, a₂, ...

Countable sets include:
- ℕ itself (trivially)
- ℤ (integers): list as 0, 1, -1, 2, -2, 3, -3, ...
- ℚ (rationals): Cantor's diagonal enumeration of the grid ℕ × ℕ
- Any finite union of countable sets
- The set of all finite strings over a finite alphabet (so all computer programs are countable)

## Cantor's Diagonal Argument

**Theorem (Cantor, 1874, 1891)**: The real numbers ℝ are uncountable. No bijection ℕ → ℝ exists.

**Proof (diagonal argument)**:

Suppose for contradiction that f : ℕ → (0,1) is a surjection (a list of all reals in (0,1)):

```
f(0) = 0.a₀₀ a₀₁ a₀₂ a₀₃ ...
f(1) = 0.a₁₀ a₁₁ a₁₂ a₁₃ ...
f(2) = 0.a₂₀ a₂₁ a₂₂ a₂₃ ...
...
```

Construct the *diagonal real* d by: d_n = (a_nn + 1) mod 9 — take the nth decimal digit of f(n) and change it (avoid 0 and 9 to sidestep 0.999... = 1.000... issues).

Then d differs from f(n) in the nth decimal place, so d ≠ f(n) for all n. But d ∈ (0,1), so d should appear in the list — contradiction. ∎

The diagonal construction is a *witness*: not an abstract non-existence proof, but an explicit real number missing from the list.

## Cardinalities of Infinity

Cantor introduced cardinal numbers to measure infinite sizes:

| Set | Cardinality | Symbol |
|-----|-------------|--------|
| ℕ, ℤ, ℚ | Countably infinite | ℵ₀ ("aleph-null") |
| ℝ, (0,1), 𝒫(ℕ) | Uncountably infinite | 𝔠 ("continuum") |
| 𝒫(ℝ), ℝ^ℝ | Larger still | 2^𝔠 |

We have ℵ₀ < 𝔠 (by Cantor's theorem: |A| < |𝒫(A)|). And |ℝ| = |𝒫(ℕ)| = 2^ℵ₀ = 𝔠.

**The Continuum Hypothesis (CH)**: Is there a cardinality strictly between ℵ₀ and 𝔠? Cantor believed not — that 𝔠 = ℵ₁ (the next cardinal after ℵ₀). This is CH.

Gödel (1940) showed CH is *consistent* with ZFC (it can be true). Cohen (1963) showed ¬CH is *consistent* with ZFC (it can also be false). Together: CH is *independent* of ZFC — neither provable nor refutable. This was one of the most important results of 20th century mathematics, and Cohen's method (forcing) became a major technique in set theory.

## The Uncountability of ℝ and Analysis

Uncountability is not merely a curiosity — it has mathematical consequences:

**Measure theory**: The countable additivity of Lebesgue measure means countable sets have measure zero. But ℝ has measure ∞. Uncountability is what makes "most" real numbers pathological (non-computable, transcendental, etc.).

**Computability**: Only countably many programs exist (programs are finite strings). Only countably many functions are computable. But there are uncountably many functions ℕ → ℕ. So *most* mathematical functions are not computable.

**Baire category**: The real numbers form a "meager" (first-category) space in certain senses — every open interval is non-meager, but countable sets are negligible. The Baire category theorem implies results like: the set of differentiable functions is "small" within the continuous functions.

## Cantor's Legacy

Cantor's contemporaries resisted violently. Kronecker called him a "corrupter of youth." Poincaré dismissed set theory as a pathology. Wittgenstein later called Cantor's diagonal argument a "pernicious trick." Yet today, uncountability is as accepted as negative numbers — a foundational fact taught in analysis, logic, and computability theory.

Hilbert, who understood the stakes, wrote: "No one shall expel us from the paradise that Cantor has created."
