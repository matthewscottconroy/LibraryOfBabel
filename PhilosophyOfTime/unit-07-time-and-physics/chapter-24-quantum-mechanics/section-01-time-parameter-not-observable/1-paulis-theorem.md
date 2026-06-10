# Pauli's Theorem: Time Cannot Be a Quantum Observable

## The Standard Picture of Quantum Observables

In quantum mechanics, physical observables — measurable properties of a system — are represented by self-adjoint (Hermitian) operators on a Hilbert space. When we measure an observable A, the result is always one of A's eigenvalues, and after the measurement, the state of the system is the corresponding eigenstate. The expectation value of A in state |Ψ⟩ is ⟨Ψ|A|Ψ⟩.

Two observables A and B have a canonical commutation relation [A, B] = iℏ if they are conjugate variables — position and momentum are the canonical example: [x̂, p̂] = iℏ. The Heisenberg uncertainty principle, ΔxΔp ≥ ℏ/2, follows directly from this commutation relation.

The Hamiltonian Ĥ (the energy operator) is the generator of time evolution: the Schrödinger equation is iℏ d|Ψ⟩/dt = Ĥ|Ψ⟩. Time *t* appears as the parameter of evolution, not as an operator. A natural question: is there a time operator T̂ that is canonically conjugate to the Hamiltonian, satisfying [T̂, Ĥ] = iℏ, in analogy with [x̂, p̂] = iℏ?

## Pauli's Argument

Wolfgang Pauli addressed this question in a 1933 footnote in his Handbuch article on quantum mechanics (Pauli 1933). His argument is simple and elegant:

1. Suppose there were a time operator T̂ satisfying [T̂, Ĥ] = iℏ.

2. Then, for any state |E⟩ with definite energy E (an eigenstate of Ĥ with eigenvalue E), the operator exp(-iεT̂/ℏ) would generate a shift in the energy by ε: it would transform |E⟩ to a state |E + ε⟩ with energy E + ε.

3. Since ε is arbitrary, this means that the energy spectrum of any system with a time operator must be the entire real line: the Hamiltonian must have eigenvalues from -∞ to +∞.

4. But physical Hamiltonians are bounded below: they have a ground state with a minimum energy. A system with a ground state cannot have eigenvalues from -∞ to +∞.

5. Therefore, no self-adjoint time operator T̂ canonically conjugate to Ĥ can exist for any system with a bounded-below Hamiltonian.

This is Pauli's theorem. It shows that time, in the standard formulation of quantum mechanics, cannot be a self-adjoint quantum observable.

## What Pauli's Theorem Does and Does Not Show

Pauli's theorem is genuine and important, but it is sometimes over-interpreted. Let us be precise about what it does and does not establish.

**What it shows:** A canonically conjugate time operator — one satisfying [T̂, Ĥ] = iℏ strictly — cannot be self-adjoint if the Hamiltonian is bounded below. Therefore, time is not an observable in the same sense as position or momentum.

**What it does not show:** It does not show that there are no interesting time operators at all. Several responses to Pauli's theorem have been developed:

- *Positive operator-valued measures (POVMs):* There are generalized "observables" in quantum mechanics (POVMs) that are not self-adjoint operators but capture probabilities of measurement outcomes. Various "arrival time" and "time of flight" operators have been defined as POVMs. These evade Pauli's theorem by using the more general POVM formalism (Muga et al. 2002).

- *Restricted spectral support:* It is possible to have an operator T̂ that satisfies [T̂, Ĥ] = iℏ on a restricted domain of states — not globally, but approximately for states localized in energy. Such operators avoid the strict conditions of Pauli's theorem.

- *Different time concepts:* Pauli's theorem applies to a parameter time — the external parameter *t* in the Schrödinger equation. There are also physical time concepts like "arrival time" (when does the particle arrive at a detector?) that are different from parameter time. These physical time quantities can be investigated without running afoul of Pauli's theorem.

The broader point: Pauli's theorem confirms that time plays a different role in quantum mechanics than position or momentum. Time is not a property of the quantum system in the way that energy or angular momentum are. It is an external parameter of the theory. This asymmetry is deeply connected to the problem of time in quantum gravity, which we examine in Chapter 25.

## Why This Matters Philosophically

The asymmetry between time and observables in quantum mechanics has philosophical significance. In classical mechanics, time and space are treated democratically: both are parameters of the theory, and position (a spatial observable) is the dynamical variable that evolves with time. In quantum mechanics, position becomes an operator — a quantum observable — but time does not. Time retains its classical character as an external parameter.

This asymmetry becomes acute when we try to combine quantum mechanics with general relativity. In GR, time is dynamical — it is part of the spacetime metric, which evolves according to the field equations. If we try to quantize GR, we face the question: if time is dynamical in GR and therefore a quantum degree of freedom, how does it reconcile with the quantum mechanical requirement that time is an external parameter? This tension is the heart of the "problem of time" in quantum gravity (Chapter 25).

**References**

Muga, J. G., R. Sala Mayato, and I. L. Egusquiza, eds. 2002. *Time in Quantum Mechanics*. Berlin: Springer.

Pauli, Wolfgang. 1933. "Die allgemeinen Prinzipien der Wellenmechanik." *Handbuch der Physik*, 2nd ed., vol. 24, pt. 1, 83–272. Berlin: Springer. (English translation: "General Principles of Quantum Mechanics." Berlin: Springer, 1980.)

Busch, Paul. 2008. "The Time-Energy Uncertainty Relation." In *Time in Quantum Mechanics*, vol. 1, 2nd ed., edited by J. G. Muga et al., 73–105. Berlin: Springer.
