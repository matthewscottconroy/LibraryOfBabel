# Compactness and Derivation

## Natural Deduction Trees

Proofs, formally speaking, are not paragraphs. They are trees. Natural deduction — invented by Gerhard Gentzen in 1935 — organizes proofs as tree-shaped derivations, where each node carries a formula and is justified by an inference rule applied to its children.

Here is the notation. A *derivation* of φ from hypotheses Γ is a finite tree whose leaves are hypotheses (or axioms), whose root is labeled φ, and where each internal node is justified by a rule. We write the rule:

```
  φ   ψ
  -----  (rule name)
  φ ∧ ψ
```

to mean: from proofs of φ and ψ, we derive φ ∧ ψ (using the conjunction-introduction rule).

The complete set of natural deduction rules for propositional logic:

**Implication:**
```
[φ]
 :
 ψ
----  (→I)     φ → ψ   φ
φ → ψ         ----------  (→E)
                   ψ
```

The →I rule says: if you can derive ψ by assuming φ (marked [φ] to show it is a discharged hypothesis), then you can derive φ → ψ. The →E rule (modus ponens) says: from φ → ψ and φ, derive ψ.

**Conjunction:**
```
φ   ψ               φ ∧ ψ         φ ∧ ψ
-----  (∧I)         -----  (∧E₁)  -----  (∧E₂)
φ ∧ ψ                 φ              ψ
```

**Disjunction:**
```
  φ              ψ          φ ∨ ψ  [φ]  [ψ]
-----  (∨I₁)  -----  (∨I₂)        :     :
φ ∨ ψ         φ ∨ ψ               χ     χ
                              ----------  (∨E)
                                   χ
```

**Negation:**
```
[φ]                φ   ¬φ
 :                 --------  (¬E)
 ⊥                     ⊥
----  (¬I)
 ¬φ
```

**Falsum:**
```
  ⊥
-----  (⊥E, ex falso quodlibet)
  φ
```

**Double negation (classical):**
```
¬¬φ
---  (DNE)
 φ
```

The DNE rule is what makes this system *classical*. Without it (or equivalent rules like proof by contradiction or excluded middle), we have *intuitionistic* natural deduction. This distinction is crucial: HoTT uses intuitionistic logic, without DNE. We study both.

## A Sample Derivation

Let us prove the hypothetical syllogism: (P → Q) → (Q → R) → (P → R).

```
        [P]¹  [P → Q]²
        ---------------  (→E)
[Q → R]³      Q
-------------------  (→E)
           R
        --------  (→I, discharging [P]¹)
          P → R
     -----------  (→I, discharging [Q → R]³)
  (Q → R) → (P → R)
-------------------------------  (→I, discharging [P → Q]²)
  (P → Q) → (Q → R) → (P → R)
```

Read the tree from leaves to root. The superscripts mark which hypotheses are discharged at which →I step.

This derivation is not just a proof — it is a program. Under the Curry-Howard correspondence, it is the lambda term:

```
λ(h₂ : P → Q). λ(h₃ : Q → R). λ(h₁ : P). h₃(h₂(h₁))
```

The function composition of h₂ and h₃. Every natural deduction derivation is a lambda term. Every →I is a lambda abstraction. Every →E is a function application.

## The Compactness Theorem

We now prove one of the central results of mathematical logic: the *Compactness Theorem*.

**Theorem (Compactness for Propositional Logic).** A set of propositional formulas Γ is satisfiable if and only if every finite subset of Γ is satisfiable.

The "only if" direction is trivial: if Γ has a model (a satisfying assignment), then every finite subset shares that model.

The "if" direction is the content: if no finite subset is unsatisfiable, the whole infinite set is satisfiable. This is remarkable. An infinite set of constraints can be globally consistent purely because no finite obstruction exists.

**Proof.** We prove the contrapositive of the forward direction for the "if" implication. Suppose Γ is not satisfiable. We show some finite subset is not satisfiable.

Actually, let us prove the result constructively. We construct a satisfying assignment for Γ, given that every finite subset is satisfiable.

Let P₁, P₂, P₃, ... be an enumeration of all atomic propositions appearing in formulas of Γ.

We define a truth assignment v inductively. At stage n, we set v(Pₙ) = T or F as follows:
- Consider Γ ∪ {Pₙ = T} (the set with Pₙ assigned T). If every finite subset of this augmented set is satisfiable, set v(Pₙ) = T.
- Otherwise, set v(Pₙ) = F.

We claim the resulting assignment v satisfies all of Γ.

**Claim.** After setting v(P₁), ..., v(Pₙ), every finite subset of Γ is satisfiable by an assignment extending v|_{P₁,...,Pₙ}.

*Proof of claim by induction on n.* Base: n = 0. This is the hypothesis.

Inductive step: Suppose the claim holds for n. We set v(Pₙ₊₁).

