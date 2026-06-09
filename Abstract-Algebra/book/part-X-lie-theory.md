# Part X — Lie Theory

**Chapters 46–50**

---

## What This Part Establishes

Part X develops Lie theory: the study of continuous symmetry groups (Lie groups) and their infinitesimal algebraic counterparts (Lie algebras). Lie theory is the bridge between the discrete symmetry of finite group theory and the differential geometry of smooth manifolds. It provides the algebraic language for describing the symmetries of differential equations, quantum mechanics, gauge theories, and much of modern geometry.

Chapter 46 introduces Lie groups as groups that are simultaneously smooth manifolds, with the matrix Lie groups as the primary examples. Chapter 47 develops the Lie algebra of a Lie group — the tangent space at the identity — as the "infinitesimal" shadow of the group, connected via the exponential map. Chapter 48 develops the structural hierarchy of Lie algebras: solvable, nilpotent, and semisimple, paralleling the group-theoretic hierarchy, culminating in Cartan's criterion and Weyl's complete reducibility theorem. Chapter 49 develops the classification of semisimple Lie algebras via root systems and Dynkin diagrams — one of the most beautiful classification theorems in mathematics, reducing the infinite problem of classifying simple Lie algebras to a finite combinatorial problem whose answer fits in a short list. Chapter 50 proves the complete classification of finite-dimensional representations of semisimple Lie algebras via the highest weight theorem and computes the characters via the Weyl character formula.

By the end of Part X, the reader has mastered the classical Lie groups and their representations, understands root systems and Dynkin diagrams, and can compute with the representation theory of any semisimple Lie algebra.

---

## Internal Dependency Map

```
Ch 46 (Lie Groups: manifolds, matrix groups, homomorphisms)
         |
         v
Ch 47 (Lie Algebras: tangent space, bracket, exponential map)
         |
         v
Ch 48 (Solvable, Nilpotent, Semisimple Lie Algebras)
         |
         v
Ch 49 (Root Systems, Dynkin Diagrams, Classification)
         |
         v
Ch 50 (Highest Weight Theory, Weyl Character Formula)
```

---

## Chapter 46 — Lie Groups

**What it establishes:** Lie groups as groups carrying a compatible smooth manifold structure; the classical matrix groups as the primary examples; Lie group homomorphisms, covering groups, and the relationship between a Lie group and its subgroups.

---

**46.1 Smooth Manifolds (Background)**

**46.1.1 What a Smooth Manifold Is: Coordinate Charts and Transitions**
A *smooth $n$-manifold* is a topological space$M$ covered by open sets$U_\alpha$, each equipped with a homeomorphism$\phi_\alpha: U_\alpha \to \mathbb{R}^n$ (a *coordinate chart*), such that the *transition maps*$\phi_\beta \circ \phi_\alpha^{-1}: \phi_\alpha(U_\alpha \cap U_\beta) \to \phi_\beta(U_\alpha \cap U_\beta)$ are smooth (infinitely differentiable) where defined. The coordinate charts give local Euclidean structure; the smooth transition maps ensure that "smoothness" is a well-defined concept independent of which chart one uses.

**46.1.2 Smooth Functions and Smooth Maps**
A function $f: M \to \mathbb{R}$ on a smooth manifold is *smooth* if$f \circ \phi_\alpha^{-1}: \mathbb{R}^n \to \mathbb{R}$ is smooth for every chart$\phi_\alpha$. A map$F: M \to N$ between smooth manifolds is smooth if$\psi_\beta \circ F \circ \phi_\alpha^{-1}$ is smooth for all compatible charts. The composition of smooth maps is smooth. Smooth manifolds and smooth maps form a category **Mfd**.

**46.1.3 The Tangent Space at a Point**
The *tangent space* $T_p M$ at a point$p \in M$ is an$n$-dimensional real vector space whose elements (tangent vectors) can be thought of as: (a) equivalence classes of curves$\gamma: (-\varepsilon, \varepsilon) \to M$ with$\gamma(0) = p$ (two curves equivalent iff they have the same velocity in every chart); or (b) derivations on the algebra of smooth functions at$p$: linear maps$v: C^\infty(M) \to \mathbb{R}$ with$v(fg) = v(f)g(p) + f(p)v(g)$. A smooth map$F: M \to N$ induces a linear map$dF_p: T_p M \to T_{F(p)} N$ (the *differential* of$F$ at$p$).

---

**46.2 Lie Groups**

**46.2.1 Definition: A Group That Is Also a Smooth Manifold, with Smooth Operations**
A *Lie group* is a smooth manifold $G$ equipped with a group structure such that the multiplication map$\mu: G \times G \to G$,$(g, h) \mapsto gh$, and the inversion map$\iota: G \to G$,$g \mapsto g^{-1}$, are both smooth. Lie groups are the mathematical objects encoding "continuous symmetry": groups where the group elements vary continuously and the group operations are smooth.

**46.2.2 Matrix Lie Groups: Closed Subgroups of $GL_n$**
A *matrix Lie group* is a closed subgroup of $GL_n(\mathbb{R})$ or$GL_n(\mathbb{C})$. Every closed subgroup of$GL_n$ is a Lie group (a theorem: closed subgroups of Lie groups are Lie groups, and$GL_n$ is a Lie group with the manifold structure from$M_n(\mathbb{R}) \cong \mathbb{R}^{n^2}$). Matrix Lie groups are the most important examples and suffice for essentially all applications in algebra and physics.

**46.2.3 The Classical Groups: $GL_n$,$SL_n$,$O(n)$,$SO(n)$,$U(n)$,$SU(n)$,$Sp(2n)$**
The *classical Lie groups*: $GL_n(\mathbb{R})$ (invertible$n \times n$ real matrices, dimension$n^2$);$SL_n(\mathbb{R})$ (det = 1, dimension$n^2 - 1$);$O(n)$ (orthogonal matrices$A^T A = I$, dimension$n(n-1)/2$);$SO(n)$ (special orthogonal,$O(n) \cap SL_n$);$U(n)$ (unitary matrices$A^* A = I$, dimension$n^2$);$SU(n)$ (special unitary, dimension$n^2 - 1$);$Sp(2n)$ (symplectic matrices preserving a non-degenerate skew form, dimension$n(2n+1)$). These groups are the symmetry groups of the fundamental geometric structures (Euclidean, Hermitian, symplectic).

**46.2.4 Compact and Non-Compact Lie Groups**
A Lie group is *compact* if its underlying manifold is compact (closed and bounded in any embedding). The compact classical groups are $O(n)$,$SO(n)$,$U(n)$,$SU(n)$, and$Sp(n)$; the non-compact ones include$GL_n$,$SL_n$, and the symplectic group$Sp(2n, \mathbb{R})$. Compact Lie groups have a natural invariant integral (the Haar measure, finite volume), making their representation theory as clean as that of finite groups. Non-compact groups have infinite-dimensional unitary representations (the setting of the Langlands program).

**46.2.5 One-Parameter Subgroups: Smooth Homomorphisms $\mathbb{R} \to G$**
A *one-parameter subgroup* of a Lie group $G$ is a smooth group homomorphism$\gamma: (\mathbb{R}, +) \to G$. Every one-parameter subgroup satisfies$\gamma(s + t) = \gamma(s)\gamma(t)$ and$\gamma(0) = e$. For matrix groups, one-parameter subgroups are of the form$\gamma(t) = e^{tX}$ for some fixed matrix$X$. The collection of all one-parameter subgroups of$G$ — parameterized by their velocity$\gamma'(0)$ at$e$ — is the Lie algebra of$G$.

