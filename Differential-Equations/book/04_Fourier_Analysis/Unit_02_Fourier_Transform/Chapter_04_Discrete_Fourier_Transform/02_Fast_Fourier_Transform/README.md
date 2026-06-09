# The Fast Fourier Transform

Computing the DFT of an $N$-element sequence naively requires $N$ multiplications and $N-1$ additions for each of $N$ outputs, giving $O(N^2)$ total operations. For $N = 10^6$ (a one-second audio clip at 44,100 Hz rounded up), this is $10^{12}$ operations — several minutes even on fast hardware. The Fast Fourier Transform (FFT) computes the same result in $O(N\log N)$ operations: about $2\times 10^7$ for the same $N$, roughly 50,000 times faster. This difference has made computationally intensive Fourier analysis practical, and the FFT is widely considered one of the most important algorithms of the 20th century.

## The Key Idea: Divide and Conquer

For $N = 2^m$ (a power of 2), the Cooley-Tukey FFT splits the $N$-point DFT into two $(N/2)$-point DFTs and combines them efficiently. The key algebraic identity is:

Split the sequence $x_0, x_1, \ldots, x_{N-1}$ into even-indexed and odd-indexed terms:
$$X_k = \sum_{n=0}^{N-1} x_n \omega_N^{-kn} = \sum_{j=0}^{N/2-1} x_{2j}\omega_N^{-k(2j)} + \sum_{j=0}^{N/2-1} x_{2j+1}\omega_N^{-k(2j+1)}.$$

Since $\omega_N^{-k(2j)} = \omega_{N/2}^{-kj}$ (because $e^{-2\pi i k(2j)/N} = e^{-2\pi ikj/(N/2)}$), define:
$$E_k = \sum_{j=0}^{N/2-1} x_{2j}\,\omega_{N/2}^{-kj}, \quad O_k = \sum_{j=0}^{N/2-1} x_{2j+1}\,\omega_{N/2}^{-kj}.$$
These are the $(N/2)$-point DFTs of the even and odd subsequences, respectively. Then
$$X_k = E_k + \omega_N^{-k}\,O_k.$$
Since $E_k$ and $O_k$ have period $N/2$ in $k$, we also get $X_{k + N/2} = E_k - \omega_N^{-k}O_k$. This is the **butterfly operation**.

## The Butterfly Operation

The two formulas $X_k = E_k + \omega_N^{-k}O_k$ and $X_{k+N/2} = E_k - \omega_N^{-k}O_k$ together compute two outputs from two inputs $(E_k, O_k)$ using one complex multiplication (by $\omega_N^{-k}$) and two additions. This is called a **butterfly** because of the shape of its signal flow graph.

The total work for the $N$-point DFT is: 2 transforms of size $N/2$, plus $N/2$ butterflies (each costing 1 multiply and 2 adds). If $T(N)$ is the number of operations:
$$T(N) = 2T(N/2) + \frac{N}{2}.$$
Solving this recurrence (with $T(1) = 0$): $T(N) = \frac{N}{2}\log_2 N$. Including the constant factors, the total complex multiplications is $\frac{N}{2}\log_2 N$ and additions is $N\log_2 N$.

## Recursive Structure

The FFT is applied recursively: each $(N/2)$-point DFT is itself split into two $(N/4)$-point DFTs, and so on, until we reach the base case of $N = 1$ (trivial: $X_0 = x_0$). The total depth of recursion is $\log_2 N$, and at each level there are $N/2$ butterfly operations.

**Bit-Reversal Permutation.** The recursive splitting reorders the input: after all $\log_2 N$ levels of splitting, the input is in **bit-reversed order**. If the original index $n$ has binary representation $n = b_{m-1}\ldots b_1 b_0$, it ends up at position $b_0 b_1\ldots b_{m-1}$ (the bits reversed). Most FFT implementations perform this bit-reversal permutation first, then proceed with the butterfly operations in-place.

## Worked Example: $N = 8$

For $N = 8$ ($m = 3$ levels), the FFT computes:
- **Level 1:** 4 butterflies (combining pairs into size-2 DFTs).
- **Level 2:** 4 butterflies with twiddle factors $\omega_4^k = i^{-k}$ (combining into size-4 DFTs).
- **Level 3:** 4 butterflies with twiddle factors $\omega_8^k$ (combining into size-8 DFT).

Total: $4 \times 3 = 12$ butterflies, versus $8^2/2 = 32$ multiplications naively. The speedup factor grows as $\log_2 N / (N/2) \to 0$ as $N \to \infty$ — the FFT becomes asymptotically much faster.

## Variants and Generalizations

**Arbitrary $N$:** The FFT can handle $N$ not a power of 2 using mixed-radix factorization: if $N = pq$, split into $p$ DFTs of size $q$ and $q$ DFTs of size $p$. For prime $N$, other algorithms (Bluestein's chirp-Z algorithm) still achieve $O(N\log N)$.

**Radix-4 and split-radix FFT:** Instead of splitting $N$ into two halves, split into four quarters. This reduces the constant in the $O(N\log N)$ bound.

**Multidimensional FFT:** For a 2D array of size $M\times N$, apply 1D FFTs to each row, then to each column (or vice versa). Total operations: $O(MN\log(MN))$. Essential for image processing.

**Real-input FFT:** If the input is real, the output has Hermitian symmetry $X_{N-k} = \overline{X_k}$. Exploiting this halves the computation.

**Number-theoretic transform (NTT):** Replace complex roots of unity with roots of unity modulo a prime. Enables exact arithmetic and is used in cryptography and integer multiplication.

## Historical Note

The FFT algorithm was famously "rediscovered" by Cooley and Tukey in 1965, leading to its widespread adoption. Historical research has since shown that Gauss knew an equivalent algorithm around 1805. The algorithm was also known to Runge, Danielson, and Lanczos in various forms. But it was Cooley and Tukey's 1965 paper in Mathematics of Computation that triggered the explosion of applications, coinciding with the spread of digital computers capable of exploiting the speedup.

## Computational Complexity

| Method | Multiplications | Additions |
|---|---|---|
| Direct DFT | $N^2$ | $N(N-1)$ |
| Radix-2 FFT | $\frac{N}{2}\log_2 N$ | $N\log_2 N$ |
| Split-radix FFT | $\frac{N}{3}\log_2 N$ (approx.) | $N\log_2 N$ |

For $N = 1024$: direct requires $\sim 10^6$ operations; FFT requires $\sim 5000$. For $N = 10^6$: direct needs $\sim 10^{12}$; FFT needs $\sim 2\times 10^7$. The difference is not merely quantitative but qualitative: it determines whether an application is feasible in real time.
