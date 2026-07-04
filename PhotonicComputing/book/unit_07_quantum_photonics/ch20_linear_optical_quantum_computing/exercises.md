# Chapter 20: Exercises

---

## Mathematical Exercises

**M20.1 — Every Single-Qubit Gate Is a Mach-Zehnder Interferometer**

A dual-rail qubit $(\alpha,\beta)$ occupies two modes. Build gates from the phase shifter $P(\phi) = \operatorname{diag}(1, e^{i\phi})$ and the 50/50 beam splitter $B(\pi/4) = \tfrac{1}{\sqrt2}\begin{pmatrix} 1 & i \\ i & 1\end{pmatrix}$.

(a) Multiply out $U_{\text{MZI}}(\phi,\varphi) = P(\varphi)\,B(\pi/4)\,P(\phi)\,B(\pi/4)$ and show that, up to a global phase, it equals $\begin{pmatrix} \sin(\phi/2) & \cos(\phi/2) \\ e^{i\varphi}\cos(\phi/2) & -e^{i\varphi}\sin(\phi/2)\end{pmatrix}$.

(b) Find $(\phi,\varphi)$ realizing the Hadamard gate and the Pauli $Z$ gate. Explain why $X$ is instead implemented as a physical waveguide crossing rather than by tuning phases.

(c) Show that as $(\phi,\varphi)$ range over $[0,2\pi)^2$ the construction covers all of SU(2) up to global phase. Why does this mean single-qubit photonic logic is "free," and what physical imperfection (not fidelity) nonetheless makes even single-qubit elements costly downstream?

**M20.2 — KLM Success Probabilities and Teleportation Boosting**

(a) The nonlinear-sign (NS) gate succeeds with probability $1/4$. Using the two-NS-gate construction of Section 20.2.2, confirm the heralded CZ succeeds with probability $(1/4)^2 = 1/16$, and explain why the *postselected* CZ achieves $1/9$ without ancillas yet cannot be composed into deep circuits.

(b) In boosted teleportation with the $2n$-mode ancilla $|t_n\rangle$, a single teleported qubit fails only for detector outcomes $k=0$ or $k=n+1$, giving $P_{\text{fail}} = 1/(n+1)$. Show that a CZ teleported through two such ancillas succeeds with probability $\big(n/(n+1)\big)^2$, and evaluate it for $n = 1, 9, 99$.

(c) To apply $G$ sequential gates each with success probability $q = \big(n/(n+1)\big)^2$ and to keep the total failure probability below $\epsilon$, estimate the ancilla size $n$ required as a function of $G$ and $\epsilon$. Comment on why this is polynomial but "brutally expensive."

**M20.3 — Cluster-State Stabilizers and Measurement Patterns**

Consider the five-qubit "cross" graph: a central vertex $c$ bonded to four leaves $\{1,2,3,4\}$.

(a) Write the five stabilizer generators $K_v = X_v \prod_{w\in N(v)} Z_w$. Verify any two of them commute. Which single-qubit graph state is this cross locally equivalent to, and why?

(b) A $Z$-measurement on the central qubit $c$ yields outcome $+1$. Describe the resulting graph and state on the four leaves. Repeat for a $Z$-measurement on a single leaf.

(c) For the two-qubit rotation gadget of Section 20.3.2, verify by direct computation that measuring the input qubit in the $|{\pm}\theta\rangle$ basis leaves $X^{s} H R_z(-\theta)|\psi\rangle$ on the output, with $s\in\{0,1\}$ the outcome. Then show that a byproduct $X^s$ preceding the next rotation forces the adaptive rule $\theta \to (-1)^{s}\theta$, using $R_z(\theta)X = XR_z(-\theta)$.

**M20.4 — The Permanent, the Determinant, and Counting Outputs**

(a) Write out $\operatorname{Perm}(A)$ for a generic $3\times 3$ matrix (all six terms) and contrast with $\det(A)$. Estimate the number of arithmetic operations to compute an $n\times n$ permanent by brute force, and state the improvement from Ryser's algorithm.

(b) For $n$ photons in $m$ modes, derive the number of output occupation patterns (multisets) $\binom{m+n-1}{n}$ and the number of collision-free patterns $\binom{m}{n}$. Evaluate both for $n=3, m=9$ and for $n=5, m=25$.

(c) Show that the probability that some pair of $n$ photons collide scales as $\sim \binom{n}{2}/m$, and explain quantitatively why the Aaronson-Arkhipov hardness argument requires $m \gg n^2$.

**M20.5 — Hafnians and Squeezed-Light Statistics**

(a) Enumerate the perfect matchings of $\{1,2,3,4\}$ and write $\operatorname{Haf}$ of a $4\times 4$ symmetric matrix. Repeat the *count* for a $6\times 6$ matrix, and state the general number of matchings $(2n-1)!!$ for $2n$ objects. What is the classical cost of the best Hafnian algorithm, and why does GBS advantage appear around $50$–$100$ detected photons?

(b) Verify $\operatorname{Perm}(B) = \operatorname{Haf}\!\left(\begin{smallmatrix} 0 & B \\ B^{\mathsf T} & 0\end{smallmatrix}\right)$ for a $2\times 2$ matrix $B$.

