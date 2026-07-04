# Chapter 22: Exercises

---

## Mathematical Exercises

**M22.1 — BB84 Sifting, QBER, and the 11% Threshold**

Alice sends $N = 10^7$ BB84 pulses; Bob detects $10^6$ (the rest lost); bases match on half of the detected events.

(a) Compute the sifted-key length. After sacrificing $10\%$ of the sifted bits for parameter estimation, how many remain?

(b) Parameter estimation gives QBER $Q = 4\%$. Using the asymptotic secret fraction $r = 1 - 2h(Q)$ with $h(x) = -x\log_2 x - (1-x)\log_2(1-x)$, compute the secret-key length. Repeat for $Q = 8\%$ and $Q = 10.5\%$.

(c) Show that intercept-resend on *every* pulse induces $Q = 25\%$, and locate the $Q$ at which $r \to 0$. Explain why one-way post-processing cannot rescue a key above $11\%$.

**M22.2 — Weak Coherent Pulses and Decoy-State Logic**

An attenuated laser emits coherent states of mean photon number $\mu$; the photon number is Poissonian.

(a) For $\mu = 0.5$, compute $P(0)$, $P(1)$, and $P(n \ge 2)$. What fraction of the *non-empty* pulses are multiphoton?

(b) Write the overall gain as $Q_\mu = \sum_{n} P(n|\mu)\,Y_n$, where $Y_n$ is the yield of an $n$-photon pulse. Explain, using the fact that Eve cannot distinguish a signal photon from a decoy photon, why measuring $Q_\mu$ and $Q_\nu$ at two intensities bounds the single-photon yield $Y_1$ and error $e_1$.

(c) Without decoys, the provably-single-photon rate scales as $O(\eta^2)$; with decoys, $O(\eta)$. At $\eta = 10^{-3}$ (about 150 km of fibre), quantify the advantage decoys buy.

**M22.3 — PLOB Bound and Key Rate versus Distance**

Standard fibre: $\alpha = 0.2$ dB/km, transmission $\eta = 10^{-\alpha L / 10}$.

(a) Tabulate $\eta$ at $L = 50, 100, 200, 400$ km.

(b) The PLOB bound is $K \le -\log_2(1-\eta)$ secret bits per channel use. Compute it at each distance and verify $K \approx 1.44\,\eta$ for small $\eta$.

(c) At a 1 GHz clock, convert to bits/s. At what distance does the PLOB ceiling fall below 1 bit/s? Compare with the 421 km decoy-BB84 fibre record.

**M22.4 — Twin-Field $\sqrt{\eta}$ Crossover**

Twin-field QKD scales as $\sqrt{\eta}$ (one photon over half the path), against PLOB's $\eta$.

(a) Writing the two rates as $R_{\text{PLOB}} = c_1\eta$ and $R_{\text{TF}} = c_2\sqrt{\eta}$, find the transmission $\eta^\ast$ (hence distance $L^\ast$) at which twin-field overtakes PLOB, in terms of $c_2/c_1$.

(b) With a representative $c_2/c_1 = 10^{-2}$ and $\alpha = 0.2$ dB/km, estimate $L^\ast$ in kilometres.

(c) Compute the ratio $R_{\text{TF}}/R_{\text{PLOB}}$ at 600 km. Why must the middle node be *untrusted* rather than trusted, and how does interfering the two fields there keep the key secret from it?

**M22.5 — Entanglement-Swapping Fidelity Down a Chain**

For Werner pairs of fidelity $F$ (target Bell state weight $F$, the other three each $(1-F)/3$), a perfect swap gives $F' = F^2 + \tfrac{1}{3}(1-F)^2$.

(a) Verify $F' \approx F^2$ for $F$ near 1, and compute $F'$ for $F = 0.95$ after one swap.

(b) For a chain of $n$ segments ($n-1$ swaps of equal-fidelity pairs), compute $F_n$ for $F_0 = 0.99$ at $n = 2, 5, 10, 20$. Below which value does a 20-segment chain fall, and is it still above the BB84 threshold fidelity?

(c) One round of purification maps two pairs of fidelity $F$ to $F_{\text{out}} = (F^2 + a^2)/(F^2 + 2Fa + 5a^2)$ with $a = (1-F)/3$. Show $F_{\text{out}} > F$ for $F > 1/2$, and compute the gain at $F = 0.9$.

