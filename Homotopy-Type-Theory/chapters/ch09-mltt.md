# Chapter 9: Martin-Löf Type Theory

## Introduction

Martin-Löf Type Theory (MLTT) is the central engine of the entire curriculum. Everything before it has been preparation; everything after builds on it. It is the formal system in which proofs in Agda are written, from which Lean 4's type theory descends, and which HoTT extends by adding the univalence axiom and higher inductive types.

MLTT was developed by Per Martin-Löf through a series of lectures and papers in the 1970s and 1980s. Its central innovation over previous type theories is the *identity type*: a *type* $a =_A b$ whose elements are proofs of equality between $a$ and $b$. This seems like a small addition. It is not. The identity type is the seed from which all of homotopy type theory grows.

In this chapter we present MLTT carefully: its four judgments, its type formers with their full rules, and the crucial subtleties of the identity type that make HoTT possible.

---

## 1. The Four Judgments of MLTT

MLTT is built on four *judgments* (primitive forms of assertion):

1. $A \,\mathsf{type}$ — "$A$ is a type."
2. $A = B \,\mathsf{type}$ — "$A$ and $B$ are definitionally equal types."
3. $a : A$ — "$a$ is a term of type $A$."
4. $a = b : A$ — "$a$ and $b$ are definitionally equal terms of type $A$."

All four judgments are made *in context* $\Gamma$: a list of variable declarations $x_1 : A_1, \ldots, x_n : A_n$ (where each $A_i$ may depend on $x_1, \ldots, x_{i-1}$).

**The important distinction:** *Definitional equality* ($a = b : A$) is different from *propositional equality* ($a =_A b$, the identity type). Definitional equality holds when $a$ and $b$ reduce to the same normal form. Propositional equality is a *type* in the system — it can be proved (or disproved), and proofs of it can be manipulated.

This distinction is fundamental. Classical mathematics uses a single notion of equality. MLTT has two, and the relationship between them is one of the deepest questions in the subject.

### 1.1 Contexts

**Definition 9.1.** A *context* is a list $\Gamma = x_1 : A_1, x_2 : A_2(x_1), \ldots, x_n : A_n(x_1, \ldots, x_{n-1})$ where each type may depend on the preceding variables.

The *empty context* $()$ is the context with no variables. The judgment $\vdash A \,\mathsf{type}$ (in empty context) means $A$ is a closed type — it has no free variables.

**Context extension:** If $\Gamma \vdash A \,\mathsf{type}$, then $\Gamma, x : A$ is a valid context (for a fresh variable $x$).

---

## 2. Type Formers: Formation, Introduction, Elimination, Computation

Each type in MLTT is presented by four groups of rules:
- **Formation:** How to form the type from constituent types.
- **Introduction:** How to construct elements (the *constructors*).
- **Elimination:** How to use elements — the *eliminator* or *recursor*.
- **Computation:** How the eliminator computes on the constructors ($\beta$-rules).
- (Optional) **Uniqueness:** How elements are determined by their eliminator behavior ($\eta$-rules).

This FIEC structure is the backbone of Martin-Löf's approach. Every type in HoTT follows it — including the identity type.

---

## 3. Dependent Function Types (Π)

**Formation:**
$$\frac{\Gamma \vdash A \,\mathsf{type} \quad \Gamma, x : A \vdash B \,\mathsf{type}}{\Gamma \vdash \Pi_{x:A} B \,\mathsf{type}}$$

**Introduction:**
$$\frac{\Gamma, x : A \vdash b : B}{\Gamma \vdash \lambda x : A,\, b : \Pi_{x:A} B}$$

**Elimination:**
$$\frac{\Gamma \vdash f : \Pi_{x:A} B \quad \Gamma \vdash a : A}{\Gamma \vdash f\, a : B[a/x]}$$

**Computation ($\beta$):**
$$\frac{\Gamma, x : A \vdash b : B \quad \Gamma \vdash a : A}{\Gamma \vdash (\lambda x, b)\, a = b[a/x] : B[a/x]}$$

**Uniqueness ($\eta$):**
$$\frac{\Gamma \vdash f : \Pi_{x:A} B}{\Gamma \vdash f = \lambda x,\, f\, x : \Pi_{x:A} B}$$

---

## 4. Dependent Pair Types (Σ)

**Formation, Introduction, Elimination:** As in Chapter 8.

**Computation:**
$$\mathsf{fst}(a, b) = a : A \qquad \mathsf{snd}(a, b) = b : B[a/x]$$

