# 49.2 Cable and DOCSIS

**The cable operators had a better wire and a worse architecture**, and both facts still shape
the service.

## The plant they inherited

**Cable television networks were built to broadcast**, one way, from a head end to every home
in a region, **and the medium is coaxial cable — which passes 750 MHz to 1.2 GHz** where
telephone twisted pair struggles above 2 MHz.

```
   Head end ══fibre══ Node ══coax══ ┬── home
                                    ├── home     one coaxial branch,
                                    ├── home     amplified, shared by
                                    ├── home     100–500 homes
                                    └── home
```

**Modern plant is hybrid fibre-coax (HFC)**: fibre from the head end to a neighbourhood
**optical node**, coaxial from there to the homes, with amplifiers along the way.

**Two properties follow, and they are the whole story:**

**The bandwidth is enormous.** A 1 GHz plant carries roughly 160 six-megahertz television
channels' worth of spectrum, **any of which can carry data instead of television.**

**It is shared.** Everyone on a coaxial branch receives everything, and everyone transmits
into the same amplifier chain. **This is Chapter 16's shared medium, at neighbourhood scale.**

> **DSL gives each subscriber a private wire of poor quality. DOCSIS gives them a share of an
> excellent one.** Every difference in behaviour between the two services comes from that
> sentence.

## The spectrum split, and why upstream is the problem

**The plant was built for one-way television**, so the return path was an afterthought — a
narrow slice at the bottom of the spectrum, **below where the television channels start.**

```
   ┌──────────┬──────────────────────────────────────────────┐
   │ 5–42 MHz │        54 MHz  –  1002 MHz                   │
   │ UPSTREAM │              DOWNSTREAM                      │
   │  37 MHz  │                948 MHz                       │
   └──────────┴──────────────────────────────────────────────┘
        ▲
   and it is the noisiest part of the band
```

**Two independent problems live in that 37 MHz.**

**It is small.** Twenty-five times less spectrum than downstream, which sets the asymmetry
before any modulation choice is made.

**It is noisy, and the noise adds up.** Every home's return path is combined on the way to the
node, **so every home's ingress noise — from a corroded connector, an unterminated splitter, a
badly-shielded drop, a nearby transmitter — arrives at the head end summed with everyone
else's.** This is the **noise funnel**, and it is peculiar to the upstream direction.

> **One faulty connector in one home can degrade the upstream for an entire node**, and
> finding it requires physically walking the plant. **It is the characteristic cable fault and
> it has no equivalent in DSL**, where a bad line affects only its own subscriber.

**Mid-split and high-split** move the boundary upward — to 85 MHz or 204 MHz — **and DOCSIS
4.0's full duplex goes further**, but every change requires replacing amplifiers throughout the
plant, which is why the asymmetry has proved so durable.

## The DOCSIS generations

| Version | Year | Downstream | Upstream | Mechanism |
|---|---|---|---|---|
| 1.0 / 1.1 | 1997/99 | **40 Mb/s** | 10 Mb/s | one channel each |
| **2.0** | 2001 | 40 Mb/s | **30 Mb/s** | better upstream modulation |
| **3.0** | 2006 | **~1.4 Gb/s** | ~200 Mb/s | **channel bonding** |
| **3.1** | 2013 | **10 Gb/s** | **1–2 Gb/s** | **OFDM, LDPC, 4096-QAM** |
| **4.0** | 2017 | 10 Gb/s | **6 Gb/s** | **full duplex / extended spectrum** |

**Channel bonding (3.0) is the significant one architecturally.** A single 6 MHz QAM-256
channel carries **about 43 Mb/s raw** — arithmetic worth doing once:

$$5.36 \text{ Msym/s} \times 8 \text{ bits/symbol} \approx 42.9 \text{ Mb/s}$$

**Bond 32 of them and you have 1.37 Gb/s**, delivered as one logical pipe. **The modem tunes
32 receivers simultaneously** and the CMTS schedules across all of them.

**DOCSIS 3.1 replaces the 6 MHz channel grid entirely** with OFDM blocks up to 192 MHz wide,
**subcarriers of 25 or 50 kHz**, and per-subcarrier modulation up to 4096-QAM — **which is
exactly the DMT bit-loading of §49.1, and exactly Wi-Fi's OFDMA (Chapter 44 §44.4), arriving
in a third medium for the third time.**

