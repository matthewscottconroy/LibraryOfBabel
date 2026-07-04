# Chapter 20: Important Concepts

---

## 1. The Dual-Rail Qubit and Interferometric Single-Qubit Gates

A qubit is one photon shared between two modes: $|0\rangle_L = |1,0\rangle$, $|1\rangle_L = |0,1\rangle$. Because the state is $(\alpha\,\hat a^\dagger + \beta\,\hat b^\dagger)|\text{vac}\rangle$, a mode unitary $U\in\mathrm{U}(2)$ acts *directly* on the amplitudes $(\alpha,\beta)$ — the mode transformation *is* the qubit gate. Every SU(2) rotation is a Mach-Zehnder interferometer, $U_{\text{MZI}}(\phi,\varphi) = P(\varphi)B(\tfrac{\pi}{4})P(\phi)B(\tfrac{\pi}{4})$: two phases and two 50/50 couplers, at process fidelities of $99.9\%$+. Single-qubit logic in photonics is inherited wholesale from classical interferometer design.

## 2. The No-Interaction Problem

Linear optics transforms mode operators linearly; each photon evolves indifferent to the others. A CNOT requires the opposite — one photon's state must condition another's — so no linear-optical circuit can implement a deterministic two-qubit gate. Every difficulty in the chapter concentrates at this single point: making two photons "talk." The escape is that **photodetection is not linear**; measurement supplies the missing nonlinearity.

## 3. Measurement-Induced Nonlinearity: The NS Gate

The nonlinear-sign gate flips the sign of the two-photon amplitude in a mode, $\alpha_0|0\rangle + \alpha_1|1\rangle + \alpha_2|2\rangle \to \alpha_0|0\rangle + \alpha_1|1\rangle - \alpha_2|2\rangle$ — impossible unitarily, but achievable by interfering with ancilla photons and post-selecting on a heralding detector pattern, with success probability $1/4$ (Knill, Laflamme & Milburn, 2001). This heralded nonlinearity is the primitive from which all linear-optical entangling gates are built.

## 4. Nondeterministic CZ and Gate Teleportation

Two NS gates wrapped around a Hong-Ou-Mandel interference on the "1" rails give a heralded CZ succeeding with probability $(1/4)^2 = 1/16$; an ancilla-free postselected variant reaches $1/9$ but cannot be composed. Scalability comes from **gate teleportation** (Gottesman & Chuang, 1999): gamble offline on resource states, then teleport data through them. Boosted teleportation with a $2n$-mode ancilla succeeds with probability $n/(n+1)$ per qubit, approaching determinism at polynomial cost.

## 5. The Linear-Optics Bell-Measurement Bound

Teleportation needs a Bell-state measurement, but with linear optics and photon counting only **two of the four** Bell states are distinguishable, capping success at $P_{\text{BSM}} = 1/2$ (Calsamiglia & Lütkenhaus, 2001). Ancilla photons boost this — an ancillary Bell pair reaches $3/4$ — and iterating approaches one. This $1/2$ bottleneck reappears as the type-II fusion success probability and sets the percolation problem of FBQC.

## 6. Cluster States and the One-Way Model

A graph state is $|G\rangle = \prod_{(u,v)\in E} CZ_{uv}\,|+\rangle^{\otimes|V|}$, the unique $+1$ eigenstate of the stabilizers $K_v = X_v\prod_{w\in N(v)} Z_w$. The one-way computer (Raussendorf & Briegel, 2001) consumes such a state by adaptive single-qubit measurements: measuring in $|{\pm}\theta\rangle = (|0\rangle \pm e^{i\theta}|1\rangle)/\sqrt2$ teleports the qubit one bond over as $X^s H R_z(-\theta)$, with feed-forward correcting the Pauli byproduct. A 2D cluster is universal.

## 7. Fusion Gates and FBQC

Fusion gates (Browne & Rudolph, 2005) weld small resource states into large graphs; type-II fusion succeeds at $1/2$, boostable higher. Fusion-based quantum computation (Bartolucci et al., 2023) makes constant-size resource states plus fusion measurements the *entire* machine, with cluster growth a **percolation** problem: above the lattice threshold a spanning cluster forms ballistically (Gimeno-Segovia et al., 2015). FBQC tolerates a few-percent-to-$10\%$ photon loss and slashed KLM's overhead by orders of magnitude.

## 8. Boson Sampling and the Permanent

Inject $n$ photons into a Haar-random $m$-mode interferometer ($m\gg n^2$) and sample the output. The amplitude is $\operatorname{Perm}(U_{S,T})/\sqrt{\prod s_j!\prod t_k!}$, and the permanent is #P-hard (Valiant, 1979) — unlike the determinant of fermionic scattering, computable in $O(n^3)$. Efficient classical sampling would collapse the polynomial hierarchy (Aaronson & Arkhipov, 2011). Not universal, not error-corrected: a minimal witness that quantum dynamics outruns classical simulation.

## 9. Gaussian Boson Sampling and the Hafnian

Replace Fock inputs with deterministically squeezed vacuum (Hamilton et al., 2017). Output probabilities become $\propto|\operatorname{Haf}(A_S)|^2$, the Hafnian counting perfect matchings — also #P-hard, with classical cost $\sim 2^{N/2}$ in detected photon number $N$. Threshold detectors give the Torontonian. GBS is far easier experimentally (no on-demand single photons) and its perfect-matching structure connects to graph problems and molecular vibronic spectra.

## 10. Photon Loss as Erasure

Loss is a beam splitter to the environment, $\hat a \to \sqrt\eta\,\hat a + \sqrt{1-\eta}\,\hat b_{\text{env}}$. In dual rail, both $|0\rangle_L$ and $|1\rangle_L$ carry exactly one photon, so a loss leaves vacuum — a *located* (heralded) **erasure**, not a Pauli error. A distance-$d$ code corrects $d-1$ erasures but only $\lfloor(d-1)/2\rfloor$ Pauli errors; erasure thresholds ($\sim 50\%$ on the surface code) far exceed Pauli thresholds ($\sim 11\%$). Photonics' dominant error is the easy one.

## 11. Topological Codes on Photonic Graph States

Foliating a surface code into a 3D cluster (Raussendorf, Harrington & Goyal, 2007) makes the measurements that *run* a computation also *correct* it, in $(2{+}1)$ dimensions. Tree encodings (Varnava, Browne & Rudolph, 2006) infer lost photons from survivors, approaching the $50\%$ loss ceiling. Below threshold the logical error falls as $(p/p_{\text{th}})^{(d+1)/2}$, so distance $d\sim 30$ buys algorithm-scale reliability at $\sim d^2$ physical qubits each — the arithmetic behind million-qubit foundry photonics.
