# 4.1 Predicate Logic: Quantifiers and Their Rules

## From Propositions to Predicates

Propositional logic talks about whole statements as atomic units. It can handle "if it rains then the ground is wet" by treating both clauses as atoms. But it cannot analyze "for every prime $p$, $p^2 - 1$ is divisible by 24 for $p > 3$" — here the truth of the statement depends on the *internal structure* of what $p$ is.

*Predicate logic* (first-order logic) adds the ability to talk about:
- *Objects* in some domain
- *Properties* of those objects (predicates)
- *Relations* between objects
- *Functions* from objects to objects
- *Quantifiers* ranging over all or some objects

This is the language in which essentially all of mathematics is written.

## The Language of First-Order Logic

**Definition (First-Order Language).** A first-order language $\mathcal{L}$ specifies:
- A set of *constant symbols* $c_1, c_2, \ldots$ (naming specific objects)
- A set of *function symbols* $f, g, \ldots$ each with an arity $n \geq 1$
- A set of *predicate symbols* $P, Q, R, \ldots$ each with an arity $n \geq 0$
- *Variables* $x, y, z, \ldots$ (ranging over objects in the domain)
- *Logical symbols*: $\neg, \wedge, \vee, \to, \leftrightarrow, \forall, \exists, =$
- *Punctuation*: parentheses and commas

**Terms** (expressions denoting objects):
- Every variable is a term.
- Every constant symbol is a term.
- If $f$ is an $n$-ary function symbol and $t_1, \ldots, t_n$ are terms, then $f(t_1, \ldots, t_n)$ is a term.

**Atomic formulas** (simplest meaningful claims):
- $t_1 = t_2$ (equality of terms)
- $P(t_1, \ldots, t_n)$ for an $n$-ary predicate $P$ and terms $t_i$

**Formulas** (built from atomic formulas using connectives and quantifiers):
- Every atomic formula is a formula.
- If $\varphi$ is a formula, so is $\neg\varphi$.
- If $\varphi, \psi$ are formulas, so are $\varphi \wedge \psi$, $\varphi \vee \psi$, $\varphi \to \psi$, $\varphi \leftrightarrow \psi$.
- If $\varphi$ is a formula and $x$ is a variable, so are $\forall x, \varphi$ and $\exists x, \varphi$.

**The language of arithmetic** $\mathcal{L}_{\text{arith}}$: constants $0, 1$; function symbols $+, \times$ (arity 2), $S$ (arity 1, successor); predicate $=$ (arity 2).

Example formulas:
- $\forall x, \exists y, y > x$ — "for every $x$ there's a larger $y$" (true in $\mathbb{N}$, false in $\{1, \ldots, 100\}$)
- $\exists x, \forall y, y \geq x$ — "there's a smallest element" (true in $\mathbb{N}$, false in $\mathbb{Z}$)
- $\forall x, (x > 1 \to \exists p, \text{prime}(p) \wedge p \mid x)$ — "every number $> 1$ has a prime factor"

## Free and Bound Variables

**Definition.** An occurrence of variable $x$ in a formula $\varphi$ is:
- *Bound* if it appears within the scope of a $\forall x$ or $\exists x$ quantifier.
- *Free* if it is not bound.

A formula with no free variables is a *sentence* (or *closed formula*). The truth of a sentence depends only on the interpretation (the domain and interpretation of function/predicate symbols), not on variable assignments.

**Examples:**
- $\forall x, P(x)$: the $x$ is bound. This is a sentence.
- $P(x)$: the $x$ is free. This is not a sentence — its truth depends on what $x$ denotes.
- $\forall x, (P(x) \to Q(x, y))$: $x$ is bound, $y$ is free. Not a sentence.
- $\exists x, (x = y)$: $x$ is bound, $y$ is free.

**Variable clash:** Care must be taken with substitution. The formula $\forall y, (x < y)$ has $x$ free. If we substitute $y$ for $x$ (naively), we get $\forall y, (y < y)$, which is a different meaning — the $y$ was *captured* by the quantifier. Proper substitution avoids capture by renaming bound variables if necessary.

## The Proof Rules for Quantifiers

Here are the natural deduction rules for the quantifiers. These are the formal rules that justify the steps mathematicians take when working with "for all" and "there exists."

### Universal Quantifier ($\forall$)

**$\forall$-Introduction:** 
$$\frac{\Gamma \vdash \varphi(x)}{\Gamma \vdash \forall x, \varphi(x)} \quad [x \text{ not free in } \Gamma]$$

