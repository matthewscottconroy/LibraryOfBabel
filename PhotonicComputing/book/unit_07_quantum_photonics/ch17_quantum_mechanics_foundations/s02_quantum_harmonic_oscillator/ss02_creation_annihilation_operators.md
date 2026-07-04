# 17.2.2 Creation and Annihilation Operators

## Factoring the Hamiltonian

Define the dimensionless, non-Hermitian pair

$$\hat{a} = \sqrt{\frac{m\omega}{2\hbar}}\left(\hat{x} + \frac{i\hat{p}}{m\omega}\right), \qquad \hat{a}^\dagger = \sqrt{\frac{m\omega}{2\hbar}}\left(\hat{x} - \frac{i\hat{p}}{m\omega}\right)$$

with the inverse relations

$$\hat{x} = \sqrt{\frac{\hbar}{2m\omega}}\,(\hat{a} + \hat{a}^\dagger), \qquad \hat{p} = -i\sqrt{\frac{\hbar m\omega}{2}}\,(\hat{a} - \hat{a}^\dagger)$$

The single canonical commutator $[\hat{x}, \hat{p}] = i\hbar$ translates into

$$[\hat{a}, \hat{a}^\dagger] = 1$$

*Derivation:* only the cross terms survive, since $[\hat{x},\hat{x}] = [\hat{p},\hat{p}] = 0$:

$$[\hat{a}, \hat{a}^\dagger] = \frac{m\omega}{2\hbar}\left(\left[\hat{x},\, -\frac{i\hat{p}}{m\omega}\right] + \left[\frac{i\hat{p}}{m\omega},\, \hat{x}\right]\right) = \frac{m\omega}{2\hbar}\left(-\frac{i}{m\omega}(i\hbar) - \frac{i}{m\omega}(i\hbar)\right) = \frac{m\omega}{2\hbar}\cdot\frac{2\hbar}{m\omega} = 1$$

Substituting into $\hat{H}$ and using the commutator once to reorder $\hat{a}\hat{a}^\dagger = \hat{a}^\dagger\hat{a} + 1$:

$$\hat{H} = \hbar\omega\left(\hat{a}^\dagger\hat{a} + \frac{1}{2}\right) \equiv \hbar\omega\left(\hat{n} + \frac{1}{2}\right), \qquad \hat{n} = \hat{a}^\dagger\hat{a}$$

The **number operator** $\hat{n}$ is Hermitian and positive semidefinite ($\langle\psi|\hat{n}|\psi\rangle = \|\hat{a}|\psi\rangle\|^2 \geq 0$), so the entire spectrum problem reduces to finding the eigenvalues of $\hat{n}$.

## The Ladder Algebra

From $[\hat{a}, \hat{a}^\dagger] = 1$ follow the two commutators that give the operators their names:

$$[\hat{n}, \hat{a}] = -\hat{a}, \qquad [\hat{n}, \hat{a}^\dagger] = +\hat{a}^\dagger$$

Let $|n\rangle$ be a normalized eigenstate, $\hat{n}|n\rangle = n|n\rangle$. Then

$$\hat{n}\,(\hat{a}|n\rangle) = (\hat{a}\hat{n} - \hat{a})|n\rangle = (n-1)\,(\hat{a}|n\rangle)$$

so $\hat{a}|n\rangle$ is again an eigenstate, with eigenvalue lowered by one; likewise $\hat{a}^\dagger|n\rangle$ raises it by one. The operators walk the ladder. Their normalizations follow from $\|\hat{a}|n\rangle\|^2 = \langle n|\hat{a}^\dagger\hat{a}|n\rangle = n$ and $\|\hat{a}^\dagger|n\rangle\|^2 = \langle n|\hat{a}\hat{a}^\dagger|n\rangle = n + 1$:

$$\boxed{\;\hat{a}|n\rangle = \sqrt{n}\,|n-1\rangle, \qquad \hat{a}^\dagger|n\rangle = \sqrt{n+1}\,|n+1\rangle\;}$$

Now the spectrum falls out of positivity. Descending the ladder from any eigenstate lowers $n$ by integers; if $n$ were not an integer, repeated lowering would eventually produce a state with negative norm-squared $n < 0$ — impossible. The descent must instead *terminate*, which happens only at a state annihilated outright:

$$\hat{a}|0\rangle = 0$$

This is the **vacuum state** (ground state), with $\hat{n}$-eigenvalue $0$ and energy $\hbar\omega/2$. The allowed eigenvalues are exactly $n = 0, 1, 2, \ldots$, and every excited state is built by raising:

$$|n\rangle = \frac{(\hat{a}^\dagger)^n}{\sqrt{n!}}\,|0\rangle$$

One commutation relation has produced the full spectrum, the eigenstates, and their normalizations — no differential equation solved, no Hermite polynomial in sight. When the oscillator is a field mode, $\hat{a}^\dagger$ **creates a photon** and $\hat{a}$ **annihilates one**; the $\sqrt{n}$ and $\sqrt{n+1}$ factors are not bookkeeping but physics: the $\sqrt{n+1}$ in emission is the origin of stimulated emission's proportionality to occupation (the "$+1$" being spontaneous emission), as first exploited in the laser theory of Chapter 4, and the $\sqrt{n}$ in absorption makes an $n$-photon state $n$ times easier to absorb from.

**A caution on interpretation:** $\hat{a}$ is not Hermitian and is *not* an observable; there is no "annihilation meter." Its role is structural — observables ($\hat{x}$, $\hat{p}$, $\hat{n}$, the field $\hat{E}$) and dynamics are all built from it. Its eigenstates, however, exist and are legitimate states: they are the coherent states of Section 17.3.2, and the asymmetry that $\hat{a}$ has eigenstates while $\hat{a}^\dagger$ has none (removing the bottom rung is impossible to invert) will matter there.

## Heisenberg Dynamics: The Rotating Amplitude

In the Heisenberg picture (17.1.4), the equation of motion is

$$\frac{d\hat{a}}{dt} = \frac{i}{\hbar}[\hat{H}, \hat{a}] = \frac{i}{\hbar}\,\hbar\omega\,[\hat{n}, \hat{a}] = -i\omega\,\hat{a} \quad\Longrightarrow\quad \hat{a}(t) = \hat{a}(0)\,e^{-i\omega t}$$

The annihilation operator is the quantum descendant of the classical complex amplitude $\alpha e^{-i\omega t}$ of Chapters 1–2: it rotates uniformly in phase space at the oscillation frequency. Every phasor manipulation in classical photonics — phase shifts, interference, coupled-mode theory — lifts to quantum mechanics by the substitution *complex amplitude $\to$ annihilation operator*. This substitution rule is the working method of Chapter 18.

**Worked example (quadrature fluctuations of a Fock state).** Compute $\langle\hat{x}\rangle$ and $\langle\hat{x}^2\rangle$ in the state $|n\rangle$. Since $\hat{x} \propto (\hat{a} + \hat{a}^\dagger)$ connects $|n\rangle$ only to $|n\pm 1\rangle$, orthogonality gives $\langle n|\hat{x}|n\rangle = 0$: *a number state has zero mean displacement* — and, for a field mode, zero mean field, however many photons it holds. For the second moment,

$$\langle n|\hat{x}^2|n\rangle = \frac{\hbar}{2m\omega}\,\langle n|(\hat{a} + \hat{a}^\dagger)^2|n\rangle = \frac{\hbar}{2m\omega}\,\langle n|(\hat{a}\hat{a}^\dagger + \hat{a}^\dagger\hat{a})|n\rangle = \frac{\hbar}{2m\omega}\,(2n + 1)$$

(the $\hat{a}^2$ and $\hat{a}^{\dagger 2}$ terms connect to $|n \mp 2\rangle$ and vanish in the expectation). The fluctuations grow linearly with $n$: energy in a Fock state lives entirely in *noise* around a zero mean, the first hint (developed in 17.2.3) that photon-number states are as far from classical stable waves as a state can be.

## Normal Ordering: A Convention with Physical Teeth

A product of ladder operators is **normally ordered** when all $\hat{a}^\dagger$ stand to the left of all $\hat{a}$ (notation $:\!\hat{a}\hat{a}^\dagger\!: \,= \hat{a}^\dagger\hat{a}$). Reordering costs commutators — each swap of $\hat{a}$ past $\hat{a}^\dagger$ produces a $+1$, a vacuum contribution. Normally ordered expectation values therefore vanish in vacuum by construction, which is exactly right for **photodetection**: a detector absorbs photons and cannot click on vacuum fluctuations. Glauber's photodetection theory (Section 18.1) accordingly defines measurable intensities and correlation functions as normally ordered moments, $\langle \hat{a}^\dagger \hat{a}\rangle$, $\langle \hat{a}^{\dagger}\hat{a}^{\dagger}\hat{a}\hat{a}\rangle$, and the ordering convention silently encodes which quantities experiments can see.
