# Chapter 23: Cubical Type Theory — Computational HoTT

## Introduction

Cubical type theory is the answer to a question that troubled HoTT for years: does the univalence axiom have computational content? In standard HoTT (as in the HoTT Book), univalence is an axiom — a postulate with no reduction rule. A proof term that uses univalence cannot be evaluated to a normal form. This breaks *canonicity*: the property that closed terms of type $\mathbb{N}$ reduce to numerals.

The Cohen-Coquand-Huber-Mörtberg (CCHM) cubical type theory, introduced in 2015, solves this problem. It extends Martin-Löf type theory with an *interval* primitive and a *composition* operation, making univalence and function extensionality *provable* with genuine computational content. Every closed term at a base type reduces to a canonical form.

This chapter develops the foundations of cubical type theory: the interval, faces, partial elements, composition, the Glue type, and univalence as a theorem.

---

## 1. The Core Idea: Cubes as Computation

### 1.1 Paths as Cubes

In Martin-Löf type theory, a path $p : a = b$ is an element of an identity type — an atomic thing with no internal structure. In cubical type theory, a path is literally a *function* from an interval to a type:

$$p : a = b \text{ in } A \quad \Longleftrightarrow \quad p : \mathbb{I} \to A \text{ with } p(0) = a \text{ and } p(1) = b$$

This is not a new axiom but a new *definition* of what a path is. The equality $a = b$ is defined to be a function type, with the constraint that both endpoints agree definitionally.

**Higher-dimensional cubes:** A 2-dimensional path (homotopy) $H : p = q$ for $p, q : a = b$ is a function $H : \mathbb{I} \times \mathbb{I} \to A$ satisfying:
- $H(i, 0) = p(i)$ (left face)
- $H(i, 1) = q(i)$ (right face)
- $H(0, j) = a$ (bottom face, both $p$ and $q$ start at $a$)
- $H(1, j) = b$ (top face, both $p$ and $q$ end at $b$)

An $n$-dimensional cube is a function from the $n$-cube $\mathbb{I}^n$ to $A$, satisfying face constraints. This is the origin of the name "cubical" — higher paths are cubes.

### 1.2 Why Cubes Give Computational Content

The key insight is that transport and composition can be *defined* computationally from the interval structure:

- **Transport** of $a : A(0)$ along $p : A(0) = A(1)$ is just $p(1)$ — evaluating a path at its endpoint.
- **Composition** fills a partial cube: given three faces of a square, produce the fourth.

These operations are not axiomatic. They are *definitional*, meaning they reduce during computation. This is what gives canonicity.

---

## 2. The Interval and Faces

### 2.1 The Interval $\mathbb{I}$

The interval $\mathbb{I}$ is a *primitive* type in cubical type theory — not defined from other types. It has:
- Two elements: $0 : \mathbb{I}$ and $1 : \mathbb{I}$
- It is *not* a type in the usual universe (it cannot be quantified over in the same way as types)

**Face lattice:** The interval carries a De Morgan algebra structure:
$$\sim r : \mathbb{I} \quad \text{(complement)}$$
$$r \wedge s : \mathbb{I} \quad \text{(meet)}$$
$$r \vee s : \mathbb{I} \quad \text{(join)}$$

satisfying De Morgan laws:
- $\sim(\sim r) = r$
- $\sim(r \wedge s) = \sim r \vee \sim s$
- $\sim(r \vee s) = \sim r \wedge \sim s$
- $r \wedge 0 = 0$, $r \vee 1 = 1$
- $r \wedge \sim r = 0$, $r \vee \sim r = 1$

**Dimension variables:** The interval variables $i, j, k : \mathbb{I}$ are *dimension variables* that range over the interval. The context $\Gamma$ can include dimension variables alongside term variables.

### 2.2 Face Formulas

A *face formula* $\phi$ is an element of the free De Morgan algebra on dimension variables:

$$\phi ::= 0 \mid 1 \mid i \mid \sim i \mid \phi \wedge \psi \mid \phi \vee \psi$$

Face formulas specify "faces" of cubes. The constraint $\phi = 1$ defines when a face is "open":
- $\phi = (i = 0)$: the "left" face
- $\phi = (i = 1)$: the "right" face
- $\phi = (i = 0) \vee (j = 1)$: the union of two faces

