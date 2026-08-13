# 45.1 Site Surveys and AP Placement

**A site survey answers one question: where do the access points go?** Everything in
Chapters 42–44 exists to make that answer derivable rather than guessed.

## The three kinds

| Survey | When | Method | Cost |
|---|---|---|---|
| **Predictive** | **before building** | modelling software + floor plans | **low** |
| **Passive** | after deployment | walk and listen | moderate |
| **Active** | after deployment | walk and **associate** | moderate |
| **Spectrum** | when diagnosing | analyser (Ch 43 §43.4) | needs hardware |

**They answer different questions and a competent project uses all of them.**

### Predictive

**Model the building and let software place the access points.**

**What you supply:** floor plans to scale, **wall materials and their attenuation**
(Chapter 42 §42.1), ceiling height, the access-point model, and the coverage target.

**What it produces:** a placement plan, predicted coverage heat maps, channel assignments and
a bill of materials.

**Its accuracy depends entirely on the wall data.** A plan drawn with every internal wall
marked "drywall" when half are brick **will be wrong by 6–10 dB in those areas**, and the
error compounds through several walls.

> **Predictive surveys are as good as their inputs and no better.** They are excellent for
> estimating quantity and cost, and they are not a substitute for measuring.

**Use it for:** budgeting, new construction, and a starting point.

### Passive

**Walk the building listening**, without associating. The survey tool records **RSSI, SNR,
channel and noise floor** at every point, and produces a heat map of what is actually there.

**It measures coverage** — which access points are audible where, and how strongly.

**It does not measure whether the network works.** A client may hear an access point at
−60 dBm and be unable to pass traffic (Chapter 43 §43.4).

### Active

**Walk while associated**, and measure **throughput, retries, roaming behaviour and packet
loss.**

**This is the one that tells you whether the network works**, and it is the one most often
skipped because it takes longer.

**Two variants:**

**Associated to a specific BSSID** — measures one access point's real coverage.

**Associated normally, roaming freely** — measures the client experience, including
**where roaming happens and whether it is clean** (§45.2).

### Doing it properly

**Four rules that are frequently broken:**

**Survey with the client devices you will use.** A survey adapter with a good antenna hears
access points a phone cannot. **The measurement that matters is what your worst common device
experiences** — and in most organisations that is a phone or a handheld scanner, not a laptop.

**Survey the building as it will be occupied.** Chapter 42 §42.1 — bodies absorb, and
furniture, stock and machinery all matter. **A survey of an empty warehouse before the racking
arrives is worthless.**

**Survey while moving, slowly.** Chapter 42 §42.4's fast fading means a stationary reading
samples one point in a pattern varying over centimetres.

**Survey every floor, and remember the vertical.** Access points above and below are
neighbours (Chapter 43 §43.2).

## The coverage target

**The number to design against**, and it follows from Chapter 42 §42.3's sensitivity table.

| Requirement | Target RSSI | SNR |
|---|---|---|
| **Voice and video** | **−67 dBm** | **25 dB** |
| Data, high performance | −67 dBm | 25 dB |
| **Data, general** | **−70 dBm** | 20 dB |
| Basic connectivity | −75 dBm | 15 dB |
| Location tracking | −75 dBm, **from three APs** | — |

