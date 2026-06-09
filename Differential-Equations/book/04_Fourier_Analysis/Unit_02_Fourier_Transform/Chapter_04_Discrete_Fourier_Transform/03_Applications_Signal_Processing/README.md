# Applications to Signal Processing

The DFT and FFT are not mathematical curiosities but the computational engines underlying an enormous range of practical technologies: audio compression (MP3), image compression (JPEG), wireless communication (OFDM in 4G/5G), radar, MRI imaging, seismic analysis, and numerical PDE solvers. This section surveys the principal applications, showing how the algebraic properties of the DFT translate into practical algorithms.

## Spectral Analysis

Given a measured signal $(x_0, \ldots, x_{N-1})$, computing its DFT $(X_0, \ldots, X_{N-1})$ reveals the frequency content. The **power spectrum** $P_k = |X_k|^2/N$ gives the power at each discrete frequency $k/N$ (in cycles per sample). Peaks in the power spectrum identify dominant frequencies in the signal.

**Example.** Audio signal recorded at sampling rate $f_s = 44100$ Hz for $N = 4096$ samples. The DFT has $N/2 + 1 = 2049$ independent frequencies (for real input), ranging from $0$ to $f_s/2 = 22050$ Hz with resolution $\Delta f = f_s/N \approx 10.8$ Hz per bin. A peak at bin $k = 100$ corresponds to frequency $100 \times 44100/4096 \approx 1076$ Hz, which is approximately $C_6$ (the C two octaves above middle C).

## Digital Filtering

A **digital filter** multiplies the DFT of the input by a frequency-domain mask $H_k$ and then applies the IDFT. The output is the circular convolution of the input with the filter's impulse response.

More precisely: to apply a filter with transfer function $H_k$ to input $\mathbf{x}$:
1. Compute $X_k = \text{DFT}(\mathbf{x})$ in $O(N\log N)$.
2. Multiply: $Y_k = H_k\cdot X_k$ in $O(N)$.
3. Compute $\mathbf{y} = \text{IDFT}(\mathbf{Y})$ in $O(N\log N)$.

Total: $O(N\log N)$, versus $O(N^2)$ for direct convolution with the impulse response.

**Low-pass filter:** $H_k = 1$ for $|k| \leq k_0$ and $H_k = 0$ for $|k| > k_0$. Sets high-frequency components to zero, smoothing the signal.

**High-pass filter:** $H_k = 1 - L_k$ where $L_k$ is a low-pass mask. Retains high-frequency detail.

**Band-pass filter:** Nonzero only in a range $[k_1, k_2]$ of frequencies.

## Fast Convolution

The circular convolution theorem says $\text{DFT}[x \circledast y] = \text{DFT}[x]\cdot\text{DFT}[y]$. This is used to compute **linear** (not circular) convolution via the technique of zero-padding:

To convolve length-$M$ sequence $x$ with length-$L$ sequence $y$ (producing length $M+L-1$ output):
1. Zero-pad both to length $N \geq M + L - 1$.
2. Compute $X = \text{FFT}(x_\text{padded})$ and $Y = \text{FFT}(y_\text{padded})$.
3. Compute $Z_k = X_k Y_k$.
4. Output $z = \text{IFFT}(Z)$ (first $M+L-1$ terms).

Complexity: $O(N\log N)$ versus $O(ML)$ for direct convolution.

## The Sampling Theorem

The **Nyquist-Shannon sampling theorem** is the bridge between continuous and discrete Fourier analysis:

**Theorem.** A bandlimited signal $f$ — one whose Fourier transform satisfies $\hat{f}(\xi) = 0$ for $|\xi| > B$ — is completely determined by its samples at any rate $f_s > 2B$ (the Nyquist rate). The reconstruction formula is
$$f(t) = \sum_{n=-\infty}^\infty f(n/f_s)\,\text{sinc}(f_s t - n).$$

The Nyquist frequency $B_{\max} = f_s/2$ is the highest frequency that can be represented without aliasing. Frequencies above $B_{\max}$ in the sampled signal fold back (alias) to lower frequencies: a sinusoid at frequency $f_s/2 + \Delta$ is indistinguishable from one at $f_s/2 - \Delta$.

In practice, signals are passed through a **anti-aliasing filter** (low-pass filter with cutoff $f_s/2$) before sampling, to prevent high-frequency components from aliasing.

## Spectrogram and Time-Frequency Analysis

A single DFT gives frequency information averaged over the entire signal. To track how frequency content changes over time, the **Short-Time Fourier Transform (STFT)** applies the DFT to overlapping windowed segments:
$$S(n, k) = \sum_{m=0}^{M-1} x_{n+m}\,w_m\,e^{-2\pi ikm/M},$$
where $w_m$ is a window function (e.g., Hann: $w_m = \sin^2(\pi m/M)$) of length $M$.

Displaying $|S(n,k)|^2$ as an image (horizontal axis: time $n$, vertical axis: frequency $k$) produces a **spectrogram**, a visual time-frequency representation of the signal. Spectrograms are used in speech recognition, music analysis, and biomedical signal processing.

The window length $M$ creates a time-frequency tradeoff: long windows give fine frequency resolution but poor time resolution; short windows give fine time resolution but poor frequency resolution. This is the discrete uncertainty principle.

## Numerical PDE: Spectral Methods

The DFT enables **spectral methods** for solving PDEs on periodic domains. For the heat equation $u_t = u_{xx}$ on $[0, 2\pi]$ with periodic boundary conditions:

1. Represent $u(\cdot, t)$ by its DFT: $u(x,t) \approx \sum_k \hat{u}_k(t)e^{ikx}$.
2. The PDE becomes $\dot{\hat{u}}_k = -k^2\hat{u}_k$ for each $k$.
3. Exact solution: $\hat{u}_k(t) = \hat{u}_k(0)e^{-k^2 t}$.
4. At each time step, apply IFFT to recover $u(\cdot, t)$.

Spectral methods achieve **exponential accuracy** for smooth periodic functions (the Fourier coefficients of smooth functions decay exponentially, so truncating at frequency $N/2$ introduces only exponentially small error). They are preferred for problems requiring high accuracy with moderate $N$.

## Compression

**JPEG image compression** exploits the DFT's energy compaction property. A typical natural image has most of its energy in low-frequency DFT components. By retaining only the largest DFT coefficients and discarding the small ones, significant compression is achieved with minimal perceptual quality loss.

More precisely, JPEG applies the Discrete Cosine Transform (DCT — a variant of the DFT for real signals) to $8\times 8$ blocks of pixels, quantizes the coefficients, and entropy-codes the result. The quantization step is where compression occurs: small coefficients are rounded to zero, and the surviving nonzero coefficients are stored.

The human visual system is less sensitive to high-frequency components (fine texture details), so aggressive quantization of those coefficients produces little perceptual degradation. This is why heavily compressed JPEG images show "blocking" artifacts at block boundaries (where discarded high-frequency information would have smoothed the transitions) rather than uniform degradation.
