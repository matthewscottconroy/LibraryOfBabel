# Chapter 11 — Ring Theory

**Part III: Abstract Algebra**
*Prerequisites: [Chapter 10](ch10-group-theory-structure.md)*
*Next: [Chapter 12 — Module Theory](ch12-module-theory.md)*

---

## Learning Objectives

- Define rings and their hierarchy: domains, PID, UFD, Euclidean domains
- Understand ideals and quotient rings; apply the isomorphism theorems for rings
- Work with polynomial rings; apply the division algorithm and irreducibility criteria
- Understand prime and maximal ideals and their quotients
- Classify rings by their factorization properties
- Understand localization as a tool for studying ring structure locally

---

## 11.1 Rings and Ring Homomorphisms

### 11.1.1 Definition

A **ring** $(R, +, \cdot)$ is a set with two operations satisfying:
- $(R, +)$ is an abelian group (with identity $0$)
- $(R, \cdot)$ is associative (with identity $1$ if **unital**)
- Distributivity: $a(b+c) = ab + ac$ and $(a+b)c = ac + bc$

**Conventions:** We assume rings are unital ($1 \in R$) and write $1_R$ to disambiguate. A ring is **commutative** if $ab = ba$.

### 11.1.2 Examples

| Ring | Commutative? | Domain? | Field? |
|------|-------------|---------|--------|
| $\mathbb{Z}$ | Yes | Yes | No |
| $\mathbb{Q}, \mathbb{R}, \mathbb{C}$ | Yes | Yes | Yes |
| $\mathbb{Z}/n\mathbb{Z}$ | Yes | Iff $n$ prime | Iff $n$ prime |
| $F[x]$ (polynomials) | Yes | Yes | No |
| $M_n(F)$ | No (for $n>1$) | No | No |
| $\mathbb{Z}[i] = \{a+bi \mid a,b\in\mathbb{Z}\}$ | Yes | Yes | No |
| $\mathbb{Z}[\sqrt{-5}]$ | Yes | Yes | No (not UFD) |

### 11.1.3 Ring Homomorphisms

$\phi: R \to S$ is a **ring homomorphism** if:
- $\phi(a+b) = \phi(a) + \phi(b)$
- $\phi(ab) = \phi(a)\phi(b)$
- $\phi(1_R) = 1_S$

$\ker\phi = \phi^{-1}(0_S)$ is an **ideal** of $R$; $\mathrm{im}\,\phi$ is a subring of $S$.

---

## 11.2 Ideals and Quotient Rings

### 11.2.1 Ideals

A subset $I \subseteq R$ is a **(two-sided) ideal** if:
- $(I, +)$ is a subgroup of $(R, +)$
- $rI \subseteq I$ and $Ir \subseteq I$ for all $r \in R$ (**absorption**)

**Left ideal:** only $rI \subseteq I$. **Right ideal:** only $Ir \subseteq I$.

**Examples:**
- $n\mathbb{Z} \trianglelefteq \mathbb{Z}$ (multiples of $n$)
- $(f) = fR$ for any $f \in R$ — **principal ideal**
- Every ideal in $\mathbb{Z}$ is principal: $I = n\mathbb{Z}$ for some $n$

### 11.2.2 Quotient Rings

For $I \trianglelefteq R$, the **quotient ring** $R/I = \{r + I \mid r \in R\}$ with:
$$(r+I) + (s+I) = (r+s)+I, \qquad (r+I)(s+I) = rs + I$$

**First Isomorphism Theorem for Rings:** If $\phi: R \to S$ is a ring homomorphism:
$$R/\ker\phi \cong \mathrm{im}\,\phi$$

**Examples:**
- $\mathbb{Z}/n\mathbb{Z}$ is the quotient by the ideal $n\mathbb{Z}$
- $\mathbb{R}[x]/(x^2+1) \cong \mathbb{C}$ (adjoin a root of $x^2+1$)
- $F[x]/(f)$ for irreducible $f$ is a field extension of $F$

### 11.2.3 Prime and Maximal Ideals

$P \trianglelefteq R$ is **prime** if $ab \in P \Rightarrow a \in P$ or $b \in P$.

$M \trianglelefteq R$ is **maximal** if no ideal satisfies $M \subsetneq I \subsetneq R$.