**M22.6 — Satellite versus Fibre Loss Budget**

A LEO downlink at $\lambda = 810$ nm uses a $D_t = 0.30$ m transmitter and a $D_r = 1.2$ m receiver over slant range $L = 1{,}200$ km.

(a) With diffraction divergence $\theta \approx \lambda / D_t$, compute the beam diameter at the ground and the geometric collection fraction $(D_r / \theta L)^2$, expressed in dB.

(b) Compute the loss of $1{,}200$ km of $0.2$ dB/km fibre. Form the ratio (in dB and as a factor) between fibre loss and the free-space geometric loss.

(c) Ignoring atmosphere, at what fibre length does fibre loss equal the realistic satellite budget of a few tens of dB? Explain why satellites win at continental scale while fibre wins in the metro.

---

## Conceptual Exercises

**C22.7 — Two Faces of No-Cloning**

The no-cloning theorem both *secures* QKD and *forbids* the optical amplifier that would extend it. Explain how a single four-line theorem does both. Connect the optimal universal cloning fidelity ($5/6$) to the QBER a cloning attack induces, and explain why a phase-insensitive amplifier of gain $G$ must add at least $(G-1)/2$ noise photons per quadrature rather than cloning noiselessly.

**C22.8 — Trust Models: Trusted Nodes, MDI, and Entanglement-Based QKD**

For each of (i) a trusted-node relay chain, (ii) MDI-QKD with an untrusted relay, and (iii) satellite entanglement-based BBM92, state precisely *what must be trusted* (source, channel, detectors, relay) and *what may be adversarial*. Rank the three by trust footprint and map each onto a stage of the quantum-internet roadmap.

**C22.9 — Classifying Systems on the Roadmap**

Assign each of the following to a Wehner–Elkouss–Hanson stage and justify: (a) the 2,000 km Beijing–Shanghai backbone with 32 trusted nodes; (b) Micius distributing entangled pairs for BBM92 over 1,120 km; (c) the Delft three-node NV network teleporting across a swap; (d) a metropolitan MDI-QKD ring with an untrusted switch; (e) a hypothetical network of five error-corrected logical qubits sharing distilled Bell pairs. Explain why stage is set by end-node capability, not distance spanned.

---

## Programming Projects

**P22.10 — BB84 QKD Simulator**

Implement full BB84 with an intercept-resend eavesdropper. Alice draws random bits and bases; Eve intercepts a fraction $p_E$ of pulses (measuring in a random basis and resending); Bob measures in a random basis.

(a) Over $\ge 10^6$ pulses, compute the sifted-key rate and measured QBER as functions of $p_E$, verifying $Q \approx 25\% \times p_E$.

(b) Apply $r = 1 - 2h(Q)$ and plot the secure-key fraction versus $p_E$, locating the abort point.

(c) Add channel loss ($0.2$ dB/km) and detector dark counts, and plot the secure key *rate* versus distance; identify the distance at which it hits zero.

**P22.11 — Entanglement-Swapping / Repeater-Chain Simulator**

Model an $n$-segment repeater chain of Werner pairs with initial fidelity $F_0$.

(a) Implement one swap, $F' = F_1F_2 + (1-F_1)(1-F_2)/3$, and reproduce $F_n$ versus $n$ for $F_0 = 0.99$.

(b) Add depolarizing memory noise that decays fidelity with dwell time $\tau$ at rate $\Gamma$, and study $F_n$ versus storage time — connecting to the memory lifetime requirement of Section 22.2.2.

(c) Insert DEJMPS purification between swaps and find, for each $F_0$, the maximum chain length delivering $F_n > 0.9$.

**P22.12 — Rate–Distance Landscape: PLOB, Decoy-BB84, and Twin-Field**

Numerically compute and plot, versus distance $0$–$1{,}000$ km at $0.2$ dB/km: (a) the PLOB bound $-\log_2(1-\eta)$; (b) a decoy-state BB84 rate (a simple $R \propto \eta$ model with detector dark counts producing a cutoff near 400–500 km); (c) a twin-field rate $\propto \sqrt{\eta}$. Mark the distance at which twin-field crosses PLOB and annotate the 421 km (decoy) and 1,002 km (twin-field) fibre records. Discuss which distance regime each protocol owns.
