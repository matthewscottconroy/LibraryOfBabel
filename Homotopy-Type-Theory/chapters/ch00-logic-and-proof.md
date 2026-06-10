# Chapter 0: Logic and the Art of Proof

## Introduction

Mathematics is the art of drawing conclusions from assumptions with absolute certainty. The engine of this art is logic — a formal system that makes precise what "follows from" means. Before we can study type theory, before we can even state what homotopy type theory is, we need a solid command of how mathematical reasoning works, why it works, and where its limits lie.

This chapter is not merely an introduction to logic as a subject. It is training in a discipline: the discipline of writing proofs. Every subsequent chapter in this curriculum will demand that you construct arguments, and those arguments must be correct — not just plausible, not just convincing, but *valid* by standards that a machine could verify. This is not pedantry. When we arrive at formal proof assistants in Phase 6, we will be asking exactly that: that our arguments be machine-checkable. The training here is the foundation.

A warning at the outset: mathematical proof is not the same as argument, persuasion, or evidence. A proof is a *derivation* — a finite sequence of steps, each one following from what came before by an agreed rule. Learning to write proofs is learning to be honest about every single step.

---

## 1. Propositional Logic

### 1.1 Syntax: The Language of Propositions

Propositional logic begins with the simplest possible things we might assert: *atomic propositions*, which we denote by letters $P, Q, R, \ldots$ From these, we build more complex propositions using *logical connectives*.

**Definition 0.1 (Propositional Formula).** The set of *well-formed formulas* (wff) of propositional logic is defined inductively:
- Every atomic proposition $P$ is a wff.
- If $\varphi$ is a wff, then $\neg \varphi$ (negation, "not $\varphi$") is a wff.
- If $\varphi$ and $\psi$ are wffs, then $(\varphi \wedge \psi)$, $(\varphi \vee \psi)$, $(\varphi \to \psi)$, and $(\varphi \leftrightarrow \psi)$ are wffs.
- Nothing else is a wff.

We read these connectives as: $\wedge$ ("and"), $\vee$ ("or"), $\to$ ("implies" or "if...then"), $\leftrightarrow$ ("if and only if"). We adopt the usual precedence conventions: $\neg$ binds most tightly, then $\wedge$, then $\vee$, then $\to$, then $\leftrightarrow$. So $P \vee Q \to R$ means $(P \vee Q) \to R$.

The structural definition above is an *inductive definition*: we specify base cases (atoms) and construction steps (connectives). This pattern recurs throughout mathematics and type theory. When we define something inductively, we can prove properties of it by *structural induction* — a technique we will see shortly.

### 1.2 Semantics: Truth and Valuation

Syntax tells us what counts as a formula. Semantics tells us what formulas *mean*. In propositional logic, meaning is given by *truth values*.

**Definition 0.2 (Valuation).** A *valuation* (or *interpretation*) is a function $v : \{\text{atoms}\} \to \{\textbf{T}, \textbf{F}\}$ assigning a truth value to each atomic proposition.

A valuation extends uniquely to all formulas by the *truth tables* for the connectives:

| $\varphi$ | $\psi$ | $\neg\varphi$ | $\varphi \wedge \psi$ | $\varphi \vee \psi$ | $\varphi \to \psi$ | $\varphi \leftrightarrow \psi$ |
|---|---|---|---|---|---|---|
| T | T | F | T | T | T | T |
| T | F | F | F | T | F | F |
| F | T | T | F | T | T | F |
| F | F | T | F | F | T | T |

The only row where $\varphi \to \psi$ is false is when $\varphi$ is true and $\psi$ is false. This matches the everyday intuition that a conditional promise "if it rains, I will bring an umbrella" is only broken if it rains and you fail to bring an umbrella.

**Definition 0.3.** A formula $\varphi$ is:
- *Satisfiable* if there exists a valuation $v$ with $v(\varphi) = \textbf{T}$.
- A *tautology* (or *logically valid*) if $v(\varphi) = \textbf{T}$ for every valuation $v$.
- A *contradiction* if $v(\varphi) = \textbf{F}$ for every valuation $v$.

