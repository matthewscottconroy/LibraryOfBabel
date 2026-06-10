# Chapter 19: Higher Inductive Types

## Introduction

Ordinary inductive types are defined by *point constructors* — ways to build elements. Higher inductive types (HITs) are defined by both point constructors and *path constructors* — ways to introduce paths (equalities) between elements.

HITs are the device by which HoTT can define non-trivial topological spaces *synthetically*: the circle, spheres, suspensions, pushouts, and truncations are all defined as HITs. Without HITs, HoTT could only reason about spaces that are already part of the type-theoretic universe (sets, Π types, Σ types); with HITs, we can *construct* spaces with nontrivial homotopy groups directly.

---

## 1. The Interval Type

The simplest non-trivial HIT is the *interval type* — a type that models the topological interval $[0,1]$.

**Definition 19.1 (Interval Type).** The interval $\mathbb{I}$ is defined by:
- Point constructor: $0 : \mathbb{I}$
- Point constructor: $1 : \mathbb{I}$
- Path constructor: $\mathsf{seg} : 0 =_\mathbb{I} 1$

The eliminator: given any type $B$, points $b_0, b_1 : B$, and a path $p : b_0 =_B b_1$, there is a function $\mathsf{ind}_\mathbb{I}(b_0, b_1, p) : \mathbb{I} \to B$ with:
$$\mathsf{ind}_\mathbb{I}(b_0, b_1, p)(0) \equiv b_0 \qquad \mathsf{ind}_\mathbb{I}(b_0, b_1, p)(1) \equiv b_1$$
$$\mathsf{ap}_{\mathsf{ind}(b_0,b_1,p)}(\mathsf{seg}) = p$$

The last equation says: the function sends $\mathsf{seg}$ to $p$, i.e., it maps the "canonical path" in $\mathbb{I}$ to any given path $p$ in $B$.

**Theorem 19.2.** $\mathbb{I}$ is contractible.

*Proof.* Take $0$ as the center. The contracting homotopy sends $1$ to $0$ (via $\mathsf{seg}^{-1}$) and fixes $0$. The eliminator gives this contraction. $\square$

**Why the interval is interesting:** Although contractible, the interval type gives us function extensionality (from the data of a homotopy, construct a function $\mathbb{I} \to (A \to B)$). It is also used in cubical type theory as a primitive type.

---

## 2. The Circle $S^1$

The circle is the paradigmatic non-trivial HIT.

**Definition 19.3 (Circle).** $S^1$ is defined by:
- Point constructor: $\mathsf{base} : S^1$
- Path constructor: $\mathsf{loop} : \mathsf{base} =_{S^1} \mathsf{base}$

The *eliminator* (the dependent version): given:
- A type family $P : S^1 \to \mathsf{Type}$
- A point $b : P(\mathsf{base})$
- A path-over: $\ell : \mathsf{transport}^P(\mathsf{loop})(b) = b$

There is a function $\mathsf{ind}_{S^1}(b, \ell) : \Pi_{x:S^1} P(x)$ with:
$$\mathsf{ind}_{S^1}(b, \ell)(\mathsf{base}) \equiv b$$
$$\mathsf{apd}_{\mathsf{ind}(b,\ell)}(\mathsf{loop}) = \ell$$

(Here $\mathsf{apd}$ is the dependent version of $\mathsf{ap}$, for sections of a family over a path.)

**Non-dependent eliminator:** Given $b : B$ and $\ell : b = b$ (a loop in $B$), there is a unique (up to homotopy) function $\mathsf{rec}_{S^1}(b, \ell) : S^1 \to B$ with $\mathsf{rec}(\mathsf{base}) = b$ and $\mathsf{ap}_{\mathsf{rec}}(\mathsf{loop}) = \ell$.

**Example 19.4.** The function $S^1 \to S^1$ that doubles the winding number: $\mathsf{rec}_{S^1}(\mathsf{base}, \mathsf{loop} \cdot \mathsf{loop})$.

**Example 19.5.** Any group $G$ gives a map from $\pi_1(S^1) \to G$: pick $g \in G$ (the image of the generator $\mathsf{loop}$) and use $\mathsf{rec}_{S^1}(\mathsf{id}_G, g)$... but this requires $G$ to be the underlying type with a suitable structure.

---

## 3. $\pi_1(S^1) = \mathbb{Z}$: The Fundamental Computation

The calculation of $\pi_1(S^1) = \mathbb{Z}$ is the canonical benchmark of HoTT. It is a genuine theorem, proved using the HIT structure of $S^1$, the encode-decode method, and the structure of $\mathbb{Z}$.

### 3.1 The Encode-Decode Method

**Step 1: Define a fibration ("code family").**

Define a type family $\mathsf{code} : S^1 \to \mathsf{Type}$ over the circle:
$$\mathsf{code}(\mathsf{base}) :\equiv \mathbb{Z}$$
$$\mathsf{transport}^{\mathsf{code}}(\mathsf{loop}) :\equiv \mathsf{succ}_\mathbb{Z} \quad \text{(the successor function)}$$

