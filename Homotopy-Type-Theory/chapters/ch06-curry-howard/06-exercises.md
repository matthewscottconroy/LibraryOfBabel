# Exercises

---

**Exercise 6.1.** For each of the following propositions, write the corresponding type in STLC, and write a proof term (lambda expression) that has that type:

(a) $(P \wedge Q) \to (Q \wedge P)$

(b) $(P \to Q) \to (Q \to R) \to (P \to R)$

(c) $P \vee Q \to Q \vee P$

(d) $(P \wedge (P \to Q)) \to Q$ (modus ponens with a conjunction)

(e) $((P \to Q) \to P) \to P$ — is this provable? If not, which classical axiom would give it?

---

**Exercise 6.2.** Verify that each term you wrote in Exercise 6.1 satisfies the typing rules: write out the complete typing derivation tree for at least two of your terms.

---

**Exercise 6.3.** Reduce each term to normal form. Show each reduction step:

(a) $(\lambda x : A.\, \lambda y : B.\, x)\, a\, b$ for $a : A$, $b : B$

(b) $\text{fst}((\lambda p : A \times B.\, (\text{snd}(p), \text{fst}(p)))\, (a, b))$

(c) $(\lambda f : A \to A.\, f\,(f\, a))\, (\lambda x : A.\, x)$ for $a : A$

---

**Exercise 6.4 (SKI Combinators).** Define:
- $I = \lambda x : A.\, x$ (identity)
- $K = \lambda x : A.\, \lambda y : B.\, x$ (constant)
- $S = \lambda f : A \to B \to C.\, \lambda g : A \to B.\, \lambda x : A.\, f\, x\, (g\, x)$ (substitution)

(a) What types do $I$, $K$, $S$ have? What propositions do their types correspond to?

(b) Show that $S\, K\, K$ has the same type as $I$ and behaves identically. (This shows $I$ is "definable from $S$ and $K$.")

(c) Show that $K : A \to B \to A$ corresponds to the axiom "A implies B implies A" (axiom 1 of IPC). What proof strategy does $K$ represent?

---

**Exercise 6.5.** Under the Curry-Howard correspondence, the elimination rule for $\exists$ is pattern matching on a pair:

$$\frac{\Gamma \vdash s : \Sigma_{x:A} P(x) \quad \Gamma, x:A, p:P(x) \vdash t : C}{\Gamma \vdash \text{split}(s, \lambda x.\, \lambda p.\, t) : C}$$

(a) Write a term of type $(\Sigma_{n:\mathbb{N}} \text{Even}(n)) \to \mathbb{N}$ that extracts the witness from an existential statement "there exists an even natural number." (Just extract the first component.)

(b) Write a dependent term of type $\Sigma_{f: \mathbb{N} \to \mathbb{N}} (\forall n:\mathbb{N}, f(n) > n)$ — a function with a proof that it always returns a larger value. (Hint: the function $n \mapsto n+1$ works.)

---

**Exercise 6.6 (Strong Normalization Counterexample in Untyped λ-Calculus).** The untyped term $\Omega = (\lambda x.\, x\, x)\, (\lambda x.\, x\, x)$ does not terminate.

(a) Show that $\Omega \to_\beta \Omega$ (it reduces to itself).

(b) Explain why $\Omega$ cannot be assigned a type in STLC. What type constraint fails?

(c) This shows strong normalization for STLC is a non-trivial theorem, not a vacuous one. Explain why the typing discipline is what prevents non-termination.

---

**Exercise 6.7.** In the proof of strong normalization (Section 3.1), we defined $\text{Red}(A \to B)$ as the set of terms $t$ such that $t\, s \in \text{Red}(B)$ for all $s \in \text{Red}(A)$.

(a) Verify that $\text{Red}(A \to B)$ satisfies CR2: if $t \in \text{Red}(A \to B)$ and $t \to_\beta t'$, then $t' \in \text{Red}(A \to B)$.

(b) Verify the key step: $\lambda x.\, t \in \text{Red}(A \to B)$ if $t[s/x] \in \text{Red}(B)$ for all $s \in \text{Red}(A)$.

---

**Exercise 6.8 (Identity Types).** 

(a) In Lean 4, prove the following without using library lemmas (only `rfl` and `▸`):
   - `congrArg : (f : α → β) → a = b → f a = f b`
   - `congrFun : (f g : α → β) → f = g → (x : α) → f x = g x`

(b) What are these lemmas called under the homotopy interpretation? (Hint: one is "applying a continuous function to a path gives a path.")

---

**Exercise 6.9 (Classical Computation).** Consider adding the following "oracle" to STLC:

```
classical : (A → 𝟎) → 𝟎 → A  -- double negation elimination as a function
```

(a) What type does this oracle have? What proposition does it correspond to?

(b) Show that with this oracle, you can derive LEM: write a term of type $A + (A \to \mathbf{0})$.

(c) In the presence of this oracle, does strong normalization still hold? (Hint: consider $\lambda f : A \to \mathbf{0}.\, \text{classical}\, f\, (\text{classical}\, f\, \ldots)$)

---

**Exercise 6.10 (Lean 4 Practice).** Open Lean 4 (or use the online Lean 4 playground) and verify the following:

(a) Write proof terms (not tactic proofs) for the propositions in Exercise 6.1.

(b) Write a proof of `∀ (P Q : Prop), P ∧ Q ↔ Q ∧ P` using only `fun`, `⟨_, _⟩`, `.1`, `.2`, and `Iff.intro`.

(c) Write a dependent function `repeat_n : {α : Type} → (f : α → α) → ℕ → α → α` that applies `f` exactly `n` times. What is the type of `repeat_n`? What proposition does it correspond to?

---

**Exercise 6.11 (Conceptual).** The Curry-Howard correspondence was discovered in stages:
- Curry (1934): combinators ↔ axioms of implicational logic
- Howard (1969/1980): natural deduction proofs ↔ typed lambda terms
- Girard (1971): System F ↔ second-order logic
- Martin-Löf (1975+): dependent type theory ↔ predicate logic
- Voevodsky (2006+): HoTT ↔ homotopy theory

For each extension, explain what new logical or computational phenomenon it captured that the previous stage could not handle. Write 2–3 sentences per stage.
