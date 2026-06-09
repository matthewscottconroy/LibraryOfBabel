# Chapter 17 — Spectral Sequences

**Part IV: Homological Algebra**
*Prerequisites: [Chapter 16](ch16-derived-functors.md)*
*Next: [Chapter 18 — Representations of Finite Groups](ch18-representations-finite-groups.md)*

---

## Learning Objectives

- Understand spectral sequences as a tool for organizing iterated homological computation
- Define the $E_r$ pages and the differentials $d_r$
- Understand convergence: what "$E_r \Rightarrow H$" means
- Compute with the Serre spectral sequence and the Lyndon–Hochschild–Serre spectral sequence
- Apply spectral sequences to compute group cohomology and extension groups
- Develop comfort with the "spectral sequence as machine" perspective

---

## 17.1 The Idea

### 17.1.1 Motivation

A spectral sequence arises when you want to compute the homology of a complex that has "two directions." For example:

- Cohomology of a fiber bundle (Serre): $E^{p,q}_2 = H^p(B, H^q(F)) \Rightarrow H^{p+q}(E)$
- Group cohomology of an extension: $E^{p,q}_2 = H^p(G/N, H^q(N, M)) \Rightarrow H^{p+q}(G, M)$
- Hypercohomology of a double complex

The spectral sequence is a machine: you input "easy" data (the $E_2$ page), run the machine (successive differentials), and output the "hard" data (the graded pieces of the answer).

### 17.1.2 The Basic Setup

A **spectral sequence** (first quadrant, cohomological) is a sequence of **pages** $\{(E_r^{p,q}, d_r)\}_{r \geq r_0}$ where:

- Each $E_r^{p,q}$ is an abelian group (or module), bigraded by $(p,q)$
- Each $d_r: E_r^{p,q} \to E_r^{p+r, q-r+1}$ is a differential: $d_r \circ d_r = 0$
- The next page is the cohomology of the current page: $E_{r+1}^{p,q} = \ker(d_r^{p,q})/\mathrm{im}(d_r^{p-r, q+r-1})$

After enough pages, the differentials become zero and the pages stabilize: $E_\infty^{p,q}$.

---

## 17.2 Double Complexes

### 17.2.1 Definition

A **double complex** $C^{\bullet,\bullet}$ is a bigraded collection of modules $C^{p,q}$ with horizontal differentials $d_h: C^{p,q} \to C^{p+1,q}$ and vertical differentials $d_v: C^{p,q} \to C^{p,q+1}$ satisfying:
$$d_h^2 = 0, \quad d_v^2 = 0, \quad d_h d_v + d_v d_h = 0$$

The **total complex** $\mathrm{Tot}(C)^n = \bigoplus_{p+q=n} C^{p,q}$ with differential $d = d_h + d_v$.

### 17.2.2 Two Spectral Sequences

A double complex gives two spectral sequences:

**Horizontal first:** Filter by $p$. Page $E_1^{p,q} = H^q_v(C^{p,\bullet})$ (vertical cohomology). Page $E_2^{p,q} = H^p_h(H^q_v(C))$.

**Vertical first:** Filter by $q$. Page $E_1^{p,q} = H^p_h(C^{\bullet,q})$.

Both converge to $H^{p+q}(\mathrm{Tot}(C))$ under appropriate boundedness conditions.

---

## 17.3 Convergence

### 17.3.1 What Convergence Means

"$E_r^{p,q} \Rightarrow H^{p+q}$" means:

1. The pages stabilize: $E_\infty^{p,q} = E_r^{p,q}$ for all $r \gg 0$
2. There is a filtration $\cdots \subseteq F^{p+1}H^n \subseteq F^p H^n \subseteq \cdots$ of $H^n$
3. $E_\infty^{p,q} \cong F^p H^{p+q} / F^{p+1} H^{p+q}$ (the $\infty$-page is the associated graded)

**Warning:** Convergence gives the associated graded of $H$, not $H$ itself. The extension problems (how pieces $E_\infty^{p,q}$ assemble into $H^{p+q}$) can be non-trivial. A spectral sequence "degenerates at $E_2$" when all $d_r = 0$ for $r \geq 2$, making computation easier.

---

## 17.4 The Lyndon–Hochschild–Serre Spectral Sequence

### 17.4.1 Setup

Let $1 \to N \to G \to Q \to 1$ be a short exact sequence of groups and $M$ a $G$-module.

**LHS spectral sequence:**
$$E_2^{p,q} = H^p(Q, H^q(N, M)) \Rightarrow H^{p+q}(G, M)$$

### 17.4.2 Low-Degree Terms (5-Term Exact Sequence)

From the spectral sequence, extract:
$$0 \to H^1(Q, M^N) \to H^1(G, M) \to H^1(N, M)^Q \to H^2(Q, M^N) \to H^2(G, M)$$

