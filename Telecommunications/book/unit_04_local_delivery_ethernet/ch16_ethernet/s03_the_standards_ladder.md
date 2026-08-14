# 16.3 The Standards Ladder

Forty-three years, a factor of eighty thousand in speed, and one unchanged frame
format. This section is about what changed, what did not, and the question the
chapter exists to answer.

## Reading the names

The naming convention encodes the specification, and reading it saves consulting a
table.

```
   10GBASE-SR
   │  │    │
   │  │    └── medium and reach: S=short-reach MMF, L=long SMF, T=twisted pair,
   │  │        R=64B/66B coding, X=8B/10B coding, 4=four lanes
   │  └─────── BASE = baseband signalling (Chapter 5 §5.4)
   └────────── rate: 10 Gb/s
```

Older names put the segment length in hundreds of metres instead: **10BASE5** was
10 Mb/s, baseband, 500 m maximum segment. **10BASE2** was 185 m, rounded up in the
name.

## The ladder

| Standard | Year | Rate | Medium | Reach | Coding |
|---|---|---|---|---|---|
| 10BASE5 | 1983 | 10 Mb/s | thick coax | 500 m | Manchester |
| 10BASE2 | 1985 | 10 Mb/s | thin coax | 185 m | Manchester |
| **10BASE-T** | 1990 | 10 Mb/s | **Cat3 UTP** | 100 m | Manchester |
| 100BASE-TX | 1995 | 100 Mb/s | Cat5 UTP | 100 m | 4B/5B + MLT-3 |
| 100BASE-FX | 1995 | 100 Mb/s | MMF | 2 km | 4B/5B |
| **1000BASE-T** | 1999 | 1 Gb/s | Cat5e, 4 pairs | 100 m | PAM-5 + trellis |
| 1000BASE-SX/LX | 1998 | 1 Gb/s | MMF / SMF | 550 m / 5 km | 8B/10B |
| 10GBASE-T | 2006 | 10 Gb/s | Cat6a | 100 m | PAM-16 + LDPC |
| 10GBASE-SR/LR | 2002 | 10 Gb/s | MMF / SMF | 400 m / 10 km | 64B/66B |
| 40GBASE-SR4 | 2010 | 40 Gb/s | MMF, 4 lanes | 150 m | 64B/66B |
| 100GBASE-LR4 | 2010 | 100 Gb/s | SMF, 4 λ | 10 km | 64B/66B |
| **2.5G/5GBASE-T** | 2016 | 2.5/5 Gb/s | **Cat5e/Cat6** | 100 m | PAM-16 + LDPC |
| 400GBASE-DR4 | 2017 | 400 Gb/s | SMF, 4 lanes | 500 m | PAM-4 + RS-FEC |
| 800GBASE / 1.6T | 2024– | 800 Gb/s+ | SMF | various | PAM-4 + FEC |

Every mechanism in the right-hand column is from Unit II — Chapter 7's line codes,
Chapter 7 §7.4's multilevel signalling, Chapter 8's modulation ideas applied to
baseband, and Chapter 4's coding gain. The ladder is Unit II applied repeatedly.

## What changed at each rung

**10BASE5 → 10BASE2.** Thinner, cheaper, more flexible coax; shorter segments. The
same logical bus, easier to install.

**10BASE2 → 10BASE-T (1990).** The most consequential step in the table, and it was
not about speed — the rate was unchanged.

It moved from a **shared coaxial bus** to a **physical star over twisted pair**, and
Chapter 11 §11.3's physical/logical distinction is the key: the logical topology was
still a bus, because the hub repeated every signal to every port. Nothing about
contention changed.

What changed was **operational**:

- **Fault isolation.** One cable fails, one station is affected. On the bus, one bad
  connector took down everyone and finding it meant walking the cable.
- **Non-disruptive changes.** Adding a station affects nobody.
- **Reuse of existing cabling.** Buildings already had twisted pair for telephones.
  Cat3 was already installed, and 10BASE-T's Manchester encoding at 10 Mb/s needs a
  10 MHz fundamental, which fits inside Cat3's 16 MHz (Chapter 7 §7.2).

Chapter 10 §10.2 argued that coax lost the LAN on operational rather than electrical
grounds. This is that argument's decisive moment.

**10BASE-T → 100BASE-TX (1995).** Ten times the rate on Cat5, achieved by
abandoning Manchester's 50% efficiency for 4B/5B's 80% and adding MLT-3 to cut the
fundamental frequency by four (Chapter 7 §7.2, §7.3). The bandwidth arithmetic is
worked in Chapter 5 §5.3.

**100BASE-TX → 1000BASE-T (1999).** A hundred times 10BASE-T's rate on cable
specified for a hundredth of it, by using **all four pairs**, **bidirectionally**,
with **five voltage levels** and **trellis coding**. Chapter 7 §7.4 does the
arithmetic: 250 Mb/s per pair, 2 bits per symbol, 125 Mbaud, about 62.5 MHz — inside
Cat5e's 100 MHz.

Simultaneous bidirectional transmission requires each transceiver to subtract its own
signal from what it receives — **echo cancellation**, borrowed from telephony.

