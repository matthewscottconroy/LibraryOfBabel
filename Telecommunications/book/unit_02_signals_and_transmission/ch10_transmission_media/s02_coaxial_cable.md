# 10.2 Coaxial Cable

Coax lost the LAN and is still carrying broadband to several hundred million
households, every antenna feed in existence, and a substantial fraction of the
short-reach connections inside data centres. Reports of its death were premature
and are still being repeated.

## Construction, and why the geometry matters

A central conductor, a dielectric, a cylindrical shield, and a jacket:

```
        ┌─────────────────────────┐  jacket
        │ ┌─────────────────────┐ │  shield (braid and/or foil)
        │ │ ┌─────────────────┐ │ │  dielectric
        │ │ │ ●               │ │ │  centre conductor
        │ │ └─────────────────┘ │ │
        │ └─────────────────────┘ │
        └─────────────────────────┘
```

The important property follows from that geometry: **the signal's electromagnetic
field is entirely contained between the centre conductor and the shield.** It does
not extend outside the cable.

This is a categorically different mechanism from twisted pair's. Twisted pair
rejects interference by *symmetry* — the interference couples into both conductors
and cancels in the differential subtraction (Chapter 6 §6.4). Coax rejects it by
*confinement* — the field is inside a conductive tube and external fields do not
reach it.

The consequences:

- **Excellent shielding** without needing balance. Coax works unbalanced,
  referenced to ground, which simplifies a great deal.
- **Very wide bandwidth.** Modern coaxial plant carries 1 GHz and beyond, against
  twisted pair's 500 MHz for Cat6a.
- **Physically inflexible and bulky.** A rigid dielectric and a braid do not bend
  tightly, and bend radius violations damage the geometry permanently.
- **Awkward to terminate.** Connectors require stripping to precise dimensions and
  either crimping or compression, and a poorly made connection reflects.

## Impedance, and why 50 and 75 both exist

A coaxial cable has a **characteristic impedance** set by its geometry and
dielectric — the ratio of voltage to current for a wave travelling along it.

Two values dominate, and the reason is a genuine engineering optimum in each case.

**50 Ω** is close to the geometry that **minimises loss for a given power handling
capability**. Strictly, minimum attenuation for an air dielectric occurs at about
77 Ω and maximum power handling at about 30 Ω; 50 Ω is a compromise, and it is what
you use where you are transmitting significant power. Hence: radio transmitters,
antenna feeds, test equipment, Wi-Fi access points, and — historically —
10BASE5 and 10BASE2 Ethernet.

**75 Ω** is close to the **minimum-attenuation** geometry for the practical
dielectrics used in cable television, and it is what you use where power is low and
distance is long. Hence: broadcast and cable television distribution, DOCSIS, and
digital video interconnects (SDI).

**Mismatched impedance causes reflections.** Connect a 50 Ω source to a 75 Ω cable
and part of the signal bounces back at the discontinuity. The reflection coefficient
is

$$\Gamma = \frac{Z_2 - Z_1}{Z_2 + Z_1} = \frac{75 - 50}{75 + 50} = 0.2$$

so 20% of the voltage — 4% of the power — is reflected. On a short cable this is
tolerable; on a long one, or at high frequency, the reflections produce standing
waves and the return loss specification is violated.

**Termination matters for the same reason.** An unterminated coax end reflects
everything. This is why the old 10BASE2 bus required a 50 Ω terminator at each end,
and why removing one — or having a single loose BNC connector — took down the entire
segment rather than one station. Anyone who administered a coaxial Ethernet remembers
this, and it is a substantial part of why 10BASE-T's star topology (Chapter 11
§11.3) was such a relief.

## The types you will meet

| Type | Impedance | Where |
|---|---|---|
| RG-6 | 75 Ω | Cable television and DOCSIS drops; the cable in most homes |
| RG-11 | 75 Ω | Longer CATV runs; lower loss, thicker |
| RG-59 | 75 Ω | Older CCTV and video; largely superseded by RG-6 |
| RG-58 | 50 Ω | 10BASE2 "thinnet"; test leads; amateur radio |
| RG-8 / LMR-400 | 50 Ω | Antenna feeds where loss matters |
| Hardline | 75 Ω | CATV trunk distribution; rigid, very low loss |
| Twinaxial | 100 Ω differential | **Direct-attach copper** in data centres |

