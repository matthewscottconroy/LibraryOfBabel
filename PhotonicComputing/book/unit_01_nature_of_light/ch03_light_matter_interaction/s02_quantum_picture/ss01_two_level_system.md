# 3.2.1 — The Two-Level System

## Energy Levels and Transition Frequency

The simplest quantum model of an atom is the *two-level system* (TLS): it has exactly two energy eigenstates, $|1\rangle$ (ground state, energy $E_1$) and $|2\rangle$ (excited state, energy $E_2 > E_1$). The *transition frequency* is:

$$\omega_0 = \frac{E_2 - E_1}{\hbar}$$

All interactions involving this atom with an electromagnetic field near frequency $\omega_0$ can be described (to first approximation) by this two-level model. Real atoms have many energy levels, but near any particular transition, the two-level approximation is valid as long as other transitions are well separated in frequency.

## The Transition Dipole Moment

The interaction between the atom and the electric field is dominated by the *electric dipole interaction*:

$$H_\text{int} = -\hat{\mathbf{d}} \cdot \mathbf{E}(t) = -\mathbf{d}_{12} \cdot \mathbf{E}(t)(|1\rangle\langle 2| + |2\rangle\langle 1|)$$

where $\mathbf{d}_{12} = \langle 1|\hat{\mathbf{d}}|2\rangle = -e\langle 1|\hat{\mathbf{r}}|2\rangle$ is the *transition dipole moment* — the matrix element of the dipole operator between the ground and excited states. This is a real vector (in most cases) with magnitude of order $ea_0 \sim 10^{-29}$ C·m (Bohr radius times electron charge).

The transition dipole moment determines:
- How strongly the atom couples to the field (large $\mathbf{d}_{12}$ = strong coupling)
- The selection rules for which transitions are allowed (transitions with $\mathbf{d}_{12} = 0$ are "dipole-forbidden")
- The rate of spontaneous emission (scales as $|\mathbf{d}_{12}|^2\omega_0^3$)

## State Evolution: The Bloch Equations

The general state of the two-level system is a superposition:

$$|\psi(t)\rangle = c_1(t)|1\rangle + c_2(t)|2\rangle$$

with $|c_1|^2 + |c_2|^2 = 1$. The populations are $N_1 = |c_1|^2$ and $N_2 = |c_2|^2$ (fraction of time in each state, or fraction of an ensemble of atoms in each state).

The Schrödinger equation for the two-level system driven by a resonant field $E(t) = E_0\cos\omega_0 t$ gives (after the rotating wave approximation, which drops terms oscillating at $2\omega_0$):

$$\dot{c}_2 = -\frac{i\Omega_R}{2}e^{i\phi}c_1, \qquad \dot{c}_1 = -\frac{i\Omega_R}{2}e^{-i\phi}c_2$$

where $\Omega_R = |\mathbf{d}_{12} \cdot \mathbf{E}_0|/\hbar$ is the *Rabi frequency* and $\phi$ is the phase of the field.

The solution for an atom initially in $|1\rangle$ ($c_1(0) = 1$, $c_2(0) = 0$):

$$|c_2(t)|^2 = \sin^2\left(\frac{\Omega_R t}{2}\right)$$

The excited-state population oscillates between 0 and 1 at the Rabi frequency $\Omega_R$. These are *Rabi oscillations*: the atom cycles coherently between ground and excited states at a rate determined by the field amplitude.

**Rabi frequency and field intensity**: $\Omega_R = |\mathbf{d}_{12}|E_0/\hbar$. For a typical atomic dipole moment $|\mathbf{d}_{12}| = ea_0 \approx 8.5 \times 10^{-30}$ C·m and field $E_0 = 10^6$ V/m (intensity $\sim 1.3$ MW/m²): $\Omega_R \approx 8.5 \times 10^{-30} \times 10^6/(10^{-34}) \approx 8.5 \times 10^{10}$ rad/s $= 85$ Grad/s. The Rabi period $2\pi/\Omega_R \approx 74$ ps — achievable with modern pulsed lasers.

