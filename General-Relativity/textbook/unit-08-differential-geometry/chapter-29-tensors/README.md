# Chapter 29: Tensor Fields and Tensor Algebra

---

## Chapter Introduction

A tensor field assigns a tensor to every point of a manifold, consistently and smoothly. The metric tensor $g_{\mu\nu}$ of general relativity is a tensor field — at each event in spacetime, it assigns a bilinear form on tangent vectors. The Riemann curvature tensor $R^\rho_{\ \sigma\mu\nu}$ is a (1,3)-tensor field. The stress-energy tensor $T^{\mu\nu}$ is a (2,0)-tensor field.

The reason physics uses tensors is their coordinate-independence. An equation written in tensor form — $G_{\mu\nu} = 8\pi G T_{\mu\nu}/c^4$ — holds in every coordinate system if it holds in one. This is the mathematical expression of the principle of general covariance: the laws of physics take the same form in all coordinate systems.

This chapter develops tensor algebra systematically: the operations of contraction, symmetrization, antisymmetrization, and the index notation that makes these operations practical. It also introduces the metric tensor and how it acts as an isomorphism between tangent and cotangent spaces (raising and lowering indices).

---

## Tensor Fields

A **smooth $(r,s)$-tensor field** on a manifold $M$ is a smooth assignment of an $(r,s)$-tensor to each point — or equivalently, a $C^\infty(M)$-multilinear map:
$$T: \underbrace{\Omega^1(M)\times\cdots\times\Omega^1(M)}_r\times\underbrace{\mathfrak{X}(M)\times\cdots\times\mathfrak{X}(M)}_s \to C^\infty(M)$$

In local coordinates, $T$ has components $T^{i_1\cdots i_r}_{\ \ \ j_1\cdots j_s}(x)$ — $n^{r+s}$ smooth functions.

The tensor fields of type $(r,s)$ on $M$ form a module over $C^\infty(M)$, denoted $\mathcal{T}^r_s(M)$.

---

## Index Notation (Einstein Summation Convention)

Repeated indices, one upstairs and one downstairs, are implicitly summed:
$$v^\mu\alpha_\mu = \sum_{\mu=0}^{3}v^\mu\alpha_\mu, \quad T^\mu_{\ \nu}S^\nu_{\ \rho} = \sum_\nu T^\mu_{\ \nu}S^\nu_{\ \rho}$$

Rules:
- A free index appears exactly once (either up or down) in each term — it labels a family of equations
- A repeated (dummy) index appears exactly twice in a term, once up and once down — it is summed over
- Never repeat an index more than twice in a term

**Abstract index notation** (Penrose): Indices $a, b, c, \ldots$ label tensor type, not coordinates. $v^a$ is a vector, $\alpha_a$ is a covector, $g_{ab}$ is the metric. This notation is coordinate-independent and avoids the confusion of "component indices."

---

## Tensor Operations

**Tensor product**: $(A\otimes B)^{ij\cdots}_{kl\cdots} = A^{i\cdots}_{k\cdots}B^{j\cdots}_{l\cdots}$

**Contraction**: Set an upper and lower index equal and sum. Reduces the tensor type by $(1,1)$:
$$T^\mu_{\ \mu\nu} = \sum_\mu T^\mu_{\ \mu\nu}$$
The trace of a $(1,1)$-tensor: $T^\mu_{\ \mu}$. The Ricci scalar: $R = g^{\mu\nu}R_{\mu\nu}$.

**Symmetrization**: 
$$T_{(ij)} = \frac{1}{2}(T_{ij} + T_{ji}), \quad T_{(i_1\cdots i_k)} = \frac{1}{k!}\sum_\sigma T_{i_{\sigma(1)}\cdots i_{\sigma(k)}}$$

**Antisymmetrization**:
$$T_{[ij]} = \frac{1}{2}(T_{ij} - T_{ji}), \quad T_{[i_1\cdots i_k]} = \frac{1}{k!}\sum_\sigma\text{sgn}(\sigma)T_{i_{\sigma(1)}\cdots i_{\sigma(k)}}$$

A symmetric tensor satisfies $T_{ij} = T_{(ij)} = T_{ji}$ (so $T_{[ij]} = 0$). An antisymmetric tensor satisfies $T_{ij} = T_{[ij]} = -T_{ji}$.

---

## The Metric Tensor

A **Riemannian metric** on $M$ is a smooth $(0,2)$-tensor field $g$ that is:
- Symmetric: $g_{ij} = g_{ji}$
- Positive definite: $g(v,v) > 0$ for all nonzero $v\in T_p M$

A **pseudo-Riemannian metric** (Lorentzian if the signature is $(-,+,+,+)$) drops positive-definiteness, requiring only non-degeneracy.

**The metric isomorphism**: $g$ gives a canonical isomorphism $g^\flat: T_p M\to T^*_p M$ (lowering indices):
$$(g^\flat v)_i = g_{ij}v^j$$

with inverse $g^\sharp: T^*_p M\to T_p M$ (raising indices):
$$(\alpha^\sharp)^i = g^{ij}\alpha_j$$

where $g^{ij}$ is the inverse metric ($g^{ik}g_{kj} = \delta^i_j$).

This is what "raising and lowering indices" means: $v_i = g_{ij}v^j$ (lower), $\alpha^i = g^{ij}\alpha_j$ (raise).

**The inner product**: $g(v,w) = g_{ij}v^iw^j = v^i w_i = v_i w^i$.

---

## Important Tensor Fields in GR

**Metric tensor** $g_{\mu\nu}$ (0,2): Defines distances, angles, causal structure.

