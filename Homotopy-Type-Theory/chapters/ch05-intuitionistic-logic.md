# Chapter 5: Intuitionistic Logic and Constructive Mathematics

## Introduction

Classical mathematics operates under the assumption that every proposition is either true or false, whether or not we know which. The law of excluded middle ($P \vee \neg P$) is taken as a logical axiom. Proof by contradiction is standard. Functions may be asserted to exist without any algorithm for computing them.

Constructive mathematics — and in particular *intuitionistic* logic — rejects this. In constructive mathematics, a proof of $P$ must be a *construction*: a method, algorithm, or explicit witness. A proof of $P \vee Q$ must say *which* disjunct holds and give a proof of it. A proof of $\exists x, P(x)$ must exhibit a specific $x$ and a proof of $P(x)$.

This is not merely a philosophical position. It has profound mathematical and computational consequences:
- Constructive proofs are *programs*. The Curry-Howard correspondence (Chapter 6) makes this precise.
- Constructive existence theorems have algorithmic content. A constructive proof of "there exists a root of this polynomial" gives you an algorithm to find the root.
- Constructive logic is the internal logic of toposes, of sheaf models, and of Martin-Löf type theory. HoTT is built on constructive foundations.

However, constructivism does not simply "reject" classical mathematics. Rather, it asks: *which theorems have constructive proofs, and when does a classical proof actually hide computable content?* Understanding where the classical/constructive boundary lies is one of the central skills of the type-theorist.

---

## 1. The Brouwer-Heyting-Kolmogorov (BHK) Interpretation

The BHK interpretation defines what it means to have a *proof* of each kind of proposition, constructively.

**A proof of $P \wedge Q$** is a pair: a proof of $P$ and a proof of $Q$.
$$\text{proof of } P \wedge Q \;\Leftrightarrow\; \text{pair } (p, q) \text{ where } p \text{ proves } P \text{ and } q \text{ proves } Q$$

**A proof of $P \vee Q$** is either: a proof of $P$ (tagged as "left") or a proof of $Q$ (tagged as "right"). We must say which.
$$\text{proof of } P \vee Q \;\Leftrightarrow\; \text{either } \mathsf{inl}(p) \text{ (with } p \text{ proving } P) \text{ or } \mathsf{inr}(q) \text{ (with } q \text{ proving } Q)$$

**A proof of $P \to Q$** is a *function*: a method that transforms any proof of $P$ into a proof of $Q$.
$$\text{proof of } P \to Q \;\Leftrightarrow\; \text{function } f \text{ such that given any proof } p \text{ of } P, f(p) \text{ is a proof of } Q$$

**A proof of $\neg P$** (i.e., $P \to \bot$) is a function that turns any proof of $P$ into a proof of $\bot$ — which is impossible, since $\bot$ has no proof. So $\neg P$ means: we have a method showing that any purported proof of $P$ leads to absurdity.
$$\text{proof of } \neg P \;\Leftrightarrow\; \text{function showing } P \text{ has no proof}$$

**A proof of $\forall x : A, P(x)$** is a function: for any element $a : A$, a proof of $P(a)$.
$$\text{proof of } \forall x : A, P(x) \;\Leftrightarrow\; \text{function } f \text{ with } f(a) \text{ proving } P(a) \text{ for any } a : A$$

**A proof of $\exists x : A, P(x)$** is a pair: a specific element $a : A$ and a proof of $P(a)$.
$$\text{proof of } \exists x : A, P(x) \;\Leftrightarrow\; \text{pair } (a, p) \text{ where } a : A \text{ and } p \text{ proves } P(a)$$

**$\bot$ has no proof** and thus $\bot$ is never derivable (constructively or classically, from no hypotheses).

**Remark 5.1.** The BHK interpretation is deliberately informal — it says "function" and "proof" without specifying what these are. Different *realizability* and *constructive* systems formalize it differently. The most important formalization for us is *dependent type theory*: under Curry-Howard, each BHK clause is exactly the rule for the corresponding type constructor.

