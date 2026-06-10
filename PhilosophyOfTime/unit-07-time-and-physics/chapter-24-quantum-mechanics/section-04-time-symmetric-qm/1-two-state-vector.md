# The Two-State Vector Formalism

## Aharonov, Bergmann, and Lebowitz (1964)

The seed of the two-state vector formalism was planted in a 1964 paper by Yakir Aharonov, Peter Bergmann, and Joel Lebowitz. They asked a natural question: in standard quantum mechanics, the probability of measuring outcome aₙ when an observable A is measured on a system prepared in state |ψ⟩ is |⟨aₙ|ψ⟩|². This gives the probability of the outcome conditioned on the initial state preparation. But what is the probability of outcome aₙ conditioned on *both* the initial preparation |ψ⟩ *and* a final state measurement that found the system in state |φ⟩?

The answer, they showed, is given by the *Aharonov-Bergmann-Lebowitz (ABL) rule*:

*P(aₙ | ψ, φ) = |⟨φ|aₙ⟩⟨aₙ|ψ⟩|² / Σₘ |⟨φ|aₘ⟩⟨aₘ|ψ⟩|²*

This rule is time-symmetric: it treats the initial state |ψ⟩ and the final state |φ⟩ symmetrically. Both the past and the future boundary conditions contribute to the probability of the intermediate measurement outcome. The formula is not the standard rule (which uses only the initial state), but it is what follows if we condition on both boundary conditions.

## The Two-State Vector

Aharonov and Vaidman (1991) developed these ideas into a systematic reformulation. In the two-state vector formalism, a quantum system at any intermediate time t (between preparation at time t₁ and final measurement at time t₂) is described by *two* state vectors:

- |Ψ(t)⟩: the usual forward-evolving state, propagated from the initial state by the Schrödinger equation.
- ⟨Φ(t)|: a backward-evolving state, propagated from the final state backward in time by the time-reversed Schrödinger equation.

Together, these two vectors — the "two-state vector" — give a complete description of the quantum system at the intermediate time. Neither vector alone is sufficient; the quantum reality of the system at time t is not captured by forward-evolving state or backward-evolving state alone.

## Weak Values

One of the key applications of the TSVF is the concept of *weak values*. In a standard quantum measurement, the outcome is an eigenvalue of the measured observable, and the post-measurement state is the corresponding eigenstate. This is a "strong" measurement that disturbs the state.

Aharonov and Vaidman showed that if a measurement is sufficiently "weak" — if the measurement interaction is too weak to collapse the state, but is instead recorded in a pointer variable — and if we *post-select* on a specific final state, then the average value of the pointer variable is the *weak value* of the observable:

*A_w = ⟨Φ|Â|Ψ⟩ / ⟨Φ|Ψ⟩*

Weak values can be complex numbers, and they can lie far outside the range of eigenvalues of Â. This is not a measurement error; it is a genuine physical result that has been experimentally confirmed.

## Retrocausation and the Arrow of Time

The two-state vector formalism suggests that quantum reality at any moment is determined by both past and future boundary conditions. This is a genuinely time-symmetric picture of quantum mechanics. Does it imply *retrocausation* — causal influence from the future to the past?

Aharonov and various collaborators have suggested that it does, in a specific sense: the future boundary condition (the post-selection) affects the "weak values" of observables at intermediate times, and these weak values are physically real and measurable. In this sense, the future influences the past.

However, it is important to distinguish two interpretations of this claim:

1. **The predictive interpretation:** The TSVF is a mathematical tool for calculating probabilities of measurement outcomes, conditioned on both past and future boundary conditions. On this interpretation, the "retrocausation" is merely a feature of the calculation, not a claim about ontological influence from the future.

2. **The ontological interpretation:** Aharonov's preferred interpretation is that the two-state vector is a genuine description of quantum reality, and the future boundary condition genuinely constitutes part of what the system is at the intermediate time. On this view, retrocausation is a feature of the physical world.

If retrocausation is real in the sense of (2), it has profound implications for the arrow of time. The standard picture — causes precede effects, the future is determined by the past — would need to be revised: in quantum mechanics, "future" boundary conditions would be as physically real and causally relevant as "past" boundary conditions. The arrow of time, on this view, would not be a feature of quantum reality but a feature of how we choose to specify boundary conditions.

The TSVF remains a minority interpretation, but it illustrates the richness of the conceptual landscape around time in quantum mechanics. Whether quantum mechanics is fundamentally time-asymmetric or time-symmetric — whether the future is genuinely open or as determined as the past — is not settled by the formalism alone. It requires philosophical interpretation.

**References**

Aharonov, Yakir, Peter G. Bergmann, and Joel L. Lebowitz. 1964. "Time Symmetry in the Quantum Process of Measurement." *Physical Review* 134 (6B): B1410–B1416.

Aharonov, Yakir, and Lev Vaidman. 1991. "Complete Description of a Quantum System at a Given Time." *Journal of Physics A* 24 (10): 2315–2328.

Price, Huw. 1994. "A Neglected Route to Realism about Quantum Mechanics." *Mind* 103 (411): 303–336.
