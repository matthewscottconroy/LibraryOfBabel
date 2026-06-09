# Chapter 28 — Category Theory and Dynamical Systems

> *A dynamical system is an object in a category. A factor map is a morphism. Conjugacy is isomorphism. The study of dynamical systems is the study of a category — and category theory tells us what questions are meaningful.*

**Prerequisites:** Chapter 6 (topological dynamics), Chapter 7 (ergodic theory), Chapter 12 (symbolic dynamics). Some familiarity with categories helpful.

---

## 28.1 Categories of Dynamical Systems

### 28.1.1 The Basic Categories

**Definition 28.1.1.** The category $\mathbf{Top.Dyn}$ of *topological dynamical systems* has:
- *Objects*: pairs $(X, f)$ where $X$ is a compact metric space and $f: X \to X$ is continuous
- *Morphisms*: continuous maps $\phi: (X, f) \to (Y, g)$ with $\phi \circ f = g \circ \phi$ (factor maps)
- *Identity*: $\text{id}_X$
- *Composition*: composition of factor maps

**Definition 28.1.2.** The category $\mathbf{Meas.Dyn}$ of *measure-preserving systems* has:
- *Objects*: $(X, \mathcal{B}, \mu, f)$ standard probability spaces with measure-preserving $f$
- *Morphisms*: measure-preserving maps $\phi$ with $\phi \circ f = g \circ \phi$ a.e.
- *Isomorphism*: $\phi$ is an isomorphism iff it is a.e. bijective (measure-theoretic conjugacy)

**Observation 28.1.3.** There is a forgetful functor $\mathbf{Meas.Dyn} \to \mathbf{Top.Dyn}$ (forget the measure). Not every topological factor map preserves a given measure.

### 28.1.2 Limits and Colimits in $\mathbf{Top.Dyn}$

**Products:** The product $(X \times Y, f \times g)$ is the categorical product in $\mathbf{Top.Dyn}$. An $f$-invariant measure on $X$ and a $g$-invariant measure on $Y$ yield an $(f \times g)$-invariant measure on $X \times Y$.

**Inverse Limits (Projective Limits):** If $(X_n, f_n)$ form a projective system with factor maps $\pi_{n+1}: (X_{n+1}, f_{n+1}) \to (X_n, f_n)$, the inverse limit $\varprojlim (X_n, f_n)$ is the dynamical system on:
$$X_\infty = \left\{(x_n) \in \prod_n X_n : \pi_{n+1}(x_{n+1}) = x_n\right\}$$
with $f_\infty(x_n) = (f_n(x_n))$.

**Example 28.1.4 (Natural Extension).** The *natural extension* of a non-invertible system $(X, f)$ is the inverse limit of the system $(X, f) \xleftarrow{f} (X, f) \xleftarrow{f} \cdots$. This is the smallest invertible extension, and it is the standard tool for studying non-invertible ergodic systems.

---

## 28.2 Functors Between Dynamics and Algebra

### 28.2.1 The Koopman Functor

**Definition 28.2.1.** The *Koopman operator* $U_f: L^2(X, \mu) \to L^2(X, \mu)$, $U_f\varphi = \varphi \circ f$, is the image of the morphism $f$ under the *Koopman functor*:
$$K: \mathbf{Meas.Dyn}^{op} \to \mathbf{Hilb}$$
sending $(X, \mu, f) \mapsto (L^2(X,\mu), U_f)$.

**Theorem 28.2.2 (Von Neumann, categorical formulation).** The Koopman functor $K$ is faithful (injective on morphisms) when restricted to ergodic systems. Two ergodic systems are isomorphic iff their Koopman operators are unitarily equivalent *and* the isomorphism preserves the algebra structure.

**Definition 28.2.3.** The *spectral theory* of a dynamical system is the image under $K$: the unitary operator $U_f$ on $L^2$. The eigenvalues of $U_f$ are the *eigenvalues of the system* — they are elements of $S^1$.

**Theorem 28.2.4 (Halmos-von Neumann).** Two ergodic rotations $R_\alpha, R_\beta$ on $S^1$ are isomorphic iff $\alpha = \pm\beta$. The spectrum of $R_\alpha$ is $\{e^{2\pi i n\alpha} : n \in {\mathbb Z}\}$.

### 28.2.2 The Groupoid of a Dynamical System

**Definition 28.2.5.** The *orbit groupoid* (or *transformation groupoid*) of a dynamical system $(X, f)$ is:
$$\mathcal{G}(X, f) = \{(x, n, y) : f^n(x) = y, n \in {\mathbb Z}\}$$
with multiplication $(x, n, y) \cdot (y, m, z) = (x, n+m, z)$ and inversion $(x, n, y)^{-1} = (y, -n, x)$.

**Theorem 28.2.6.** Two minimal dynamical systems are orbit-equivalent iff their orbit groupoids are isomorphic as étale groupoids.

This is the groupoid formulation of the Giordano-Putnam-Skau theorem for minimal ${\mathbb Z}$-systems.

---

## 28.3 Topoi and Dynamical Systems

### 28.3.1 The Topos of a Group Action

**Definition 28.3.1.** For a topological group $G$ acting on a set $X$, the *topos of $G$-sets* $\mathbf{Set}^G$ consists of sets with $G$-action. A "generalized dynamical system" is an object in this topos.

**Theorem 28.3.2 (Lawvere).** The category $\mathbf{Set}^{\mathbb Z}$ (sets with a ${\mathbb Z}$-action, i.e., sets with an automorphism) is a topos. Dynamical systems (in the topological sense) correspond to sheaves on this topos.

