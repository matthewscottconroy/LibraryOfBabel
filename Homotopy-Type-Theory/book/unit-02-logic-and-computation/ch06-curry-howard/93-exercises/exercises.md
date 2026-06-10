# Exercises: The Curry-Howard Correspondence

## Section 1: Propositions as Types

**Exercise 1.** Fill in the right column of the Curry-Howard dictionary:

| Logic | Type Theory |
|---|---|
| $A \wedge B$ | ? |
| $A \vee B$ | ? |
| $A \to B$ | ? |
| $\bot$ | ? |
| $\top$ | ? |
| $\neg A$ | ? |
| $\forall x : A, B(x)$ | ? |
| $\exists x : A, B(x)$ | ? |

**Exercise 2.** Write the term (program) corresponding to each of the following proofs under Curry-Howard:

(a) The proof of $A \to A$ (identity)
(b) The proof of $A \wedge B \to B \wedge A$ (commutativity of conjunction)
(c) The proof of $(A \to B) \to (B \to C) \to (A \to C)$ (transitivity / composition)
(d) The proof of $\neg A \vee B \to A \to B$
(e) The proof of $A \to \neg\neg A$

**Exercise 3.** What proposition (logical formula) does each of the following programs prove?

(a) $\lambda x : A.\, x$
(b) $\lambda p : A \times B.\, (\pi_2\, p, \pi_1\, p)$
(c) $\lambda f : A \to B.\, \lambda g : B \to C.\, \lambda x : A.\, g\, (f\, x)$
(d) $\lambda f : A \times B \to C.\, \lambda x : A.\, \lambda y : B.\, f\, (x, y)$
(e) $\lambda e : \mathbf{0}.\, \text{absurd}\, e$

**Exercise 4.** Under Curry-Howard, what is the computational interpretation of each of the following natural deduction rules?

(a) $\wedge$I (conjunction introduction)
(b) $\to$E (modus ponens)
(c) $\vee$E (proof by cases)
(d) $\exists$I (existential introduction)
(e) $\bot$E (ex falso)

## Section 2: Simply Typed Lambda Calculus

**Exercise 5.** Derive typing judgments (write a full derivation tree) for each of the following:

(a) $\vdash \lambda x : A.\, \lambda y : B.\, x : A \to B \to A$
(b) $f : A \to B, x : A \vdash f\, x : B$
(c) $\vdash \lambda p : A \times B.\, (\pi_2\, p, \pi_1\, p) : A \times B \to B \times A$

**Exercise 6.** Determine which of the following terms are well-typed in STLC. For well-typed ones, give the type; for ill-typed ones, explain why:

(a) $\lambda x : A.\, x\, x$ (self-application)
(b) $\lambda f : A \to B.\, \lambda x : A.\, f\, x$
(c) $\lambda f : A.\, f$ (when $A$ is a base type)
(d) $(\lambda x : A.\, x)(\lambda x : A.\, x)$
(e) $\lambda x : A.\, (\lambda y : A \to B.\, y\, x)$

**Exercise 7.** Compute the normal form (result of beta reduction) of each of the following:

(a) $(\lambda x : A.\, x)\, a$
(b) $\pi_1\, (a, b)$
(c) $(\lambda f : A \to B.\, \lambda g : B \to C.\, \lambda x : A.\, g\, (f\, x))\, h\, k$
(d) $\text{case}(\mathsf{inl}(a), x.t_1, y.t_2)$

**Exercise 8.** State the Substitution Lemma for STLC. Give a proof by induction on the derivation of $\Gamma, x : A \vdash t : B$ for the following cases:

(a) $t = x$ (the variable being substituted)
(b) $t = y$ for $y \neq x$ (a different variable)
(c) $t = f\, s$ (application)

**Exercise 9.** State the Preservation Theorem (Subject Reduction) for STLC. Give a proof for the case of beta reduction: if $\Gamma \vdash (\lambda x : A.\, t)\, s : B$ and $(\lambda x : A.\, t)\, s \to_\beta t[s/x]$, then $\Gamma \vdash t[s/x] : B$.

**Exercise 10.** Explain the difference between Church-style and Curry-style lambda calculus. Give an example of a term that has a unique type in Church-style but multiple types in Curry-style.

## Section 3: Normalization and Consistency

**Exercise 11.** Define the reducibility predicate $\text{Red}(A)$ for:

(a) A base type $\alpha$
(b) A function type $A \to B$
(c) A product type $A \times B$

State the three key properties CR1, CR2, CR3.

**Exercise 12.** Prove that $\text{Red}(\alpha)$ satisfies CR1, CR2, CR3 for a base type $\alpha$, where $\text{Red}(\alpha)$ is defined as the set of strongly normalizing terms of type $\alpha$.