---

## 2. Why the Law of Excluded Middle Fails Constructively

Under BHK, a proof of $P \vee \neg P$ (LEM) would have to be: either a proof of $P$ or a proof of $\neg P$ — for every proposition $P$. But we cannot construct such a thing in general:

**Example 5.2.** Let $P$ be "there exist infinitely many twin primes." We do not know (as of 2026) whether this is true or false. A constructive proof of $P \vee \neg P$ would have to tell us which one holds — but we have no such proof.

More formally: the *rule* $\Gamma \vdash P \vee \neg P$ is not *derivable* in intuitionistic logic. This means: there is no proof tree using only the natural deduction rules of Chapter 4 (without DNE) that derives $P \vee \neg P$ from no hypotheses.

**Example 5.3.** The following classical theorem is not constructively valid: "Every bounded monotone sequence of real numbers converges." The classical proof uses LEM to say: either the supremum is attained or it is a limit — but a constructive proof would require an algorithm to compute the limit, which is not given by the hypothesis.

**Example 5.4 (Double Negation).** Classically, $\neg\neg P \leftrightarrow P$. Constructively, we have $P \to \neg\neg P$ (from a proof $p$ of $P$, define $f(\phi) = \phi(p)$ where $\phi : P \to \bot$), but not $\neg\neg P \to P$ in general.

---

## 3. Intuitionistic Propositional Logic (IPC)

Formally, *intuitionistic propositional logic* is the natural deduction system of Chapter 4, *minus* the classical rule DNE (double negation elimination) or LEM.

### 3.1 Axioms for IPC (Hilbert-Style)

Alternatively, IPC can be presented with these axiom schemes and modus ponens:
1. $P \to (Q \to P)$
2. $(P \to (Q \to R)) \to ((P \to Q) \to (P \to R))$
3. $P \to P \wedge P$
4. $P \wedge Q \to P$
5. $P \wedge Q \to Q$
6. $P \to P \vee Q$
7. $Q \to P \vee Q$
8. $(P \to R) \to (Q \to R) \to (P \vee Q) \to R$
9. $\bot \to P$ (ex falso)

These axioms are all *intuitionistically valid* (they have BHK proofs). The classical axiom $(P \to Q) \to (\neg Q \to \neg P)$ is an intuitionistic theorem too — but $\neg\neg P \to P$ is not.

### 3.2 The Disjunction Property

One of the most important features of intuitionistic logic:

**Theorem 5.5 (Disjunction Property).** If IPC proves $\varphi \vee \psi$, then IPC proves $\varphi$ or IPC proves $\psi$.

This fails for classical logic! Classical logic proves $P \vee \neg P$ without proving $P$ and without proving $\neg P$.

**Proof idea.** Use a model of IPC (Kripke semantics, below) in which $P$ and $\neg P$ are both "not forced at any world." Then $P \vee \neg P$ is not forced, contradicting the assumption. $\square$

The disjunction property is analogous to *canonicity* in type theory: if a closed program of type $A + B$ terminates, it produces either a value of $A$ or a value of $B$.

### 3.3 The Existence Property

**Theorem 5.6 (Existence Property).** If IPC proves $\exists x : \mathbb{N}, P(x)$ (in the arithmetic extension), then there is a specific numeral $\bar{n}$ such that IPC proves $P(\bar{n})$.

Again, this fails classically: classical logic proves $\exists n, (n = 0 \vee n = 1)$ trivially, but the specific $n$ is not determined.

---

## 4. Kripke Semantics

Classical logic is modeled by Boolean algebras (where each proposition is true or false). Intuitionistic logic is modeled by *Heyting algebras* — or, more concretely, by *Kripke frames*.

### 4.1 Kripke Frames

