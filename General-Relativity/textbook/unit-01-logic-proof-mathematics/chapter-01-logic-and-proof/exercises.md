# Chapter 1: Exercises, Thought Experiments, and Laboratory Projects

---

## Exercises

### Section 1.1: Propositional Logic

**1.1.1** Determine which of the following are propositions. For those that are, give their truth value. For those that are not, explain why.

(a) "The Riemann curvature tensor vanishes identically in flat spacetime."  
(b) "Is general relativity correct?"  
(c) "x² + y² = z² has no positive integer solutions."  
(d) "Every black hole has an event horizon."  
(e) "This sentence is false."  
(f) "Look at the elegance of Einstein's field equations."

**1.1.2** Construct the truth table for each compound proposition:

(a) (P ∧ Q) ∨ ¬R  
(b) (P → Q) ∧ (Q → R) → (P → R)  
(c) (P ↔ Q) ↔ ((P ∧ Q) ∨ (¬P ∧ ¬Q))  
(d) ¬(P → Q) ↔ (P ∧ ¬Q)

**1.1.3** Which of the propositions in Exercise 1.1.2 are tautologies? Justify.

**1.1.4** Using only the laws of propositional logic (no truth tables), prove the following equivalences:

(a) P → (Q ∧ R) ≡ (P → Q) ∧ (P → R)  
(b) ¬(P ∨ Q) ≡ ¬P ∧ ¬Q (De Morgan)  
(c) (P ∨ Q) → R ≡ (P → R) ∧ (Q → R)  
(d) P → Q ≡ ¬Q → ¬P (contrapositive)

**1.1.5** The following conditional statement from physics: "If the spacetime metric is Minkowski, then all components of the Riemann curvature tensor vanish." 

(a) Write the converse, inverse, and contrapositive of this statement.  
(b) Which of the four statements (original, converse, inverse, contrapositive) are true? Justify using your knowledge of GR (or look it up if needed).  
(c) What is the negation of the original statement?

**1.1.6** (Deeper) The following is an important theorem: "A differentiable function f: ℝ → ℝ satisfies f'(x) = 0 for all x if and only if f is constant." Write this as a biconditional using logical notation, then break it into two implications and think about how each would be proved.

---

### Section 1.2: Predicate Logic

**1.2.1** Let the domain be ℝ. For each formula, determine whether it is True or False, and explain:

(a) ∀x (x² ≥ 0)  
(b) ∃x (x² = -1)  
(c) ∀x ∃y (x + y = 0)  
(d) ∃y ∀x (x + y = 0)  
(e) ∀x ∀y (x < y → ∃z (x < z < y))  
(f) ∀ε > 0 ∃N ∈ ℕ ∀n ≥ N (1/n < ε)

**1.2.2** Write each of the following mathematical statements in formal predicate logic with explicit quantifiers and domain:

(a) "Every continuous function on a closed interval is bounded."  
(b) "There exists an irrational number between any two rational numbers."  
(c) "The metric tensor g_{μν} is non-degenerate at every point p of spacetime M."  
(d) "The Schwarzschild solution is the unique spherically symmetric vacuum solution."  
(e) "No function is simultaneously its own derivative and equal to zero."

**1.2.3** Negate each formula in Exercise 1.2.1. Write the negation in formal predicate logic and in plain English.

**1.2.4** Consider the predicate logic formalization of Einstein's Equivalence Principle:
$$\forall p \in M \; \forall \varepsilon > 0 \; \exists U_p \; \text{(neighborhood of } p\text{)} \; \forall \text{measurements in } U_p: |\text{result} - \text{SR prediction}| < \varepsilon$$

(a) Write the negation of this statement.  
(b) What physical situation would the negation describe?  
(c) How does the Penrose singularity theorem (which we prove in Chapter 59) relate to this negation?

**1.2.5** The statement "For every ε > 0, there exists δ > 0, such that for all x with 0 < |x - 2| < δ, we have |x² - 4| < ε" claims that lim_{x→2} x² = 4.

