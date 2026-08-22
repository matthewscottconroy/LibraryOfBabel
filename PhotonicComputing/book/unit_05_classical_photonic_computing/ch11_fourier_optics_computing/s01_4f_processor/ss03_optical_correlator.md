# Subsection 11.1.3: The Optical Correlator

## Orientation

The optical correlator is the classical optical computing system that came closest to production deployment before electronic computing overtook it. Understanding what it does well, what it does poorly, and why it lost to digital FFTs provides essential insight into what makes any analog optical processor genuinely advantageous versus merely theoretically appealing.

---

## 11.1.3.1 Architecture and Operation

### The Joint Transform Correlator (JTC)

The Joint Transform Correlator [1], simpler than the VanderLugt filter, places both the input image $g(x,y)$ and the reference pattern $f(x,y)$ side-by-side at the input plane:

$$U_{\text{in}}(x,y) = g\left(x+\frac{a}{2}, y\right) + f\left(x-\frac{a}{2}, y\right)$$

where $a$ is the separation between the two patterns. The lens computes the joint Fourier transform:

$$\hat{U}_{\text{in}}(f_x, f_y) = e^{i\pi a f_x} \hat{g}(f_x, f_y) + e^{-i\pi a f_x} \hat{f}(f_x, f_y)$$

The intensity at the Fourier plane (which is recorded by a camera or SLM):

$$I(f_x, f_y) = |\hat{U}_{\text{in}}|^2 = |\hat{g}|^2 + |\hat{f}|^2 + \hat{g}\hat{f}^* e^{-i2\pi a f_x} + \hat{g}^*\hat{f} e^{i2\pi a f_x}$$

The cross terms $\hat{g}\hat{f}^*$ and $\hat{g}^*\hat{f}$ contain the cross-correlation and cross-correlation-conjugate information. A second Fourier transform (by a second lens, or by re-illuminating the recorded intensity) produces:

$$\mathcal{F}^{-1}\{I\} = (g\star g)(x,y) + (f\star f)(x,y) + (g\star f)(x+a,y) + (f\star g)(x-a,y)$$

The last two terms are the cross-correlations, displaced to positions $\pm a$ from the center. A bright spot at $(a, 0)$ indicates a match between $g$ and $f$.

**Advantage over VanderLugt**: No holographic recording required. Both images are placed in the input plane simultaneously; the processing is entirely electronic-free until the output detection.

### The VanderLugt Correlator

The VanderLugt approach pre-records the filter $H = \hat{f}^*$ (the complex conjugate Fourier transform of the reference) as a hologram at the Fourier plane. The filter is then placed back at the Fourier plane of the 4f system, and only the query image $g$ is placed at the input. The output is:

$$U_{\text{out}}(x,y) = \mathcal{F}^{-1}\{\hat{g} \cdot \hat{f}^*\} = (f \star g)(x,y)$$

The peak intensity in the correlation output:
$$|U_{\text{out}}(0,0)|^2 = \left|\int\int g(x,y) f^*(x,y) dx\,dy\right|^2$$

This is the squared inner product of $g$ and $f$ — maximized when $g = f$, vanishing when they are orthogonal.

---

## 11.1.3.2 Performance Analysis

### Speed Comparison with Digital FFT

**Digital 2D FFT** on a modern CPU (Intel Core i9-13900K at 5.6 GHz, AVX-512):
- $N \times N$ real FFT: $\sim N^2 \log_2 N$ multiply-add operations
- For $N = 1024$: $\sim 10^7$ operations
- Peak throughput: $\sim 2 \times 10^{12}$ FLOPS/core
- Time: $\sim 10^7 / (2\times10^{12}) \sim 5$ μs for a single 1024×1024 2D FFT

**Optical 4f processor with SLM**:
- The optical processing itself: $L_{\text{4f}}/c \approx (4 \times 0.1 \text{ m}) / (3\times10^8 \text{ m/s}) \approx 1.3$ ns
- Input encoding (writing the image to the SLM): ~5–10 ms (SLM frame rate 60–180 Hz)
- Output detection (camera readout): ~1–33 ms (30–1000 fps camera)

The optical processing is nanoseconds. The input/output is milliseconds. **The bottleneck is not the optical processing — it's the data conversion.**

