# 4.1 Extending to Dependent Types: Π and Σ

## The Limits of Simple Types

STLC handles propositional logic beautifully. But real mathematics requires predicate logic — statements that quantify over elements of a domain. "Every natural number $n$ has a successor" requires universal quantification over $\mathbb{N}$. "There exists a prime greater than 100" requires existential quantification.

In simple types, the type of a term doesn't depend on the *value* of another term. To handle predicate logic, we need types that can depend on terms.

**Dependent types** are types that depend on terms (values). They extend the STLC's function types and product types to allow this dependence.

## Pi Types: Dependent Functions

The universal statement $\forall x : A, P(x)$ corresponds to the **dependent function type** (also called Pi type or $\Pi$-type):

$$\Pi_{x:A} P(x) \quad \text{or} \quad (x : A) \to P(x)$$

A term $f : \Pi_{x:A} P(x)$ is a function that, given any $a : A$, returns a term $f(a) : P(a)$. The *type* of the output depends on the *value* of the input.

**Simple function types as a special case:** When $P$ doesn't depend on $x$ (when $P(x) = B$ for all $x$), the type $\Pi_{x:A} P(x)$ is just $A \to B$. So $\Pi$-types generalize ordinary function types.

**The typing rule for Pi types:**

$$\frac{\Gamma, x : A \vdash P(x)\ \text{type}}{\Gamma \vdash \Pi_{x:A} P(x)\ \text{type}} \quad (\Pi\text{-form})$$

$$\frac{\Gamma, x : A \vdash t : P(x)}{\Gamma \vdash \lambda x.\, t : \Pi_{x:A} P(x)} \quad (\Pi\text{-intro})$$

$$\frac{\Gamma \vdash f : \Pi_{x:A} P(x) \quad \Gamma \vdash a : A}{\Gamma \vdash f(a) : P(a)} \quad (\Pi\text{-elim})$$

**Examples:**
- $\text{id} : \Pi_{A:\mathcal{U}} A \to A$ is the *polymorphic identity function*: for any type $A$, it's the identity on $A$.
- $\text{Vec} : \Pi_{n:\mathbb{N}} \text{Type}$ maps each natural number to the type of vectors of that length.
- $\text{head} : \Pi_{n:\mathbb{N}} \text{Vec}(n+1) \to A$ is the head function, typed to only work on non-empty vectors.

## Sigma Types: Dependent Pairs

The existential statement $\exists x : A, P(x)$ corresponds to the **dependent pair type** (also called Sigma type or $\Sigma$-type):

$$\Sigma_{x:A} P(x)$$

A term $(a, p) : \Sigma_{x:A} P(x)$ consists of a first component $a : A$ and a second component $p : P(a)$. The *type* of the second component depends on the *value* of the first.

**Simple product types as a special case:** When $P$ doesn't depend on $x$, $\Sigma_{x:A} P(x)$ is just $A \times P$. So $\Sigma$-types generalize ordinary product types.

**The typing rule for Sigma types:**

$$\frac{\Gamma, x : A \vdash P(x)\ \text{type}}{\Gamma \vdash \Sigma_{x:A} P(x)\ \text{type}} \quad (\Sigma\text{-form})$$

$$\frac{\Gamma \vdash a : A \quad \Gamma \vdash p : P(a)}{\Gamma \vdash (a, p) : \Sigma_{x:A} P(x)} \quad (\Sigma\text{-intro})$$

$$\frac{\Gamma \vdash s : \Sigma_{x:A} P(x)}{\Gamma \vdash \pi_1(s) : A} \quad \frac{\Gamma \vdash s : \Sigma_{x:A} P(x)}{\Gamma \vdash \pi_2(s) : P(\pi_1(s))} \quad (\Sigma\text{-elim})$$

**Examples:**
- $\text{EvenNat} := \Sigma_{n:\mathbb{N}} \text{Even}(n)$ is the type of even natural numbers (pairs of a number and a proof of evenness).
- $\text{Sorted} := \Sigma_{l:\text{List}(\mathbb{N})} \text{IsSorted}(l)$ is the type of sorted lists.
- $\text{Prime} := \Sigma_{p:\mathbb{N}} \text{IsPrime}(p)$ is the type of prime numbers.

## The Quantifier Correspondence

| Predicate Logic | Dependent Type Theory |
|---|---|
| $\forall x : A, P(x)$ | $\Pi_{x:A} P(x)$ |
| $\exists x : A, P(x)$ | $\Sigma_{x:A} P(x)$ |
| Proof of $\forall x, P(x)$ | Function $f$ with $f(a) : P(a)$ |
| Proof of $\exists x, P(x)$ | Pair $(a, p)$ with $p : P(a)$ |
| Universal instantiation | Function application |
| Existential witness extraction | First projection |
| Existential proof extraction | Second projection |

