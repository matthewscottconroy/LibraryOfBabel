# 9.4.2 LDPC Codes and Polar Codes

## Low-Density Parity-Check (LDPC) Codes

LDPC codes, invented by Gallager in 1960 [1] and rediscovered in the 1990s, are linear block codes defined by a sparse parity-check matrix $H$. An $n$-bit codeword $\mathbf{c}$ satisfies $H\mathbf{c} = \mathbf{0}$ (mod 2). The "low-density" property means $H$ has very few 1s — typically $d_v = 3$–6 ones per column (variable node degree) and $d_c = 6$–15 ones per row (check node degree).

LDPC codes achieve near-Shannon-limit performance with a practical iterative decoding algorithm: **belief propagation** (sum-product algorithm). Messages (log-likelihood ratios) are passed between variable nodes (bits) and check nodes (parity constraints) in a bipartite factor graph:

**Iterative decoding steps**:
1. Initialize: $L_v = L_{ch}$ (channel LLR for each bit)
2. Check-to-variable messages: $\Delta_{c\to v} = 2\text{atanh}\left(\prod_{v' \in N(c)\setminus v}\tanh(L_{v'}/2)\right)$
3. Variable-to-check messages: $L_{v\to c} = L_{ch} + \sum_{c' \in N(v)\setminus c}\Delta_{c'\to v}$
4. Posterior LLR: $L_v = L_{ch} + \sum_{c \in N(v)}\Delta_{c\to v}$
5. Hard decision: bit = 1 if $L_v < 0$; check syndrome; iterate if needed

For a typical LDPC code with rate 0.8 and $n = 64,800$ bits (DVB-S2 standard [2]):
- ~50 decoding iterations needed for convergence
- BER performance within ~0.5 dB of Shannon limit at the waterfall region
- Decoding complexity: $O(n \cdot I)$ where $I$ is iteration count
- Throughput: With pipelined hardware decoder at 800 MHz, ~1 Tbps decoding possible

LDPC codes are now standard in optical coherent transceivers (OIF 400ZR, OpenZR+), 5G cellular (eMBB data channels), WiFi (802.11n/ac/ax), and satellite communications (DVB-S2).

## Polar Codes

Polar codes, invented by Erdal Arıkan in 2009 [3], are the first class of codes proven to *provably* achieve the Shannon capacity with an explicit construction. The construction relies on "channel polarization": combining $N = 2^n$ copies of a noisy channel transforms them into a set of "good" (reliable) synthetic channels and "bad" (unreliable) synthetic channels. Information bits are sent over the good channels; the bad channels are "frozen" (set to known values).

**Construction**: The $N \times N$ generator matrix is $G_N = B_N F^{\otimes n}$ where $F = \begin{pmatrix}1&0\\1&1\end{pmatrix}$ and $B_N$ is a bit-reversal permutation.

**Decoding**: Successive cancellation (SC) decoding has complexity $O(N\log N)$ — the same as an FFT. SC-List decoding (maintaining $L$ candidate paths) achieves performance comparable to LDPC at similar rate.

Polar codes were selected for the 5G NR (New Radio) standard for control channels. For optical communications, polar codes are an active research area; their proven capacity-achieving property makes them theoretically attractive, but their block length requirements ($N = 2^n$, currently up to $2^{20}$) and SC-List decoding complexity present implementation challenges.

## FEC Complexity for Photonic Computing

The decoding complexity of LDPC belief propagation is dominated by the sum-product update:

$$\Delta_{c\to v} = 2\text{atanh}\left(\prod\tanh(L/2)\right)$$

This involves hyperbolic tangent functions — fundamentally nonlinear operations. Electronic LDPC decoders implement these in lookup tables or min-sum approximations; the operations are highly parallel (each check and variable node updates independently) and amenable to custom VLSI.

A photonic decoder would need to implement the same operations. The non-linearity requirement is the barrier: as Section 8.2.3 noted, passive photonic systems can only implement linear transformations. Nonlinear photonic elements (saturable absorbers, bistable resonators) exist but are not yet competitive with CMOS for complex logic at the scale needed for LDPC decoding.

---

## References

[1] Gallager, R.G. (1962). "Low-density parity-check codes." *IRE Transactions on Information Theory*, 8(1), 21–28. [The original LDPC paper. Largely ignored until rediscovered in 1995.]

[2] ETSI EN 302 307-1. *Digital Video Broadcasting (DVB); Second generation framing structure, channel coding and modulation systems for Broadcasting, Interactive Services, News Gathering and other broadband satellite applications.* [The DVB-S2 standard that made LDPC codes mainstream.]

[3] Arıkan, E. (2009). "Channel polarization: A method for constructing capacity-achieving codes for symmetric binary-input memoryless channels." *IEEE Transactions on Information Theory*, 55(7), 3051–3073. [The polar code paper; one of the most important coding theory results of the 21st century.]