For a single 2D FFT, the digital CPU is 1000× faster end-to-end (5 μs vs. 5 ms). The optical system becomes advantageous only when:

1. **Batch mode**: Many queries use the same filter, allowing amortization of the filter encoding cost. For $B$ queries:
   - Optical: $B \times 1.3 \text{ ns}$ (processing) + 10 ms (filter encoding, amortized)
   - Digital: $B \times 5 \text{ μs}$
   - Breakeven: $B = 10 \text{ ms} / (5\text{ μs} - 1.3\text{ ns}) \approx 2000$ queries

2. **High-resolution inputs**: For $N \times N$ inputs with large $N$, digital FFT scales as $N^2 \log N$ while optical scales as $N^2$ (through its SBP). For $N = 10^4$: digital $\sim 1.3 \times 10^{10}$ operations, optical $\sim N^2/c =$ same SBP. The optical advantage grows as $N$ exceeds the digital processor's cache.

3. **Analog input**: If the input is inherently analog (e.g., a camera image at frame rate), the optical system avoids the ADC step.

### Practical Applications Where Optical Correlators Win

**Fingerprint identification (historical)**: The FBI and commercial systems used optical correlators for fingerprint matching in the 1980s–1990s, achieving $\sim 10^4$ matches/second per system before digital FFT processors became fast enough. Some systems used arrays of 4f processors in parallel.

**LADAR (Laser Detection and Ranging)**: Range profiles from LADAR systems have high bandwidth (GHz) that exceeds electronic ADC rates. Photonic matched filtering at the IF frequency identifies range returns without full digitization. Still used in some military LADAR systems.

**Optical coherence tomography (OCT)**: The cross-correlation between the reference arm and sample arm in OCT is inherently computed optically by the spectrometer. This is a legitimate application where optics computes the correlation as part of the physics of the measurement.

**Real-time video rate processing**: For applications that require processing at video rates (30–120 fps) on very large images ($N > 10^4$ pixels) where the digital FFT would be too slow or power-hungry, optical correlators can provide a power-efficient alternative. An active area as of 2024 is using optical correlators for autonomous vehicle LiDAR processing.

---

## 11.1.3.3 Why Optical Logic Won and Then Lost

### The 1960s-1980s Context

In the 1960s and 1970s, optical correlators were genuinely faster than digital alternatives for large-scale pattern matching. The SBP of an optical system (10^8 pixels) far exceeded the capability of digital computers ($\sim 10^5$ pixels per second for a 1970s minicomputer). Optical computing had a real, measurable advantage.

The development of the FFT algorithm (Cooley-Tukey, 1965 [2]) and the exponential scaling of VLSI made digital processing increasingly competitive through the 1980s. By 1990, digital FFT processors could match optical correlators for typical image sizes; by 2000, they exceeded them.

The reason optical computing lost is instructive: **the FFT made the Fourier transform algorithmically cheap**, collapsing the advantage of the optical implementation that did it "for free." This is a general lesson: optical computing advantages can be undermined by algorithmic advances in digital computing.

The analog computing revival of the 2010s in the form of photonic neural networks (MZI meshes, diffractive layers) faces the same threat: hardware improvements in GPU matrix multiplication may reduce the performance gap that motivates optical alternatives. We will return to this comparison quantitatively in Chapter 12.

---

## References

[1] Weaver, C.S., & Goodman, J.W. (1966). "A technique for optically convolving two functions." *Applied Optics*, 5(7), 1248–1249. [Original Joint Transform Correlator paper.]

[2] Cooley, J.W., & Tukey, J.W. (1965). "An algorithm for the machine calculation of complex Fourier series." *Mathematics of Computation*, 19(90), 297–301. [The FFT paper; significantly reduced the computational cost of digital Fourier transforms and undermined the optical correlator's competitive advantage.]

[3] Goodman, J.W., Dias, A.R., & Woody, L.M. (1978). "Fully parallel, high-speed incoherent optical method for performing discrete Fourier transforms." *Optics Letters*, 2(1), 1–3. [Important contribution to incoherent optical computing; Goodman demonstrating the feasibility of parallel optical processing.]

[4] Psaltis, D. (2006). "Optics in the 21st century." *Science*, 314(5798), 419–420. [Psaltis' perspective on the future of optical information processing after the failure of optical logic; discusses what optical computing is genuinely good for.]