The second equation uses the non-dependent eliminator for $S^1$: to define $\mathsf{code} : S^1 \to \mathsf{Type}$, we need a type at $\mathsf{base}$ and an equivalence at $\mathsf{loop}$. We choose $\mathbb{Z}$ and the equivalence $\mathsf{succ}_\mathbb{Z} : \mathbb{Z} \simeq \mathbb{Z}$ (which is indeed an equivalence). By univalence, $\mathsf{succ}_\mathbb{Z} : \mathbb{Z} = \mathbb{Z}$ is a path.

**Step 2: Define encode and decode.**

$$\mathsf{encode} : \Pi_{x : S^1}\, (\mathsf{base} = x) \to \mathsf{code}(x)$$
$$\mathsf{encode}(x, p) :\equiv \mathsf{transport}^{\mathsf{code}}(p)(0)$$

(Transport the number $0$ along $p$; each traversal of $\mathsf{loop}$ increments by 1.)

$$\mathsf{decode} : \Pi_{x : S^1}\, \mathsf{code}(x) \to (\mathsf{base} = x)$$
$$\mathsf{decode}(\mathsf{base}, n) :\equiv \mathsf{loop}^n$$

(The $n$-th power of $\mathsf{loop}$: $\mathsf{loop}^0 = \mathsf{refl}$, $\mathsf{loop}^{n+1} = \mathsf{loop}^n \cdot \mathsf{loop}$, $\mathsf{loop}^{-n} = (\mathsf{loop}^n)^{-1}$.)

**Step 3: Show encode and decode are inverse.**

Show $\mathsf{decode}(\mathsf{encode}(p)) = p$ and $\mathsf{encode}(\mathsf{decode}(n)) = n$.

The second is straightforward: $\mathsf{encode}(\mathsf{base}, \mathsf{loop}^n) = \mathsf{transport}^{\mathsf{code}}(\mathsf{loop}^n)(0) = n$ (each $\mathsf{loop}$ increments by 1, so applying $n$ times gives $n$).

