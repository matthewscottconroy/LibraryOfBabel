# Chapter 18: Exercises

---

## Mathematical Exercises

**M18.1 — Photon Statistics of the Canonical States**

Compute the second-order coherence $g^{(2)}(0) = \langle\hat{n}(\hat{n}-1)\rangle/\langle\hat{n}\rangle^2$ and the Mandel parameter $Q = \langle(\Delta\hat{n})^2\rangle/\langle\hat{n}\rangle - 1$ for each state.

(a) Coherent state $|\alpha\rangle$ (Poissonian). Show $g^{(2)}(0)=1$ and $Q=0$ for all $\bar{n}$.

(b) Single-mode thermal state, $P_n = \bar{n}^n/(1+\bar{n})^{n+1}$. Show $\langle\hat{n}(\hat{n}-1)\rangle = 2\bar{n}^2$, hence $g^{(2)}(0)=2$ and $Q=\bar{n}$.

(c) Fock state $|n\rangle$. Show $g^{(2)}(0)=1-1/n$ and $Q=-1$; evaluate for $n=1,2,10$ and comment on the approach to the classical value.

(d) Single-mode squeezed vacuum with $\langle\hat{n}\rangle=\sinh^2 r$. Using the even-photon-number structure of Section 17.3.3, show that squeezed vacuum is *super-bunched*, $g^{(2)}(0) = 3 + 1/\sinh^2 r$, and evaluate for a $10$ dB state ($r=1.15$). Why is it noisier than thermal light?

**M18.2 — The Quantum Beam Splitter**

For a lossless beam splitter $\hat{c}=t\hat{a}+r\hat{b}$, $\hat{d}=r\hat{a}+t\hat{b}$:

(a) Impose the bosonic commutators $[\hat{c},\hat{c}^\dagger]=[\hat{d},\hat{d}^\dagger]=1$ and $[\hat{c},\hat{d}^\dagger]=0$ and derive the unitarity constraints $|r|^2+|t|^2=1$ and $r^*t+rt^*=0$.

(b) Verify that $t=1/\sqrt2$, $r=i/\sqrt2$ satisfies both, write the transformation as a $2\times2$ matrix, and confirm it is unitary.

(c) Send $|1,0\rangle$ into the 50/50 splitter. Compute the output state, the single-detector click probabilities, and the coincidence probability.

(d) Show explicitly that discarding the second (vacuum) input port — writing $\hat{c}=\hat{a}/\sqrt2$ — gives $[\hat{c},\hat{c}^\dagger]=1/2$, an unphysical mode. Explain in one sentence why the vacuum port cannot be ignored.

**M18.3 — The Hong-Ou-Mandel Effect**

(a) Starting from $|1,1\rangle=\hat{a}^\dagger\hat{b}^\dagger|0,0\rangle$ and the 50/50 creation-operator map, reproduce the full algebra leading to $\frac{i}{\sqrt2}(|2,0\rangle+|0,2\rangle)$. Identify where the cross term cancels and interpret $t^2+r^2=0$ physically.

(b) Repeat for a general (unbalanced) beam splitter of transmissivity $T=|t|^2$ and reflectivity $R=1-T$. Show that the coincidence probability for a $|1,1\rangle$ input is $P_{\text{coinc}} = (T-R)^2 = (2T-1)^2$, vanishing only at $T=1/2$.

(c) For Gaussian photon spectra of rms bandwidth $\sigma$, the dip is $P_{\text{coinc}}(\tau)=\frac12(1-e^{-\sigma^2\tau^2})$. Given a filter bandwidth $\Delta\nu = 1$ THz, estimate the dip width in optical delay $\tau$ and comment on why detector jitter of $20$ ps does not wash it out.

(d) The observed visibility is $V=M$ ideally, but multi-photon emission caps it near $V_{\max}\approx 1-2g^{(2)}(0)$. For photons with intrinsic indistinguishability $M=0.98$ and $g^{(2)}(0)=0.01$, estimate the measured dip visibility.

**M18.4 — Single-Mode Squeezing and Loss**

(a) From the Bogoliubov transformation $\hat{S}^\dagger(r)\hat{a}\hat{S}(r)=\hat{a}\cosh r - \hat{a}^\dagger\sinh r$ (real $\xi$), derive $\hat{S}^\dagger\hat{X}_1\hat{S}=e^{-r}\hat{X}_1$ and $\hat{S}^\dagger\hat{X}_2\hat{S}=e^{+r}\hat{X}_2$, and confirm $\Delta X_1\Delta X_2=1/4$ for squeezed vacuum.

(b) Convert: what $r$ gives $10$ dB of squeezing? What is $\langle\hat{n}\rangle=\sinh^2 r$ there?

(c) Loss of transmission $\eta$ gives detected variance $\Delta X^2_{\text{out}}=\eta e^{-2r}/4 + (1-\eta)/4$. Derive the asymptotic ceiling $S_{\max}=-10\log_{10}(1-\eta)$ and tabulate it for $\eta = 0.90,\ 0.95,\ 0.99$.

(d) A source generates $12$ dB of squeezing but the detection chain has $\eta=0.92$. What squeezing is actually observed?

**M18.5 — SPDC Pair Statistics and Heralding**

The two-mode squeezed vacuum has pair distribution $P_n=(1-\lambda)\lambda^n$ with $\lambda=\tanh^2 r$ and mean $\mu=\sinh^2 r$.

