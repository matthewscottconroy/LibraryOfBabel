# Chapter 28: Tangent and Cotangent Spaces

---

## Chapter Introduction

The tangent space $T_p M$ at a point $p$ of a manifold captures the infinitesimal structure — the directions you can move from $p$. Its dual space, the **cotangent space** $T^*_p M$, captures the "one-forms" — the linear functionals on directions.

This duality is fundamental. In classical mechanics, the state space of a system with $n$ degrees of freedom is a $2n$-dimensional phase space: $n$ positions (on the configuration manifold $Q$) and $n$ momenta (in the cotangent bundle $T^*Q$). In GR, the metric $g_{\mu\nu}$ is a bilinear form on tangent vectors — it maps a pair of vectors to a number, and its inverse maps a pair of covectors to a number. The distinction between "upstairs" and "downstairs" indices is precisely the distinction between vectors and covectors.

This chapter defines the cotangent space, the natural pairing between vectors and covectors, differential forms, and the operations of pullback and pushforward — the fundamental maps between tangent and cotangent spaces.

---

## The Cotangent Space

**Definition**: The **cotangent space** $T^*_p M$ is the dual vector space to $T_p M$: the space of linear maps $\alpha: T_p M\to\mathbb{R}$.

Elements of $T^*_p M$ are called **covectors** (or **1-forms** at $p$, or **covariant vectors**).

If $\{e_i\} = \{\partial/\partial x^i\big|_p\}$ is a basis for $T_p M$, the **dual basis** $\{e^i\} = \{dx^i\big|_p\}$ for $T^*_p M$ is defined by:
$$e^i(e_j) = \delta^i_j, \quad \text{i.e.,}\quad dx^i\left(\frac{\partial}{\partial x^j}\right) = \delta^i_j$$

A covector $\alpha\in T^*_p M$ has components $\alpha_i = \alpha(\partial/\partial x^i)$ in this basis:
$$\alpha = \alpha_i dx^i$$

The natural pairing between a vector $v = v^i\partial/\partial x^i$ and a covector $\alpha = \alpha_i dx^i$:
$$\langle\alpha, v\rangle = \alpha(v) = \alpha_i v^i$$

---

## Differential of a Function

The most natural example of a covector is the **differential** $df$ of a smooth function $f: M\to\mathbb{R}$.

At each point $p$, $df_p: T_p M\to\mathbb{R}$ is defined by:
$$df_p(v) = v(f)$$

(applying the derivation $v$ to $f$). In local coordinates:
$$df = \frac{\partial f}{\partial x^i}dx^i$$

This is exactly the gradient of $f$, expressed as a covector. Note: $df$ is a covector, not a vector. In coordinates:
$$df\left(\frac{\partial}{\partial x^j}\right) = \frac{\partial f}{\partial x^j}$$

This is why $dx^i$ is the differential of the coordinate function $x^i$: $dx^i(\partial/\partial x^j) = \partial x^i/\partial x^j = \delta^i_j$. ✓

---

## Change of Basis: Contravariant vs. Covariant

Under a change of coordinates $x^i\to\tilde{x}^j(x)$, the transformation laws for vectors and covectors differ:

**Vectors** (contravariant, upper indices):
$$\tilde{v}^j = \frac{\partial\tilde{x}^j}{\partial x^i}v^i$$

**Covectors** (covariant, lower indices):
$$\tilde{\alpha}_j = \frac{\partial x^i}{\partial\tilde{x}^j}\alpha_i$$

The transformation matrices are inverses of each other. The pairing $\alpha_i v^i$ is invariant (a scalar): $\tilde{\alpha}_j\tilde{v}^j = \alpha_i v^i$. This is why the notation distinguishes upper from lower indices.

**Historical context**: The names "contravariant" (transforms inversely to the basis) and "covariant" (transforms the same as the basis) come from classical tensor calculus. In modern notation: upper indices for vectors, lower indices for covectors. The metric $g_{\mu\nu}$ provides an isomorphism between $T_p M$ and $T^*_p M$ (raising and lowering indices).

---

## Tensor Products and Tensors

A $(r, s)$-**tensor** at $p$ is a multilinear map:
$$T: \underbrace{T^*_p M\times\cdots\times T^*_p M}_{r} \times \underbrace{T_p M\times\cdots\times T_p M}_{s} \to\mathbb{R}$$

In components: $T^{i_1\cdots i_r}_{\ \ \ j_1\cdots j_s}$.

- $(1,0)$-tensor: vector $v^i$
- $(0,1)$-tensor: covector $\alpha_i$  
- $(0,2)$-tensor: bilinear form, e.g., the metric $g_{ij}$
- $(1,1)$-tensor: linear map $T_p M\to T_p M$, e.g., $\delta^i_j$
- $(2,0)$-tensor: contravariant 2-tensor, e.g., $g^{ij}$ (inverse metric)

The **tensor product** of a vector $v$ and a covector $\alpha$ is the $(1,1)$-tensor $(v\otimes\alpha)(w, \beta) = \langle\beta, v\rangle\langle\alpha, w\rangle$ (for vector $w$ and covector $\beta$). In components: $(v\otimes\alpha)^i_{\ j} = v^i\alpha_j$.

---

## Pushforward and Pullback

If $F: M\to N$ is a smooth map, it induces:

**Pushforward** (differential) $F_*: T_p M\to T_{F(p)}N$:
$$(F_* v)(f) = v(f\circ F) \quad\text{for }f\in C^\infty(N)$$

In coordinates: $(F_* v)^i = \frac{\partial F^i}{\partial x^j}v^j$ (the Jacobian matrix times $v$).

**Pullback** $F^*: T^*_{F(p)}N\to T^*_p M$:
$$(F^*\alpha)(v) = \alpha(F_* v) \quad\text{for }v\in T_p M$$

