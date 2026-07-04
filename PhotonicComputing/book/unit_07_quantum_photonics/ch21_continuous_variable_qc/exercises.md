# Exercises — Chapter 21: Continuous-Variable and Xanadu's Quantum Computing

## Mathematical Exercises

**21.1** (Quadrature Algebra)
Starting from $[\hat{a}, \hat{a}^\dagger] = 1$ and the definitions $\hat{x} = (\hat{a}+\hat{a}^\dagger)/\sqrt{2}$, $\hat{p} = (\hat{a}-\hat{a}^\dagger)/i\sqrt{2}$:

(a) Verify $[\hat{x}, \hat{p}] = i$.

(b) Show that the vacuum satisfies $\langle 0|\hat{x}^2|0\rangle = \langle 0|\hat{p}^2|0\rangle = 1/2$.

(c) For the coherent state $|\alpha\rangle$ with $\alpha = 2 + i$, compute $\langle\hat{x}\rangle$, $\langle\hat{p}\rangle$, $\Delta x$, and $\Delta p$.

(d) Show that the rotated quadrature $\hat{x}_\theta = \hat{x}\cos\theta + \hat{p}\sin\theta$ satisfies $[\hat{x}_\theta, \hat{x}_{\theta+\pi/2}] = i$ for every $\theta$.

---

**21.2** (Squeezing in Decibels)
A squeezed vacuum state has quadrature variances $(\Delta x)^2 = e^{-2r}/2$ and $(\Delta p)^2 = e^{+2r}/2$.

(a) Derive the relation $S_\text{dB} = (20/\ln 10)\, r \approx 8.69\, r$ and compute $r$ for the 15 dB world record.

(b) Compute the mean photon number $\langle \hat{n}\rangle = \sinh^2 r$ of 15 dB squeezed vacuum.

(c) A squeezed beam with ideal squeezing $S_\text{dB} \to \infty$ passes through a channel with power transmission $\eta$. Show that the measured squeezed variance is bounded by $(1-\eta)/2$, and hence that $\eta = 0.95$ caps observable squeezing at $\approx 13$ dB. What transmission is needed to observe 20 dB?

---

**21.3** (Beam Splitter as a Symplectic Rotation)
A beam splitter of power reflectivity $R$ (transmissivity $\tau = 1-R$) acts on two modes as $\hat{a} \to \sqrt{\tau}\,\hat{a} + \sqrt{R}\,\hat{b}$, $\hat{b} \to -\sqrt{R}\,\hat{a} + \sqrt{\tau}\,\hat{b}$.

(a) Write the corresponding $4\times 4$ matrix $S$ acting on $(\hat{x}_a, \hat{p}_a, \hat{x}_b, \hat{p}_b)^T$.

(b) Verify explicitly that $S\,\Omega\,S^T = \Omega$, confirming that the beam splitter is symplectic (indeed orthogonal).

(c) A 50:50 beam splitter combines two identical single-mode squeezed vacua, one squeezed in $x$ and one in $p$. Compute the output covariance matrix and show the outputs form a two-mode squeezed (EPR-entangled) state with $\text{Var}(\hat{x}_1 - \hat{x}_2) = \text{Var}(\hat{p}_1 + \hat{p}_2) = e^{-2r}$.

---

**21.4** (Heterodyne Noise Penalty)
Model heterodyne detection as splitting the signal on a 50:50 beam splitter (vacuum entering the other port) and homodyning $\hat{x}$ on one output and $\hat{p}$ on the other.

(a) Show each inferred quadrature (rescaled by $\sqrt{2}$) carries variance $(\Delta x)^2_\text{signal} + 1/2$: exactly one extra vacuum unit.

(b) Explain why simultaneous noiseless measurement of $\hat{x}$ and $\hat{p}$ would violate the uncertainty principle.

---

**21.5** (Gaussian Simulability by Hand)
A single-mode squeezed vacuum ($r = 1$, squeezed in $x$) passes through a phase rotation of $45°$ and is then measured by homodyne detection of $\hat{x}$.

(a) Propagate the covariance matrix through the rotation and give the outcome variance.

(b) Write down the (Gaussian) probability density of homodyne outcomes.

(c) Explain, in one paragraph, why this covariance-propagation procedure scales polynomially in the number of modes, and precisely which circuit elements would break it.

---

