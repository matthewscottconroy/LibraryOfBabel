# Chapter 71 — Further Reading

## A note on reading in this area

The signal-to-noise ratio in frontier literature is poor, and the discipline that helps is
the one §71.1 sets out: read the physics constraint first, then the demonstration's conditions,
then the claim. A demonstration at three metres in a laboratory is a demonstration at three
metres in a laboratory, and the paper will say so while the press release will not.

Prefer, in order: the measurement paper, the standards document, the sceptical practitioner,
the vendor white paper, the press release.

## Beyond 5G

The ITU-R IMT-2030 framework recommendation, and 3GPP's Release 19 and 20 study items.
The standards documents state what is actually being specified, which is a smaller and more
sober set than the research literature.

Rappaport, T. et al. — the terahertz propagation measurement papers (NYU Wireless).
Measured path loss, blockage and material penetration at 100–300 GHz. Read the measurement
sections and note the distances.

Rappaport, T. — *Wireless Communications: Principles and Practice* (Chapter 42's reading) —
for the propagation arithmetic that bounds all of it.

The IEEE 802.11bf sensing amendment, for integrated sensing at Wi-Fi frequencies — which is
deployable now and is the near-term version of §71.1's argument.

Ofcom's, the FCC's and CEPT's spectrum consultations above 100 GHz — for what is actually
being allocated, which lags the research by years and determines what can be built.

## Optics

Winzer, P., Neilson, D. & Chraplyvy, A. (2018). "Fiber-optic transmission and networking: the
previous 20 and the next 20 years." *Optics Express*.
Recommended in Chapter 50 and it is this section's core reading. Free, and unusually frank
about which limits are fundamental.

Essiambre, R.-J. et al. (2010). "Capacity Limits of Optical Fiber Networks." *JLT*.
The non-linear Shannon limit, derived. Mathematical, and the conclusions are readable
without the derivation.

The OFC post-deadline papers, annually (Chapter 50's reading) — where the records are
announced, and the abstracts state the conditions.

Submarine Telecoms Forum's annual reports, and the cable suppliers' technical papers — for
the power-versus-pairs argument of §71.2, which is discussed openly because it is a design
constraint rather than a competitive secret.

The 800G Pluggable MSA and OIF's co-packaged optics material — for the thermal constraint,
which is where the next generation's difficulty is.

## Quantum

Bennett, C. & Brassard, G. (1984). "Quantum Cryptography: Public Key Distribution and Coin
Tossing."
Six pages, freely available, and it is remarkable how complete it is.

Pirandola, S. et al. (2017). "Fundamental Limits of Repeaterless Quantum Communications."
*Nature Communications*.
**The PLOB bound.** §71.3's distance arithmetic is this paper, and it is the reason the
distance question is settled rather than open.

The NCSC's, NSA's, BSI's and ANSSI's published guidance on QKD.
**F4 uses one.** Short, direct, and they explain their reasoning — which is unusual and
makes them the most useful documents in this section.

Pirandola, S. et al. (2020). "Advances in Quantum Cryptography." *Advances in Optics and
Photonics*.
A long survey, and the sections on practical security and side channels are the ones that
temper the theoretical claims.

Wehner, S., Elkouss, D. & Hanson, R. (2018). "Quantum Internet: A Vision for the Road Ahead."
*Science*.
**The stages-of-development framework**, which is the honest way to discuss timescales — and note
which stage current demonstrations are at.

NIST's post-quantum project pages (Chapter 58's reading) — the alternative, and the one
being deployed.

## Deterministic networking

The IEEE 802.1 TSN standards overview (1.ieee802.org/tsn) — the amendment list with what
each does, which is the map through a confusing set of numbers.

**IEC/IEEE 60802**, the industrial TSN profile — which subset is actually required for
industrial automation, and it is more useful than the full standard set.

The Avnu Alliance's material — **certification and interoperability**, and it states what
"TSN-capable" actually means, which vendors' data sheets do not.

**RFC 8655 (DetNet architecture) and RFC 8938.**
**Short.** The problem statement is the clearest available description of why Layer 3
determinism is harder than Layer 2.

Linux's `taprio`, `etf` and `mqprio` documentation, and the `ptp4l`/`linuxptp` material —
F5 and F6 use them, and TSN on Linux is genuinely usable for experimentation.

Finn, N. — the various tutorial presentations on TSN — by one of the standards' authors,
and better than the standards for understanding the intent.

## AI, in both senses

**On the network** — Chapter 70 §70.4's reading applies unchanged, and the sceptical framing
there is the one to carry.

**In the network:**

The Ultra Ethernet Consortium's specifications and white papers — for what is being built,
and the problem statement is the useful part.

Meta's, Microsoft's and Google's published papers on AI cluster networking — **"Jupiter
Rising"** (Chapter 67's reading) and its successors, and the RoCE-at-scale papers.
Meta's and Alibaba's accounts of running RoCE in production are the honest ones about PFC's
failure modes.

Gangidi, A. et al. and the Meta AI cluster papers, and "Understanding RoCE at scale" —
for the deadlock and congestion problems as encountered rather than as theorised.

**NVIDIA's SHARP documentation** — **in-network aggregation, deployed**, and it is Chapter 68
§68.3's argument with a product.

The MLPerf and MLCommons benchmark results — for what the communication actually costs as
a fraction of training time, which is the number that justifies the fabric.

Barroso's *The Datacenter as a Computer* (Chapters 56, 67, 69) — for the power and cooling
context, which §71.5 says is now the binding constraint.

## Assessing frontier claims generally

**Chapter 4** and **Chapter 42** — the two chapters in this book that bound what is possible,
and most frontier claims are assessable against one of them.

Gartner's hype cycle material, used as a historical record rather than as a forecast.
Reading a ten-year-old one and comparing it with what arrived is more instructive than reading
the current one.

**And the discipline of §71.1's five questions:** what does the physics permit; what is the
link budget; what spectrum and who has it; what is the cost per covered area; and which promise
from the last generation does this repeat?

## Where to look next

**Chapter 72** is the last chapter and the one this book has been for: turning everything into
a design that can be defended — and the frontier material's role there is as a set of
constraints and options rather than as a set of predictions.