The interval satisfies: if $\phi = 1$, then we are "on the face $\phi$."

---

## 3. Partial Elements and Extensions

### 3.1 Partial Types

Given a face formula $\phi$ and a type $A$, the *partial type* $[\phi \vdash A]$ is the type of elements of $A$ that are only defined when $\phi = 1$.

**Formally:** A partial element $u : [\phi \vdash A]$ is a term $u$ such that under the assumption $\phi = 1$, we have $u : A$. Outside this assumption (when $\phi \neq 1$), $u$ is undefined.

**Notation:** We write:

$$u : [\phi \vdash A]$$

and use the notation $u$ in contexts where $\phi$ is assumed. The partial element $u$ specifies only part of a cube — the part where $\phi$ holds.

### 3.2 Extension Types

Given a partial element $u : [\phi \vdash A]$ and a type $A$, an *extension* is an element $a : A$ that agrees with $u$ on $\phi$:

$$a : A[\phi \vdash u]$$

means $a : A$ and $a = u$ whenever $\phi = 1$ (definitionally).

**The filling problem:** Suppose we have three faces of a square — a partial 2-cube. An extension fills in the missing fourth face to produce a complete 2-cube. This is the *filling problem*, and the composition operation solves it.

---

## 4. Composition and Transport

### 4.1 The Composition Operation

The *composition* operation `hcomp` is the computational heart of cubical type theory:

$$\mathsf{hcomp}^A_\phi(u, a_0) : A$$

where:
- $A$ is a type
- $\phi$ is a face formula
- $u : (i : \mathbb{I}) \to [\phi \vdash A]$ is a "tube" — a partial path along dimension $i$
- $a_0 : A[\phi \vdash u(0)]$ is the "bottom" face, agreeing with $u$ at $i = 0$

The result is a "top" element completing the tube. Intuitively: given a cylinder (tube) with an open top and a closed bottom, `hcomp` fills in the top.

**Computation rule:** When $\phi = 1$, the composition reduces:
$$\mathsf{hcomp}^A_1(u, a_0) = u(1)$$

This says: if the tube is "full" (defined everywhere), the composition is just the other endpoint of the tube.

### 4.2 Transport

*Transport* along a path of types is defined from composition:

$$\mathsf{transp}^A_\phi(a) : A(1)$$

where:
- $A : \mathbb{I} \to \mathsf{Type}$ is a path of types
- $\phi$ is a constraint saying when transport should be trivial (the "is-contr" condition)
- $a : A(0)$ is the element to transport

**Computation rule:** When $\phi = 1$ (the type family is constant), transport is the identity:
$$\mathsf{transp}^A_1(a) = a$$

**Path type:**  The path type $a =_A b$ is defined as:

$$a =_A b :\equiv (i : \mathbb{I}) \to A$$

with the constraint that the function maps $0$ to $a$ and $1$ to $b$ definitionally. An element of $a =_A b$ is literally a function $p$ with $p(0) \equiv a$ and $p(1) \equiv b$ (definitional equality, not propositional equality).

**Reflexivity:** $\mathsf{refl}_a :\equiv \lambda i. a$ — the constant path.

**Symmetry:** $\mathsf{sym}(p) :\equiv \lambda i. p(\sim i)$ — reversal using complement.

**Concatenation:** $p \cdot q$ is defined using `hcomp`:
$$p \cdot q :\equiv \mathsf{hcomp}^A_{i=0 \vee i=1}(\lambda j. [i=0 \mapsto p(j), i=1 \mapsto q(j)], p(0))$$

Wait — more precisely, for $p : a = b$ and $q : b = c$, concatenation is:

$$p \cdot q :\equiv \lambda i. \mathsf{hcomp}^A_{i=0}(\lambda j. [i=0 \mapsto p(j)], q(i))$$

The key point is that these are *definitional* constructions with *definitional* computation rules, not axioms.

---

## 5. The Kan Condition

### 5.1 Kan Filling

A type $A$ in cubical type theory must satisfy the *Kan condition*: all box filling problems have solutions. Specifically:

**Open box:** An *open box* is a partial cube with all faces except one specified. In dimension 2, this is three sides of a square. The Kan condition says the missing face can always be filled.