**Example 0.4.** The formula $P \vee \neg P$ is a tautology (called the *law of excluded middle*, or LEM). The formula $P \wedge \neg P$ is a contradiction. The formula $P \vee Q$ is satisfiable but not a tautology.

**Definition 0.5 (Semantic Entailment).** A set of formulas $\Gamma$ *semantically entails* $\varphi$ (written $\Gamma \models \varphi$) if every valuation making every formula in $\Gamma$ true also makes $\varphi$ true. We write $\models \varphi$ for $\emptyset \models \varphi$ (i.e., $\varphi$ is a tautology).

**Example 0.6.** We have $\{P, P \to Q\} \models Q$: if $P$ is true and $P \to Q$ is true, then $Q$ must be true. This is the rule *modus ponens*.

---

## 2. Proof Techniques

With semantics in hand, we know what we want to prove: that certain formulas are tautologies, or that certain premises entail certain conclusions. Now we turn to the *techniques* of proof.

### 2.1 Direct Proof

A *direct proof* of $P \to Q$ assumes $P$ and derives $Q$ step by step, citing a justification at each step.

**Example 0.7.** Prove: for all integers $n$, if $n$ is even then $n^2$ is even.

*Proof.* Assume $n$ is even. Then there exists an integer $k$ such that $n = 2k$. Therefore $n^2 = (2k)^2 = 4k^2 = 2(2k^2)$. Since $2k^2$ is an integer, $n^2$ is even. $\square$

Every step is justified: we used the definition of "even," basic arithmetic, and closure of the integers under multiplication.

### 2.2 Proof by Contrapositive

The formula $(\varphi \to \psi) \leftrightarrow (\neg\psi \to \neg\varphi)$ is a tautology. To prove $\varphi \to \psi$, it therefore suffices to prove $\neg\psi \to \neg\varphi$ — the *contrapositive*.

**Example 0.8.** Prove: for all integers $n$, if $n^2$ is odd then $n$ is odd.

*Proof.* We prove the contrapositive: if $n$ is even, then $n^2$ is even. This is exactly Example 0.7. $\square$

### 2.3 Proof by Contradiction

To prove $\varphi$, assume $\neg\varphi$ and derive a contradiction (any formula of the form $\psi \wedge \neg\psi$). A contradiction cannot be true, so $\neg\varphi$ must be false, i.e., $\varphi$ must be true.

**Example 0.9.** Prove: $\sqrt{2}$ is irrational.

*Proof.* Suppose for contradiction that $\sqrt{2} = p/q$ where $p, q$ are integers with no common factor (we can always reduce a fraction). Then $2 = p^2/q^2$, so $p^2 = 2q^2$. Thus $p^2$ is even, and by Example 0.8, $p$ is even. Write $p = 2m$. Then $4m^2 = 2q^2$, so $q^2 = 2m^2$ is even, and $q$ is even. But then $p$ and $q$ share the common factor 2, contradicting our assumption. $\square$

**Remark 0.10 (Important).** Proof by contradiction is a *classical* technique: it invokes the law of excluded middle ($\varphi \vee \neg\varphi$) to conclude that since $\neg\varphi$ leads to contradiction, $\varphi$ holds. Constructive mathematics (which HoTT is built on) rejects this in general — to prove $\varphi$ constructively, you must exhibit a *construction* of $\varphi$, not merely rule out $\neg\varphi$. We will return to this in Chapter 5.

### 2.4 Proof by Cases

If $\varphi \vee \psi$ holds and you can prove $\chi$ from $\varphi$ and also from $\psi$, then $\chi$ holds. This is *disjunction elimination*.

**Example 0.11.** Prove: for all integers $n$, $n^2 + n$ is even.

*Proof.* Either $n$ is even or $n$ is odd (these two cases are exhaustive).

*Case 1:* $n$ is even. Then $n = 2k$, so $n^2 + n = 4k^2 + 2k = 2(2k^2 + k)$, which is even.

