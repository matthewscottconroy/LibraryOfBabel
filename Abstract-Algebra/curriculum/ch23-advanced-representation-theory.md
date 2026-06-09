# Chapter 23 — Advanced Topics in Representation Theory

**Part V: Representation Theory**
*Prerequisites: [Chapter 22](ch22-highest-weight-theory.md), [Chapter 16](ch16-derived-functors.md)*
*Next: [Chapter 24 — Axiomatic Set Theory](ch24-set-theory-logic.md)*

---

## Learning Objectives

- Understand modular representation theory and its differences from the characteristic 0 case
- Gain exposure to geometric representation theory: $D$-modules, perverse sheaves
- Understand quantum groups as $q$-deformations of enveloping algebras
- Gain exposure to the Langlands program as the unifying framework
- Understand algebraic groups as algebraic geometry meets representation theory

---

## 23.1 Modular Representation Theory

### 23.1.1 The Setting

Let $G$ be a finite group and $k$ a field with $\mathrm{char}(k) = p > 0$ where $p \mid |G|$.

Maschke's theorem fails: $k[G]$ is no longer semisimple. Representations need not be completely reducible.

### 23.1.2 Blocks and Defect Groups

The group algebra $k[G]$ decomposes into **blocks** (indecomposable two-sided ideals):
$$k[G] = B_1 \times B_2 \times \cdots \times B_r$$

Each block $B_i$ has an associated **defect group** $D_i \leq G$ (a $p$-subgroup); the representation theory of the block is governed by $D_i$.

- **Block of defect 0:** $D_i = \{e\}$; the block has a unique simple module
- **Block of full defect:** $D_i$ is a full Sylow $p$-subgroup; most complex

### 23.1.3 Projective Indecomposable Modules

In modular rep theory, the **projective indecomposable modules** (PIMs) replace the role of simple modules in the semisimple case. Each PIM has a unique simple **head** (top quotient) and a unique simple **socle** (bottom submodule).

The **decomposition matrix** $D = (d_{\lambda\mu})$ records:
$$[P(\mu) : L(\lambda)] = d_{\lambda\mu}$$

(multiplicity of simple $L(\lambda)$ as composition factor of PIM $P(\mu)$).

### 23.1.4 Brauer Theory

The **Brauer character** of a module $M$ records traces of $p'$-elements (elements with order coprime to $p$) — the analogue of ordinary characters.

**Brauer's theorem on blocks:** Simple modules in the same block share a common $p$-part of their central characters.

---

## 23.2 Geometric Representation Theory

### 23.2.1 The Philosophy

Many representation-theoretic objects (Verma modules, Kazhdan–Lusztig polynomials, etc.) have geometric realizations. The key dictionary:

| Algebra | Geometry |
|---------|----------|
| Representations of $\mathfrak{g}$ | Perverse sheaves on flag variety $G/B$ |
| Verma modules $M(\lambda)$ | Standard perverse sheaves $j_{!*}\mathcal{L}_\lambda$ |
| Kazhdan-Lusztig polynomials | Euler characteristics of intersection cohomology |
| Category $\mathcal{O}$ | Derived category of perverse sheaves |

### 23.2.2 The Flag Variety and Schubert Calculus

The **flag variety** $G/B$ (for a semisimple Lie group $G$ with Borel subgroup $B$) is an algebraic variety with a Schubert decomposition:
$$G/B = \bigsqcup_{w \in W} C_w \quad (C_w \cong \mathbb{A}^{\ell(w)})$$

The **Bruhat order** on $W$ controls the closure relations of Schubert cells.

The cohomology ring $H^*(G/B, \mathbb{Z})$ is computed by Schubert calculus — related to symmetric function theory.

### 23.2.3 $D$-Modules and the Beilinson–Bernstein Theorem

**$D$-modules** are sheaves of modules over the sheaf $\mathcal{D}_X$ of differential operators on a smooth variety $X$.

**Theorem (Beilinson–Bernstein, 1981):** For dominant $\lambda \in \mathfrak{h}^*$:
$$\left\{\text{representations of } \mathfrak{g} \text{ with generalized central char. } \lambda\right\} \cong \left\{\mathcal{D}_\lambda\text{-modules on } G/B\right\}$$

This realizes $\mathfrak{g}$-modules geometrically, and the Riemann–Hilbert correspondence connects them to perverse sheaves.

### 23.2.4 Kazhdan–Lusztig Theory

The **Kazhdan–Lusztig polynomials** $P_{x,w}(q)$ (for $x, w \in W$) control:
- The multiplicities of simples in Verma modules (KL conjecture, proved by Beilinson-Bernstein and Brylinski-Kashiwara, 1981)
- The topology of Schubert varieties
- Characters of representations in characteristic $p$ (modular KL theory)

---

## 23.3 Algebraic Groups

### 23.3.1 Definition

An **algebraic group** over a field $k$ is an affine algebraic variety $G$ that is also a group, with multiplication and inversion morphisms of varieties.

