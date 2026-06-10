# Exercises

---

**Exercise 8.1.** In each case, determine whether the type is inhabited, and if so, construct a term:

(a) $\prod_{A : \mathsf{Type}} A \to A$ — the polymorphic identity

(b) $\sum_{n : \mathbb{N}} \mathsf{IsEven}(n) \times (n > 10)$ — an even number greater than 10

(c) $\prod_{n : \mathbb{N}} \mathsf{IsEven}(n) \lor \mathsf{IsOdd}(n)$ — every natural number is even or odd

(d) $\prod_{n : \mathbb{N}} \mathsf{IsEven}(n) \times \mathsf{IsOdd}(n)$ — every natural number is both even and odd (is this inhabited?)

---

**Exercise 8.2.** Write out the full type of the `append` function for vectors, then define it by induction. Show the computation for `append [1,2] [3,4]` step by step.

$$\mathsf{append} : ?$$
$$\mathsf{append}\, [1,2]\, [3,4] \to_\beta^* \ldots$$

---

**Exercise 8.3 (Type-Theoretic Curry-Howard).** For each logical formula, write the corresponding type and construct a term (proof) of that type:

(a) $\forall A, B : \mathsf{Prop}. (A \Rightarrow B) \Rightarrow A \Rightarrow B$ — modus ponens

(b) $\forall A, B, C : \mathsf{Prop}. (A \Rightarrow B \Rightarrow C) \Rightarrow (A \Rightarrow B) \Rightarrow A \Rightarrow C$ — the S combinator

(c) $\forall A : \mathsf{Prop}. A \Rightarrow \neg\neg A$ — double negation introduction

(d) $\forall A, B : \mathsf{Prop}. (\neg A \lor B) \Rightarrow (A \Rightarrow B)$ — classical implication

---

**Exercise 8.4 (Induction Proofs).** Prove the following by induction, writing out both the informal proof and the type-theoretic term:

(a) $\prod_{n : \mathbb{N}} 0 + n = n$

(b) $\prod_{m\, n : \mathbb{N}} \mathsf{succ}(m) + n = \mathsf{succ}(m + n)$

(c) $\prod_{m\, n : \mathbb{N}} m + n = n + m$ (commutativity of addition) — this requires both (a) and (b)

(d) $\prod_{n : \mathbb{N}} n \times 0 = 0$

---

**Exercise 8.5 (Universe Levels).** Determine the smallest universe level at which each of the following lives:

(a) $\mathbb{N}$

(b) $\mathbb{N} \to \mathsf{Type}_0$

(c) $\prod_{A : \mathsf{Type}_0} A \to A$

(d) $\sum_{A : \mathsf{Type}_0} (A \to A)$

(e) $\mathsf{Type}_0 \to \mathsf{Type}_0$

---

**Exercise 8.6 (Dependent Currying).** Prove the dependent currying isomorphism:

$$\left(\sum_{a:A} B(a) \to C\right) \simeq \left(\prod_{a:A} B(a) \to C\right)$$

Explicitly:
(a) Define the forward map $f \mapsto \lambda a. \lambda b. f\, (a, b)$.
(b) Define the backward map $g \mapsto \lambda p. g\, (\pi_1 p)\, (\pi_2 p)$.
(c) Show these are inverse by computing the round-trips via $\beta$/$\eta$.

---

**Exercise 8.7 (W-Types).** The W-type $W_{a:A} B(a)$ with $A = \mathbf{1} + \mathbf{1}$ (two-element type $\{\mathsf{inl}(\mathsf{tt}), \mathsf{inr}(\mathsf{tt})\}$) and:
$$B(\mathsf{inl}(\mathsf{tt})) = \mathbf{0}, \quad B(\mathsf{inr}(\mathsf{tt})) = \mathbf{1}$$

(a) What is this W-type isomorphic to? Describe its elements.

(b) Write the isomorphism explicitly: a map $\mathbb{N} \to W$ and a map $W \to \mathbb{N}$.

(c) More generally: given $A = \{c_0, c_1\}$ and $B(c_0) = \mathbf{0}$, $B(c_1) = \mathbf{1}$, justify that $W_{a:A} B(a) \cong \mathbb{N}$ using the structural properties of W-types.

---

**Exercise 8.8 (Constructive Axiom of Choice).** Prove the following theorem directly, by giving an explicit term:

$$\left(\prod_{a:A} \sum_{b:B(a)} C(a, b)\right) \to \sum_{f : \prod_{a:A} B(a)} \prod_{a:A} C(a, f\, a)$$

(a) Write the proof term explicitly.
(b) Explain why this is constructively trivial but classically non-trivial.
(c) In classical set theory, AC is non-constructive because you can have $\forall x, \exists y, P(x,y)$ without a definable choice function. What feature of constructive type theory prevents this?

---

**Exercise 8.9 (Type Families as Predicates).** For each mathematical predicate, write the corresponding type family and give an example of an element of each Σ-type:

(a) "$n$ is a perfect square": $P : \mathbb{N} \to \mathsf{Type}$ where $P(n) = \sum_{k:\mathbb{N}} n = k^2$

(b) "$f$ is injective": $\mathsf{Injective}(f) = \prod_{a\, b : A} f\, a = f\, b \to a = b$

(c) "$l$ is sorted": $\mathsf{Sorted} : \mathsf{List}(\mathbb{N}) \to \mathsf{Type}$ (define this inductively)

(d) "$G$ forms a group": write $\mathsf{IsGroup}(G, \cdot, e, \mathsf{inv})$ as a Σ-type of group axioms

---

**Exercise 8.10 (Identity Types: First Properties).** The identity type $\mathsf{Id}_A(a, b)$ will be developed fully in Chapter 9. But using only the constructor $\mathsf{refl} : a = a$ and the J-rule:

$$J : \prod_{P : \prod_{b:A} (a = b) \to \mathsf{Type}} P(a, \mathsf{refl}) \to \prod_{b:A} \prod_{p:a=b} P(b, p)$$

(a) Define transport: $\mathsf{transport} : \prod_{B : A \to \mathsf{Type}} a = b \to B(a) \to B(b)$

(b) Define symmetry: $\mathsf{sym} : a = b \to b = a$

(c) Define transitivity: $\mathsf{trans} : a = b \to b = c \to a = c$

(d) Define $\mathsf{ap}$: $\mathsf{ap}_f : a = b \to f\, a = f\, b$ for $f : A \to B$

In each case, write out the term explicitly using $J$.

---

**Exercise 8.11 (Research Problem).** The *propositions-as-types* correspondence becomes subtle when dealing with classical axioms.

(a) Explain why $\neg\neg P \to P$ (double negation elimination) has no canonical computational interpretation.

(b) In a proof assistant, if you add LEM as an axiom, you can still prove theorems, but those proofs may not compute to normal forms. Explain what "not computing" means for a type-theoretic proof.

(c) Describe the difference between:
   - `Prop` in Lean 4 (a universe with proof irrelevance)
   - `Type` in Lean 4 (a universe with computational content)
   
   Why would you put a statement in `Prop` vs. `Type`?

(d) In Agda, the `--prop` flag enables a separate universe of propositions. Look up or reason about: what changes about the Σ type when the second component lives in `Prop`?
