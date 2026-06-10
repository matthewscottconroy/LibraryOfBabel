# Chapter 11: Exercises

## Mathematical Exercises

**11.1** (Lens Fourier transform) A thin lens of focal length $f = 150$ mm is illuminated with a coherent plane wave at $\lambda = 532$ nm. A transparency with transmittance $t(x,y) = \text{rect}(x/a) \cdot \text{rect}(y/b)$ is placed at the front focal plane, where $a = 2$ mm and $b = 1$ mm.

(a) Write the expression for the field at the back focal plane and identify the spatial frequencies at which features appear.

(b) The rect function has Fourier transform $\hat{t}(f_x, f_y) = ab \cdot \text{sinc}(a f_x) \cdot \text{sinc}(b f_y)$. Find the positions (in mm) of the first zeros in $x'$ and $y'$ at the back focal plane.

(c) Find the width (in μm) of the central sinc lobe in $x'$.

(d) If the lens aperture is 25 mm, what is the spatial frequency cutoff of the lens, and does the aperture limit the output in this case?

**11.2** (4f spatial filter) In a 4f system with $f = 200$ mm and $\lambda = 633$ nm, a low-pass filter consists of a circular aperture of radius $r_0 = 2$ mm at the Fourier plane.

(a) What is the cutoff spatial frequency $f_{c}$ (in cycles/mm) at the input?

(b) An image of a binary grating with period $\Lambda = 50$ μm is placed at the input. Does the 4f system pass or block the fundamental spatial frequency of the grating? The first harmonic?

(c) Write the expression for the impulse response $h(r)$ of the circular aperture filter (in terms of $J_1$, the first-order Bessel function).

(d) If the input image contains two point sources separated by 30 μm, does the 4f system resolve them?

**11.3** (Optical correlator performance) An optical correlator is used to locate a $10 \times 10$ mm target in a $100 \times 100$ mm scene. The matched filter is a VanderLugt filter at $\lambda = 633$ nm with lens focal length $f = 300$ mm.

(a) What is the size of the autocorrelation peak at the output plane (Rayleigh criterion)?

(b) The input scene is digitized at 50 μm/pixel. How many pixels is the autocorrelation peak?

(c) A digital FFT-based correlator uses the same discretization. Estimate the computation time on a CPU (see text for throughput estimate).

(d) Under what conditions (batch size, input size) is the optical correlator faster end-to-end? Use the SLM update time of 8 ms and CPU FFT time of 5 μs as given in the text.

**11.4** (Photonic ADC jitter analysis) A photonic ADC uses a mode-locked laser with 0.8 fs RMS timing jitter.

(a) What is the ENOB at input frequencies of 10, 40, and 100 GHz?

(b) Compare to an electronic ADC with 20 fs jitter at the same frequencies.

(c) At what input frequency does the electronic ADC drop below 6-bit ENOB?

(d) At what input frequency does the photonic ADC drop below 6-bit ENOB?

**11.5** (True time delay beamforming) A 16-element linear phased array has 15 mm element spacing and is designed to operate at 77 GHz (automotive radar) with 4 GHz bandwidth.

(a) Calculate the time delay between adjacent elements needed to steer to $\theta = 30°$.

(b) What is the beam squint $\Delta\theta$ at $f_0 \pm 2$ GHz if a phase shifter (not TTD) is used?

(c) A photonic TTD system uses SMF-28 fiber ($D = 17$ ps/nm/km) with variable lengths. What fiber length is needed for element $n = 8$ (furthest from center) for the steering in (a)? How much wavelength tuning (in nm) gives this delay?

(d) An on-chip Si₃N₄ waveguide TTD element uses 10 cm of waveguide with group index $n_g = 1.98$ and thermo-optic tuning of $\Delta n_g/\Delta T = 2.5\times10^{-5}$ K$^{-1}$. What temperature change provides the delay in (c)?

**11.6** (Kerr switching energy) A chalcogenide glass (As₂S₃) microring resonator has radius 10 μm, waveguide cross-section 0.8 μm × 0.5 μm, $n_2 = 3\times10^{-18}$ m²/W, $\beta_{\text{TPA}} = 0.001$ cm/GW, and $Q = 15{,}000$.

