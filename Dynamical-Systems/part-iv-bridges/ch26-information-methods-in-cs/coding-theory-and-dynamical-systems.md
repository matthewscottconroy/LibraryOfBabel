# 26.5 Coding Theory and Dynamical Systems

We end where we began the book — with the connection between information theory and dynamical systems — but now from the coding theory side.

Error-correcting codes are the engineering application of Shannon's channel coding theorem. Shannon (1948) proved that every channel has a capacity $C$, and that reliable communication is possible at any rate $R < C$ using sufficiently long codes. But he gave no construction — only an existence proof via random coding. The problem of *constructing* capacity-achieving codes explicitly is one of the central problems of information theory, and its solution — polar codes (Arıkan, 2009) — uses a recursive structure that is, at its core, a dynamical system.

**Definition 26.5.1.** A *linear code* $C \subseteq \mathbb{F}_q^n$ of dimension $k$ and minimum distance $d$ encodes $k$ symbols into $n$ symbols and can correct up to $\lfloor(d-1)/2\rfloor$ errors.

**Theorem 26.5.2 (Gilbert-Varshamov Bound).** There exist codes with rate $R = k/n$ and relative distance $\delta = d/n$ satisfying:
$$R \geq 1 - H_q(\delta),$$
where $H_q(\delta) = -\delta\log_q(\delta/(q-1)) - (1-\delta)\log_q(1-\delta)$ is the $q$-ary entropy function.

The Gilbert-Varshamov bound is a probabilistic existence argument: a random linear code over $\mathbb{F}_q$ achieves this rate-distance trade-off. The bound is tight for binary codes (it's essentially the sphere-packing bound), and the question of whether explicit codes meet it is still open for $q = 2$.

The entropy function $H_q(\delta)$ appearing in the bound is not coincidental: it's the same entropy function as in Shannon's channel coding theorem. The capacity of a $q$-ary symmetric channel with error probability $\delta$ is $\log q - H_q(\delta)$, which equals $R$ in the GV bound. The codes achieving the GV bound are exactly the capacity-achieving codes for the $q$-ary symmetric channel.

**Connection to Subshifts:** Good codes correspond to subshifts with high topological entropy but good distance properties. Low-density parity-check (LDPC) codes correspond to sparse constraint graphs — the *Tanner graph* is essentially a factor graph of a Markov random field on a constrained shift.

The Tanner graph of an LDPC code is a bipartite graph connecting variable nodes (code symbols) to check nodes (parity constraints). The code is the set of binary sequences satisfying all the parity checks — equivalently, the subshift defined by the Tanner graph constraints. The topological entropy of this subshift is the rate of the code. The distance properties depend on the expansion properties of the Tanner graph.

LDPC codes with random Tanner graphs achieve the Gilbert-Varshamov bound, but explicit LDPC codes (with degree-distribution optimization) can be designed to approach capacity for specific channels. This is where the subshift theory of Chapter 24 and the expander theory of Section 26.4 converge: good LDPC codes have Tanner graphs that are good expanders, and this expansion is what gives them capacity-approaching performance.

**Theorem 26.5.3 (Capacity-Achieving Codes — Polar Codes, Arıkan 2009).** Polar codes achieve the Shannon capacity of any binary-input memoryless channel with complexity $O(n\log n)$ encoding/decoding, using a recursive structure based on the butterfly network — a discrete dynamical system.

Polar codes are constructed by applying a recursive $2 \times 2$ transformation (the Hadamard matrix $\begin{pmatrix} 1 & 0 \\ 1 & 1 \end{pmatrix}$ over $\mathbb{F}_2$) to $n$ uses of the channel. After $\log_2 n$ recursive doublings, some channels become "very good" (capacity near 1) and some become "very bad" (capacity near 0). The good channels are used to transmit information; the bad ones are frozen with known values. The fraction of good channels equals the channel capacity.

The recursive doubling is a discrete dynamical system: the channel capacity $I$ evolves according to $I \mapsto I^+$ and $I \mapsto I^-$ depending on the bit position. As you apply the transformation more times, the capacities polarize: they all converge to either 0 or 1. The rate of polarization — how quickly the channels polarize — determines the code's finite-length performance.

This is the dynamical systems perspective on polar codes: the channel polarization process is an orbit of a discrete map on the space of channels, and the capacity theorem says that this orbit has a fixed-point structure (all channels converge to the two extremes). The attractor of the polarization map is the set $\{0, 1\}$ of extreme channels, and the basin of attraction has measure equal to the channel capacity.

And so we close the loop. We started the book with dynamical systems, introduced information theory as a tool for measuring dynamical complexity, and now we see that the construction of optimal error-correcting codes is itself a dynamical system — one whose orbit structure directly encodes the channel capacity. Information theory and dynamical systems aren't just connected. They are, at the deepest level, the same subject.
