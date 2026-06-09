# Chapter 16 — Derived Functors: Ext and Tor

**Part IV: Homological Algebra**
*Prerequisites: [Chapter 15](ch15-homological-algebra-basics.md)*
*Next: [Chapter 17 — Spectral Sequences](ch17-spectral-sequences.md)*

---

## Learning Objectives

- Define derived functors via resolutions; understand independence of resolution choice
- Compute $\mathrm{Ext}^n_R(M,N)$ and $\mathrm{Tor}_n^R(M,N)$
- Understand long exact sequences in Ext and Tor
- Interpret $\mathrm{Ext}^1$ as classifying extensions; $\mathrm{Ext}^0 = \mathrm{Hom}$
- Apply Ext and Tor to detect projective, injective, and flat modules
- Understand group cohomology as a special case

---

## 16.1 Derived Functors: The Idea

### 16.1.1 The Problem

$\mathrm{Hom}_R(M, -)$ is left exact: $0 \to A \to B \to C \to 0$ gives $0 \to \mathrm{Hom}(M,A) \to \mathrm{Hom}(M,B) \to \mathrm{Hom}(M,C)$, but the last map need not be surjective.

The **derived functor** $\mathrm{Ext}^n_R(M, -)$ measures exactly this failure, and extends the sequence to the right.

Similarly, $M \otimes_R -$ is right exact; $\mathrm{Tor}_n^R(M, -)$ measures failure of left exactness.

### 16.1.2 The Construction

To compute $R^n F(M)$ (the $n$-th right derived functor of a left-exact functor $F$):

1. Choose an **injective resolution** of $M$: $0 \to M \to I^0 \to I^1 \to I^2 \to \cdots$
2. Apply $F$ to the truncated complex (drop $M$): $0 \to F(I^0) \to F(I^1) \to F(I^2) \to \cdots$
3. Take cohomology: $R^n F(M) = H^n(F(I^\bullet))$

This is independent of the choice of injective resolution (by the comparison theorem of Chapter 15).

Dually, **left derived functors** $L_n F(M)$ use projective resolutions and apply $F$, then take homology.

---

## 16.2 Ext Groups

### 16.2.1 Definition

$$\mathrm{Ext}^n_R(M, N) = R^n \mathrm{Hom}_R(M, -)(N)$$

computed using an injective resolution of $N$. Equivalently (by a symmetry theorem):

$$\mathrm{Ext}^n_R(M, N) = H^n(\mathrm{Hom}_R(P_\bullet, N))$$

where $P_\bullet \to M$ is a projective resolution of $M$ (and we apply $\mathrm{Hom}_R(-, N)$).

Both computations give the same groups — this is a non-trivial symmetry of the theory.

### 16.2.2 Low-Degree Interpretations

- $\mathrm{Ext}^0_R(M, N) \cong \mathrm{Hom}_R(M, N)$
- $\mathrm{Ext}^1_R(M, N) \cong $ {equivalence classes of SES $0 \to N \to E \to M \to 0$} / ${\sim}$

The second identification is the Baer sum construction: extensions are a group under a "stacking" operation.

### 16.2.3 Computing Ext over $\mathbb{Z}$

**Projective resolution of $\mathbb{Z}/n\mathbb{Z}$:**
$$0 \to \mathbb{Z} \xrightarrow{\times n} \mathbb{Z} \to \mathbb{Z}/n\mathbb{Z} \to 0$$

Apply $\mathrm{Hom}_\mathbb{Z}(-, \mathbb{Z}/m\mathbb{Z})$:
$$0 \to \mathrm{Hom}(\mathbb{Z}/n\mathbb{Z}, \mathbb{Z}/m\mathbb{Z}) \to \mathrm{Hom}(\mathbb{Z}, \mathbb{Z}/m\mathbb{Z}) \xrightarrow{\times n} \mathrm{Hom}(\mathbb{Z}, \mathbb{Z}/m\mathbb{Z}) \to 0$$
$$= 0 \to \mathbb{Z}/\gcd(m,n)\mathbb{Z} \to \mathbb{Z}/m\mathbb{Z} \xrightarrow{\times n} \mathbb{Z}/m\mathbb{Z} \to 0$$

