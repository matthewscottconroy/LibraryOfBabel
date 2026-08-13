# Chapter 50 — Important Concepts

**A leased line sells a path, not bandwidth** *(§50.1)* — Priced by distance, **because the
carrier is genuinely reserving capacity along a route** and its cost is the cost of not selling
that path to anyone else. **Chapter 13 §13.1's circuit switching, priced honestly.**

**Everything derives from the DS0** *(§50.1)* — 4 kHz voice → 8,000 samples/s (Nyquist) ×
8 bits (PCM) = **64 kb/s**, and **every rate in carrier transport is a multiple of it.**
T1 = 24 × 64 + 8 kb/s framing = **1.544 Mb/s**; E1 = 32 × 64 = **2.048 Mb/s**, of which 30
carry voice.

**The T1/E1 split is a 1962 transistor-count decision that never went away** *(§50.1)* —
America standardised on 24 channels because that is what could be multiplexed reliably; Europe
standardised later on 30 with a dedicated signalling channel. **Neither is wrong. International
circuits have converted between them for sixty years**, and both still appear in carrier price
lists. **Standards adopted early enough to matter are adopted before you know enough to get
them right.**

**Robbed-bit signalling is where 56 kb/s came from** *(§50.1)* — T1 has no signalling channel,
so the least significant bit of every sixth frame was stolen. **Inaudible for voice, fatal for
data**, leaving 7 bits per sample. **The same 56 kb/s as V.90's** (Chapter 49 §49.1), from the
same 1960s decision.

**Plesiochronous means you must unwind the whole hierarchy** *(§50.1)* — Independent clocks
force stuffing bits, so **reaching one DS0 inside a DS3 requires demultiplexing to DS2, to DS1,
to DS0, and multiplexing it all back.** Twelve towns on a route means twelve complete cycles.
**This is the specific problem SONET was built to solve.**

**Synchronisation converts multiplexing into addressing** *(§50.2)* — With every element locked
to a traceable atomic reference, **a byte's position in the frame is its identity.** Byte 47 is
the same channel always, **so you reach in and take it — add-drop multiplexing**, and it is
what makes serving a chain of cities economic.

**The STS-1 frame is built around a voice sample** *(§50.2)* — 9 rows × 90 bytes × 8 bits ×
8,000 frames/s = **51.84 Mb/s**, and the 8,000 is the telephone sampling rate, **so one byte
per frame is exactly one DS0.** Every SONET rate is $n \times 51.84$ Mb/s.

**50 ms was a requirement, not a result** *(§50.2)* — **Telephone switches drop a call at about
60 ms of lost circuit**, so 50 was chosen with margin. It is achieved by **pre-provisioned
protection capacity, physical-layer failure detection, and a purely local decision** — no
convergence, no election, no distributed computation. **Compare classic spanning tree's 30–50
seconds** (Chapter 19 §19.2).

**Protection costs half the capacity, by design** *(§50.2)* — A 1+1 ring runs at 50%
utilisation with an identical amount idle **for the milliseconds in which it is needed.**
**Packet networks refused that trade** — statistical multiplexing exists to avoid reserving
idle capacity — **and spent twenty years reaching comparable protection by other means.**

**Three overhead layers make a fault localise itself** *(§50.2)* — **Section** between
repeaters, **line** between multiplexers, **path** end to end, each independently monitored.
**A B2 error count rising at one multiplexer and not its neighbour identifies the failing span
without leaving the office.** Chapter 65's layer-by-layer method, done in hardware on every
frame.

**OTN is a wrapper, and the FEC is the point** *(§50.2)* — **Put any client in, get it out
unchanged**, with **RS(255,239) at about 7% overhead giving roughly 6 dB of coding gain.**
**6 dB is a factor of four in required optical power** — spent on reach, and **every
regeneration site avoided is a building, a power feed and a twenty-five-year maintenance
contract.**

**DWDM is Chapter 9's FDM applied to light** *(§50.3)* — The C-band is about **4.4 THz wide**,
giving **~44 channels at 100 GHz spacing or ~88 at 50 GHz**; at 400 Gb/s each that is **tens of
terabits on one fibre pair** — a pair that carried 2.5 Gb/s when it was lit.

**The C-band is a coincidence** *(§50.3)* — **Silica's loss minimum is at 1550 nm** (Rayleigh
scattering rising one way, infrared absorption the other) **and erbium's gain band happens to
sit at 1530–1565 nm.** There is no engineering reason for this, **and the entire long-haul
industry is built on it.**

