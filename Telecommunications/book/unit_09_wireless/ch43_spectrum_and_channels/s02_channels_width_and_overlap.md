# 43.2 Channels, Width and Overlap

"Use channels 1, 6 and 11" is the most repeated advice in wireless networking. It is
correct, and almost nobody who repeats it can say why — which means they cannot reason about
the cases where it does not apply.

**This section derives it.**

## What a channel is

A channel is a centre frequency plus a bandwidth. A signal occupies a range around its
centre, and that occupied range is what matters for interference — not the centre
frequency alone.

```
        │◀────── 20 MHz ──────▶│
        │                      │
    ────┤▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁├────
        │      centre          │
     2.401                  2.423 GHz
```

**And the sides do not fall vertically.** A transmitter's power decays outside its nominal
channel according to a **spectral mask**, so energy spills into neighbouring frequencies —
which is why "adjacent channel" interference exists at all.

## The 2.4 GHz problem

The band is 2.400–2.4835 GHz — about 83.5 MHz (Chapter 43 §43.1).

Channels are defined every 5 MHz, numbered 1 to 14:

| Channel | Centre |
|---|---|
| 1 | 2.412 GHz |
| 2 | 2.417 GHz |
| 3 | 2.422 GHz |
| … | (+5 MHz each) |
| 11 | 2.462 GHz |
| 13 | 2.472 GHz (Europe) |
| 14 | 2.484 GHz (Japan, 802.11b only) |

But each channel is 20 MHz wide — and they are spaced 5 MHz apart.

> **The channels overlap.** A 20 MHz signal centred on channel 1 extends over what would be
> channels 1 through 5.

**The arithmetic:** channel numbers are 5 MHz apart, so *n* channel numbers of separation
gives 5*n* MHz of frequency separation. To clear a 20 MHz signal you need at least
20 MHz, so at least 4 channel numbers — and 5 gives a margin:

$$5 \text{ channel numbers} \times 5\ \text{MHz} = 25\ \text{MHz} > 20\ \text{MHz width}$$

**So non-overlapping channels are 5 apart:**

$$1,\ 6,\ 11$$

**Check it directly:** channel 1 is centred at 2412 and spans **2402–2422**; channel 6 is
centred at 2437 and spans **2427–2447**. A 5 MHz gap between them. Channels 1 and 5
would span 2402–2422 and 2422–2442 — **touching exactly**, with no margin for the spectral
mask's skirts, which is why 1/6/11 rather than 1/5/9 is the American convention.

```
   Ch:    1    2    3    4    5    6    7    8    9   10   11   12   13
        ┌─────────────────┐
        │   channel 1     │
        └─────────────────┘
                          ┌─────────────────┐
                          │   channel 6     │
                          └─────────────────┘
                                            ┌─────────────────┐
                                            │   channel 11    │
                                            └─────────────────┘
   2.401                                                          2.473 GHz
```

**Three non-overlapping channels.** In Europe, where channel 13 is available, 1, 5, 9, 13
gives four — with slightly imperfect separation that is generally accepted.

**And that is all there is.** The 2.4 GHz band supports three simultaneous
non-interfering networks, which for any building with more than a few access points is
the binding constraint.

## Why overlap is worse than sharing

The point that makes the rule matter, and it is the reverse of what intuition suggests.

Two access points on the same channel hear each other and **take turns** — CSMA/CA
(Chapter 44 §44.2) works as designed. They share the capacity, and each gets roughly half.

Two access points on *overlapping* channels — say 1 and 3 — cannot decode each other.
Each sees the other's transmissions as **raised noise floor**, not as a signal to defer to.

> **They transmit over each other, continuously, and both fail.** Neither backs off because
> neither recognises the other as a transmission.

**The result:**

| Configuration | Outcome |
|---|---|
| Same channel | **share the medium; each gets ~50%** |
| **Partially overlapping** | **corrupt each other; both get far less than 50%** |
| Non-overlapping | full capacity each |

So a deployment using channels 1, 3, 5, 7, 9, 11 — which sounds like more channels —
performs substantially worse than one using 1, 6, 11.

This is the single most common wireless configuration error, and it comes from the
reasonable-sounding belief that using more of the band must be better.

## Channel width — the capacity trade

**Wider channels carry more data.** Shannon (Chapter 4 §4.2): capacity is proportional to
bandwidth.

| Width | Relative capacity | Channels available (5 GHz, US) |
|---|---|---|
| **20 MHz** | 1× | **~25** |
| **40 MHz** | ~2× | ~12 |
| **80 MHz** | ~4× | **~6** |
| **160 MHz** | ~8× | **~2** |
| 320 MHz (6 GHz, Wi-Fi 7) | ~16× | ~3 (US) |

And the trade is exact: every doubling of width halves the number of channels.

**Two further costs:**

**Wider channels are noisier.** Thermal noise power is proportional to bandwidth
(Chapter 42 §42.1), so a 80 MHz channel has 6 dB more noise than a 20 MHz one — which
costs range, because the SNR at a given distance is lower.

Wider channels are more likely to overlap something. An 80 MHz channel spans four
20 MHz channels' worth of spectrum, so the probability that some neighbour is using part
of it is four times higher.

### The guidance