(a) Prove this limit using the ε-δ definition. (Find explicit δ in terms of ε.)  
(b) Write the negation of the limit statement.  
(c) Does the negation hold for any limit value L ≠ 4? Explain.

---

### Section 1.3: Methods of Proof

**1.3.1** (Direct proof) Prove each of the following directly:

(a) The sum of any two odd integers is even.  
(b) If n is divisible by 6, then n is divisible by both 2 and 3.  
(c) For all real x, y: (x + y)² ≥ 0. (This seems trivial — write a careful proof from the definition of "≥ 0" for real numbers.)  
(d) If a | b and b | c, then a | c (transitivity of divisibility).  
(e) The product of any three consecutive integers is divisible by 6.

**1.3.2** (Contrapositive) Prove each of the following by proving the contrapositive:

(a) If n² is even, then n is even.  
(b) If n² is divisible by 3, then n is divisible by 3.  
(c) If f: A → B is injective and g: B → C is injective, then g ∘ f is injective.  
(d) If a real number r is not rational, then r + 1 is not rational.

**1.3.3** (Contradiction) Prove each of the following by contradiction:

(a) √3 is irrational.  
(b) There is no largest prime number. (Euclid's theorem; prove it in your own words.)  
(c) There is no rational number r with r² = 3.  
(d) If p is prime and p | ab, then p | a or p | b (Euclid's lemma — you may use the fact that gcd(p, a) is either 1 or p).

**1.3.4** (Mathematical induction) Prove each of the following by induction:

(a) For all n ≥ 1: 1² + 2² + ... + n² = n(n+1)(2n+1)/6.  
(b) For all n ≥ 0: the number of subsets of an n-element set is 2ⁿ.  
(c) For all n ≥ 1: 1/(1·2) + 1/(2·3) + ... + 1/(n(n+1)) = n/(n+1).  
(d) For all n ≥ 1: n < 2ⁿ.  
(e) (Strong induction) Every integer n ≥ 2 has a prime factorization.

**1.3.5** (Choosing the right strategy) For each of the following, identify the most natural proof strategy (direct, contrapositive, contradiction, induction) and briefly justify your choice before proving it:

(a) For all n ∈ ℕ, 3 | (n³ - n).  
(b) There is no bijection between ℕ and ℙ(ℕ) (the power set of ℕ).  
(c) If f and g are both differentiable at a point a, then so is f + g.  
(d) For all n ∈ ℕ, if n² is odd then n is odd.

---

## Thought Experiments

**TE 1.1** (*The Liar's Paradox and Self-Reference*)  
The sentence "This statement is false" is not a proposition (it has no stable truth value). What happens if you try to assign it the truth value True? False? Can you construct a mathematical statement (about a formal system) that has the same self-referential character? This is related to Gödel's incompleteness theorems. Do some research: what does Gödel's theorem actually say, and why does it apply to formal systems powerful enough to express arithmetic?

**TE 1.2** (*The Power of Contrapositive*)  
Consider the following four statements:
1. If the metric is flat, the Riemann tensor vanishes.
2. If the Riemann tensor does not vanish, the metric is not flat.
3. If the metric is not flat, the Riemann tensor does not vanish.
4. If the Riemann tensor vanishes, the metric is flat.

Which of these are logically equivalent to which? (Identify the original, converse, contrapositive, inverse.) Which are true in general relativity? (Research this — the relationship between local flatness and the Riemann tensor is subtle and central to GR.)

**TE 1.3** (*Counterexamples and the Scientific Method*)  
In physics, we use the pattern: "If theory T is correct, then experiment E should yield result R. Experiment E did not yield R. Therefore T is not correct (in its current form)." 

(a) Identify the logical form of this argument.  
(b) The 1887 Michelson-Morley experiment failed to detect ether drift. What theory did it refute, and how clean was the logical refutation?  
(c) Explain why a positive experimental result can never *prove* a theory (only a negative result can *disprove* it). This is Popper's falsifiability criterion (Popper, 1934/1959).

**TE 1.4** (*Induction and Physical Law*)  
The uniformity of nature — the assumption that physical laws apply everywhere and at all times — is sometimes described as "inductive inference from observations." How is this different from mathematical induction? Can you construct an argument (using logical tools from this chapter) for why scientific induction (observing a regularity and inferring a general law) is philosophically weaker than mathematical induction?

**TE 1.5** (*Vacuous Truth and the Equivalence Principle*)  
The implication P → Q is vacuously true when P is false. Suppose we try to test the equivalence principle in flat spacetime (where gravity = 0). The principle says: "If an observer is in a gravitational field, they cannot distinguish it from acceleration by local experiments." In flat spacetime, the hypothesis is false. Does the equivalence principle hold vacuously in flat spacetime? What does this tell us about how we should interpret physical principles stated as conditionals?

---

## Laboratory Projects

**Lab 1.1** (*Truth Table Verification with Code*)  
Write a computer program (in Python, Julia, or your preferred language) that:
(a) Takes a propositional formula as input (e.g., "(P → Q) ∧ ¬Q → ¬P").
(b) Generates the complete truth table.
(c) Determines whether the formula is a tautology, contradiction, or contingent.

Test your program on all the formulas from Exercise 1.1.2. Use it to verify De Morgan's laws. Then use it to verify modus ponens and modus tollens as tautologies.

*Learning goal*: The computational implementation of truth tables reveals that propositional logic is *decidable* — there is a finite mechanical procedure (checking all rows of the truth table) that determines the truth value of any propositional formula. This decidability fails for predicate logic, as Turing proved (Church-Turing theorem, 1936).

**Lab 1.2** (*The Limits of Rational Approximation*)  
The irrationality of √2 means it cannot be expressed as p/q. But rational numbers can *approximate* it arbitrarily well. 

(a) Compute the continued fraction expansion of √2: √2 = 1 + 1/(2 + 1/(2 + 1/(2 + ...))).
(b) Compute the first ten convergents p_n/q_n of this continued fraction and find how close each is to √2.
(c) Prove (or look up) that the error |√2 - p_n/q_n| < 1/q_n². This is related to **Dirichlet's approximation theorem**.
(d) *Connection to physics*: The Kepler problem in GR involves precessing orbits. The ratio of the orbital period to the precession period is generally irrational. When this ratio is well-approximated by a rational number, **orbital resonances** can occur. The continued fraction expansion of an irrational number determines how resistant it is to rational approximation — and hence how resistant the corresponding orbit is to resonance. Investigate the concept of "KAM (Kolmogorov-Arnold-Moser) tori" in Hamiltonian mechanics and connect it to the mathematics of irrational approximation.

**Lab 1.3** (*Gödel's Incompleteness: A Computational Experiment*)  
Gödel's theorem says that in any consistent formal system powerful enough to describe arithmetic, there are true statements that cannot be proved within the system. One of the clearest examples is the **Goodstein sequence** (Goodstein, 1944):

(a) Look up the definition of the Goodstein sequence G_n(m) for a given starting value m.
(b) Compute the Goodstein sequence for m = 3 by hand for the first several steps.
(c) Goodstein's theorem states: every Goodstein sequence eventually terminates (reaches 0). This is *true* but *cannot be proved in Peano Arithmetic* (the standard axioms of arithmetic). It can be proved using infinite ordinals (transfinite induction), which go beyond Peano Arithmetic.
(d) Write a program to compute Goodstein sequences. For m = 4, how many steps does it take to reach 0? (Warning: it is a very large number.)

*Learning goal*: This provides a concrete, computable example of Gödel's incompleteness — a true mathematical statement that lies beyond the reach of a specific formal system.

---

## Further Reading and References

See [further-reading.md](further-reading.md) for the annotated bibliography for this chapter.

---

## Important Concepts in Chapter 1

See [important-concepts.md](important-concepts.md) for the glossary.

---

## Important Researchers in Chapter 1

See [important-researchers.md](important-researchers.md) for biographical portraits.
