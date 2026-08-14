# Chapter 10 — Further Reading

## Primary sources

**Kao, K. C. & Hockham, G. A. (1966). "Dielectric-fibre surface waveguides for
optical frequencies." *Proceedings of the IEE* 113(7): 1151–1158.**
The paper. Read §1 and §5: the argument that attenuation is a materials problem
rather than a physical limit, and the estimate that 20 dB/km would suffice. It is
a good example of a paper whose most important contribution is a change of
question.

**Kao, C. K. (2009). "Sand from Centuries Past: Send Future Voices Fast." Nobel
Lecture.**
Kao's own account, delivered by his wife. Short and worth reading for the
description of persuading glass manufacturers to attempt something nobody had
asked for.

**Heaviside, O. (1880). British Patent 1,407 — coaxial cable.**
Brief and historically interesting.

**TIA-568.2-D, *Balanced Twisted-Pair Telecommunications Cabling and Components
Standard*.** And **ISO/IEC 11801** for the international equivalent.
Where every category specification in §10.1 is defined. Read the parameter list
once: attenuation, NEXT, PSNEXT, ACR-F, return loss, delay skew — it makes concrete
that a category is a set of measurements rather than a speed rating.

**IEEE 802.3, Clause 33 (PoE), Clause 145 (802.3bt).**
The power delivery specifications, including the classification handshake and the
cable-loss allowances that produce the gap between source and device wattage.

**ITU-T G.652 (single-mode), G.657 (bend-insensitive), G.651.1 (multimode).**
The fibre specifications. G.652.D is the low-water-peak variant; G.657 is why
in-building fibre became practical.

## Books

**Hayes, J. (2020). *Fiber Optics Technician's Manual*, 5th ed. Cengage.**
The practical book. Loss budgets, splicing, connector cleaning, OTDR
interpretation, and the realities of installation. Written for people who will hold
the equipment, and better than most academic treatments on the things that actually
go wrong.

**Barnes, S., Hyde, J. & Ross, S. (2018). *BICSI Information Technology Systems
Installation Methods Manual.***
The installation standard-of-practice reference. Dry and authoritative on
containment, bend radius, separation from power, labelling and testing.

**Agrawal, G. P. (2012). *Fiber-Optic Communication Systems*, 4th ed. Wiley.**
Chapter 2 covers fibre physics rigorously: modes, dispersion, attenuation
mechanisms. Where to go when §10.3's summary is insufficient.

**Ramo, S., Whinnery, J. R. & Van Duzer, T. (1994). *Fields and Waves in
Communication Electronics*, 3rd ed. Wiley.**
Transmission line theory, characteristic impedance and the coaxial optima of §10.2,
derived rather than asserted.

**Ott, H. W. (2009). *Electromagnetic Compatibility Engineering.* Wiley.**
Chapter 3 on cabling and the grounding chapters are the reference for §10.1's
shielding-and-earthing warning and §10.5's inter-building argument. Read before
specifying shielded cable or copper between buildings.

**Rappaport, T. S. (2024). *Wireless Communications: Principles and Practice*,
3rd ed. Cambridge.**
For §10.4's propagation and band characteristics, with measurement methodology.

## Applied and practitioner

**The Fiber Optic Association (foa.org) reference guides.**
Free, correct, and comprehensive on loss budgets, connector types and polish,
cleaning, and testing. The cleaning material deserves particular attention: dirty
connectors are the most common fibre fault and the least discussed topic in
textbooks.

**Fluke Networks and Viavi certification application notes.**
Vendor-written, freely available, and genuinely good on what each cable test
measures and what each failure indicates. The material on distinguishing a split
pair, a NEXT failure and a length failure is directly useful.

**Cisco and Juniper transceiver compatibility matrices.**
Unglamorous and worth knowing exist. The reach figures in §10.3 vary by transceiver,
and the authoritative source is the vendor's matrix rather than the standard's
headline number.

**Ofcom, FCC and ITU-R spectrum allocation charts.**
The FCC's spectrum allocation chart is a poster-sized document worth looking at
once: it makes §10.4's point about spectrum being an allocated, contested resource
immediately visual.

## Tools

**`perfcalc.py linkbudget`** in this book's [tools/](../../../tools/) directory —
for §10.4's wireless links and for checking exercise B6 of Chapter 10.

**An optical power meter and a visual fault locator**, if your institution has
them. Measuring a real link's loss and comparing it against a computed budget takes
twenty minutes and makes §10.3 concrete. A fibre inspection scope showing a dirty
ferrule before and after cleaning is the most persuasive demonstration in
the chapter.

**A cable certifier**, if available. Lab 02 builds a split pair deliberately; seeing
it pass a continuity tester and fail a certifier's NEXT test is the demonstration
that makes Chapter 6 §6.4 permanent.

## For the certification-minded

Objective 1.5 is essentially this chapter: media types, distances, connectors,
transceivers, and the selection criteria. Objective 2.4 covers physical installation
and PoE. Objective 5.2 covers cable fault diagnosis.

Four things worth over-learning, because they are examined and because they are
load-bearing in practice:

1. **Distance limits**, and that 100 m for twisted pair is absolute rather than a
   guideline.
2. **Category is a bandwidth rating**, not a data rate — which is why the same
   Cat5e serves three generations of Ethernet.
3. **PoE budgets are per switch**, and the arithmetic must be done.
4. **Single-mode versus multimode** is decided by reach and by transceiver cost
   multiplied by link count, not by the price of the glass.

And the one that is not on any exam and matters more than any of them:
**fibre between buildings, for earthing and lightning reasons**. Nobody is tested
on it and it destroys equipment when it is got wrong.
