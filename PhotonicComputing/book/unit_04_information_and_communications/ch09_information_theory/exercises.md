# Chapter 9: Exercises

## Mathematical

**M9.1** — Calculate the entropy of a source with symbols at probabilities $\{0.5, 0.25, 0.125, 0.125\}$. Show that a Huffman code achieves average length equal to the entropy. Compare to a fixed-length 2-bit code.

**M9.2** — For a single-mode optical fiber with 96 WDM channels at 50 GHz spacing, each with an OSNR of 22 dB:
(a) Calculate the capacity per channel using Shannon-Hartley.
(b) Calculate the total fiber capacity.
(c) Compare to the total bandwidth of the C-band (4.4 THz).

**M9.3** — Derive the OSNR accumulation formula for $N$ cascaded EDFAs, each with gain $G$, noise figure $F_n$, and input power $P$. Show that OSNR $\propto 1/N$.

**M9.4** — For DP-16QAM at 32 GBaud with 20% FEC overhead: calculate (a) the gross symbol rate, (b) the information bit rate, (c) the spectral efficiency after FEC, (d) the Shannon limit for the required SNR of 16 dB.

## Conceptual

**C9.1** — Why does probabilistic shaping reduce the Shannon gap? Explain in terms of the match between the signal statistics and the optimal Gaussian distribution.

**C9.2** — Why does the Shannon capacity of an optical fiber decrease at very high launch power? What physical phenomenon causes this, and how does it differ from the linear noise model?

**C9.3** — A data center operator must choose between 400G per wavelength (32 GBaud DP-16QAM) at 50 GHz channel spacing and 100G per wavelength (10 GBaud DP-QPSK) at 50 GHz spacing. The fiber has OSNR = 20 dB. Which gives higher total throughput? Which is more energy-efficient?

## Lab

**L9.1 (Python)** — Implement the binary entropy function $H_b(p)$ and plot it for $p \in [0,1]$. Verify that it peaks at 1 bit for $p = 0.5$. For a binary symmetric channel with crossover probability $p = 0.01$, calculate the channel capacity and compare to the rate of a 1% overhead FEC code.

**L9.2 (Python)** — Simulate OOK and QPSK modulation over an AWGN channel. For each format, plot BER vs. SNR curves and identify the SNR required for BER = $10^{-3}$. Overlay the Shannon capacity limit.
