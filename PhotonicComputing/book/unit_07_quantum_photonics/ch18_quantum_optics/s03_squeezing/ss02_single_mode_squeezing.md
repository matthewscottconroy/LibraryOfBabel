# 18.3.2 Single-Mode Squeezing

## The Degenerate Limit

Tune a parametric down-converter so that signal and idler are the *same* mode — same frequency $\omega_s = \omega_i = \omega_p/2$, same polarization, same direction. The two-mode pair creation $\hat{a}_s^\dagger\hat{a}_i^\dagger$ of Section 18.3.1 collapses into $\hat{a}^{\dagger 2}$: photons are now added to a single mode two at a time. The generator is the **single-mode squeezing operator** of Section 17.3.3,

$$\hat{S}(\xi) = \exp\!\left[\tfrac{1}{2}\big(\xi^*\hat{a}^2 - \xi\,\hat{a}^{\dagger 2}\big)\right], \qquad \xi = re^{i\theta}.$$

In practice one builds a **degenerate optical parametric oscillator (OPO)**: a $\chi^{(2)}$ crystal inside a cavity resonant at the subharmonic $\omega_p/2$, pumped *below* its oscillation threshold. Below threshold there is no bright output — only vacuum, reshaped. The cavity enhances the nonlinear interaction and, just as importantly, defines a single clean spatial-temporal mode into which the squeezing is emitted, which (as we are about to see) is what makes low-loss detection possible.

## Quadratures and the Decibel Scale

The action of $\hat{S}(r)$ on the quadratures was derived in Section 17.3.3; we need the result. With $\hat{X}_1 = (\hat{a}+\hat{a}^\dagger)/2$ and $\hat{X}_2 = (\hat{a}-\hat{a}^\dagger)/2i$,

$$\hat{S}^\dagger(r)\,\hat{X}_1\,\hat{S}(r) = e^{-r}\hat{X}_1, \qquad \hat{S}^\dagger(r)\,\hat{X}_2\,\hat{S}(r) = e^{+r}\hat{X}_2,$$

so the **squeezed vacuum** $\hat{S}(\xi)|0\rangle$ has

$$\Delta X_1 = \frac{e^{-r}}{2}, \qquad \Delta X_2 = \frac{e^{+r}}{2}, \qquad \Delta X_1\,\Delta X_2 = \frac{1}{4}.$$

One quadrature is quieter than vacuum, the conjugate noisier, the product still at the Heisenberg floor — noise relocated, never deleted. Experimentalists quote the noise reduction logarithmically:

$$\text{squeezing (dB)} = -10\log_{10}\!\frac{\Delta X_1^2}{(1/2)^2} = -10\log_{10} e^{-2r} \approx 8.686\,r.$$

Thus $3$ dB is a halving of the noise power ($r = 0.35$), $10$ dB a tenfold reduction ($r = 1.15$), and the record $15$ dB corresponds to $r \approx 1.73$ and a mean of only $\langle\hat{n}\rangle = \sinh^2 r \approx 7.4$ photons — squeezing is a few-photon effect with macroscopic consequences.

## Loss Is the Enemy

Squeezing is the most loss-fragile resource in this book, for the reason established in Section 18.2.1: any loss is a beam splitter tapping the signal against vacuum, and the vacuum leaking in through that port refills the quiet quadrature. A channel of power transmission $\eta$ delivers a detected variance

$$\Delta X_{1,\text{out}}^2 = \eta\,\frac{e^{-2r}}{4} + (1-\eta)\,\frac{1}{4},$$

so the *observed* squeezing in decibels is

$$S_{\text{obs}} = -10\log_{10}\!\big[\eta\,e^{-2r} + (1-\eta)\big].$$

The second term is a vacuum floor that no amount of pumping can beat. As $r\to\infty$ the best achievable squeezing saturates at

$$S_{\max} = -10\log_{10}(1-\eta),$$

set entirely by loss.

**Worked example.** *How much loss can a 13 dB or 15 dB target tolerate?*

Suppose total detection efficiency is $95\%$ ($5\%$ loss, $\eta = 0.95$). The asymptotic ceiling is

$$S_{\max} = -10\log_{10}(1 - 0.95) = -10\log_{10}(0.05) = 13.0\ \text{dB}.$$

Five percent loss caps the observable squeezing at $13$ dB *however hard the OPO is pumped*. Feed a genuinely $15$ dB source ($e^{-2r} = 10^{-1.5} = 0.0316$) through this same $5\%$ loss and the detected value is

$$S_{\text{obs}} = -10\log_{10}\!\big[0.95(0.0316) + 0.05\big] = -10\log_{10}(0.080) = 11.0\ \text{dB}.$$

So $15$ dB generated becomes $11$ dB seen. To *observe* $15$ dB one needs the ceiling above $15$ dB, i.e. total loss below

$$1 - \eta < 10^{-1.5} = 3.2\% ,$$

and in practice below $\sim2.5\%$ once finite $r$ is folded in. This single inequality is the entire engineering program behind the record: ultralow-loss crystal coatings, photodiodes with quantum efficiency above $99\%$, and near-perfect spatial-mode matching to the local oscillator. Contrast a laser beam, which shrugs off $5\%$ loss — squeezing does not.

## Measuring Squeezing: Homodyne Detection

Squeezing lives in a quadrature, and quadratures are read out by **balanced homodyne detection**: interfere the squeezed field with a bright coherent **local oscillator** on a 50/50 beam splitter, subtract the two photocurrents, and the difference is proportional to the field quadrature $\hat{X}_\vartheta$ selected by the local-oscillator phase $\vartheta$. Scanning $\vartheta$ traces the noise ellipse — variance dropping below the vacuum level (the shot-noise reference, taken by blocking the squeezed input) at the squeezed angle and rising above it a quarter-cycle later. This is where the loss budget bites hardest: the spatial and temporal mode of the squeezed light must overlap the local oscillator with near-unity efficiency, and any mode mismatch acts as additional loss in the formula above. The homodyne apparatus is also the reason a *single clean mode* out of the OPO matters — you can only beat against a local oscillator what you can mode-match to it — and it is the same detection principle that continuous-variable quantum computing (Chapter 21) uses to measure its cluster states.

## History and the Record

The milestones trace the maturing of the technique:

- **Slusher, Hollberg, Yurke, Mertz, and Valley (1985)** made the first observation of squeezed light, using four-wave mixing in a sodium atomic beam inside a cavity — a modest but decisive tenths-of-a-decibel dip below vacuum that proved the effect real.
- **Wu, Kimble, Hall, and Wu (1986)** generated the first squeezing by parametric down-conversion in a sub-threshold OPO, reaching several decibels and establishing the OPO as the platform of choice.
- **Vahlbruch, Mehmet, Danzmann, and Schnabel (2016)** hold the current record of **15 dB** of directly detected squeezing at $1064$ nm, achieved at the Albert Einstein Institute in Hannover after a decade-long campaign to drive total loss below $2.5\%$. This is the same wavelength and the same laboratory lineage that supplies the squeezed vacuum for gravitational-wave detection (Section 18.3.3).

## Why It Matters

Single-mode squeezed light is the raw material of continuous-variable quantum computing (Chapter 21), where GKP qubits and cluster states are literally built from squeezing, and of quantum-enhanced sensing, where it lowers the shot-noise floor below the standard quantum limit. The loss formula above is the master constraint of both applications: it explains why CV architectures live or die by optical loss, and why the leap from Wu's few decibels to Vahlbruch's fifteen took thirty years. The next subsection puts this resource to work in the most demanding measurement humanity has yet built.