---

**46.3 Lie Group Homomorphisms**

**46.3.1 Smooth Group Homomorphisms**
A *Lie group homomorphism* is a smooth map $\phi: G \to H$ between Lie groups that is also a group homomorphism. The differential$d\phi_e: T_e G \to T_e H$ is a linear map between the Lie algebras$\mathfrak{g} = T_e G$ and$\mathfrak{h} = T_e H$. Every Lie group homomorphism induces a Lie algebra homomorphism. The theory of Lie groups is related to the theory of Lie algebras via this differentiation functor.

**46.3.2 The Kernel and Image of a Lie Group Map**
The kernel of a Lie group homomorphism $\phi: G \to H$ is a closed normal subgroup of$G$ (hence a Lie group). The image is an immersed Lie subgroup of$H$. The first isomorphism theorem holds:$G/\ker\phi \cong \mathrm{im}\,\phi$ as Lie groups. These basic structural results parallel the group-theoretic isomorphism theorems and hold in the smooth category.

**46.3.3 Discrete Subgroups; Quotient Lie Groups**
A *discrete subgroup* $\Gamma \leq G$ is a closed subgroup with no accumulation points — a subgroup that looks like a group of isolated points inside$G$. If$\Gamma$ is also normal in$G$, the quotient$G/\Gamma$ is a smooth manifold and a Lie group, with the projection$G \to G/\Gamma$ a smooth Lie group homomorphism. Example:$\mathbb{R}/\mathbb{Z} \cong S^1$ (the circle group, a compact Lie group);$SL_2(\mathbb{R})/SL_2(\mathbb{Z})$ (a modular surface, the base of the theory of modular forms).

**46.3.4 Covering Groups and the Universal Cover**
Every connected Lie group $G$ has a *universal covering group*$\tilde G$: a simply connected Lie group with a covering map$\pi: \tilde G \to G$ that is a Lie group homomorphism with discrete kernel$\pi_1(G)$ (the fundamental group of$G$). Example:$SU(2) \to SO(3)$ is a 2-fold covering (since$\pi_1(SO(3)) = \mathbb{Z}/2\mathbb{Z}$); the spin group$\mathrm{Spin}(n)$ is the universal cover of$SO(n)$. Covering theory for Lie groups parallels covering theory for topological spaces and is the bridge to the algebraic theory of the fundamental group.

---

## Chapter 47 — Lie Algebras

**What it establishes:** The Lie algebra of a Lie group as the tangent space at the identity equipped with the Lie bracket; the abstract axioms of a Lie algebra; the exponential map connecting the Lie algebra to the Lie group; and the adjoint representation.

---

**47.1 The Lie Algebra of a Lie Group**

**47.1.1 $\mathfrak{g} = T_e G$: The Tangent Space at the Identity**
The *Lie algebra* $\mathfrak{g}$ of a Lie group$G$ is the tangent space$T_e G$ at the identity element$e$. As a vector space (over$\mathbb{R}$),$\mathfrak{g}$ has dimension equal to the dimension of$G$ as a manifold. For matrix groups,$\mathfrak{g}$ consists of the matrices$X$ such that$e^{tX} \in G$ for all$t \in \mathbb{R}$ (the "velocities of curves in$G$ passing through$I$"). The Lie algebra encodes the infinitesimal behavior of the group near the identity.

**47.1.2 Left-Invariant Vector Fields and the Lie Bracket**
A *vector field* on $G$ assigns a tangent vector$X_g \in T_g G$ to each point$g \in G$ (smoothly). A vector field$X$ is *left-invariant* if$d(L_g)_h(X_h) = X_{gh}$ for all$g, h$ (where$L_g$ is left multiplication by$g$). Left-invariant vector fields on$G$ are in bijection with$T_e G = \mathfrak{g}$ (each tangent vector at$e$ extends uniquely to a left-invariant vector field). The *Lie bracket*$[X, Y]$ of two vector fields is defined as the commutator of their derivations on smooth functions; for left-invariant vector fields, the bracket is again left-invariant. This gives$\mathfrak{g}$ the Lie algebra structure.

**47.1.3 The Lie Algebra of a Matrix Group: $\{X : e^{tX} \in G\}$**
For a matrix Lie group $G \leq GL_n$, the Lie algebra is$\mathfrak{g} = \{X \in M_n : e^{tX} \in G \text{ for all } t \in \mathbb{R}\}$, and the Lie bracket is the commutator of matrices:$[X, Y] = XY - YX$. Examples:$\mathrm{Lie}(GL_n(\mathbb{R})) = \mathfrak{gl}_n(\mathbb{R}) = M_n(\mathbb{R})$;$\mathrm{Lie}(SL_n) = \mathfrak{sl}_n = \{X : \mathrm{tr}(X) = 0\}$ (trace-free matrices, since$\det(e^X) = e^{\mathrm{tr}(X)} = 1$);$\mathrm{Lie}(O(n)) = \mathfrak{o}(n) = \{X : X^T = -X\}$ (skew-symmetric matrices, since$(e^X)^T = e^{X^T} = e^{-X} = (e^X)^{-1}$).

