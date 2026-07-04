# Chapter 25: Exercises

These problems exercise the one skill this chapter exists to teach: drawing the energy, latency, and precision boundary around a *whole* computation and refusing to let any term hide outside it. Every problem is quantitative; several ask you to audit a claim, which is quantitative work of a different kind. Numbers are drawn from Sections 25.1–25.3 so that answers can be checked against the worked examples there.

Useful constants: photon energy at $\lambda = 1550$ nm, $E_{\text{ph}} = 0.128$ aJ; TOPS/W $= 2/E_{\text{MAC}}[\text{pJ}]$ (MAC $=$ 2 OP); ENOB $= (\text{SNDR}_{\text{dB}} - 1.76)/6.02$.

---

## Group A — Throughput, Amortization, and Energy Budgets

**A1 — Peak throughput and the counting convention.**
A $64 \times 64$ photonic matrix-vector multiplier is clocked at vector rate $f = 50$ GHz.

(a) Compute the peak throughput in MAC/s and in TOPS under the MAC $=$ 2 OP convention.

(b) A competitor's datasheet lists a rival chip at "205 TMAC/s." Convert both parts to a common unit and show that the two numbers describe the *same* performance — identify the factor-of-two convention gap and state the audit rule it implies.

(c) The mesh uses $N(N-1)/2$ MZIs, each with a 10 mW thermo-optic heater. Compute the total static heating power. Express it also as an energy per MAC at $f = 50$ GHz and compare to your answer for the optical computation energy (assume 10 fJ detected per output symbol).

**A2 — The conversion tax and why small arrays lose.**
Take the interface tax as 5 pJ per input–output sample pair (Section 25.1.1).

(a) Using $E_{\text{interface}}/\text{MAC} = \text{tax}/N$, tabulate the per-MAC interface energy for $N = 8, 16, 64, 256$.

(b) An H100-class *system* delivers ~0.71 pJ/MAC. Above what $N$ does the interface tax *alone* fall below this figure? Above what $N$ does it fall below 100 fJ/MAC?

(c) In one sentence, justify the chapter's claim that "small photonic accelerators lose before the light is turned on," citing your table.

**A3 — End-to-end energy per inference (the honest budget).**
For the $64\times64$, 10 GHz, 6-bit engine of Section 25.2.1, the per-MAC contributions are: laser (wall-plug) 8 fJ, conversion tax 81 fJ, clock 16 fJ, digital post-processing 50 fJ, and weight-hold either 732 fJ (thermo-optic) or 1 fJ (non-volatile).

(a) Compute $E_{\text{MAC}}$ and TOPS/W for each weight-hold technology.

(b) ResNet-50 inference costs ≈ 2 GMAC per 224×224 image. Compute the energy per image for each configuration.

(c) An H100-class system delivers ≈ 0.71 pJ/MAC → ≈ 1.42 mJ/image. Which photonic configuration beats the GPU, and by how much? Which loses, and why?

(d) Now suppose poor activation reuse forces off-chip DRAM traffic at ~1 nJ per 64-bit access. Explain quantitatively how this term can dominate *both* machines and why energy-per-inference, not TOPS/W, is the number a datacenter operator actually pays.

**A4 — When static power erases the advantage.**
The full $64\times64$ mesh holds its weights with 30 W of thermo-optic power; a MEMS/PCM alternative holds them at 40 mW.

(a) Verify that the thermo-optic hold contributes 732 fJ/MAC at $f = 10$ GHz, and that the non-volatile hold contributes ~1 fJ/MAC.

(b) What per-MZI heater power would reduce the static contribution to 50 fJ/MAC? (There are $N(N-1)/2 = 2016$ heaters.) Is this achievable with silicon thermo-optics, and what does your answer imply about the relative value of "a better modulator" versus "a better phase-shifter technology"?

---

## Group B — Precision, Noise, and Photon Budgets

**B1 — The $2^{2b}$ photon law.**
Shot-noise-limited detection of $2^b$ output levels requires $n_{\max} \gtrsim 2^{2b}$ photons per output symbol.

(a) Tabulate $n_{\max}$ and the detected energy per symbol for $b = 4, 6, 8, 10$.

(b) Fold in 10 dB path loss and 10% laser wall-plug efficiency. What is the wall-plug optical energy per output symbol at $b = 8$?

(c) One output symbol absorbs the light of $N$ MACs. For $N = 64$ at $b = 8$, express your part-(b) answer as a wall-plug optical energy *per MAC*, and explain why this term improves as $N$ grows while digital MAC energy does not.

**B2 — ENOB, the jitter ceiling, and an impossible datasheet.**

(a) A full analog chain measures SNDR $=$ 38 dB at its digitized output. What is its ENOB?

(b) With RMS aperture jitter $\sigma_j = 100$ fs, compute the jitter-limited SNR and ENOB for analog input frequencies of 10 GHz and 25 GHz using $\text{SNR}_{\text{jitter}} = -20\log_{10}(2\pi f_{\text{in}}\sigma_j)$.

(c) A vendor claims 10 effective bits at 20 GHz analog input. What aperture jitter would that require? Compare to the ~100 fs of excellent integrated clocking and state the audit conclusion.

**B3 — RIN and thermal floors.**
Laser relative intensity noise caps SNR at $1/(\text{RIN}\cdot B)$ over receiver bandwidth $B = 10$ GHz.

