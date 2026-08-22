# Chapter 21 — Semisimple Lie Algebras and Root Systems

**Part V: Representation Theory**
*Prerequisites: [Chapter 20](ch20-lie-groups-algebras.md)*
*Next: [Chapter 22 — Highest Weight Theory](ch22-highest-weight-theory.md)*

---

## Learning Objectives

- Prove the structure theorem for semisimple Lie algebras via Cartan subalgebras
- Define root systems axiomatically; classify them via Dynkin diagrams
- Understand the root space decomposition and the $\mathfrak{sl}_2$ subalgebras
- Work with the Weyl group
- State and use the classification: $A_n, B_n, C_n, D_n$ + exceptional types
- Begin representation theory via roots and weights

---

## 21.1 Semisimple Lie Algebras

### 21.1.1 Definitions

A Lie algebra $\mathfrak{g}$ is:
- **Simple** if it is non-abelian and has no proper ideals
- **Semisimple** if it is a direct sum of simple Lie algebras
- Equivalently (Cartan's criterion): $\mathfrak{g}$ is semisimple $\Leftrightarrow$ the Killing form $B$ is non-degenerate

**Examples of simple Lie algebras:**
- $\mathfrak{sl}_n(\mathbb{C})$ for $n \geq 2$ (type $A_{n-1}$)
- $\mathfrak{so}_{2n+1}(\mathbb{C})$ (type $B_n$)
- $\mathfrak{sp}_{2n}(\mathbb{C})$ (type $C_n$)
- $\mathfrak{so}_{2n}(\mathbb{C})$ (type $D_n$)
- Exceptional types: $G_2, F_4, E_6, E_7, E_8$

### 21.1.2 Complete Reducibility

**Theorem (Weyl):** Every finite-dimensional representation of a semisimple Lie algebra $\mathfrak{g}$ over $\mathbb{C}$ is completely reducible.

Proof uses the Casimir element (from the Killing form) to construct a $\mathfrak{g}$-equivariant projection onto any submodule.

---

## 21.2 Cartan Subalgebras

### 21.2.1 Definition

A **Cartan subalgebra** (CSA) $\mathfrak{h} \subseteq \mathfrak{g}$ is a nilpotent self-normalizing subalgebra: $\mathfrak{h}$ nilpotent and $\{X \in \mathfrak{g} \mid [X, \mathfrak{h}] \subseteq \mathfrak{h}\} = \mathfrak{h}$.

**Key fact:** For semisimple $\mathfrak{g}$, CSAs are abelian and all CSAs are conjugate.

**Examples:**
- For $\mathfrak{sl}_n$: $\mathfrak{h} =$ diagonal matrices with trace 0
- For $\mathfrak{so}_{2n}$: $\mathfrak{h} =$ block diagonal $2 \times 2$ rotation generators

The **rank** of $\mathfrak{g}$ is $r = \dim \mathfrak{h}$.

### 21.2.2 Root Space Decomposition

Since $\mathfrak{h}$ is abelian and acts on $\mathfrak{g}$ via $\mathrm{ad}$, we can simultaneously diagonalize:
$$\mathfrak{g} = \mathfrak{h} \oplus \bigoplus_{\alpha \in \Phi} \mathfrak{g}_\alpha$$

where:
- $\Phi \subseteq \mathfrak{h}^*$ is the set of **roots** (nonzero eigenvalues of $\mathrm{ad}(\mathfrak{h})$)
- $\mathfrak{g}_\alpha = \{X \in \mathfrak{g} \mid [H, X] = \alpha(H) X \text{ for all } H \in \mathfrak{h}\}$ is the **root space**

**Properties:**
- Each $\mathfrak{g}_\alpha$ is one-dimensional (for semisimple $\mathfrak{g}$)
- $[\mathfrak{g}_\alpha, \mathfrak{g}_\beta] \subseteq \mathfrak{g}_{\alpha+\beta}$ (if $\alpha + \beta \in \Phi$, else 0)
- $\alpha \in \Phi \Rightarrow -\alpha \in \Phi$
- Roots come in $\pm$ pairs: $\{E_\alpha, E_{-\alpha}, H_\alpha\}$ form an $\mathfrak{sl}_2$ triple

---

## 21.3 $\mathfrak{sl}_2(\mathbb{C})$ and Its Representations

### 21.3.1 The Prototype

$$\mathfrak{sl}_2 = \mathrm{span}\left\{e = \begin{pmatrix}0&1\\0&0\end{pmatrix},\quad f = \begin{pmatrix}0&0\\1&0\end{pmatrix},\quad h = \begin{pmatrix}1&0\\0&-1\end{pmatrix}\right\}$$

Bracket relations: $[h, e] = 2e$, $[h, f] = -2f$, $[e, f] = h$.

**Finite-dimensional representations of $\mathfrak{sl}_2$:** Classified by a highest weight $\lambda \in \mathbb{Z}_{\geq 0}$. The irreducible representation $V_\lambda$ has dimension $\lambda + 1$, with basis $\{v_\lambda, v_{\lambda-2}, \ldots, v_{-\lambda}\}$ (the $h$-eigenspaces).

Action: $h \cdot v_\mu = \mu v_\mu$, $e \cdot v_{\lambda-2k} = (\lambda-k+1)v_{\lambda-2k+2}$, $f \cdot v_{\lambda-2k} = (k+1) v_{\lambda-2k-2}$.

Every finite-dimensional $\mathfrak{sl}_2$-module decomposes as $\bigoplus_\lambda V_\lambda^{\oplus m_\lambda}$.

### 21.3.2 $\mathfrak{sl}_2$-Triples in a Semisimple Algebra

For each root $\alpha$, the elements $e_\alpha \in \mathfrak{g}_\alpha$, $f_\alpha \in \mathfrak{g}_{-\alpha}$, $h_\alpha = [e_\alpha, f_\alpha] \in \mathfrak{h}$ satisfy the $\mathfrak{sl}_2$ relations (after normalization). The sub-algebra they span is isomorphic to $\mathfrak{sl}_2$.

This embeds $\mathfrak{sl}_2$ into $\mathfrak{g}$ for each root, and the $\mathfrak{sl}_2$ representation theory controls the structure of roots.

---

## 21.4 Root Systems

### 21.4.1 Axiomatic Definition

A **root system** is a finite subset $\Phi \subseteq V$ (a Euclidean space) satisfying:
1. $\Phi$ spans $V$ and $0 \notin \Phi$
2. $\alpha \in \Phi \Rightarrow -\alpha \in \Phi$
3. $\alpha \in \Phi$: the only multiples of $\alpha$ in $\Phi$ are $\pm \alpha$
4. **Closure:** For $\alpha, \beta \in \Phi$: $\beta - \langle \beta, \alpha^\vee \rangle \alpha \in \Phi$, where $\langle \beta, \alpha^\vee \rangle = 2(\beta, \alpha)/(\alpha, \alpha) \in \mathbb{Z}$

**Cartan integers:** $\langle \beta, \alpha^\vee \rangle \in \{0, \pm 1, \pm 2, \pm 3\}$

### 21.4.2 Simple Roots and Positive Roots

Choose a hyperplane not meeting $\Phi$; call roots on one side **positive** ($\Phi^+$), the others **negative**.

**Simple roots** $\Delta \subseteq \Phi^+$: the positive roots that cannot be written as sums of other positive roots.

$|\Delta| = \dim V =$ rank of the root system.

Every positive root is a non-negative integer linear combination of simple roots.

### 21.4.3 The Weyl Group

The **Weyl group** $W$ is generated by reflections $s_\alpha: v \mapsto v - \langle v, \alpha^\vee \rangle \alpha$ for $\alpha \in \Phi$.

$W$ is a finite group acting on $V$ that permutes $\Phi$.

**Examples:**
- $A_{n-1}$ root system: $W \cong S_n$ (symmetric group)
- $B_n, C_n$: $W \cong (\mathbb{Z}/2\mathbb{Z})^n \rtimes S_n$
- $D_n$: $W \cong (\mathbb{Z}/2\mathbb{Z})^{n-1} \rtimes S_n$

---

## 21.5 Dynkin Diagrams and Classification

### 21.5.1 The Cartan Matrix

The **Cartan matrix** $A_{ij} = \langle \alpha_i, \alpha_j^\vee \rangle = 2(\alpha_i, \alpha_j)/(\alpha_j, \alpha_j)$ for simple roots $\alpha_1, \ldots, \alpha_r$.

$A$ determines the root system (up to isomorphism).

### 21.5.2 Dynkin Diagrams

Draw one node per simple root. Connect nodes $i, j$ by $A_{ij} A_{ji}$ edges (0, 1, 2, or 3). Add an arrow pointing to the shorter root if they have different lengths.

The Dynkin diagram is a complete invariant of the root system.

### 21.5.3 Classification

**Theorem:** The connected Dynkin diagrams are exactly:

| Type | Algebra | Diagram |
|------|---------|---------|
| $A_n$ ($n \geq 1$) | $\mathfrak{sl}_{n+1}$ | Chain: $\circ - \circ - \cdots - \circ$ |
| $B_n$ ($n \geq 2$) | $\mathfrak{so}_{2n+1}$ | Chain with double bond at end: $\circ - \cdots - \circ \Rightarrow \circ$ |
| $C_n$ ($n \geq 3$) | $\mathfrak{sp}_{2n}$ | Chain with double bond reversed: $\circ - \cdots - \circ \Leftarrow \circ$ |
| $D_n$ ($n \geq 4$) | $\mathfrak{so}_{2n}$ | Chain with fork at end |
| $G_2$ | 14-dimensional exceptional | Triple bond |
| $F_4$ | 52-dimensional exceptional | |
| $E_6$ | 78-dimensional exceptional | |
| $E_7$ | 133-dimensional exceptional | |
| $E_8$ | 248-dimensional exceptional | |

This is the **Killing–Cartan classification** — one of the great theorems of 19th-century mathematics.

---

## 21.6 Weights and Weight Spaces

### 21.6.1 Weights of a Representation

For a representation $V$ of $\mathfrak{g}$ and $\lambda \in \mathfrak{h}^*$:
$$V_\lambda = \{v \in V \mid H \cdot v = \lambda(H) v \text{ for all } H \in \mathfrak{h}\}$$

If $V_\lambda \neq 0$, $\lambda$ is a **weight** and $V_\lambda$ is a **weight space**.

$V = \bigoplus_\lambda V_\lambda$ (decomposition into weight spaces, for semisimple $\mathfrak{g}$).

### 21.6.2 Dominant Weights

A weight $\lambda$ is **dominant** if $\langle \lambda, \alpha^\vee \rangle \geq 0$ for all simple roots $\alpha$.

**Theorem:** Each irreducible finite-dimensional representation of $\mathfrak{g}$ has a unique highest weight, and this weight is dominant integral.

The next chapter develops the complete theory.

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| Weyl's theorem | Semisimple Lie algebra $\Rightarrow$ complete reducibility |
| Root space decomposition | $\mathfrak{g} = \mathfrak{h} \oplus \bigoplus_\alpha \mathfrak{g}_\alpha$ |
| $\mathfrak{sl}_2$ classification | Irreducibles labeled by non-negative integers |
| Killing–Cartan classification | Simple Lie algebras are $A_n, B_n, C_n, D_n, E_6, E_7, E_8, F_4, G_2$ |

---

## Milestone Exercises

1. Compute the root system of $\mathfrak{sl}_3(\mathbb{C})$ explicitly. Draw it. Identify the simple roots, Cartan matrix, and Dynkin diagram.

2. Verify the $\mathfrak{sl}_2$ bracket relations for $e, f, h \in \mathfrak{sl}_2$.

3. Show that the Weyl group of $A_{n-1}$ is $S_n$ by identifying simple reflections with transpositions.

4. Prove: in a root system, if $\alpha, \beta \in \Phi$ with $\langle \alpha, \beta^\vee \rangle = -1$, then $\alpha + \beta \in \Phi$.

5. Let $\mathfrak{g} = \mathfrak{sl}_2$ and $V = V_3$ (the 4-dimensional representation). Compute the action of $e, f, h$ on a weight basis. Verify $[e,f] = h$ on each weight vector.

6. Determine the root system of $\mathfrak{so}_5 \cong \mathfrak{sp}_4$ (rank 2). It should be $B_2 = C_2$. Draw it.

7. Show $E_8$ has dimension 248 by computing $r + |\Phi|$ where $r = 8$ is the rank and $|\Phi| = 240$.

---

## Connections Forward

- **Chapter 22:** The Weyl character formula and highest weight theory give a complete classification of representations.
- **Chapter 23:** Kac-Moody algebras generalize to infinite-rank root systems; quantum groups $q$-deform the picture.

---

*Next: [Chapter 22 — Highest Weight Theory](ch22-highest-weight-theory.md)*
