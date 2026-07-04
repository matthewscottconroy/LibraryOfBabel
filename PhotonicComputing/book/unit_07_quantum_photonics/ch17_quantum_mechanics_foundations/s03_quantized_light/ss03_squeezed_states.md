# 17.3.3 Squeezed States

## Reshaping Vacuum Noise

The uncertainty relation $\Delta X_1 \Delta X_2 \geq 1/4$ constrains the *product* of quadrature noises, not either factor alone. Vacuum and coherent states split the noise evenly ($\Delta X_1 = \Delta X_2 = 1/2$). A **squeezed state** trades: noise in one quadrature below the vacuum level, at the cost of correspondingly more in the conjugate quadrature. In phase space, the vacuum's circular fuzz-ball becomes an ellipse of the same minimum area.

The generator is the unitary **squeezing operator**

$$\hat{S}(\xi) = \exp\left[\frac{1}{2}\left(\xi^*\hat{a}^2 - \xi\,\hat{a}^{\dagger 2}\right)\right], \qquad \xi = r\,e^{i\theta}$$

with **squeezing parameter** $r \geq 0$ and squeezing angle $\theta$ (orientation of the ellipse). The tell-tale structure is the $\hat{a}^{\dagger 2}$ term: squeezing adds and removes photons *in pairs* — which is precisely what a parametric amplifier does when a pump photon converts into two signal photons (Section 18.3.1 supplies the hardware; here we develop the state).

## Quadrature Transformation

In the Heisenberg picture, $\hat{S}$ effects a **Bogoliubov transformation** (take $\theta = 0$):

$$\hat{S}^\dagger(r)\,\hat{a}\,\hat{S}(r) = \hat{a}\cosh r - \hat{a}^\dagger \sinh r$$

which for the quadratures diagonalizes beautifully:

$$\hat{S}^\dagger(r)\,\hat{X}_1\,\hat{S}(r) = e^{-r}\,\hat{X}_1, \qquad \hat{S}^\dagger(r)\,\hat{X}_2\,\hat{S}(r) = e^{+r}\,\hat{X}_2$$

Applied to vacuum, the **squeezed vacuum** $|\xi\rangle = \hat{S}(\xi)|0\rangle$ therefore has

$$\Delta X_1 = \frac{e^{-r}}{2}, \qquad \Delta X_2 = \frac{e^{+r}}{2}, \qquad \Delta X_1\,\Delta X_2 = \frac{1}{4}$$

still a minimum-uncertainty state — the Heisenberg bound is *saturated*, never beaten. Squeezing is noise relocation, not noise deletion. The standard logarithmic measure compares the squeezed variance to vacuum:

$$\text{squeezing in dB} = -10\log_{10}\!\left(\frac{\Delta X_1^2}{(1/2)^2}\right) = -10\log_{10} e^{-2r} \approx 8.686\, r$$

**Benchmarks:** 3 dB means the noise power halved ($r = 0.35$); 10 dB, a factor 10 ($r = 1.15$); the world record for directly measured optical squeezing is **15 dB** ($r \approx 1.73$), achieved by Vahlbruch, Mehmet, Danzmann, and Schnabel at the Albert Einstein Institute, Hannover (2016), at 1064 nm.

## What a Squeezed Vacuum Contains

Squeezed vacuum is not empty. Expanding $|\xi\rangle$ in the Fock basis (the pair structure of $\hat{a}^{\dagger 2}$ guarantees only *even* photon numbers appear):

$$|\xi\rangle = \frac{1}{\sqrt{\cosh r}}\sum_{n=0}^{\infty}\left(-e^{i\theta}\tanh r\right)^n \frac{\sqrt{(2n)!}}{2^n\, n!}\;|2n\rangle$$

with mean photon number

$$\langle\hat{n}\rangle = \sinh^2 r$$