Formally, for any $A : \mathsf{Type}$, the Kan condition states that:
$$\mathsf{hcomp}^A_\phi(u, a_0) : A$$

is always defined and satisfies the face condition: $\mathsf{hcomp}^A_\phi(u, a_0) = u(1)$ when $\phi = 1$.

**Geometric picture:** A Kan type is one where every "open box" can be filled — exactly the Kan condition from simplicial topology, now internal to type theory.

### 5.2 Composition for Inductive Types

For inductive types (like $\mathbb{N}$, $\mathsf{Bool}$, $\mathsf{List}$), the Kan/composition operation is computed *structurally*:

- For $\mathbb{N}$: the composition of a tube of naturals at $i=1$ produces the value of the tube at $1$.
- For a path type $a = b$: composition produces a path by applying the composition pointwise.
- For function types: composition is defined pointwise (apply the function to each point in the composition).
- For $\Sigma$ types: compose component-wise.

This is the *coherence* machinery: every type former comes with a defined composition operation, making all types Kan fibrations automatically.

---

## 6. The Glue Type and Univalence

### 6.1 The Glue Type

The Glue type is the key new primitive enabling univalence. Given:
- $\phi : \mathbb{F}$ (a face formula)
- $T : [\phi \vdash \mathsf{Type}]$ (a partial type, defined when $\phi = 1$)
- $A : \mathsf{Type}$ (a global type)
- $e : [\phi \vdash T \simeq A]$ (a partial equivalence from $T$ to $A$)

The *Glue type* is:

$$\mathsf{Glue}[\phi \vdash (T, e)] A : \mathsf{Type}$$

**Introduction:** Elements of the Glue type are constructed by:
$$\mathsf{glue}[\phi \vdash t] a : \mathsf{Glue}[\phi \vdash (T, e)] A$$

where $t : [\phi \vdash T]$ is a partial element and $a : A$ with $a = e(t)$ when $\phi = 1$.

**Elimination:** The ungluing map:
$$\mathsf{unglue}[\phi \vdash e](g) : A$$

extracts the underlying $A$-element.

**Computation rules:**
- When $\phi = 1$: $\mathsf{Glue}[\phi \vdash (T, e)] A = T$
- When $\phi = 1$: $\mathsf{glue}[\phi \vdash t] a = t$
- When $\phi = 1$: $\mathsf{unglue}[\phi \vdash e](t) = e(t)$

### 6.2 Proving Univalence

**Theorem 23.1 (Univalence in CCHM).** For any types $A, B : \mathsf{Type}$:
$$\mathsf{ua} : (A \simeq B) \to (A = B)$$

*Construction.* Given $e : A \simeq B$, define the path $\mathsf{ua}(e) : A = B$ as the function:

$$\mathsf{ua}(e) :\equiv \lambda i. \mathsf{Glue}[i=0 \vdash (A, e), i=1 \vdash (B, \mathsf{id}_B)] B$$

- At $i=0$: $\mathsf{Glue}[1 \vdash (A, e)] B = A$ (by the computation rule when $\phi = 1$).
- At $i=1$: $\mathsf{Glue}[1 \vdash (B, \mathsf{id})] B = B$.
- In between: $\mathsf{Glue}[... ] B$ is a "mixture" of $A$ and $B$ interpolated by the equivalence $e$.

So $\mathsf{ua}(e)$ is a genuine path from $A$ to $B$ in the universe. $\square$

**The computation rule:**

$$\mathsf{transport}(\mathsf{ua}(e), a) = e(a)$$

This holds *definitionally* (by the computation rules for Glue). This is the crucial difference from axiomatic univalence: we can compute with $\mathsf{ua}$.

### 6.3 Function Extensionality

In cubical type theory, function extensionality is also a theorem (not an axiom):

**Theorem 23.2 (FunExt).** If $h : \Pi_{x:A} f(x) = g(x)$ is a homotopy, then $f = g$.

*Proof.* Define $\mathsf{funExt}(h) :\equiv \lambda i. \lambda x. h(x)(i)$.

This is literally rearranging function arguments: the homotopy $h(x)$ is a path $I \to B$, so $\lambda i. \lambda x. h(x)(i)$ is a path $I \to (A \to B)$ from $f$ to $g$. The endpoints reduce definitionally. $\square$

---

