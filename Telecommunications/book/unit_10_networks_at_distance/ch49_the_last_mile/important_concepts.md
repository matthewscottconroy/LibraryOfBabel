# Chapter 49 — Important Concepts

The last mile cannot be shared, and that is the whole chapter *(intro)* — Every other
segment's cost per customer falls with scale; the final connection to one building serves one
building, and trenching, permits and a technician's visit do not get cheaper. Every
technology here is an attempt to avoid building a new last mile.

V.34 stopped at 33.6 kb/s because the channel ran out, not the engineering *(§49.1)* —
$3400 \times \log_2(1+1000) \approx 33.9$ kb/s. Fifteen years of modem development
converged on a number Shannon computed in 1948.

V.90 changed channels rather than beating Shannon *(§49.1)* — Downstream, the ISP is
digitally attached to a network that is already carrying 8,000 8-bit PCM samples per second.
The modem reads codewords rather than decoding a waveform, 7 usable bits × 8,000 = exactly
56 kb/s. Upstream stays at 33.6 because the home modem must still produce an analogue
waveform. The asymmetry is a consequence of where the converters sit.

When a channel's limit is reached, find a different channel in the same medium *(§49.1)* —
V.90 did it to the voiceband, DSL did it to the same copper, DOCSIS did it to television
spectrum. The recurring move of this chapter.

The 3.4 kHz limit was never a property of the wire *(§49.1)* — It was the filters, loading
coils and channel banks the telephone company attached. The copper passes megahertz, badly,
over diminishing distance.

ADSL's entire integration cost was a passive filter *(§49.1)* — Voice below 4 kHz, data
above it, a splitter between. The telephone keeps working, including when the power fails.
Chapter 28 §28.1 again: the technology that demands nothing of the installed base wins.

DMT is bit-loading, and it is OFDM by another name *(§49.1)* — Each 4.3125 kHz subcarrier
is measured and given 0–15 bits according to its own SNR. A short line loads subcarriers up
to 1.1 MHz; a long line loads nothing above 400 kHz. The rate is a sum over the spectrum,
not a single number.

Every DSL variant makes the same trade *(§49.1)* — More spectrum, more bits, less
distance. ADSL2+ 2.2 MHz over kilometres; VDSL2 17–30 MHz over hundreds of metres;
G.fast 106–212 MHz over about 100 m.

FTTC replaces the part that can be replaced *(§49.1)* — The drop into the building is the
expensive asset; the long copper run is not. Moving the DSLAM to a street cabinet turned
4 km of copper into 300 m and delivered 100 Mb/s to tens of millions of homes. Its binding
constraint becomes crosstalk rather than attenuation, and vectoring — MIMO on a copper
bundle — is the answer.

DSL gives a private wire of poor quality; DOCSIS gives a share of an excellent one
*(§49.2)* — Every behavioural difference between the two services follows from that
sentence, including which complaint each generates: "it's always this slow" versus "it's slow
at 8 p.m."

The cable upstream has two independent problems *(§49.2)* — **It is small** — 5–42 MHz
against 948 MHz of downstream, which sets the asymmetry before any modulation choice — and it
is a noise funnel: every home's ingress noise is summed on the way to the node, so one
corroded connector degrades an entire neighbourhood. There is no DSL equivalent.

Channel bonding, then OFDM *(§49.2)* — A 6 MHz QAM-256 channel is $5.36 \times 8 \approx
43$ Mb/s; 32 bonded give 1.37 Gb/s. DOCSIS 3.1 then abandons the channel grid for OFDM
blocks up to 192 MHz with 25/50 kHz subcarriers and up to 4096-QAM — DSL's DMT and Wi-Fi's
OFDMA arriving in a third medium.

Downstream broadcast means privacy is a decision *(§49.2, §49.3)* — In both DOCSIS and PON,
every subscriber physically receives every other subscriber's traffic and discards what is
not addressed to them. BPI+ and GPON's per-ONT AES are what make that acceptable, and
before BPI existed a promiscuous modem could read the neighbourhood.

