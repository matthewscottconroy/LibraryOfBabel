# 17.2.3 Fock States

## The Number Basis

The eigenstates of the number operator,

$$|n\rangle = \frac{(\hat{a}^\dagger)^n}{\sqrt{n!}}\,|0\rangle, \qquad \hat{n}|n\rangle = n|n\rangle, \qquad \langle m|n\rangle = \delta_{mn}$$

are the **Fock states** (Vladimir Fock introduced the construction in 1932 for many-particle systems). They are complete — $\sum_{n=0}^\infty |n\rangle\langle n| = \mathbb{1}$ — so any state of the oscillator, hence any state of a light mode, can be expanded as $|\psi\rangle = \sum_n c_n |n\rangle$. The Fock basis is to quantum optics what the plane-wave basis is to Fourier optics: the universal reference frame in which all other states (coherent, squeezed, thermal, cat) are written.

As energy eigenstates, Fock states are **stationary**: under free evolution $|n\rangle \to e^{-i(n+1/2)\omega t}|n\rangle$, a global phase. A Fock state never moves. Its photon number is perfectly sharp:

$$\langle\hat{n}\rangle = n, \qquad \langle(\Delta \hat{n})^2\rangle = 0$$

A photon-number-resolving detector aimed at an ideal $|n\rangle$ reads exactly $n$, every trial.

## Sharp Number, Random Phase

The price of definite photon number is total phase indeterminacy. From 17.2.2, $\langle n|\hat{a}|n\rangle = 0$, so the mean field vanishes:

$$\langle n|\hat{E}|n\rangle = 0 \quad\text{for every } n$$

while the field *variance* $\langle \hat{E}^2\rangle \propto (2n+1)$ grows with $n$. A Fock state is not a feeble version of a classical wave; it is energy with no phasor attached — a distribution spread uniformly over all phases of the phase-space circle of radius $\sim\sqrt{n}$. This is the extreme opposite of laser light, whose phase is well defined at the expense of photon-number fluctuations (Section 17.3.2). The two regimes bracket a **number-phase uncertainty** tradeoff, heuristically $\Delta n\,\Delta\phi \gtrsim 1/2$ — heuristic because no strictly self-adjoint phase operator exists, but quantitatively reliable for the states of this unit.

Sub-Poissonian statistics make Fock states non-classical in a sharp, testable sense. The **Mandel parameter**

$$Q = \frac{\langle(\Delta\hat{n})^2\rangle - \langle\hat{n}\rangle}{\langle\hat{n}\rangle}$$

is $0$ for coherent (Poissonian) light, positive for thermal light, and $Q = -1$ for any Fock state — the most negative value possible. No classical intensity distribution can produce $Q < 0$ (Section 18.1 proves the equivalent statement $g^{(2)}(0) \geq 1$ for classical fields, while $|n\rangle$ has $g^{(2)}(0) = 1 - 1/n < 1$). Equivalently, Section 17.3.4 will show the Wigner function of any Fock state with $n \geq 1$ takes *negative values* — the unambiguous phase-space signature that no classical statistical model underlies the state.

## The Single Photon, $|1\rangle$

The Fock state $|1\rangle$ is the star of this unit: the qubit carrier of Chapters 19–20 and 22. Three of its properties do the work:

1. **Indivisibility.** A beam splitter cannot split it: the output is a *superposition* of "photon in port $c$" and "photon in port $d$", never half a photon in each (Section 18.2.1). Detected coincidences between the two outputs vanish — the Grangier-Roger-Aspect anticorrelation experiment (1986), and the operational meaning of $g^{(2)}(0) = 0$.
2. **Perfect antibunching.** One photon cannot trigger two detections; $|1\rangle$ has $g^{(2)}(0) = 0$, the certificate demanded of single-photon sources (Sections 18.1.3, 19.1).
3. **Interference with itself.** Split across two modes, $|1\rangle$ becomes the dual-rail qubit $(\alpha|1,0\rangle + \beta|0,1\rangle)$, and every MZI acts on it as a single-qubit gate — full quantum coherence with zero risk of multi-photon errors.

**Worked example (energy density of one photon).** A single 1550-nm photon carries $\hbar\omega = 1.28\times 10^{-19}$ J $= 0.80$ eV. Confined to a silicon microring of mode volume $V \approx 5\ \mu\text{m}^3$, its energy density corresponds to a root-mean-square field $E_{\text{rms}} = \sqrt{\hbar\omega/(2\varepsilon_0 V)} \approx 3.8\times 10^4$ V/m — tens of kilovolts per meter from *one quantum*. Tight confinement is what makes single photons able to drive appreciable physics (and, in cavity QED, to saturate a single atom); in free space the same photon spread over a millimeter beam is fantastically dilute. Mode volume, a classical design parameter from Unit III, directly sets the strength of single-photon light-matter interaction.

## Why Fock States Are Hard to Make

Nothing in ordinary light generation produces Fock states. A laser emits coherent states; a lamp emits thermal states; attenuating either changes the mean photon number but *never* the statistics class — a dim laser pulse with $\bar{n} = 0.1$ is still Poissonian, containing two photons in about $0.5\%$ of the pulses that contain any. Since attenuation and any other *linear, Gaussian* operation preserves classicality, genuine Fock states require either:

- **A single quantum emitter** — one atom, ion, quantum dot, or color center, which by construction holds only one excitation and emits photons one at a time (Chapter 19); or
- **Measurement-induced preparation** — generate correlated pairs (SPDC, Section 18.3.1) and *herald*: detecting the idler photon projects, via the Born rule, the signal mode into $|1\rangle$. The nonlinearity is supplied by the detector, a strategy that returns at full scale in the KLM protocol (Chapter 20); or
- **Strong nonlinearity at the few-photon level** — photon blockade in cavity QED (Chapter 19), where one photon shifts the resonance enough to exclude a second.

All three routes are probabilistic, lossy, or technologically demanding, and higher-$n$ Fock states are harder still ($|n\rangle$ for $n \gtrsim 3$ remains a laboratory event, not a resource). The scarcity is fundamental to the field's economics: single photons are the *currency* of discrete-variable photonic quantum computing, and Chapters 19–20 are largely about minting, storing, and not losing them.
