# Chapter 13 — Field Theory and Galois Theory

**Part III: Abstract Algebra**
*Prerequisites: [Chapter 11](ch11-ring-theory.md), [Chapter 10](ch10-group-theory-structure.md)*
*Next: [Chapter 14 — Category Theory](ch14-category-theory.md)*

---

## Learning Objectives

- Understand field extensions: algebraic vs. transcendental; degree; minimal polynomial
- Construct splitting fields and algebraic closures
- Understand separability and the Frobenius endomorphism in characteristic $p$
- State and prove the fundamental theorem of Galois theory
- Apply Galois theory to solvability by radicals and classical ruler-and-compass problems
- Understand finite fields completely

---

## 13.1 Field Extensions

### 13.1.1 Basic Definitions

A **field extension** is a pair $F \subseteq E$ where $E$ is a field containing $F$ as a subfield. Write $E/F$ (read "$E$ over $F$").

$E$ is an $F$-vector space; the **degree** $[E:F] = \dim_F E$. If $[E:F] < \infty$, the extension is **finite**.

**Tower law:** $[E:F] = [E:K][K:F]$ for $F \subseteq K \subseteq E$.

### 13.1.2 Algebraic and Transcendental Elements

$\alpha \in E$ is **algebraic over $F$** if $p(\alpha) = 0$ for some non-zero $p \in F[x]$.

The **minimal polynomial** of $\alpha$ over $F$: the unique monic polynomial $\mathrm{min}_{F,\alpha} \in F[x]$ of smallest degree with $\mathrm{min}_{F,\alpha}(\alpha) = 0$. It is irreducible.

$\alpha$ is **transcendental** if no such polynomial exists.

**Simple extensions:**
$$F(\alpha) \cong \begin{cases} F[x]/(\mathrm{min}_{F,\alpha}) & \text{if } \alpha \text{ algebraic} \\ F(x) = \mathrm{Frac}(F[x]) & \text{if } \alpha \text{ transcendental} \end{cases}$$

**Degree:** $[F(\alpha):F] = \deg(\mathrm{min}_{F,\alpha})$ if $\alpha$ algebraic.

### 13.1.3 Algebraic Extensions

$E/F$ is **algebraic** if every element of $E$ is algebraic over $F$.

**Theorem:** Finite extensions are algebraic. The composition of algebraic extensions is algebraic.

**Algebraic closure:** An algebraically closed field $\bar{F}$ with $F \subseteq \bar{F}$ algebraic; every polynomial in $F[x]$ splits completely in $\bar{F}$.

