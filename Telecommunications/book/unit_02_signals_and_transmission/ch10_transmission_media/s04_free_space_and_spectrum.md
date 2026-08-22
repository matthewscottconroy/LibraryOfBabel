# 10.4 Free Space and Spectrum

The medium nobody installs and everybody shares.

Chapter 42 develops radio propagation properly. This section covers free space as a
*medium to be chosen* — what it costs, what it requires, and where it is the right
answer.

## Spectrum as a regulated resource

The distinguishing feature of free space is that **you do not own it and cannot
buy exclusive use of it without permission.**

Every other medium in this chapter is property. You buy the cable, you install it,
and nobody else's signal is on it. Spectrum is a shared natural resource,
administered nationally by regulators — the FCC in the United States, Ofcom in the
UK, the BNetzA in Germany — under an international framework coordinated by the
ITU-R through the World Radiocommunication Conference.

Three regimes, and the choice between them is a design decision with commercial
consequences:

**Licensed.** You pay, sometimes enormously, for exclusive use of a band in a
geographic area. In exchange you get legal protection: nobody else may transmit
there, interference is an enforceable offence, and you can plan capacity with
confidence and use high power. Cellular operators live here. The 2021 US C-band
auction raised \$81 billion, which is a useful figure for calibrating how much
predictability is worth.

**Unlicensed.** Free, shared, no protection. You must accept interference from
others and must not cause harmful interference to licensed users. Wi-Fi, Bluetooth,
Zigbee and most short-range devices live here. Chapter 43 §43.1 tells the story of
how the ISM bands became available and why they were regarded as worthless.

**Lightly licensed / shared.** Intermediate arrangements. The US CBRS band at
3.5 GHz uses a spectrum access system granting priority tiers dynamically;
6 GHz standard-power operation requires automated frequency coordination to protect
incumbent microwave links. These schemes attempt to recover some of licensed
spectrum's predictability without its cost, and they are where much of the current
regulatory experimentation is happening.

## What the band determines

The frequency chosen is not a free parameter; it fixes several properties at once.

| Band | Typical use | Range | Penetration | Bandwidth available |
|---|---|---|---|---|
| Sub-1 GHz | LPWAN, legacy cellular, TV | **Best** | **Best** | Little |
| 2.4 GHz | Wi-Fi, Bluetooth, Zigbee | Good | Good | ~80 MHz |
| 5 GHz | Wi-Fi, some fixed links | Moderate | Moderate | ~500 MHz |
| 6 GHz | Wi-Fi 6E/7 | Shorter | Poorer | **~1,200 MHz** |
| 24–86 GHz (mmWave) | 5G, fixed links, backhaul | **Very short** | Very poor | **Enormous** |
| 60 GHz | WiGig, short links | Metres | None | 7 GHz+ |

The pattern, which Chapter 42 §42.2 derives: **higher frequency means more
bandwidth available and less range**, because free-space path loss contains a
20 log₁₀(*f*) term and because absorption by materials rises with frequency.

The 60 GHz case is the extreme and is instructive: oxygen absorbs strongly at
60 GHz, roughly 15 dB/km, which makes it useless for distance and *ideal* for dense
short-range reuse, since one room's transmission does not reach the next.

## Where free space is the right choice

**When installing cable is impossible or prohibitive.** Across a public road, a
river, a railway, a listed building's fabric, or a leasehold boundary where you
cannot obtain wayleave. This is the most common reason and it is a legal and
commercial constraint rather than a technical one.

**When the endpoint moves.** A vehicle, a handheld scanner, a person. No cable
serves a moving target.

**When deployment speed matters.** A point-to-point radio link can be operational
in days; a fibre installation across a business park takes months and requires
permits.

**When the cost of the path dominates and the distance is short.** A 300 m
point-to-point link across a car park costs perhaps £3,000 in radios and a day's
work. Trenching the same 300 m costs an order of magnitude more.

**For temporary or backup connectivity.** Disaster recovery, events, construction
sites, and a diverse backup path that does not share a duct with the primary
(Chapter 56 §56.2's shared-fate argument).

## What it costs

**No guarantee of availability.** Unlicensed spectrum's performance depends on your
neighbours, and neighbours change. This is the largest difference from
cable, and it makes SLAs on unlicensed links essentially unofferable.

**Weather.** Rain attenuates significantly above about 10 GHz — **rain fade** — and
a link engineered for clear air fails in a storm. Availability targets like 99.99%
translate into rain-fade margins computed from local rainfall statistics
(ITU-R P.530 is the standard method), and at high frequencies the required margin
can be 20 dB or more.

**Line of sight, and more than line of sight.** Chapter 42 §42.4's Fresnel zone
must be clear, not merely the direct path. Trees grow. Buildings appear.

**Security by default is absent.** Anyone within range receives every frame.
Encryption is structural rather than optional, which Chapter 44 and Chapter 61
develop.

**Regulatory compliance.** Power limits, out-of-band emission limits, and in some
bands a licence. Exceeding EIRP limits is an offence, not a performance choice, and
Chapter 42's link budget reports EIRP separately for exactly this reason.

## The point-to-point case

Worth treating separately because it is the case where free space competes directly
with fibre.

A licensed microwave link at 18 or 23 GHz, with parabolic antennas at both ends,
delivers hundreds of megabits to a few gigabits over 5–30 km with carrier-grade
availability. It is the standard solution for cellular backhaul where fibre is
absent, and for enterprise links across obstacles.

Design considerations, all of which appear in Chapter 42's link budget:

- **Antenna size** trades against gain. A 60 cm dish at 18 GHz gives about 38 dBi;
  a 1.2 m dish gives 44 dBi. Larger is better and needs a stronger mount and
  attracts planning objections.
- **Rain fade margin** from local statistics and the availability target.
- **Path clearance**, including Fresnel zone and terrain profile.
- **Frequency coordination**, which for licensed bands means an application and a
  wait.
- **Alignment**, which for a narrow-beam dish is a real engineering task —
  a 1.2 m dish at 23 GHz has a beamwidth under 1°, and a mount that shifts a
  degree in wind takes the link down.

**Free-space optics** — laser links through air — is the other option: multi-gigabit
capacity, no spectrum licence at all, and defeated by fog. Deployed where a short
hop needs high capacity and the climate cooperates.

## What breaks here

**A link engineered for clear air.** Works for months, fails in the first serious
storm. Compute the rain margin from local statistics, not from optimism.

**Fresnel zone intrusion by growing vegetation.** Degrades over years, slowly
enough that nobody connects it to the trees.

**A neighbour deploying on your unlicensed channel.** Nothing you can do about it
except move, and there may be nowhere to move to.

**Antenna misalignment after wind or building settlement.** Narrow beams are
unforgiving; a link that was fine at commissioning drifts.

**Exceeding EIRP limits by fitting a higher-gain antenna** to a radio already at
maximum output. This is a regulatory offence in most jurisdictions, and the link
budget's EIRP line is where you check.

> **Network+ note.** Objective 1.5 covers wireless media and objective 2.3 covers
> the bands. The transferable point for design work is §10.4's framing: free space
> is chosen when **installing cable is impossible, prohibitive, or too slow, or the
> endpoint moves** — and it is paid for with the loss of any guarantee.