## 7. Canonicity and Decidability

### 7.1 Canonicity

**Theorem 23.3 (Canonicity, Huber 2018).** In CCHM cubical type theory, every closed term of type $\mathbb{N}$ is definitionally equal to a numeral $\overline{n}$ for some $n : \mathbb{N}$.

*Why this matters:* Univalence is a theorem in CCHM, so proofs using univalence can be evaluated. The canonicity theorem means: even if you use univalence many times in a proof of a concrete statement, the proof term can still be reduced to a canonical value.

**In contrast:** In axiomatic HoTT (the HoTT Book), canonicity is an *open problem*. Because `ua` is an axiom with no reduction rule, a proof term that uses `ua` is stuck — it cannot be evaluated.

### 7.2 Normalization

A stronger result:

**Theorem 23.4 (Normalization, Huber 2019).** Every term in CCHM cubical type theory has a normal form (up to $\alpha$-equivalence and definitional equality of dimension expressions).

This means cubical type theory is a *computational* foundation for mathematics: every theorem has a computational interpretation.

---

## 8. Variations of Cubical Type Theory

### 8.1 Cartesian Cubical Type Theory

The CCHM theory uses a *De Morgan* algebra on the interval (with $\sim$, $\wedge$, $\vee$). A variant, *Cartesian cubical type theory* (Angiuli, Brunerie, Coquand, Harper, Hou, Licata), uses only the *Cartesian product* structure on cubes (no complement or De Morgan operations).

**Consequences of removing complement:**
- Path reversal is not definitional (requires a separate operation)
- The interval has fewer computational rules
- The type theory is simpler to implement

Cartesian cubical type theory is the basis for the **RedPRL** and **cooltt** proof assistants.

### 8.2 XTT: Cubical Type Theory with Extensional Equality

**XTT** (Sterling, Angiuli, Gratzer) extends cubical type theory with a *boundary separation* axiom: the only paths between two terms are those that agree on all faces.

**Key feature:** XTT validates *definitional* equality of all homotopy operations (associativity of path concatenation, etc.), at the cost of adding new definitional equalities.

XTT also incorporates *realizability semantics*, connecting cubical type theory to programming language theory.

### 8.3 Agda's Cubical Mode

Cubical Agda (Chapter 22) implements a variant of CCHM cubical type theory. It uses:
- The primitive interval `I` with `i0`, `i1`, `~_`, `_∧_`, `_∨_`
- Path types as function types `(i : I) → A`
- `transp` for transport
- `hcomp` for composition
- `Glue` for the Glue type

The implementation follows the CCHM paper closely, giving a practical proof assistant for computational HoTT.

---

## 9. The Semantics of Cubical Type Theory

### 9.1 Presheaf Models

Cubical type theory has a natural *presheaf* semantics. The underlying category is the *cube category* $\square$, whose objects are $[n] = \{0,1\}^n$ (the vertices of the $n$-cube) and whose morphisms are maps between cubes.

A cubical set is a presheaf $X : \square^\mathsf{op} \to \mathsf{Set}$.

**Types** are interpreted as *fibrant* cubical sets — those satisfying a filling condition analogous to Kan fibrations. The Kan condition in the presheaf semantics corresponds exactly to the composition operation in the type theory.

### 9.2 The Hofmann-Streicher Universe

The universe of types is modeled by the *Hofmann-Streicher universe* in the presheaf model: a fibrant cubical set $\mathsf{U}$ whose elements are "small" fibrant cubical sets, and whose path space between $A$ and $B$ is the space of equivalences $A \simeq B$.

Univalence follows from the structure of this universe in the presheaf model — it is a *theorem* about the model, not an assumption.

### 9.3 Connection to Simplicial Sets

The Kan-Quillen model structure on simplicial sets (Chapter 15) is the classical setting for HoTT. Cubical sets with the Kan condition form an analogous model, and there are comparison functors between simplicial and cubical sets.

The key advantage of cubical sets over simplicial sets for type theory: the composition operation in the cubical model has a direct computational interpretation, while the analogous operation in the simplicial model is defined using choice and is not computational.

---

## 10. Implementing Cubical Type Theory

### 10.1 The Implementation Challenge