So: $\mathrm{Ext}^0(\mathbb{Z}/n, \mathbb{Z}/m) = \mathbb{Z}/\gcd(m,n)\mathbb{Z}$ and $\mathrm{Ext}^1(\mathbb{Z}/n, \mathbb{Z}/m) = \mathbb{Z}/\gcd(m,n)\mathbb{Z}$.

### 16.2.4 Long Exact Sequences in Ext

A SES $0 \to A \to B \to C \to 0$ gives:
$$\cdots \to \mathrm{Ext}^{n-1}(M,C) \to \mathrm{Ext}^n(M,A) \to \mathrm{Ext}^n(M,B) \to \mathrm{Ext}^n(M,C) \to \cdots$$

and similarly in the first argument:
$$\cdots \to \mathrm{Ext}^n(C,N) \to \mathrm{Ext}^n(B,N) \to \mathrm{Ext}^n(A,N) \to \mathrm{Ext}^{n+1}(C,N) \to \cdots$$

### 16.2.5 Ext and Projectivity

- $M$ is projective $\Leftrightarrow$ $\mathrm{Ext}^n_R(M, N) = 0$ for all $n \geq 1$ and all $N$
- $N$ is injective $\Leftrightarrow$ $\mathrm{Ext}^n_R(M, N) = 0$ for all $n \geq 1$ and all $M$

**Projective dimension** of $M$: $\mathrm{pd}(M) = \min\{n \mid \mathrm{Ext}^{n+1}(M, -) = 0\}$.

**Global dimension** of $R$: $\mathrm{gl.dim}(R) = \sup_M \mathrm{pd}(M)$.

---

## 16.3 Tor Groups

### 16.3.1 Definition

$$\mathrm{Tor}_n^R(M, N) = L_n(M \otimes_R -)(N)$$

computed using a projective resolution of $M$ (or $N$ — the result is the same):
$$\mathrm{Tor}_n^R(M, N) = H_n(P_\bullet \otimes_R N)$$

where $P_\bullet \to M$ is a projective resolution.

### 16.3.2 Low-Degree Interpretations

- $\mathrm{Tor}_0^R(M, N) = M \otimes_R N$
- $\mathrm{Tor}_1^R(M, N)$ measures "torsion obstruction" to flatness

### 16.3.3 Computing Tor over $\mathbb{Z}$

Using the same resolution of $\mathbb{Z}/n\mathbb{Z}$, apply $- \otimes_\mathbb{Z} \mathbb{Z}/m\mathbb{Z}$:
$$\mathbb{Z}/m\mathbb{Z} \xrightarrow{\times n} \mathbb{Z}/m\mathbb{Z}$$

Taking homology:
- $\mathrm{Tor}_0(\mathbb{Z}/n, \mathbb{Z}/m) = \mathbb{Z}/\gcd(m,n)\mathbb{Z}$ (= $M \otimes N$)
- $\mathrm{Tor}_1(\mathbb{Z}/n, \mathbb{Z}/m) = \mathbb{Z}/\gcd(m,n)\mathbb{Z}$
- $\mathrm{Tor}_k = 0$ for $k \geq 2$ (projective dimension of $\mathbb{Z}/n$ over $\mathbb{Z}$ is 1)

### 16.3.4 Flatness via Tor

- $M$ is flat $\Leftrightarrow$ $\mathrm{Tor}_n^R(M, N) = 0$ for all $n \geq 1$ and all $N$
- $M$ is flat $\Leftrightarrow$ $\mathrm{Tor}_1^R(M, N) = 0$ for all $N$ (suffices to check degree 1)

---

## 16.4 Group Cohomology

### 16.4.1 The Group Ring

For a group $G$ and ring $R$, the **group ring** $R[G]$ is the free $R$-module on elements of $G$ with multiplication extended from $G$.