## Detuning and Off-Resonance Behavior

For a field at frequency $\omega$ detuned from resonance by $\Delta = \omega - \omega_0$ (the *detuning*), the excited state population oscillates at the *generalized Rabi frequency* $\tilde{\Omega}_R = \sqrt{\Omega_R^2 + \Delta^2}$ but never reaches 1 for $\Delta \neq 0$:

$$|c_2(t)|^2 = \frac{\Omega_R^2}{\tilde{\Omega}_R^2}\sin^2\left(\frac{\tilde{\Omega}_R t}{2}\right)$$

Maximum excited-state population is $\Omega_R^2/(\Omega_R^2 + \Delta^2) < 1$ — decreasing with detuning.

## The Bloch Sphere

The state of a two-level system can be visualized geometrically using the Bloch sphere (isomorphic to the Poincaré sphere for polarization, Section 2.4.3). Define the *Bloch vector* $\mathbf{r} = (u, v, w)$:

$$u = 2\text{Re}(c_1^* c_2), \quad v = 2\text{Im}(c_1^* c_2), \quad w = |c_2|^2 - |c_1|^2$$

$w = -1$: atom in ground state $|1\rangle$. $w = +1$: atom in excited state $|2\rangle$. Equator ($w = 0$): equal superposition.

The Schrödinger equation for the two-level system translates into a precession equation for the Bloch vector:

$$\dot{\mathbf{r}} = \boldsymbol{\Omega} \times \mathbf{r}$$

where $\boldsymbol{\Omega} = (\Omega_R\cos\phi, -\Omega_R\sin\phi, -\Delta)$ is the "torque vector." Resonant excitation ($\Delta = 0$) causes the Bloch vector to precess around the equatorial axis — rotating from the south pole ($|1\rangle$) through the equator (superposition) to the north pole ($|2\rangle$) in a *$\pi$-pulse* (Rabi flip).

**Relevance to photonic computing**: The Bloch sphere is the state space of a qubit in quantum information theory. In quantum photonic computing (Unit VII), two-level systems (quantum dots, nitrogen-vacancy centers, trapped ions) are used as qubits. The Rabi frequency and detuning are control parameters for implementing quantum gates. Understanding the two-level dynamics is therefore essential for quantum photonic computing as well as for understanding laser gain.

## Relaxation and Dephasing

Real two-level systems are not isolated. They couple to their environment (phonons, electromagnetic vacuum fluctuations, neighboring atoms), leading to:

- **Longitudinal relaxation** (population decay) at rate $1/T_1$: the excited state decays to the ground state, emitting a photon (spontaneous emission) or a phonon. $T_1$ is the population lifetime.
- **Transverse relaxation** (dephasing) at rate $1/T_2$: the off-diagonal coherences ($c_1^*c_2$) decay due to random phase kicks from the environment. $T_2 \leq 2T_1$.

The full equations including relaxation are the *optical Bloch equations*:

$$\dot{u} = -\Delta v - u/T_2$$
$$\dot{v} = \Delta u + \Omega_R w - v/T_2$$
$$\dot{w} = -\Omega_R v - (w+1)/T_1$$

In steady state (CW field), the solution gives the population inversion $w$ and coherences $u$, $v$ as Lorentzian functions of detuning — reproducing the Lorentz oscillator result in the limit of weak fields. The classical Lorentz oscillator is the limit of the quantum two-level system at low field intensity.

## Summary

- Two-level system: ground state $|1\rangle$, excited state $|2\rangle$, transition frequency $\omega_0 = (E_2-E_1)/\hbar$.
- Transition dipole moment $\mathbf{d}_{12}$: determines coupling strength, selection rules, spontaneous emission rate.
- Resonant driving → Rabi oscillations at $\Omega_R = |\mathbf{d}_{12}|E_0/\hbar$.
- Bloch sphere: geometric picture of two-level state; precession = Rabi oscillation.
- Optical Bloch equations: include population relaxation ($T_1$) and dephasing ($T_2$).