**Exercise 13.** State the Main Lemma of the normalization proof (the lemma that all well-typed terms are reducible). Prove it for the case of lambda abstraction: if the Main Lemma holds for the body, it holds for the abstraction.

**Exercise 14.** Derive the consistency of intuitionistic propositional logic from strong normalization: there is no closed term of the empty type $\mathbf{0}$, hence $\bot$ is not provable.

**Exercise 15.** The *Church-Rosser property* says: if $t \to^* s_1$ and $t \to^* s_2$, then there is $u$ with $s_1 \to^* u$ and $s_2 \to^* u$. Explain why Church-Rosser implies uniqueness of normal forms.

## Section 4: Dependent Types

**Exercise 16.** State the formation, introduction, elimination, and computation rules for the $\Pi$-type $\Pi_{x:A} B(x)$.

**Exercise 17.** Explain how $\Pi_{x:A} B(x)$ generalizes the function type $A \to B$. Give a specific example of a dependent function (one where the output type genuinely depends on the input value) that cannot be expressed as an ordinary function type.

**Exercise 18.** State the formation, introduction, elimination, and computation rules for the $\Sigma$-type $\Sigma_{x:A} B(x)$.

**Exercise 19.** Express the following as types in a dependent type theory:

(a) "Every vector of length $n + m$ can be split into a vector of length $n$ and a vector of length $m$."
(b) "There exists a prime number greater than 100."
(c) "Every function $f : \mathbb{N} \to \mathbb{N}$ that is strictly increasing satisfies $f(n) \geq n$ for all $n$."

**Exercise 20.** The identity type $a =_A b$ is introduced by `refl` and eliminated by path induction ($J$). State the $J$ eliminator precisely. Explain why it says "to prove something about all identity proofs, it suffices to prove it for reflexivity."

## Proof-Level Exercises

**Exercise 21.** Prove, using the Curry-Howard correspondence, that the following types are inhabited (the corresponding propositions are provable) by exhibiting explicit terms:

(a) $(A \to B) \to (A \to B \to C) \to A \to C$
(b) $(A \times B) \to (B \times A)$
(c) $(A \to (B \to C)) \to (A \times B) \to C$
(d) $(A + B) \to (A \to C) \to (B \to C) \to C$

**Exercise 22.** Prove that the following types are *uninhabited* in STLC (the corresponding propositions are not IPC-valid) by showing that any purported term would not type-check or would require self-application:

(a) $A + (A \to \mathbf{0})$ (LEM)
(b) $(A \to \mathbf{0} \to \mathbf{0}) \to A$ (DNE)

**Exercise 23.** Under the Curry-Howard correspondence, what is the computational interpretation of Peirce's law $((A \to B) \to A) \to A$? What kind of program would have this type? (Hint: consider call/cc.)

**Exercise 24.** Write a Lean 4 (or Coq or Agda) proof of commutativity of conjunction: `∀ A B : Prop, A ∧ B → B ∧ A`. Examine the proof term produced. Identify which term constructors correspond to which proof steps.

**Exercise 25.** The *principle of explosion* (ex falso quodlibet) says: from a proof of $\bot$, any proposition follows. Under Curry-Howard, this is the existence of a function `absurd : 𝟎 → A` for any type `A`. Write this function. What does it compute? Can it ever be called?

## Advanced Exercises

**Exercise 26.** The *J eliminator* for the identity type has the type:

$$J : \Pi_{A:\mathsf{Type}} \Pi_{C : \Pi_{a:A} \Pi_{b:A} (a =_A b) \to \mathsf{Type}} \Pi_{c : \Pi_{a:A} C\, a\, a\, \mathsf{refl}_a} \Pi_{a,b:A} \Pi_{p : a =_A b} C\, a\, b\, p$$

Write out what $J$ says in English. Prove symmetry of equality ($a = b \to b = a$) using $J$.

**Exercise 27.** Define (in dependent type theory) the type `Vec A n` of vectors of type `A` and length `n`. Give the types of `nil : Vec A 0`, `cons : A → Vec A n → Vec A (n+1)`, and `append : Vec A m → Vec A n → Vec A (m+n)`.

**Exercise 28.** The *Curry isomorphism* (currying) says $(A \times B) \to C \simeq A \to (B \to C)$. Write the two functions establishing the isomorphism in STLC. What are their types? What do their beta-normal forms look like?

**Exercise 29.** A *free theorem* from a type signature: a term of type $\forall \alpha.\, \alpha \to \alpha$ must be the identity. Explain informally why this is true. What property of System F (Reynolds' parametricity) does this depend on?

**Exercise 30.** Prove in STLC (by constructing terms) that the following types are isomorphic: $A \to (B \times C) \simeq (A \to B) \times (A \to C)$. Give both functions and verify the isomorphism equations hold by beta reduction.
