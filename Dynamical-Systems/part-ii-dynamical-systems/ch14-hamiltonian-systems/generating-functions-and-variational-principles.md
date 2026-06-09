# 14.6 Generating Functions and Variational Principles

There is a variational approach to Hamiltonian mechanics that is older than the symplectic approach — Hamilton's principle of least action — and it turns out to be the right framework for proving results like Aubry-Mather theory and Arnold diffusion. The key is to describe symplectomorphisms and their orbits as critical points of an action functional.

## Generating Functions

A symplectomorphism $f: M \to M$ can often be described by a single function — its generating function — that encodes the map implicitly. For a symplectomorphism of $(T^*Q, \sum dp \wedge dq)$, the generating function $S(q, Q)$ satisfies:

**Definition 14.6.1.** For a symplectomorphism $f: M \to M$, a *generating function* $S(q, Q)$ satisfies:
$$p = -\frac{\partial S}{\partial q}, \quad P = \frac{\partial S}{\partial Q},$$
where $(q, p) \mapsto (Q, P) = f(q, p)$.

This is an implicit description: given $q$ and $Q$ (the "before" and "after" positions), the momentum $p$ and $P$ are determined by the partial derivatives of $S$. Not every symplectomorphism has a generating function globally, but every one does locally.

For the standard map, the generating function is $S(q, Q) = (Q-q)^2/2 + K\cos q$, and the orbits of the map correspond to sequences $(q_n)$ that minimize the total action $\sum_n S(q_n, q_{n+1})$.

## The Principle of Least Action

**Hamilton's Variational Principle:** True orbits of the Hamiltonian system are the critical points of the action functional:
$$\mathcal{A}[\gamma] = \int_{t_1}^{t_2} \left(p\,\dot{q} - H(q,p)\right)\,dt,$$
i.e., $\frac{d}{d\varepsilon}\Big|_{\varepsilon=0} \mathcal{A}[\gamma + \varepsilon \eta] = 0$ for all compactly supported variations $\eta$.

This variational principle is one of the most beautiful ideas in classical mechanics: the actual trajectory of a system is the one that makes the action stationary — not necessarily a minimum, but a critical point (the "principle of stationary action" is more accurate than "least action").

For the Euler-Lagrange equations (the Lagrangian version on configuration space $Q$), the action is $\mathcal{A}[q] = \int L(q, \dot{q})\,dt$ where $L = p\dot{q} - H$ is the Lagrangian. Critical points of this functional satisfy the equations of motion.

## Mather's Variational Theory

**Theorem 14.6.2 (Mather's Variational Theory).** Define the *Mather set* $\tilde{\mathcal{M}}_\alpha$ as the support of the *action-minimizing measure* of rotation number $\alpha$: the probability measure on the torus that minimizes the average action $\int L(q, \dot{q})\,d\mu$ over all invariant measures of rotation number $\alpha$.

Mather sets generalize KAM tori to the non-perturbative regime. They provide invariant objects (action-minimizing orbits and measures) even when KAM theory breaks down completely. The Aubry-Mather sets of Section 14.4 are the Mather sets for twist maps.

Mather's theory works without any smallness assumption on the Hamiltonian — it is a non-perturbative theory. The Mather sets always exist, for any Hamiltonian and any rotation number/cohomology class. When KAM tori exist, they are the Mather sets. When they do not, the Mather sets are Cantor sets or more complicated objects.

The deep connection between the variational approach (action-minimizing orbits) and the geometric approach (invariant manifolds, heteroclinic connections) is the key to proving Arnold diffusion: Mather's "connecting orbits" theorem says that under generic conditions, the Mather sets for different rotation numbers are connected by heteroclinic orbits, and these heteroclinic connections allow orbits to drift across the resonance web.