**Definition 5.7.** A *Kripke frame* is a pair $(W, \leq)$ where $W$ is a set of *worlds* and $\leq$ is a *preorder* on $W$ (reflexive and transitive). A *valuation* assigns to each atom $P$ a *persistent* set $V(P) \subseteq W$ — persistent meaning: if $w \in V(P)$ and $w \leq w'$, then $w' \in V(P)$.

**Forcing relation** ($w \Vdash \varphi$, read "world $w$ forces $\varphi$"):
- $w \Vdash P \Leftrightarrow w \in V(P)$ (atoms: given by valuation)
- $w \Vdash \varphi \wedge \psi \Leftrightarrow w \Vdash \varphi$ and $w \Vdash \psi$
- $w \Vdash \varphi \vee \psi \Leftrightarrow w \Vdash \varphi$ or $w \Vdash \psi$
- $w \Vdash \varphi \to \psi \Leftrightarrow$ for all $w' \geq w$: if $w' \Vdash \varphi$ then $w' \Vdash \psi$
- $w \Vdash \bot \Leftrightarrow$ never (false at all worlds)
- $w \Vdash \neg\varphi \Leftrightarrow$ for all $w' \geq w$, $w' \not\Vdash \varphi$

**Key property (Persistence/Monotonicity):** If $w \Vdash \varphi$ and $w \leq w'$, then $w' \Vdash \varphi$. (Truth can only be gained, never lost, as we move to later worlds.)

### 4.2 Soundness and Completeness

**Theorem 5.8 (Kripke Soundness).** If IPC $\vdash \varphi$, then $\varphi$ is valid in all Kripke frames (forced at every world of every frame).

**Theorem 5.9 (Kripke Completeness).** If $\varphi$ is valid in all Kripke frames, then IPC $\vdash \varphi$.

**Example 5.10.** LEM $P \vee \neg P$ is not valid in the following Kripke frame:
- Worlds: $\{w_0, w_1\}$ with $w_0 \leq w_1$.
- $V(P) = \{w_1\}$ (P is true at $w_1$ but not at $w_0$).

Then:
- $w_0 \not\Vdash P$ (since $w_0 \notin V(P)$).
- $w_0 \not\Vdash \neg P$: we need all $w' \geq w_0$ to not force $P$; but $w_1 \geq w_0$ and $w_1 \Vdash P$. So $w_0 \not\Vdash \neg P$.
- Therefore $w_0 \not\Vdash P \vee \neg P$.

This gives a Kripke model where LEM fails — so LEM is not intuitionistically provable.

**Intuition:** Think of worlds as *stages of knowledge*. At $w_0$ (early stage), we don't yet know whether $P$ is true. At $w_1$ (later stage), we've acquired the information that $P$ is true. We cannot assert $P \vee \neg P$ at $w_0$ because we haven't yet determined which.

---

## 5. The Double-Negation Translation

Classical logic and intuitionistic logic are not completely separate: every classical theorem has an intuitionistic *translation* that is provable intuitionistically.

**Definition 5.11 (Gödel-Gentzen Translation).** Define $\varphi^{\circ}$ by:
- $P^{\circ} = \neg\neg P$ for atoms $P$
- $\bot^{\circ} = \bot$
- $(\varphi \wedge \psi)^{\circ} = \varphi^{\circ} \wedge \psi^{\circ}$
- $(\varphi \vee \psi)^{\circ} = \neg\neg(\varphi^{\circ} \vee \psi^{\circ})$
- $(\varphi \to \psi)^{\circ} = \varphi^{\circ} \to \psi^{\circ}$
- $(\neg\varphi)^{\circ} = \neg\varphi^{\circ}$

**Theorem 5.12.** Classical propositional logic proves $\varphi$ if and only if intuitionistic propositional logic proves $\varphi^{\circ}$.

*Consequence:* Classical logic is *conservative over* intuitionistic logic for $\neg\neg$-closed formulas. In particular, if a classical proof of $\neg\neg\varphi$ can be translated, and then (if $\varphi$ itself is $\neg\neg$-closed) the double negation can be absorbed.