In coordinates: $(F^*\alpha)_i = \frac{\partial F^j}{\partial x^i}\alpha_j$ (transpose Jacobian times $\alpha$).

Key properties:
- The pushforward of a vector goes **forward** along $F$; the pullback of a form goes **backward**
- $(F\circ G)_* = F_*\circ G_*$, $(F\circ G)^* = G^*\circ F^*$
- Only the pullback extends naturally to differential forms; vector fields don't generally push forward unless $F$ is a diffeomorphism

---

## Differential Forms and the Exterior Algebra

The **$k$-th exterior power** $\Lambda^k T^*_p M$ consists of **$k$-forms** at $p$ — totally antisymmetric $(0,k)$-tensors.

The **exterior product** (wedge product) of a $k$-form $\alpha$ and an $l$-form $\beta$:
$$(\alpha\wedge\beta)(v_1,\ldots,v_{k+l}) = \frac{1}{k!l!}\sum_\sigma\text{sgn}(\sigma)\alpha(v_{\sigma(1)},\ldots,v_{\sigma(k)})\beta(v_{\sigma(k+1)},\ldots,v_{\sigma(k+l)})$$

Properties:
- Associative: $(\alpha\wedge\beta)\wedge\gamma = \alpha\wedge(\beta\wedge\gamma)$
- Anticommutative: $\alpha\wedge\beta = (-1)^{kl}\beta\wedge\alpha$ for $k$-form $\alpha$, $l$-form $\beta$

A basis for $\Lambda^k T^*_p M$: $\{dx^{i_1}\wedge\cdots\wedge dx^{i_k}: i_1 < \cdots < i_k\}$. Dimension: $\binom{n}{k}$.

**Volume form**: A nonzero $n$-form $\omega\in\Lambda^n T^*_p M$ is a volume element. On an oriented Riemannian manifold with metric $g$: $\omega = \sqrt{\det g}\,dx^1\wedge\cdots\wedge dx^n$. On a Lorentzian manifold: $\omega = \sqrt{-\det g}\,dx^0\wedge\cdots\wedge dx^3$.

---

## The Cotangent Bundle and 1-Forms

The **cotangent bundle** $T^*M = \coprod_p T^*_p M$ with the natural smooth structure is a $2n$-dimensional manifold.

A **1-form** (differential form of degree 1) is a smooth section $\alpha: M\to T^*M$, assigning to each $p$ a covector $\alpha_p\in T^*_p M$. In coordinates: $\alpha = \alpha_i(x)dx^i$.

The space $\Omega^1(M)$ of 1-forms is the dual of the space $\mathfrak{X}(M)$ of vector fields. The pairing $\langle\alpha, X\rangle(p) = \alpha_p(X_p)$ is a function on $M$.

**Exact vs. closed 1-forms**: $\alpha$ is **exact** if $\alpha = df$ for some $f\in C^\infty(M)$. $\alpha$ is **closed** if $d\alpha = 0$ (the exterior derivative, introduced in Chapter 30). Every exact form is closed; the converse holds locally (Poincaré lemma) but may fail globally — this is the content of de Rham cohomology.

---

## Exercises

**28.1.** *Coordinate transformation of covectors.*

(a) In 2D with Cartesian coordinates $(x, y)$, the covector $\alpha = dx + 2dy$. Transform to polar coordinates $(r, \phi)$ with $x = r\cos\phi$, $y = r\sin\phi$. Express $\alpha$ in the polar basis $\{dr, d\phi\}$.

(b) Compute $\partial/\partial r$ and $\partial/\partial\phi$ in terms of $\partial/\partial x$ and $\partial/\partial y$.

(c) Verify that $\langle\alpha, v\rangle$ is the same in both coordinate systems, where $v = \partial/\partial x$.

---

**28.2.** *Differential of a function.*

(a) Compute the differential $df$ for $f = x^2 + y^2$ on $\mathbb{R}^2$.

(b) In polar coordinates, express $d(r^2)$ using the chain rule for differentials.

(c) The covector $df$ at $(1, 0)$ in Cartesian coordinates: compute $df_{(1,0)}(v)$ for $v = \partial/\partial x + \partial/\partial y$. Interpret this geometrically.

---

**28.3.** *Tensors on $S^2$.*

The metric on $S^2$ (unit sphere) in spherical coordinates $(\theta, \phi)$ is $g = d\theta^2 + \sin^2\theta\,d\phi^2$.

(a) Write $g$ as a $(0,2)$-tensor and give its matrix components $g_{ij}$.

(b) Compute the inverse metric $g^{ij}$ and write $g^{-1} = g^{ij}\partial_i\otimes\partial_j$.

(c) The gradient of a function $f$ on $S^2$: $(\nabla f)^i = g^{ij}\partial_j f$. Compute $\nabla(\cos\theta)$.

---

**Thought Experiment T28.1.** *Why cotangent bundles are phase spaces.*

In classical mechanics, the configuration space is an $n$-dimensional manifold $Q$ (position variables). The phase space is the cotangent bundle $T^*Q$ (positions and momenta). Hamilton's equations describe the flow of a system on $T^*Q$.

Why momenta are naturally covectors: if $L(q, \dot{q})$ is a Lagrangian, the momentum $p_i = \partial L/\partial\dot{q}^i$ transforms as a covector (covariant) under coordinate transformations, while $\dot{q}^i$ is contravariant (a tangent vector). The Hamiltonian $H(q, p)$ is defined on the cotangent bundle.

What physical meaning does the distinction between tangent (velocity) and cotangent (momentum) have? In what physical situations does this distinction matter — e.g., when is $p_i \neq m\dot{q}^i$?
