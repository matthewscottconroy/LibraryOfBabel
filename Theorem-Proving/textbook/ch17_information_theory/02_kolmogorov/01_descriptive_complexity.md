# Kolmogorov Complexity

Shannon entropy measures the information in a *distribution*. Kolmogorov complexity measures the information in an *individual string* — how compressible it is, how "random" it is, independently of any probability model.

## Definition

The *Kolmogorov complexity* of a string x (with respect to a universal Turing machine U) is:

```
K(x) = min { |p| : U(p) = x }
```

The length of the shortest program that outputs x on the universal machine U.

**Intuitively**: K(x) is the "true description length" of x — the minimum information needed to specify x.

## Basic Properties

- **K(x) ≤ |x| + O(1)**: Every string has a trivial program "print x" of length ~|x|.
- **K is not computable**: No algorithm can compute K(x) for all x (by reduction to the halting problem).
- **Invariance theorem**: K depends on the choice of U only up to a constant. The choice of programming language doesn't matter asymptotically.
- **Most strings are incompressible**: For each n, at most 2^(n-c) strings of length n have K(x) ≤ n-c. So "most" strings are incompressible — random.

## Algorithmic Randomness

A string x is *Kolmogorov random* (or *incompressible*) if K(x) ≥ |x| - c for some constant c.

An infinite sequence ω is *Martin-Löf random* if no effective statistical test distinguishes it from a truly random sequence. Equivalently (Schnorr's theorem): the prefix-free Kolmogorov complexity of its length-n prefixes grows at rate ~n.

This gives a precise mathematical definition of "randomness" for individual sequences — something classical probability theory cannot do (probability speaks of distributions, not individual outcomes).

## Logical Depth (Bennett)

Charles Bennett (1988) observed that neither high complexity (random strings) nor low complexity (simple strings) captures "meaningful" content. A random string has maximal K but no structure. A constant string has minimal K but no meaning.

*Logical depth* of x = the running time of the shortest program that produces x.

Shallow strings: simple to compute (low depth) or random (any short program produces something different).
Deep strings: have short programs, but those programs take long to run — they encode "crystallized computation."

A deep string is the output of a long computation that has been "compressed" into a short but slow program. Mathematical theorems (especially those with short statements but difficult proofs) are logically deep: the axiom system is short, the proof is long.

## Connections to Logic

**Gödel incompleteness via Chaitin**: For any formal system T, there is a constant L such that T cannot prove "K(x) > L" for any specific x. (Though K(x) > L is true for all sufficiently long x.) The unprovability of "K(x) > L" for large L is a form of Gödel incompleteness — a property that's true but unprovable.

**Berry's paradox**: "The smallest positive integer not definable in fewer than thirteen words." This definition is itself fewer than thirteen words — contradiction. This paradox is the semantic counterpart of Gödel's incompleteness, and Kolmogorov complexity formalizes and resolves it: "definability" becomes "Kolmogorov complexity below a threshold."

**Levin's universal search**: Dovetailing all programs in order of length and running time provides an optimal (up to polynomial) algorithm for any NP problem with a small Kolmogorov-random solution. This connects Kolmogorov complexity to computational complexity theory.
