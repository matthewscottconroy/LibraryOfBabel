# Chapter 9: Important Concepts

## 1. Shannon Entropy
$H(X) = -\sum_i p_i\log_2 p_i$ bits/symbol. Measures the average information content (surprise) of a random variable. Maximum $\log_2 n$ for $n$ equally likely outcomes. Zero when outcome is certain. The fundamental unit of information.

## 2. Channel Capacity
$C = \max_{p(x)} I(X;Y)$ bits per channel use. The maximum reliable transmission rate over any noisy channel. Shannon's theorem: reliable communication is possible iff $R < C$. Impossible for $R > C$, regardless of code complexity.

## 3. Shannon-Hartley Theorem
$C = B\log_2(1 + \text{SNR})$ bits/s. Capacity of the AWGN channel. Doubling bandwidth doubles capacity; doubling power adds only ~1 bit/Hz at high SNR. The regime where bandwidth is more valuable than power.

## 4. Optical Channel OSNR and Capacity
In EDFA-amplified fiber, OSNR degrades with number of spans as $\text{OSNR} \approx P_{\text{launch}}/(N F_n \hbar\omega\Delta\nu)$. Current long-haul systems (20 spans × 80 km) achieve OSNR ≈ 20–22 dB, supporting DP-16QAM at spectral efficiency ~7 bits/s/Hz, within 2–3 dB of the nonlinear Shannon limit.

## 5. Modulation Formats and Shannon Gap
| Format | SE (bits/s/Hz) | Required SNR (dB) | Shannon gap |
|--------|-----------------|-------------------|-------------|
| OOK | 1 | 14 | Large |
| PAM4 | 2 | 20 | Large |
| DP-QPSK | 4 | 10 | ~6 dB |
| DP-16QAM | 8 | 16 | ~7 dB |
| DP-64QAM+PCS | 10–11 | 25 | ~2 dB |

## 6. WDM Channel Plan
ITU DWDM grid anchored at 193.1 THz with 50 GHz or 12.5 GHz spacing. C-band: 96 channels at 50 GHz, or up to 384 channels at 12.5 GHz flex-grid. C+L band: ~10 THz total usable bandwidth; 192+ channels supporting >100 Tbps per fiber pair.

## 7. FEC Net Coding Gain
7% HD-FEC: NCG ≈ 6 dB; 20% SD-LDPC FEC: NCG ≈ 9–10 dB. FEC allows operating at BER $10^{-3}$–$10^{-4}$ pre-FEC to achieve $10^{-15}$ post-FEC BER. Essential for operating near Shannon capacity.

## 8. Probabilistic Shaping
Transmitting inner QAM constellation points more frequently (Maxwell-Boltzmann distribution) reduces required SNR by ~1–1.5 dB and enables variable spectral efficiency from the same constellation. DP-64QAM with PCS achieves 8–11 bits/s/Hz within ~2 dB of Shannon limit.
