# Chapter 50 — Carrier and Optical Transport

On 22 August 1858, three days before Queen Victoria's message reached President
Buchanan, the *Niagara* and the *Agamemnon* had finished paying out 2,500 nautical
miles of cable from a mid-Atlantic rendezvous. The cable weighed about a tonne per
mile, the ships had failed twice before, and the total investment represented a
substantial fraction of the capital available for such a venture in either country.

Today there are roughly five hundred submarine cable systems in service, carrying
something above 99% of all intercontinental data traffic. Satellites — which the
public imagines carries the Internet — carry a small fraction of a percent of it.
The Internet is a set of glass fibres lying on the ocean floor, and the maps are
public and worth looking at.

This chapter is about the layer below everything else: the long-haul transport
systems that carry aggregated traffic between cities and continents, and the physical
plant they run on.

## Why this layer looks different

Enterprise networking optimises for flexibility and cost. Carrier transport optimises
for two things that barely feature in Units IV through VIII: availability measured
in nines, and **capacity per fibre**.

The availability requirement produced SONET/SDH's most distinctive feature. A SONET
ring is built from two counter-rotating fibre paths, and when a fibre is cut the
traffic is switched to the alternate direction in **under 50 milliseconds** — fast
enough that a telephone call does not drop and a human does not perceive it. That
number was a design requirement, it was met in 1988, and Ethernet-based networks took
until the 2010s to approach it. Anyone tempted to dismiss SONET as legacy should sit
with the fact that its protection switching outperformed spanning tree (Chapter 19)
by three orders of magnitude for twenty years.

The capacity requirement produced DWDM, which is the reason a single fibre pair laid
in 2001 can carry two hundred times the traffic today that it carried when it was
lit — with no change to the fibre at all.

## The hierarchy, inherited

§50.1 picks up the digital hierarchy from Chapter 12. The DS0 at 64 kb/s, derived
from Nyquist in Chapter 4 §4.2; twenty-four of them plus framing making a **T1** at
1.544 Mb/s in North America and Japan; thirty plus signalling making an **E1** at
2.048 Mb/s everywhere else. The incompatibility is a genuine historical accident with
real consequences for international circuits, and it persists in carrier price lists
today.

Above that, SONET (North America) and SDH (elsewhere) define a synchronous hierarchy:
OC-3 at 155 Mb/s, OC-12 at 622, OC-48 at 2.5 Gb/s, OC-192 at 10 Gb/s. **Synchronous**
is the key word — every element in the network is locked to a common clock traceable
to an atomic reference, which permits a low-rate tributary to be extracted from a
high-rate stream without demultiplexing the entire thing. That capability, "add-drop
multiplexing," is why SONET rings could serve a chain of cities elegantly.

**OTN** (Optical Transport Network, ITU-T G.709) is the modern replacement, and its
key addition is the forward error correction wrapper that lets a signal survive a much
worse optical path — buying reach, which as §50.3 shows is worth more than almost
anything else at these distances.

## DWDM: the multiplication

The most important technology in this chapter, and a direct application of
Chapter 9's frequency-division multiplexing to light.

A fibre carries one wavelength. It also carries a second wavelength, and a third,
each modulated independently, all sharing the same glass without interacting. The
standard grid (ITU-T G.694.1) specifies 50 GHz or 100 GHz spacing in the C-band around
1550 nm, giving 80 to 96 channels on one fibre pair. At 400 Gb/s per channel, that
is 30–40 Tb/s on a single pair, and flexible-grid systems with higher-order modulation
push beyond it.

Two enabling technologies deserve naming, because without them DWDM would be
impractical.

The erbium-doped fibre amplifier (David Payne's group at Southampton, and
Bell Labs, 1987) amplifies light *as light*, without converting to electrical and
back. Crucially it amplifies a whole band at once — all ninety-six wavelengths
simultaneously, in one device. Before the EDFA, every wavelength would have needed its
own regenerator every 40 km, and the economics would have been impossible. The EDFA is
why long-haul optical networking exists, and it is the reason the C-band is the C-band
— erbium's gain happens to sit there.

Coherent detection with digital signal processing (from around 2008) recovers the
optical signal's phase as well as its amplitude, permitting the QAM constellations of
Chapter 8 §8.3 at optical frequencies, and permitting chromatic dispersion to be
compensated **electronically** in the receiver rather than optically in the line. That
last point transformed the economics: existing fibre routes could be upgraded to far
higher capacity by replacing only the terminal equipment, which is Chapter 10's lesson
that a medium's properties are a function of the current manufacturing art.

## MPLS: the circuit returns

§50.4 covers **Multi-Protocol Label Switching**, and it deserves attention because it
is Chapter 13's argument resolved in a third way.

An MPLS router forwards on a short fixed-length **label** rather than by longest-prefix
match on a destination address (Chapter 29 §29.3). Labels are assigned at the network
edge and swapped at each hop along a pre-established **label-switched path**.

The original motivation was speed — fixed-label lookup was faster than prefix matching
in 1997 hardware — and that motivation evaporated as lookup hardware improved. MPLS
survived and thrived for entirely different reasons:

**Traffic engineering.** A label-switched path can be *placed* — routed deliberately
along a chosen route with reserved bandwidth, rather than wherever the IGP's shortest
path happens to go. This is a virtual circuit (Chapter 13 §13.2) on packet
infrastructure, and it is what carriers actually wanted.

**VPNs.** Labels stack. An outer label routes across the carrier's core; an inner
label identifies the customer. Many customers with overlapping RFC 1918 address space
share one infrastructure with complete separation. MPLS L3VPN became the standard
enterprise WAN product for two decades, and Chapter 51 covers its displacement.

**Fast reroute**, which brings sub-50 ms protection to packet networks and closes the
gap with SONET.

The lesson worth extracting: MPLS is what happens when an industry that abandoned
circuits discovers it wanted some of their properties after all. It will not be the
last time in this book.

## By the end you will be able to

- State the digital hierarchy from DS0 upward and explain the T1/E1 divergence.
- Explain SONET's ring protection and why 50 ms was the target.
- Explain DWDM and compute a fibre pair's capacity from channel count and rate.
- Explain what the EDFA and coherent detection each made possible.
- Explain label switching and the two reasons MPLS outlived its original
  justification.
- Explain how MPLS L3VPN separates customers with overlapping address space.
