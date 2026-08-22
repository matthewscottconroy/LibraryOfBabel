# Chapter 15 — Complexes, Homology, and Exact Sequences

**Part IV: Homological Algebra**
*Prerequisites: [Chapter 12](ch12-module-theory.md), [Chapter 14](ch14-category-theory.md)*
*Next: [Chapter 16 — Derived Functors: Ext and Tor](ch16-derived-functors.md)*

---

## Learning Objectives

- Define chain complexes and their homology
- Understand short and long exact sequences; the connecting homomorphism
- Prove the snake lemma and the five lemma; apply diagram chasing
- Compute homology in simple examples
- Understand the connection between exact sequences and module extensions
- Build the conceptual framework for derived functors

---

## 15.1 Chain Complexes

### 15.1.1 Definition

A **chain complex** $(C_\bullet, d_\bullet)$ of $R$-modules is a sequence:
$$\cdots \xrightarrow{d_{n+1}} C_n \xrightarrow{d_n} C_{n-1} \xrightarrow{d_{n-1}} \cdots$$

where the **boundary maps** $d_n: C_n \to C_{n-1}$ satisfy:
$$d_{n-1} \circ d_n = 0 \quad \text{for all } n$$

Equivalently: $\mathrm{im}(d_{n+1}) \subseteq \ker(d_n)$.

**Cochain complex** $(C^\bullet, d^\bullet)$: arrows go in the other direction, $d^n: C^n \to C^{n+1}$, and $d^{n+1} \circ d^n = 0$.

### 15.1.2 Homology

The **$n$-th homology module**:
$$H_n(C_\bullet) = \ker(d_n) / \mathrm{im}(d_{n+1})$$

- Elements of $\ker d_n$ are **$n$-cycles**
- Elements of $\mathrm{im}\, d_{n+1}$ are **$n$-boundaries**
- $H_n = 0$ means every cycle is a boundary — the sequence is exact at $C_n$

**Cohomology**: $H^n(C^\bullet) = \ker(d^n)/\mathrm{im}(d^{n-1})$.

### 15.1.3 Examples

**Exact sequences:** A chain complex with all $H_n = 0$ is called an **exact sequence**.

**Free resolution of $\mathbb{Z}/n\mathbb{Z}$:**
$$0 \to \mathbb{Z} \xrightarrow{\times n} \mathbb{Z} \xrightarrow{\pi} \mathbb{Z}/n\mathbb{Z} \to 0$$
This is a short exact sequence; $H_0 = \mathbb{Z}/n\mathbb{Z}$, $H_k = 0$ for $k \neq 0$.

**Simplicial homology:** A simplicial complex gives a chain complex with $C_n =$ free module on $n$-simplices and $d_n =$ signed sum of faces. $H_n$ measures $n$-dimensional "holes."

---

## 15.2 Exact Sequences

### 15.2.1 Short Exact Sequences

A **short exact sequence** (SES):
$$0 \to A \xrightarrow{f} B \xrightarrow{g} C \to 0$$

means: $f$ is injective, $\mathrm{im}\, f = \ker g$, and $g$ is surjective.

Equivalently: $A \cong \ker g$ (embedded in $B$) and $C \cong B/A$.

**Splitting:** The SES **splits** if $B \cong A \oplus C$, i.e., there exists $s: C \to B$ with $g \circ s = \mathrm{id}_C$ (right splitting) or $r: B \to A$ with $r \circ f = \mathrm{id}_A$ (left splitting).

**Non-split example:** $0 \to \mathbb{Z} \xrightarrow{\times 2} \mathbb{Z} \to \mathbb{Z}/2\mathbb{Z} \to 0$ does not split (no section $\mathbb{Z}/2\mathbb{Z} \to \mathbb{Z}$).

### 15.2.2 Long Exact Sequences

The fundamental machine of homological algebra. A SES of chain complexes:
$$0 \to A_\bullet \to B_\bullet \to C_\bullet \to 0$$

(meaning exact at each degree) induces a **long exact sequence in homology**:

$$\cdots \to H_n(A_\bullet) \to H_n(B_\bullet) \to H_n(C_\bullet) \xrightarrow{\partial_n} H_{n-1}(A_\bullet) \to \cdots$$

The maps $\partial_n: H_n(C) \to H_{n-1}(A)$ are **connecting homomorphisms**, constructed by a diagram chase.

This long exact sequence is the central computational tool in homological algebra and topology.

---

## 15.3 Diagram Lemmas

### 15.3.1 The Snake Lemma

Given a commutative diagram with exact rows:

$$\begin{array}{ccccccc}
& A & \xrightarrow{f} & B & \xrightarrow{g} & C & \to 0 \\
& \downarrow\alpha & & \downarrow\beta & & \downarrow\gamma & \\
0 \to & A' & \xrightarrow{f'} & B' & \xrightarrow{g'} & C' &
\end{array}$$

There is an exact sequence:
$$\ker\alpha \to \ker\beta \to \ker\gamma \xrightarrow{\delta} \mathrm{coker}\,\alpha \to \mathrm{coker}\,\beta \to \mathrm{coker}\,\gamma$$