| Environment | Width | Reasoning |
|---|---|---|
| **2.4 GHz, always** | **20 MHz** | there is only room for three channels as it is |
| **High-density enterprise (5 GHz)** | **20 or 40 MHz** | **channel reuse matters more than per-client speed** |
| Typical office | 40 MHz | a reasonable compromise |
| **Home, few neighbours** | **80 MHz** | fewer competing networks; speed is the goal |
| **6 GHz, enterprise** | **80 or 160 MHz** | **there is enough spectrum to afford it** |

> **In a dense deployment, narrow channels are better.** More channels means less
> co-channel contention, and contention costs more than the per-client rate gains from a
> wider channel.

This is counter-intuitive and it is the correct answer, and Chapter 45 §45.3 develops
it as a design principle.

**A concrete illustration:** forty access points in a building.

- With 20 MHz channels in 5 GHz, ~25 channels are available, so most access points can
  have a channel to themselves.
- With 80 MHz channels, ~6 are available, so each channel is shared by about seven
  access points — and they contend with each other continuously.

The 80 MHz deployment has four times the theoretical per-channel rate and roughly seven
times the contention, and in practice performs worse.

## The 5 GHz channels

More complicated than 2.4 GHz, and the complication is DFS.

| Sub-band | Channels | Notes |
|---|---|---|
| **UNII-1** | 36–48 | **indoor, no DFS** — the popular ones |
| **UNII-2A** | 52–64 | **DFS required** |
| **UNII-2C** | 100–144 | **DFS required**; some are weather-radar channels |
| **UNII-3** | 149–165 | **no DFS**, higher power permitted (US) |

Channels are numbered every 5 MHz as at 2.4 GHz, but the allocated 20 MHz channels are
spaced 20 MHz apart — 36, 40, 44, 48 — so they do not overlap.

> This is the crucial difference: in 5 GHz, adjacent channel numbers do not overlap.
> Channels 36 and 40 are neighbours in numbering and separate in frequency.

Wider channels are formed by bonding adjacent ones:

```
   20 MHz:   36    40    44    48    52 ...
   40 MHz:   └─36─┘      └─44─┘      └─52─┘
   80 MHz:   └────── 36 ──────┘      └── 52 ...
   160 MHz:  └──────────── 36 ────────────┘
```

And the bonding is fixed — an 80 MHz channel starting at 36 occupies 36, 40, 44 and 48,
so it cannot coexist with anything on those.

**The DFS consequence** (§43.1): a deployment avoiding DFS has UNII-1 and UNII-3 only —
about 9 channels at 20 MHz, 4 at 40 MHz, 2 at 80 MHz. Which is why avoiding DFS is
expensive, and why enterprise deployments generally use it despite the radar-event risk.

## The 6 GHz channels

**The reason Wi-Fi 6E and 7 matter.**

| Width | Channels (US, 1,200 MHz) | Channels (EU, 480 MHz) |
|---|---|---|
| 20 MHz | **59** | 24 |
| 40 MHz | 29 | 11 |
| **80 MHz** | **14** | **5** |
| **160 MHz** | **7** | **2** |
| 320 MHz | 3 | 1 |

Seven 160 MHz channels in the US — compared with two in 5 GHz — changes the design
calculus entirely. Wide channels become affordable because there are enough of them.

**And there is no legacy.** No 802.11b devices, no ancient clients forcing protection
mechanisms, and — for now — no neighbours, because only recent devices can use it.

The catch is propagation (Chapter 42 §42.3): 6 GHz is another ~1.6 dB of free-space loss
above 5 GHz and worse penetration, so cells are smaller and more access points are
needed for the same coverage.

## Channel planning

**In 2.4 GHz** — a solved problem with a bad solution:

```
   Floor plan, three-channel reuse:

     [1]     [6]     [11]    [1]
        [11]    [1]     [6]
     [6]     [11]    [1]     [6]
```

Adjacent access points must differ, and so must diagonal neighbours — and with three
channels in two dimensions, every access point has a co-channel neighbour within two
cells. This is unavoidable and is why 2.4 GHz is treated as a coverage band for legacy and
IoT devices rather than a capacity band.

**In 5 GHz** — genuinely solvable with enough channels, and automatic channel selection
usually does it adequately.

**The manual cases worth knowing:**

- **Verify what the automatic algorithm chose.** It optimises what it can measure, which is
  not always what matters.
- **Consider vertical separation.** Access points on the floor above and below are
  neighbours, and a two-dimensional plan misses them.
- Static assignment for high-density areas — lecture theatres, auditoria — where the
  automatic algorithm's periodic changes are disruptive.

## What breaks here

Poor performance with several access points on channels 1, 3, 5, 7, 9, 11. Partial
overlap. Use 1, 6, 11.

A high-density deployment underperforming with 80 MHz channels. Too few channels, too
much co-channel contention. Narrow them.

**Only four 5 GHz channels available.** DFS is disabled. Enable it, and accept the
occasional radar event.

5 GHz coverage smaller than expected after moving to 160 MHz. Wider channel, higher
noise, lower SNR at range.

Interference from an access point two floors up. Channel plans are three-dimensional.

A channel plan that was correct and degraded. A neighbour deployed something. You have
no recourse (§43.1) and must re-plan around them.

> **Network+ note.** Objective 2.4 expects channels and non-overlapping channel selection,
> and **this is examined directly.** Over-learn: 2.4 GHz has three non-overlapping
> channels — 1, 6 and 11; 5 GHz has many more and adjacent channel numbers do not
> overlap; **wider channels give more throughput and fewer channels**; and **partially
> overlapping channels are worse than sharing one.** The 1/6/11 answer is guaranteed to
> appear.