(c) A single-mode squeezed vacuum has $\langle \hat n\rangle = \sinh^2 r$ and support only on even photon numbers. Evaluate $\langle \hat n\rangle$ at $r=1.0$ and estimate the total detected photon number when $50$ such modes feed an interferometer. Why does threshold ("click") detection replace the Hafnian with the Torontonian?

**M20.6 — Loss Survival and Erasure Thresholds**

A dual-rail photon traverses $N$ components, each of transmission $\eta_c$.

(a) Tabulate the survival probability $\eta_c^N$ for $N = 20, 100, 500$ at $\eta_c = 0.99$ and $0.999$. Convert $\eta_c = 0.999$ to decibels.

(b) A fusion network tolerates total photon loss up to $10\%$. Over a path of $N=50$ components, find the maximum permissible per-component loss in both fractional and dB terms.

(c) Explain, using the distance-$d$ relations "corrects $d-1$ erasures" versus "corrects $\lfloor (d-1)/2\rfloor$ Pauli errors," why erasure thresholds ($\sim 50\%$ on the surface code) so exceed Pauli thresholds ($\sim 11\%$). Using $p_L \approx (p/p_{\text{th}})^{(d+1)/2}$ with $p=10^{-3}$, $p_{\text{th}}=10^{-2}$, find the distance $d$ needed for $p_L = 10^{-15}$ and the resulting physical-qubit count per logical qubit.

---

## Conceptual Exercises

**C20.7 — Heralded, Postselected, and Failed-as-Erasure**

Distinguish a *heralded* gate (an ancilla measurement announces success before the output is used) from a *postselected* gate (success is inferred from the output measurement). Why can the $P=1/16$ heralded CZ be composed into deep circuits while the $P=1/9$ postselected CZ cannot? Explain why "failure is a $Z$-measurement at a known location" is the single property that ties KLM gate teleportation, fusion failure, and photon loss into one error model, and why that property is worth more to a code than a higher success probability would be.

**C20.8 — Why Measurement-Based Computing Suits Photons**

Give three physical reasons the one-way model fits photonics better than the circuit model does. In your answer, address: (i) why destructive measurement is an asset rather than a liability here; (ii) why probabilistic entangling operations are tolerable at the resource-preparation stage but not on data qubits; and (iii) which measurements must be adaptive (feed-forward) and which may be performed in any order, and what physically enforces that partial time-ordering.

**C20.9 — What Boson Sampling Does and Does Not Establish**

Boson sampling is not universal, carries no error correction, and solves no useful problem. State precisely what a successful demonstration *does* establish, and what it does not. In your answer: explain the role of the polynomial-hierarchy-collapse argument and the two conjectures (permanent-of-Gaussians, anti-concentration) it rests on; explain the *validation problem* (why the same hardness that implies advantage also frustrates verification); and explain why fermion sampling is classically easy while boson sampling is not, in terms of determinants versus permanents.

---

## Programming Projects

**P20.10 — Linear-Optical CZ Gate Simulation**

Implement the KLM nondeterministic CZ (or its NS-gate primitive) in a Fock-space simulator. Represent the dual-rail qubits plus ancilla modes, apply the beam-splitter and phase-shifter network as mode transformations, and post-select on the heralding pattern. Run $10^4$ trials with randomized measurement outcomes; verify that heralded-success events (rate $\approx 1/16$ for the full CZ, $1/4$ for NS) apply the correct unitary to the logical subspace, and that failures act as located $Z$-measurements. Plot the empirical success rate against the theoretical value.

**P20.11 — Boson-Sampling Simulator with Ryser's Algorithm**

Build a classical boson sampler for $n \le 20$. Draw a Haar-random $m\times m$ unitary ($m = O(n^2)$), and sample output patterns whose probabilities are $|\operatorname{Perm}(U_{S,T})|^2/\prod_j s_j!$, computing permanents with Ryser's $O(2^n n)$ formula. Measure and plot wall-clock time versus $n$ to expose the exponential wall; verify that the sampled two-photon correlations exceed the distinguishable-particle (classical) baseline, reproducing the bosonic-bunching signature used for experimental validation.

**P20.12 — Gaussian Boson Sampling and the Hafnian**

Using a photonic simulation library (e.g. Strawberry Fields) or a direct Hafnian implementation, set up a GBS circuit on $N=10$ modes with squeezing $r=1.0$ and a random interferometer. Sample $10^3$ output patterns, and verify that pattern probabilities match $|\operatorname{Haf}(A_S)|^2$ for the appropriate submatrix. Confirm the even-photon-number structure of single-mode squeezed inputs, and compare threshold-detector statistics (Torontonian) against photon-number-resolved statistics (Hafnian).

**P20.13 — Cluster-State One-Way Computation**

Write an MBQC simulator. (a) Generate a 1D cluster of $N=8$ qubits by applying $CZ$s to $|+\rangle^{\otimes 8}$; verify the stabilizers $K_v$. (b) Implement a logical Hadamard by a sequence of equatorial measurements, tracking Pauli byproducts and applying the adaptive angle rule. (c) Realize quantum teleportation as an MBQC pattern. (d) Verify output-state fidelity against the intended unitaries, and quantify how a single simulated photon loss (a forced $Z$-measurement) propagates as an erasure through the pattern.
