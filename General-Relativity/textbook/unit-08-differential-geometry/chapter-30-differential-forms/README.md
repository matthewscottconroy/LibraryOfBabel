# Chapter 30: Differential Forms and Integration

---

## Chapter Introduction

Differential forms are the natural objects of integration on manifolds. A $k$-form on an $n$-manifold is an antisymmetric tensor of type $(0,k)$ — an object that can be integrated over $k$-dimensional submanifolds. The 1-form $f(x)dx$ integrates over curves. The 2-form $\omega = F_x\,dy\wedge dz + F_y\,dz\wedge dx + F_z\,dx\wedge dy$ integrates over surfaces (giving the flux of $\mathbf{F}$). The volume form $\omega = \sqrt{g}\,dx^1\wedge\cdots\wedge dx^n$ integrates over the manifold.

The power of differential forms comes from:

1. **The exterior derivative $d$**: A map $d: \Omega^k(M)\to\Omega^{k+1}(M)$ with $d^2 = 0$. It generalizes the gradient, curl, and divergence — and unifies Green's theorem, Stokes' theorem, and the divergence theorem into a single formula.

2. **Stokes' theorem**: $\int_M d\omega = \int_{\partial M}\omega$. This is the master theorem of integral calculus on manifolds, subsuming all the classical integral theorems.

3. **de Rham cohomology**: The quotient $H^k(M) = \ker(d:\Omega^k\to\Omega^{k+1})/\text{im}(d:\Omega^{k-1}\to\Omega^k)$ captures the topology of $M$ — it measures global obstructions to solving $d\omega = 0$.

In physics, $k$-forms appear everywhere: the electromagnetic field strength $F = dA$ is a 2-form; the action of a particle is $\int p_\mu dx^\mu$ (integral of a 1-form); the symplectic form in Hamiltonian mechanics is a closed 2-form.

---

## Differential $k$-Forms

A **differential $k$-form** $\omega\in\Omega^k(M)$ is a smooth totally antisymmetric $(0,k)$-tensor field. In coordinates:
$$\omega = \omega_{i_1\cdots i_k}(x)dx^{i_1}\wedge\cdots\wedge dx^{i_k}$$

with the convention that we sum over ordered multi-indices $i_1 < i_2 < \cdots < i_k$, or equivalently sum over all multi-indices and include the factor $1/k!$.

$\Omega^0(M) = C^\infty(M)$ (functions). $\Omega^n(M)$ (top forms) is a 1-dimensional module — volume forms.

**Wedge product**: $\Omega^k(M)\times\Omega^l(M)\to\Omega^{k+l}(M)$:
$$(\alpha\wedge\beta)_{i_1\cdots i_{k+l}} = \frac{(k+l)!}{k!l!}\alpha_{[i_1\cdots i_k}\beta_{i_{k+1}\cdots i_{k+l}]}$$

Anticommutativity: $\alpha\wedge\beta = (-1)^{kl}\beta\wedge\alpha$.

---

## The Exterior Derivative

The **exterior derivative** $d:\Omega^k(M)\to\Omega^{k+1}(M)$ is characterized by:
1. $df = \partial_i f\,dx^i$ for $f\in C^\infty(M)$
2. $d(\alpha\wedge\beta) = d\alpha\wedge\beta + (-1)^k\alpha\wedge d\beta$ (graded Leibniz rule)
3. $d^2 = 0$ (i.e., $d(d\alpha) = 0$ for all $\alpha$)
4. $d$ is $\mathbb{R}$-linear

In components: $(d\omega)_{i_0 i_1\cdots i_k} = (k+1)\partial_{[i_0}\omega_{i_1\cdots i_k]}$.

For a 1-form $\alpha = \alpha_i dx^i$:
$$(d\alpha)_{ij} = \partial_i\alpha_j - \partial_j\alpha_i$$

This is the antisymmetrized gradient — the curl in 3D.

**$d^2 = 0$**: For any smooth form $\omega$, $d(d\omega) = 0$. This is because $(d^2\omega)_{i_0\cdots i_{k+2}} \propto \partial_{[i_0}\partial_{i_1}\omega_{i_2\cdots i_{k+2}]} = 0$ by symmetry of second partials.

---

## Closed and Exact Forms

- $\omega$ is **closed** if $d\omega = 0$
- $\omega$ is **exact** if $\omega = d\alpha$ for some $\alpha$

Every exact form is closed ($d^2 = 0$). The converse is only locally true.

