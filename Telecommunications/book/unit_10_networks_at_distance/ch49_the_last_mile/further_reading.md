# Chapter 49 — Further Reading

## Standards

ITU-T G.992.x (ADSL), G.993.x (VDSL2), G.9700/G.9701 (G.fast), G.993.5 (vectoring).
Purchasable, and the summaries are freely available. The band plans and the bit-loading
descriptions are the parts worth having.

ITU-T G.984 (GPON), G.987 (XG-PON), G.9807 (XGS-PON), G.9804 (50G-PON).
G.984.3's ranging and DBA clauses are the ones to read — F5 uses them, and they are
unusually clear for a telecoms standard.

IEEE 802.3ah / 802.3av — EPON and 10G-EPON.
The Ethernet-framed alternative to the ITU's PON family. Reading both shows two committees
solving the same problem with different inherited assumptions.

**CableLabs DOCSIS specifications** (cablelabs.com).
Freely available, which is unusual and welcome. The DOCSIS 3.1 PHY specification's OFDM
sections are a good companion to Chapter 8.

**ITU-T V.34 and V.90.**
Historical now. V.90's description of how the digital-to-analogue asymmetry is exploited is
worth reading once as an example of an engineering argument rather than a specification.

## Books

Starr, T., Cioffi, J. & Silverman, P. — *Understanding Digital Subscriber Line Technology*,
and the later *DSL Advances*.
The standard texts, by the people who built it. Mathematical where it needs to be, and
the chapters on loop impairments are the practical reference for §49.1's troubleshooting.

Ovadia, S. — *Broadband Cable TV Access Networks*.
The HFC plant explained by an engineer. Good on the noise funnel and on why the return path
is the way it is.

Lam, C. F. — *Passive Optical Networks: Principles and Practice*.
The reference for §49.3. The optical budget and ranging chapters are the ones you will
return to.

Maral, G., Bousquet, M. & Sun, Z. — *Satellite Communications Systems*.
The standard textbook. The orbital mechanics and link budget chapters give you the tools to
verify §49.4's arithmetic yourself.

## Papers and history

Lechleider, J. (1991). "High Bit Rate Digital Subscriber Lines: A Review of HDSL Progress."
*IEEE JSAC*. The asymmetry argument in the author's own words.

Chow, P. S., Tu, J. C. & Cioffi, J. (1991). "A Discrete Multitone Transceiver System for
HDSL Applications." *IEEE JSAC*.
**The DMT case**, made while the standards committee was still deciding.

Clarke, A. C. (1945). "Extra-Terrestrial Relays." *Wireless World*, October 1945.
Four pages, freely available, and it contains the entire geostationary concept. Read it
for the pleasure of watching someone get something completely right with no possibility of
building it.

Kessler, D. & Cour-Palais, B. (1978). "Collision Frequency of Artificial Satellites: The
Creation of a Debris Belt." *Journal of Geophysical Research*.
The paper behind §49.4's orbital congestion discussion, and it reads very differently now
than it did in 1978.

Handley, M. (2018). "Delay is Not an Option: Low Latency Routing in Space." HotNets.
The analysis of whether LEO constellations can beat fibre on long paths, with the
refractive-index argument worked properly. The accompanying simulations are on YouTube and
are genuinely worth watching.

## Measurement and practical work

**Your own line's statistics.** Every DSL modem, cable modem and ONT exposes them, and F1
and F2 are built on this. The DSL numbers to find are attenuation, SNR margin, and the
error-second counters; on cable, downstream and upstream power, SNR, and the T3/T4 timeout
counts; on PON, **received optical power.**

`rtl_433` and an SDR — will not read your access line, and will show you what else is in
the spectrum near a fixed wireless installation.

The FCC Measuring Broadband America reports, and equivalent programmes in other countries
(Ofcom's UK reports, ACCC's Australian ones).
Independent measurement of what access technologies actually deliver against what is
advertised, by technology and by operator. The DSL-versus-cable-versus-fibre comparisons
are exactly §49.2's argument, measured.

**Satellite tracking** — `n2yo.com`, the Heavens-Above site, or a phone application. **F6 uses
one.** Watching a Starlink train pass overhead makes the constellation size argument concrete
in a way no table does.

Public speed test infrastructure, used sceptically. Most test servers sit inside your own
ISP, which measures the access link and nothing else — a useful measurement, and not the one
most people think they are making.

## Following the field

**Broadband Forum** (broadband-forum.org) — the industry body for access network architecture.
TR-069 and TR-369 (USP), the remote management protocols every home gateway implements, are
theirs and are worth knowing about.

Light Reading, Fierce Telecom, and the ISP trade press — for what is actually being
deployed, which lags the standards by three to seven years and leads the textbooks by about
the same.

The Starlink and OneWeb regulatory filings with the FCC and ITU. Public, detailed, and
the most reliable source on constellation design — far better than press coverage, because
the numbers are legally binding.

## Where to look next

**Chapter 50** goes behind the access network to the carrier transport that feeds it;
**Chapter 52** covers what happens when a shared access network must carry traffic with
different requirements; and **Chapter 66** returns to the bandwidth–delay product of §49.4 as
a general performance-diagnosis tool.