**1000BASE-T → 10GBASE-T (2006).** PAM-16 at 800 Mbaud on four pairs, with LDPC
coding supplying the gain to offset a 23.5 dB SNR penalty. Requires Cat6a because of
alien crosstalk (Chapter 6 §6.4), and consumes noticeably more power than fibre
equivalents because the DSP is doing a great deal of work.

**And then 802.3bz (2016)**, which is the interesting one. **2.5GBASE-T and
5GBASE-T** are 10GBASE-T's signalling scaled down to fit Cat5e and Cat6 at 100 m.

Nothing about the cable changed. Cat5e specified in 1999 for gigabit now carries
2.5 Gb/s, purely by encoding harder. The standard exists because the world's
installed base of Cat5e is enormous and replacing it building by building is
prohibitive — a purely economic motivation producing a purely technical result, and
Chapter 10's opening claim that a medium's properties are a function of the current
manufacturing art.

## Why Ethernet won

The question the chapter is built around, and the reason it is interesting is that
Ethernet was, by the standards of 1985, the worse design.

**Token Ring** (IEEE 802.5) was **deterministic**: a station holding the token
transmits without contention, so the maximum delay before any station can transmit
is **bounded**. Under heavy load it degraded gracefully; Ethernet degraded badly.
IBM backed it, and IBM in 1985 was not a company against which one bet lightly.

Ethernet was a shouting match with a random backoff timer, with an unbounded worst
case and a capture effect.

Token Ring hardware is now a museum piece.

### Three reasons

**It was cheaper, at every point in its history.** A Token Ring adapter cost several
times an Ethernet one, because the protocol was more complex — token management,
monitor election, ring insertion and removal, error recovery — and complexity is
silicon. The gap never closed. For a technology sold in volume, a persistent cost
disadvantage is very difficult to overcome with technical superiority.

**It was good enough, and then the objection evaporated.** Token Ring's advantage was
determinism under contention. **Switching removed the contention.** Once every station
had its own collision domain and full duplex (§16.4), there was nothing to be
non-deterministic about — Ethernet became deterministic by construction, and Token
Ring's central argument no longer applied to anything.

That is worth stating precisely: Ethernet did not out-argue Token Ring. It changed
the terms so that the argument was moot.

**And the interface stayed stable while the implementation was replaced entirely.**

This is the reason that generalises furthest.

| Aspect | 1983 | 2026 | Changed? |
|---|---|---|---|
| **Frame format** | dest, src, type, payload, FCS | identical | **No** |
| **Addressing** | 48-bit MAC | identical | **No** |
| Medium | thick coax | twisted pair, fibre | Yes |
| Topology | bus | star / fabric | Yes |
| Coding | Manchester | PAM-4 + FEC | Yes |
| Arbitration | CSMA/CD | none | Yes — **removed** |
| Duplex | half | full | Yes |
| Rate | 10 Mb/s | 800 Gb/s | Yes — **80,000×** |

**Everything below the frame format was thrown away and rebuilt, four times over.
The frame format did not change.**

That is why an Ethernet driver written in 1985 describes a frame the same way one
written today does; why a network engineer's mental model transfers across four
decades of hardware; and why each new speed could be adopted incrementally, one link
at a time, without changing anything above.

**Standardise the interface, not the mechanism.** It is the lesson Chapter 21 makes
abstractly about layering, and Ethernet is its most complete demonstration.

## Where Ethernet went next

Having won the LAN, it went on to win markets it was never designed for:

- **Metro and wide area** — Carrier Ethernet, displacing SONET for many services
  (Chapter 50).
- **Data centre fabric** — leaf-spine at 100 and 400 Gb/s (Chapter 67 §67.4).
- **Storage** — iSCSI and FCoE, displacing Fibre Channel for many workloads.
- **Industrial control** — with TSN adding the determinism Token Ring had
  (Chapter 71 §71.4), which is a pleasing irony.
- **Automotive** — 100BASE-T1 and 1000BASE-T1 over a single twisted pair,
  displacing purpose-built vehicle buses.

Each is Chapter 14 §14.4's convergence pattern: a general-purpose substrate absorbing
a specialised one because its economics and rate of improvement are better.

## What breaks here

**Assuming a category rating is a data rate.** Cat5e is 100 MHz and carries 100 Mb/s,
1 Gb/s or 2.5 Gb/s depending on the transceiver. Chapter 5 §5.3.

**Assuming reach figures are universal.** 10GBASE-T is 100 m on Cat6a and 55 m or
less on Cat6, depending on bundling. Multimode reach depends on the OM grade.

**Reading a standard name wrongly.** `1000BASE-LX` is single-mode long-reach;
`1000BASE-SX` is multimode short. Putting the wrong transceiver on a fibre produces
no link or a marginal one.

**Expecting a new rate to work on old cable without checking.** 2.5GBASE-T on Cat5e
works and has less noise margin, so a marginal run fails at the higher rate first
(Chapter 7 §7.4's diagnostic scenario).

> **Network+ note.** Objective 1.5 expects the Ethernet standards, their media and
> their distances. **Learn to read the names** rather than memorising the table —
> rate, BASE, medium/reach — and the specific figures worth knowing cold are
> **100 m for all twisted pair**, **10GBASE-T needs Cat6a**, and **1000BASE-T uses
> all four pairs**.
