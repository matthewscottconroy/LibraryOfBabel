# 28.2 Functors Between Dynamics and Algebra

Two dynamical systems can be "the same" in different senses. Measure-theoretically isomorphic systems share all ergodic-theoretic properties. Topologically conjugate systems share all topological dynamical properties. But how do we *detect* isomorphism? We need invariants — and the most powerful invariants come from functors, systematic translations that carry dynamical information into algebraic form.

## 28.2.1 The Koopman Functor

The oldest and most powerful such functor is the Koopman construction, which translates a dynamical system into a unitary operator on a Hilbert space. This translation was used implicitly since von Neumann's work in the 1930s, but seeing it as a functor clarifies exactly what information it carries and what it loses.

**Definition 28.2.1.** The *Koopman operator* $U_f: L^2(X, \mu) \to L^2(X, \mu)$, $U_f\varphi = \varphi \circ f$, is the image of the morphism $f$ under the *Koopman functor*:
$$K: \mathbf{Meas.Dyn}^{op} \to \mathbf{Hilb}$$
sending $(X, \mu, f) \mapsto (L^2(X,\mu), U_f)$.

Note the $op$: the Koopman functor is *contravariant*. A factor map $\phi: (X, f) \to (Y, g)$ goes forward in $\mathbf{Meas.Dyn}$ but backward in $\mathbf{Hilb}$, pulling functions back from $Y$ to $X$ via $\psi \mapsto \psi \circ \phi$.

**Theorem 28.2.2 (Von Neumann, categorical formulation).** The Koopman functor $K$ is faithful (injective on morphisms) when restricted to ergodic systems. Two ergodic systems are isomorphic iff their Koopman operators are unitarily equivalent *and* the isomorphism preserves the algebra structure.

The algebra structure here is key: we need the isomorphism to preserve not just the linear operator but the multiplicative structure of $L^2$ (the pointwise product). The spectral theory alone — the eigenvalues and eigenfunctions of $U_f$ — does not fully determine the system. But spectrum plus algebra does.

**Definition 28.2.3.** The *spectral theory* of a dynamical system is the image under $K$: the unitary operator $U_f$ on $L^2$. The eigenvalues of $U_f$ are the *eigenvalues of the system* — they are elements of $S^1$.

**Theorem 28.2.4 (Halmos-von Neumann).** Two ergodic rotations $R_\alpha, R_\beta$ on $S^1$ are isomorphic iff $\alpha = \pm\beta$. The spectrum of $R_\alpha$ is $\{e^{2\pi i n\alpha} : n \in \mathbb{Z}\}$.

This is a complete classification for rotations: the spectrum is a complete invariant. For rotations, the Koopman functor is fully faithful. For more complex systems — Bernoulli shifts, mixing systems — the Koopman spectrum is the same (the whole circle), and more sophisticated invariants are needed. This is where entropy and the deeper theory of Chapter 7 come in.

## 28.2.2 The Groupoid of a Dynamical System

The Koopman functor translates dynamics into Hilbert space operators. The groupoid construction translates it into algebra — specifically, into a groupoid, which generalizes both groups and equivalence relations.

**Definition 28.2.5.** The *orbit groupoid* (or *transformation groupoid*) of a dynamical system $(X, f)$ is:
$$\mathcal{G}(X, f) = \{(x, n, y) : f^n(x) = y, n \in \mathbb{Z}\}$$
with multiplication $(x, n, y) \cdot (y, m, z) = (x, n+m, z)$ and inversion $(x, n, y)^{-1} = (y, -n, x)$.

Think of the groupoid as encoding the orbit structure: the elements $(x, n, y)$ say "$x$ and $y$ are in the same orbit, $n$ steps apart." The multiplication says "if $x$ goes to $y$ in $n$ steps and $y$ goes to $z$ in $m$ steps, then $x$ goes to $z$ in $n+m$ steps." It's a category where every morphism has an inverse.

**Theorem 28.2.6.** Two minimal dynamical systems are orbit-equivalent iff their orbit groupoids are isomorphic as étale groupoids.

This is the groupoid formulation of the Giordano-Putnam-Skau theorem for minimal $\mathbb{Z}$-systems. Orbit equivalence — a coarser notion than conjugacy — is captured by groupoid isomorphism. The groupoid sees exactly the right amount of structure.

The next section brings in the most functorial of all the constructions: the topos.