*Case 2:* $n$ is odd. Then $n = 2k+1$, so $n^2 + n = (2k+1)^2 + (2k+1) = 4k^2 + 4k + 1 + 2k + 1 = 4k^2 + 6k + 2 = 2(2k^2 + 3k + 1)$, which is even.

In both cases, $n^2 + n$ is even. $\square$

---

## 3. Mathematical Induction

One of the most powerful proof techniques — and the one most directly connected to type theory — is induction.

### 3.1 Weak Induction

**Theorem 0.12 (Principle of Mathematical Induction).** Let $P(n)$ be a property of natural numbers. If:
1. *Base case:* $P(0)$ holds.
2. *Inductive step:* For all $n \in \mathbb{N}$, $P(n) \to P(n+1)$.

Then $P(n)$ holds for all $n \in \mathbb{N}$.

The inductive step says: from the *induction hypothesis* $P(n)$ (which we may assume), derive $P(n+1)$.

**Example 0.13.** Prove: $\sum_{k=0}^{n} k = \frac{n(n+1)}{2}$ for all $n \geq 0$.

*Proof.* By induction on $n$.

*Base case ($n = 0$):* $\sum_{k=0}^{0} k = 0 = \frac{0 \cdot 1}{2}$. ✓

*Inductive step:* Assume $\sum_{k=0}^{n} k = \frac{n(n+1)}{2}$ (induction hypothesis). Then:
$$\sum_{k=0}^{n+1} k = \left(\sum_{k=0}^{n} k\right) + (n+1) = \frac{n(n+1)}{2} + (n+1) = (n+1)\left(\frac{n}{2} + 1\right) = \frac{(n+1)(n+2)}{2}.$$
This is the desired formula with $n+1$ in place of $n$. $\square$

### 3.2 Strong Induction

**Theorem 0.14 (Strong Induction).** Let $P(n)$ be a property of natural numbers. If for all $n$, $(\forall k < n, P(k)) \to P(n)$, then $P(n)$ holds for all $n$.

In strong induction, the inductive hypothesis gives you $P(k)$ for *all* $k < n$, not just $P(n-1)$.

**Example 0.15.** Prove: every integer $n \geq 2$ has a prime factorization.

*Proof.* By strong induction. Let $n \geq 2$. Assume that every integer $m$ with $2 \leq m < n$ has a prime factorization. If $n$ is prime, it is its own factorization. If $n$ is not prime, then $n = ab$ for some $2 \leq a, b < n$. By the induction hypothesis, both $a$ and $b$ have prime factorizations. Concatenating them gives a prime factorization of $n$. $\square$

### 3.3 Structural Induction

When we define a set *inductively* (as we did with propositional formulas in Definition 0.1), we can prove properties of all its members by induction on the *construction*.

**Theorem 0.16 (Structural Induction for Formulas).** Let $P(\varphi)$ be a property of propositional formulas. If:
1. $P(p)$ holds for every atom $p$.
2. If $P(\varphi)$, then $P(\neg\varphi)$.
3. If $P(\varphi)$ and $P(\psi)$, then $P(\varphi \star \psi)$ for $\star \in \{\wedge, \vee, \to, \leftrightarrow\}$.

Then $P(\varphi)$ holds for all formulas $\varphi$.

**Example 0.17.** Prove: every propositional formula has an equal number of left and right parentheses.

*Proof.* By structural induction. Let $L(\varphi)$ and $R(\varphi)$ denote the number of left and right parentheses in $\varphi$.

*Atoms:* $L(p) = R(p) = 0$. ✓

*Negation:* $L(\neg\varphi) = L(\varphi)$ and $R(\neg\varphi) = R(\varphi)$. By hypothesis $L(\varphi) = R(\varphi)$, so $L(\neg\varphi) = R(\neg\varphi)$. ✓

*Binary connectives:* $(\varphi \star \psi)$ adds one left and one right parenthesis. By hypothesis $L(\varphi) = R(\varphi)$ and $L(\psi) = R(\psi)$, so $L(\varphi \star \psi) = 1 + L(\varphi) + L(\psi) = 1 + R(\varphi) + R(\psi) = R(\varphi \star \psi)$. ✓ $\square$