**For arithmetic (Gödel 1933):** Every theorem of classical first-order arithmetic (PA) translates to a theorem of intuitionistic arithmetic (HA) via $^{\circ}$. Classical arithmetic is consistent relative to intuitionistic arithmetic.

---

## 6. The Principles of Constructive Mathematics

Different schools of constructive mathematics accept different "extra axioms" beyond pure intuitionistic logic.

### 6.1 Recursive Constructivism (Markov, Russian School)

Adds *Markov's principle*: if $P : \mathbb{N} \to \{0, 1\}$ is a decidable property and $\neg\neg\exists n, P(n) = 1$, then $\exists n, P(n) = 1$.

Informally: if it is impossible for a halting computer program to run forever, then it must terminate. This is provable classically (by LEM: either it terminates, or it runs forever — but the latter is impossible). Constructively, Markov's principle adds exactly this computational closure.

### 6.2 Brouwer's Intuitionism

Brouwer accepted *Choice Sequences* — not-yet-determined sequences of natural numbers — and the *Fan Theorem* (a continuity principle). This leads to theorems that contradict classical mathematics! E.g., "Every total real-valued function on $[0,1]$ is uniformly continuous" is true intuitionistically under Brouwer's principles but false classically.

### 6.3 Bishop's Constructivism

Errett Bishop's *Constructive Analysis* avoids both classical axioms and Brouwer's unusual principles. It is consistent with both classical and intuitionistic mathematics. Every theorem of Bishop's mathematics has a classical proof (add LEM) and is consistent with Markov's principle.

### 6.4 HoTT's Position

HoTT is built on intuitionistic type theory. It is *compatible* with LEM (LEM can be consistently added as an axiom in certain models) but does not *assume* it. The univalence axiom is independent of LEM. The interaction between univalence and classical logic is subtle: with both, one can prove things that neither alone proves.

Practically: when formalizing in Lean 4 or Agda without classical axioms, you are working constructively. In Lean 4 with `import Mathlib`, classical axioms (including LEM and the axiom of choice) are available. Knowing where each proof uses them is essential for understanding computational content.

---

## 7. Decidability

**Definition 5.13.** A proposition $P$ is *decidable* if $P \vee \neg P$ is provable (constructively). A predicate $P : A \to \text{Prop}$ is *decidably equal* if $\forall x, y : A, (x = y) \vee (x \neq y)$ is provable.

In HoTT, *sets* (h-level 0 types) often have decidable equality, but general types do not.

**Example 5.14.** $\mathbb{N}$ has decidable equality (we can compute whether two naturals are equal). $\mathbb{R}$ does not, constructively: given two real numbers given by infinite Cauchy sequences, we cannot in general decide in finite time whether they are equal.

**Example 5.15.** The type of natural numbers is a set (h-level 0) with decidable equality. The circle $S^1$ as a type in HoTT has identity types that are isomorphic to $\mathbb{Z}$ — decidability of equality for $S^1$ is a meaningful question with a specific answer (it is not decidable in the na¨ive sense, because different representations of the "same loop" may not be syntactically equal).

---

## Exercises

**5.1.** Verify the BHK interpretation for the following tautologies — that is, describe explicitly the function (program) proving each:
  - $P \to P$
  - $P \to Q \to P$
  - $(P \to Q \to R) \to (P \to Q) \to P \to R$

**5.2.** Show that the following are provable in IPC:
  - $P \to \neg\neg P$
  - $\neg\neg\neg P \leftrightarrow \neg P$
  - $\neg(P \vee Q) \leftrightarrow \neg P \wedge \neg Q$
  - $\neg P \vee \neg Q \to \neg(P \wedge Q)$ (but not the converse, intuitionistically)

**5.3.** Show that $\neg\neg(P \vee \neg P)$ is provable in IPC (even though $P \vee \neg P$ is not). What does this mean computationally?