Implementing a cubical type theory checker requires:
1. A representation of dimension variables and face formulas
2. Reduction rules for `transp` and `hcomp` for each type former
3. A type checker that checks definitional equality modulo these reductions
4. A universe (with Glue and univalence)

The main difficulty is the *confluence* of the reduction system: all reduction paths must reach the same normal form.

### 10.2 Reference Implementations

- **Cubical Agda**: The most mature implementation; built on top of the Agda proof assistant. Uses CCHM cubical type theory.
- **cooltt**: A research implementation of Cartesian cubical type theory by Angiuli, Sterling, and collaborators.
- **redtt** (predecessor to cooltt): Also Cartesian cubical.
- **Mini-TT**: A minimal dependent type theory by Coquand et al., serving as a reference for normalization-by-evaluation techniques used in cubical implementations.

### 10.3 Normalization by Evaluation

The standard technique for type checking in cubical type theory is *normalization by evaluation* (NbE): instead of reducing terms syntactically, evaluate them into a semantic domain and then "read back" a normal form.

NbE in cubical type theory requires:
- A semantic domain that handles the interval and face formulas
- Evaluation of `transp` and `hcomp` in the semantic domain
- A "read back" function that produces canonical syntax

This gives a sound and complete type-checking algorithm for cubical type theory.

---

## 11. Cubical Type Theory vs. Book HoTT

### 11.1 What Changes

| Feature | Book HoTT | Cubical TT |
|---------|-----------|------------|
| Identity type | Inductive, J-based | Paths as functions $I \to A$ |
| Univalence | Axiom | Theorem (via Glue) |
| Funext | Axiom | Theorem (rearrange arguments) |
| Canonicity | Open problem | Theorem |
| Computation rules | Propositional | Definitional |
| Path inversion | Via J | Via complement $\sim$ |

### 11.2 What Stays the Same

Both systems validate:
- All theorems of Martin-Löf type theory
- The full HoTT hierarchy (h-levels, equivalences, HITs)
- Homotopy group computations ($\pi_1(S^1) = \mathbb{Z}$, etc.)
- The univalence principle and structure invariance
- All synthetic homotopy theory results

The mathematical *content* is the same; cubical TT adds computational *content* to the proofs.

### 11.3 What's Lost

Cubical type theory makes some things harder:
- **Axiom K**: Disabled (just as in `--without-K` Agda). Pattern matching on identity types requires explicit handling.
- **Uniqueness of Identity Proofs**: Not valid — just as in Book HoTT.
- **Classical logic**: Can be added, but at the cost of canonicity (excluded middle has no computational interpretation).

---

## Exercises

**23.1.** Show that path concatenation defined via `hcomp` satisfies the left unit law $\mathsf{refl} \cdot p = p$ *definitionally* (i.e., by reduction). What is the normal form of `hcomp` when the face formula is satisfied?

**23.2.** Compute transport in the universe: if $p : A = B$ (a path of types) and $a : A$, what is $\mathsf{transport}(p, a) : B$? Use the Glue type definition to unpack the computation.

**23.3.** In Cubical Agda, prove that the path $\mathsf{ua}(\mathsf{id}_{A})$ is definitionally equal to $\mathsf{refl}_A$. (*Hint:* Use the computation rules for Glue and the fact that $\mathsf{unglue}$ of $\mathsf{glue}$ reduces.)

**23.4.** Explain why the De Morgan algebra structure on the interval is necessary for path reversal to be definitional. What would go wrong in Cartesian cubical type theory (which lacks complement)?

**23.5.** The interval $\mathbb{I}$ is *not* a type in the usual sense — it cannot appear as a fiber of a dependent type. Explain why this restriction is necessary for the Kan condition to hold. (*Hint:* What would $\Pi_{i:\mathbb{I}} A(i)$ mean if $\mathbb{I}$ were a type?)

**23.6.** In CCHM, prove the *Kan condition for function types*: if $A$ and $B$ are Kan types, show how to define `hcomp` for the function type $A \to B$. What is the construction?

**23.7 (Research).** Read the original CCHM paper: "Cubical Type Theory: A Constructive Interpretation of the Univalence Axiom" (Cohen, Coquand, Huber, Mörtberg, 2015). Identify the exact place where the Glue type is used to prove univalence, and explain in your own words why the computation rule for transport along $\mathsf{ua}(e)$ holds definitionally.
