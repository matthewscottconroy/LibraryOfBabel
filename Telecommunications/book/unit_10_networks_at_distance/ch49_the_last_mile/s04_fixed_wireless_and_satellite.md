# 49.4 Fixed Wireless and Satellite

When there is no cable and there is not going to be one, the remaining option is to send
the last mile through the air — and the interesting constraints are geometric rather than
electronic.

## Fixed wireless access

Point-to-multipoint radio from a tower to fixed antennas on buildings. Chapter 42's link
budget arithmetic, applied commercially.

| | |
|---|---|
| Bands | **unlicensed 5/6 GHz; licensed 3.5 GHz (CBRS), 28 GHz; sub-6 5G FWA** |
| Range | **1–10 km** typically; more with clear line of sight |
| Rates | **25–300 Mb/s**, up to gigabit on mmWave over short hops |
| Deployment time | **days**, against months or years for trenching |
| Cost per subscriber | **low, and it does not scale with distance** |

Its economics are the inverse of everything else in this chapter. Wired access costs
per metre of route; fixed wireless costs per tower and per subscriber terminal. So it
wins exactly where the route is long and the subscribers are few — rural areas, and the
places wired operators decline to serve.

Its constraints are the ones Chapter 42 predicts:

**Line of sight, and Fresnel clearance.** A tree in the path is not a minor obstruction
(Chapter 42 §42.1) — foliage absorbs strongly and varies with season and rain, so a link
commissioned in February may fail in May.

**Weather.** Rain fade is negligible below about 6 GHz and severe above 20 GHz, which is
the single largest argument against mmWave for anything needing reliability.

**Shared spectrum.** Unlicensed operation means your neighbour's network is your noise
floor (Chapter 43 §43.4), and it degrades as the area develops. Licensed or CBRS-shared
spectrum costs money and removes the risk.

**Capacity per sector.** A tower sector's capacity is divided among everyone it serves —
Chapter 46 §46.1's cellular argument. The answer is the same: more sectors, narrower beams,
smaller cells.

> 5G fixed wireless access is this technology with a cellular operator's spectrum and
> scale, and it has become a genuine competitor to cable in suburban markets — not because
> the radio improved dramatically, but because mid-band 5G spectrum became available and the
> equipment became a consumer product (Chapter 46 §46.4).

## Satellite: the geometry decides everything

Orbital altitude determines latency, coverage, satellite count and cost, and no engineering
changes any of it.

| Orbit | Altitude | One-way to satellite | **Bent-pipe round trip** |
|---|---|---|---|
| **GEO** | **35,786 km** | **119 ms** | **477 ms** |
| MEO | ~8,000 km | 27 ms | **107 ms** |
| **LEO** | **~550 km** | **1.8 ms** | **~7 ms of propagation** |

The round-trip figure is four one-way hops — user to satellite, satellite to ground
station, and back — because a bent-pipe satellite is a mirror, not a router.

> 477 ms is not a deficiency in GEO equipment. It is the speed of light over 143,000 km,
> and it will be 477 ms in a century. Every other property of GEO service follows from it.

**Why GEO was built anyway:** a satellite at 35,786 km orbits in exactly one sidereal day,
so it appears stationary from the ground. A fixed dish, aimed once, never moves again —
and three satellites cover the entire populated Earth. For broadcast, that is close to
ideal, which is why television used it for forty years.

## What 477 ms does to TCP

This is Chapter 3 §3.4's bandwidth–delay product, and it is worth working through, because
it explains why GEO Internet feels the way it does regardless of the advertised rate.

A single TCP stream's throughput is bounded by window ÷ RTT:

$$\text{throughput} \le \frac{64 \text{ KB} \times 8}{0.477 \text{ s}} \approx 1.1 \text{ Mb/s}$$

A 64 KB window on a 477 ms path achieves 1.1 Mb/s on a 100 Mb/s link. The capacity is
there and TCP cannot fill it.

**Window scaling** (Chapter 37 §37.4) raises the ceiling — a 4 MB window gives 70 Mb/s — but
slow start still takes a great many round trips to reach it, and every loss event costs
477 ms to detect and another to recover.

Which is why every GEO provider deploys a performance-enhancing proxy:

```
   Client ──TCP──▶ [PEP] ══satellite══ [PEP] ──TCP──▶ Server
                     │                   │
              acknowledges locally,  re-establishes a
              hiding the RTT         normal TCP session
```

The PEP terminates TCP at each end and spoofs acknowledgements, so the client's stack sees
a short RTT and ramps up immediately. It is a deliberate, load-bearing violation of the
end-to-end principle (Chapter 23 §23.4), and it is completely defeated by encryption —
a PEP cannot inspect or split a QUIC connection or a VPN tunnel.

> So a VPN over GEO satellite performs dramatically worse than plain traffic over the same
> link, and the reason is architectural rather than a fault. The same is increasingly true
> of QUIC, which is one of the underappreciated consequences of encrypting the transport
> header (Chapter 38 §38.4).

## LEO constellations

Fly two orders of magnitude lower and the latency problem disappears. Everything else gets
harder.