**Theorem:** Every field has an algebraic closure, unique up to isomorphism (proof uses Zorn's Lemma).

**Examples:** $\bar{\mathbb{Q}}$ (algebraic numbers), $\mathbb{C}$ is its own closure.

---

## 13.2 Splitting Fields and Normal Extensions

### 13.2.1 Splitting Fields

For $f \in F[x]$, a **splitting field** of $f$ over $F$ is the smallest extension $E/F$ in which $f$ factors completely into linear factors.

**Existence and uniqueness:** Every polynomial has a splitting field; any two splitting fields of $f$ over $F$ are isomorphic via an isomorphism fixing $F$.

**Degree bound:** $[E:F] \leq (\deg f)!$

### 13.2.2 Normal Extensions

$E/F$ is **normal** if:
- $E/F$ is algebraic, and
- Every irreducible $p \in F[x]$ with a root in $E$ splits completely in $E$

**Equivalent:** $E$ is the splitting field of some family of polynomials over $F$.

**Key:** A finite extension $E/F$ is normal iff it is a splitting field of some $f \in F[x]$.

---

## 13.3 Separability

### 13.3.1 Separable Polynomials

$f \in F[x]$ is **separable** if it has no repeated roots in $\bar{F}$.

Criterion: $f$ is separable $\Leftrightarrow$ $\gcd(f, f') = 1$ (where $f' = df/dx$).

### 13.3.2 Separable Extensions

$\alpha \in E$ is **separable** over $F$ if its minimal polynomial is separable.

$E/F$ is **separable** if every element is separable over $F$.

**In characteristic 0:** All algebraic extensions are separable.

**In characteristic $p$:** Inseparable elements exist. The **Frobenius endomorphism** $\phi: F \to F$, $\phi(a) = a^p$, plays a central role.

**Primitive element theorem:** If $E/F$ is finite and separable, then $E = F(\alpha)$ for some $\alpha$ (a primitive element).

---

## 13.4 Galois Theory

### 13.4.1 Galois Extensions and Galois Groups

$E/F$ is a **Galois extension** if it is both normal and separable.

The **Galois group**: $\mathrm{Gal}(E/F) = \{\sigma: E \xrightarrow{\sim} E \mid \sigma|_F = \mathrm{id}_F\}$ — the group of field automorphisms of $E$ fixing $F$ pointwise.

**Order theorem:** For a finite Galois extension: $|\mathrm{Gal}(E/F)| = [E:F]$.

### 13.4.2 The Fixed Field and Fundamental Theorem

For $H \leq \mathrm{Gal}(E/F)$, the **fixed field**:
$$E^H = \{\alpha \in E \mid \sigma(\alpha) = \alpha \text{ for all } \sigma \in H\}$$

**Fundamental Theorem of Galois Theory:** Let $E/F$ be a finite Galois extension with $G = \mathrm{Gal}(E/F)$. There is an inclusion-reversing bijection:

$$\left\{\text{subgroups } H \leq G\right\} \longleftrightarrow \left\{\text{intermediate fields } F \subseteq K \subseteq E\right\}$$

given by $H \mapsto E^H$ and $K \mapsto \mathrm{Gal}(E/K)$.

**Under this correspondence:**
- $[H_1 : H_2] = [E^{H_2} : E^{H_1}]$
- $H \trianglelefteq G$ $\Leftrightarrow$ $E^H / F$ is a Galois extension
- When $H \trianglelefteq G$: $\mathrm{Gal}(E^H/F) \cong G/H$

This is one of the most beautiful theorems in mathematics: a complete dictionary between field extensions and subgroups.

### 13.4.3 Computing Galois Groups

**Example:** $\mathbb{Q}(\sqrt{2}, \sqrt{3})/\mathbb{Q}$ has degree 4. The Galois group is $\mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/2\mathbb{Z}$, with generators $\sigma: \sqrt{2} \mapsto -\sqrt{2}$ and $\tau: \sqrt{3} \mapsto -\sqrt{3}$.

**Discriminant method:** $\mathrm{Gal}(f/\mathbb{Q}) \hookrightarrow S_n$ as a transitive subgroup; the discriminant distinguishes $A_n$ vs. non-$A_n$ parts.

**Cyclotomic fields:** $\mathrm{Gal}(\mathbb{Q}(\zeta_n)/\mathbb{Q}) \cong (\mathbb{Z}/n\mathbb{Z})^*$ where $\zeta_n = e^{2\pi i/n}$.

---

## 13.5 Applications of Galois Theory

### 13.5.1 Solvability by Radicals

$f \in F[x]$ is **solvable by radicals** if its roots can be expressed using $+, -, \times, \div$ and taking $n$-th roots.

**Theorem (Abel–Ruffini):** $f$ is solvable by radicals $\Leftrightarrow$ $\mathrm{Gal}(f/F)$ is a solvable group.

A group $G$ is **solvable** if it has a composition series with abelian quotients.

**Corollary:** The general polynomial of degree $n \geq 5$ is not solvable by radicals, since $S_n$ (and typically $A_n$) is not solvable for $n \geq 5$.

This gives a group-theoretic proof of the impossibility of a quintic formula — the resolution of a 300-year-old problem.

### 13.5.2 Classical Impossibility Results

Using Galois theory (or degree arguments alone):
- **Doubling the cube:** Requires solving $x^3 = 2$; $[{\mathbb{Q}(\sqrt[3]{2})}:\mathbb{Q}] = 3 \neq 2^k$ — impossible by straightedge and compass.
- **Trisecting an angle:** $\cos(20°)$ has minimal polynomial of degree 3 over $\mathbb{Q}$ — impossible in general.
- **Squaring the circle:** Requires $\pi$ to be algebraic — but $\pi$ is transcendental (Lindemann, 1882) — impossible.

---

## 13.6 Finite Fields

**Theorem:** For any prime power $q = p^n$, there exists a unique (up to isomorphism) finite field $\mathbb{F}_q$ of order $q$.

$\mathbb{F}_q = \mathbb{F}_p[x]/(f)$ for any irreducible $f \in \mathbb{F}_p[x]$ of degree $n$.

**Structure:**
- Additive group $(\mathbb{F}_q, +) \cong (\mathbb{Z}/p\mathbb{Z})^n$
- Multiplicative group $(\mathbb{F}_q^*, \cdot) \cong \mathbb{Z}/(q-1)\mathbb{Z}$ (cyclic!)
- $\mathbb{F}_q$ is the splitting field of $x^{q} - x$ over $\mathbb{F}_p$
- $\mathrm{Gal}(\mathbb{F}_{p^n}/\mathbb{F}_p) \cong \mathbb{Z}/n\mathbb{Z}$, generated by Frobenius $x \mapsto x^p$

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| Tower law | $[E:F] = [E:K][K:F]$ |
| Primitive element theorem | Finite separable $\Rightarrow E = F(\alpha)$ |
| Fundamental theorem of Galois theory | Subgroups $\leftrightarrow$ intermediate fields (bijection) |
| Abel–Ruffini theorem | Solvable by radicals $\Leftrightarrow$ solvable Galois group |
| Finite field structure | Unique $\mathbb{F}_{p^n}$; multiplicative group cyclic |

---

## Milestone Exercises

1. Find the minimal polynomial of $\alpha = \sqrt{2} + \sqrt{3}$ over $\mathbb{Q}$. Find $[\mathbb{Q}(\alpha):\mathbb{Q}]$.

2. Compute $\mathrm{Gal}(\mathbb{Q}(\zeta_8)/\mathbb{Q})$ and draw the correspondence between subgroups and intermediate fields.

3. Show $x^5 - 2 \in \mathbb{Q}[x]$ is irreducible. Compute its Galois group over $\mathbb{Q}$.

4. Prove that every finite extension of $\mathbb{F}_p$ is Galois with cyclic Galois group generated by Frobenius.

5. Show that the regular 17-gon is constructible (Gauss): $\mathrm{Gal}(\mathbb{Q}(\zeta_{17})/\mathbb{Q}) \cong (\mathbb{Z}/17\mathbb{Z})^* \cong \mathbb{Z}/16\mathbb{Z}$ is a 2-group.

6. Prove: if $[E:F] = p$ (prime), then there are no intermediate fields.

7. Let $f = x^4 - 5x^2 + 6 \in \mathbb{Q}[x]$. Factor $f$; find the splitting field; compute $\mathrm{Gal}(f/\mathbb{Q})$.

---

## Connections Forward

- **Chapter 14:** Galois theory is naturally a statement about functors and adjunctions in category theory.
- **Chapter 18:** The Galois group of a polynomial extension acts on the roots — this is a group action, and the representation theory of Galois groups is deep.
- **Chapter 24:** Absolute Galois group $\mathrm{Gal}(\bar{\mathbb{Q}}/\mathbb{Q})$ is one of the central objects in number theory; the Langlands program relates its representations to automorphic forms.

---

*Next: [Chapter 14 — Category Theory](ch14-category-theory.md)*