**−67 dBm is the standard figure**, and the reason is not arbitrary: it is roughly where a
typical client sustains a data rate high enough that airtime consumption stays reasonable
(Chapter 44 §44.2's fairness argument).

**And "coverage" means coverage at the target, not merely association.** A client will
associate at −85 dBm and will do so at a low MCS, consuming disproportionate airtime and
degrading the cell for everyone.

> **Design for the rate you want, not for the signal a client will tolerate.**

**And design at the 5 GHz or 6 GHz level** (Chapter 43 §43.3) — **the shorter-range band
determines the spacing**, and a design meeting −67 dBm at 2.4 GHz will have holes at 5 GHz.

## Placement

**The principles, and each has a reason from Chapters 42–43:**

**Ceiling-mounted, in open space.** Access-point antennas are designed for a downward
hemisphere. **Above a suspended ceiling is acceptable if the tiles are not foil-backed;
inside a metal enclosure is not** (Chapter 42 §42.4).

**Not in corners or against outside walls**, unless you intend to cover outward. **Half the
pattern is wasted**, and it also leaks signal outside the building (Chapter 45 §45.4's
security note).

**Not directly above the highest-density area.** Chapter 42 §42.2's null: **a ceiling-mounted
omni has weak coverage directly beneath it.** Offset it.

**Away from metal, lift shafts, ductwork and large water tanks.** Reflection and absorption.

**Away from other interferers** — microwave ovens, motors, industrial equipment
(Chapter 43 §43.4).

**Where power and cable can reach.** PoE (Chapter 16 §16.4), and **cable runs are limited to
100 m** — which constrains placement more often than people expect and should be checked
against the comms room locations before the plan is finalised.

**And for directional coverage, use directional antennas.** A corridor wants a patch antenna
along it, not an omni in the middle of it.

## Cell size and overlap

**The two numbers that determine the design.**

**Cell size** is controlled by **transmit power** and **the minimum data rate** (Chapter 44
§44.2):

| To make cells | Do |
|---|---|
| **Smaller** | **reduce power**; **raise the minimum basic rate** |
| Larger | raise power; lower the minimum rate |

**Raising the minimum basic rate is the better lever**, because it shrinks the cell **without
creating the asymmetry** that reducing only the access point's power avoids but that raising
it causes (Chapter 42 §42.2).

**Overlap** should be **15–20%** between adjacent cells at the target RSSI.

| Overlap | Consequence |
|---|---|
| **Too little** | **coverage holes**; roaming fails or is slow |
| **15–20%** | **clean roaming** |
| **Too much** | **co-channel contention** (Ch 43 §43.4); clients stay associated too long |

**And overlap is measured at the design threshold**, not at the edge of audibility. **Two
cells that overlap at −85 dBm do not overlap for design purposes.**

## Capacity changes the arithmetic

**Coverage design asks "can a client hear an access point?" Capacity design asks "how many
clients can this access point serve?"** — and §45.3 develops the difference.

**The short version:** in a dense environment, **the number of access points is determined by
client count rather than by area**, and cells are deliberately made small so that fewer
clients share each one.

**Which is why a lecture theatre may need six access points in a space one would cover.**

## The deliverable

**A survey report should contain:**

| | |
|---|---|
| **Heat maps** | RSSI, SNR, and **data rate** — per band |
| **AP placement** | on the floor plan, with mounting notes |
| **Channel and power plan** | per access point, per band |
| **Coverage gaps** | identified explicitly, with the reason |
| **Interference found** | from the spectrum survey |
| **Assumptions** | client type, occupancy, target |
| **Cable runs** | with lengths, checked against 100 m |

**The assumptions section is the one that matters six months later**, when someone asks why
coverage is poor in a room that was surveyed — and the answer is that it was surveyed empty,
for laptops, and is now full of people using phones.

## What breaks here

**A predictive survey that did not match reality.** Wall materials, or occupancy.

**Coverage measured with a survey adapter and complaints from phones.** Survey with the worst
common client.

**A hole directly under an access point.** Chapter 42 §42.2's null.

**Coverage good at 2.4 GHz and holes at 5 GHz.** Designed for the wrong band.

**Roaming failing at cell edges.** Insufficient overlap.

**Everything overlapping and performing badly.** Too much overlap, and co-channel contention.

**An access point that cannot be installed where the plan says.** Cable length, power, or
mounting. Check before finalising.

> **Network+ note.** Objective 3.1 expects site surveys and heat maps; objective 2.4 expects
> coverage considerations. Over-learn: **the three survey types and what each measures**;
> **−67 dBm as the voice design target**; **15–20% cell overlap for roaming**; and **survey
> with the actual client devices, in the occupied state.**
