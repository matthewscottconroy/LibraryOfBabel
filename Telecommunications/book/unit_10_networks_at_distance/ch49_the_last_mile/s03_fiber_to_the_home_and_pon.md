# 49.3 Fibre to the Home and PON

**When you finally build new infrastructure, the question is how little of it you can get away
with** — and PON's answer is unusually elegant.

## The problem with the obvious design

**Point-to-point fibre** — one strand from the exchange to each home — is the simple design,
and it is what some operators build.

**Its costs are real:**

| | Point-to-point | |
|---|---|---|
| Fibre strands from the exchange | **one per home** | 10,000 homes = 10,000 strands |
| Duct and cable size | **large** | and duct space is the scarce resource |
| **Transceivers at the exchange** | **one per home** | power, space, cooling, capital |
| Bandwidth per home | **the whole fibre** | genuinely dedicated |

**A 10,000-home exchange needs 10,000 optical transceivers in a room, all powered and cooled**,
and a cable plant sized accordingly. **It works and it is expensive.**

## The splitter

**A passive optical splitter is a piece of glass that divides light.** No power, no
electronics, no active components, **nothing to fail and nothing to maintain.**

```
                              ┌── ONT (home)
                              ├── ONT
   OLT ═══one fibre═══ [1:32] ┼── ONT     one transceiver at the exchange,
   (exchange)          splitter├── ONT     one fibre in the duct,
                              ├── ONT     thirty-two homes
                              └── ⋮
```

| | |
|---|---|
| **OLT** — Optical Line Terminal | at the exchange; **one port serves the whole tree** |
| **ODN** — Optical Distribution Network | fibre and splitters; **entirely passive** |
| **ONT / ONU** | at the home |

> **The splitter has no power supply and no failure mode short of physical damage.** A PON's
> outside plant can sit in a sealed enclosure in a footway box for thirty years, **which is
> the property that makes fibre-to-the-home economic.**

**Splitting is lossy, and the loss is the design constraint.**

$$\text{loss} = 10 \log_{10}(N) + \text{excess}$$

| Split | Ideal loss | With excess | GPON share if all active |
|---|---|---|---|
| **1:8** | 9.0 dB | ~10 dB | **311 Mb/s** |
| **1:16** | 12.0 dB | ~13 dB | 155 Mb/s |
| **1:32** | **15.1 dB** | **~17 dB** | **78 Mb/s** |
| 1:64 | 18.1 dB | ~20 dB | 39 Mb/s |
| 1:128 | 21.1 dB | ~23 dB | 19 Mb/s |

**A GPON Class B+ link budget is 28 dB.** Spend 17 dB on a 1:32 split and **5 dB on 20 km of
fibre** (at roughly 0.25 dB/km at 1490 nm), **and 22 dB of the 28 is gone** — leaving margin
for connectors, splices and ageing. **This arithmetic is why 1:32 over 20 km is the standard
deployment and why deeper splits require shorter reach or better optics.**

## Downstream is a broadcast

**The OLT transmits once and every ONT receives everything.**

**Which means, exactly as in DOCSIS (§49.2), that separation is cryptographic and not
physical.** Each ONT is assigned a **GEM port ID** and discards frames not addressed to it —
**and a modified ONT would not.** **GPON therefore encrypts downstream traffic with AES, with
a per-ONT key.**

> **Every subscriber on a PON physically receives every other subscriber's photons.** The
> privacy is a decision, implemented in software, and it is worth knowing that it is a
> decision.

**Upstream cannot be a broadcast**, because thirty-two ONTs transmitting simultaneously into
one fibre would collide in the splitter. **So upstream is time-division multiple access**
(Chapter 9), scheduled by the OLT.

## Ranging: the interesting problem

**ONTs are at different distances from the splitter** — one 200 m away, another 18 km — **and
light in glass takes about 4.9 µs per kilometre** (Chapter 6). A grant travels out and the
burst travels back, so **that 17.8 km spread puts arrival times about 175 µs apart**: an ONT
told to transmit "now" would land anywhere in that window depending on where it is.

**So each ONT must be measured and compensated.**

```
   OLT ──── "quiet window: everyone stop transmitting" ────▶
   OLT ──── ranging request ───────────────────────────────▶
   new ONT ─── response ──────────────────────────────────▶
   OLT: measures round-trip delay, computes equalisation delay
   OLT ──── "your offset is 47.3 µs" ─────────────────────▶
```

