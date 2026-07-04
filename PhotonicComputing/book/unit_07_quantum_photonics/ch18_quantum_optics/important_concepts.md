# Chapter 18: Important Concepts

---

## 1. Second-Order Coherence $g^{(2)}(0)$

The normally ordered intensity correlation $g^{(2)}(\tau) = \langle\hat{a}^\dagger(t)\hat{a}^\dagger(t+\tau)\hat{a}(t+\tau)\hat{a}(t)\rangle / \langle\hat{a}^\dagger\hat{a}\rangle^2$ is the master diagnostic of quantum light, equal at zero delay to $\langle\hat{n}(\hat{n}-1)\rangle/\langle\hat{n}\rangle^2$. Benchmarks: coherent (laser) $=1$, thermal $=2$, Fock $|n\rangle$ $= 1-1/n$ (so $|1\rangle\to 0$), single-photon source $\to 0$. First-order coherence $g^{(1)}$ (fringe visibility, the power spectrum's transform) is *blind* to these distinctions — the statistics live at second order.

---

## 2. The Classical Bound and Mandel $Q$

Every classical field obeys $g^{(2)}(0) = 1 + \langle(\Delta I)^2\rangle/\langle I\rangle^2 \ge 1$ (a variance is non-negative — the Cauchy-Schwarz inequality for light). The Mandel parameter $Q = \langle(\Delta\hat{n})^2\rangle/\langle\hat{n}\rangle - 1$ recasts the same content in number statistics, with $g^{(2)}(0) = 1 + Q/\langle\hat{n}\rangle$: $Q=0$ Poissonian, $Q>0$ super-Poissonian, $Q<0$ sub-Poissonian (floor $Q=-1$ for a Fock state). Sub-Poissonian light and antibunching are both non-classical and coincide for a single stationary mode.

---

## 3. Hanbury Brown-Twiss

A source, a 50/50 beam splitter, two detectors, a coincidence counter: the HBT geometry measures $g^{(2)}(\tau)$ directly. Splitting onto two detectors defeats single-detector dead time and afterpulsing, making the $\tau\to0$ correlation accessible. The cross-correlation between the two outputs equals the input auto-correlation, $g^{(2)}_{cd}(0)=g^{(2)}(0)$. In pulsed operation, $g^{(2)}(0) = (\text{central-peak area})/(\text{side-peak area})$. HBT's 1956 discovery of thermal-light bunching ($g^{(2)}(0)=2$) is classical, but it forced Glauber's quantum coherence theory into existence.

---

## 4. Antibunching as a Non-Classicality Witness

$g^{(2)}(0) < 1$ is forbidden to every classical field and requires a Glauber-Sudarshan $P(\alpha)$ that fails to be a probability density. A single quantum emitter enforces it physically: with one quantum of excitation, it cannot emit two photons at once and must re-excite over $\sim1/\Gamma$, giving $g^{(2)}(\tau)=(1-e^{-\Gamma\tau/2})^2$. First seen by Kimble, Dagenais & Mandel (1977) in atomic resonance fluorescence; the Grangier-Roger-Aspect anticorrelation parameter $\alpha = P_{cd}/P_cP_d \approx 0.18 < 1$ (1986) proved the photon indivisible on a beam splitter. Modern sources reach $g^{(2)}(0) < 10^{-4}$.

---

## 5. The Beam Splitter as a Two-Mode Unitary

Output modes are unitary combinations of inputs, $\hat{c}=t\hat{a}+r\hat{b}$, $\hat{d}=r\hat{a}+t\hat{b}$, with commutator preservation forcing $|r|^2+|t|^2=1$ and $r^*t+rt^*=0$. The symmetric 50/50 choice is $t=1/\sqrt2$, $r=i/\sqrt2$. A single photon delocalizes rather than splits: $|1,0\rangle \to (|1,0\rangle+i|0,1\rangle)/\sqrt2$, one click never two — the dual-rail qubit of Chapter 20.

---

## 6. The Vacuum Port Is Mandatory

An "unused" beam-splitter port carries vacuum, and that vacuum enters the output whether or not one accounts for it. Dropping it breaks the commutator: $\hat{c}=\hat{a}/\sqrt2$ gives $[\hat{c},\hat{c}^\dagger]=1/2\neq1$, an invalid mode. This single fact underlies the beam-splitter model of loss (vacuum refilling a lossy channel, Section 17.3.3), shot noise in an interferometer (vacuum in the dark port), and the injection of squeezed vacuum into LIGO (Section 18.3.3).

---

## 7. The Hong-Ou-Mandel Effect

Two indistinguishable photons entering opposite ports of a 50/50 beam splitter always bunch: $\hat{a}^\dagger\hat{b}^\dagger|0\rangle \to \frac{i}{\sqrt2}(|2,0\rangle+|0,2\rangle)$, coincidences vanish. The mechanism is destructive interference of two-photon amplitudes — both-transmit ($t^2=\tfrac12$) plus both-reflect ($r^2=-\tfrac12$) sum to zero — not any interaction. Scanning relative delay traces the HOM dip $P_{\text{coinc}}(\tau)=\tfrac12(1-e^{-\sigma^2\tau^2})$; its depth is the visibility $V=1-2P_{\min}$, which equals the indistinguishability $M=|\langle\psi_1|\psi_2\rangle|^2$. HOM measures whether two photons are truly the same photon.

---

## 8. HOM as the LOQC Primitive

Photons do not interact, so two-photon interference is the *only* entangling resource in linear optics; measurement (photodetection) supplies the nonlinearity (KLM, Chapter 20). Every linear-optical entangling gate and fusion operation is HOM interference read out by detectors. Partial distinguishability is gate error: $\varepsilon_{\text{gate}}\sim(1-M)$. For Gaussian spectra offset by $\Delta\omega$, $M=e^{-\Delta\omega^2/4\sigma^2}$, so a $20\%$-of-bandwidth mismatch gives $M\approx0.99$ and $\sim1\%$ error; fault tolerance demands $V=M>99.9\%$, i.e. center frequencies matched to $\sim6\%$ of bandwidth.

---

## 9. Parametric Down-Conversion and Two-Mode Squeezing

A $\chi^{(2)}$ crystal converts one pump photon into a signal-idler pair under energy ($\omega_p=\omega_s+\omega_i$) and phase-matching ($\mathbf{k}_p=\mathbf{k}_s+\mathbf{k}_i$) conservation, realizing the two-mode squeezer $\hat{S}_2(\xi)=\exp(\xi^*\hat{a}_s\hat{a}_i-\xi\hat{a}_s^\dagger\hat{a}_i^\dagger)$. From vacuum it makes $|\psi\rangle=\frac{1}{\cosh r}\sum_n\tanh^n r|n,n\rangle$: number-correlated pairs (thermal marginals, $\bar{n}=\sinh^2 r$). Type-II phase matching gives polarization-entangled Bell pairs (Kwiat et al., 1995). Threshold-heralded purity is $g^{(2)}_h(0)=2\mu/(1+\mu)$, so purity forces a dim pump — the brightness-purity trade-off. PPKTP delivers $\sim10^6$ pairs/s at mW pump.

---

## 10. Single-Mode Squeezing, the dB Scale, and Loss Fragility

Degenerate down-conversion (sub-threshold OPO) realizes $\hat{S}(\xi)=\exp[\tfrac12(\xi^*\hat{a}^2-\xi\hat{a}^{\dagger2})]$, giving squeezed vacuum $\Delta X_1=e^{-r}/2$, $\Delta X_2=e^{+r}/2$, quantified as $-10\log_{10}e^{-2r}\approx8.686\,r$ dB. Loss un-squeezes: $\Delta X^2_{\text{out}}=\eta e^{-2r}/4+(1-\eta)/4$, with an asymptotic ceiling $S_{\max}=-10\log_{10}(1-\eta)$ — $5\%$ loss caps squeezing at $13$ dB. The $15$ dB record (Vahlbruch et al., 2016; $r\approx1.73$, $\langle\hat{n}\rangle\approx7.4$) required total loss below $\sim2.5\%$.

---

## 11. Squeezed Light and Quantum Metrology

Interferometric phase sensitivity is limited at the standard quantum limit $\Delta\phi\sim1/\sqrt{N}$ by vacuum entering the dark port (Caves, 1981). Injecting squeezed vacuum there beats the limit: $S$ dB of squeezing improves amplitude sensitivity by $10^{S/20}$. Advanced LIGO's $\sim3$ dB (Tse et al., 2019) yielded a $\sim15\%$ binary-neutron-star range gain and $\sim40$–$50\%$ more detections (rate $\propto$ range$^3$). Frequency-dependent squeezing via a filter cavity evades the radiation-pressure penalty for broadband sub-SQL operation — quantum optics as working astronomical infrastructure.
