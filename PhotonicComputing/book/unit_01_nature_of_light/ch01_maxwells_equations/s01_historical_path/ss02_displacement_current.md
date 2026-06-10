# 1.1.2 The Displacement Current: Maxwell's Contribution

## The Problem with Ampère's Law

Ampère's law as stated by Ampère — $\oint_C \mathbf{B} \cdot d\boldsymbol{\ell} = \mu_0 I_{\text{enc}}$ — works perfectly for steady currents. But Maxwell noticed in the 1860s that it is inconsistent with the conservation of charge when currents are time-varying [1]. Let us examine this inconsistency carefully, because the resolution leads directly to the prediction of electromagnetic waves.

Consider a parallel-plate capacitor being charged by a wire. The current $I$ flows through the wire and onto the plates; charge accumulates on the plates; the electric field between the plates grows. Now apply Ampère's law: choose a closed loop $C$ around the wire, and consider two different surfaces bounded by that loop.

**Surface 1**: A flat disk that the wire passes through. The enclosed current is $I$. Ampère's law gives $\oint_C \mathbf{B} \cdot d\boldsymbol{\ell} = \mu_0 I$. Fine.

**Surface 2**: A balloon-shaped surface that bulges out between the capacitor plates, avoiding the wire entirely. No charge passes through this surface — the current stops at the plates. Ampère's law gives $\oint_C \mathbf{B} \cdot d\boldsymbol{\ell} = 0$.

But the loop $C$ is the same in both cases! The left side of Ampère's law — the line integral of $\mathbf{B}$ around the fixed loop $C$ — cannot equal two different values. Ampère's law, applied to time-varying situations, gives contradictory results depending on which surface you choose. It is therefore incomplete.

## The Displacement Current: A Logical Necessity

Maxwell's resolution was to add a new term to Ampère's law. Between the capacitor plates, even though no charge current flows, there is a changing electric field — as charge accumulates on the plates, the electric field between them increases. Maxwell proposed that this changing electric field has the same magnetic effect as a real current. He called the term the **displacement current density**:

$$\mathbf{J}_D = \varepsilon_0 \frac{\partial \mathbf{E}}{\partial t}$$

With this addition, the modified Ampère-Maxwell law becomes:

$$\oint_C \mathbf{B} \cdot d\boldsymbol{\ell} = \mu_0 I_{\text{enc}} + \mu_0 \varepsilon_0 \frac{d}{dt}\int_S \mathbf{E} \cdot d\mathbf{A}$$

Let us verify that this resolves the capacitor paradox.

**Surface 2 revisited**: No real current passes through the balloon surface, so $I_{\text{enc}} = 0$. But the electric field between the plates is changing, and the flux of $\mathbf{E}$ through the balloon surface is not zero. The rate of change of that flux equals $\mu_0^{-1}$ times the current flowing onto the plates — which equals $I$. So both surfaces now give the same result: $\mu_0 I$. The inconsistency is resolved.

## Why "Displacement"?

The name "displacement current" is a historical artifact of Maxwell's original ether model, in which he imagined the field as a mechanical stress in a medium [2]. In the modern view, there is no ether, and the displacement current is simply what happens when an electric field changes: it has the same magnetic effect as a real current. The name persists, but the physical picture is clear: **a time-varying electric field produces a magnetic field**.

## The Symmetry This Restores

Faraday's law tells us: a time-varying magnetic field produces an electric field.
Maxwell's addition tells us: a time-varying electric field produces a magnetic field.

Together, these two statements create a symmetry between electric and magnetic fields that did not exist in Ampère's original formulation. And this symmetry — as we will see in Section 1.4 — is precisely what allows the electromagnetic field to sustain itself as a propagating wave, with neither the electric nor the magnetic component running down, each continuously regenerating the other.

This is the physical mechanism of light. And it follows from a logical requirement — the consistency of Ampère's law with charge conservation — rather than from any new experimental discovery. Maxwell's displacement current was a theoretical prediction, not an empirical finding.

## Charge Conservation as a Constraint

Let us make explicit the connection between the displacement current and charge conservation. The law of charge conservation states that charge cannot be created or destroyed:

$$\frac{\partial \rho}{\partial t} + \nabla \cdot \mathbf{J} = 0$$

where $\rho$ is the charge density and $\mathbf{J}$ is the current density. This is the *continuity equation* for charge.

Gauss's law says $\nabla \cdot \mathbf{E} = \rho / \varepsilon_0$. Taking the time derivative: $\nabla \cdot (\partial \mathbf{E}/\partial t) = (1/\varepsilon_0) \partial \rho / \partial t$. By charge conservation, $\partial \rho / \partial t = -\nabla \cdot \mathbf{J}$, so:

$$\nabla \cdot \left(\mathbf{J} + \varepsilon_0 \frac{\partial \mathbf{E}}{\partial t}\right) = 0$$

The quantity in parentheses — the total current density including the displacement current — is divergence-free. In Ampère's law, it is the divergence-free total current that appears on the right side, ensuring consistency. Without the displacement current, Ampère's law would violate charge conservation for time-varying fields.

The displacement current is therefore not an optional addition to Ampère's law. It is required by the empirically established law of charge conservation. Maxwell did not guess it; he derived its necessity from what was already known.

## Historical Significance

The completion of Ampère's law by the displacement current is one of the great examples of theoretical physics. Maxwell did not add the displacement current because experiment demanded it — no experiment at the time could directly measure the displacement current in vacuum. He added it because *logic* demanded it. And then — as he showed in the same paper [1] — the modified equations predicted that the electromagnetic field could propagate as a wave, with a speed that he calculated to be $c = 1/\sqrt{\mu_0 \varepsilon_0} \approx 3 \times 10^8$ m/s. This was already known to be the speed of light. The identification of light as an electromagnetic wave was thus not a speculation but a mathematical consequence.

---

## References

[1] Maxwell, J.C. (1865). "A dynamical theory of the electromagnetic field." *Philosophical Transactions of the Royal Society of London*, 155, 459–512. [Maxwell's original paper containing the displacement current and the prediction of electromagnetic waves.]

[2] Maxwell, J.C. (1861–1862). "On physical lines of force." *Philosophical Magazine*, 21 and 23. [Maxwell's earlier mechanical ether model, from which the displacement current was originally conceived.]

[3] Griffiths, D.J. (2017). *Introduction to Electrodynamics*, 4th ed. Cambridge University Press. Ch. 7. [Clear modern treatment of the displacement current and its necessity.]
