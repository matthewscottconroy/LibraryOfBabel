# 14.1 Symplectic Geometry

Hamiltonian mechanics lives on a symplectic manifold. This is not just a formal generalization — the symplectic structure is the mathematical encoding of the Hamiltonian exchange between position and momentum, and it is what makes energy conservation possible and the theory so rigid.

To understand why we need a 2-form (rather than, say, a metric), think about what Hamilton's equations say: $\dot{q} = \partial H/\partial p$ and $\dot{p} = -\partial H/\partial q$. The vector field $(\dot{q}, \dot{p})$ is determined by the gradient $(dH)$ of the Hamiltonian via a skew-symmetric pairing that maps $\partial/\partial p \mapsto dq$ and $\partial/\partial q \mapsto -dp$. That skew-symmetric pairing is the symplectic form.

**Definition 14.1.1.** A *symplectic manifold* $(M, \omega)$ is a smooth manifold $M^{2n}$ equipped with a closed ($d\omega = 0$) non-degenerate ($\omega^n \neq 0$, i.e., $\omega \wedge \cdots \wedge \omega$ is a volume form) 2-form $\omega$ (the *symplectic form*).

**Canonical Example:** $({\mathbb R}^{2n}, \omega_0)$ with:
$$\omega_0 = \sum_{i=1}^n dq_i \wedge dp_i.$$

This is the standard symplectic form on phase space. The $dq_i \wedge dp_i$ term says that position and momentum in the $i$-th direction are paired by $\omega_0$.

**Theorem 14.1.2 (Darboux).** Every symplectic manifold is locally symplectomorphic to $(\mathbb{R}^{2n}, \omega_0)$: there exist local coordinates $(q_1, \ldots, q_n, p_1, \ldots, p_n)$ (Darboux coordinates) with $\omega = \sum_i dq_i \wedge dp_i$.

*Darboux's theorem is the symplectic analogue of the Riemannian theorem on normal coordinates — but with the opposite conclusion. A Riemannian manifold has intrinsic local invariants (curvature), so no such uniform local form exists. A symplectic manifold has no local invariants whatsoever: every symplectic manifold of dimension $2n$ looks locally identical to every other. All the interesting symplectic geometry is global.*

This is striking: you cannot tell a "curved" symplectic manifold from a flat one by looking locally. The global topology (the de Rham cohomology class of $\omega$, for instance) is a genuine invariant, but all local structure is trivially equivalent.

## 14.1.1 Hamilton's Equations

Given a Hamiltonian function $H: M \to \mathbb{R}$, the dynamics is determined by the symplectic form.

**Definition 14.1.3.** Given $H: M \to \mathbb{R}$, the *Hamiltonian vector field* $X_H$ is defined by:
$$\iota_{X_H}\omega = dH,$$
i.e., $\omega(X_H, \cdot) = dH(\cdot)$.

In Darboux coordinates, this gives the familiar Hamilton's equations:
$$\dot{q}_i = \frac{\partial H}{\partial p_i}, \quad \dot{p}_i = -\frac{\partial H}{\partial q_i}.$$

The symplectic form converts the "gradient" $dH$ (a 1-form, a covector field) into a vector field $X_H$. The skew-symmetry of $\omega$ is what produces the minus sign in $\dot{p}_i = -\partial H/\partial q_i$.

**Theorem 14.1.4 (Conservation of Energy).** $H$ is constant along trajectories:
$$\frac{d}{dt} H(\gamma(t)) = dH(\dot\gamma) = dH(X_H) = \omega(X_H, X_H) = 0$$
(using $\omega(X_H, X_H) = 0$ because $\omega$ is antisymmetric).

**Theorem 14.1.5 (Liouville's Theorem).** The Hamiltonian flow preserves the symplectic volume form $\omega^n/n!$ (the *Liouville measure*). In Darboux coordinates: $dq_1 \cdots dq_n\, dp_1 \cdots dp_n$ is invariant.

*Proof:* $\mathcal{L}_{X_H}(\omega^n) = n\omega^{n-1} \wedge \mathcal{L}_{X_H}\omega = n\omega^{n-1} \wedge d(\iota_{X_H}\omega) = n\omega^{n-1} \wedge d(dH) = 0$.

Liouville's theorem is fundamental: Hamiltonian flows preserve phase-space volume. This is the reason that Hamiltonian systems cannot have attractors in the usual sense — you cannot have an attracting set of lower dimension if volume is preserved. (In contrast, the dissipative systems of Chapter 11 — the Lorenz system, the Hénon map — do not preserve volume, and that is why they have strange attractors.)

The contrast between dissipative and conservative systems is one of the organizing themes of the subject. We are now firmly in the conservative world.