**47.1.4 Functoriality: A Lie Group Map Induces a Lie Algebra Map**
A Lie group homomorphism $\phi: G \to H$ induces a Lie algebra homomorphism$d\phi_e: \mathfrak{g} \to \mathfrak{h}$ by differentiating at the identity. The induced map preserves the Lie bracket:$d\phi_e([X, Y]) = [d\phi_e(X), d\phi_e(Y)]$. The passage$G \mapsto \mathfrak{g}$,$\phi \mapsto d\phi_e$ is a functor from the category of Lie groups to the category of Lie algebras. This functor is an equivalence between simply connected Lie groups and Lie algebras (Lie's third theorem): every Lie algebra is the Lie algebra of a unique simply connected Lie group.

---

**47.2 The Lie Algebra Axioms**

**47.2.1 Abstract Lie Algebras: Bilinear, Antisymmetric, Jacobi Identity**
An *abstract Lie algebra* over a field $k$ is a$k$-vector space$\mathfrak{g}$ equipped with a bilinear map$[\cdot, \cdot]: \mathfrak{g} \times \mathfrak{g} \to \mathfrak{g}$ (the *Lie bracket*) satisfying: (i) *antisymmetry*:$[X, X] = 0$ for all$X$ (equivalently,$[X, Y] = -[Y, X]$); (ii) *Jacobi identity*:$[X,[Y,Z]] + [Y,[Z,X]] + [Z,[X,Y]] = 0$ for all$X, Y, Z$. The Jacobi identity is the key non-trivial axiom; it says the bracket satisfies a "twisted associativity" —$\mathrm{ad}(X) = [X,-]$ is a derivation of the bracket.

**47.2.2 Lie Subalgebras and Ideals**
A *Lie subalgebra* of $\mathfrak{g}$ is a subspace$\mathfrak{h} \subseteq \mathfrak{g}$ closed under the bracket:$[\mathfrak{h}, \mathfrak{h}] \subseteq \mathfrak{h}$. An *ideal* is a subspace$\mathfrak{a} \subseteq \mathfrak{g}$ with$[\mathfrak{g}, \mathfrak{a}] \subseteq \mathfrak{a}$ (equivalently,$[\mathfrak{a}, \mathfrak{g}] \subseteq \mathfrak{a}$, since the bracket is antisymmetric). The quotient$\mathfrak{g}/\mathfrak{a}$ inherits a Lie algebra structure from$\mathfrak{g}$ iff$\mathfrak{a}$ is an ideal. The correspondence between ideals in$\mathfrak{g}$ and normal subgroups in$G$ (via the exponential map) is one of the key correspondences in Lie theory.

**47.2.3 Lie Algebra Homomorphisms and Isomorphisms**
A *Lie algebra homomorphism* $\phi: \mathfrak{g} \to \mathfrak{h}$ is a linear map preserving the bracket:$\phi([X,Y]) = [\phi(X), \phi(Y)]$. Kernels and images are subalgebras; kernels are ideals. The isomorphism theorems hold for Lie algebras. A Lie algebra homomorphism is an isomorphism if it is a bijective linear map whose inverse is also a Lie algebra homomorphism (equivalently, a bijective homomorphism, since the inverse of a bijective linear map preserving the bracket also preserves the bracket).

**47.2.4 The Lie Algebras of the Classical Groups**
The Lie algebras of the classical groups: $\mathfrak{gl}_n(\mathbb{R}) = M_n(\mathbb{R})$ with$[X,Y] = XY - YX$;$\mathfrak{sl}_n = \ker(\mathrm{tr}: \mathfrak{gl}_n \to \mathbb{R})$;$\mathfrak{so}(n) = \{X \in M_n : X + X^T = 0\}$ (skew-symmetric matrices);$\mathfrak{u}(n) = \{X \in M_n(\mathbb{C}) : X + X^* = 0\}$ (skew-Hermitian);$\mathfrak{su}(n) = \mathfrak{u}(n) \cap \mathfrak{sl}_n(\mathbb{C})$;$\mathfrak{sp}(2n) = \{X : X^T J + JX = 0\}$ (where$J$ is the standard symplectic form). These Lie algebras will be classified by the Dynkin diagrams$A_n, B_n, C_n, D_n$ in Chapter 49.

---

**47.3 The Exponential Map**

**47.3.1 $\exp: \mathfrak{g} \to G$ for Matrix Groups:$\exp(X) = e^X = \sum X^n/n!$**
For a matrix Lie group $G \leq GL_n$, the *exponential map*$\exp: \mathfrak{g} \to G$ is the matrix exponential:$\exp(X) = e^X = \sum_{n=0}^\infty X^n/n!$. The series converges for all$X \in M_n$ (since$\|X^n/n!\| \leq \|X\|^n/n! \to 0$). Key properties:$e^0 = I$;$\frac{d}{dt}|_{t=0} e^{tX} = X$ (the tangent vector at$t=0$ is$X$, consistent with$\mathfrak{g} = T_e G$);$\det(e^X) = e^{\mathrm{tr}(X)}$ (explaining why$\mathrm{tr}(X) = 0$ for$X \in \mathfrak{sl}_n$).

**47.3.2 $\exp$ Is a Local Diffeomorphism; Surjectivity for Compact$G$**
The exponential map is a local diffeomorphism near $0 \in \mathfrak{g}$: the derivative of$\exp$ at$0$ is the identity, so by the inverse function theorem,$\exp$ is a diffeomorphism from a neighborhood of$0$ in$\mathfrak{g}$ onto a neighborhood of$e$ in$G$. For compact connected Lie groups,$\exp$ is surjective: every element$g \in G$ is$e^X$ for some$X \in \mathfrak{g}$. Surjectivity fails for non-compact groups (e.g., not every matrix in$SL_2(\mathbb{R})$ is an exponential).

**47.3.3 The Baker–Campbell–Hausdorff Formula**
For matrices $X, Y$ with$\|X\| + \|Y\|$ small,$e^X e^Y = e^{Z}$ where$Z = X + Y + \frac{1}{2}[X,Y] + \frac{1}{12}([X,[X,Y]] - [Y,[X,Y]]) + \cdots$ (the *Baker–Campbell–Hausdorff formula*). The BCH formula expresses the product of two exponentials as a single exponential, with the exponent given by an infinite series of nested brackets. It shows that the local group structure of$G$ near$e$ is completely determined by the Lie algebra structure of$\mathfrak{g}$ — the Lie algebra determines the Lie group locally.

**47.3.4 The Dictionary: Subgroups $\leftrightarrow$ Subalgebras, Ideals$\leftrightarrow$ Normal Subgroups**
For a connected Lie group $G$ with Lie algebra$\mathfrak{g}$: connected Lie subgroups of$G$ correspond to Lie subalgebras of$\mathfrak{g}$; connected normal subgroups correspond to ideals. The exponential map converts subalgebra structure (in$\mathfrak{g}$) to subgroup structure (in$G$). The Lie correspondence (Lie's theorem) gives a perfect dictionary: connected simply connected Lie groups are completely determined by their Lie algebras, and the dictionary between them is the exponential map.

---

**47.4 The Adjoint Representation**

**47.4.1 $\mathrm{Ad}: G \to GL(\mathfrak{g})$: Conjugation Acts on the Lie Algebra**
The *adjoint representation* $\mathrm{Ad}: G \to GL(\mathfrak{g})$ is defined by$\mathrm{Ad}(g)(X) = d(\mathrm{conj}_g)_e(X)$, where$\mathrm{conj}_g: h \mapsto ghg^{-1}$ is conjugation by$g$. For matrix groups:$\mathrm{Ad}(g)(X) = gXg^{-1}$ (conjugation of matrices). The adjoint representation is the canonical representation of$G$ on its own Lie algebra; it captures how the group acts on its own infinitesimal structure.

**47.4.2 $\mathrm{ad}: \mathfrak{g} \to \mathfrak{gl}(\mathfrak{g})$:$\mathrm{ad}(X)(Y) = [X,Y]$**
The differential of $\mathrm{Ad}$ at the identity is the *adjoint representation of the Lie algebra*:$\mathrm{ad}: \mathfrak{g} \to \mathfrak{gl}(\mathfrak{g})$,$\mathrm{ad}(X)(Y) = [X, Y]$. For matrix groups:$\mathrm{ad}(X)(Y) = \frac{d}{dt}|_{t=0} e^{tX} Y e^{-tX} = XY - YX = [X, Y]$. The adjoint representation of the Lie algebra on itself encodes the bracket structure as a linear map.

**47.4.3 The Jacobi Identity = $\mathrm{ad}$ Is a Lie Algebra Homomorphism**
The Jacobi identity $[X,[Y,Z]] = [[X,Y],Z] + [Y,[X,Z]]$ (equivalently,$\mathrm{ad}(X)([Y,Z]) = [\mathrm{ad}(X)(Y), Z] + [Y, \mathrm{ad}(X)(Z)]$) says exactly that$\mathrm{ad}(X)$ is a *derivation* of the Lie bracket. Equivalently,$\mathrm{ad}: \mathfrak{g} \to \mathfrak{gl}(\mathfrak{g})$ is a Lie algebra homomorphism:$\mathrm{ad}([X,Y]) = [\mathrm{ad}(X), \mathrm{ad}(Y)]$ (where the bracket in$\mathfrak{gl}(\mathfrak{g})$ is the commutator of linear maps). The Jacobi identity is the abstract reformulation of the property "the adjoint is a homomorphism."

**47.4.4 The Killing Form $B(X,Y) = \mathrm{tr}(\mathrm{ad}(X)\mathrm{ad}(Y))$**
The *Killing form* is the symmetric bilinear form $B: \mathfrak{g} \times \mathfrak{g} \to k$ defined by$B(X,Y) = \mathrm{tr}(\mathrm{ad}(X) \circ \mathrm{ad}(Y))$. The Killing form is$G$-invariant (i.e.,$B(\mathrm{Ad}(g)X, \mathrm{Ad}(g)Y) = B(X,Y)$) and associative ($B([X,Y],Z) = B(X,[Y,Z])$). Its non-degeneracy characterizes semisimple Lie algebras (Cartan's criterion). The Killing form is the canonical "inner product" on$\mathfrak{g}$ that the structure of$\mathfrak{g}$ produces, analogous to the normalized Killing form used to define roots.

---

## Chapter 48 — Solvable, Nilpotent, and Semisimple Lie Algebras

**What it establishes:** The structural hierarchy of Lie algebras — solvable and nilpotent algebras at the "bottom," semisimple algebras at the "top" — with the Levi decomposition showing every Lie algebra is built from these.

---

**48.1 Solvable Lie Algebras**

**48.1.1 The Derived Series $\mathfrak{g}^{(0)} \supset \mathfrak{g}^{(1)} \supset \cdots$**
The *derived series* of a Lie algebra $\mathfrak{g}$ is$\mathfrak{g}^{(0)} = \mathfrak{g}$,$\mathfrak{g}^{(1)} = [\mathfrak{g}, \mathfrak{g}]$,$\mathfrak{g}^{(2)} = [\mathfrak{g}^{(1)}, \mathfrak{g}^{(1)}]$, and in general$\mathfrak{g}^{(n+1)} = [\mathfrak{g}^{(n)}, \mathfrak{g}^{(n)}]$. Each term$\mathfrak{g}^{(n)}$ is an ideal in$\mathfrak{g}$ (since ideals are preserved under the bracket). This parallels the derived series for groups. The quotient$\mathfrak{g}/\mathfrak{g}^{(1)} = \mathfrak{g}/[\mathfrak{g},\mathfrak{g}]$ is the abelianization of$\mathfrak{g}$.

**48.1.2 Solvable Lie Algebras; Examples and Non-Examples**
$\mathfrak{g}$ is *solvable* if its derived series reaches zero:$\mathfrak{g}^{(n)} = 0$ for some$n$. Every abelian Lie algebra ($[\mathfrak{g},\mathfrak{g}] = 0$) is solvable. The Borel subalgebra$\mathfrak{b} = \{$ upper triangular matrices in$\mathfrak{gl}_n\}$ is solvable (since the derived series reaches the strictly upper triangular matrices, which form a nilpotent algebra). The algebra$\mathfrak{sl}_2(\mathbb{C})$ is not solvable (it is simple). Solvable Lie algebras are the "building blocks" from which any Lie algebra is built (in the sense of the Levi decomposition).

**48.1.3 Lie's Theorem: Solvable Actions over $\mathbb{C}$ Have a Common Eigenvector**
*Lie's Theorem:* If $\mathfrak{g}$ is a solvable Lie algebra over$\mathbb{C}$ and$V$ is a nonzero finite-dimensional$\mathfrak{g}$-module (representation), then there exists a vector$v \in V$ that is a common eigenvector for all operators in$\mathfrak{g}$:$X \cdot v = \lambda(X) v$ for some linear function$\lambda: \mathfrak{g} \to \mathbb{C}$. Lie's theorem is the Lie algebra analogue of the fact that commuting linear maps over$\mathbb{C}$ share a common eigenvector.

**48.1.4 Corollary: Solvable Algebras Are Upper-Triangularizable**
By Lie's theorem applied repeatedly (find the first common eigenvector, quotient, find the next), every representation of a solvable Lie algebra over $\mathbb{C}$ can be put in upper triangular form (with respect to some basis of$V$). Equivalently: the image of any representation of a solvable Lie algebra in$\mathfrak{gl}_n(\mathbb{C})$ can be simultaneously upper-triangularized. This is the Lie algebra version of the Borel fixed-point theorem.

---

**48.2 Nilpotent Lie Algebras**

**48.2.1 The Lower Central Series; Nilpotent Lie Algebras**
The *lower central series* is $\mathfrak{g}^0 = \mathfrak{g}$,$\mathfrak{g}^1 = [\mathfrak{g}, \mathfrak{g}]$,$\mathfrak{g}^{n+1} = [\mathfrak{g}, \mathfrak{g}^n]$. Unlike the derived series (which brackets each term with itself), the lower central series brackets each term with the full$\mathfrak{g}$. The algebra$\mathfrak{g}$ is *nilpotent* if$\mathfrak{g}^n = 0$ for some$n$. Nilpotent implies solvable (the lower central series decreases at least as fast as the derived series). Examples: the Heisenberg algebra$\mathfrak{h}_n$ (nilpotent); strictly upper triangular matrices$\mathfrak{n}$ (nilpotent);$\mathfrak{sl}_2$ (not nilpotent).

**48.2.2 Engel's Theorem: $\mathrm{ad}(X)$ Nilpotent for All$X$ $\Leftrightarrow$ $\mathfrak{g}$ Nilpotent**
*Engel's Theorem:* A Lie algebra $\mathfrak{g}$ (finite-dimensional, over any field) is nilpotent iff$\mathrm{ad}(X): \mathfrak{g} \to \mathfrak{g}$ is a nilpotent linear map for every$X \in \mathfrak{g}$. The key step: if every$\mathrm{ad}(X)$ is nilpotent, then$\mathfrak{g}$ acts on itself (via$\mathrm{ad}$) as a nilpotent Lie algebra, and by Engel's theorem for representations, there is a common eigenvector with eigenvalue 0 (i.e., a central element). Inducting gives nilpotency. Engel's theorem characterizes nilpotency intrinsically (without reference to a filtration).

**48.2.3 Nilpotent $\Rightarrow$ Solvable; the Heisenberg Algebra**
Every nilpotent Lie algebra is solvable. The *Heisenberg algebra* $\mathfrak{h}$ is the 3-dimensional Lie algebra with basis$\{e, f, z\}$ and brackets$[e, f] = z$,$[e, z] = [f, z] = 0$. The derived subalgebra$[\mathfrak{h}, \mathfrak{h}] = \mathrm{span}(z)$ is central (lies in$Z(\mathfrak{h})$), so the lower central series is$\mathfrak{h} \supset \mathrm{span}(z) \supset 0$ — nilpotent of class 2. The Heisenberg algebra is the infinitesimal symmetry of the Weyl algebra (canonical commutation relations in quantum mechanics).

---

**48.3 Semisimple Lie Algebras**

**48.3.1 Simple and Semisimple Lie Algebras**
A Lie algebra $\mathfrak{g}$ is *simple* if$\dim \mathfrak{g} > 1$,$\mathfrak{g}$ is non-abelian, and$\mathfrak{g}$ has no proper nonzero ideals. It is *semisimple* if it is a direct sum of simple ideals:$\mathfrak{g} = \mathfrak{g}_1 \oplus \cdots \oplus \mathfrak{g}_r$ with each$\mathfrak{g}_i$ simple. Semisimple Lie algebras are the non-abelian counterpart of fields — the "completely irreducible" Lie algebras. The classification of simple Lie algebras (Chapter 49) is one of the great achievements of mathematics.

**48.3.2 Cartan's Criterion: Non-Degenerate Killing Form $\Leftrightarrow$ Semisimple**
*Cartan's Criterion:* A Lie algebra $\mathfrak{g}$ (over$\mathbb{C}$ or any algebraically closed field of characteristic 0) is semisimple iff its Killing form$B(X,Y) = \mathrm{tr}(\mathrm{ad}(X)\mathrm{ad}(Y))$ is non-degenerate. The proof: if$\mathfrak{g}$ has a solvable ideal$\mathfrak{a}$, then by Lie's theorem the elements of$\mathfrak{a}$ act by upper triangular matrices in any faithful representation, and a computation shows$B$ is degenerate on$\mathfrak{a}$. Conversely, if$B$ is non-degenerate, Engel's theorem shows$\mathfrak{g}$ has no nonzero abelian ideals, and a further argument shows it is semisimple.

**48.3.3 Weyl's Theorem: Every Representation of a Semisimple Algebra Is Completely Reducible**
*Weyl's Complete Reducibility Theorem:* Every finite-dimensional representation of a semisimple Lie algebra $\mathfrak{g}$ over$\mathbb{C}$ is completely reducible — a direct sum of irreducibles. The proof uses the Casimir element of$\mathfrak{g}$ (an element of the universal enveloping algebra$\mathcal{U}(\mathfrak{g})$ that acts by a scalar on each irreducible representation): by Schur's lemma, the Casimir separates irreducibles, allowing one to construct equivariant projections onto subrepresentations. Weyl's theorem is the Lie algebra analogue of Maschke's theorem.

**48.3.4 The Radical; Levi Decomposition: $\mathfrak{g} = \mathfrak{s} \ltimes \mathrm{rad}(\mathfrak{g})$**
The *radical* $\mathrm{rad}(\mathfrak{g})$ of a Lie algebra$\mathfrak{g}$ is the largest solvable ideal. The *Levi decomposition* (Levi's theorem): every finite-dimensional Lie algebra decomposes as$\mathfrak{g} = \mathfrak{s} \ltimes \mathrm{rad}(\mathfrak{g})$, where$\mathfrak{s}$ is a semisimple subalgebra (a *Levi factor*) and$\ltimes$ denotes the semidirect product (with$\mathfrak{s}$ acting on$\mathrm{rad}(\mathfrak{g})$ via the adjoint). The Levi factor is unique up to conjugation. The Levi decomposition reduces the study of arbitrary Lie algebras to the study of semisimple algebras and their actions on solvable algebras.

---

## Chapter 49 — Root Systems and Dynkin Diagrams

**What it establishes:** The complete classification of semisimple Lie algebras over $\mathbb{C}$ via the combinatorial data of root systems, encoded in Dynkin diagrams — reducing an infinite problem to a finite list of classical and exceptional types.

---

**49.1 The Root Space Decomposition**

**49.1.1 Cartan Subalgebras: Maximal Abelian, $\mathrm{ad}$-Diagonalizable**
A *Cartan subalgebra* $\mathfrak{h} \subseteq \mathfrak{g}$ is a maximal abelian subalgebra such that$\mathrm{ad}(H)$ is diagonalizable (on$\mathfrak{g}$) for all$H \in \mathfrak{h}$. For$\mathfrak{sl}_n$,$\mathfrak{h}$ = diagonal matrices with trace 0. Cartan subalgebras exist in any semisimple$\mathfrak{g}$ over$\mathbb{C}$ and are unique up to conjugation by the Lie group. The dimension of$\mathfrak{h}$ is the *rank* of$\mathfrak{g}$.

**49.1.2 Root Spaces $\mathfrak{g}_\alpha$; the Root System$\Phi \subset \mathfrak{h}^*$**
Since $\mathfrak{h}$ acts on$\mathfrak{g}$ by commuting diagonalizable operators,$\mathfrak{g}$ decomposes into simultaneous eigenspaces:$\mathfrak{g} = \mathfrak{h} \oplus \bigoplus_{\alpha \in \Phi} \mathfrak{g}_\alpha$, where$\mathfrak{g}_\alpha = \{X \in \mathfrak{g} : [H, X] = \alpha(H) X \text{ for all } H \in \mathfrak{h}\}$ and$\alpha \in \mathfrak{h}^* \setminus \{0\}$ are the *roots*. Each root space$\mathfrak{g}_\alpha$ has dimension 1 (a non-trivial theorem). The set$\Phi \subset \mathfrak{h}^*$ of roots is the *root system* of$\mathfrak{g}$.

**49.1.3 $\mathfrak{sl}_2$-Triples from Each Root; Root Strings**
For each root $\alpha \in \Phi$, there is an "$\mathfrak{sl}_2$-triple"$\{e_\alpha \in \mathfrak{g}_\alpha, f_\alpha \in \mathfrak{g}_{-\alpha}, h_\alpha \in \mathfrak{h}\}$ with$[e_\alpha, f_\alpha] = h_\alpha$,$[h_\alpha, e_\alpha] = 2e_\alpha$,$[h_\alpha, f_\alpha] = -2f_\alpha$. The *root string through$\beta$ for$\alpha$* is the sequence$\beta - p\alpha, \ldots, \beta, \ldots, \beta + q\alpha$ (the string of roots of the form$\beta + n\alpha$,$n \in \mathbb{Z}$). The representation theory of$\mathfrak{sl}_2$ constrains these strings:$p - q = \langle \beta, \alpha^\vee \rangle$ (an integer called the Cartan integer).

**49.1.4 Properties of the Root System**
The root system $\Phi \subset \mathfrak{h}^* \cong \mathbb{R}^\ell$ (where$\ell = \mathrm{rank}(\mathfrak{g})$) satisfies: (i)$\Phi$ spans$\mathfrak{h}^*$; (ii) if$\alpha \in \Phi$, then$-\alpha \in \Phi$ (and$n\alpha \notin \Phi$ for$n \neq \pm 1$); (iii) for all$\alpha, \beta \in \Phi$,$\langle \beta, \alpha^\vee \rangle = 2\frac{\langle \beta, \alpha \rangle}{\langle \alpha, \alpha \rangle} \in \mathbb{Z}$; (iv) the reflection$s_\alpha(\beta) = \beta - \langle \beta, \alpha^\vee \rangle \alpha$ sends$\Phi$ to itself. These properties define an abstract root system, independent of the Lie algebra.

---

**49.2 Abstract Root Systems**

**49.2.1 Axiomatic Definition of a Root System in a Euclidean Space**
An *abstract root system* in a Euclidean space $(V, \langle \cdot, \cdot \rangle)$ is a finite set$\Phi \subset V \setminus \{0\}$ satisfying: (i)$\Phi$ spans$V$; (ii) if$\alpha \in \Phi$, then$\mathbb{R}\alpha \cap \Phi = \{\pm \alpha\}$; (iii) for all$\alpha, \beta \in \Phi$,$\langle \beta, \alpha^\vee \rangle \in \mathbb{Z}$ (integrality); (iv)$s_\alpha(\Phi) = \Phi$ for all$\alpha \in \Phi$ (the Weyl group$W$ acts on$\Phi$). The rank of the root system is$\dim V$. Two root systems are isomorphic iff there is an orthogonal isomorphism of their ambient spaces sending one to the other.

**49.2.2 Positive Roots and Simple Roots; the Weyl Chamber**
A *base* of $\Phi$ is a subset$\Delta \subset \Phi$ such that$\Delta$ is a basis of$V$ and every root is either a non-negative or non-positive$\mathbb{Z}$-linear combination of elements of$\Delta$. Elements of$\Delta$ are *simple roots*$\alpha_1, \ldots, \alpha_\ell$. Roots that are positive combinations of$\Delta$ are the *positive roots*$\Phi^+$; negative combinations give$\Phi^-$. A *Weyl chamber* is a connected component of$V \setminus \bigcup_{\alpha \in \Phi} H_\alpha$ (where$H_\alpha$ is the hyperplane perpendicular to$\alpha$); each Weyl chamber determines a unique set of simple roots.

**49.2.3 The Cartan Matrix $A_{ij} = \langle \alpha_i, \alpha_j^\vee \rangle$**
The *Cartan matrix* of a root system with simple roots $\alpha_1, \ldots, \alpha_\ell$ is the$\ell \times \ell$ integer matrix$A_{ij} = \langle \alpha_i, \alpha_j^\vee \rangle = 2\frac{\langle \alpha_i, \alpha_j \rangle}{\langle \alpha_j, \alpha_j \rangle}$. Diagonal entries:$A_{ii} = 2$. Off-diagonal entries:$A_{ij} \in \{0, -1, -2, -3\}$ for$i \neq j$ (with the constraint$A_{ij} A_{ji} \in \{0, 1, 2, 3\}$). The Cartan matrix encodes all information about the root system.

**49.2.4 The Weyl Group: Generated by Simple Reflections**
The *Weyl group* $W$ of a root system is the finite group generated by the reflections$s_{\alpha_i}$ (simple reflections).$W$ acts faithfully on$\Phi$, on the dual$\mathfrak{h}^*$, and on the weight lattice$P = \{v \in V : \langle v, \alpha^\vee \rangle \in \mathbb{Z} \text{ for all } \alpha\}$. The Weyl group is always finite (since it is a reflection group in a Euclidean space). For$\mathfrak{sl}_{n+1}$,$W = S_{n+1}$ (the symmetric group), acting by permuting the simple roots and the standard basis vectors.

**49.2.5 Length Function; the Longest Element $w_0$**
The *length* $\ell(w)$ of$w \in W$ is the minimum number of simple reflections in any expression for$w$. The unique element$w_0 \in W$ of maximum length is the *longest element*; it sends$\Phi^+$ to$\Phi^-$ and satisfies$w_0^2 = \mathrm{id}$. The longest element is$-\mathrm{id}$ for type$A_1$ and$E_8$ (but not for all types). The length function and Bruhat order on$W$ are central to the Kazhdan–Lusztig theory (Chapter 52).

---

**49.3 Classification by Dynkin Diagrams**

**49.3.1 Dynkin Diagrams: Encoding the Cartan Matrix**
A *Dynkin diagram* encodes the Cartan matrix: draw one node for each simple root; connect nodes $i$ and$j$ with$A_{ij} A_{ji}$ edges (0, 1, 2, or 3 edges); if$A_{ij} \neq A_{ji}$, add an arrow pointing toward the shorter root. The Dynkin diagram is a labeled graph that completely determines the root system (and hence the simple Lie algebra) up to isomorphism.

**49.3.2 The Classification: $A_n$,$B_n$,$C_n$,$D_n$,$E_6$,$E_7$,$E_8$,$F_4$,$G_2$**
The classification of irreducible root systems (equivalently, of simple Lie algebras over $\mathbb{C}$): *Classical types*:$A_n$ ($n \geq 1$, Dynkin diagram$\circ - \circ - \cdots - \circ$, corresponding to$\mathfrak{sl}_{n+1}$);$B_n$ ($n \geq 2$, double arrow at the end,$\mathfrak{so}(2n+1)$);$C_n$ ($n \geq 3$, double arrow reversed,$\mathfrak{sp}(2n)$);$D_n$ ($n \geq 4$, forked at the end,$\mathfrak{so}(2n)$). *Exceptional types*:$E_6$,$E_7$,$E_8$ (exceptional with$E$-shaped diagrams),$F_4$ (with a double edge),$G_2$ (with a triple edge). There are exactly these types, and no others.

**49.3.3 Recovering the Root System from the Dynkin Diagram**
From the Dynkin diagram, one reads off the Cartan matrix $A$; from$A$, one recovers the simple roots and their angles/lengths; from the simple roots, one generates all positive roots by applying simple reflections; the full root system$\Phi = \Phi^+ \cup \Phi^-$. The number of positive roots is determined by the Weyl group:$|\Phi^+| = \ell(w_0)$ (the length of the longest element). For$A_n$:$|\Phi^+| = \binom{n+1}{2}$.

**49.3.4 The Exceptional Algebras: Dimensions and Root System Sizes**
The exceptional Lie algebras have the following data: $G_2$ (dimension 14, rank 2,$|\Phi^+| = 6$);$F_4$ (dimension 52, rank 4,$|\Phi^+| = 24$);$E_6$ (dimension 78, rank 6,$|\Phi^+| = 36$);$E_7$ (dimension 133, rank 7,$|\Phi^+| = 63$);$E_8$ (dimension 248, rank 8,$|\Phi^+| = 120$). The algebra$E_8$ is especially important in string theory, M-theory, and the moonshine conjectures.

---

**49.4 The Structure of Semisimple Lie Algebras via Roots**

**49.4.1 Chevalley Generators $e_\alpha$,$f_\alpha$,$h_\alpha$; the Chevalley Basis**
The *Chevalley basis* of a semisimple Lie algebra $\mathfrak{g}$ consists of:$h_{\alpha_i} \in \mathfrak{h}$ for each simple root;$e_\alpha \in \mathfrak{g}_\alpha$ for each positive root$\alpha$;$f_\alpha = e_{-\alpha} \in \mathfrak{g}_{-\alpha}$ for each negative root. Chevalley normalized these vectors so that all structure constants (the$[e_\alpha, e_\beta] = N_{\alpha,\beta} e_{\alpha+\beta}$ constants) are integers. The Chevalley basis makes the Lie algebra defined over$\mathbb{Z}$ (and hence over any field), enabling the study of Lie algebras and Chevalley groups in positive characteristic.

**49.4.2 Serre's Relations: Presenting $\mathfrak{g}$ via Generators and Relations**
*Serre's theorem:* The semisimple Lie algebra $\mathfrak{g}$ of Dynkin type$A$ is isomorphic to the Lie algebra generated by$\{e_i, f_i, h_i : 1 \leq i \leq \ell\}$ subject to the relations:$[h_i, h_j] = 0$;$[h_i, e_j] = A_{ij} e_j$;$[h_i, f_j] = -A_{ij} f_j$;$[e_i, f_j] = \delta_{ij} h_i$;$(\mathrm{ad}\, e_i)^{1-A_{ij}} e_j = 0$;$(\mathrm{ad}\, f_i)^{1-A_{ij}} f_j = 0$ for$i \neq j$. The last two (the *Serre relations*) encode the root string structure. This presentation shows that the Lie algebra is completely determined by the Cartan matrix.

**49.4.3 Reconstruction: Every Dynkin Diagram Gives a Unique Simple Lie Algebra**
By Serre's theorem, to each connected Dynkin diagram there corresponds exactly one simple Lie algebra over $\mathbb{C}$ (up to isomorphism). The classification of simple Lie algebras therefore reduces to: (a) classify all connected Dynkin diagrams; (b) for each diagram, the Lie algebra is presented by Serre's relations. The classification of Dynkin diagrams is a finite combinatorial problem (the possible diagrams are exactly$A_n, B_n, C_n, D_n, E_6, E_7, E_8, F_4, G_2$), making the classification of simple Lie algebras complete and explicit.

---

## Chapter 50 — Highest Weight Theory

**What it establishes:** The complete classification of finite-dimensional irreducible representations of a semisimple Lie algebra by their highest weight — an element of the dominant integral weight lattice — together with the Weyl character formula that computes the dimension and character of every irreducible.

---

**50.1 Representations and Weight Spaces**

**50.1.1 Representations of $\mathfrak{g}$;$\mathfrak{h}$-Semisimplicity; Weight Spaces$V_\lambda$**
A representation of a semisimple Lie algebra $\mathfrak{g}$ is a Lie algebra homomorphism$\rho: \mathfrak{g} \to \mathfrak{gl}(V)$. Since$\mathfrak{h}$ is abelian and acts by semisimple operators,$V$ decomposes into simultaneous$\mathfrak{h}$-eigenspaces:$V = \bigoplus_{\lambda \in \mathfrak{h}^*} V_\lambda$ where$V_\lambda = \{v \in V : H \cdot v = \lambda(H) v \text{ for all } H \in \mathfrak{h}\}$. The nonzero$\lambda$ for which$V_\lambda \neq 0$ are the *weights* of the representation.

**50.1.2 The Weights of a Representation; the Weight Lattice $P$**
The weights of any finite-dimensional representation lie in the *weight lattice* $P = \{\lambda \in \mathfrak{h}^* : \langle \lambda, \alpha^\vee \rangle \in \mathbb{Z} \text{ for all } \alpha \in \Phi\}$. The roots themselves are weights (of the adjoint representation). The *dominant weights* are$P^+ = \{\lambda \in P : \langle \lambda, \alpha_i^\vee \rangle \geq 0 \text{ for all simple } \alpha_i\}$. The *fundamental weights*$\omega_1, \ldots, \omega_\ell$ are the dual basis to the coroots:$\langle \omega_i, \alpha_j^\vee \rangle = \delta_{ij}$.

**50.1.3 $\mathfrak{sl}_2(\mathbb{C})$ and Its Irreducible Representations$V_n$**
For $\mathfrak{sl}_2(\mathbb{C}) = \mathrm{span}\{e, f, h\}$ with$[h,e] = 2e$,$[h,f] = -2f$,$[e,f] = h$: the finite-dimensional irreducible representations are$V_n$ for$n \geq 0$, of dimension$n+1$, with basis$\{v_0, v_1, \ldots, v_n\}$ where$h \cdot v_k = (n - 2k) v_k$ (eigenvalues$n, n-2, \ldots, -n$),$e \cdot v_k = (n - k + 1) v_{k-1}$,$f \cdot v_k = (k+1) v_{k+1}$. The highest weight is$n$ (the largest eigenvalue of$h$). This computation is the prototype for the general theory.

**50.1.4 Using $\mathfrak{sl}_2$-Triples to Control Weights in General$\mathfrak{g}$**
For each positive root $\alpha$, the triple$(e_\alpha, f_\alpha, h_\alpha)$ spans an$\mathfrak{sl}_2$-subalgebra of$\mathfrak{g}$. By restricting to this$\mathfrak{sl}_2$-subalgebra, the representation theory of$\mathfrak{sl}_2$ forces constraints on the weight spaces:$\alpha$-strings of weights are symmetric and have integer lengths. The$\mathfrak{sl}_2$-triple technique is the fundamental tool for proving that weights are in$P$ and that highest weight modules have the right structure.

---

**50.2 Highest Weights and Verma Modules**

**50.2.1 Highest Weight Vectors: $e_\alpha \cdot v_\lambda = 0$ for All Positive$\alpha$**
A vector $v \in V_\lambda$ is a *highest weight vector* of weight$\lambda$ if$e_\alpha \cdot v = 0$ for all positive roots$\alpha$ (equivalently, all raising operators annihilate$v$). In any finite-dimensional representation, highest weight vectors exist (by the finiteness of the weight set and the "raising" nature of$e_\alpha$). The weight$\lambda$ of a highest weight vector is the *highest weight* of the representation (or of the subrepresentation generated by$v$).

**50.2.2 The Universal Enveloping Algebra $\mathcal{U}(\mathfrak{g})$ and the PBW Theorem**
The *universal enveloping algebra* $\mathcal{U}(\mathfrak{g})$ is the associative algebra generated by$\mathfrak{g}$ subject to the relation$XY - YX = [X,Y]$ for all$X, Y \in \mathfrak{g}$. The *Poincaré–Birkhoff–Witt (PBW) theorem* gives a basis of$\mathcal{U}(\mathfrak{g})$: for any ordered basis$X_1, \ldots, X_n$ of$\mathfrak{g}$, the monomials$X_1^{a_1} \cdots X_n^{a_n}$ (with$a_i \geq 0$) form a basis of$\mathcal{U}(\mathfrak{g})$. Representations of$\mathfrak{g}$ as a Lie algebra are the same as$\mathcal{U}(\mathfrak{g})$-modules.

**50.2.3 Verma Modules $M(\lambda) = \mathcal{U}(\mathfrak{g}) \otimes_{\mathcal{U}(\mathfrak{b})} k_\lambda$**
For $\lambda \in \mathfrak{h}^*$, the *Verma module*$M(\lambda)$ is the$\mathfrak{g}$-module induced from the one-dimensional$\mathfrak{b}$-module$k_\lambda$ (where$\mathfrak{b} = \mathfrak{h} \oplus \mathfrak{n}^+$ is the Borel subalgebra and$\mathfrak{n}^+$ acts by 0):$M(\lambda) = \mathcal{U}(\mathfrak{g}) \otimes_{\mathcal{U}(\mathfrak{b})} k_\lambda$. The Verma module has a highest weight vector$v_\lambda$ (the generator) and is the "most general"$\mathfrak{g}$-module with highest weight$\lambda$: every highest weight module with highest weight$\lambda$ is a quotient of$M(\lambda)$. The PBW theorem shows that as a vector space,$M(\lambda)$ has basis$\{f_{\alpha_1}^{a_1} \cdots f_{\alpha_N}^{a_N} v_\lambda : a_i \geq 0\}$.

**50.2.4 $M(\lambda)$ Has a Unique Irreducible Quotient$L(\lambda)$**
The Verma module $M(\lambda)$ has a unique maximal proper submodule$J(\lambda)$ (the intersection of all maximal submodules — a well-defined ideal since the structure is so constrained). The quotient$L(\lambda) = M(\lambda)/J(\lambda)$ is the unique irreducible module with highest weight$\lambda$. Every irreducible$\mathfrak{g}$-module with a highest weight vector of weight$\lambda$ is isomorphic to$L(\lambda)$. This identifies the irreducible representations: they are in bijection with highest weights.

---

**50.3 The Classification Theorem**

**50.3.1 Dominant Integral Weights: $\langle \lambda, \alpha_i^\vee \rangle \in \mathbb{Z}_{\geq 0}$**
A weight $\lambda \in P$ is *dominant integral* if$\langle \lambda, \alpha_i^\vee \rangle \in \mathbb{Z}_{\geq 0}$ for all simple roots$\alpha_i$. The set of dominant integral weights is$P^+ = \{\lambda = \sum m_i \omega_i : m_i \in \mathbb{Z}_{\geq 0}\}$ (non-negative integer combinations of fundamental weights). These are the weights for which the$\mathfrak{sl}_2$-representation theory forces the Verma module to have a finite-dimensional quotient.

**50.3.2 Fundamental Weights $\omega_i$; the Positive Cone$P^+$**
The *fundamental weights* $\omega_1, \ldots, \omega_\ell$ are the elements of$P^+$ defined by$\langle \omega_i, \alpha_j^\vee \rangle = \delta_{ij}$. Every dominant integral weight is a non-negative integer combination$\lambda = m_1 \omega_1 + \cdots + m_\ell \omega_\ell$; the$m_i$ are the *Dynkin labels*. For$\mathfrak{sl}_2$: the fundamental weight$\omega_1$ is the weight of the standard representation, and$m\omega_1$ corresponds to the$(m+1)$-dimensional irreducible$V_m$.

**50.3.3 Bijection: $P^+ \leftrightarrow$ \{Finite-Dimensional Irreducibles\}**
*The highest weight theorem:* Every finite-dimensional irreducible representation of $\mathfrak{g}$ has a unique highest weight$\lambda \in P^+$, and every$\lambda \in P^+$ is the highest weight of a unique (up to isomorphism) finite-dimensional irreducible$L(\lambda)$. This gives a perfect bijection between dominant integral weights and finite-dimensional irreducible representations. The theorem reduces the classification of all finite-dimensional representations (by Weyl's complete reducibility, every representation is a direct sum of irreducibles) to the classification of dominant integral weights — which is just the combinatorics of$\mathbb{Z}_{\geq 0}^\ell$.

**50.3.4 The Finite-Dimensionality Theorem for Dominant Integral $\lambda$**
$L(\lambda)$ is finite-dimensional iff$\lambda \in P^+$. The proof that dominant integral$\lambda$ gives finite-dimensional$L(\lambda)$ is non-trivial: one constructs$L(\lambda)$ explicitly (via the PBW theorem and a careful analysis of the submodule$J(\lambda)$) and shows it has finite dimension$= \prod_{\alpha \in \Phi^+} \frac{\langle \lambda + \rho, \alpha^\vee \rangle}{\langle \rho, \alpha^\vee \rangle}$ (the Weyl dimension formula). The proof uses the$\mathfrak{sl}_2$-triple technique to control the weight spaces.

---

**50.4 The Weyl Character Formula**

**50.4.1 Formal Characters in the Group Ring $\mathbb{Z}[P]$**
The *formal character* of a finite-dimensional representation $V$ is$\mathrm{ch}(V) = \sum_{\lambda \in P} (\dim V_\lambda) e^\lambda \in \mathbb{Z}[P]$, where$\mathbb{Z}[P]$ is the group ring of the weight lattice$P$ with basis$\{e^\lambda : \lambda \in P\}$ and multiplication$e^\lambda \cdot e^\mu = e^{\lambda + \mu}$. The formal character encodes the full weight space data: knowing$\mathrm{ch}(V)$ is the same as knowing$\dim V_\lambda$ for all$\lambda$. For$V = L(\lambda)$, the formal character is a$W$-invariant element of$\mathbb{Z}[P]$.

**50.4.2 The Weyl Denominator and the Character Formula**
The *Weyl denominator* is $D = e^\rho \prod_{\alpha \in \Phi^+} (1 - e^{-\alpha}) = \sum_{w \in W} \mathrm{sgn}(w) e^{w(\rho)}$, where$\rho = \frac{1}{2}\sum_{\alpha > 0} \alpha$ is the Weyl vector (half the sum of positive roots). *The Weyl character formula:*
$$\mathrm{ch}(L(\lambda)) = \frac{\sum_{w \in W} \mathrm{sgn}(w) e^{w(\lambda + \rho)}}{\sum_{w \in W} \mathrm{sgn}(w) e^{w(\rho)}}$$
This formula expresses the character of the irreducible of highest weight $\lambda$ as a ratio of alternating sums over the Weyl group. The denominator is the same for all$\lambda$ (the Weyl denominator); the numerator is shifted by$\lambda$.

**50.4.3 The Weyl Dimension Formula**
Setting all $e^\lambda = 1$ in the Weyl character formula (i.e., evaluating at 0) gives the *Weyl dimension formula*:
$$\dim L(\lambda) = \prod_{\alpha \in \Phi^+} \frac{\langle \lambda + \rho, \alpha^\vee \rangle}{\langle \rho, \alpha^\vee \rangle}$$
For $\mathfrak{sl}_{n+1}$ with$\lambda = \sum m_i \omega_i$, this reduces to a product formula involving the$m_i$ and the positive roots. The dimension formula is a remarkable closed-form expression: it computes the dimension of any irreducible representation from the Dynkin labels alone.

**50.4.4 Examples: $\mathfrak{sl}_2$,$\mathfrak{sl}_3$,$G_2$**
*$\mathfrak{sl}_2$:*$L(m\omega) = V_m$ has$\dim = m+1$; the character formula gives$\mathrm{ch}(V_m) = e^{m} + e^{m-2} + \cdots + e^{-m}$ (sum of$e^{m-2k}$ for$k = 0, \ldots, m$). *$\mathfrak{sl}_3$:* Dominant weights are$\lambda = (m,n)\omega_1 + n\omega_2$;$\dim L(m,n) = \frac{1}{2}(m+1)(n+1)(m+n+2)$. *$G_2$:* The smallest nontrivial irreducible has dimension 7 (the standard representation); the next has dimension 14 (the adjoint).

**50.4.5 Freudenthal's Formula: Weight Multiplicities**
The Weyl character formula gives the formal character but does not directly give the individual weight multiplicities $\dim V_\lambda$. *Freudenthal's formula* is a recursion:
$$\dim V_\mu = \frac{2}{\langle \lambda + \rho, \lambda + \rho \rangle - \langle \mu + \rho, \mu + \rho \rangle} \sum_{\alpha > 0} \sum_{k \geq 1} \dim V_{\mu + k\alpha} \langle \mu + k\alpha, \alpha^\vee \rangle$$
that computes the multiplicity of each weight $\mu$ from the multiplicities of higher weights (in the ordering$\mu < \lambda$ means$\lambda - \mu$ is a positive linear combination of simple roots). Freudenthal's formula is the primary tool for computing weight multiplicities in practice.

---

**50.5 The Center of $\mathcal{U}(\mathfrak{g})$**

**50.5.1 The Casimir Element and Its Eigenvalue on $L(\lambda)$**
The *Casimir element* $C = \sum_i h_i^2 + \sum_{\alpha > 0} (e_\alpha f_\alpha + f_\alpha e_\alpha) \in \mathcal{U}(\mathfrak{g})$ (in a suitable normalization) is a central element of$\mathcal{U}(\mathfrak{g})$: it commutes with all of$\mathfrak{g}$. By Schur's lemma,$C$ acts by a scalar on any irreducible$L(\lambda)$. The eigenvalue is$\langle \lambda, \lambda + 2\rho \rangle$ (the value depends on the normalization of the Killing form). Casimir elements are the analogue of the "Laplacian" on a Lie group and are used to prove Weyl's complete reducibility theorem.

**50.5.2 The Harish-Chandra Isomorphism $Z(\mathcal{U}(\mathfrak{g})) \cong \mathbb{C}[\mathfrak{h}^*]^W$**
*The Harish-Chandra isomorphism:* The center $Z(\mathcal{U}(\mathfrak{g}))$ of the universal enveloping algebra is isomorphic (as a$\mathbb{C}$-algebra) to the ring of$W$-invariant polynomial functions on$\mathfrak{h}^*$. The isomorphism sends the Casimir element$C$ to the function$\lambda \mapsto \langle \lambda, \lambda + 2\rho \rangle$. The Harish-Chandra isomorphism parametrizes all central characters (the ways the center acts on irreducibles) and is a fundamental tool in the representation theory of Lie groups and the study of$\mathcal{D}$-modules on flag varieties.

---

*Next: [Part XI — Advanced Representation Theory](part-XI-advanced-representation.md)*

*Prerequisites satisfied: Part I (logic), Part II (linear algebra, inner products, eigentheory), Part III (group theory, including solvable/nilpotent groups), Part IV (ring theory, including the notion of algebra), Part V (modules, tensor products), Part VII (category theory, adjoint functors), Part IX (representation theory of finite groups — the finite case informs the Lie case).*
