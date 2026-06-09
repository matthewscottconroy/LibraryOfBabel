# Chapter 10 — Group Theory II: Structure and Actions

**Part III: Abstract Algebra**
*Prerequisites: [Chapter 9](ch09-group-theory-foundations.md)*
*Next: [Chapter 11 — Ring Theory](ch11-ring-theory.md)*

---

## Learning Objectives

- Define and apply group homomorphisms; prove the isomorphism theorems
- Understand group actions and the orbit-stabilizer theorem
- Apply the Sylow theorems to classify and analyze finite groups
- Understand direct and semidirect products as group-building tools
- Know the classification of finitely generated abelian groups
- Recognize simple groups and understand their significance

---

## 10.1 Group Homomorphisms

### 10.1.1 Definition

A map $\phi: G \to H$ is a **group homomorphism** if:
$$\phi(ab) = \phi(a)\phi(b) \quad \text{for all } a, b \in G$$

Immediately: $\phi(e_G) = e_H$ and $\phi(a^{-1}) = \phi(a)^{-1}$.

Special cases:
- **Isomorphism:** bijective homomorphism ($G \cong H$ means an isomorphism exists)
- **Endomorphism:** $\phi: G \to G$
- **Automorphism:** bijective endomorphism; $\mathrm{Aut}(G)$ = group of automorphisms under composition
- **Monomorphism / epimorphism:** injective / surjective homomorphism

### 10.1.2 Kernel and Image

$$\ker \phi = \{g \in G \mid \phi(g) = e_H\}, \quad \mathrm{im}\, \phi = \{\phi(g) \mid g \in G\} \leq H$$

$\ker \phi \trianglelefteq G$ (always normal). $\phi$ is injective $\Leftrightarrow$ $\ker \phi = \{e\}$.

**Conjugation:** For fixed $x \in G$, the map $\phi_x: G \to G$, $\phi_x(g) = xgx^{-1}$ is an automorphism (inner automorphism). $\mathrm{Inn}(G) \trianglelefteq \mathrm{Aut}(G)$.

### 10.1.3 The Isomorphism Theorems

**First Isomorphism Theorem:** If $\phi: G \to H$ is a homomorphism, then:
$$G/\ker\phi \cong \mathrm{im}\,\phi$$

*Proof:* The map $g(\ker\phi) \mapsto \phi(g)$ is a well-defined isomorphism.

**Second Isomorphism Theorem:** If $H \leq G$ and $N \trianglelefteq G$, then $HN \leq G$, $H \cap N \trianglelefteq H$, and:
$$H/(H \cap N) \cong HN/N$$

**Third Isomorphism Theorem:** If $N \trianglelefteq M \trianglelefteq G$ and $N \trianglelefteq G$:
$$(G/N)/(M/N) \cong G/M$$

**Fourth Isomorphism Theorem (Correspondence Theorem):** There is a bijection between subgroups of $G/N$ and subgroups of $G$ containing $N$, preserving index, normality, and quotient structure.

---

## 10.2 Group Actions

### 10.2.1 Definition

A **left action** of $G$ on a set $X$ is a map $G \times X \to X$, $(g, x) \mapsto g \cdot x$, satisfying:
- $e \cdot x = x$
- $(gh) \cdot x = g \cdot (h \cdot x)$

Equivalently: a homomorphism $G \to \mathrm{Sym}(X)$ (the group of bijections of $X$).

### 10.2.2 Orbits and Stabilizers

The **orbit** of $x \in X$: $G \cdot x = \{g \cdot x \mid g \in G\}$.

The **stabilizer** of $x$: $G_x = \{g \in G \mid g \cdot x = x\} \leq G$ (always a subgroup).

**Orbit-Stabilizer Theorem:**
$$|G| = |G \cdot x| \cdot |G_x|$$

*Proof:* The bijection $G/G_x \to G \cdot x$ given by $gG_x \mapsto g \cdot x$.

The orbits partition $X$: $X = \bigsqcup_{\text{orbits}} G \cdot x$.

### 10.2.3 Important Actions