**The OLT opens a quiet window**, discovers the new ONT, measures its round-trip time, and
**assigns an equalisation delay that makes every ONT appear to be at the same distance.**

> **After ranging, all ONTs are logically equidistant**, and the OLT can schedule upstream
> slots with only a small guard interval between bursts. **It is the same problem as DOCSIS's
> ranging (§49.2) and as LTE's timing advance (Chapter 46 §46.3)** — and all three solve it
> the same way, because there is only one way to solve it.

**Dynamic Bandwidth Allocation (DBA)** then does the scheduling: **ONTs report how much they
have queued, and the OLT grants slots accordingly**, typically every 125 µs. **An idle ONT
consumes almost no upstream capacity**, which is what makes the sharing efficient.

## The generations

| Standard | Downstream | Upstream | Wavelengths (down/up) |
|---|---|---|---|
| **GPON** (G.984) | **2.488 Gb/s** | **1.244 Gb/s** | 1490 / 1310 nm |
| **XGS-PON** (G.9807) | **9.953 Gb/s** | **9.953 Gb/s** | **1577 / 1270 nm** |
| 25GS-PON | 25 Gb/s | 25 Gb/s | |
| **50G-PON** (G.9804) | 50 Gb/s | 25–50 Gb/s | |
| EPON / 10G-EPON | 1.25 / 10 Gb/s | symmetric | IEEE, Ethernet framing |

**The wavelength choice is the important detail**, and it is why upgrades are tractable.

**GPON and XGS-PON use different wavelengths**, so **both can run on the same fibre and the
same splitters simultaneously** — a WDM filter at the OLT separates them. **An operator
upgrades a subscriber by swapping the ONT**, not by touching the outside plant.

> **The passive network outlives several generations of electronics**, which is the whole
> economic argument for building it. **Trenching is a fifty-year investment; transceivers are
> a seven-year one.**

**And 1550 nm is left free** for an RF video overlay — analogue television broadcast over the
same fibre — which some operators used during the transition and most have now retired.

## What the sharing means

**A 1:32 GPON gives 2.488 Gb/s among 32 homes: 78 Mb/s each if all transmit continuously.**

**They do not**, and the statistical multiplexing argument (Chapter 9) applies exactly as it
does for DOCSIS. **The difference is the numbers:**

| | **DOCSIS node** | **GPON tree** |
|---|---|---|
| Homes sharing | **100–500** | **16–64** |
| Shared capacity down | ~1–10 Gb/s | 2.5 Gb/s (GPON) |
| **Shared capacity up** | **~200 Mb/s – 1 Gb/s** | **1.25 Gb/s** |
| **Symmetry** | **poor** | **better; symmetric on XGS** |
| Noise ingress | **the noise funnel** | **none — it is glass** |

**PON shares less among fewer, and its upstream is not architecturally crippled.** **That
combination — plus immunity to water, corrosion, electrical noise and crosstalk — is why fibre
wins wherever the capital can be found**, and the argument is not really about bandwidth.

## What breaks here

**One home dark, neighbours fine.** **The drop fibre or the ONT.** A bend, a dirty connector,
a rodent. **Check the ONT's received optical power** — it should be roughly −8 to −27 dBm on
GPON.

**A whole street dark.** **The feeder fibre or the splitter enclosure.** Almost always physical
— a contractor's excavator is the leading cause of fibre outages worldwide.

**An ONT that will not register.** Ranging failed, the serial number is not provisioned, or
the optical power is out of range. **The OLT's log names which.**

**Degrading power over months.** **A dirty or damaged connector**, or water in an enclosure.
**Fibre does not gradually get worse on its own; something is happening to it.**

**Slow at peak with good optics.** **Tree congestion** — too many homes on one PON port. The
fix is to re-split, which means splitting the tree at the OLT, and it is planned work.

**A working ONT after a lightning strike, but no service.** Fibre is immune to induced
surges; **the power supply and the copper Ethernet side are not.** Check the ONT's power and
the LAN port before suspecting the line.

> **Network+ note.** Objective 1.5 covers fibre and 2.1 touches PON. Over-learn: **PON uses a
> passive splitter to share one fibre among many subscribers**; **GPON is 2.5 Gb/s down and
> 1.25 up, shared**; **the ONT is at the customer and the OLT at the provider**; and **single-
> mode fibre is used for long distances, multimode for short.** The passive-splitter concept is
> the examinable idea.
