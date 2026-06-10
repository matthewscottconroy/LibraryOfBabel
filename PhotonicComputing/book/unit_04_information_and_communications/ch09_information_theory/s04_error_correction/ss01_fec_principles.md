# 9.4.1 FEC Principles and Net Coding Gain

## The Rate-Distance Tradeoff

An FEC code with rate $R = k/n$ takes $k$ information bits and produces $n$ coded bits ($n > k$), with redundancy $(n-k)/n = 1-R$. The overhead is $(n-k)/k = 1/R - 1$.

The minimum Hamming distance $d_{\min}$ of a code is the smallest number of bit differences between any two codewords. A code with distance $d_{\min}$ can:
- **Detect** up to $d_{\min} - 1$ errors
- **Correct** up to $\lfloor(d_{\min}-1)/2\rfloor$ errors

The **Hamming bound** limits what codes can achieve:

$$2^k \times \sum_{i=0}^t \binom{n}{i} \leq 2^n, \quad t = \lfloor(d_{\min}-1)/2\rfloor$$

This says the number of codewords times the number of error patterns each can correct cannot exceed the total number of $n$-bit strings. Codes that meet the Hamming bound with equality are called **perfect codes**.

For optical communications, the FEC overhead is typically 7–25%:
- **Hard-decision FEC (HD-FEC)** at 7% overhead: Rate 0.935, used in legacy systems
- **Soft-decision FEC (SD-FEC)** at 20% overhead: Rate 0.833, state-of-art coherent
- **Ultra-low overhead FEC** at 5%: For energy-constrained systems

## Net Coding Gain

The **net coding gain (NCG)** is the reduction in required SNR (in dB) achieved by using FEC, accounting for the reduced efficiency from FEC overhead. For a target BER after decoding:

$$\text{NCG} = \underbrace{\text{SNR gain from coding}}_{\text{raw coding gain}} - \underbrace{10\log_{10}(1/(1-r_{\text{FEC}}))}_{\text{overhead penalty}}$$

For 7% HD-FEC at BER = $10^{-15}$: NCG ≈ 6–7 dB (relative to uncoded at BER = $10^{-15}$). This means a system using FEC needs 7 dB less OSNR to achieve $10^{-15}$ BER — but the FEC operates at the pre-FEC threshold of $\sim 10^{-3}$ BER (the threshold at which the decoder converges).

For 20% SD-FEC with LDPC: NCG ≈ 9–10 dB. The additional 3 dB over HD-FEC comes from soft-decision decoding, which uses the full analog likelihood of each bit rather than a hard 0/1 decision.

## Hard vs. Soft Decision Decoding

**Hard-decision (HD)** decoding makes a binary decision (0 or 1) on each bit before the FEC decoder, discarding the confidence information. Simple and fast; traditionally implemented in hardware with few gates.

**Soft-decision (SD)** decoding passes log-likelihood ratios (LLRs) to the decoder: $L = \log(P(\text{bit}=0)/P(\text{bit}=1))$. Large positive $L$: confident 0; large negative $L$: confident 1; $L \approx 0$: uncertain. The decoder exploits this uncertainty information to achieve ~2–3 dB better performance than HD at the same code rate.

Modern coherent systems universally use SD-FEC because the DSP already computes soft output from the complex received field; it costs minimal additional computation to pass LLRs to the FEC decoder rather than hard bits.

---

## References

[1] Lin, S. & Costello, D.J. (2004). *Error Control Coding*, 2nd ed. Pearson Prentice-Hall. [The comprehensive textbook on error-correcting codes; Hamming bound, linear codes, and hard-decision decoding.]

[2] Alvarado, A., Agrell, E., Lavery, D., Maher, R., & Bayvel, P. (2015). "Replacing the soft-decision FEC limit paradigm in the design of optical communication systems." *Journal of Lightwave Technology*, 33(20), 4338–4352. [The clarification of FEC threshold paradigms for optical systems; NCG calculations.]
