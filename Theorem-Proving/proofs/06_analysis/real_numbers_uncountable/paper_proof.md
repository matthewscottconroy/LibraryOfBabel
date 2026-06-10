# Cantor's Diagonal Argument

## Theorem
ℝ is uncountable. Equivalently, (0,1) is uncountable.

## Proof
Suppose for contradiction that f : ℕ → (0,1) is a surjection (an enumeration of all reals in (0,1)).
Write each value as a decimal:
```
f(0) = 0.d₀₀ d₀₁ d₀₂ d₀₃ ...
f(1) = 0.d₁₀ d₁₁ d₁₂ d₁₃ ...
f(2) = 0.d₂₀ d₂₁ d₂₂ d₂₃ ...
...
```
Define a new number x = 0.x₀ x₁ x₂ ... where:
xₙ = 5 if dₙₙ ≠ 5, else xₙ = 6.

Then x ∈ (0,1) and x ≠ f(n) for all n (they differ in the n-th decimal place).
So x is not in the range of f — contradicting the assumption that f is surjective. □

## Important Notes
1. The diagonal argument uses the specific decimal digits to ensure distinctness.
   We choose 5 and 6 (rather than e.g. 1 and 2) to avoid the 0.999... = 1.000... issue.

2. The proof applies to any putative enumeration — it constructs a real number specifically
   tailored to differ from every listed number.

3. This was one of Cantor's most controversial results. Kronecker called Cantor a "corrupter
   of youth." Today it is universally accepted.

## Corollary: |ℝ| = |𝒫(ℕ)|
There is a bijection between ℝ and 𝒫(ℕ) (the power set of ℕ). Both are uncountable.
This cardinality is denoted 𝔠 (the continuum) or 2^ℵ₀.

## The Continuum Hypothesis
Is there a cardinality strictly between ℵ₀ (countable) and 𝔠 (continuum)?
This is the **Continuum Hypothesis** (CH): 𝔠 = ℵ₁.
It is independent of ZFC (Gödel 1940, Cohen 1963).
