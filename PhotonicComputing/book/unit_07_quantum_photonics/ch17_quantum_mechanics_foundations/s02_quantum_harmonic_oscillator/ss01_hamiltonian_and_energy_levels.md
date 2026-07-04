# 17.2.1 The Hamiltonian and Energy Levels

## From Classical Spring to Quantum Operator

The classical harmonic oscillator — mass $m$, angular frequency $\omega$ — has energy

$$H = \frac{p^2}{2m} + \frac{1}{2}m\omega^2 x^2$$

Any classical trajectory is an ellipse in the $(x, p)$ phase plane, traversed at frequency $\omega$, with energy free to take *any* non-negative value, including exactly zero (the mass at rest at the origin).

Quantization promotes $x$ and $p$ to Hermitian operators obeying the canonical commutation relation

$$[\hat{x}, \hat{p}] = i\hbar$$

and the Hamiltonian becomes the operator

$$\hat{H} = \frac{\hat{p}^2}{2m} + \frac{1}{2}m\omega^2\hat{x}^2$$

Postulate 2 now makes a sharp claim: the possible energies of the oscillator are the eigenvalues of $\hat{H}$, whatever they turn out to be. The result — derived cleanly in 17.2.2 by the ladder-operator method — is one of the most consequential spectra in physics:

$$E_n = \hbar\omega\left(n + \frac{1}{2}\right), \qquad n = 0, 1, 2, \ldots$$

so that

$$\hat{H} = \hbar\omega\left(\hat{a}^\dagger\hat{a} + \frac{1}{2}\right) = \hbar\omega\left(\hat{n} + \frac{1}{2}\right)$$

Three features of this spectrum deserve individual attention, because each one becomes a pillar of quantum optics.

## Feature 1: The Ladder Is Evenly Spaced

The levels form an infinite ladder with *uniform* spacing $\hbar\omega$. Adding one quantum of excitation always costs exactly the same energy, no matter how excited the oscillator already is. Contrast the hydrogen atom or a semiconductor quantum well, whose level spacings shrink or vary with excitation. The even spacing is what allows us to speak of the $n$-th excited state as containing "$n$ quanta" — $n$ interchangeable packets of energy $\hbar\omega$ — rather than as some unstructured $n$-th level. When the oscillator is a mode of the electromagnetic field at frequency $\omega$ (Section 17.3.1), those quanta are **photons**, and the even spacing is why a photon of a given mode is a well-defined, repeatable object: the second photon in a mode is energetically identical to the first. Planck's $E = \hbar\omega$ and Einstein's photoelectric quanta, encountered in Chapter 3, are recovered here as an *eigenvalue theorem*.

## Feature 2: The Ground State Energy Is Not Zero

The lowest eigenvalue is $E_0 = \hbar\omega/2$, not zero. This **zero-point energy** is forced by the uncertainty principle: a state with $E = 0$ would need both $\langle\hat{x}^2\rangle = 0$ and $\langle\hat{p}^2\rangle = 0$, i.e., definite position *and* momentum, violating $\sigma_x\sigma_p \geq \hbar/2$. The ground state is instead the best compromise the uncertainty relation permits — a Gaussian wavepacket with

$$\sigma_x^2 = \frac{\hbar}{2m\omega}, \qquad \sigma_p^2 = \frac{\hbar m\omega}{2}, \qquad \sigma_x\sigma_p = \frac{\hbar}{2}$$

saturating the bound. The oscillator at absolute zero still jitters. For light this becomes the statement that the electromagnetic **vacuum fluctuates**: even a mode containing zero photons has $\langle\hat{E}^2\rangle \neq 0$ (Section 17.3.1). Those vacuum fluctuations are physical and measurable — they drive spontaneous emission, set the shot-noise floor of interferometry, and are the raw material that squeezing (Sections 17.3.3, 18.3) redistributes.

**Worked example (how big is zero-point motion?).** A silicon optomechanical beam resonator with effective mass $m = 10$ pg $= 10^{-14}$ kg and mechanical frequency $\omega/2\pi = 5$ GHz has

$$x_{\text{zpf}} = \sigma_x = \sqrt{\frac{\hbar}{2m\omega}} = \sqrt{\frac{1.055\times 10^{-34}}{2 \times 10^{-14} \times 3.14\times 10^{10}}} \approx 4.1\times 10^{-16}\ \text{m}$$

— less than a nuclear diameter, yet routinely resolved by cavity optomechanics experiments. And the thermal occupation at temperature $T$ follows Bose-Einstein statistics, $\bar{n} = 1/(e^{\hbar\omega/k_BT} - 1)$: at $T = 300$ K, a 5 GHz mechanical mode holds $\bar{n} \approx k_BT/\hbar\omega \approx 1250$ thermal quanta, while an *optical* mode at $\lambda = 1550$ nm ($\hbar\omega \approx 0.8$ eV, $\hbar\omega/k_B \approx 9280$ K) holds $\bar{n} \approx e^{-31} \approx 3\times 10^{-14}$ — effectively zero. This single Boltzmann factor is why optical quantum photonics works on a room-temperature bench while superconducting (microwave) qubits need dilution refrigerators: at optical frequencies, *the universe is already in its ground state*.

## Feature 3: The Eigenstates Are Number States

The eigenstate $|n\rangle$ belonging to $E_n$ is the **Fock state** or number state, examined in detail in 17.2.3. In the position representation the $|n\rangle$ wavefunctions are Gaussians multiplied by Hermite polynomials,

$$\psi_n(x) = \left(\frac{m\omega}{\pi\hbar}\right)^{1/4}\frac{1}{\sqrt{2^n n!}}\,H_n\!\left(\sqrt{\tfrac{m\omega}{\hbar}}\,x\right)e^{-m\omega x^2/2\hbar}$$

with $n$ nodes each — but we will almost never need them. The algebraic relations of the next subsection carry all the physics, and in field quantization the "position" variable is a field quadrature, not a coordinate anyone measures with a ruler.

## Why the Oscillator Owns This Unit

The oscillator's Hamiltonian is *quadratic* in $\hat{x}$ and $\hat{p}$. Quadratic Hamiltonians have a rare property: they map Gaussian states to Gaussian states and their Heisenberg equations of motion are linear — the quantum dynamics shadows the classical dynamics exactly ($\langle\hat{x}\rangle$ and $\langle\hat{p}\rangle$ obey the classical equations, Ehrenfest's theorem, with no approximation). This is why so much of photonics survives quantization untouched, and why the genuinely quantum phenomena of this unit — antibunching, Hong-Ou-Mandel interference, Wigner negativity — all require either *non-Gaussian states* (single photons) or *measurement* (photon counting) to appear. The boundary between "classical-looking" and "irreducibly quantum" optics is, to a first approximation, the boundary between quadratic and non-quadratic physics; Chapter 21 elevates this observation into an architecture criterion for continuous-variable quantum computers.