(a) Calculate the FOM and verify the device is above the switching threshold.

(b) Calculate the switching energy needed to shift the resonance by one linewidth.

(c) Calculate the TPA-induced power loss for an intracavity intensity of $I = 10$ GW/cm².

(d) Compare the switching energy in (b) to a 5 nm CMOS NAND gate (20 aJ). By how many orders of magnitude does the optical switch lose?

---

## Conceptual Exercises

**11.7** The Fourier transforming property of a lens requires that the input field be placed at the *front focal plane*, not at the lens itself. Explain physically why this is the case. What happens if the input is placed at the lens plane (z = 0)?

**11.8** The VanderLugt filter computes the cross-correlation between input $g$ and reference $f$. For inputs that are exact copies of the reference (i.e., $g = f$), the output is the autocorrelation. For what class of references $f$ does the autocorrelation have the narrowest peak? Why does this matter for pattern recognition performance?

**11.9** The photonic ADC avoids clock jitter by using mode-locked laser pulses as the sampling gate. However, there is another source of timing error: the timing jitter of the *electro-optic modulator* that imprints the RF signal. Explain why modulator timing jitter is not a fundamental limit for the photonic ADC (as opposed to the electronic ADC), and under what conditions it could become one.

**11.10** The time-stretch ADC uses a stretch factor $M$ to convert high-bandwidth signals to lower bandwidth for electronic ADC capture. Explain what happens to the SNR as $M$ is increased. Is there an optimal stretch factor?

**11.11** The Kerr FOM = $n_2/(\lambda \beta_{\text{TPA}})$ must exceed $\sim 1/4\pi$ for Kerr switching to be practical. Si₃N₄ has FOM $= \infty$ (no TPA) but $n_2$ is 25× smaller than silicon. Does this make Si₃N₄ better for Kerr switching? Explain the trade-off.

**11.12** SOA cross-gain modulation (XGM) can implement optical NOT but has limited cascadability. Explain, using the noise figure of an SOA ($F \approx 2 N_{\text{sp}} = 4$–6 dB), why a chain of 10 SOA gates has much worse signal quality than a chain of 10 CMOS logic gates.

---

## Lab / Experimental Exercises

**11.13** (Simulation: 4f spatial filter) Using Python and numpy's FFT functions:

(a) Simulate a 4f processor with an input image (e.g., a JPEG of your choice). Implement low-pass, high-pass, and band-pass spatial filters by multiplying the FFT spectrum by an appropriate mask.

(b) Add Gaussian noise to the input and observe how the noise affects each filtered output differently.

(c) Implement a VanderLugt matched filter for a simple reference pattern (a small $50 \times 50$ pixel template). Show that the cross-correlation peak moves to the correct location when the template is embedded in a noisy background.

(d) Measure the peak-to-sidelobe ratio (PSR) of the correlation peak as a function of the template's uniqueness (e.g., compare a generic circle vs. a specific letter shape).

**11.14** (Microwave photonics link energy model) Implement a Python model of a photonic RF link:

(a) Write a function that computes RF gain, noise figure, and spurious-free dynamic range (SFDR) for an IMDD link as a function of optical power $P_0$ and modulator $V_\pi$.

(b) Find the optimal operating point (maximum SFDR) as a function of $P_0$.

(c) Compare the noise figure of IMDD to a push-pull MZI link and a carrier-suppressed single-sideband (CSSB) link.

(d) At what optical power does the link NF = 3 dB (comparable to the best electronic amplifiers)?

**11.15** (Optical correlator vs. digital) Measure or estimate the following for your computing environment:

(a) Time to compute one 1024×1024 2D FFT using numpy.fft.fft2.

(b) Time to compute the cross-correlation of two 1024×1024 images using the FFT method.

(c) Given an SLM update rate of 60 Hz (16.7 ms per frame), for how many consecutive correlation queries does the optical correlator become faster than the digital approach?

(d) If the SLM frame rate increases to 1000 Hz (1 ms), how does this change the crossover query count?