**Connectors:** **F-type** (screw-on, 75 Ω, cable television and DOCSIS), **BNC**
(bayonet, 50 or 75 Ω, test equipment and video), **N-type** (threaded, 50 Ω,
weatherproof, antenna feeds), **SMA** (small, 50 Ω, Wi-Fi and instrumentation).

## Where coax is now

**Cable television and DOCSIS.** Several hundred million households, and
Chapter 49 §49.2 covers the architecture. Modern plant is **hybrid fibre-coax**:
fibre from the head end to a neighbourhood node, coax from the node to the homes.
The coax segment is the shared one, which is the architectural fact producing the
"slow at 8 p.m." complaint that DSL subscribers do not have.

DOCSIS 3.1 reaches 10 Gb/s downstream over that plant using OFDM (Chapter 8 §8.4),
which is a remarkable thing to extract from cable installed for analogue television.

**Antenna feeds.** Every access point with an external antenna, every base station,
every broadcast transmitter. Loss in the feeder is a direct entry in Chapter 42's
link budget, and a long run of cheap coax at 5 GHz can cost more than the antenna
gains.

**Direct-attach copper in data centres.** Twinaxial assemblies with transceivers
moulded on, for 10, 25, 40 and 100 Gb/s over 1–7 m. Cheaper than optics, lower
power, no transceiver to fail — and length-limited, which is exactly why they are
used top-of-rack and nowhere else.

**Instrumentation and test.** Everything with a BNC or SMA connector.

## Why coax lost the LAN

Worth understanding, because the reasons are about topology and operations rather
than about the medium's electrical properties — which were, and are, good.

**The bus topology was operationally fragile.** One cable, all stations tapped onto
it, terminated at both ends. A break anywhere, a loose connector anywhere, or a
missing terminator took down **every** station — and finding which of forty taps was
faulty meant walking the cable.

**Moves and changes were disruptive.** Adding a station meant tapping the bus or
breaking it, which interrupted everyone.

**10BASE-T's star** put one cable per station into a central point. A fault affects
one station. Adding a station affects nobody. And it ran on the twisted pair already
installed in buildings for telephones, which meant no new cabling at all.

The lesson generalises and is worth extracting: **the medium's electrical properties
were not what decided it.** Operational characteristics — fault isolation, ease of
change, reuse of existing infrastructure — decided it, and that is the pattern
Chapter 16 §16.3 identifies in Ethernet's broader history.

## What breaks here

**Impedance mismatch**, from mixing 50 Ω and 75 Ω components or from a poor
connector. Produces reflections and return loss failures. A time-domain
reflectometer locates the discontinuity by distance.

**Water ingress.** Coax outdoors with an imperfectly sealed connector wicks water
into the dielectric, which raises loss dramatically and permanently. The cable does
not recover when it dries. This is the most common outdoor coax fault and the
reason weatherproofing tape exists.

**Bend radius violation.** Deforms the geometry, changes the local impedance,
reflects. Unlike twisted pair, coax does not forgive a tight bend.

**Missing or failed terminator** on a bus segment. Everything stops. Historical,
and still met on legacy industrial and instrumentation buses.

**A cheap feeder at high frequency.** At 5 GHz, RG-58 loses roughly 1 dB per metre.
A 15 m run costs 15 dB, which is more than most antennas gain. Specify low-loss
cable for high-frequency feeders and keep them short — Chapter 6 §6.2's Friis
argument says put the amplifier at the antenna, not at the far end of the feeder.

> **Network+ note.** Objective 1.5 expects coaxial cable, RG-6, F-type and BNC
> connectors, and the 50/75 Ω distinction. Objective 2.4 covers direct-attach
> copper. The point worth carrying beyond the exam is §10.2's closing lesson: coax
> lost the LAN on **operational** grounds, not electrical ones, and that is how
> most media decisions are actually made.
