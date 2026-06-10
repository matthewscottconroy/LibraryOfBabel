# 3.2 Structural and Well-Founded Induction

## Beyond Numbers

Mathematical induction on $\mathbb{N}$ is powerful, but many mathematical objects are not numbers — they are trees, formulas, lists, terms, derivations. For these, we need a more general form of induction: one that follows the *structure* of the objects being defined.

The key insight: whenever you have an *inductively defined* set (one whose elements are built by rules from smaller elements), you get an induction principle for free. This is *structural induction*, and it is the universal form of induction throughout mathematics and computer science.

## Structural Induction

An *inductively defined set* $X$ is specified by:
- *Base cases:* elements that are simply given (atoms).
- *Constructors:* operations that build new elements from existing ones.

The corresponding induction principle: to prove a property $P$ of all elements of $X$, prove:
- $P$ holds for each base case.
- If $P$ holds for each input to a constructor, then $P$ holds for the output.

**Example: Structural induction on propositional formulas.**

Recall the inductive definition of propositional formulas from Section 1.1:
- Base: atoms $P, Q, R, \ldots$
- Negation: if $\varphi$ is a formula, so is $\neg\varphi$.
- Binary connectives: if $\varphi$ and $\psi$ are formulas, so are $\varphi \wedge \psi$, $\varphi \vee \psi$, $\varphi \to \psi$, $\varphi \leftrightarrow \psi$.

**Theorem.** Every propositional formula has balanced parentheses (equal numbers of left and right parentheses).

*Proof by structural induction.*

Let $P(\varphi)$ = "$\varphi$ has equal numbers of left and right parentheses."

*Base case (atoms):* Atoms have no parentheses: $0 = 0$. ✓

*Negation case:* $\neg\varphi$ has the same parentheses as $\varphi$. If $P(\varphi)$ holds (equal counts in $\varphi$), then it holds in $\neg\varphi$. ✓

*Binary connective case:* $(\varphi \star \psi)$ adds one left and one right parenthesis. If $P(\varphi)$ and $P(\psi)$ hold (equal counts in each), then the counts in $(\varphi \star \psi)$ are $(1 + L(\varphi) + L(\psi))$ left and $(1 + R(\varphi) + R(\psi))$ right, which are equal. ✓

By structural induction, $P(\varphi)$ holds for all formulas $\varphi$. $\square$

**Example: Structural induction on lists.**

A *list* over a type $A$ is defined inductively:
- Base: $\text{nil}$ (the empty list).
- Constructor: if $a : A$ and $\ell$ is a list, then $\text{cons}(a, \ell)$ is a list (prepend $a$ to $\ell$).

Induction on lists: to prove $P$ for all lists, prove:
- $P(\text{nil})$
- For all $a : A$ and lists $\ell$: if $P(\ell)$ then $P(\text{cons}(a, \ell))$.

**Theorem.** The concatenation of two lists $\ell_1$ and $\ell_2$ has length $\text{len}(\ell_1) + \text{len}(\ell_2)$.

*Proof by structural induction on $\ell_1$.*

*Base case ($\ell_1 = \text{nil}$):* $\text{len}(\text{nil} \,{++}\, \ell_2) = \text{len}(\ell_2) = 0 + \text{len}(\ell_2)$. ✓

*Inductive case ($\ell_1 = \text{cons}(a, \ell)$):* Assume $\text{len}(\ell \,{++}\, \ell_2) = \text{len}(\ell) + \text{len}(\ell_2)$.
$$\text{len}(\text{cons}(a,\ell) \,{++}\, \ell_2) = 1 + \text{len}(\ell \,{++}\, \ell_2) = 1 + \text{len}(\ell) + \text{len}(\ell_2) = \text{len}(\text{cons}(a,\ell)) + \text{len}(\ell_2)$$
$\square$

**Example: Structural induction on binary trees.**

A *binary tree* is defined inductively:
- Base: $\text{leaf}$ (a leaf node).
- Constructor: $\text{node}(T_1, T_2)$ (a node with left subtree $T_1$ and right subtree $T_2$).

**Theorem.** Every binary tree with $n$ leaves has $n - 1$ internal nodes.

*Proof by structural induction.*

Let $\ell(T)$ = number of leaves, $i(T)$ = number of internal nodes.

*Base case:* $\ell(\text{leaf}) = 1$, $i(\text{leaf}) = 0$. Claim: $i = \ell - 1$, i.e., $0 = 1 - 1$. ✓

