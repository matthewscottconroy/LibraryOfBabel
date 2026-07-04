# Chapter 12: Exercises

## Mathematical Exercises

**12.1** (MZI unitarity and spectrum) Consider the balanced MZI transfer matrix with internal phase $\theta$ and input phase $\phi$ used in Section 12.2.1.

(a) Verify by direct multiplication that $U_{\text{MZI}}^\dagger U_{\text{MZI}} = I$ for all $(\theta, \phi)$.

(b) For $\phi = 0$, find the eigenvalues and eigenvectors of $U_{\text{MZI}}(\theta)$ and show the eigenvalues lie on the unit circle.

(c) How many real parameters does a general $2\times2$ unitary matrix have? How many does one MZI with $(\theta, \phi)$ provide? What hardware supplies the missing parameters in a full mesh?

**12.2** (Clements decomposition of the DFT) The $4\times4$ discrete Fourier transform matrix has entries $F_{jk} = \frac{1}{2} e^{-2\pi i jk/4}$.

(a) Verify that $F$ is unitary.

(b) How many MZIs and how many output phase shifters does the Clements decomposition require for $N = 4$? What is the mesh depth?

(c) Carry out the decomposition (by hand or symbolically) and give the six $(\theta, \phi)$ pairs. Sketch the rectangular mesh with each MZI labeled.

(d) Compare with the Reck decomposition of the same matrix: same MZI count, different depth. Which mesh accumulates less worst-case insertion loss if each MZI stage costs 0.3 dB?

**12.3** (SVD implementation cost) A trained fully connected layer has weight matrix $W \in \mathbb{R}^{64 \times 64}$ with condition number $\kappa = 20$.

(a) Count the MZIs for the full SVD implementation ($V^\dagger$ mesh + $\Sigma$ + $U$ mesh).

(b) With each MZI occupying 500 μm × 100 μm, estimate the chip area.

(c) The singular values are normalized by $\sigma_{\max}$. What is the optical power transmission of the weakest singular channel, in dB? How many bits of shot-noise-limited ENOB does this cost that channel relative to the strongest?

(d) Suppose 90% of $\|W\|_F^2$ lies in the top 16 singular values. Estimate the Frobenius-norm error of the rank-16 truncation, the reduced MZI count, and the recovered insertion loss (0.25 dB per mesh stage).

**12.4** (Shot-noise-limited precision) A photonic matrix multiplier delivers $P = 10$ μW to each output photodetector at $\lambda = 1550$ nm with integration time $T_{\text{int}} = 100$ ps and quantum efficiency $\eta = 0.8$.

(a) Compute the mean detected photon number and the shot-noise-limited SNR.

(b) Convert to ENOB via $\text{ENOB} = (\text{SNR}_{\text{dB}} - 1.76)/6.02$.

(c) What minimum optical power is required for 8-bit precision at this integration time?

(d) The system is scaled from $N = 64$ to $N = 512$ at fixed total laser power. Assuming detected power per output scales as $1/N$, how many bits are lost?

**12.5** (Ring weight bank crosstalk) A weight bank uses rings of loaded $Q = 12{,}000$ at 1550 nm on a 100-GHz WDM grid.

(a) Compute the ring linewidth in GHz and nm.

(b) Using the Lorentzian model of Section 12.4.1, compute the nearest-neighbor crosstalk weight when a ring sits exactly on its own channel.

(c) The ring radius is 7 μm with group index $n_g = 4.2$. Compute the FSR and the maximum channel count on this grid.

(d) The signal on each channel is modulated at 10 Gb/s. Is the ring linewidth sufficient to pass the modulation sidebands without distortion? What constraint does this place on the maximum usable $Q$?

**12.6** (Thermal drift budget) A 32-mode Clements mesh uses thermo-optic phase shifters of length 100 μm in silicon ($dn/dT = 1.87\times10^{-4}$ K$^{-1}$, $\lambda = 1550$ nm).

(a) Compute the phase error per shifter for a uniform 0.5 K chip temperature rise.

(b) Using the error scaling $\langle\|\delta \mathbf{y}\|\rangle/\|\mathbf{y}\| \sim \sqrt{K}\,\sigma_{\text{phase}}$ with $K = N(N-1)/2$, estimate the relative output error.

(c) What temperature stability is required to keep the mesh at 6-bit matrix fidelity?

(d) Each heater dissipates 10 mW at its $\pi$ setting. Estimate the total static power for the mesh with average setting $\pi/2$, and comment on the self-heating implied by your answer to (c).

## Design Exercises