**Poincaré lemma**: On a star-shaped (contractible) open set $U\subset\mathbb{R}^n$, every closed $k$-form is exact for $k \geq 1$.

**Global obstruction**: On a general manifold, closed but not exact forms exist. Example: the 1-form $\theta = \frac{-y\,dx + x\,dy}{x^2+y^2}$ on $\mathbb{R}^2\setminus\{0\}$ satisfies $d\theta = 0$ but $\theta \neq df$ globally (its integral around the origin is $2\pi \neq 0$). This is the first de Rham cohomology class of $\mathbb{R}^2\setminus\{0\}$.

---

## Integration of Differential Forms

A $k$-form can be integrated over a $k$-dimensional oriented submanifold. Let $\phi: D\to M$ be a smooth embedding of an oriented domain $D\subset\mathbb{R}^k$:

$$\int_\phi\omega = \int_D\phi^*\omega$$

where $\phi^*\omega$ is the pullback of $\omega$ — an ordinary Euclidean integral. This is coordinate-independent: changing the orientation of the parametrization reverses the sign.

**Volume integration**: On an oriented $n$-manifold with volume form $\text{vol}_g = \sqrt{|\det g|}\,dx^1\wedge\cdots\wedge dx^n$, the integral of a function $f$ is:
$$\int_M f\,\text{vol}_g = \int_M f\sqrt{|\det g|}\,dx^1\cdots dx^n$$

---

## Stokes' Theorem

The central theorem of differential geometry:

**Stokes' Theorem**: Let $M$ be a compact oriented smooth $n$-manifold with boundary $\partial M$ (with induced orientation). For any $(n-1)$-form $\omega\in\Omega^{n-1}(M)$:
$$\int_M d\omega = \int_{\partial M}\omega$$

**Special cases**:
- **Fundamental Theorem of Calculus**: $M = [a,b]$, $\omega = f$: $\int_a^b f'(x)dx = f(b) - f(a)$.
- **Green's Theorem** (2D): $\int_D\left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right)dx\,dy = \oint_{\partial D}(P\,dx + Q\,dy)$.
- **Divergence Theorem** (3D): $\int_V\nabla\cdot\mathbf{F}\,dV = \oint_{\partial V}\mathbf{F}\cdot d\mathbf{S}$.
- **Classical Stokes' Theorem** (surface): $\int_S(\nabla\times\mathbf{F})\cdot d\mathbf{S} = \oint_{\partial S}\mathbf{F}\cdot d\mathbf{l}$.

All four are instances of the single abstract theorem $\int_M d\omega = \int_{\partial M}\omega$.

---

## Electromagnetism in the Language of Forms

The power of differential forms becomes clear in EM. The electromagnetic field strength is a 2-form:
$$F = F_{\mu\nu}dx^\mu\wedge dx^\nu = E_i(dx^i\wedge dt/c) + \sum_{ijk}\varepsilon_{ijk}B^k dx^i\wedge dx^j$$

In 4D spacetime, the first two Maxwell equations $dF = 0$ are:
$$dF = 0 \quad\Leftrightarrow\quad \nabla\cdot\mathbf{B} = 0 \text{ and } \partial_t\mathbf{B} + \nabla\times\mathbf{E} = 0$$

The gauge potential $A = A_\mu dx^\mu$ satisfies $F = dA$ (locally). The gauge transformation $A\to A + d\lambda$ leaves $F = dA$ unchanged. The remaining two Maxwell equations:
$$d\star F = \frac{4\pi}{c}\star J$$

where $\star$ is the Hodge star and $J = J^\mu\partial_\mu$ is the 4-current.

---

## Hodge Duality and the $\star$ Operator

On an oriented pseudo-Riemannian $n$-manifold, the **Hodge star** $\star: \Omega^k(M)\to\Omega^{n-k}(M)$ is defined by:
$$\alpha\wedge\star\beta = \langle\alpha,\beta\rangle\,\text{vol}$$

where $\langle\alpha,\beta\rangle = g^{i_1 j_1}\cdots g^{i_k j_k}\alpha_{i_1\cdots i_k}\beta_{j_1\cdots j_k}$ is the inner product on forms.

In components: $(\star\omega)_{i_{k+1}\cdots i_n} = \frac{1}{k!}\varepsilon^{j_1\cdots j_k}_{\ \ i_{k+1}\cdots i_n}\omega_{j_1\cdots j_k}$.