The connecting map $\delta$ is defined by the **snake**: given $c \in \ker\gamma$, lift to $b \in B$, map to $B'$ via $\beta$, pull back to $A'$ via exactness, project to $\mathrm{coker}\,\alpha$.

**Corollary:** Every short exact sequence of chain complexes gives a long exact sequence in homology (the connecting homomorphism is the snake).

### 15.3.2 The Five Lemma

Given a commutative diagram with exact rows:
$$A_1 \to A_2 \to A_3 \to A_4 \to A_5$$
$$\downarrow\alpha_1\quad\downarrow\alpha_2\quad\downarrow\alpha_3\quad\downarrow\alpha_4\quad\downarrow\alpha_5$$
$$B_1 \to B_2 \to B_3 \to B_4 \to B_5$$

If $\alpha_1, \alpha_2, \alpha_4, \alpha_5$ are isomorphisms, then so is $\alpha_3$.

More precisely: $\alpha_1$ epi + $\alpha_2, \alpha_4$ iso + $\alpha_5$ mono $\Rightarrow$ $\alpha_3$ iso.

### 15.3.3 The Four Lemma and the Short Five Lemma

**Short five lemma:** For commutative diagram with exact rows:
$$0 \to A \to B \to C \to 0$$
If $\alpha: A \to A'$ and $\gamma: C \to C'$ are isomorphisms, so is $\beta: B \to B'$.

---

## 15.4 The Horseshoe and Comparison Lemmas

### 15.4.1 Projective and Injective Resolutions

A **projective resolution** of $M$:
$$\cdots \to P_2 \xrightarrow{d_2} P_1 \xrightarrow{d_1} P_0 \xrightarrow{\varepsilon} M \to 0$$

where each $P_i$ is projective and the sequence is exact.

**Existence:** Every module has a projective resolution (use the free module surjecting onto $M$; take kernel; iterate).

An **injective resolution** $0 \to M \to I^0 \to I^1 \to \cdots$ is defined dually.

### 15.4.2 Comparison Theorem

Any map $f: M \to N$ lifts to a chain map between any two projective resolutions of $M$ and $N$, and any two such liftings are chain homotopic.

This is what makes derived functors well-defined: they don't depend on the choice of resolution.

---

## 15.5 Extensions

### 15.5.1 Module Extensions

An **extension** of $C$ by $A$ is a short exact sequence $0 \to A \to B \to C \to 0$.

Two extensions are **equivalent** if there is a commutative diagram:
$$\begin{array}{ccccc}
0 \to A \to B \to C \to 0\\
\quad\quad\quad\parallel\quad\downarrow\sim\quad\parallel\\
0 \to A \to B' \to C \to 0
\end{array}$$

The set of equivalence classes of extensions of $C$ by $A$ forms an abelian group $\mathrm{Ext}^1_R(C, A)$ — the first Ext group (properly defined via derived functors in Chapter 16).

The split extensions correspond to the zero element.

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| Long exact sequence | SES of complexes $\Rightarrow$ LES in homology (with connecting maps) |
| Snake lemma | Connecting morphism $\ker\gamma \to \mathrm{coker}\,\alpha$ |
| Five lemma | 4 isos force the middle iso |
| Comparison theorem | Liftings of maps to resolutions exist and are unique up to chain homotopy |
| Existence of resolutions | Every module has projective and injective resolutions |

---

## Milestone Exercises

1. Construct a projective resolution of $\mathbb{Z}/6\mathbb{Z}$ as a $\mathbb{Z}$-module.

2. Prove the snake lemma by explicit diagram chase.

3. Let $0 \to A \xrightarrow{f} B \xrightarrow{g} C \to 0$ be a short exact sequence of abelian groups. Show this splits iff $g$ has a right inverse.

4. Compute the homology of the complex: $\mathbb{Z} \xrightarrow{\times 4} \mathbb{Z} \xrightarrow{\times 6} \mathbb{Z}$.

5. Show that $H_n$ is a functor from the category of chain complexes to $R$-modules.

6. Prove the five lemma from the four lemma.

7. Show that the extension $0 \to \mathbb{Z} \to \mathbb{Z} \oplus \mathbb{Z}/2\mathbb{Z} \to \mathbb{Z}/2\mathbb{Z} \to 0$ splits, and find two non-isomorphic extensions of $\mathbb{Z}/2\mathbb{Z}$ by $\mathbb{Z}$.

---

## Connections Forward

- **Chapter 16:** Ext and Tor are defined as derived functors of Hom and Tensor; long exact sequences in Ext and Tor are the derived version of the LES above.
- **Chapter 17:** Spectral sequences are a tool to compute homology of double complexes — iterated long exact sequences.
- **Chapter 18:** The cohomology of a group $G$ with coefficients in an $G$-module $M$ is $H^n(G, M) = \mathrm{Ext}^n_{\mathbb{Z}[G]}(\mathbb{Z}, M)$.

---

*Next: [Chapter 16 — Derived Functors: Ext and Tor](ch16-derived-functors.md)*