**Theorem:**
- $P$ is prime $\Leftrightarrow$ $R/P$ is an integral domain
- $M$ is maximal $\Leftrightarrow$ $R/M$ is a field
- Every maximal ideal is prime (but not vice versa)
- Every ring has a maximal ideal (by Zorn's Lemma)

**Example in $\mathbb{Z}$:** Prime ideals are $(0)$ and $(p)$ for prime $p$. Maximal ideals are $(p)$ for prime $p$.

---

## 11.3 Domains and Divisibility

### 11.3.1 Integral Domains

A commutative unital ring $R$ is an **integral domain** (or **domain**) if $R \neq 0$ and it has no zero divisors ($ab = 0 \Rightarrow a=0$ or $b=0$).

**Cancellation:** In a domain, $ac = bc$ and $c \neq 0$ implies $a = b$.

### 11.3.2 Divisibility in Domains

In a commutative domain $R$:
- $a \mid b$ (**$a$ divides $b$**) if $b = ac$ for some $c \in R$
- $a, b$ are **associates** if $a = ub$ for a unit $u$ (invertible element)
- $p$ is **irreducible** if $p \neq 0$, $p$ not a unit, and $p = ab \Rightarrow$ $a$ or $b$ is a unit
- $p$ is **prime** if $p \neq 0$, $p$ not a unit, and $p \mid ab \Rightarrow p \mid a$ or $p \mid b$

In a domain: prime $\Rightarrow$ irreducible. The converse fails in general.

---

## 11.4 The Hierarchy of Domains

### 11.4.1 Euclidean Domains

$R$ is a **Euclidean domain** if there exists $N: R \setminus \{0\} \to \mathbb{N}_0$ (**Euclidean norm**) such that for all $a, b \in R$ with $b \neq 0$:
$$a = qb + r \quad \text{where } r = 0 \text{ or } N(r) < N(b)$$

**Examples:** $\mathbb{Z}$ (norm = $|n|$), $F[x]$ (norm = degree), $\mathbb{Z}[i]$ (Gaussian integers, norm = $a^2+b^2$).

### 11.4.2 Principal Ideal Domains

$R$ is a **PID** (principal ideal domain) if $R$ is a domain and every ideal is principal: $I = (a)$ for some $a \in R$.

**Theorem:** Euclidean domain $\Rightarrow$ PID $\Rightarrow$ UFD. Neither implication reverses in general.

In a PID: GCD exists; irreducible $\Leftrightarrow$ prime; $(p)$ is prime iff $p$ is irreducible.

### 11.4.3 Unique Factorization Domains

$R$ is a **UFD** if every non-zero non-unit factors into irreducibles, and the factorization is unique up to order and associates.

**Theorem:** $R$ UFD $\Rightarrow$ $R[x]$ UFD (Gauss's theorem).

**Example of non-UFD:** In $\mathbb{Z}[\sqrt{-5}]$: $6 = 2 \cdot 3 = (1+\sqrt{-5})(1-\sqrt{-5})$ are two distinct factorizations into irreducibles.

### 11.4.4 The Hierarchy

$$\text{Fields} \subset \text{Euclidean Domains} \subset \text{PIDs} \subset \text{UFDs} \subset \text{Domains} \subset \text{Rings}$$

---

## 11.5 Polynomial Rings

### 11.5.1 $R[x]$ and the Division Algorithm

For a ring $R$, $R[x]$ is the ring of polynomials with coefficients in $R$.

**Division Algorithm (for $F[x]$, $F$ a field):** For $f, g \in F[x]$ with $g \neq 0$:
$$f = qg + r, \quad \deg r < \deg g \text{ or } r = 0$$

This makes $F[x]$ a Euclidean domain (hence PID, hence UFD).

### 11.5.2 Roots and Irreducibility

$a \in F$ is a **root** of $f$ iff $(x-a) \mid f$ in $F[x]$.

A polynomial $f$ of degree $\geq 1$ is **irreducible** over $F$ if it has no factorization into polynomials of lower degree.

**Over $\mathbb{C}$ (Fundamental Theorem of Algebra):** Every non-constant polynomial factors into linear factors.

**Over $\mathbb{R}$:** Irreducibles are linear and quadratic with negative discriminant.

**Over $\mathbb{Q}$ (Eisenstein's criterion):** If $f = a_n x^n + \cdots + a_0 \in \mathbb{Z}[x]$ and prime $p$ satisfies $p \nmid a_n$, $p \mid a_i$ for $i < n$, and $p^2 \nmid a_0$, then $f$ is irreducible over $\mathbb{Q}$.

### 11.5.3 Gauss's Lemma

A polynomial $f \in \mathbb{Z}[x]$ that is irreducible over $\mathbb{Z}$ is irreducible over $\mathbb{Q}$.

More generally: if $R$ is a UFD, then $R[x]$ is a UFD, and irreducibility over $R$ implies irreducibility over $\mathrm{Frac}(R)$.

### 11.5.4 Multivariate Polynomial Rings

$F[x_1, \ldots, x_n]$ is a UFD (by induction on $n$, using $F[x_1, \ldots, x_n] = F[x_1, \ldots, x_{n-1}][x_n]$).

**Hilbert's Basis Theorem:** If $R$ is Noetherian, so is $R[x]$. In particular, $F[x_1, \ldots, x_n]$ is Noetherian — every ideal is finitely generated.

---

## 11.6 Localization

### 11.6.1 The Fraction Field

For a domain $R$, the **fraction field** $\mathrm{Frac}(R) = \{a/b \mid a, b \in R, b \neq 0\}/{\sim}$ where $a/b \sim c/d$ iff $ad = bc$.

**Examples:** $\mathrm{Frac}(\mathbb{Z}) = \mathbb{Q}$; $\mathrm{Frac}(F[x]) = F(x)$ (rational functions).

### 11.6.2 Localization at a Multiplicative Set

For a multiplicative set $S \subseteq R$ (closed under multiplication, $1 \in S$), the **localization** $S^{-1}R$ formally inverts elements of $S$.

**Key cases:**
- $S = R \setminus \{0\}$: fraction field
- $S = R \setminus P$ for prime $P$: local ring $R_P$ (localizing at $P$) — used in algebraic geometry and commutative algebra
- $S = \{1, f, f^2, \ldots\}$: $R_f$ (invert the element $f$)

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| Isomorphism theorem | $R/\ker\phi \cong \mathrm{im}\,\phi$ |
| Prime ideal criterion | $P$ prime $\Leftrightarrow$ $R/P$ domain |
| Maximal ideal criterion | $M$ maximal $\Leftrightarrow$ $R/M$ field |
| ED $\subset$ PID $\subset$ UFD | Hierarchy of divisibility domains |
| $R$ UFD $\Rightarrow$ $R[x]$ UFD | Gauss's theorem |
| Hilbert's basis theorem | Noetherian $\Rightarrow$ $R[x]$ Noetherian |
| Eisenstein's criterion | Sufficient condition for irreducibility over $\mathbb{Q}$ |

---

## Milestone Exercises

1. Show $\mathbb{Z}[i]$ is a Euclidean domain with norm $N(a+bi) = a^2 + b^2$.

2. In $\mathbb{Z}[\sqrt{-5}]$, show $2$ is irreducible but not prime.

3. Find all prime and maximal ideals of $\mathbb{Z}[x]$.

4. Show $\mathbb{Q}[x]/(x^2-2) \cong \mathbb{Q}(\sqrt{2})$.

5. Prove Eisenstein's criterion from scratch.

6. Show the ideal $(2, x) \trianglelefteq \mathbb{Z}[x]$ is maximal. Is $\mathbb{Z}[x]$ a PID?

7. Compute $\mathbb{Z}[x]/(x^2-1)$. Is this a domain? A field?

8. Prove Hilbert's basis theorem.

---

## Connections Forward

- **Chapter 12:** Modules are the ring-theoretic generalization of vector spaces; the structure theorem requires PID technology.
- **Chapter 13:** Field theory is the study of commutative rings that are fields; Galois theory analyzes their extensions.
- **Chapter 15:** Homological algebra requires the ideal/module framework.
- **Chapter 26:** Schemes in algebraic geometry are defined via spectra of commutative rings.

---

*Next: [Chapter 12 — Module Theory](ch12-module-theory.md)*
