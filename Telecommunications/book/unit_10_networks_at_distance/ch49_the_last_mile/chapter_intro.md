# Chapter 49 — The Last Mile

The most expensive kilometre of any network is the last one, and it is expensive for
a reason that no technology fixes: it cannot be shared.

Every other segment of a network serves many customers. A submarine cable serves
millions; a metro fibre ring serves thousands; a neighbourhood distribution serves
hundreds. The cost per customer of those segments falls as the customer count rises.

The final connection to one building serves exactly one building. Its cost —
trenching, permits, poles, ducts, the labour of a technician visiting — is borne
entirely by that one customer, and it does not fall with scale.

This single economic fact explains essentially every technology in this chapter.
Every one of them is an attempt to **avoid** building a new last mile by extracting
more capacity from infrastructure that already exists.

## Reusing the telephone copper

Telephone companies had, by 1990, spent a century installing a twisted pair to
virtually every building in the developed world. That plant was designed for 3.4 kHz
of voice (Chapter 12 §12.2) and was, by any sensible reading, useless for data beyond
a few tens of kilobits.

**DSL** is the observation that the cable is not actually limited to 3.4 kHz — the
*telephone equipment* was. The copper itself passes megahertz, poorly and over
diminishing distances, and if you attach different equipment you can use frequencies
the telephone never touched. ADSL puts voice below 4 kHz and data above it, so both
share the line; the splitter at the customer's premises is the entire integration
cost.

The constraint is Chapter 6's attenuation, which rises with both frequency and
distance, and the result is the table every DSL engineer knows:

| Distance from exchange | Approximate ADSL2+ rate |
|---|---|
| < 1 km | 20+ Mb/s |
| 2 km | ~12 Mb/s |
| 3 km | ~6 Mb/s |
| 5 km | ~1.5 Mb/s |
| > 5.5 km | marginal or unusable |

VDSL2 and G.fast push rates far higher — VDSL2 offering 100 Mb/s, G.fast up to
1 Gb/s — over dramatically shorter distances, which is why they are deployed from
street cabinets rather than exchanges. The industry's answer to distance has been to
move the fibre closer and shorten the copper, which is fibre-to-the-cabinet, and it
is a genuinely sensible intermediate step.

## Reusing the television coax

Cable television networks had installed coaxial cable to a large fraction of homes,
designed for 750 MHz or more of downstream television — vastly more bandwidth than
telephone copper. **DOCSIS** uses some of those television channels for data.

The architectural difference from DSL matters and is worth being precise about.
A DSL line is dedicated to one subscriber; a DOCSIS segment is shared among
perhaps 100–500 homes on the same coaxial branch. Chapter 9's statistical multiplexing
applies, with its gains and its consequence: performance depends on your neighbours'
usage, which is the origin of the "slow at 8 p.m." complaint that cable subscribers
recognise and DSL subscribers do not.

DOCSIS 3.1 (2013) introduced OFDM (Chapter 8 §8.4) and reaches 10 Gb/s downstream and
1–2 Gb/s upstream. The asymmetry is not a technical necessity but an inheritance:
cable plant was built for one-way television, and the return path occupies a narrow,
noisy band at the bottom of the spectrum. DOCSIS 4.0 addresses this with a
substantially expanded upstream, and the deployment cost is why it is arriving slowly.

## Building new: PON

When fibre is actually installed, running a dedicated strand from the exchange to
every home would be prohibitive in both fibre count and terminal equipment. **Passive
optical networking** shares one fibre among many subscribers using a purely optical
splitter — no power, no electronics, no maintenance — typically 1:32 or 1:64.

Downstream, the head end broadcasts to everyone and each terminal ignores what is not
addressed to it, which means downstream traffic is encrypted, because every
subscriber physically receives every other subscriber's frames. Upstream, terminals
transmit in assigned time slots, which is Chapter 9's TDM applied to light, with the
extra complication that each subscriber is a different distance from the head end and
must be range-calibrated so that its slot arrives at the right moment.

GPON offers 2.5 Gb/s down and 1.25 Gb/s up, shared; XGS-PON offers 10 Gb/s
symmetric; 25G and 50G PON are standardised. The "shared" is important: a 1:32 GPON
split gives each subscriber a fair share of 2.5 Gb/s, not 2.5 Gb/s each, and the
statistical multiplexing argument again supplies the reason this is usually
acceptable.

## Where there is no cable

**Fixed wireless** — point-to-multipoint radio from a tower, using the link budget
arithmetic of Chapter 42 §42.3. Cheap to deploy where trenching is not viable;
limited by line of sight, weather and spectrum.

**Satellite**, where the orbit determines everything, and the numbers are pure
Chapter 1 §1.1:

| Orbit | Altitude | One-way delay | Round trip |
|---|---|---|---|
| GEO | 35,786 km | 119 ms | ≥ 477 ms |
| MEO | ~8,000 km | 27 ms | ~110 ms |
| LEO | ~550 km | 1.8 ms | ~20–50 ms in practice |

The GEO figure is not an engineering deficiency; it is the speed of light over
71,572 km of round-trip path, and no satellite operator will ever improve it. It is
why GEO internet feels the way it does — Chapter 3 §3.4's bandwidth–delay product
means a single TCP stream with a 64 KB window achieves about 1 Mb/s regardless of the
link's capacity, which is why every GEO provider deploys performance-enhancing
proxies that terminate TCP locally and spoof acknowledgements.

**LEO constellations** solve the latency problem by flying two orders of magnitude
lower, at the cost of needing thousands of satellites (because each covers a small
area and moves quickly), continuous handover between them, and steerable or phased-
array antennas. Starlink and its competitors are the current instance, and §49.4
treats them as an engineering tradeoff — latency and capacity bought with launch
cadence, constellation size, and orbital congestion — rather than as a product
announcement.

## By the end you will be able to

- Explain why the last mile dominates network cost and why that shapes every
  technology here.
- Predict DSL performance from loop length and explain the frequency/attenuation
  relationship behind it.
- Explain the architectural difference between DSL and DOCSIS and predict the
  different complaint patterns each produces.
- Explain PON's splitter, why downstream must be encrypted, and how upstream slots
  are assigned.
- Compute satellite latency from orbital altitude and explain the consequences for
  TCP.
- Recommend an access technology for a stated location, requirement and budget.