### 3.4 Well-Founded Induction

The most general form of induction works over any *well-founded relation* — a relation with no infinite descending chain.

**Definition 0.18.** A relation $<$ on a set $X$ is *well-founded* if there is no infinite descending sequence $x_0 > x_1 > x_2 > \cdots$

**Theorem 0.19 (Well-Founded Induction).** Let $<$ be a well-founded relation on $X$, and let $P : X \to \{\text{true, false}\}$. If for all $x \in X$, $(\forall y < x, P(y)) \to P(x)$, then $P(x)$ for all $x \in X$.

The natural numbers with the usual ordering, the set of trees ordered by subtree inclusion, and the set of well-formed formulas ordered by subformula inclusion are all well-founded. In type theory, *all recursive functions must decrease on a well-founded measure* — this is how proof assistants ensure termination.

---

## 4. Predicate Logic

Propositional logic reasons about statements as atomic units. Predicate logic (first-order logic) allows us to talk about *objects* and their *properties*.

**Definition 0.20 (First-Order Language).** A first-order language $\mathcal{L}$ consists of:
- *Constants*: $c_1, c_2, \ldots$ (names for specific objects)
- *Function symbols*: $f, g, \ldots$ each with an arity $n \geq 1$
- *Predicate symbols*: $P, Q, R, \ldots$ each with an arity $n \geq 0$ (0-ary predicates are propositions)
- *Variables*: $x, y, z, \ldots$
- *Quantifiers*: $\forall$ ("for all"), $\exists$ ("there exists")
- *Connectives*: as in propositional logic

**Terms** are built from variables and constants using function symbols. **Formulas** are built from atomic formulas (predicate symbols applied to terms) using connectives and quantifiers.

**Example 0.21.** In the language of arithmetic: constants $0, 1$; function symbols $+, \times$ (arity 2), $S$ (arity 1, successor); predicate $=$ (arity 2).

The formula $\forall x\, \exists y\, (x = y + y)$ says "every number is the double of some number" — which is false in $\mathbb{N}$ (1 is not the double of anything).

**Definition 0.22 (Free and Bound Variables).** An occurrence of a variable $x$ in a formula $\varphi$ is *bound* if it is in the scope of a quantifier $\forall x$ or $\exists x$; otherwise it is *free*. A formula with no free variables is called a *sentence*.

**Proof rules for quantifiers** (natural deduction style):
- **Universal introduction** ($\forall$-I): If you can prove $P(x)$ for an *arbitrary* variable $x$ (one with no special assumptions about it), you may conclude $\forall x, P(x)$.
- **Universal elimination** ($\forall$-E): From $\forall x, P(x)$, you may conclude $P(t)$ for any term $t$.
- **Existential introduction** ($\exists$-I): From $P(t)$, you may conclude $\exists x, P(x)$.
- **Existential elimination** ($\exists$-E): From $\exists x, P(x)$ and a proof of $Q$ from $P(c)$ for an *arbitrary fresh constant* $c$, you may conclude $Q$.

**Example 0.23.** Prove: $(\forall x, P(x)) \to (\exists x, P(x))$ — provided the domain is nonempty.

*Proof.* Assume $\forall x, P(x)$. Let $a$ be any element of the domain. By $\forall$-E, $P(a)$. By $\exists$-I, $\exists x, P(x)$. $\square$

Note: this step requires the domain to be nonempty — we need *some* element $a$ to instantiate. This is an important subtlety.

---

## 5. Common Pitfalls and Standards

**Pitfall 1: Claiming without justification.** Every step must be justified by a definition, an axiom, a previously proved result, or a logical rule. "Clearly" and "obviously" are warning signs.

**Pitfall 2: Confusing $\Rightarrow$ with $\Leftrightarrow$.** To prove $P \leftrightarrow Q$ you must prove both $P \to Q$ and $Q \to P$.