This is the Curry-Howard correspondence extended to predicate logic.

## The Identity Type: The Key New Element

In STLC, equality of terms is *definitional*: $t = s$ iff they reduce to the same normal form. This is a meta-theoretic notion.

In dependent type theory, we need to *internalize* equality: to talk about equality as a mathematical object.

**The identity type** (also called the *equality type* or *propositional equality*):

$$\text{Id}_A(a, b) \quad \text{or} \quad a =_A b$$

This is a type. A term $p : a =_A b$ is a *proof* that $a$ and $b$ are equal.

**Introduction rule (reflexivity):**
$$\frac{\Gamma \vdash a : A}{\Gamma \vdash \text{refl}_a : a =_A a}$$

The only canonical way to construct an element of $a =_A b$ is when $a$ and $b$ are definitionally equal, in which case $\text{refl}_a : a =_A a$.

**Elimination rule (path induction / J):**
$$\frac{\Gamma \vdash p : a =_A b \quad \Gamma, x:A, y:A, q: x=_A y \vdash C(x,y,q)\, \text{type} \quad \Gamma, z:A \vdash d : C(z,z,\text{refl}_z)}{\Gamma \vdash J(p, d) : C(a,b,p)}$$

This says: to prove any property $C$ of equality proofs, it suffices to prove $C$ for the reflexivity proof (the "canonical" equality proof). Then $J$ applies this to any equality proof $p$.

**This is the crucial new element of dependent type theory.** The identity type turns "equality" from a logical relation into a *type*, and proofs of equality into *terms*. This is the foundation of the homotopy interpretation.

## The Homotopy Interpretation

In the homotopy interpretation:
- A type $A$ is a *topological space* (or $\infty$-groupoid).
- A term $a : A$ is a *point* in the space.
- An element $p : a =_A b$ is a *path* from $a$ to $b$.
- An element $h : p =_{a=b} q$ is a *homotopy* between paths $p$ and $q$.
- And so on: higher equality types correspond to higher homotopies.

This is not just a metaphor. There is a precise mathematical model: the *groupoid model* (Hofmann-Streicher 1994) showed that the identity type has non-trivial higher structure, and *simplicial sets* / *Kan complexes* give fully faithful models of HoTT (Voevodsky, Kapulkin-Lumsdaine 2012–2018).

The key theorem that makes this work:

**Theorem (Groupoid Laws).** For any type $A$ and elements $a, b, c : A$:
1. $\text{refl}_a : a = a$ (identity path)
2. If $p : a = b$, then $p^{-1} : b = a$ (inverse/reversal)
3. If $p : a = b$ and $q : b = c$, then $p \cdot q : a = c$ (composition/concatenation)
4. These satisfy: $p \cdot p^{-1} = \text{refl}_a$ and $p^{-1} \cdot p = \text{refl}_b$ and $(p \cdot q) \cdot r = p \cdot (q \cdot r)$ — all up to higher homotopy.

These groupoid laws are provable in MLTT! They're not extra axioms but consequences of path induction.

## The Univalence Axiom: A Glimpse

The Univalence Axiom (Voevodsky) extends the identity type to the *universe of types* $\mathcal{U}$:

$$\text{ua} : (A \simeq B) \to (A =_{\mathcal{U}} B)$$

An *equivalence* $A \simeq B$ is a bijection (more precisely, a function with contractible fibers). Univalence says: equivalent types are equal in the universe.

Under the homotopy interpretation: the universe $\mathcal{U}$ is a "classifying space for types," and a path $A =_{\mathcal{U}} B$ is a continuous deformation of the space $A$ into the space $B$ — which is exactly an equivalence (up to homotopy).

Univalence has the mathematical consequence we've been building toward: *isomorphic mathematical structures are literally equal*. Two constructions of the real numbers that are isomorphic give equal elements of $\mathcal{U}$.

This is the resolution of the identity problem from Chapter 1, promised throughout our analysis chapter (Chapter 3).

## Looking Forward

We've now seen the full arc of the correspondence:

| Classical ZFC | Constructive Type Theory | HoTT |
|---|---|---|
| Sets | Types | Types with homotopy structure |
| Propositions (true/false) | Types (propositions-as-types) | Types at h-level $-1$ |
| Functions (arbitrary) | Terms (always continuous) | Terms (with coherence) |
| Equality (in a set) | Identity type (in a type) | Path space |
| Isomorphism | Equivalence | Identity (by Univalence) |

Chapters 8–11 will develop the full type-theoretic machinery (STLC, System F, dependent types, MLTT). Chapters 16–18 will develop the HoTT-specific additions (identity types, h-levels, Univalence). By the end of the curriculum, the full picture will be in place.