**21.6** (GKP Logical Error Probability)
A GKP qubit's $x$ quadrature suffers Gaussian displacement noise of standard deviation $\sigma$ per error-correction round; the decoder rounds to the nearest lattice point of spacing $\sqrt{\pi}$.

(a) Show the leading-order logical flip probability is $p \approx \text{erfc}\!\left(\sqrt{\pi}/(2\sqrt{2}\sigma)\right)$.

(b) Evaluate $p$ for noise 6 dB, 10 dB, and 13 dB below the vacuum variance of $1/2$.

(c) For a computation of $10^9$ error-correction rounds targeting a total failure budget of 1%, what squeezing (in dB) is required at this level of modeling?

---

**21.7** (Universal CV Gate Set)
Consider the gate generators $\hat{x}$ (displacement), $\hat{x}^2$ and $\hat{x}\hat{p}+\hat{p}\hat{x}$ (squeezing/shear), $\hat{x}^2+\hat{p}^2$ (rotation), and the cubic generator $\hat{x}^3$.

(a) Using $[\hat{x},\hat{p}] = i$, show that $[\hat{x}^3, \hat{p}^2] = 3i\,(\hat{p}\hat{x}^2 + \hat{x}^2\hat{p})$ — a degree-3 polynomial with *mixed* $x$-$p$ terms that no quadratic Hamiltonian can produce.

(b) Argue (following Lloyd & Braunstein) that quadratic generators alone close under commutation — hence generate only Gaussian operations — while adding any single cubic generator allows commutators to reach polynomials of arbitrary degree, giving universality over CV.

(c) Where, physically, does Xanadu's architecture obtain its non-Gaussian element, given that a strong deterministic cubic phase gate does not yet exist?

---

**21.8** (Time-Domain Multiplexing Arithmetic)
Borealis emits squeezed pulses every 167 ns and routes them through delay loops of $1\tau$, $6\tau$, and $36\tau$.

(a) Verify that 216 pulses span $\approx 36\ \mu$s, and explain the significance of $216 = 6^3$ for the three-loop coupling ranges.

(b) What physical length of standard fiber ($n_g \approx 1.47$) realizes the $36\tau$ loop?

(c) If each loop round trip imposes 1% loss, estimate the transmission of a pulse that traverses all three loops once, and comment on why loss compounding constrains loop-based architectures.

## Programming Projects

**Project 21.1: GBS for Maximum Clique (Strawberry Fields / The Walrus).**
Encode a random 12-node graph (edge probability 0.5, plus one planted 6-clique) into a GBS distribution. Sample (simulator), post-select samples onto subgraphs, and greedily shrink/expand them into cliques. Compare the clique sizes found per 1,000 samples against uniform random subgraph sampling and against classical brute force. Report how performance degrades when you replace GBS samples with samples from a classical model matching only the single-mode photon statistics.

**Project 21.2: Train a CV Quantum Neural Network (PennyLane).**
Build a one-mode CV-QNN (layers of rotation–squeezing–rotation–displacement–Kerr) on the Strawberry Fields Fock backend (cutoff $\geq 10$) and train it by parameter-shift gradient descent to fit $f(x) = \sin(2\pi x)$ on $[-1, 1]$, encoding $x$ by displacement and reading out $\langle\hat{x}\rangle$. Plot the fit and the loss curve; then retrain with the Kerr gates removed and explain the change in expressivity in terms of the Gaussian simulability theorem.

## Thought Experiments

**21.9** (Where Does the Quantumness Live?)
A colleague claims: "Squeezed states are the resource powering Borealis's quantum advantage." Another counters: "No — squeezed states plus beam splitters plus homodyne are classically simulable; the advantage comes entirely from the photon-number-resolving detectors." Adjudicate, carefully separating the roles of the Gaussian state's hafnian structure and the non-Gaussian measurement. What would Borealis compute if its TES detectors were swapped for homodyne receivers?

**21.10** (Analog Noise versus Digital Loss)
DV photonic computing fails by *losing photons* (erasures, flagged); CV computing fails by *finite squeezing* (small Gaussian noise in every operation, unflagged). For which failure model is error correction conceptually easier? Which is easier to *diagnose* in hardware? Use the GKP code's conversion of analog noise into digital syndromes to argue that the distinction is ultimately engineered away — and identify what physical price is paid for that conversion.