| Action | $G$ acts on | $G \cdot x$ | $G_x$ |
|--------|------------|-------------|--------|
| Left multiplication | $G$ | All of $G$ (1 orbit) | $\{e\}$ |
| Conjugation | $G$ | Conjugacy class of $x$ | Centralizer $C_G(x)$ |
| $G$ on cosets of $H$ | $G/H$ | All of $G/H$ | Conjugate of $H$ |
| $GL_n(F)$ on $F^n \setminus \{0\}$ | $F^n \setminus \{0\}$ | All of $F^n \setminus \{0\}$ | Depends on vector |

### 10.2.4 The Class Equation

Applying orbit-stabilizer to the conjugation action of $G$ on itself:
$$|G| = |Z(G)| + \sum_{[x]: |G \cdot x| > 1} \frac{|G|}{|C_G(x)|}$$

where the sum is over conjugacy classes of size $> 1$.

**Application:** If $|G| = p^n$ ($p$ prime), then $|Z(G)| > 1$ (groups of prime power order have non-trivial center). In particular, groups of order $p^2$ are abelian.

### 10.2.5 Burnside's Lemma

The number of orbits of $G$ acting on $X$:
$$|X/G| = \frac{1}{|G|} \sum_{g \in G} |X^g|$$
where $X^g = \{x \in X \mid g \cdot x = x\}$ is the fixed-point set of $g$.

**Application:** Counting colorings up to symmetry (Pólya enumeration theory).

---

## 10.3 Sylow Theorems

### 10.3.1 $p$-Groups and $p$-Sylow Subgroups

A **$p$-group** is a group in which every element has order a power of $p$.

If $|G| = p^a m$ with $\gcd(p, m) = 1$, a **Sylow $p$-subgroup** is a subgroup of order $p^a$ (maximum possible power of $p$).

### 10.3.2 The Three Sylow Theorems

Let $G$ be finite with $|G| = p^a m$, $\gcd(p,m) = 1$.

**Sylow I:** $G$ has a Sylow $p$-subgroup.

**Sylow II:** All Sylow $p$-subgroups of $G$ are conjugate. Hence: $G$ has exactly one Sylow $p$-subgroup iff it is normal.

**Sylow III:** The number $n_p$ of Sylow $p$-subgroups satisfies:
- $n_p \equiv 1 \pmod{p}$
- $n_p \mid m$

### 10.3.3 Classifying Groups with Sylow

**Strategy:** Given $|G| = n$, use Sylow to constrain the Sylow subgroups; if $n_p = 1$, the Sylow $p$-subgroup is normal and $G$ has non-trivial structure.

**Example:** If $|G| = 15 = 3 \cdot 5$, then $n_3 \mid 5$ and $n_3 \equiv 1 \pmod 3$, so $n_3 = 1$; and $n_5 \mid 3$ and $n_5 \equiv 1 \pmod 5$, so $n_5 = 1$. Both Sylow subgroups are normal and $G \cong \mathbb{Z}/15\mathbb{Z}$.

**Example:** No group of order $pq$ with $p < q$ and $p \nmid q-1$ is simple.

---

## 10.4 Direct and Semidirect Products

### 10.4.1 Direct Products

The **direct product** $G \times H$ has underlying set $G \times H$ with componentwise operation: $(g_1, h_1)(g_2, h_2) = (g_1 g_2, h_1 h_2)$.

**Internal direct product:** $G = N_1 \times N_2$ internally iff:
- $N_1, N_2 \trianglelefteq G$
- $N_1 N_2 = G$
- $N_1 \cap N_2 = \{e\}$

### 10.4.2 Semidirect Products

More general: given $N, H$ and $\phi: H \to \mathrm{Aut}(N)$, the **semidirect product** $N \rtimes_\phi H$ has underlying set $N \times H$ with:
$$(n_1, h_1)(n_2, h_2) = (n_1 \cdot \phi(h_1)(n_2),\, h_1 h_2)$$

This generalizes the direct product (where $\phi$ is trivial). Non-trivial $\phi$ gives non-abelian groups.

**Example:** $D_n \cong \mathbb{Z}/n\mathbb{Z} \rtimes \mathbb{Z}/2\mathbb{Z}$ where the generator of $\mathbb{Z}/2\mathbb{Z}$ acts on $\mathbb{Z}/n\mathbb{Z}$ by inversion $k \mapsto -k$.

---

## 10.5 Finitely Generated Abelian Groups

**Theorem (Fundamental Theorem of Finitely Generated Abelian Groups):**