**Properties**: $\star\star\omega = (-1)^{k(n-k)}s\,\omega$ where $s = \text{sign}(\det g)$.

The **Laplacian on forms** (Hodge Laplacian): $\Delta = d\star d\star + \star d\star d$. For functions on Riemannian manifolds: $\Delta f = -\text{div}(\text{grad}\,f) = -g^{ij}\nabla_i\nabla_j f$.

---

## De Rham Cohomology

The **de Rham cohomology** of $M$ is:
$$H^k_{\rm dR}(M;\mathbb{R}) = \frac{\ker(d:\Omega^k(M)\to\Omega^{k+1}(M))}{\text{im}(d:\Omega^{k-1}(M)\to\Omega^k(M))}$$

**de Rham theorem**: $H^k_{\rm dR}(M;\mathbb{R}) \cong H^k(M;\mathbb{R})$ (singular cohomology with real coefficients). The de Rham groups are topological invariants.

**Betti numbers**: $b_k = \dim H^k_{\rm dR}(M)$ count the de Rham cohomology dimensions. For $S^n$: $b_0 = b_n = 1$, all others zero. For $T^2$: $b_0 = 1$, $b_1 = 2$, $b_2 = 1$.

**Physical significance**: Electromagnetism lives in $H^2(M)$. A magnetic monopole would require a 2-cycle (a closed 2-surface) in spacetime with nonzero $\int_\Sigma F$ — possible only if $H^2(M) \neq 0$. The absence of magnetic monopoles in the standard model is (in part) a statement about the topology of the gauge bundle.

---

## Exercises

**30.1.** *The exterior derivative in 3D.*

In $\mathbb{R}^3$ with coordinates $(x,y,z)$:

(a) Compute $d(xy^2 dz)$.

(b) Compute $d(E_x\,dy\wedge dz + E_y\,dz\wedge dx + E_z\,dx\wedge dy)$ and identify the result with $(\nabla\cdot\mathbf{E})dx\wedge dy\wedge dz$.

(c) Show $d^2(fdx) = 0$ by explicit computation.

---

**30.2.** *Stokes' theorem for a surface.*

Apply Stokes' theorem $\int_S d\alpha = \int_{\partial S}\alpha$ to:

(a) $\alpha = y\,dx + x^2\,dy$, $S$ = the unit square $[0,1]\times[0,1]$ in the $xy$-plane.

(b) Verify by computing both sides explicitly.

(c) The integral $\oint_C \frac{-y\,dx + x\,dy}{x^2+y^2}$ for $C$ = unit circle. Compute directly. Why does Stokes' theorem not give zero despite $d\theta = 0$?

---

**30.3.** *EM 2-form.*

In Minkowski spacetime with $ds^2 = -c^2dt^2 + dx^2 + dy^2 + dz^2$, the EM 2-form is:
$$F = E_x\,c\,dt\wedge dx + E_y\,c\,dt\wedge dy + E_z\,c\,dt\wedge dz + B_z\,dx\wedge dy - B_y\,dx\wedge dz + B_x\,dy\wedge dz$$

(a) Compute $dF$ and show $dF = 0$ gives the homogeneous Maxwell equations $\nabla\cdot\mathbf{B} = 0$ and $\partial_t\mathbf{B} = -c\nabla\times\mathbf{E}$.

(b) Write $F = dA$ where $A = A_\mu dx^\mu$ and identify $E_i$ and $B_i$ in terms of $A_0$ and $A_i$.

(c) Under a gauge transformation $A\to A + d\lambda$: show $F$ is unchanged.

---

**Thought Experiment T30.1.** *Why $d^2 = 0$?*

The identity $d^2 = 0$ is more than a calculation — it encodes a deep topological fact. In the de Rham complex $0\to\Omega^0\to\Omega^1\to\cdots\to\Omega^n\to 0$, the condition $d^2 = 0$ makes each $d$ a boundary operator, and the cohomology measures the "holes" in the manifold.

In electromagnetism: $d^2 = 0$ and $F = dA$ imply $dF = 0$ automatically. But what if spacetime had a topological defect (a "hole")? Then $dF = 0$ doesn't imply $F = dA$ globally — there could be a magnetic monopole contribution with $\int_\Sigma F = g_M/\varepsilon_0\neq 0$. This is the Dirac monopole condition.

Is the absence of magnetic monopoles in nature a statement about the topology of spacetime, or about the structure of the gauge theory? Could spacetime have nontrivial topology on macroscopic scales?