**12.7** (Architecture selection) For each workload below, choose the most appropriate architecture from: coherent MZI mesh, broadcast-and-weight ring network, PCM crossbar + comb, time-wavelength interleaving. Justify each choice in 3–5 sentences using the criteria of Subsection 12.4.3: (a) inference-only CNN on a satellite with a 10 W power budget and annual weight updates; (b) real-time equalization of a 100 GBaud coherent optical link; (c) a quantum-photonics experiment requiring programmable $8\times8$ unitaries; (d) a data-center transformer accelerator requiring weight updates every few minutes.

**12.8** (Broadcast-and-weight power budget) Design a 24-neuron broadcast-and-weight network at 3 GHz bandwidth. Choose the WDM grid, ring $Q$, and FSR; verify crosstalk < 1%; and produce a per-neuron power budget (laser share after 1:24 splitting, weight-bank insertion loss, heater power at 2.5 mW/ring, 40 mW analog front end). Report system power, MACs per second, and pJ/MAC. Identify the single largest power line item and propose one design change to reduce it.

**12.9** (Differential encoding) Signed inputs $x_j \in [-1, 1]$ must be encoded on non-negative optical power. Two schemes are proposed: (i) bias encoding $P_j \propto (x_j + 1)/2$ with electronic subtraction of the known offset response $\sum_j w_{ij}/2$; (ii) dual-rail encoding with separate positive and negative channels. Analyze both for: detector dynamic-range consumption, sensitivity to laser power drift, hardware count, and noise accumulation. Which would you use for $N = 64$, and why?

## Conceptual Exercises

**12.10** Coherent meshes hold weights in interferometer phases; incoherent banks hold weights in resonator detunings; PCM crossbars hold weights in material states. For each, explain (a) what physical drift mechanism corrupts the stored weight, (b) the timescale of that drift, and (c) the natural correction strategy. What general principle relates weight *volatility* to weight *update speed* across all three?

**12.11** An ONN trained offline at FP32 loses 9 percentage points of accuracy when deployed on an MZI mesh. Rank the following candidate causes by likelihood and describe one diagnostic measurement for each: coupler splitting-ratio error, thermal crosstalk between heaters, DAC quantization of phases, photodetector shot noise, laser wavelength drift.

**12.12** Explain why the photodetector is simultaneously the greatest convenience and the greatest constraint of incoherent photonic computing. Your answer should address: automatic summation, the $|E|^2$ nonlinearity, loss of phase, and why incoherent multi-layer networks are necessarily O/E/O per layer.

---

## Lab / Computational Exercises

**12.13** (Clements decomposition library) Implement Reck and Clements decompositions in Python/NumPy. Given a random $8\times8$ unitary (from QR decomposition of a complex Gaussian matrix), extract all phases, reconstruct the unitary from MZI products, and verify $\|U_{\text{rec}} - U\|_F < 10^{-12}$. Add Gaussian phase noise $\sigma \in \{0.005, 0.01, 0.02, 0.05\}$ rad to every phase and plot matrix fidelity vs. $\sigma$ for $N = 4, 8, 16, 32$; confirm the $\sqrt{K}\sigma$ scaling.

**12.14** (Ring weight bank simulator) Model an 8-channel weight bank: Lorentzian ring responses, 100-GHz grid, thermal tuning with 10 GHz/K sensitivity and Gaussian temperature noise. Compute the actual weight matrix (including crosstalk) as a function of target weights; quantify effective weight ENOB vs. ring $Q$ and temperature noise amplitude; implement a simple dither-based feedback loop and measure the precision improvement.

**12.15** (End-to-end photonic MNIST) Using PyTorch, build a 2-layer MLP (784→64→10) where each linear layer is parameterized as $U \Sigma V^\dagger$ with $U, V$ built from Clements-mesh phases. Train on MNIST (a) noiselessly; (b) with phase noise $\sigma = 0.02$ rad injected during training; (c) noiselessly, then evaluated with the same noise. Report the accuracy triplet and explain the ordering. Add per-output shot noise corresponding to 6-bit ENOB and repeat.

**12.16** (Power model shoot-out) Build a component-level energy model for a $64\times64$ matrix-vector multiply on: (i) an MZI mesh with thermo-optic weights, (ii) a broadcast-and-weight network, (iii) a PCM crossbar with comb source, and (iv) a GPU tensor core at 1 fJ/MAC arithmetic + 100 fJ/MAC DRAM traffic (batch size 1) or 5 fJ/MAC (batch 512). Include laser wall-plug efficiency (10%), DAC/ADC at 1 pJ/conversion, heater power, and TIA power. Plot pJ/MAC vs. batch size and identify the crossover regimes.
