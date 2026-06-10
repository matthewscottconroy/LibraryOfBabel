# Chapter 20: Synthetic Homotopy Theory

## Introduction

Synthetic homotopy theory is homotopy theory done *inside* HoTT. Rather than building topological spaces and proving theorems about them using point-set methods, we reason about types (which *are* homotopy types) using only the rules of type theory.

The word "synthetic" contrasts with "analytic": analytic geometry builds curves as subsets of $\mathbb{R}^2$; synthetic geometry states properties directly from axioms. Similarly, analytic homotopy theory builds spaces as sets with extra structure; synthetic homotopy theory reasons about "spaces" (types) using only the axioms of HoTT.

This chapter presents the key theorems of synthetic homotopy theory: the computation $\pi_1(S^1) = \mathbb{Z}$, the Seifert-van Kampen theorem, the Freudenthal suspension theorem, and the Hopf fibration. These are genuine theorems with genuine proofs — they are not just translations of classical results, but new proofs using new methods.

---

## 1. The Encode-Decode Method

The primary computational technique in synthetic homotopy theory is the *encode-decode method*, introduced for the computation of $\pi_1(S^1)$ (Chapter 19) and generalized to all higher homotopy groups.

### 1.1 The General Setup

To compute $\pi_n(X, x_0)$ (the $n$-th homotopy group of a type $X$ at a basepoint $x_0$):

**Step 1 (Code):** Define a type family $\mathsf{code} : X \to \mathsf{Type}$ such that:
- $\mathsf{code}(x_0)$ is the group we expect $\pi_n(X)$ to be
- The action of each path constructor of $X$ on $\mathsf{code}$ gives the correct group operation

**Step 2 (Encode):** Define $\mathsf{encode} : (x_0 = x) \to \mathsf{code}(x)$ by transport.

**Step 3 (Decode):** Define $\mathsf{decode} : \mathsf{code}(x) \to (x_0 = x)$ using the HIT elimination principle.

**Step 4 (Inverse):** Show $\mathsf{encode} \circ \mathsf{decode} = \mathsf{id}$ and $\mathsf{decode} \circ \mathsf{encode} = \mathsf{id}$.

This gives: $(x_0 = x) \simeq \mathsf{code}(x)$, and at $x = x_0$: $\Omega(X, x_0) = (x_0 = x_0) \simeq \mathsf{code}(x_0)$.

### 1.2 Generalization to Higher Groups

For $\pi_n(X, x_0)$ with $n \geq 2$: apply the encode-decode method to $\Omega^{n-1}(X, x_0)$ (the $(n-1)$-fold loop space), where loops are now $(n-1)$-dimensional.

---

## 2. $\pi_1(S^1) = \mathbb{Z}$: Full Proof

We sketch the full proof.

**Setup:** $S^1 = \{\ \mathsf{base},\, \mathsf{loop} : \mathsf{base} = \mathsf{base}\ \}$.

**Code:** By univalence, there is a path $\mathsf{ua}(\mathsf{succ}) : \mathbb{Z} = \mathbb{Z}$. Define:
$$\mathsf{code} : S^1 \to \mathsf{Type}$$
$$\mathsf{code}(\mathsf{base}) :\equiv \mathbb{Z}, \qquad \mathsf{ap}_\mathsf{code}(\mathsf{loop}) :\equiv \mathsf{ua}(\mathsf{succ})$$

(By the $S^1$ eliminator: we need a type at $\mathsf{base}$ and a path in $\mathsf{Type}$ at $\mathsf{loop}$.)

**Encode:** For any $p : \mathsf{base} = x$:
$$\mathsf{encode}(p) :\equiv \mathsf{transport}^\mathsf{code}(p)(0_\mathbb{Z})$$

This "walks" along $p$, incrementing the integer counter each time it traverses $\mathsf{loop}$.

**Lemma 20.1 (Encode of loop powers).**
$$\mathsf{encode}(\mathsf{loop}^n) = n \quad \text{for all } n : \mathbb{Z}$$

*Proof.* By induction on $n$:
- $\mathsf{encode}(\mathsf{refl}) = \mathsf{transport}^\mathsf{code}(\mathsf{refl})(0) = 0$ ✓
- $\mathsf{encode}(\mathsf{loop}^{n+1}) = \mathsf{transport}^\mathsf{code}(\mathsf{loop}^n \cdot \mathsf{loop})(0) = \mathsf{transport}^\mathsf{code}(\mathsf{loop})(\mathsf{encode}(\mathsf{loop}^n)) = \mathsf{succ}(n) = n+1$ ✓
- Negative case: similar using $\mathsf{loop}^{-1}$ and $\mathsf{pred}$. $\square$

**Decode:** $\mathsf{decode}(\mathsf{base}, n) :\equiv \mathsf{loop}^n$.

**Theorem 20.2.** $\mathsf{encode}$ and $\mathsf{decode}$ are inverse equivalences, so $(\mathsf{base} = \mathsf{base}) \simeq \mathbb{Z}$.