*Inductive case:* Let $T = \text{node}(T_1, T_2)$. By hypothesis: $i(T_1) = \ell(T_1) - 1$ and $i(T_2) = \ell(T_2) - 1$.
$$i(T) = i(T_1) + i(T_2) + 1 = (\ell(T_1) - 1) + (\ell(T_2) - 1) + 1 = \ell(T_1) + \ell(T_2) - 1 = \ell(T) - 1 \quad\square$$

## Well-Founded Induction

The most general form of induction works over any *well-founded relation*.

**Definition.** A binary relation $\prec$ on a set $X$ is *well-founded* if there is no infinite descending sequence:
$$x_0 \succ x_1 \succ x_2 \succ \cdots$$

Equivalently: every non-empty subset of $X$ has a $\prec$-minimal element.

**Principle of Well-Founded Induction.** Let $\prec$ be a well-founded relation on $X$ and $P : X \to \{\text{true, false}\}$. If for all $x \in X$:
$$\left(\forall y \prec x,\, P(y)\right) \to P(x)$$
then $P(x)$ holds for all $x \in X$.

This generalizes everything:
- Weak induction: $\prec$ = "less than" on $\mathbb{N}$, well-founded.
- Strong induction: same relation, but we're explicit about using all $y < n$.
- Structural induction: $\prec$ = "proper substructure" (e.g., proper subformula, proper subtree), well-founded.

**Why well-foundedness matters for termination.** In a proof assistant or functional programming language, *every function must terminate*. The standard way to guarantee termination is to show that each recursive call decreases on a well-founded measure. Well-founded induction is the theoretical foundation for this:

- If each recursive call goes to a smaller element under a well-founded $\prec$, then no call chain can go on forever (no infinite descending sequence in $\prec$).

**Example: Euclid's algorithm terminates.**

The Euclidean algorithm computes $\gcd(a, b)$ by the rule $\gcd(a, b) = \gcd(b, a \bmod b)$ (with $\gcd(a, 0) = a$). Why does it terminate?

Measure: the second argument $b$. Each recursive call passes $a \bmod b < b$ as the new second argument. The sequence of second arguments is a strictly decreasing sequence of non-negative integers, which must eventually reach 0. $\square$

The well-founded order here is $<$ on $\mathbb{N}_{\geq 0}$.

**Example: Confluence of reduction.**

In Section 2 of Chapter 2, we proved that word reduction in a free group is confluent (different orders of cancellation give the same reduced word). The proof used well-founded induction on the *length* of words: every reduction step strictly decreases the length, so the process terminates.

## Structural Induction in Type Theory

In Martin-Löf Type Theory (Chapter 9), *every* type is inductively defined. The induction principle for a type is its *elimination rule* (or *recursor*).

- For $\mathbb{N}$: the elimination rule is induction / recursion.
- For lists: the elimination rule is the structural induction above.
- For binary trees: similarly.
- For the identity type $a = b$: the elimination rule is *path induction*, which says that to prove $P$ for all proofs $p : a = b$ (all paths), it suffices to prove $P(\text{refl}_a)$ for all $a$.

This last example is crucial for HoTT. The induction principle for identity types captures the geometry of path spaces:

```
J-rule (path induction): 
  To prove P(a, b, p) for all a b : A and p : a = b,
  it suffices to prove P(a, a, refl_a) for all a : A.
```

This says: all proofs of equality can be reduced to reflexivity proofs, in the sense that any property of equalities that holds for the trivial equality holds for all equalities.

The J-rule is to identity types what mathematical induction is to natural numbers: the fundamental tool for reasoning about all elements of an inductively defined type.

## Why Well-Foundedness Cannot Be Dropped

If we allow a non-well-founded relation, the "induction principle" becomes unsound. Example:

Suppose $\prec$ is defined on $\mathbb{Z}$ by $n \prec n - 1$ for all $n$ (each integer precedes its predecessor). This is not well-founded: $0 \succ -1 \succ -2 \succ \cdots$.

The "induction principle" for $\prec$ would say: to prove $P(n)$ for all $n \in \mathbb{Z}$, assuming $P(n-1)$ implies $P(n)$... but wait, this is just saying: if the property propagates forward, it holds everywhere. And that's false: "n is positive" propagates forward (if $n-1$ is positive, so is $n$), but is not true for all integers.

Well-foundedness is the condition that prevents these false applications.

In proof assistants, the *termination checker* ensures that all recursion descends on a well-founded measure. Accepting a non-terminating function would make the type theory unsound — you could prove false propositions by building non-terminating "proofs."