**General Eliminator (Σ-rec/ind):**
For a motive $C : \Sigma_{x:A} B(x) \to \mathsf{Type}$ and a section $g : \Pi_{x:A} \Pi_{y:B(x)} C(x, y)$:
$$\mathsf{ind}_\Sigma(g, (a, b)) = g\, a\, b : C(a, b)$$

---

## 5. Inductive Types: Natural Numbers

**Formation, Introduction:** As in Chapter 8.

**Eliminator (dependent recursor):** Given:
- Motive: $P : \mathbb{N} \to \mathsf{Type}$
- Base: $p_0 : P(\mathsf{zero})$
- Step: $p_s : \Pi_{n:\mathbb{N}}\, P(n) \to P(\mathsf{succ}\, n)$

Then $\mathsf{ind}_\mathbb{N}(P, p_0, p_s) : \Pi_{n:\mathbb{N}} P(n)$.

**Computation:**
$$\mathsf{ind}_\mathbb{N}(P, p_0, p_s, \mathsf{zero}) = p_0$$
$$\mathsf{ind}_\mathbb{N}(P, p_0, p_s, \mathsf{succ}\, n) = p_s\, n\, (\mathsf{ind}_\mathbb{N}(P, p_0, p_s, n))$$

---

## 6. The Identity Type

This is the heart of MLTT and the key to HoTT.

### 6.1 Formation

**Definition 9.2 (Identity Type).** Given a type $A$ and terms $a, b : A$, the *identity type* is:
$$\frac{\Gamma \vdash A \,\mathsf{type} \quad \Gamma \vdash a : A \quad \Gamma \vdash b : A}{\Gamma \vdash a =_A b \,\mathsf{type}}$$

An element of $a =_A b$ is a *proof of equality* (or, in HoTT, a *path from $a$ to $b$*).

The subscript $A$ is often omitted when clear from context: $a = b$.

### 6.2 Introduction (Reflexivity)

The only constructor for the identity type is *reflexivity*:

$$\frac{\Gamma \vdash a : A}{\Gamma \vdash \mathsf{refl}_a : a =_A a}$$

Every element is equal to itself, witnessed by $\mathsf{refl}$.

**Important:** $\mathsf{refl}_a$ is the *only* way to construct an element of $a = b$ using the introduction rule. But the eliminator allows us to derive other equalities by induction.

### 6.3 Elimination: Path Induction (J Eliminator)

The elimination rule for the identity type is *path induction*, also called the *J rule* or *J eliminator*. It is the central principle of the identity type.

**J Eliminator:** Given:
- $A : \mathsf{Type}$ and $a : A$ (the *basepoint*)
- $C : \Pi_{b:A}\, (a =_A b) \to \mathsf{Type}$ (the *motive* or *path family*)
- $d : C\, a\, \mathsf{refl}_a$ (the *base case*: what $C$ holds at the reflexive path)

The eliminator produces:
$$\mathsf{J}(C, d) : \Pi_{b:A}\, \Pi_{p : a =_A b}\, C\, b\, p$$

**Computation rule ($\beta$):**
$$\mathsf{J}(C, d)\, a\, \mathsf{refl}_a = d : C\, a\, \mathsf{refl}_a$$

**Reading J:** J says: to prove a property of *all paths* starting from $a$, it suffices to prove it for the *trivial path* $\mathsf{refl}_a$. This is path induction: any path starting at $a$ can be contracted to the reflexive path.

### 6.4 Alternative Formulation: Based vs. Unbased Path Induction

There are two common forms:

**Based path induction (J as above):** The basepoint $a$ is fixed; we induct over all paths starting at $a$.