*Proof of decode ∘ encode = id:* For any $p : \mathsf{base} = x$, we need $\mathsf{decode}(x, \mathsf{encode}(p)) = p$. By path induction on $p$: reduces to $\mathsf{decode}(\mathsf{base}, 0) = \mathsf{loop}^0 = \mathsf{refl} = p$ ✓.

*Proof of encode ∘ decode = id:* Need $\mathsf{encode}(\mathsf{loop}^n) = n$. This is Lemma 20.1. $\square$

**Corollary 20.3.** The fundamental group of the circle is the integers: $\pi_1(S^1) = \mathbb{Z}$.

---

## 3. The Seifert-van Kampen Theorem

**Theorem 20.4 (van Kampen in HoTT).** Let $A$ and $B$ be types with a common subtype $C$ (maps $f : C \to A$ and $g : C \to B$). Let $X = A \sqcup_C B$ be the pushout. Then:
$$\pi_1(X) \cong \pi_1(A) *_{\pi_1(C)} \pi_1(B)$$
where the right side is the amalgamated free product of groups (pushout in the category of groups).

**Proof strategy:**
1. Define $\mathsf{code} : X \to X \to \mathsf{Type}$ — the path code fibration over the pushout.
2. Use the universal property of the pushout to define $\mathsf{code}$ in terms of $\mathsf{code}_A$ and $\mathsf{code}_B$ (the path code families for $A$ and $B$), glued together along $\mathsf{code}_C$.
3. Use encode-decode to identify paths in $X$ with elements of the amalgamated free product.

**Example 20.5 ($\pi_1$ of the torus).** The torus $T^2 = A \sqcup_C B$ where $A = B = $ cylinders and $C = $ the equator. By van Kampen:
$$\pi_1(T^2) = \mathbb{Z} *_\mathbb{Z} \mathbb{Z} = \langle a, b \mid aba^{-1}b^{-1} \rangle = \mathbb{Z}^2$$

**Example 20.6 ($\pi_1$ of a wedge).** $S^1 \vee S^1$ has $\pi_1 = \mathbb{Z} * \mathbb{Z} = F_2$ (the free group on two generators).

---

## 4. Connectedness and Truncatedness

### 4.1 Connected Types

**Definition 20.7.** A type $A$ is *$n$-connected* if $\|A\|_n$ is contractible.
- $(-1)$-connected: $A$ is inhabited (has an element)
- $0$-connected: $A$ is path-connected (any two elements are merely equal)
- $1$-connected (simply connected): $\pi_0(A) = \mathbf{1}$ and $\pi_1(A, a) = \mathbf{1}$ for all $a$

**Theorem 20.8.** The suspension $\Sigma A$ of an $n$-connected type is $(n+1)$-connected.

**Theorem 20.9.** $S^n$ is $(n-1)$-connected.

---

## 5. The Freudenthal Suspension Theorem

**Theorem 20.10 (Freudenthal Suspension, HoTT version).** If $A$ is $n$-connected, then the natural map $A \to \Omega \Sigma A$ (the *unit* of the suspension-loop adjunction) is $2n$-connected.

**Corollary 20.11 (Stability of homotopy groups).** For $k \leq 2n - 1$ and any $n$-connected type $A$:
$$\pi_{k+1}(\Sigma A) \cong \pi_k(A)$$

**Example 20.12.** $S^n$ is $(n-1)$-connected, so for $k \leq 2(n-1) - 1 = 2n - 3$:
$$\pi_k(S^n) \cong \pi_{k+1}(S^{n+1})$$

The homotopy groups of spheres "stabilize" as $n \to \infty$. The *stable homotopy groups* $\pi_k^s = \lim_{n \to \infty} \pi_{n+k}(S^n)$ are the subject of stable homotopy theory.

**Proof sketch in HoTT:** Use the encode-decode method applied to the suspension HIT. The key step is showing that the fibers of the map $A \to \Omega \Sigma A$ are $2n$-connected, using the Blakers-Massey theorem.

---

## 6. The Blakers-Massey Theorem

**Theorem 20.13 (Blakers-Massey / Excision, HoTT version).** Let $A \sqcup_C B$ be a pushout with $f : C \to A$ being $m$-connected and $g : C \to B$ being $n$-connected. Then the natural map:
$$C \to A \times_{A \sqcup_C B} B \quad (\text{into the pullback})$$
is $(m + n)$-connected.

This is the key technical tool for proving Freudenthal and computing higher homotopy groups.

**History:** The first synthetic proof of Blakers-Massey in HoTT was given by Anel, Biedermann, Finster, and Joyal (2017) using the "orthogonal factorization system" approach. A second proof was given by Favonia and Shulman using the encode-decode method. Both proofs are entirely internal to HoTT.

---

## 7. The Hopf Fibration in HoTT

