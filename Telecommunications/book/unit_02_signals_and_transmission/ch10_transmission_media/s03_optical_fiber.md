# 10.3 Optical Fibre

Glass, drawn to a hair's thickness, carrying light by a mechanism that a
seventeenth-century physicist would recognise, at a purity that took a century of
materials science to achieve.

## Total internal reflection

Light travelling from a denser medium into a less dense one bends away from the
normal. Beyond a **critical angle**, it does not cross the boundary at all — it
reflects entirely.

Snell's law gives the condition. For refractive indices *n*₁ (core) and *n*₂
(cladding), with *n*₁ > *n*₂:

$$\theta_c = \arcsin\left(\frac{n_2}{n_1}\right)$$

A typical fibre has *n*₁ = 1.4682 and *n*₂ = 1.4629, giving a critical angle of
about 85.1°. Light entering within a narrow cone of the axis strikes the boundary
at more than that and is trapped.

```
   cladding  n₂ = 1.4629
  ═══════════════════════════════════
   core  n₁ = 1.4682   ╲    ╱╲    ╱╲
                        ╲  ╱  ╲  ╱
  ═══════════════════════╲╱════╲╱═══
   cladding
```

**The cladding is not optional and it is not a protective coating.** Without it the
core's boundary would be with air, contaminants, adhesive and whatever the fibre
touches — an ill-defined and constantly changing interface. The cladding provides a
controlled, uniform, permanent boundary of known index, and it is why a fibre can be
handled, spliced and buried without its optical properties changing.

The **numerical aperture** describes the acceptance cone:

$$\text{NA} = \sqrt{n_1^2 - n_2^2}$$

which for the figures above is about 0.125 — a half-angle of roughly 7°. Light
outside that cone is not guided.

## Single-mode and multimode

The distinction that determines nearly every fibre decision.

**Multimode** has a wide core — 50 or 62.5 µm — which supports many propagation
paths, called modes. Light entering at different angles takes geometrically
different routes and therefore arrives at different times: **modal dispersion**
(Chapter 6 §6.3), which limits the distance-bandwidth product.

**Single-mode** has a core of about **9 µm**, narrow enough that only one mode
propagates. Modal dispersion is eliminated entirely. Chromatic dispersion and
polarisation-mode dispersion remain, and both are far smaller.

| | Multimode | Single-mode |
|---|---|---|
| Core | 50 or 62.5 µm | ~9 µm |
| Modes | many | one |
| Modal dispersion | **yes** — the limit | none |
| Reach at 10 Gb/s | 33–400 m | 10–80 km+ |
| Light source | LED or **VCSEL** | Fabry-Perot or DFB laser |
| Source cost | **low** | higher |
| Alignment tolerance | **forgiving** | tight |
| Typical wavelength | 850, 1300 nm | 1310, 1550 nm |

The counterintuitive part: **single-mode fibre is not more expensive than
multimode**, and is frequently cheaper. The cost difference is in the
**transceivers** — a VCSEL coupling into a 50 µm core is a cheap, high-volume
device; a DFB laser coupling into a 9 µm core is not.

So the decision is:

- **Short reach, many links, cost-sensitive** → multimode, because the transceiver
  saving multiplies by the link count. Inside a data centre, inside a building.
- **Long reach, or few links, or uncertain future** → single-mode. Between
  buildings, in risers, anywhere you might later want a higher rate or a longer
  distance.

And an argument worth making explicitly: **when in doubt, install single-mode.**
The fibre costs about the same, the labour is identical, and a single-mode plant
supports every rate and distance you might later want. A multimode plant installed
in 2015 for 1 Gb/s over 300 m may not support 40 Gb/s over the same run.

## OM and OS grades

Multimode grades, distinguished by bandwidth-distance product and by whether they
are laser-optimised:

| Grade | Core | Colour | 1 Gb/s | 10 Gb/s | 40/100 Gb/s |
|---|---|---|---|---|---|
| OM1 | 62.5 µm | orange | 275 m | 33 m | — |
| OM2 | 50 µm | orange | 550 m | 82 m | — |
| OM3 | 50 µm | **aqua** | 800 m | 300 m | 100 m |
| OM4 | 50 µm | aqua/violet | 1,100 m | 400 m | 150 m |
| OM5 | 50 µm | lime green | 1,100 m | 400 m | 150 m + SWDM |

Single-mode grades:

| Grade | Notes |
|---|---|
| OS1 | Indoor, tight-buffered, ≤1 dB/km |
| OS2 | Outdoor, loose tube, ≤0.4 dB/km — the long-haul standard |

The tenfold improvement from OM1 to OM4 at 10 Gb/s comes entirely from controlling
the refractive index profile so that different modes travel at more nearly equal
speeds — a graded index rather than a step. OM5 adds specified performance across a
range of wavelengths, permitting short-wavelength WDM on multimode.