**Examples:** $GL_n, SL_n, O_n, Sp_{2n}$ over any field $k$.

**Advantage over Lie groups:** Works over arbitrary fields, including characteristic $p$.

### 23.3.2 Representations of Algebraic Groups

A **rational representation** is a group homomorphism $G \to GL(V)$ that is a morphism of varieties.

The representation theory of algebraic groups (especially reductive groups like $GL_n, SL_n$) over algebraically closed fields is rich, combining Lie theory with algebraic geometry.

**Steinberg's tensor product theorem:** In characteristic $p$, every irreducible $G$-module is a tensor product of "Frobenius twists" of restricted representations.

---

## 23.4 Quantum Groups

### 23.4.1 Definition

For $q \in k^*$ (not a root of unity), the **quantum group** $U_q(\mathfrak{g})$ is a $q$-deformation of $\mathcal{U}(\mathfrak{g})$:

Replace Serre relations with $q$-analogues. For $\mathfrak{sl}_2$:
$$[H, E] = 2E, \quad [H, F] = -2F, \quad [E, F] = \frac{q^H - q^{-H}}{q - q^{-1}}$$

As $q \to 1$: $\frac{q^H - q^{-H}}{q - q^{-1}} \to H$, recovering $\mathcal{U}(\mathfrak{sl}_2)$.

### 23.4.2 Properties

- $U_q(\mathfrak{g})$ is a **Hopf algebra** (has comultiplication, counit, antipode — the proper algebraic framework for "groups in the tensor category sense")
- Representation theory parallels the classical case for generic $q$
- At roots of unity ($q^N = 1$): modular-like phenomena appear; relates to representations in characteristic $p$

### 23.4.3 Applications

- **Knot and link invariants:** Jones polynomial, HOMFLY polynomial
- **Topological quantum field theory**
- **Categorification:** Khovanov homology categorifies the Jones polynomial (raises a numerical invariant to a homological invariant)

---

## 23.5 The Langlands Program (Overview)

### 23.5.1 The Unifying Vision

The **Langlands program** is a vast web of conjectures relating:

| Analytic side | Algebraic/Geometric side |
|--------------|--------------------------|
| Automorphic forms | Galois representations |
| $L$-functions | Motives |
| Representations of $p$-adic groups | Arithmetic geometry |

### 23.5.2 Local Langlands (for $GL_n$)

The **local Langlands correspondence** for $GL_n(\mathbb{Q}_p)$ (proved by Harris–Taylor, 2001):

$$\left\{\text{irred. smooth reps of } GL_n(\mathbb{Q}_p)\right\} \longleftrightarrow \left\{n\text{-dim. reps of } \mathrm{Gal}(\bar{\mathbb{Q}}_p/\mathbb{Q}_p)\right\}$$

### 23.5.3 Geometric Langlands

Replaces groups over number fields with groups over curves over $\mathbb{F}_q$ (or $\mathbb{C}$):

$$\left\{D\text{-modules on } \mathrm{Bun}_G(C)\right\} \longleftrightarrow \left\{\text{local systems on } C \text{ for } {}^L G\right\}$$

where ${}^L G$ is the Langlands dual group and $\mathrm{Bun}_G(C)$ is the moduli stack of $G$-bundles on a curve $C$.

---

## Milestone Exercises

1. Let $k = \mathbb{F}_p$ and $G = \mathbb{Z}/p\mathbb{Z}$. Show $k[G] \cong k[x]/(x^p)$ is not semisimple. Find all indecomposable modules.

2. Describe the Schubert decomposition of $G/B$ for $G = SL_3(\mathbb{C})$ (the complete flag variety $\mathrm{Fl}(3)$). Count Schubert cells and compute their dimensions.

3. Compute the Kazhdan-Lusztig polynomials $P_{e,w}$ for $W = S_3$ (all $w \in S_3$). They should all equal 1 for $S_3$.

4. Show that $U_q(\mathfrak{sl}_2)$ defined by the $q$-relations has the same irreducible representations (for generic $q$) as $\mathcal{U}(\mathfrak{sl}_2)$.

5. Research: What is the role of the exceptional group $E_8$ in string theory and the Langlands program? Write a 1-page summary of the connections.

6. For $GL_2(\mathbb{F}_p)$: find all irreducible representations and their dimensions.

---

## Connections to Research

The topics in this chapter are at the frontier. Research directions include:

- **Geometric Langlands with quantum parameters** (Frenkel, Ben-Zvi, Nadler)
- **$p$-adic Langlands** (Breuil, Colmez, Emerton)
- **Categorification** (Khovanov, Lauda, Rouquier)
- **Canonical bases and crystal bases** (Kashiwara, Lusztig)
- **Derived algebraic geometry** approach to Langlands (Ben-Zvi, Nadler, Gaitsgory)

---

*Next: [Chapter 24 — Axiomatic Set Theory](ch24-set-theory-logic.md)*