**Internal Logic:** The internal language of the topos $\mathbf{Set}^{\mathbb Z}$ is intuitionistic — propositions like "eventually $P$ holds" and "always $P$ holds" have different truth values, corresponding to temporal logic.

### 28.3.2 The Stone Space Functor

**Definition 28.3.3.** The *Stone space* of a Boolean algebra $B$ is the compact totally disconnected space $\text{Stone}(B)$ of ultrafilters on $B$. A dynamical system $(X, f)$ with $X$ a Stone space corresponds to a Boolean algebra $B(X)$ with an automorphism $f^*$.

**Theorem 28.3.4 (Stone Duality for Dynamics).** The functor $X \mapsto C(X, {\mathbb Z}/2{\mathbb Z})$ (continuous functions to $\{0,1\}$) gives a contravariant equivalence:
$$\{\text{0-dim compact systems}\}^{op} \simeq \{\text{Boolean algebras with automorphism}\}.$$

Under this duality, subshifts correspond to finitely generated Boolean algebras with automorphism — exactly the sofic shifts.

---

## 28.4 Categorical Entropy

### 28.4.1 Entropy as a Functor

**Theorem 28.4.1 (Leinster, 2011).** The entropy functor $H: \mathbf{FinProb} \to {\mathbb R}_{\geq 0}$ (from finite probability spaces to nonneg reals) is the unique functor satisfying:
1. $H(p_1, \ldots, p_n) = H(p_{\sigma(1)}, \ldots, p_{\sigma(n)})$ (symmetry)
2. $H(1) = 0$ (deterministic states have zero entropy)
3. $H(p_1, \ldots, p_n) = H(p_1 + p_2, p_3, \ldots, p_n) + (p_1+p_2)H\left(\frac{p_1}{p_1+p_2}, \frac{p_2}{p_1+p_2}\right)$ (chain rule)
4. $H(1/2, 1/2) = 1$

**Remark 28.4.2.** This is the categorification of the Faddeev (1956) axiomatization of entropy — the chain rule is the "functoriality" condition. The entropy function is uniquely determined by being a morphism from the category of finite probability spaces (with the composition of coarse-graining) to ${\mathbb R}_{\geq 0}$.

### 28.4.2 Categorical Dynamics and Enriched Categories

**Definition 28.4.3.** A *$V$-enriched dynamical system* for a monoidal category $V$ is an object $X$ in $V$ together with a morphism $f: X \to X$ in $V$.

**Example 28.4.4.** 
- $V = \mathbf{Set}$: ordinary discrete dynamical systems
- $V = \mathbf{Top}$: topological dynamical systems  
- $V = \mathbf{Meas}$: measurable dynamical systems
- $V = \mathbf{Hilb}$: quantum dynamical systems (CPTP maps as morphisms)

The enrichment perspective shows that "quantum dynamics" is just dynamics in the enriched category $\mathbf{Hilb}$.

---

## 28.5 Operator Algebras and Dynamics

### 28.5.1 C*-Algebras from Dynamical Systems

**Definition 28.5.1.** For a dynamical system $(X, f)$, the *crossed product* $C(X) \rtimes_f {\mathbb Z}$ is the C*-algebra generated by $C(X)$ and a unitary $U$ with $Ug = (g \circ f^{-1})U$ for $g \in C(X)$.

**Theorem 28.5.2.** Two minimal dynamical systems $(X, f)$ and $(Y, g)$ are orbit-equivalent iff $C(X) \rtimes_f {\mathbb Z} \cong C(Y) \rtimes_g {\mathbb Z}$ as C*-algebras (Giordano-Putnam-Skau, 1995).

**Remark 28.5.3.** The K-theory groups $K_0(C(X) \rtimes_f {\mathbb Z})$ and $K_1(C(X) \rtimes_f {\mathbb Z})$ are invariants of the orbit equivalence class. For minimal ${\mathbb Z}$-systems, these K-groups are complete orbit-equivalence invariants.

---

## Exercises

**Exercise 28.1.** Show that the category $\mathbf{Top.Dyn}$ has all finite products and coproducts. Describe the coproduct of $(X, f)$ and $(Y, g)$ explicitly.

**Exercise 28.2.** (Koopman Functor) For the doubling map $f(x) = 2x \pmod 1$ with Lebesgue measure: describe the Koopman operator $U_f$ on $L^2([0,1])$ using the Fourier basis $\{e^{2\pi inx}\}$. What is the spectrum of $U_f$?

**Exercise 28.3.** (Stone Duality) Show that the golden mean shift (forbidden word: $11$) corresponds to a finitely generated Boolean algebra. Describe the generators.

**Exercise 28.4.** Verify Leinster's axioms for Shannon entropy: show that the chain rule axiom is equivalent to the standard chain rule $H(X,Y) = H(X) + H(Y|X)$ for finite random variables.

---

## Chapter Notes

The category $\mathbf{Meas.Dyn}$ and the Koopman functor are discussed in Halmos's *Lectures on Ergodic Theory*. The groupoid and C*-algebra perspective is in Renault's *A Groupoid Approach to C*-Algebras* (1980) and the Giordano-Putnam-Skau papers in *J. Reine Angew. Math.* (1995, 1999).

Leinster's categorical characterization of entropy is in *A Characterization of Entropy in Terms of Information Loss* (Entropy, 2011). The topos-theoretic approach to dynamics connects to Lawvere's work on cohesion; see Lawvere-Schanuel's *Conceptual Mathematics* for background.

Operator algebras and dynamics: Davidson's *C*-Algebras by Example* (Chapter VIII) covers crossed products. Williams's *A Tour Through Mathematical Physics* connects these to quantum field theory.