**Jacket colour is a convention, not a standard**, and it is unreliable. Aqua
usually means OM3 or OM4; yellow usually means single-mode. Read the printing on the
jacket, which is authoritative.

## Connectors

| Connector | Form | Where |
|---|---|---|
| **LC** | small, latching duplex | The modern default; SFP transceivers |
| SC | square, push-pull | Older equipment, patch panels, PON |
| ST | bayonet, round | Legacy multimode |
| MPO/MTP | 12 or 24 fibres in one ferrule | 40G/100G parallel optics, trunk cabling |
| FC | threaded | Instrumentation, high-vibration |

**Polish types** matter and are frequently overlooked:

- **PC** — physical contact, flat. Return loss around −40 dB.
- **UPC** — ultra physical contact, better polished. Around −50 dB. Blue housing.
- **APC** — angled physical contact, 8° angle. Around −60 dB. **Green housing.**

The angle in APC reflects light out of the core rather than back down it, which
matters for PON and analogue video where reflections degrade the signal
significantly.

**Never mate APC to UPC.** The angle mismatch produces a large air gap, high
insertion loss, and can physically damage the ferrules. Green means green.

## The loss budget

The calculation that determines whether a link works, and the one you will do in
the project.

```
  Transmitter launch power (min)      −3.0 dBm
  Receiver sensitivity                −23.0 dBm
  ──────────────────────────────────────────────
  Available budget                     20.0 dB

  Fibre:      42 km × 0.25 dB/km      −10.5 dB
  Splices:    8 × 0.10 dB              −0.8 dB
  Connectors: 4 × 0.50 dB              −2.0 dB
  Ageing and repair allowance          −3.0 dB
  ──────────────────────────────────────────────
  Total loss                           16.3 dB
  Margin                                3.7 dB
```

Typical component figures:

| Element | Loss |
|---|---|
| SMF at 1310 nm | 0.35 dB/km |
| SMF at 1550 nm | **0.17–0.25 dB/km** |
| MMF at 850 nm | 3.0 dB/km |
| Fusion splice | 0.05–0.1 dB |
| Mechanical splice | 0.3 dB |
| Connector pair | 0.3–0.75 dB |
| Repair allowance | 0.1 dB/km or a fixed 2–3 dB |

**Margin under 3 dB is marginal**; it will work on the bench and fail after a
repair splice or a warm summer. Over 6 dB is comfortable.

**And too much power is also a fault.** Every receiver has a maximum as well as a
minimum. Connect a long-reach transceiver across a 2 m patch lead and the receiver
saturates, producing errors that look exactly like insufficient power. Optical
attenuators exist for this and it surprises people every time.

## Practical realities

**Dirty connectors are the most common fibre fault, by a wide margin.** A
fingerprint on a ferrule can cost several dB — a substantial fraction of a typical
budget. The remedy is a cleaning kit costing very little; the diagnosis is an
inspection scope or an optical power meter. Chapter 64 §64.4 covers both.

**Bend radius** is specified and violating it costs light. Modern bend-insensitive
fibre (G.657) tolerates far tighter bends than older types, which has made
in-building fibre installation dramatically more practical.

**Never look into a fibre.** 1310 and 1550 nm are invisible to the eye, so there is
no blink reflex, and the power in a long-haul system is sufficient to cause retinal
damage. Use a power meter or a viewer, always, and treat every fibre as live.

**Fusion splicing** aligns and melts two fibres together, giving 0.05–0.1 dB. The
splicer costs several thousand pounds and the result is permanent. **Mechanical
splices** align mechanically with index-matching gel, giving about 0.3 dB, and are
for temporary repairs.

## What breaks here

**Dirty connector.** Several dB, invisible, most common fault.

**APC mated to UPC.** High loss and possible physical damage. Check for green.

**Receiver saturation** on a short link with a long-reach transceiver. Insert an
attenuator.

**Transmit and receive reversed.** Link does not come up; swap the duplex pair.
Trivial and extremely common.

**Multimode transceiver on single-mode fibre**, or vice versa. Usually produces no
link at all; occasionally produces a marginal one that fails intermittently.

**A run exceeding the OM grade's reach at the rate in use.** Works at 1 Gb/s, fails
at 10. Read the grade printed on the jacket against the table above.

> **Network+ note.** Objective 1.5 expects single-mode versus multimode and their
> applications, the connector types, and the fibre transceiver types (SFP, SFP+,
> QSFP). Objective 5.2 expects fibre fault diagnosis. The two things to over-learn:
> **single-mode's reach advantage comes from eliminating modal dispersion**, and
> **the cost difference is in the transceivers, not the glass** — which is why "when
> in doubt, single-mode" is defensible advice.