**Pitfall 3: Assuming what you want to prove.** Circular reasoning invalidates a proof entirely.

**Pitfall 4: Incorrect induction.** The base case and inductive step are both required. Dropping either produces no valid proof. Check that the inductive step applies at the base: sometimes induction starts at $n = 1$ or $n = 2$, not $n = 0$.

**Pitfall 5: Quantifier order.** $\forall x\, \exists y, P(x, y)$ and $\exists y\, \forall x, P(x, y)$ say very different things. The first says "for every $x$ there is (possibly a different) $y$"; the second says "there is a single $y$ that works for all $x$."

**Standard 0.24 (What a Complete Proof Must Include):**
1. A clear statement of what is being proved.
2. Identification of the proof strategy (direct, contradiction, induction, etc.).
3. Every hypothesis stated explicitly when first used.
4. Every step justified.
5. A clear end (the $\square$ or QED symbol, with the statement of what was concluded).

---

## 6. Connection to What Comes Next

The patterns in this chapter — inductive definition, structural induction, proof by the rules of natural deduction — are not merely techniques. In type theory, they become the *substance* of the formal system.

When we define a type $A$ in Martin-Löf type theory, we define it *inductively*: we specify what the elements of $A$ are by giving *constructors*. To prove something about all elements of $A$, we use an *eliminator*, which is precisely structural induction. The quantifiers $\forall$ and $\exists$ become the dependent types $\Pi$ and $\Sigma$.

The discipline of writing rigorous, step-by-step proofs — internalized here — becomes the discipline of writing *terms* in a proof assistant.

---

## Exercises

**0.1.** Using truth tables, verify that the following are tautologies:
  - $P \to (Q \to P)$
  - $(P \to (Q \to R)) \to ((P \to Q) \to (P \to R))$
  - $(\neg P \to \neg Q) \to (Q \to P)$
  
  These are the axioms of classical propositional logic. Note that the third one is *not* an axiom of intuitionistic logic.

**0.2.** Determine whether each formula is a tautology, satisfiable but not a tautology, or a contradiction. Justify with a truth table or counterexample.
  - $(P \to Q) \to (\neg Q \to \neg P)$
  - $(P \to Q) \to (Q \to P)$
  - $((P \to Q) \to P) \to P$ (Peirce's law)

**0.3.** Prove by induction: $\sum_{k=1}^{n} k^2 = \frac{n(n+1)(2n+1)}{6}$.

**0.4.** Prove by strong induction: every positive integer $n$ can be written in *binary* — as a sum of distinct powers of 2.

**0.5.** A *binary tree* is either a *leaf* or a *node* with two binary tree children. Prove by structural induction that a binary tree with $n$ leaves has exactly $n - 1$ nodes.

**0.6.** For each of the following proofs, identify the flaw:
  - "Proof that all positive integers are equal: By strong induction. Assume all positive integers less than $n$ are equal. Then in particular $n-1$ and all numbers less than $n-1$ are equal, so by transitivity, $n-1 = 1$, so $n = 2$... " (Where does this break?)
  - "Proof that $1 = 2$: Let $a = b = 1$. Then $a^2 = ab$, so $a^2 - b^2 = ab - b^2$, so $(a+b)(a-b) = b(a-b)$, so $a + b = b$, so $2 = 1$."

**0.7 (Challenge).** Prove the *compactness theorem* for propositional logic: if every finite subset of an infinite set $\Gamma$ of formulas is satisfiable, then $\Gamma$ itself is satisfiable. (*Hint:* enumerate the atoms; build a valuation one atom at a time, always preserving satisfiability of every finite subset.)

**0.8.** Write a fully rigorous proof, in the style demanded by this chapter, that the square root of 3 is irrational.

**0.9.** State and prove the *induction principle for lists*: if $P$ holds for the empty list and if $P$ holds for a list $\ell$ whenever it holds for its tail, then $P$ holds for all finite lists.

**0.10 (Reflection).** Proof by contradiction uses the law of excluded middle. Find a proof in this chapter that uses contradiction and rewrite it as a direct proof. Can you always do this?