The Hopf fibration $S^1 \to S^3 \to S^2$ is a classical result in algebraic topology, providing the generator of $\pi_3(S^2) = \mathbb{Z}$. It can be constructed synthetically in HoTT.

### 7.1 The Join Construction

**Definition 20.14.** The *join* $A * B = A \sqcup_{A \times B} B$ (pushout along the two projections).

**Example:** $S^m * S^n \simeq S^{m+n+1}$. In particular, $S^1 * S^1 \simeq S^3$.

### 7.2 The Hopf Construction

**Theorem 20.15 (Hopf Construction in HoTT).** Given a *multiplication* $\mu : A \times A \to A$ on a type $A$ (satisfying left and right cancellation — a "H-space" structure), there is a fibration:
$$A \to A * A \xrightarrow{\mathsf{Hopf}_\mu} \Sigma A$$
with fiber $A$.

For $A = S^1$ (with multiplication given by the group structure of the integers $\pi_1(S^1) = \mathbb{Z}$):
- $A * A = S^1 * S^1 \simeq S^3$
- $\Sigma A = \Sigma S^1 \simeq S^2$
- Fiber $= S^1$

This gives the Hopf fibration $S^1 \to S^3 \to S^2$.

### 7.3 Applying the Long Exact Sequence

The Hopf fibration gives a long exact sequence:
$$\ldots \to \pi_3(S^1) \to \pi_3(S^3) \to \pi_3(S^2) \to \pi_2(S^1) \to \pi_2(S^3) \to \pi_2(S^2) \to \pi_1(S^1) \to \ldots$$

Using $\pi_k(S^1) = 0$ for $k \geq 2$ and $\pi_k(S^3) = 0$ for $k \leq 2$:
$$0 \to \mathbb{Z} \to \pi_3(S^2) \to 0$$

**Theorem 20.16.** $\pi_3(S^2) = \mathbb{Z}$, generated by the Hopf map.

---

## 8. Brunerie's Number

In 2016, Guillaume Brunerie completed a formalization in HoTT of the computation $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$. The proof was constructive: every step was formalized in a type theory, and the result was verified computationally.

The proof uses:
1. The Hopf fibration ($S^1 \to S^3 \to S^2$)
2. The Freudenthal suspension theorem
3. A cohomology computation (cup products in HoTT)
4. The James construction (a model of $\Omega \Sigma X$ as an infinite HIT)

The *Brunerie number* $n$ is defined as the element of $\pi_4(S^3)$ constructed in the proof. Brunerie conjectured $n = 2$ (the generator of $\mathbb{Z}/2\mathbb{Z}$), but the proof term was so large that it could only be verified by computer — not simplified by hand.

**Open problem:** Give a *simpler* proof of $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ in HoTT, one that could be read and verified by a human. This is an active research problem as of 2026.

---

## 9. What Makes Synthetic Homotopy Theory Powerful?

**1. No point-set overhead:** We never need to specify open sets, check continuity conditions, or verify that maps are measurable. Every function in HoTT is "automatically" continuous.

**2. Computational content:** Every proof of a homotopy group calculation is a program. The Brunerie computation is a specific term of a specific type — a witness with computational content.

**3. New proofs of old theorems:** The synthetic proofs of van Kampen, Freudenthal, and Blakers-Massey are genuinely new — they use different techniques from the classical proofs and illuminate different aspects of the mathematics.

**4. Machine verification:** Every result in this chapter has been (or could be) formalized in Agda or Lean, giving the highest possible level of certainty.

---

## Exercises

**20.1.** Carry out the encode-decode proof of $\pi_1(S^1) = \mathbb{Z}$ in Cubical Agda. Use the library's definition of the circle and the integers.

**20.2.** Compute $\pi_1(S^1 \vee S^1)$ using van Kampen's theorem in HoTT. Describe the presentation of the free group $F_2$ that arises.

**20.3.** Show that $S^2$ is simply connected: $\pi_1(S^2) = \mathbf{1}$. (*Hint:* Show that any loop on $S^2$ can be contracted; use the fact that $S^2$ is 1-connected.)

**20.4.** Using the Freudenthal suspension theorem, show that $\pi_3(S^2) \cong \pi_4(S^3)$ (the suspension isomorphism in the stable range).

**20.5.** Describe (at a high level) how the Hopf construction gives a fibration. What is the H-space structure on $S^1$ used in the construction?

**20.6.** Brunerie's number: look up Brunerie's thesis and describe (at a high level) the steps in his computation of $\pi_4(S^3)$. What is the James construction, and how does it appear?

**20.7 (Research).** Find and read one of the following papers, then write a 1-page summary:
  - Anel-Biedermann-Finster-Joyal, "Goodwillie's Calculus of Functors and Higher Topos Theory" (for Blakers-Massey)
  - van Doorn, "On the Formalization of Higher Inductive Types and Synthetic Homotopy Theory"
  - Favonia-Shulman, "A Mechanization of the Blakers-Massey Connectivity Theorem in HoTT"