(a) Compute the RIN-limited ENOB ceiling for a DFB at RIN $= -150$ dB/Hz and for a comb line at RIN $= -140$ dB/Hz.

(b) The TIA's input-referred noise ($\sigma_I \approx 0.5\ \mu$A RMS over 10 GHz) demands a peak photocurrent $> 2^b\sigma_I$ for $b$-bit precision. Compute the required optical power at $b = 6$ (responsivity 0.8 A/W) and compare to the shot-limited budget (0.52 fJ/symbol). Is a direct-detection accelerator shot-limited or thermal/RIN-limited?

(c) Homodyne (coherent) readout amplifies the signal against a strong local oscillator. Explain quantitatively which noise term this attacks and why it moves the system back toward the shot limit.

**B4 — From ENOB to task accuracy.**
A photonic classifier reports 88% accuracy on hardware; the identical model in floating point reports 90%. A second paper reports "92% accuracy" and gives no digital baseline.

(a) Which result is auditable, and what single number is missing from the second?

(b) Averaging $K$ independent zero-mean noisy passes reduces the noise standard deviation by $\sqrt{K}$. How many repeated passes would halve the analog noise contribution? Why does this averaging *not* help against slow drift or correlated component error?

(c) State the iso-accuracy reporting rule this chapter enforces, and explain why it is the analog-optical analog of MLPerf's accuracy constraint.

---

## Group C — Fair Comparison and Auditing

**C1 — Energy per MAC across three machines.**
Compare, at the same operation and boundary where possible: (a) an H100 GPU at 2.8 TOPS/W (INT8); (b) a photonic MZI core carrying 1 mW average optical power per output, with 64 inputs at 10 GHz vector rate (optical-domain energy only); (c) a biological synapse at ~10 fJ per event. Compute an energy per MAC for each, then explain which comparison is *fair* and which quietly swaps the core boundary for the system boundary. What is the honest one-sentence conclusion?

**C2 — Reservoir computer versus embedded LSTM.**
A photonic reservoir computer reports 97% on NARMA-10 at 10 ns latency and 5 mW. A digital LSTM of equivalent capacity reports 97% at 1 μs and 100 mW on an embedded MCU.

(a) Compute the energy per inference and the latency ratio for each.

(b) Identify at least three line items that a 5 mW figure for the photonic system may omit (Section 25.1.1), and estimate how a single 3 pJ readout ADC per output at the sample rate changes the comparison.

(c) State the workload regime in which the photonic advantage is real and the regime in which the omitted terms could erase it.

**C3 — Auditing a "100 TOPS/W" headline.**
A press release claims a photonic accelerator "achieves 100 TOPS/W."

(a) Convert to a core energy per MAC. This is a *core-only* figure. Add a conversion tax of 78 fJ/MAC (a 5 pJ tax at $N = 64$) and a 100 fJ/MAC static-plus-digital allowance, then recompute the *system* TOPS/W. By what factor did the headline collapse?

(b) The 100 TOPS/W was measured at 4-bit precision against an FP16 GPU baseline. Explain, using the exponential energy–precision relationship, why this comparison inflates the photonic number, and describe the precision normalization that would make it fair.

(c) Run the claim through the six-line auditor's checklist (boundary, numerator, accuracy, scale, latency, reproducibility) and list, for each line, the single question you would ask the authors.

**C4 — WDM parallelism as a tax reducer.**
A broadcast-and-weight architecture sends $K$ comb lines through one shared weight bank and one modulator–detector chain sampling at rate $f$.

(a) At fixed hardware, throughput scales with $K$ while the digitized port count (and thus the conversion tax) does not. Show that the conversion tax per MAC therefore falls as $\text{tax}/(N K)$, and tabulate it for $K = 8, 32, 128$ at $N = 64$, tax $=$ 5 pJ.

(b) List two physical mechanisms that bound $K$ in a real comb-driven system, and explain why WDM is "a multiplier on a well-designed system, not a substitute for one."

---

## Group D — Programming Projects

**D1 — Photonic accelerator design-space explorer.**
Build a parameterized model of a WDM broadcast-and-weight matrix multiplier. Sweep the number of wavelengths (8–128), detector count (8–128), and optical power per channel. For each design point compute throughput (GMAC/s) and total wall-plug power using the Section 25.2.1 master equation (laser ÷ WPE, conversion tax, static hold, digital). Plot the Pareto frontier of throughput versus energy efficiency, overlay the contemporary H100 operating point at the same model size, and identify the region of the design space (if any) that sits several-fold above the electronic frontier. Report which term dominates in each corner of the sweep.

**D2 — Full system power budget and a 2× co-design pass.**
For a $64\times64$ accelerator at 50 GHz, itemize the power drawn by each component: laser (at 10% WPE), modulator drivers, ring/phase-shifter tuning, TIAs, ADCs, DACs, clocking, and digital control. Identify the single dominant consumer. Then apply one round of co-design — non-volatile weight hold, precision-matched (6-bit) converters, 3D-integrated drivers, and weight-stationary scheduling — and show, term by term, whether you can improve total system energy per MAC by at least 2×. State explicitly which improvements came from the optics (they should be few) and which came from the electronics and the algorithm (they should be most).