**Inverse metric** $g^{\mu\nu}$ (2,0): $g^{\mu\rho}g_{\rho\nu} = \delta^\mu_\nu$.

**Kronecker delta** $\delta^\mu_\nu$ (1,1): Identity tensor. $\delta^\mu_\nu T^\nu = T^\mu$.

**Levi-Civita symbol** $\varepsilon_{\mu\nu\rho\sigma}$: Totally antisymmetric, $\varepsilon_{0123} = +1$. Not a tensor — it's a tensor density.

**Levi-Civita tensor** $\epsilon_{\mu\nu\rho\sigma} = \sqrt{-g}\,\varepsilon_{\mu\nu\rho\sigma}$: A true $(0,4)$-tensor.

**Riemann tensor** $R^\rho_{\ \sigma\mu\nu}$ (1,3): Encodes curvature.

**Ricci tensor** $R_{\mu\nu} = R^\rho_{\ \mu\rho\nu}$ (0,2): Trace of Riemann.

**Einstein tensor** $G_{\mu\nu} = R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R$ (0,2): Left side of Einstein's equations.

---

## Symmetries of the Riemann Tensor

The Riemann tensor $R_{\mu\nu\rho\sigma} = g_{\mu\lambda}R^\lambda_{\ \nu\rho\sigma}$ (fully lowered) has the symmetries:
$$R_{\mu\nu\rho\sigma} = -R_{\nu\mu\rho\sigma} = -R_{\mu\nu\sigma\rho} = R_{\rho\sigma\mu\nu}$$

These reduce 256 independent components to 20 in 4D.

The **first Bianchi identity** (algebraic):
$$R_{\mu[\nu\rho\sigma]} = 0 \quad\Leftrightarrow\quad R_{\mu\nu\rho\sigma} + R_{\mu\rho\sigma\nu} + R_{\mu\sigma\nu\rho} = 0$$

This further reduces 20 to 20 (it's not an independent reduction).

The **Weyl tensor** $C_{\mu\nu\rho\sigma}$ is the trace-free part of Riemann:
$$R_{\mu\nu\rho\sigma} = C_{\mu\nu\rho\sigma} + \frac{1}{n-2}(g_{\mu\rho}R_{\nu\sigma} - g_{\mu\sigma}R_{\nu\rho} - g_{\nu\rho}R_{\mu\sigma} + g_{\nu\sigma}R_{\mu\rho}) - \frac{R}{(n-1)(n-2)}(g_{\mu\rho}g_{\nu\sigma} - g_{\mu\sigma}g_{\nu\rho})$$

In 4D: Riemann has 20 components = 10 Weyl + 10 Ricci. Weyl is the "free gravitational field" — it can be nonzero in vacuum ($R_{\mu\nu} = 0$). Ricci is "sourced" by matter.

---

## Exercises

**29.1.** *Tensor components and transformation.*

(a) A 2D manifold with coordinates $(x, y)$ has metric $g = dx\otimes dx + dy\otimes dy$ (Euclidean). In polar coordinates $(r, \phi)$, compute $g_{rr}$, $g_{r\phi}$, $g_{\phi\phi}$ using $dx = dr\cos\phi - r\sin\phi\,d\phi$, $dy = dr\sin\phi + r\cos\phi\,d\phi$.

(b) Compute the inverse metric $g^{rr}$, $g^{r\phi}$, $g^{\phi\phi}$.

(c) Lower the index of $v^r = 1$, $v^\phi = 0$ to get $v_r$, $v_\phi$.

---

**29.2.** *Contraction and the Ricci tensor.*

For a manifold with Riemann tensor $R^\rho_{\ \sigma\mu\nu}$:

(a) Show that the Ricci tensor $R_{\mu\nu} = R^\rho_{\ \mu\rho\nu}$ (contraction of first and third indices) is symmetric: $R_{\mu\nu} = R_{\nu\mu}$.

(b) Show that contracting the second Bianchi identity $\nabla_{[\mu}R_{\rho\sigma]\nu\lambda} = 0$ twice with $g^{\mu\nu}$ gives $\nabla^\mu G_{\mu\nu} = 0$ where $G_{\mu\nu} = R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R$.

---

**29.3.** *Symmetries of Riemann.*

For a 4D manifold, the Riemann tensor has the symmetries $R_{\mu\nu\rho\sigma} = -R_{\nu\mu\rho\sigma} = -R_{\mu\nu\sigma\rho} = R_{\rho\sigma\mu\nu}$.

(a) Count the number of independent components: start with $4^4 = 256$, apply antisymmetry in $(\mu\nu)$: $6\times 4^2 = 96$, apply antisymmetry in $(\rho\sigma)$: $6\times 6 = 36$, apply pair symmetry: $36\to 21$, apply first Bianchi: subtract 1 more. Verify you get 20.

(b) In 3D (spatial slice): how many independent components does Riemann have? Does this equal the Ricci tensor components? What does this mean?

---

**Thought Experiment T29.1.** *Coordinate independence and physical meaning.*

The Einstein equations $G_{\mu\nu} = 8\pi G T_{\mu\nu}/c^4$ are tensor equations. They hold in every coordinate system. But actual computations (computing Christoffel symbols, solving geodesic equations) depend on the coordinate choice.

How do we extract physical, coordinate-independent information from a calculation done in specific coordinates? Give an example from GR (e.g., the Schwarzschild metric in Schwarzschild coordinates vs. isotropic coordinates) where two different-looking metrics describe the same spacetime. What is the invariant criterion for two metrics being equivalent?