This is the **inflation-restriction exact sequence** in group cohomology.

### 17.4.3 Applications

- Computing $H^n(G, M)$ for extensions where $N$ and $Q$ have simpler cohomology
- Proving the Hochschild–Serre formula for Lie algebras (parallel construction)
- Spectral sequence for a normal subgroup: $H^n(G/N, M^N) \to H^n(G, M) \to H^n(N, M)^{G/N}$

---

## 17.5 Reading a Spectral Sequence

### 17.5.1 The $E_2$ Page as a Grid

Place $E_2^{p,q}$ at position $(p,q)$ in a grid. Differentials $d_2$ go right 2 and down 1 (cohomological). The $E_3$ page has differentials $d_3$ going right 3 and down 2. And so on.

**Degenerate cases:**
- Only one nonzero row: $E_2^{p,q} = 0$ for $q \neq 0$. The spectral sequence collapses; $E_2 = E_\infty$.
- Only one nonzero column: similarly.
- Both conditions: conclude $H^n \cong E_2^{n,0}$ or $E_2^{0,n}$.

### 17.5.2 Edge Homomorphisms

There are canonical **edge homomorphisms**:
$$H^p(\text{base}) = E_2^{p,0} \twoheadrightarrow E_\infty^{p,0} \hookrightarrow H^p(\text{total})$$
$$H^q(\text{fiber}) = E_2^{0,q} \twoheadrightarrow E_\infty^{0,q} \hookrightarrow H^q(\text{total})$$

---

## 17.6 The Künneth Formula via Spectral Sequences

**Künneth theorem:** For chain complexes $A_\bullet, B_\bullet$ of free modules:
$$H_n(A \otimes B) \cong \bigoplus_{p+q=n} H_p(A) \otimes H_q(B)$$

More generally (with Tor correction):
$$0 \to \bigoplus_{p+q=n} H_p(A) \otimes H_q(B) \to H_n(A \otimes B) \to \bigoplus_{p+q=n-1} \mathrm{Tor}_1(H_p(A), H_q(B)) \to 0$$

This is the prototype for spectral sequence computations: the $E_2$ page involves tensor products of homologies; correction terms involve Tor; the differential encodes the extension problem.

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| LHS spectral sequence | $H^p(Q, H^q(N,M)) \Rightarrow H^{p+q}(G,M)$ |
| Inflation-restriction | 5-term exact sequence from LHS $E_2$ low-degree terms |
| Künneth formula | $H_n(A \otimes B)$ in terms of $H_*(A) \otimes H_*(B)$ and Tor |
| Degeneration | $E_2 = E_\infty$ when all $d_r = 0$ for $r \geq 2$ |

---

## Milestone Exercises

1. Let $G = \mathbb{Z}/p\mathbb{Z}$ act trivially on $M = \mathbb{Z}$. Use the LHS spectral sequence with $N = \{e\}$ to recover $H^n(\mathbb{Z}/p\mathbb{Z}, \mathbb{Z})$.

2. Use the spectral sequence for $1 \to \mathbb{Z} \to \mathbb{Z} \to \mathbb{Z}/n\mathbb{Z} \to 1$ and $M = \mathbb{Z}$ to compute $H^*(\mathbb{Z}/n\mathbb{Z}, \mathbb{Z})$.

3. A spectral sequence has $E_2^{p,q} = 0$ unless $p \in \{0,2\}$ and $q \in \{0,1\}$, with $E_2^{0,0} = E_2^{0,1} = E_2^{2,0} = E_2^{2,1} = \mathbb{Z}$. What are the possible values of $H^n(\mathrm{Tot})$?

4. Show that if $E_2^{p,q} = H^p(B) \otimes H^q(F)$ (cohomology of base $\otimes$ fiber) and the spectral sequence degenerates, then $H^n(E) \cong \bigoplus_{p+q=n} H^p(B) \otimes H^q(F)$.

5. Use the 5-term exact sequence to analyze $H^1$ and $H^2$ for a central extension $1 \to \mathbb{Z}/p\mathbb{Z} \to G \to \mathbb{Z}/p\mathbb{Z} \to 1$.

---

## Connections Forward

- **Chapter 18:** Group cohomology used to classify projective representations and extension problems.
- **Chapter 22:** The Hochschild–Serre spectral sequence for Lie algebras: $H^p(\mathfrak{g}/\mathfrak{h}, H^q(\mathfrak{h}, M)) \Rightarrow H^{p+q}(\mathfrak{g}, M)$.
- **Chapter 27:** In $\infty$-categorical or derived algebraic geometry, spectral sequences are replaced by spectral sequence methods in $\infty$-categories (homotopy limits and colimits).

---

*Next: [Chapter 18 — Representations of Finite Groups](ch18-representations-finite-groups.md)*
