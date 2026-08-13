# Chapter 50 — The People

**David N. Payne (b. 1944).** University of Southampton — **and the erbium-doped fibre
amplifier, 1987.**

**The problem Payne's group solved had defeated a decade of effort.** Optical signals had to be
regenerated electrically every 40 km or so, **and for a wavelength-multiplexed system that
meant one full regenerator per wavelength per site.** The cost scaled with channel count, which
meant **WDM was economically pointless** however elegant it was.

**Erbium's contribution is that it happens to fluoresce at 1530–1565 nm** — the same band as
silica's loss minimum — **so a length of erbium-doped fibre, pumped by a cheap laser, amplifies
the entire band at once.**

> **The EDFA did not make optical transmission possible. It made the cost of optical
> transmission independent of the number of channels**, and that is the sentence that explains
> the last thirty years of long-haul networking. **Before it, DWDM was an idea. After it, DWDM
> was obviously correct.**

**Emmanuel Desurvire, at Bell Labs, published comparable work in the same period**, and the
usual and fair statement is that the two groups arrived independently. Payne's group at
Southampton's Optoelectronics Research Centre went on to produce much of the subsequent work on
high-power fibre lasers, **and the amplifier remains the contribution every long-haul system in
service depends on.**

**Charles K. Kao (1933–2018).** **The 2009 Nobel Prize in Physics, for a 1966 paper.**

**Kao's contribution was an argument, not a device.** Glass in 1966 attenuated light at about
**1,000 dB/km** — meaning a signal was gone within metres — and the consensus was that this was
a property of glass.

**Kao's paper with George Hockham argued that it was not.** **The loss was caused by
impurities, principally transition metal ions, and not by the silica itself.** He computed
that **20 dB/km would make optical communication practical**, and asserted that purification
could reach it.

**Corning achieved 17 dB/km in 1970.** **Modern fibre is about 0.2 dB/km**, which is five
thousand times better than the 1966 material.

> **Kao spent years persuading glass manufacturers that his calculation was right**, travelling
> to companies who did not believe purification to that level was achievable. **The technical
> insight took a paper; the adoption took a decade of argument**, and that ratio recurs
> throughout this book.

**His Nobel lecture was delivered by his wife**, Gwen Kao; he had developed Alzheimer's disease
by then. **Every submarine cable in §50.5 exists because of a calculation he did and then had
to defend.**

**Yasuharu Suematsu (b. 1932) and the semiconductor laser people.** **The dynamic
single-mode laser** — a source whose wavelength does not shift as it is modulated — **and
without it DWDM's channel spacing would be impossible.**

**A laser that wanders in wavelength cannot sit in a 50 GHz channel.** The distributed feedback
(DFB) structure locks the wavelength with a grating built into the device, **and it is the
component that makes the ITU grid meaningful.**

**Eric Rosen, Yakov Rekhter, Arun Viswanathan and the MPLS working group.** **RFC 3031, 2001**,
and the earlier proposals it synthesised.

**MPLS was assembled from three competing proprietary designs** — **Ipsilon's IP Switching**
(Tom Lyon and others), **Cisco's Tag Switching** (Rekhter and Rosen), and **IBM's ARIS** — and
the interesting thing about the result is how much of the original motivation it discarded.

> **The working group produced a standard whose stated purpose was forwarding speed, in the
> knowledge that forwarding speed was already ceasing to be a problem.** What they preserved —
> **the label stack, and the separation of forwarding from routing** — turned out to be what
> mattered. **Whether that was foresight or luck is genuinely arguable**, and the participants
> have given both accounts.

**Rekhter's name recurs in this book** — **BGP** (Chapter 32), **MPLS, and L3VPN (RFC 4364 with
Rosen)** — **three of the mechanisms the commercial Internet runs on**, and the L3VPN paper in
particular is a small model of how to specify a service rather than a protocol.

**Clarence Filsfils and the segment routing group.** **Cisco, from around 2013**, and the
argument was explicitly a simplification argument.

**The case was that MPLS's control plane had become the problem it was meant to solve.** LDP
and RSVP-TE together meant **two distribution protocols, per-path soft state on every transit
router, and periodic refresh traffic proportional to the number of paths.** **Filsfils argued
for encoding the path in the packet and deleting the rest**, which is a proposal to remove
several thousand pages of accumulated specification.

> **Standards bodies rarely delete things.** Segment routing is one of the few cases in this
> book of a working group **successfully arguing for less**, and it succeeded because the
> operational cost of the state was measurable.

**Cyrus Field (1819–1892).** **The 1858 transatlantic telegraph cable — and the commercial
persistence that laid it.**

**Field was a paper merchant with no technical background** who raised the capital, assembled
the expertise, and **failed three times** before the 1858 attempt succeeded. **That cable
worked for three weeks**, at a few words per minute, **and then died** — because the chief
electrician, Wildman Whitehouse, applied 2,000 volts to force the signal through, destroying
the insulation.

**Field raised money again and laid a working cable in 1866.**

> **The 1858 failure is worth knowing because of who was right.** **William Thomson — later
> Lord Kelvin — had argued that the cable needed sensitive detection rather than brute force**,
> and had invented the mirror galvanometer to provide it. **Whitehouse's high voltage was
> preferred because it was cheaper and more obviously vigorous.** It destroyed, in three weeks, an
> asset that had taken four years and three failed attempts to place.

**Kelvin's analysis of signal propagation on a long cable** — the "law of squares", relating
signalling speed to the square of the cable's length — **is the first quantitative treatment of
a transmission line**, and it is the ancestor of Chapter 6.

## A note on what this chapter's people have in common

**Four of them were told that a physical limit was insurmountable, and were right that it was
not.**

**Kao was told glass could not be made pure enough.** **Payne's field had accepted that
regeneration was unavoidable.** **Cioffi and Lechleider, in Chapter 49, were told copper could
not carry broadband.** **Field was told the ocean was too wide.**

**And one of them was wrong in the other direction.** **Whitehouse believed the limit could be
overcome with more voltage**, and destroyed the cable proving it.

> **The distinction is whether the limit is in the physics or in the current implementation**,
> and it is the same judgement Chapter 49's people had to make. **Kao did the calculation.
> Whitehouse did not.**