**The geometry, worked through:**

A satellite at 550 km covers a circle of roughly 570 km radius at a usable 40° minimum
elevation — about 1 million km² of the Earth's 510 million. So continuous global
coverage needs several hundred satellites at minimum, and capacity requirements push the
number far higher.

**And they move.** At 550 km a satellite orbits in **about 96 minutes** and is overhead for
only a few minutes, so:

- **Handover is continuous** — every few minutes, for every user, forever
- **Antennas must track**, which means motorised dishes or electronically steered phased
  arrays (Chapter 44 §44.4's beamforming, made consumer-grade)
- Ground stations must be numerous, or the satellites must relay between themselves

| | **Starlink** | **OneWeb** | **Kuiper** |
|---|---|---|---|
| Altitude | **~550 km** | ~1,200 km | ~600 km |
| Satellites | **thousands, and growing** | ~650 | planned thousands |
| Market | **consumer + enterprise** | **enterprise and backhaul only** | consumer |
| **Inter-satellite links** | **laser, deployed** | no | planned |
| Typical latency | **25–60 ms** | 70–100 ms | — |

Note the gap between 7 ms of propagation and 25–60 ms observed. Propagation is the small
part. The rest is the ground station's backhaul to a real Internet exchange, scheduling
delay in the radio, and queueing — the same lesson as Chapter 46 §46.4's 5G latency claim:
the radio is not the whole path.

Inter-satellite laser links are the genuinely interesting development. With them, a
constellation is a routed network in orbit and traffic need not descend to a ground station
near the user.

And this produces a result that surprises people: light travels 47% faster in vacuum than
in glass (Chapter 6's refractive index of about 1.47). Over a long enough path — London to
Singapore, say — a route through orbit can beat a submarine cable on latency, despite the
extra distance climbing to 550 km and back. Financial trading firms have taken this
seriously enough to fund it.

## The problems that are not engineering

Worth stating plainly, because they will decide the technology's future more than the radio
will.

**Orbital congestion.** Tens of thousands of satellites in low orbit, with collision
avoidance manoeuvres now a routine operational task. **The Kessler syndrome** — a collision
cascade making an orbital shell unusable — is not a fringe concern; it is discussed in
operators' own filings.

**Astronomy.** LEO satellites are bright, and they streak long-exposure images. Operators
have applied darkening coatings and sunshades in response to astronomers' measurements, with
partial success, and the problem scales with the constellation.

**Lifetime and launch cadence.** A 550 km satellite deorbits in about five years — which is
good for debris and means the entire constellation must be replaced continuously. The
business model requires cheap launch to exist permanently.

**Capacity per cell.** A satellite's beam covers a large area, and everyone under it shares
it. LEO broadband is excellent where users are sparse and degrades sharply where they are
dense — which is precisely the opposite of the concentration pattern that makes wired
networks economic, and it is why LEO complements rather than replaces fibre.

> LEO is the best available answer for the genuinely remote and a poor answer for the
> suburb. Any claim that it will replace terrestrial broadband misunderstands the capacity
> arithmetic.

## Choosing

| Situation | Answer |
|---|---|
| Dense urban, capital available | **fibre (PON)** |
| Existing cable plant | **DOCSIS 3.1/4.0** |
| Existing copper, short loops | **VDSL2 / G.fast from a cabinet** |
| **Suburban, no wired option** | **5G FWA** |
| **Rural with line of sight to a tower** | **fixed wireless** |
| **Remote, no infrastructure** | **LEO satellite** |
| **Ships, aircraft, disaster response** | **LEO or MEO** |
| Broadcast to a continent | **GEO — still unbeaten** |
| Latency-critical anywhere | **not satellite, unless LEO with local ground stations** |

## What breaks here

A fixed wireless link that degrades every spring. **Foliage.** Re-survey with leaves on;
Chapter 42 §42.1.

Fixed wireless failing in heavy rain at 28 GHz. **Rain fade.** Inherent above ~20 GHz.
Lower band or shorter hop.

**Satellite service slow only over VPN.** The PEP cannot help encrypted traffic.
Architectural, not a fault — and worth explaining to users rather than troubleshooting.

GEO service that pings 600 ms rather than 500. Queueing on top of propagation. The
propagation floor is 477 ms; anything above it is buffering (Chapter 66).

LEO service dropping for a few seconds periodically. Handover, or an obstruction the
dish passes through as it tracks. Starlink's own app maps obstructions for exactly this
reason.

LEO throughput collapsing in the evening in a popular area. Cell capacity shared among
subscribers under the same beam. No local fix.

A dish that worked and now does not, with no weather. **Alignment** — wind, mounting
movement, or settling. GEO dishes are aimed to a fraction of a degree.

> **Network+ note.** Objective 1.5 and 2.4. Over-learn: **satellite has high latency,
> especially GEO**; **fixed wireless requires line of sight**; **rain fade affects high
> frequencies**; and **LEO constellations reduce latency by flying much lower.** The
> latency-versus-altitude relationship is the examinable idea, and being able to compute it
> from the altitude is better than remembering it.
