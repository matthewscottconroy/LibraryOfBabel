# 3.6 Differential Forms and de Rham Cohomology

Differential forms are the right language for integration on manifolds. They generalize the functions you integrate in calculus to objects that can be integrated over curves, surfaces, and higher-dimensional submanifolds — in a coordinate-independent way. The key result, Stokes' theorem, unifies all the classical integration theorems of vector calculus.

## 3.6.1 Differential Forms

**Definition 3.6.1.** A *differential $k$-form* on a smooth manifold $M$ is a smooth section of $\bigwedge^k T^*M$ (the $k$-th exterior power of the cotangent bundle). In local coordinates $(x_1, \ldots, x_n)$:
$$\omega = \sum_{i_1 < \cdots < i_k} f_{i_1 \cdots i_k}\,dx_{i_1} \wedge \cdots \wedge dx_{i_k}.$$

The space of $k$-forms is $\Omega^k(M)$.

The wedge product $\wedge$ is antisymmetric: $dx_i \wedge dx_j = -dx_j \wedge dx_i$. This antisymmetry is what makes differential forms sensitive to orientation — integrating a form over a surface depends on which way the surface is oriented.

- A 0-form is just a smooth function.
- A 1-form is something you can integrate along curves: $\int_\gamma \omega$ for $\gamma: [0,1] \to M$.
- A 2-form is something you can integrate over surfaces.
- An $n$-form on an $n$-dimensional manifold is something you can integrate over the whole manifold.

The exterior derivative connects forms of different degrees:

**Definition 3.6.2.** The *exterior derivative* $d: \Omega^k(M) \to \Omega^{k+1}(M)$ is the unique operator satisfying:
1. $d(f) = \sum_i \frac{\partial f}{\partial x_i} dx_i$ for functions ($0$-forms)
2. $d \circ d = 0$
3. $d(\omega \wedge \eta) = (d\omega) \wedge \eta + (-1)^k \omega \wedge (d\eta)$ for $\omega \in \Omega^k$

On $\mathbb{R}^3$: $d$ applied to 0-forms gives the gradient; applied to 1-forms gives the curl; applied to 2-forms gives the divergence. The condition $d \circ d = 0$ encodes the classical identities $\text{curl}(\text{grad}) = 0$ and $\text{div}(\text{curl}) = 0$, now in a unified framework.

## 3.6.2 Stokes' Theorem and de Rham Cohomology

**Theorem 3.6.3 (Stokes' Theorem).** Let $M$ be a smooth oriented compact $n$-manifold with boundary $\partial M$. For any $(n-1)$-form $\omega$:
$$\int_M d\omega = \int_{\partial M} \omega.$$

This single theorem simultaneously generalizes:
- The Fundamental Theorem of Calculus: $\int_a^b f'(x)\,dx = f(b) - f(a)$
- Green's Theorem in the plane
- Gauss's Divergence Theorem in $\mathbb{R}^3$
- The classical Stokes' Theorem for surfaces in $\mathbb{R}^3$

The key insight: the boundary of a boundary is empty ($\partial(\partial M) = \emptyset$), just as $d \circ d = 0$. Stokes' theorem is the "integral version" of this algebraic fact.

**Definition 3.6.4.** A form $\omega$ is *closed* if $d\omega = 0$; *exact* if $\omega = d\eta$ for some $\eta$. Since $d^2 = 0$, exact implies closed. The *de Rham cohomology* is
$$H^k_{\text{dR}}(M) = \frac{\ker(d: \Omega^k \to \Omega^{k+1})}{\text{im}(d: \Omega^{k-1} \to \Omega^k)}.$$

De Rham cohomology measures the "gap" between closed and exact forms — it detects topological holes. A closed form that's not exact witnesses a hole in the manifold. The cohomology group $H^1_{\text{dR}}(M) \cong \mathbb{Z}^k$ if $M$ has $k$ "independent loops."

**Theorem 3.6.5 (de Rham's Theorem).** $H^k_{\text{dR}}(M) \cong H^k(M; \mathbb{R})$ (singular cohomology with $\mathbb{R}$ coefficients).

De Rham's theorem is a profound result: it says the differential-geometric computation (using smooth forms and the exterior derivative) gives the same answer as the topological computation (using singular chains). Smooth structure and topological structure, different as they seem, encode the same cohomological information.

**Application in Dynamics.** The *Liouville measure* of a Hamiltonian system is a top-degree form $\omega^n$ (where $\omega$ is the symplectic form). Stokes' theorem implies $d(\omega^n) = 0$ — the Liouville measure is preserved by the Hamiltonian flow. The de Rham cohomology measures *topological obstructions* to finding global first integrals: if $dH = 0$ (energy is conserved), the corresponding cohomology class may be nontrivial, which constrains the dynamics. More concretely, Exercise 3.6 shows a closed but not exact form on $\mathbb{R}^2 \setminus \{0\}$; this is what's responsible for the impossibility of defining a global angle function on the punctured plane.
