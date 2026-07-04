# 20.2.2 The Nondeterministic CZ Gate

## From Sign Flip to Entangling Gate

The NS gate flips the sign of a two-photon amplitude *in one mode*. A CZ gate needs a sign flip on the two-photon *logical* state $|11\rangle_L$. The bridge between them is Hong-Ou-Mandel interference, which converts "two photons in two modes" into "two photons in one mode" — exactly where NS can bite.

Take dual-rail qubits $A$ (modes $a_0, a_1$) and $B$ (modes $b_0, b_1$). The construction touches only the "1" rails:

1. **Interfere** $a_1$ and $b_1$ on a 50/50 beam splitter.
2. **Apply an NS gate to each output** of that beam splitter.
3. **Recombine** the two outputs on a second 50/50 beam splitter (undoing the first).

Trace the four logical basis states:

- $|00\rangle_L$: no photon touches the central circuit. NS gates act on vacuum: nothing happens.
- $|01\rangle_L, |10\rangle_L$: exactly one photon enters the beam splitter, splits into a superposition across the two NS gates, each acting trivially on $n \leq 1$, and recombines coherently. Net effect: identity.
- $|11\rangle_L$: one photon in $a_1$ *and* one in $b_1$ meet at the splitter. HOM interference eliminates the coincidence amplitude:

$$\hat{a}_1^\dagger \hat{b}_1^\dagger|\text{vac}\rangle \to \frac{i}{2}\left[(\hat{c}^\dagger)^2 + (\hat{d}^\dagger)^2\right]|\text{vac}\rangle \propto |20\rangle + |02\rangle.$$

Every surviving amplitude has *two photons in one mode*. The NS gates multiply each by $-1$:

$$|20\rangle + |02\rangle \ \to\ -\left(|20\rangle + |02\rangle\right),$$

and the second beam splitter coherently reverses the first, returning $-\hat{a}_1^\dagger\hat{b}_1^\dagger|\text{vac}\rangle$.

Net transformation, conditioned on both NS gates heralding success:

$$|00\rangle_L \to |00\rangle_L, \quad |01\rangle_L \to |01\rangle_L, \quad |10\rangle_L \to |10\rangle_L, \quad |11\rangle_L \to -|11\rangle_L$$

— a CZ gate. The success probability is that of two independent NS gates:

$$P_{CZ} = \left(\tfrac{1}{4}\right)^2 = \boxed{\tfrac{1}{16}},$$

heralded by the four ancilla detectors. On failure, the qubits are destroyed and the attempt must be repeated with fresh photons. Note the elegance of the failure mode: because failure is *heralded*, it is an erasure at a known time and place — a theme running from here through Section 20.5.

## The Postselected Shortcut: CZ with $P = 1/9$

For experiments that only need the gate to have worked *when the photons are eventually detected*, there is a cheaper construction requiring **no ancilla photons at all**. Combine the two "1" rails on a beam splitter of reflectivity $1/3$, and the "0" rails with matching $1/3$ attenuations (so all amplitudes rescale equally); the two-photon amplitude picks up the sign flip from the partial HOM interference at the $\eta = 1/3$ splitter. Conditioned on one photon emerging in each qubit's rail pair — checkable only by detecting the photons — the state has undergone a CZ with probability

$$P = \tfrac{1}{9}.$$

This **postselected** CZ (Ralph et al. and Hofmann & Takeuchi, independently, 2002) cannot be heralded: there is no way to know it worked without measuring, and hence destroying, the output qubits. It therefore cannot be composed into deeper circuits — but it is perfect for few-photon demonstrations, and it powered the first experimental photonic CNOT (O'Brien et al., 2003, bulk optics; process fidelity ~87%) and the first integrated-waveguide quantum gates (Politi et al., 2008, silica-on-silicon — the experiment that launched integrated quantum photonics). The distinction it teaches is fundamental:

- **Heralded** gate: an ancilla measurement announces success *before* the output is used. Composable; scalable in principle.
- **Postselected** gate: success is inferred *from* the output measurement. Fine for demonstrations; a dead end for computation, since the success probabilities multiply across the circuit, $P_{total} = p^{G}$ for $G$ gates — exponential collapse.

## Why $1/16$ Is Still Not Good Enough

A heralded CZ at $P = 1/16$ *is* composable — repeat until success — but only if the rest of the machine can wait. Repetition requires either quantum memory (photons do not wait; fiber delay costs ~0.2 dB/km — Chapter 19's loss budget again) or enormous parallel redundancy. Composed naively, $G$ sequential gates each succeeding with $p = 1/16$ still require $\sim 16$ fresh attempts per gate *with the input state intact*, and a failed attempt destroys the input qubits it touched. Without a further idea, deep circuits remain exponentially expensive.

KLM's further idea — the reason their paper proves *scalability*, not just possibility — is to never gamble with data qubits at all: gamble offline, on ancilla resource states, and apply gates to data only by teleportation. That is the subject of the next subsection, and its intellectual descendants (cluster states, fusion networks) are the architecture of every serious photonic quantum computing effort today.
