# Chapter 50 — Further Reading

## Foundational papers

**Kao, K. C. & Hockham, G. A. (1966). "Dielectric-fibre surface waveguides for optical
frequencies." *Proceedings of the IEE*.**
**The paper that started fibre optics**, and it is an argument rather than a demonstration.
**Read it to see someone reason from measurements to a conclusion nobody believed.**

**Mears, R. J., Reekie, L., Jauncey, I. M. & Payne, D. N. (1987). "Low-noise erbium-doped
fibre amplifier operating at 1.54 µm." *Electronics Letters*.**
**Two pages, and thirty years of long-haul networking follow from them.**

**Thomson, W. (1855). "On the Theory of the Electric Telegraph." *Proceedings of the Royal
Society*.**
**The law of squares, and the first quantitative treatment of a transmission line.** The
ancestor of Chapter 6, and the analysis Whitehouse ignored.

**Rosen, E., Viswanathan, A. & Callon, R. (2001). RFC 3031, "Multiprotocol Label Switching
Architecture."**
**Short and clear.** The architecture, without the accumulated extensions.

**Rosen, E. & Rekhter, Y. (2006). RFC 4364, "BGP/MPLS IP Virtual Private Networks."**
**A model of how to specify a service rather than a protocol.** The RD/RT mechanism is
explained better here than in any textbook.

**Filsfils, C. et al. (2018). RFC 8402, "Segment Routing Architecture."**
**Read it against RFC 3031 and RSVP-TE** to see what "less" looks like when a working group
argues for it successfully.

## Standards

**ITU-T G.707 (SDH), G.709 (OTN), G.694.1 (the DWDM grid), G.652/G.654/G.655 (fibre types).**
**G.709's FEC clauses and G.694.1's grid are the two you will actually consult**, and F2 and
F3 use them.

**Telcordia GR-253 (SONET).**
The North American specification. **The overhead byte definitions are the reference for
§50.2's fault-localisation argument.**

**ITU-T G.977 and the submarine supplements.**
Less commonly read and worth knowing exist.

## Books

**Agrawal, G. — *Fiber-Optic Communication Systems*.**
**The standard graduate text.** Mathematical, comprehensive, and the chapters on dispersion,
nonlinearity and coherent detection are where §50.3's claims come from.

**Ramaswami, R., Sivarajan, K. & Sasaki, G. — *Optical Networks: A Practical Perspective*.**
**More architectural, less mathematical**, and better on ROADMs and network design. **The
better first book of the two.**

**Winzer, P., Neilson, D. & Chraplyvy, A. (2018). "Fiber-optic transmission and networking:
the previous 20 and the next 20 years." *Optics Express*.**
**A survey by three people who did much of it**, and unusually frank about which limits are
fundamental and which are not. **Free, and the best single overview available.**

**Minoli, D. — *Telecommunications Technology Handbook*, and Goralski, W. — *SONET/SDH*.**
The practical carrier-transport references. **Goralski is the one to reach for when a SONET
alarm needs interpreting.**

**Osterloh, H. — *IP Routing Primer Plus*, and Ghein, L. de — *MPLS Fundamentals*.**
**De Ghein is the standard practitioner's MPLS book**, and the L3VPN chapters are the ones to
read alongside F4.

## Submarine cables

**Carter, L. et al. (2009). *Submarine Cables and the Oceans: Connecting the World.***
UNEP/ICPC. **Free, well illustrated, and the best short introduction** — laying, faults,
repair and environmental impact.

**The International Cable Protection Committee** (iscpc.org).
**Fault statistics, repair practice, and the industry's own account of what breaks cables.**
The source for §50.5's fault causes.

**TeleGeography's Submarine Cable Map** (submarinecablemap.com).
**Free, current, and the thing to open when a region loses connectivity.** F1 uses it.

**Starosielski, N. — *The Undersea Network* (2015).**
**A cultural and geographic study rather than a technical one**, and it is the best account of
why cables land where they land — which turns out to be colonial-era routes, repeatedly.

**Blum, A. — *Tubes: A Journey to the Center of the Internet* (2012).**
**Journalism, and good journalism.** The chapters on landing stations and exchange buildings
make Chapter 48's abstractions physical.

**Standage, T. — *The Victorian Internet* (1998).**
The telegraph history, and **the parallels with the 1990s Internet are drawn without being
forced.** Short and enjoyable.

## Measurement and practical work

**containerlab, GNS3, or vendor virtual routers** — **F4 needs one.** A two-customer L3VPN with
overlapping 10.0.0.0/8 can be built on a laptop, and **capturing a core packet and identifying
both labels is the exercise that makes MPLS click.**

**Carrier looking glasses and `traceroute --mtu`** — for observing MPLS TTL behaviour and
finding where an MTU changes. **§50.4's traceroute-hides-hops behaviour is visible on most
carrier networks within three attempts.**

**Submarine cable status pages** — several carriers publish them, and **checking one before
troubleshooting a regional outage is a five-second habit worth acquiring.**

**Public optical performance data.** Some research networks (GÉANT, Internet2, JISC) publish
their optical layer statistics. **Watching pre-FEC error rates on a real system is more
instructive than any description of them.**

## Following the field

**OFC — the Optical Fiber Communication Conference** — post-deadline papers each March.
**Where the capacity records are announced**, and the abstracts are readable without the
mathematics.

**ECOC**, its European counterpart.

**Submarine Telecoms Forum** and the annual industry reports — **free, and the source for
who is building what.**

**MPLS/SDN/AI Net World Congress and the operator presentations at NANOG and RIPE** — for what
carriers are actually deploying, which as always lags the standards and leads the textbooks.

## Where to look next

**Chapter 51** covers what replaced MPLS L3VPN as the enterprise WAN product; **Chapter 52**
covers the queueing that happens where these links meet demand; and **Chapter 56** returns to
§50.3's and §50.5's shared risk problem as an availability calculation.