(a) Show $\lambda = \mu/(1+\mu)$, and that each arm alone is a thermal state.

(b) Write $P_0,P_1,P_2$ and the double-to-single ratio $P_2/P_1=\lambda$. Evaluate for $\mu=0.05$ and interpret the number as multi-pair contamination.

(c) The threshold-heralded purity is $g^{(2)}_h(0)=2\mu/(1+\mu)$. Find the $\mu$ giving $g^{(2)}_h(0)=0.01$; at a $1$ GHz pump rate and heralding efficiency $0.8$, compute the heralded single-photon rate at that $\mu$.

(d) Type-II SPDC yields $\frac{1}{\sqrt2}(|H\rangle_1|V\rangle_2 + e^{i\phi}|V\rangle_1|H\rangle_2)$. Explain how wave plates and a relative-phase adjustment access all four Bell states.

**M18.6 — Squeezed Light in a Gravitational-Wave Detector**

(a) Interferometric phase sensitivity at the standard quantum limit is $\Delta\phi\sim1/\sqrt{N}$. If $N=10^{20}$ photons are detected in an integration time, what shot-noise-limited phase precision results?

(b) Show that $S$ decibels of squeezing improve the amplitude (strain) sensitivity by $10^{S/20}$. Evaluate for $S=3,\,6,\,10$ dB.

(c) Detection range scales with amplitude sensitivity and detected volume as range$^3$. Advanced LIGO's O3 squeezing (up to $3$ dB) realized a band-averaged binary-neutron-star range increase of $\sim15\%$; compute the corresponding detection-rate increase and compare to the measured $40$–$50\%$. What would an idealized full $\sqrt2$ range gain give?

(d) Explain quantitatively why frequency-*independent* phase squeezing worsens low-frequency sensitivity, and what a filter cavity does about it.

---

## Conceptual Exercises

**C18.7 — Why Squeezing Cannot Break Heisenberg**

Squeezed light has $\Delta X_1 = e^{-r}/2 < 1/2$, below the vacuum level. Explain why this does not violate the uncertainty principle. Where does the "removed" noise go, and what conserved product forbids removing it from both quadratures at once? Why is squeezed vacuum still a *minimum*-uncertainty state, and how does this differ from simply having less energy than vacuum?

**C18.8 — Indistinguishability and Gate Fidelity in LOQC**

Photons do not interact, yet linear optical quantum computing needs an entangling gate. Explain how the Hong-Ou-Mandel effect supplies the missing "interaction" and why measurement is essential. Then argue why photon indistinguishability is a make-or-break specification: using $M=e^{-\Delta\omega^2/4\sigma^2}$ and $\varepsilon_{\text{gate}}\sim(1-M)$, quantify the gate error from a $1\%$ (of bandwidth) center-frequency mismatch and from a $20\%$ mismatch, and state the tolerance implied by a $V>99.9\%$ fault-tolerance requirement.

**C18.9 — Antibunching as a Strict Non-Classicality Witness**

State the Cauchy-Schwarz bound $g^{(2)}(0)\ge1$ obeyed by every classical field and explain why it forbids antibunching. What must be true of the Glauber-Sudarshan $P(\alpha)$ representation for $g^{(2)}(0)<1$? Contrast HBT bunching ($g^{(2)}(0)=2$, classical) with single-emitter antibunching ($g^{(2)}(0)\to0$, quantum): which one refutes a classical wave picture of light, and why is it the acceptance test for a single-photon source?

---

## Programming Projects

**P18.1 — Hong-Ou-Mandel Dip Simulation**

Using QuTiP (or a Fock-space linear-optics library), input $|1,1\rangle$ to a 50/50 beam splitter and compute the coincidence probability as a function of temporal delay between the two input photons, modeling each as a Gaussian wavepacket of coherence time $\tau_c$. Reproduce the dip $P_{\text{coinc}}(\tau)=\frac12(1-e^{-\tau^2/\tau_c^2})$, fit a Gaussian to extract $\tau_c$, and then add (i) an unbalanced splitter ($T\neq1/2$), (ii) partial indistinguishability $M<1$, and (iii) a multi-photon admixture with $g^{(2)}(0)=0.02$, plotting how each raises the dip floor and lowers the visibility.

**P18.2 — SPDC Source Characterization via the Joint Spectral Amplitude**

Model a Type-II SPDC source through its joint spectral amplitude (JSA), the product of the pump envelope and the phase-matching sinc function. Compute the JSA for a PPKTP crystal, perform a Schmidt (singular-value) decomposition, and extract the spectral purity $P=\sum_k\lambda_k^2$ and Schmidt number $K=1/P$. Relate the purity to the achievable heralded HOM visibility, then optimize the pump bandwidth for group-velocity-matched (factorable) emission and show that filtering trades brightness for purity.

**P18.3 — Squeezed-Light-Enhanced Interferometry**

Simulate a Mach-Zehnder interferometer at the shot-noise limit, then inject squeezed vacuum into the dark port with the squeezed quadrature aligned to the signal. Compute the phase-sensitivity improvement versus injected squeezing (0–15 dB) and versus optical loss $\eta$, reproducing the ceiling $S_{\max}=-10\log_{10}(1-\eta)$. Apply the model to a LIGO-like scenario: convert a chosen squeezing/loss budget into a binary-neutron-star range gain and an expected detection-rate increase (rate $\propto$ range$^3$), and explore the low-frequency radiation-pressure penalty that motivates frequency-dependent squeezing.