Upstream is requested and granted, not contended *(§49.2)* — The request uses a small
ALOHA-like contention window; the data transmission is scheduled and does not collide.

Node splitting, cell splitting, more access points — the same answer three times *(§49.2)*
— Chapter 46 §46.1 and Chapter 45 §45.3 give the identical remedy for the identical problem.
Halve the population sharing the medium.

"Up to 1 Gb/s" is capacity when your neighbours are idle *(§49.2)* — Not a lie and not a
guarantee. The honest question is the node's peak utilisation, not the headline rate.

A passive splitter has no power supply and nothing to fail *(§49.3)* — That property, not
bandwidth, is what makes fibre-to-the-home economic. The outside plant sits in a footway box
for thirty years while the electronics at each end are replaced several times.

The optical budget is the design constraint *(§49.3)* — $10\log_{10}(N)$ plus excess:
1:32 costs about 17 dB, 20 km of fibre about 5 dB, and a Class B+ budget is 28 dB. Deeper
splits buy subscriber density with reach.

Ranging makes every ONT logically equidistant *(§49.3)* — Light in glass takes 4.9 µs/km,
so a 300 m–19 km spread puts burst arrivals about 180 µs apart. The OLT opens a quiet
window, measures round-trip time, and assigns an equalisation delay. The same problem and
the same solution as DOCSIS ranging and LTE timing advance — because there is only one.

Different wavelengths let generations coexist on one fibre *(§49.3)* — GPON at 1490/1310,
XGS-PON at 1577/1270. An operator upgrades a subscriber by swapping the ONT, not by
touching the plant. Trenching is a fifty-year investment; transceivers are a seven-year
one.

PON shares less among fewer, with no ingress *(§49.3)* — 16–64 homes rather than 100–500,
symmetric on XGS, and immune to water, corrosion, electrical noise and crosstalk. The
argument for fibre is not really about bandwidth.

Fixed wireless inverts the cost structure *(§49.4)* — Wired access costs per metre of
route; fixed wireless costs per tower and per terminal. It wins exactly where routes
are long and subscribers few — which is where wired operators decline to serve.

Orbital altitude determines everything *(§49.4)* — GEO 35,786 km → 477 ms round trip;
MEO ~8,000 km → 107 ms; LEO 550 km → about 7 ms of propagation. The round trip is four
one-way hops, because a bent-pipe satellite is a mirror, not a router. 477 ms is the speed
of light over 143,000 km and will be 477 ms in a century.

GEO's compensation is that it does not move *(§49.4)* — One sidereal day, a fixed dish
aimed once, and three satellites covering the populated Earth. For broadcast that is close
to ideal.

477 ms destroys a single TCP stream *(§49.4)* — A 64 KB window gives 1.1 Mb/s on a
100 Mb/s link (Chapter 3 §3.4). Window scaling raises the ceiling to about 70 Mb/s at 4 MB,
but slow start still needs nine round trips and four seconds to get there, and every loss
costs 477 ms to detect.

The PEP is a deliberate end-to-end violation, and encryption defeats it *(§49.4)* — It
terminates TCP at each end and spoofs acknowledgements. A VPN or QUIC connection cannot be
split, so it performs dramatically worse over the same link — architectural, not a fault, and
one of the underappreciated consequences of encrypting the transport header.

LEO's propagation is the small part of its latency *(§49.4)* — 7 ms of physics against
25–60 ms observed. The rest is ground-station backhaul, radio scheduling and queueing —
Chapter 46 §46.4's lesson repeated: the radio is not the whole path.

Light is 47% faster in vacuum than in glass *(§49.4)* — So with inter-satellite laser
links, a route through orbit can beat a submarine cable on long paths despite the extra
distance. Trading firms have funded this on purpose.

LEO is best where users are sparse *(§49.4)* — Everyone under a beam shares it, which
is the opposite of the concentration pattern that makes wired networks economic. It
complements fibre; it does not replace it. And a 550 km satellite deorbits in about five
years, so the constellation must be replaced continuously — the business model requires
cheap launch to exist permanently.