$R[G]$-modules = $R$-modules with a $G$-action (compatible with $R$-linearity).

### 16.4.2 Group Cohomology

For a $G$-module $M$ (i.e., $\mathbb{Z}[G]$-module), the **group cohomology**:
$$H^n(G, M) = \mathrm{Ext}^n_{\mathbb{Z}[G]}(\mathbb{Z}, M)$$

where $\mathbb{Z}$ is the trivial $G$-module.

**Low degrees:**
- $H^0(G, M) = M^G = \{m \in M \mid gm = m \text{ for all } g\}$ (invariants)
- $H^1(G, M) = $ group homomorphisms $G \to M$ (when $M$ abelian, trivial action) / crossed homomorphisms
- $H^2(G, M) = $ extensions of $G$ by $M$ (group extensions!)

### 16.4.3 Bar Resolution

A standard projective resolution of $\mathbb{Z}$ over $\mathbb{Z}[G]$ — the **bar complex** — gives explicit cocycle/coboundary descriptions of $H^n(G,M)$.

**Applications:**
- Galois cohomology: $H^n(\mathrm{Gal}(L/K), L^*)$ classifies field-theoretic objects (Hilbert 90, Brauer groups)
- Lie algebra cohomology: parallel construction for Lie algebras and their representations
- Group extensions: $H^2(G, A)$ classifies extensions $0 \to A \to E \to G \to 0$

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| Independence of resolution | $\mathrm{Ext}^n$ and $\mathrm{Tor}_n$ don't depend on resolution choice |
| $\mathrm{Ext}^1$ = extensions | $\mathrm{Ext}^1(C,A) \cong$ classes of SES $0 \to A \to E \to C \to 0$ |
| Projective iff Ext vanishes | $M$ projective $\Leftrightarrow \mathrm{Ext}^n(M,-) = 0$ for $n \geq 1$ |
| Flat iff Tor vanishes | $M$ flat $\Leftrightarrow \mathrm{Tor}_n(M,-) = 0$ for $n \geq 1$ |
| $H^n(G,M) = \mathrm{Ext}^n_{\mathbb{Z}[G]}(\mathbb{Z},M)$ | Group cohomology is Ext |

---

## Milestone Exercises

1. Compute $\mathrm{Ext}^n_\mathbb{Z}(\mathbb{Z}/4\mathbb{Z}, \mathbb{Z}/6\mathbb{Z})$ for all $n \geq 0$.

2. Compute $\mathrm{Tor}_n^\mathbb{Z}(\mathbb{Z}/m\mathbb{Z}, \mathbb{Z}/n\mathbb{Z})$ for all $n \geq 0$.

3. Show that $\mathrm{gl.dim}(\mathbb{Z}) = 1$ (i.e., $\mathrm{Ext}^n_\mathbb{Z}(M,N) = 0$ for all $n \geq 2$ and all $M,N$).

4. Classify all extensions of $\mathbb{Z}/2\mathbb{Z}$ by $\mathbb{Z}/2\mathbb{Z}$ using $\mathrm{Ext}^1$.

5. Compute $H^1(\mathbb{Z}/n\mathbb{Z}, \mathbb{Z})$ where $\mathbb{Z}/n\mathbb{Z}$ acts trivially on $\mathbb{Z}$.

6. Show $H^2(G, A)$ (with $A$ abelian and $G$ acting trivially) classifies central extensions of $G$ by $A$.

7. Prove that a projective module over a Noetherian ring has finite projective dimension.

---

## Connections Forward

- **Chapter 17:** Spectral sequences compute homology of double complexes; the $E_2$ page typically involves Ext and Tor as inputs.
- **Chapter 18:** Group cohomology $H^n(G, M)$ appears in representation theory; $H^2$ classifies projective representations.
- **Chapter 22:** Lie algebra cohomology $H^n(\mathfrak{g}, M)$ is the derived functor of $M^{\mathfrak{g}}$ — parallel to group cohomology.

---

*Next: [Chapter 17 — Spectral Sequences](ch17-spectral-sequences.md)*
