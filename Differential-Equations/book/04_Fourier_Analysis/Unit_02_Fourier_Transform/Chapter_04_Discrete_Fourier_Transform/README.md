# Chapter 04: The Discrete Fourier Transform

In practice, signals are measured at a finite number of equally spaced points. A microphone records audio at 44,100 samples per second; a weather station logs temperature every hour; a CT scanner acquires a finite number of projections. In each case, the relevant mathematical object is not a function on $\mathbb{R}$ but a finite sequence of numbers. The appropriate version of Fourier analysis for finite sequences is the **Discrete Fourier Transform (DFT)**.

The DFT maps an $N$-element sequence to another $N$-element sequence, with exactly the same information content. Its importance in applications is enormous, driven by the **Fast Fourier Transform (FFT)** algorithm, which computes the DFT in $O(N\log N)$ operations instead of the naive $O(N^2)$.

## Chapter Overview

**Section 01: DFT Definition** introduces the DFT formally. Given a sequence $(x_0, x_1, \ldots, x_{N-1}) \in \mathbb{C}^N$, the DFT produces the sequence $(X_0, X_1, \ldots, X_{N-1})$ where
$$X_k = \sum_{n=0}^{N-1} x_n\,\omega_N^{-kn}, \quad \omega_N = e^{2\pi i/N}.$$
The $k$-th output $X_k$ measures the amplitude and phase of the frequency component at (angular) frequency $k \cdot 2\pi/N$. The inverse DFT recovers the original sequence: $x_n = \frac{1}{N}\sum_{k=0}^{N-1}X_k\omega_N^{kn}$. The DFT is a bijection on $\mathbb{C}^N$ that is, up to normalization, a unitary transformation.

**Section 02: Fast Fourier Transform** presents the Cooley-Tukey FFT algorithm, which computes the DFT in $O(N\log N)$ time for $N$ a power of $2$. The key idea is the **divide-and-conquer** splitting of the $N$-point DFT into two $(N/2)$-point DFTs using the periodicity and symmetry of the complex exponentials. This recursion reduces the $O(N^2)$ computation to $O(N\log N)$, which for $N = 10^6$ represents a speedup factor of $\sim 50{,}000$. The FFT is often described as one of the most influential algorithms of the 20th century.

**Section 03: Applications to Signal Processing** covers the principal practical uses of the DFT: spectral analysis (identifying frequency components of a signal), digital filtering (applying a frequency-domain filter by multiplication), fast convolution (using the FFT to compute convolutions in $O(N\log N)$ instead of $O(N^2)$), and spectral estimation. The relationship between the DFT and the continuous Fourier transform is made precise through the sampling theorem (Nyquist-Shannon), which characterizes exactly when a continuous signal can be recovered from its samples.

## Why This Chapter Matters

The DFT and FFT are not just approximation schemes for the continuous Fourier transform. They are mathematically complete and exact in their own right: the DFT of a finite sequence is an exact, invertible transformation, with its own algebraic theory (circular convolution, frequency wrapping, etc.). The connection to the continuous theory provides interpretive guidance (the $k$-th DFT output corresponds to a certain frequency), but the DFT stands independently as the Fourier theory of finite sequences.

In applications, essentially all Fourier computation happens via the FFT. Whether one is computing the frequency spectrum of an audio signal, solving a PDE by spectral methods, compressing an image, or multiplying large integers, the FFT is the enabling technology.