**Unbased path induction (sometimes called J' or the "transport" version):** Induct over all pairs $(a, b, p)$ simultaneously:

Given:
- $C : \Pi_{a b : A}\, (a =_A b) \to \mathsf{Type}$
- $d : \Pi_{a:A}\, C\, a\, a\, \mathsf{refl}_a$

Produces: $\Pi_{a b : A}\, \Pi_{p : a =_A b}\, C\, a\, b\, p$.

**Theorem 9.3.** The based and unbased forms are equivalent (each can derive the other in MLTT).

### 6.5 What J Allows: The Groupoid Laws

Using only J, we can prove:

**Symmetry (path inversion):** $p^{-1} : b =_A a$ from $p : a =_A b$.
*Proof:* Apply J with $C\, b\, p = (b =_A a)$ and $d = \mathsf{refl}_a$. Then $\mathsf{J}(C, \mathsf{refl}_a) : \Pi_{b:A}\, \Pi_{p : a=b}\, (b = a)$. Applying to $(b, p)$ gives $p^{-1}$.

**Transitivity (path concatenation):** $p \cdot q : a =_A c$ from $p : a =_A b$ and $q : b =_A c$.
*Proof:* Apply J to $q$, with $C\, c\, q' = (a =_A c)$ and $d = p$. This says: if $q' : b = c$ and we know $a = b$, then $a = c$.

**Groupoid laws:** The following hold propositionally (as elements of identity types):
- $p \cdot \mathsf{refl} = p$ and $\mathsf{refl} \cdot p = p$ (unit laws)
- $(p \cdot q) \cdot r = p \cdot (q \cdot r)$ (associativity)
- $p \cdot p^{-1} = \mathsf{refl}$ and $p^{-1} \cdot p = \mathsf{refl}$ (inverse laws)

Each of these is proved by path induction.

**The fundamental observation:** These laws say that *every type $A$ is a groupoid*, with elements as objects and identity proofs as morphisms. This is the beginning of the homotopy-theoretic interpretation.

### 6.6 What J Does NOT Allow: UIP

The *Uniqueness of Identity Proofs* (UIP) is the statement:
$$\mathsf{UIP}_A : \Pi_{a b : A}\, \Pi_{p q : a =_A b},\, p = q$$

In classical set-theoretic mathematics, any two proofs of $a = b$ are "the same" — equality is equality. But:

**Theorem 9.4 (Streicher, 1993).** UIP is *not derivable* from the J rule alone. There are models of MLTT where UIP fails.

*Evidence:* Groupoid models. Consider the fundamental groupoid of the circle $S^1$. The loop $\gamma$ (going around once) and the loop $\gamma \cdot \gamma^{-1}$ (going around and back) are different elements of $\mathsf{base} =_{S^1} \mathsf{base}$ — not equal to $\mathsf{refl}$. So two proofs of the same equality can genuinely differ.

This is the *key insight* that opens the door to HoTT: if identity types can have *multiple distinct elements* (like path spaces in topology, which are non-trivial), then we have a rich higher-dimensional structure.

### 6.7 Intensional vs. Extensional MLTT

**Intensional MLTT:** The identity type is non-trivial; UIP is not assumed. Identity proofs carry computational content. This is the version underlying HoTT.

**Extensional MLTT:** An additional rule (*reflection rule*) states that if $p : a = b$, then $a \equiv b$ (definitional equality). With this rule, any two proofs of $a = b$ are definitionally equal (so UIP holds). But type checking becomes *undecidable*.

Lean 4 and Agda use intensional MLTT (though Agda optionally adds UIP via `--with-K`, which we explicitly avoid for HoTT work).

---

## 7. Transport and the Dependent Eliminator

One of the most important derived operations from J is *transport*.

**Definition 9.5 (Transport).** Given $P : A \to \mathsf{Type}$ and $p : a =_A b$, there is a function:
$$\mathsf{transport}^P(p) : P(a) \to P(b)$$

*Construction:* Apply J to $p$, with motive $C\, b'\, p' = P(a) \to P(b')$ and base $d = \mathsf{id}_{P(a)}$. This gives a function $\Pi_{b:A}\, \Pi_{p:a=b}\, P(a) \to P(b)$.

**Meaning:** Transport says that equal elements of $A$ have "isomorphic" (in fact, equal) types. If $a = b$, then $P(a) \cong P(b)$ — via an actual function $P(a) \to P(b)$.

In homotopy theory: transport along a path $p : a \to b$ is parallel transport of the fiber $P(a)$ to the fiber $P(b)$ along the path $p$.

**Example 9.6.** If $P(n) = \mathsf{Vec}\, A\, n$ and $p : m = n$, then $\mathsf{transport}^P(p) : \mathsf{Vec}\, A\, m \to \mathsf{Vec}\, A\, n$ converts a vector of length $m$ to one of length $n$, using the proof that $m = n$.

**Example 9.7 (Substitution).** If $P(x) = (x = c)$ and $p : a = b$, then $\mathsf{transport}^P(p) : (a = c) \to (b = c)$. This is: if $a = b$ and $a = c$, then $b = c$ (by transitivity).

---

## 8. The Action on Paths (ap)

**Definition 9.8.** Given $f : A \to B$ and $p : a =_A b$, there is a path:
$$\mathsf{ap}_f(p) : f(a) =_B f(b)$$

*Construction:* Apply J to $p$, with motive $C\, b'\, p' = f(a) =_B f(b')$ and base $\mathsf{refl}_{f(a)}$.

**Meaning:** Functions preserve equality — or in homotopy language, functions are *continuous maps* that map paths to paths.

**Properties:**
- $\mathsf{ap}_f(\mathsf{refl}_a) = \mathsf{refl}_{f(a)}$
- $\mathsf{ap}_f(p \cdot q) = \mathsf{ap}_f(p) \cdot \mathsf{ap}_f(q)$
- $\mathsf{ap}_f(p^{-1}) = (\mathsf{ap}_f(p))^{-1}$
- $\mathsf{ap}_g(\mathsf{ap}_f(p)) = \mathsf{ap}_{g \circ f}(p)$

These say that $\mathsf{ap}$ makes every function a *functor between groupoids* — connecting MLTT to category theory.

---

## 9. Homotopies

**Definition 9.9 (Homotopy).** Given $f, g : A \to B$, a *homotopy* from $f$ to $g$ is:
$$H : \Pi_{x:A}\, f(x) =_B g(x)$$

Two functions are homotopic if they agree at every point, *via a specific identification*.

**Function extensionality (funext):** The statement that homotopic functions are equal:
$$\mathsf{funext} : (f \sim g) \to (f = g)$$
where $f \sim g$ is the type of homotopies. This is *not* provable from J alone in MLTT. It is a theorem in cubical type theory and follows from univalence in HoTT.

---

## 10. Definitional Equality and Computation

Two terms $a$ and $b$ are *definitionally equal* (written $a \equiv b$) if they reduce to the same normal form. Definitional equality is:
- **Reflexive:** $a \equiv a$
- **Symmetric:** $a \equiv b \Rightarrow b \equiv a$
- **Transitive:** $a \equiv b$ and $b \equiv c \Rightarrow a \equiv c$
- **Congruent:** Preserved by all type-forming operations
- **Includes $\beta$-reduction:** $(\lambda x, t)\, a \equiv t[a/x]$
- **Includes $\delta$-reduction:** $\mathsf{ind}_\mathbb{N}(P, p_0, p_s, 0) \equiv p_0$ and similarly for $\mathsf{succ}$

The type checker verifies judgments *up to definitional equality*: if $a \equiv b$ and $a : A$, then $b : A$ as well (the *conversion rule*).

---

## 11. Why MLTT Is the Right Foundation

MLTT addresses the problems with ZFC identified in Chapter 1:

| **Problem** | **MLTT's Solution** |
|---|---|
| Identity is set-membership | Types are not sets; identity is a type |
| No computational content | Every proof is a program |
| Category errors allowed | Types prevent meaningless combinations |
| Sets are "flat" | Types have higher structure (identity types of identity types) |
| AC is an axiom | The dependent version of AC is provable |

MLTT is the right foundation for mathematics because:
1. It is **consistent** (provably, via normalization)
2. It has **computational content** (proofs are programs, theorems are specifications)
3. It is **expressive** (all of mathematics can be formalized)
4. It is **constructive** (no excluded middle assumed)

And HoTT shows it is also:
5. **Homotopy-theoretically complete** (types model homotopy types, via the univalence axiom)

---

## Exercises

**9.1.** Using only the J eliminator, prove the following. Give explicit terms.
  - Symmetry: if $p : a = b$, construct $p^{-1} : b = a$.
  - Transitivity: if $p : a = b$ and $q : b = c$, construct $p \cdot q : a = c$.

**9.2.** Using transport, prove:
  - If $p : a = b$ and $q : a = c$, then $b = c$. Give the term.
  - If $P(n) = \mathsf{IsEven}(n)$ and $p : 2 = 2$, what is $\mathsf{transport}^P(p)$?

**9.3.** Prove the right unit law: $p \cdot \mathsf{refl} = p$ for any $p : a = b$. (*Hint:* Apply J to $p$.)

**9.4.** Prove $\mathsf{ap}_f(p \cdot q) = \mathsf{ap}_f(p) \cdot \mathsf{ap}_f(q)$ using J.

**9.5.** In Agda (with `--without-K`), verify that `uip` (the statement that any two proofs of `a ≡ b` are themselves equal) cannot be proved without additional axioms. Try to write the term and observe where it gets stuck.

**9.6.** Define a type family $P : \mathbb{N} \to \mathsf{Type}$ such that $P(0)$ and $P(1)$ are different types. Construct an element of $0 =_\mathbb{N} 0$ (reflexivity) and apply transport to it along this family.

**9.7.** The *J' rule* (path induction with both endpoints varying): State and prove J' from J. Show that J can also be derived from J'.

**9.8 (Challenge).** Streicher's groupoid model: Sketch how the fundamental groupoids of topological spaces provide a model of MLTT where UIP fails. Specifically:
  - What is the "type" of objects in this model?
  - What is the identity type $a =_X b$ in this model?
  - What elements does it have (for $X = S^1$ and specific $a, b$)?
  - Why does UIP fail?
