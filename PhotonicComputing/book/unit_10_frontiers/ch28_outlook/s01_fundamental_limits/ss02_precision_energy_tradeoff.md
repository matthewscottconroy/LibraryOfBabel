# 28.1.2 The Precision–Energy Trade-off

## Shannon's Bound in an Optical Channel

The precision–energy trade-off is the one fundamental limit in this section that is not a distant ceiling but a working constraint, felt in every analog optical processor. Its origin is information-theoretic. Shannon showed that a communication channel of signal-to-noise ratio $\mathrm{SNR}$ carries at most

$$B = \tfrac12\log_2\!\left(1+\mathrm{SNR}\right)$$

bits of information per use, per real degree of freedom [Shannon, *Bell System Technical Journal*, 1948]. An analog optical multiply-accumulate is exactly such a channel: it encodes an operand or a result as a continuous optical amplitude, and the number of *reliably distinguishable levels* in that amplitude — equivalently, the bits of numerical precision the operation delivers — is bounded by its SNR through Shannon's formula. Precision, in analog optics, is not free to declare; it must be purchased with SNR.

## Shot Noise Turns Precision into Photons

What sets the SNR of an optical channel? At the fundamental limit, the discreteness of light itself. A detector registering a mean of $\bar n$ signal photons suffers Poissonian shot noise of standard deviation $\sqrt{\bar n}$ (Section 28.1.3), so the power signal-to-noise ratio of a shot-noise-limited measurement grows linearly with the detected photon number, $\mathrm{SNR}\sim\bar n$, and the number of distinguishable amplitude levels grows as $\sqrt{\bar n}$. Substituting into Shannon's bound,

$$B \approx \tfrac12\log_2\!\left(1+\bar n\right)\ \xrightarrow{\ \bar n\gg1\ }\ \tfrac12\log_2 \bar n.$$

Precision and photon count are thereby locked together: the bits of a shot-noise-limited optical operation are, up to order-unity constants, half the base-two logarithm of the number of photons it detects. Energy — photons times $h\nu$ — is the currency in which analog optical precision is denominated.

## The Exponential Cost of a Bit

Invert the relation and the trade-off shows its teeth. To obtain $B$ bits requires

$$\bar n \approx 4^{B}$$

detected photons. Each additional bit of precision *quadruples* the photon budget: gaining one bit means multiplying the SNR by four (so that $\tfrac12\log_2$ rises by one), and since SNR scales with photon number, four times the photons. This is geometric, not incremental, and it is fundamental rather than a defect of any particular design. Concretely, at a telecom wavelength where $h\nu\approx1.3\times10^{-19}$ J:

- 4-bit precision needs $\bar n\approx4^4=256$ photons, about $33$ aJ;
- 8-bit precision needs $\bar n\approx4^8\approx6.6\times10^4$ photons, about $8$ fJ;
- 16-bit precision needs $\bar n\approx4^{16}\approx4.3\times10^9$ photons, about $0.6$ pJ.

These are *floors* — the shot-noise-limited energy to read a single result at that precision assuming perfect detectors and no other loss; the real energy is larger by the inverse of every efficiency in the path. The lesson is stark: analog optics is cheap at low precision and punishingly expensive at high precision, with the cost climbing fourfold per bit. A 16-bit analog optical processor is a contradiction in ambition; the shot-noise floor alone already exceeds the energy of the digital multiply-accumulate it hoped to beat, before a single real-world loss is counted.

## Why Photonic Machine Learning Chooses Low Precision

This single fact explains the otherwise curious convergence of the entire photonic-computing field on low-precision workloads. Photonic accelerators target machine-learning inference at 4- to 8-bit precision not by preference but by physics: it is the regime where the $4^B$ photon cost is still small and the optical energy per operation, amortized across a large fan-in (Section 28.1.1), can undercut a digital MAC [Hamerly et al., *Physical Review X*, 2019; Nahmias et al., *IEEE JSTQE*, 2020]. It is fortunate — and not coincidental — that neural-network inference is famously tolerant of low precision; quantization to 8 bits and below is standard practice in digital accelerators too. Photonics and modern machine learning meet at low precision because that is where each is comfortable. Ask an optical processor for the 32- or 64-bit precision of scientific linear algebra and the precision–energy trade-off forecloses the advantage before any device imperfection is considered.

## Sub-Photon MACs and the Amortization Loophole

The most striking demonstrations in the field appear, at first, to violate the trade-off: optical neural networks operating at *less than one photon per multiplication* [Wang et al., *Nature Communications*, 2022]. They do not violate it; they exploit its structure. The $4^B$ law governs the photons needed to read a *result* to $B$ bits — but a result is the sum over a fan-in of $N$ individual MACs, and the shot-noise budget of the sum is what matters, not that of each term. Spread $\bar n\approx4^B$ detected photons across $N$ multiplications and each MAC carries $\sim4^B/N$ photons; for $B=4$ and $N=1000$, that is roughly a quarter of a photon per MAC. Each individual multiplication sits far below the single-photon level and carries almost no precision on its own; usable precision materializes only in the accumulated sum, where the photons pool. This is the amortization of Section 28.1.1 and the precision–energy law of this section acting together, and it is the correct way to read sub-photon operation — not precision from nothing, but low per-MAC precision redeemed by large-fan-in summation. It works precisely because the network tolerates the noise; push $B$ higher or shrink $N$ and the accuracy collapses.

## The Optimal Operating Point

There is, then, no single "best" precision for photonic computing; there is an operating point, and it depends on the algorithm. The right question is the one Chapter 25 trains the reader to ask: what is the *minimum* precision at which the target workload still meets its accuracy specification, and what is the end-to-end energy — including the ADC, whose own bit-depth cost tracks the same exponential — at that precision? A signed or complex-valued encoding, a different detection scheme, a tolerance for a fraction of a percent of accuracy loss: each shifts the operating point, and each must be evaluated against the full ledger rather than asserted. The precision–energy trade-off does not counsel abandoning precision; it establishes that precision has a price set by the shot-noise floor, that the price is exponential in bits, and that the winning designs are the ones that need the fewest bits. This is the physics-of-computing view of analog optics in a sentence [McMahon, *Nature Reviews Physics*, 2023]: information costs photons, and photons cost energy, so ask for no more information than the problem requires.