The first requires more care (induction over the path structure using $S^1$'s eliminator).

**Theorem 19.6.** $(\mathsf{base} =_{S^1} \mathsf{base}) \simeq \mathbb{Z}$.

This says: the loop space of $S^1$ at the basepoint is the integers. In other words, $\pi_1(S^1) = \mathbb{Z}$.

---

## 4. Suspension

**Definition 19.7 (Suspension).** For any type $A$, the *suspension* $\Sigma A$ is defined by:
- Point constructor: $\mathsf{N} : \Sigma A$ (north pole)
- Point constructor: $\mathsf{S} : \Sigma A$ (south pole)
- Path constructor: $\mathsf{merid}(a) : \mathsf{N} =_{\Sigma A} \mathsf{S}$ for each $a : A$

**Examples:**
- $\Sigma \mathbf{0} = \mathbf{1}$ (suspension of empty type is a point)
- $\Sigma \mathbf{1} = \mathbb{I}$ (suspension of a point is the interval)
- $\Sigma \mathsf{Bool} = S^1$ (suspension of a two-element type is the circle)
- $\Sigma S^1 = S^2$ (suspension of the circle is the 2-sphere)
- $\Sigma S^n = S^{n+1}$

**Theorem 19.8 (Freudenthal Suspension Theorem, HoTT version).** For any $n$-connected type $A$, the suspension $\Sigma A$ is $(n+1)$-connected. Moreover, the map $\pi_k(A) \to \pi_{k+1}(\Sigma A)$ is an isomorphism for $k \leq 2n$ and surjective for $k = 2n+1$.

This is a theorem of synthetic homotopy theory — proved entirely within HoTT using the tools of path induction and the eliminator for the suspension HIT.

---

## 5. Pushouts

Pushouts are the most general "gluing" construction in homotopy theory.

**Definition 19.9 (Pushout).** Given types $A$, $B$, $C$ and maps $f : C \to A$ and $g : C \to B$, the *pushout* $A \sqcup_C B$ is defined by:
- Point constructor: $\mathsf{inl} : A \to A \sqcup_C B$
- Point constructor: $\mathsf{inr} : B \to A \sqcup_C B$
- Path constructor: $\mathsf{glue}(c) : \mathsf{inl}(f(c)) = \mathsf{inr}(g(c))$ for each $c : C$

**The universal property:** A function out of $A \sqcup_C B$ is the same as a pair of functions out of $A$ and $B$ that agree on $C$ (via the maps $f$ and $g$).

**Special cases:**
- If $C = \mathbf{0}$, then $A \sqcup_\mathbf{0} B = A + B$ (coproduct)
- If $C = A$ and $f = \mathsf{id}$, then $A \sqcup_A B = B$ (along any $g : A \to B$)
- Suspension $\Sigma A = \mathbf{1} \sqcup_A \mathbf{1}$ (pushout of $A \to \mathbf{1}$ with itself)
- Join $A * B = A \sqcup_{A \times B} B$ (along the two projections)

### 5.1 The Seifert-van Kampen Theorem in HoTT

**Theorem 19.10 (van Kampen in HoTT).** For a pushout $X = A \sqcup_C B$ and a group-valued functor $\pi_1$, if $C$, $A$, $B$ are path-connected and the maps are basepoint-preserving:
$$\pi_1(X) \cong \pi_1(A) *_{\pi_1(C)} \pi_1(B)$$

This is the synthetic proof of the van Kampen theorem — proved using the universal property of the pushout HIT and the encoding-decoding method for $\pi_1$.

---

## 6. Truncations as HITs

The propositional truncation $\|A\|$ and general $n$-truncations can be defined as HITs.

**Definition 19.11 (Propositional Truncation).** $\|A\|$ is defined by:
- Constructor: $|{-}| : A \to \|A\|$
- Path constructor: $\mathsf{squash} : \Pi_{x y : \|A\|}\, x = y$

**Definition 19.12 (Set Truncation).** $\|A\|_0$ is defined by:
- Constructor: $|{-}|_0 : A \to \|A\|_0$
- Path constructor: $\mathsf{set} : \Pi_{x y : \|A\|_0}\, \Pi_{p q : x = y}\, p = q$

**Definition 19.13 (n-Truncation).** $\|A\|_n$ is defined by:
- Constructor: $|{-}|_n : A \to \|A\|_n$
- For each $k > n$: higher path constructors making all $k$-dimensional identity types contractible

**Eliminator for $\|A\|$:** To prove $P(x)$ for all $x : \|A\|$ where $P$ is a proposition (h-prop), it suffices to prove $P(|a|)$ for all $a : A$.

**Why this is correct:** The truncation $\|A\|$ has exactly the elements of $A$ (via $|{-}|$) but with all distinct proofs forcibly identified. The universal property ensures it is the "best proposition that A maps into."

---

## 7. More HITs: Eilenberg-MacLane Spaces and Spectra

HITs can be used to define all Eilenberg-MacLane spaces:

**Definition 19.14 ($K(G, n)$ as a HIT).** For a group $G$:
- $K(G, 1)$: defined as a $K(G, 1)$ = 1-type with $\pi_1 = G$ and $\pi_k = 0$ for $k \geq 2$.
  - In HoTT: build it as a quotient of the free higher groupoid on $G$, truncated to a 1-type.
- $K(G, n)$ for $n \geq 2$: defined iteratively using suspension and truncation.

**Spectra in HoTT (advanced):** Spectra are sequences of types $E_n$ with equivalences $E_n \simeq \Omega E_{n+1}$ (each type is the loop space of the next). They model the objects of stable homotopy theory. HITs and universes allow the definition of specific spectra (Eilenberg-MacLane spectra, sphere spectrum) synthetically.

---

## Exercises

**19.1.** Verify that the eliminator for $\mathbb{I}$ gives function extensionality: if $H : \Pi_{x:A} (f(x) = g(x))$ is a homotopy, construct a function $\mathbb{I} \to (A \to B)$ and use it to derive $f = g$ (as a path in $A \to B$).

**19.2.** Define the torus $T^2$ as a HIT (it has two generators $a, b$ and a 2-cell for the relation $aba^{-1}b^{-1}$). What are its constructors?

**19.3.** Define the real projective plane $\mathbb{RP}^2$ as a HIT (one point, one loop, and a 2-cell).

**19.4.** Verify the first step of the $\pi_1(S^1)$ computation: show that $\mathsf{transport}^{\mathsf{code}}(\mathsf{loop})(n) = n + 1$ using the definition of $\mathsf{code}$ and the univalence axiom.

**19.5.** Show that $\Sigma \mathsf{Bool} \simeq S^1$ — the suspension of the booleans is the circle. (*Hint:* Map $\mathsf{N}$ and $\mathsf{S}$ to $\mathsf{base}$, and use the two meridians $\mathsf{merid}(\mathsf{true})$ and $\mathsf{merid}(\mathsf{false})$ to build the loop.)

**19.6.** The pushout $A \sqcup_C B$ (for $C = \mathbf{0}$): verify that $A \sqcup_\mathbf{0} B = A + B$ using the universal property.

**19.7.** In Agda with `--cubical`, define the circle $S^1$ as a HIT and formalize:
  - The loop $\mathsf{loop} : \mathsf{base} = \mathsf{base}$
  - The function $\mathbb{Z} \to (\mathsf{base} = \mathsf{base})$ (the decoding function)
  - The fact that $\mathsf{loop}^1 = \mathsf{loop}$

**19.8 (Challenge).** The encode-decode proof of $\pi_1(S^1) = \mathbb{Z}$: work through the full proof as given in the HoTT Book (Chapter 8.1) in detail. Identify every use of:
  - The HIT eliminator for $S^1$
  - The univalence axiom
  - Transport computations
  - Path induction
