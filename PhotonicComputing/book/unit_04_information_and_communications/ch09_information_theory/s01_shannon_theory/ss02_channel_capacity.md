# 9.1.2 Channel Capacity and the Shannon-Hartley Theorem

## The Additive White Gaussian Noise Channel

The most important channel model in communications is the **additive white Gaussian noise (AWGN) channel**:

$$Y = X + Z$$

where $X$ is the transmitted signal, $Z \sim \mathcal{N}(0, N)$ is Gaussian noise with variance $N$, and $Y$ is the received signal. The constraint on the input is a power constraint: $E[X^2] \leq S$.

The capacity of the AWGN channel is:

$$C = \max_{p(x): E[X^2]\leq S} I(X; Y)$$

To compute this, we use the fact that $I(X;Y) = h(Y) - h(Y|X) = h(Y) - h(Z)$, since the noise is independent of $X$. The noise entropy $h(Z) = \frac{1}{2}\log_2(2\pi e N)$ is fixed. To maximize $I(X;Y)$, we maximize $h(Y)$. Subject to $E[Y^2] = E[(X+Z)^2] = S + N$, the maximum differential entropy of $Y$ is achieved by Gaussian $Y$, which requires Gaussian $X$.

For Gaussian $X$ with variance $S$:

$$h(Y) = \frac{1}{2}\log_2(2\pi e(S+N))$$

Therefore:

$$C = h(Y) - h(Z) = \frac{1}{2}\log_2(2\pi e(S+N)) - \frac{1}{2}\log_2(2\pi eN) = \frac{1}{2}\log_2\left(1 + \frac{S}{N}\right)$$

This is the capacity in bits per channel use. For a channel with bandwidth $B$ (Hz), Nyquist sampling requires 2$B$ complex samples per second, and the capacity in bits per second is:

$$\boxed{C = B\log_2\left(1 + \frac{S}{N}\right) \text{ bits/s}}$$

This is the **Shannon-Hartley theorem** [1].

## Interpreting the Shannon-Hartley Theorem

The Shannon-Hartley theorem has several consequences that are not immediately obvious:

**Bandwidth and SNR are not equivalent**: Doubling the bandwidth doubles the capacity, but doubling the SNR only adds $\log_2(1 + 2S/N)$ bits, which for large SNR is approximately 1 bit. At SNR = 100 (20 dB): doubling bandwidth doubles capacity; doubling SNR from 100 to 200 adds only $\log_2(200/101) \approx 1$ bit/Hz. This is why bandwidth is more valuable than SNR in high-capacity systems.

**The capacity does not go to zero with low SNR**: Even for very low SNR ($S/N \ll 1$), $C \approx (B/\ln 2)(S/N)$ — capacity scales linearly with SNR. This means it is always possible to communicate reliably, just slowly.

**The energy per bit**: The minimum energy per bit for reliable communication approaches $E_b/N_0 = -1.59$ dB (the Shannon limit) as the rate approaches zero. Any modulation scheme that achieves this energy efficiency operates near the Shannon limit.

## From Channel Uses to Bits Per Second

A "channel use" is one transmission of a symbol through the channel. If each symbol takes time $T = 1/(2B)$ (Nyquist interval for bandwidth $B$), then there are $2B$ channel uses per second, giving the bit rate:

$$R = 2B \times I(X; Y)_{\text{per use}} = B\log_2(1 + \text{SNR})$$

In practice, the **spectral efficiency** $\eta = R/B$ bits/(s·Hz) is the key figure of merit:

$$\eta = \log_2(1 + \text{SNR})$$

For the SNR values typical in optical fiber systems:

| SNR (dB) | SNR (linear) | $\eta$ (bits/s/Hz) |
|---------|--------------|---------------------|
| 10 | 10 | 3.46 |
| 20 | 100 | 6.66 |
| 30 | 1000 | 9.97 |
| 40 | 10,000 | 13.3 |

State-of-art coherent optical systems (400G DP-16QAM at 32 GBaud with 7% FEC overhead) achieve spectral efficiency of ~6–7 bits/s/Hz, close to the Shannon limit for typical EDFA-noise-limited long-haul SNRs of 20–25 dB.

## The Capacity of the AWGN Channel at Very High SNR

At high SNR, $C \approx B\log_2(\text{SNR})$: capacity grows logarithmically with power. This has an important engineering implication: doubling the launched power in a fiber link gains only ~1 bit/s/Hz of spectral efficiency. The returns from increasing power diminish rapidly.

For optical fiber, there is an additional complication: the Kerr nonlinearity (Section 6.3) causes cross-phase modulation and four-wave mixing between WDM channels, and the noise from these nonlinear interactions increases with power. Beyond a certain launch power (the "optimal launch power"), nonlinear noise dominates and capacity actually decreases. This creates a nonlinear Shannon limit for optical fiber [2]:

$$C_{\text{fiber}} \approx \frac{1}{3}B\log_2\left(1 + \frac{1}{3}\frac{P_{\text{opt}}}{N_{\text{ASE}}(P_{\text{opt}})}\right)$$

where $P_{\text{opt}}$ is the optimal launch power and $N_{\text{ASE}}$ is the ASE noise power. The factor 1/3 arises from the nonlinear noise model for a WDM system with uniformly loaded channels. Current long-haul fiber systems operate within 2–3 dB of this nonlinear Shannon limit.

---

## References

[1] Hartley, R.V.L. (1928). "Transmission of information." *Bell System Technical Journal*, 7(3), 535–563. [The pre-Shannon precursor; establishes the bandwidth-capacity relationship without the noise analysis.]

[2] Essiambre, R.-J., Kramer, G., Winzer, P.J., Foschini, G.J., & Goebel, B. (2010). "Capacity limits of optical fiber networks." *Journal of Lightwave Technology*, 28(4), 662–701. [The definitive treatment of nonlinear Shannon limits for optical fiber; includes the factor-1/3 result and numerical capacity calculations.]