> **The same idea keeps appearing: divide the band into many narrow subcarriers, measure each
> one, and load bits according to what each will actually carry.** DSL called it DMT, Wi-Fi
> and LTE call it OFDM, DOCSIS 3.1 adopted it last. **It is the right answer whenever a
> channel's quality varies across frequency.**

## How the medium is shared

**Downstream is a broadcast.** The CMTS transmits; every modem on the branch receives
everything; **each modem discards what is not addressed to it.**

**Which means, exactly as in PON (§49.3), that privacy is cryptographic rather than
physical.** DOCSIS **BPI+** encrypts each modem's traffic with its own key. **Without it, a
modem in promiscuous mode would see the neighbourhood's traffic** — and in the mid-1990s,
before BPI, it could.

**Upstream is scheduled.** A modem cannot simply transmit: **it requests, and the CMTS grants
a time slot.**

```
   Modem ──── request (in a contention minislot) ────▶ CMTS
   Modem ◀─── grant: "transmit at time T for N µs" ─── CMTS
   Modem ──── data, in exactly that slot ───────────▶ CMTS
```

**The request itself uses contention** — a small ALOHA-like window (Chapter 16 §16.1) where
collisions are possible and retried with backoff. **Data transmission does not collide,
because it is scheduled.**

**And ranging is required**, for the same reason as in PON: modems are at different distances,
**so each is told a timing offset and a power level so that its burst arrives at the head end
in its assigned slot at the right amplitude.** A modem that loses ranging is a modem that has
dropped off the network.

## What sharing actually means for the subscriber

**The complaint pattern is diagnostic and worth understanding precisely.**

| | **DSL** | **DOCSIS** |
|---|---|---|
| Medium | **dedicated pair** | **shared branch** |
| Rate limit set by | **your loop length** | **the branch's load** |
| Neighbour's usage | irrelevant | **directly relevant** |
| Typical complaint | **"it's always this slow"** | **"it's slow at 8 p.m."** |
| Fault isolation | **your line** | **may be anyone's connector** |

**A worked example.** A node serves 300 homes with 1 Gb/s of bonded downstream.

**If every home used its full 100 Mb/s package simultaneously**, the requirement would be
30 Gb/s and the service would be unusable. **It is not, because they do not.** At a plausible
evening peak — **5% of homes actively transferring, at an average of 25 Mb/s** — the demand is
**375 Mb/s against 1 Gb/s**, and everything is fine.

**This is Chapter 9's statistical multiplexing**, and the operator's job is to watch the
utilisation and **split the node when it approaches saturation** — replacing one node serving
300 homes with two serving 150 each, which halves the contention. **Node splitting is to cable
what cell splitting is to cellular** (Chapter 46 §46.1) and what adding access points is to
Wi-Fi (Chapter 45 §45.3): **the same answer for the same reason, in a third medium.**

> **"Up to 1 Gb/s" on a cable service is not a lie and it is not a guarantee.** It is the
> capacity you receive when your neighbours are not using theirs, **and the honest question to
> ask an operator is not the headline rate but the node's peak utilisation.**

## What breaks here

**Slow only in the evenings.** **Node congestion.** The fix is a node split and it is the
operator's capital decision, not something a subscriber can influence.

**Intermittent dropouts affecting several homes on one street.** **Upstream ingress noise** —
the noise funnel. Someone's connector, splitter or drop. **It requires walking the plant.**

**High upstream SNR errors and a modem that keeps re-ranging.** Same cause, seen from the
modem's statistics. **The modem's signal page is genuinely informative**: downstream power
should sit near 0 dBmV, upstream transmit power below about 51 dBmV, **and an upstream power
pinned at maximum means the modem is shouting to be heard.**

**Fine downstream, poor upstream.** **Expected**, architecturally — and it is the reason video
calls and cloud backups behave worse than the headline rate suggests.

**One home's fault taking out a street.** **The characteristic cable failure**, and the thing
that most surprises engineers who came from DSL.

**A modem that will not come online after a power cut.** Ranging or registration failing.
**Check the downstream lock first** — the sequence is downstream lock, upstream ranging, DHCP,
configuration file, registration, **and the modem's status page names the step it stopped at.**

> **Network+ note.** Objective 1.5 and 2.1. Over-learn: **cable broadband uses DOCSIS over
> hybrid fibre-coax**; **the medium is shared among neighbours, so performance varies with
> local demand**; **it is asymmetric, with far more downstream than upstream**; and **a cable
> modem connects to a CMTS at the head end.** The shared-medium consequence is the examinable
> idea and the one that explains the complaints.