**Worked example.** A 10 dB squeezed vacuum has $r = 1.151$, hence $\langle\hat{n}\rangle = \sinh^2(1.151) \approx 2.0$ photons, distributed over $|0\rangle, |2\rangle, |4\rangle, \ldots$. Its quadrature noise ellipse has $\Delta X_1 = 0.158$ (versus vacuum's $0.5$) and $\Delta X_2 = 1.58$. Even the record 15 dB state contains only $\langle\hat{n}\rangle = \sinh^2(1.73) \approx 7.4$ photons — squeezing is a few-photon-scale quantum effect with macroscopic metrological consequences.

Displacing squeezed vacuum gives the **squeezed coherent states** $|\alpha, \xi\rangle = \hat{D}(\alpha)\hat{S}(\xi)|0\rangle$: a bright beam whose noise ellipse is squeezed. Orient the ellipse with $\theta$ and you choose *what* is quiet: **amplitude squeezing** ($\Delta n$ below Poissonian — sub-shot-noise intensity) or **phase squeezing** ($\Delta\phi$ below the coherent-state limit — the choice for interferometry).

## Fragility: Loss Un-Squeezes

Squeezing is the most loss-sensitive resource in this unit. A channel with power transmission $\eta$ mixes the state with vacuum (beam-splitter model of loss, Section 18.2.1), giving detected variance

$$\Delta X_{1,\text{out}}^2 = \eta\,\frac{e^{-2r}}{4} + (1 - \eta)\,\frac{1}{4}$$

The vacuum leaking in through the loss port sets a floor. With $\eta = 0.9$, an infinitely squeezed input ($r \to \infty$) still shows only $-10\log_{10}(0.1) = 10$ dB; a 15 dB input degrades to about 8.9 dB. Contrast a laser beam, which survives 10% loss essentially unbothered. This single formula drives the engineering agenda of squeezed-light systems — ultralow-loss optics, high-efficiency photodiodes, minimal mode mismatch — and explains why 15 dB required a decade of loss-hunting (total detection loss below $\sim 2.5\%$). It also foreshadows the central challenge of continuous-variable quantum computing (Chapter 21), whose GKP-encoded qubits consume squeezing as their raw material.

## Two-Mode Squeezing: Entanglement from Pair Production

Replace $\hat{a}^{\dagger 2}$ by $\hat{a}^\dagger\hat{b}^\dagger$ — pairs split between *two* modes — and you get the **two-mode squeezing operator**

$$\hat{S}_2(\xi) = \exp\left(\xi^*\hat{a}\hat{b} - \xi\,\hat{a}^\dagger\hat{b}^\dagger\right), \qquad \hat{S}_2(\xi)|0,0\rangle = \frac{1}{\cosh r}\sum_{n=0}^\infty \left(-e^{i\theta}\tanh r\right)^n|n, n\rangle$$

The **two-mode squeezed vacuum** has perfectly correlated photon numbers (always $|n, n\rangle$ — the basis of heralded single-photon sources, Section 18.3.1) and quadrature correlations $\mathrm{Var}(\hat{X}_1^{(a)} - \hat{X}_1^{(b)}) = e^{-2r}/2$, $\mathrm{Var}(\hat{X}_2^{(a)} + \hat{X}_2^{(b)}) = e^{-2r}/2$: in the limit $r \to \infty$ it becomes the original EPR state, with both relative position and total momentum sharp. Each mode alone is exactly a *thermal* state with $\bar{n} = \sinh^2 r$ — maximally noisy marginals hiding perfect joint correlations, the continuous-variable face of entanglement (Section 17.4) and the workhorse resource of Chapter 21.

## Why Squeezing Matters

Squeezed light is the quantum resource that has already shipped. It lowers the shot-noise floor of interferometric measurement below the standard quantum limit — LIGO has run with injected squeezed vacuum since 2019, gaining tens of percent in detection range (Section 18.3.3 tells that story in full). It is the sole non-classical ingredient in Gaussian boson sampling (Chapter 20) and the foundation of CV quantum computing (Chapter 21). And conceptually it completes the family portrait begun with Fock and coherent states: number-sharp, phase-sharp, and now *quadrature-sharp* — three incompatible ways for light to be as definite as quantum mechanics allows.