**5.4.** Construct a Kripke model showing that $(P \to Q) \vee (Q \to P)$ is not intuitionistically valid. (This is a classical tautology but fails in some Kripke frames.)

**5.5.** Apply the Gödel-Gentzen translation to the formula $P \vee \neg P$ and write out $\varphi^{\circ}$ explicitly. Verify that $\varphi^{\circ}$ is intuitionistically provable.

**5.6.** Explain the disjunction property (Theorem 5.5) in terms of the BHK interpretation. Why should a BHK proof of $\varphi \vee \psi$ tell us which disjunct holds?

**5.7.** The *Peirce's law* $(((P \to Q) \to P) \to P)$ is a classical tautology. Show:
  - It is equivalent to LEM over IPC.
  - Find a Kripke frame where it fails.

**5.8 (Research).** Look up and describe the notion of *realizability* (Kleene 1945). How does realizability formalize the BHK interpretation using actual Turing machines or recursive functions? What is the realizability interpretation of the axiom of choice?

**5.9.** In Lean 4 without importing any classical axioms, try to prove `P ∨ ¬P` for an arbitrary proposition `P`. What error do you get? Now import `Classical` and prove it using `Classical.em`. What does this tell you about Lean 4's foundational assumptions?

**5.10 (Conceptual).** Bishop says: "Meaningful distinctions deserve to be maintained." Explain what distinction is lost when we use proof by contradiction to prove $\exists n, P(n)$ rather than directly exhibiting the $n$. Give a mathematical example where the classical proof gives existence but no algorithm.

---

## See Also

**In chapters/:**
- `ch04-proof-theory` — Prerequisite. The proof theory of IPC (sequent calculus, natural deduction, normalization) provides the syntactic background for the semantic completeness results in ch05.
- `ch06-curry-howard` — The computational content of intuitionistic logic: every IPC proof is a program; every IPC theorem is an inhabited type. The BHK interpretation introduced informally in ch05 is made precise by the Curry-Howard isomorphism.
- `ch08-dependent-types` — Dependent type theory extends IPC to predicate logic: `Π(x:A), P x` is the dependent counterpart of `∀x:A. P(x)`, and `Σ(x:A), P x` is the dependent counterpart of `∃x:A. P(x)`. The BHK clause for ∀ becomes the formation rule for Π-types.
- `ch17-h-levels` — The distinction between `Σ(x:A), P x` (proof-relevant existence) and `‖Σ(x:A), P x‖` (mere existence) is the HoTT refinement of the constructive/classical distinction from ch05. Propositional truncation `‖–‖` is the formal device for moving from a proof-relevant to a proof-irrelevant statement.

**In book/:**
- `book/unit-02-logic-and-computation/ch05-intuitionistic-logic/` — Extended narrative treatment emphasizing the mathematical and philosophical significance of the rejection of LEM. Includes extended discussion of the BHK interpretation, Markov's principle, and the constructivist tradition (Brouwer, Bishop, Martin-Löf).

**In demos/:**
- `demos/demo_bhk.py` — The BHK interpretation made computational: enter a proposition; compute its BHK interpretation; enter a proof term; verify it satisfies the interpretation.
- `demos/demo_kripke.py` — Kripke models for intuitionistic propositional logic. Demonstrates which classical tautologies fail intuitionistically by providing Kripke countermodels.

**The key principles of IPC vs. classical logic:**
| Principle | Classical | Intuitionistic |
|---|---|---|
| LEM `P ∨ ¬P` | Axiom | Not provable (without K or dneg) |
| DNE `¬¬P → P` | Derivable | Not provable |
| Peirce `((P→Q)→P)→P` | Derivable | Not provable |
| De Morgan `¬(P∧Q) → ¬P∨¬Q` | Derivable | Not provable |
| EM for ℕ `Π(n:ℕ), P n ∨ ¬P n` | Derivable | Derivable if P is decidable |