Case 1: Adding {Pₙ₊₁ = T} preserves finite satisfiability. Then v(Pₙ₊₁) = T. By the inductive hypothesis applied to Γ ∪ {Pₙ₊₁ = T}, every finite subset of Γ is satisfiable by an assignment extending v|_{P₁,...,Pₙ₊₁}.

Case 2: Adding {Pₙ₊₁ = T} does not preserve finite satisfiability — some finite Γ₀ ⊆ Γ is not satisfiable with Pₙ₊₁ = T. Then we set v(Pₙ₊₁) = F.

We need to show that every finite subset of Γ is satisfiable by an extension of v|_{P₁,...,Pₙ,Pₙ₊₁=F}.

Let Γ₁ be any finite subset of Γ. By the inductive hypothesis, Γ₁ is satisfiable by some assignment w extending v|_{P₁,...,Pₙ}. Either w(Pₙ₊₁) = F (done) or w(Pₙ₊₁) = T. In the second case, Γ₀ ∪ Γ₁ is finite and by hypothesis every finite subset of Γ is satisfiable. So Γ₀ ∪ Γ₁ is satisfiable by some assignment u. This u cannot have Pₙ₊₁ = T (since Γ₀ is not satisfiable with Pₙ₊₁ = T). So u(Pₙ₊₁) = F, and u satisfies Γ₁ with Pₙ₊₁ = F, as required.

**End of claim.** By the claim, the assignment v satisfies every finite subset of Γ. For any φ ∈ Γ, the singleton {φ} is a finite subset, so v satisfies φ. Thus v satisfies all of Γ. □

## The Compactness Theorem in Predicate Logic

The same result holds for first-order predicate logic:

**Theorem (Compactness for FOL).** A set Γ of first-order sentences has a model if and only if every finite subset of Γ has a model.

The proof uses the *Henkin construction*: given a consistent (finitely satisfiable) set Γ, we extend it to a *complete* and *Henkin* set (one that contains a witness for every existential statement), and build a model from the terms of the language. This model satisfies Γ.

The Henkin construction is remarkable: it builds a model whose *elements are syntactic terms*. The witnesses for existential statements are the constant symbols we add to the language. This "term model" or "canonical model" construction is a paradigm in mathematical logic.

## Applications of Compactness

Compactness is not just a theoretical curiosity. It has striking applications.

**Non-standard models.** Let Γ be the theory of the natural numbers (the set of all sentences true in ℕ). Add a new constant symbol c and the sentences c > 0, c > 1, c > 2, .... Every finite subset is satisfiable (take c to be any sufficiently large natural number). By compactness, the whole set is satisfiable — there is a model with an element c that is larger than every standard natural number. This is a *non-standard model* of arithmetic, with "infinite" elements.

**Four-coloring compactness.** Every finite planar graph is 4-colorable (by the four-color theorem). By compactness, every infinite planar graph is 4-colorable.

**Consistency of ZFC + ¬CH.** Cohen's proof that the Continuum Hypothesis is independent of ZFC uses forcing, but compactness plays a role in establishing that the forcing extension is itself a model of ZFC.

**Tychonoff's theorem via compactness.** The compactness theorem for propositional logic is equivalent to the Boolean Prime Ideal Theorem, which implies the Tychonoff theorem for Hausdorff spaces. Logical compactness and topological compactness are, in a deep sense, the same theorem.

## Limitations of Compactness

Compactness also shows the limits of first-order logic. The theory of well-ordered sets — sets where every non-empty subset has a least element — is *not first-order axiomatizable*. Here is why: let Γ be the axioms of linear order, and add sentences saying there are elements a₁ > a₂ > a₃ > .... Every finite subset is satisfiable (take a finite linear order). By compactness, there is a model with an infinitely descending chain — which is not well-ordered.

So well-foundedness, finiteness, and Archimedeanness (the real numbers are Archimedean: for any r > 0, some natural number exceeds r) are all properties that cannot be captured by any first-order theory. They require second-order logic or other formalisms. This is the *expressive limitation* of first-order logic.

In type theory, we overcome this by working with intensional rather than extensional notions: the natural numbers are defined by their elimination rule, not by an axiom that happens to be satisfied by non-standard models. The type of natural numbers in Martin-Löf Type Theory contains no non-standard elements, because the type is defined *by its constructors* — zero and successor — and no other elements exist. This is the advantage of an intensional foundation: mathematical objects are their definitions, and the definitions uniquely determine them.

Compactness is both a beautiful theorem and a window into the fundamental gap between syntax and semantics. It tells us that local (finite) consistency implies global consistency — a powerful tool. And its failure in second-order logic tells us that first-order logic is powerful enough to be useful but not powerful enough to be categorical.

That tension — between what logic can express and what mathematics requires — is one of the engines driving the development of dependent type theory.