If you can prove $\varphi(x)$ for an *arbitrary* variable $x$ (one that doesn't appear in any hypothesis), you may conclude $\forall x, \varphi(x)$.

In everyday proofs: "Let $x$ be an arbitrary [element of the domain]. ... [prove something about $x$]. Since $x$ was arbitrary, this holds for all $x$."

The condition "$x$ not free in $\Gamma$" is crucial: you must not have used any special assumptions about $x$. If your proof of $\varphi(x)$ assumed $x > 0$, you cannot conclude $\forall x, \varphi(x)$ — only $\forall x > 0, \varphi(x)$.

**$\forall$-Elimination (Instantiation):**
$$\frac{\Gamma \vdash \forall x, \varphi(x)}{\Gamma \vdash \varphi(t)} \quad [t \text{ free for } x \text{ in } \varphi]$$

From $\forall x, \varphi(x)$, instantiate $x$ by any term $t$ to get $\varphi(t)$.

In everyday proofs: "Since [property] holds for all $x$, it holds in particular for $t = [specific value]$."

### Existential Quantifier ($\exists$)

**$\exists$-Introduction (Witness):**
$$\frac{\Gamma \vdash \varphi(t)}{\Gamma \vdash \exists x, \varphi(x)}$$

If you can prove $\varphi(t)$ for some specific term $t$, you may conclude $\exists x, \varphi(x)$.

In everyday proofs: "Let $t = [specific term]$. Then [prove $\varphi(t)$]. Therefore there exists $x$ with $\varphi(x)$."

**$\exists$-Elimination:**
$$\frac{\Gamma \vdash \exists x, \varphi(x) \quad \Gamma, \varphi(c) \vdash \psi}{\Gamma \vdash \psi} \quad [c \text{ fresh, not in } \Gamma, \psi, \text{ or } \varphi]$$

From $\exists x, \varphi(x)$ and a proof that $\psi$ follows from $\varphi(c)$ for a *fresh constant* $c$, conclude $\psi$.

In everyday proofs: "Let $c$ be [an element satisfying the existential]. Then [prove $\psi$ using only that $c$ satisfies $\varphi$, not any other assumptions about $c$]."

The "fresh constant" condition prevents circular reasoning: $c$ represents the thing whose existence we assumed, and we must not assume anything extra about it.

## Common Proof Patterns

**Universal introduction in action:**

*Claim:* For all real $x$, $x^2 \geq 0$.

*Proof.* Let $x$ be an arbitrary real number. Then $x^2 = x \cdot x$. If $x \geq 0$, then $x^2 = x \cdot x \geq 0$. If $x < 0$, then $-x > 0$, so $x^2 = (-x)^2 = (-x)(-x) \geq 0$. In both cases $x^2 \geq 0$. Since $x$ was arbitrary, $\forall x \in \mathbb{R}, x^2 \geq 0$. $\square$

**Existential introduction in action:**

*Claim:* There exists a rational number between 1 and 2.

*Proof.* Let $r = 3/2$. Then $r = 3/2 = 1.5$, which is rational and satisfies $1 < 1.5 < 2$. Therefore there exists a rational number between 1 and 2. $\square$

**Existential elimination in action:**

*Claim:* If there exists an even prime, then there exists a prime less than 3.

*Proof.* Assume there exists an even prime $p$. Then $p = 2k$ for some integer $k$, and $p$ is prime. Since $p$ is prime, $p \geq 2$. Since $p$ is even, $p = 2$. But $2 < 3$. Therefore $p$ is a prime with $p < 3$. So there exists a prime less than 3. $\square$

## Quantifier Order Matters

The order of quantifiers drastically changes meaning.

$\forall x, \exists y, P(x, y)$: for each $x$, we can find a $y$ (possibly depending on $x$) satisfying $P$. 

$\exists y, \forall x, P(x, y)$: there is a single $y$ that works for *all* $x$ simultaneously.

**Example:** Let $P(x, y)$ = "$y > x$" over $\mathbb{N}$.

- $\forall x, \exists y, y > x$: TRUE. For each $x$, take $y = x + 1$.
- $\exists y, \forall x, y > x$: FALSE. There is no single $y$ larger than all natural numbers.

This distinction is crucial in analysis: 
- Continuity: $\forall \epsilon > 0, \exists \delta > 0, \forall x, |x - a| < \delta \to |f(x) - f(a)| < \epsilon$ (the $\delta$ can depend on $\epsilon$ and $a$).
- Uniform continuity: $\forall \epsilon > 0, \exists \delta > 0, \forall x, \forall a, |x - a| < \delta \to |f(x) - f(a)| < \epsilon$ (the $\delta$ works for all $a$).

Getting quantifier order right is essential.

## Connection to Dependent Types

In type theory (Chapter 8), quantifiers become dependent types:

- $\forall x : A, P(x)$ becomes the *dependent function type* (or *Pi-type*) $\Pi_{x:A} P(x)$. A proof of $\forall x, P(x)$ is a function that takes any $a : A$ and returns a proof of $P(a)$.

- $\exists x : A, P(x)$ becomes the *dependent pair type* (or *Sigma-type*) $\Sigma_{x:A} P(x)$. A proof of $\exists x, P(x)$ is a pair $(a, p)$ where $a : A$ and $p : P(a)$.

The $\forall$-introduction rule becomes $\lambda$-abstraction (function formation). The $\forall$-elimination rule becomes function application. The $\exists$-introduction rule becomes pairing. The $\exists$-elimination rule becomes the pair-eliminator (projections).

Every proof rule in predicate logic corresponds to a type-theoretic construct. This is the Curry-Howard correspondence (Chapter 6), and it is one of the central ideas of this curriculum.