**The EDFA amplifies the whole band at once, as light** *(§50.3)* — **One device for 96
wavelengths**, where regeneration needed one per wavelength per site. It is **transparent to
rate and modulation**, so an amplifier installed for 2.5 Gb/s per channel passes 400 Gb/s
unchanged — **which is exactly why a fibre laid in 2001 is upgraded by replacing only the
terminals.** Its cost is that **it amplifies noise too**, and OSNR degrades with every span.

**Coherent detection made phase available, and dispersion electronic** *(§50.3)* — Mixing with
a local oscillator recovers amplitude **and phase**, so QAM (Chapter 8 §8.3) works at optical
frequencies; **polarisation multiplexing doubles it again — MIMO in a fibre.** And **chromatic
dispersion is undone in a digital filter**, which removed the route-specific compensation
modules from the line. **Upgrading a route became a purchase rather than a construction
project.**

**The reach–capacity trade is the whole of optical engineering** *(§50.3)* — 64QAM for metro,
16QAM for regional, **QPSK for transoceanic.** Higher-order modulation demands more SNR, which
means fewer spans. **The same Shannon curve as LoRa's spreading factors and Wi-Fi's MCS
index**, in a third medium.

**Optical invisibility is an operational hazard** *(§50.3)* — A packet may cross twenty
amplifiers and four ROADMs without becoming electricity, **and the IP layer sees one link.**
**Two "diverse" IP links may share a fibre, a duct or a ROADM** — a shared risk link group —
**and diversity must be verified against physical route maps, not circuit IDs.**

**MPLS's original justification died and deployment accelerated** *(§50.4)* — Label lookup was
faster than longest-prefix match in 1997 hardware; **TCAM made that irrelevant by 2002.**
**Two capabilities nobody had emphasised turned out to be what carriers wanted**, which is a
general lesson about how technologies are actually selected.

**A label is locally significant** *(§50.4)* — Push at the ingress, **swap at each transit hop**,
pop at egress; each router allocates its own labels and tells its upstream neighbour.
**The path is a chain of independent local agreements** — exactly ATM's VPI/VCI and Frame
Relay's DLCI, **the third appearance of the same idea.**

**Traffic engineering places paths instead of computing them** *(§50.4)* — An IGP sends
everything down the shortest path even when it is congested and the alternative is empty,
**and adjusting link costs moves every flow that used that link.** MPLS-TE says "this LSP goes
A→B→C→D and reserves 2 Gb/s." **It is a virtual circuit with admission control, on packet
infrastructure.**

**Fast reroute closes the gap with SONET by SONET's own method** *(§50.4)* — **Pre-computed
backup installed in hardware, switched locally at the node adjacent to the failure.** No
convergence, under 50 ms.

**Label stacking is what made MPLS ubiquitous** *(§50.4)* — **Outer label: get to PE-B. Inner
label: customer Acme, VRF red.** **The P routers in the core carry no customer routes at all**,
so a carrier serves ten thousand customers with overlapping RFC 1918 space while its core holds
one routing table — its own. **Route targets** then express hub-and-spoke, extranet and shared
services as import/export policy.

**Segment routing moves the state into the packet** *(§50.4)* — RSVP-TE keeps per-path state on
every transit router; **segment routing encodes the path as a label stack at the ingress and
keeps none.** **The end-to-end argument again** (Chapter 23 §23.4) — and SRv6 does it with IPv6
addresses, dispensing with MPLS labels entirely.

**A submarine cable is a garden hose with eighty amplifiers in series** *(§50.5)* — About
**500 systems, 1.4 million km, above 99% of intercontinental traffic**, 4–24 fibre pairs,
**powered from shore at up to 15 kV DC through a single conductor with the sea as return**,
unattended for twenty-five years.

**Faults are shallow-water human events, and repair takes weeks** *(§50.5)* — **Fishing gear
and anchors are the top two causes**; 150–200 faults a year; a few dozen repair ships
worldwide, **and ship availability rather than the splice is the long pole.** **The redundancy
is in having several cables, not in repairing one quickly.**

**The Internet's routing is redundant; its geography is not** *(§50.5)* — **Almost all
Europe–Asia capacity crosses Egypt overland in a corridor tens of kilometres wide.** BGP finds
another path if one exists; **it cannot create capacity that was never laid.** Alexandria 2008,
Tonga 2022, the Red Sea 2024.

**Content providers now own the majority of new transoceanic capacity** *(§50.5)* — Google
solely owns several systems; Meta's 2Africa runs about 45,000 km. **Chapter 48 §48.1's
flattening carried to its conclusion: if you are the traffic, owning the pipe is cheaper than
renting it.**

**You cannot buy latency below the route length** *(§50.5)* — Light in fibre is **4.9 µs/km**,
so London–New York is **65 ms round trip on a 6,600 km cable** against 55 ms on the 5,585 km
great circle. **That 10 ms detour is fish and seamounts, and no equipment recovers it.**