Every finitely generated abelian group $G$ is isomorphic to:
$$\mathbb{Z}^r \oplus \mathbb{Z}/d_1\mathbb{Z} \oplus \mathbb{Z}/d_2\mathbb{Z} \oplus \cdots \oplus \mathbb{Z}/d_k\mathbb{Z}$$
where $d_1 \mid d_2 \mid \cdots \mid d_k$ (invariant factor form) and $r \geq 0$, $d_i \geq 2$.

**Equivalent (primary decomposition form):**
$$\mathbb{Z}^r \oplus \mathbb{Z}/p_1^{a_1}\mathbb{Z} \oplus \cdots \oplus \mathbb{Z}/p_m^{a_m}\mathbb{Z}$$

These invariants uniquely determine $G$ up to isomorphism.

**Connection to linear algebra:** This is the group-theoretic version of the structure theorem for modules over $\mathbb{Z}$ (a PID) — developed fully in Chapter 12.

---

## 10.6 Composition Series and the Jordan–Hölder Theorem

### 10.6.1 Simple Groups

$G$ is **simple** if $G \neq \{e\}$ and $G$ has no proper non-trivial normal subgroups.

**Examples:** $\mathbb{Z}/p\mathbb{Z}$ (for prime $p$); $A_n$ for $n \geq 5$ (one of the central results of classical group theory).

**Classification of Finite Simple Groups (CFSG):** Every finite simple group is isomorphic to one of:
- Cyclic group $\mathbb{Z}/p\mathbb{Z}$
- Alternating group $A_n$ ($n \geq 5$)
- A group of Lie type (e.g., $PSL_n(\mathbb{F}_q)$, etc.)
- One of 26 sporadic groups (Monster group, etc.)

The CFSG proof spans tens of thousands of pages — the largest theorem in mathematics.

### 10.6.2 Composition Series

A **composition series** of $G$ is a chain:
$$\{e\} = G_0 \trianglelefteq G_1 \trianglelefteq \cdots \trianglelefteq G_k = G$$
where each quotient $G_{i+1}/G_i$ is simple.

**Jordan–Hölder Theorem:** Any two composition series of $G$ have the same length and the same multiset of composition factors (simple quotients).

This is the group-theoretic analogue of unique prime factorization.

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| First Isomorphism | $G/\ker\phi \cong \mathrm{im}\,\phi$ |
| Orbit-Stabilizer | $\|G\| = \|G \cdot x\| \cdot \|G_x\|$ |
| Class Equation | $\|G\| = \|Z(G)\| + \sum \|G\|/\|C_G(x)\|$ |
| Sylow I, II, III | Existence, conjugacy, counting of Sylow subgroups |
| FTFGAG | Finitely gen. abelian groups $\cong \mathbb{Z}^r \oplus \bigoplus \mathbb{Z}/d_i\mathbb{Z}$ |
| Jordan–Hölder | Composition factors are unique |

---

## Milestone Exercises

1. Prove the First Isomorphism Theorem.

2. Use the class equation to prove: groups of order $p^2$ are abelian.

3. Prove: $A_5$ is simple. (Hint: count elements of each order; show no proper normal subgroup can exist.)

4. Classify all groups of order 8 (there are 5: $\mathbb{Z}/8\mathbb{Z}$, $\mathbb{Z}/4\mathbb{Z} \times \mathbb{Z}/2\mathbb{Z}$, $\mathbb{Z}/2\mathbb{Z}^3$, $D_4$, $Q_8$).

5. Using Sylow's theorems, show every group of order 30 has a normal Sylow 5-subgroup.

6. Show $D_n \cong \mathbb{Z}/n\mathbb{Z} \rtimes \mathbb{Z}/2\mathbb{Z}$ by identifying the semidirect product structure explicitly.

7. Find all finitely generated abelian groups of order 360.

8. Prove the Jordan–Hölder theorem.

---

## Connections Forward

- **Chapter 11:** Rings have an underlying additive group; ideals are additive subgroups with extra closure properties.
- **Chapter 12:** Module theory is built on the group-theoretic structure; the structure theorem for modules over a PID generalizes FTFGAG.
- **Chapter 18:** Representation theory begins: a representation is a group homomorphism $G \to GL(V)$.
- **Chapter 20:** Lie groups generalize finite group theory to the continuous (manifold) setting.

---

*Next: [Chapter 11 — Ring Theory](ch11-ring-theory.md)*
