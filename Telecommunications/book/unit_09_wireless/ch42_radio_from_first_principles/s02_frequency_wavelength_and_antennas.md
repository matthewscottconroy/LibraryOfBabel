# 42.2 Frequency, Wavelength and Antennas

An antenna converts between a current in a conductor and a wave in space. It is a
passive device — it contains nothing that amplifies — and yet antenna choice changes a link
budget more than almost anything else available to you.

Understanding how a passive object produces "gain" is the key to the whole section.

## Why size follows wavelength

An antenna is efficient when its dimensions are a significant fraction of the
wavelength.

**The reason is resonance.** A conductor of the right length supports a standing wave of
current at the operating frequency — the current distribution and the radiated field
reinforce — and one of the wrong length does not, so most of the energy is reflected back
into the transmitter rather than radiated.

**The standard sizes:**

| Antenna | Length | At 2.4 GHz (λ = 12.5 cm) | At 900 MHz (λ = 33 cm) |
|---|---|---|---|
| **Half-wave dipole** | **λ/2** | **6.25 cm** | 16.7 cm |
| **Quarter-wave** (over a ground plane) | **λ/4** | **3.1 cm** | 8.3 cm |
| Full wave | λ | 12.5 cm | 33 cm |

This is why a 2.4 GHz antenna fits inside a phone and a 900 MHz one is a visible stub —
and why the antennas on a Wi-Fi access point are a few centimetres of wire in a plastic
housing.

**It also explains a practical fault:** an antenna cut for one band works poorly on another.
A 2.4 GHz-only antenna on a dual-band access point radiates badly at 5 GHz, and
mismatched or swapped antennas are a real cause of unexpectedly poor coverage.

## Gain — what a passive device can and cannot do

An antenna has "gain", measured in dBi. It does not amplify.

> **Antenna gain is *directionality*.** The antenna radiates the same total power and
> **concentrates it into a smaller solid angle** — so it is stronger in the favoured
> direction and weaker elsewhere.

**The analogy that makes it concrete:** a bare light bulb and the same bulb in a
spotlight reflector. The same wattage; far brighter in one direction and dark
everywhere else.

**The reference:**

| Unit | Relative to |
|---|---|
| **dBi** | an **isotropic** radiator — a theoretical point radiating equally in every direction |
| dBd | a **half-wave dipole**, which itself has 2.15 dBi |

$$\text{dBi} = \text{dBd} + 2.15$$

**Most datasheets use dBi.** A specification quoting dBd is understating by 2.15 dB
relative to the more common convention, and mixing them is a small, common error.

## The antenna types

### Omnidirectional

Radiates equally in all *horizontal* directions, and the pattern is a doughnut with the
antenna through the hole.

```
   Side view:            Top view:

      ▁▁▁▁▁                  ▁▁▁▁▁
    ▕       ▏             ▕         ▏
    ▕   ●   ▏  ← weak     ▕    ●    ▏  ← equal
    ▕       ▏    above    ▕         ▏    in all
      ▔▔▔▔▔      and        ▔▔▔▔▔      directions
                 below
```

| Gain | Pattern | Use |
|---|---|---|
| **2–3 dBi** | nearly spherical | **general indoor coverage** |
| **6 dBi** | flattened doughnut | large open areas, warehouses |
| 9+ dBi | very flat | long corridors, outdoor open sites |

The crucial and counter-intuitive point about high-gain omnis:

> **Higher gain flattens the pattern.** A 9 dBi omni reaches much further horizontally and
> **has a null directly above and below it.**

Which is why a high-gain omni mounted on a ceiling in a two-storey building may cover the
far end of the floor and not the room directly beneath it — and why access points in
multi-storey buildings usually want *lower* gain, not higher.

This surprises people, and it is the most useful antenna fact for indoor design.

### Directional

**Concentrates energy into a beam.**

| Type | Gain | Beamwidth | Use |
|---|---|---|---|
| **Patch / panel** | 6–14 dBi | 30–90° | wall-mounted coverage of one area |
| **Yagi** | 10–20 dBi | 20–60° | point-to-point over moderate distance |
| **Parabolic dish** | **20–30 dBi** | **3–10°** | **long-range point-to-point** |
| Sector | 12–17 dBi | 60–120° horizontal | cellular and WISP base stations |

Gain and beamwidth are inversely related — that is the whole mechanism. A 24 dBi dish
has a beam a few degrees wide, which is enormous gain and requires accurate aiming: a
few degrees of error puts the far end outside the beam entirely.

And it is why a long point-to-point link fails after a storm. The dish moved slightly on
its mount, and at 3° beamwidth "slightly" is enough.

## Polarisation

The orientation of the electric field — vertical, horizontal, or circular.

> **Two antennas must share polarisation.** A cross-polarised pair loses **20–30 dB**,
> which is a factor of a hundred to a thousand in power and will destroy a marginal link.

Most Wi-Fi is vertically polarised, and access-point antennas are meant to be vertical.

**The practical consequences:**

An access point mounted on its side, with the antennas horizontal, is cross-polarised
against every client holding a phone upright — and the loss is severe.

And a phone's orientation changes as the user moves it, which is one reason
**MIMO's multiple antennas** (Chapter 44 §44.4) often use **mixed polarisations** — so that
whatever the client's orientation, one of them matches reasonably.

**Circular polarisation** rotates, so it couples to any linear orientation with a fixed
3 dB penalty. Used where orientation is unpredictable — satellite links, RFID, some
industrial applications — trading a constant small loss for immunity to a large variable
one.

## EIRP — what actually leaves the antenna

**The regulated quantity** (Chapter 43 §43.1), and the one that matters:

$$\text{EIRP} = P_{\text{tx}} - L_{\text{cable}} + G_{\text{antenna}}$$

Effective Isotropic Radiated Power — the power an isotropic radiator would need to
produce the same field strength in the favoured direction.

**Worked:**

```
   Transmitter output       20 dBm  (100 mW)
   Cable and connector loss −2 dB
   Antenna gain            +12 dBi
   ─────────────────────────────────
   EIRP                     30 dBm  (1 W)
```

The transmitter emits 100 mW and the link behaves, in the beam, as though 1 W were
radiated in every direction. Nothing amplified anything; the energy was concentrated.

And regulators limit EIRP, not transmitter power — which is exactly right, because EIRP
is what interferes with everyone else. Fitting a higher-gain antenna without reducing
transmit power can put a legal installation over the limit.

## Cable loss — the part that is forgotten

Coaxial cable loss rises sharply with frequency, and at Wi-Fi frequencies it is
substantial:

| Cable | Loss per 10 m at 2.4 GHz |
|---|---|
| RG-58 (thin) | **~10 dB** |
| RG-213 | ~5 dB |
| LMR-400 | **~2.2 dB** |
| LMR-600 | ~1.4 dB |

Ten metres of RG-58 loses 10 dB — 90% of the power — and the same cable is worse at
5 GHz still.

> **Keep the antenna close to the radio.** Every metre of coaxial cable is loss you cannot
> recover, and it costs you twice — once on transmit and once on receive.

Which is why outdoor access points put the radio in the antenna housing, and why
running a long cable to a roof-mounted antenna is usually a mistake compared with putting a
small unit up there and running Ethernet.

**Connectors matter too** — 0.3 to 0.5 dB each, and a run with six connectors has lost
2–3 dB before the cable is counted.

## Reciprocity

An antenna's transmit and receive properties are identical. Gain, pattern and
polarisation apply equally in both directions.

**Which has a practical consequence worth stating:**

> **A high-gain antenna helps both directions**, and it helps the receive direction in a way
> transmit power cannot.

A client cannot transmit harder than its regulatory limit, so raising an access
point's transmit power to reach further creates an asymmetric link — the client hears the
AP and the AP cannot hear the client.

This is one of the commonest wireless design errors (Chapter 45 §45.1): turning power up
to fix a coverage complaint makes the *downlink* stronger and the *uplink* no better, so the
client associates from a distance and then cannot communicate.

A better antenna improves both. More power improves one.

## What breaks here

A high-gain ceiling antenna with a dead spot directly beneath it. The pattern's null.
Use lower gain.

**A point-to-point link that degraded after weather.** The dish moved; a narrow beam is
unforgiving.

Severe loss on a link that should work. Cross-polarisation, or a mismatched antenna.

Clients seeing the AP strongly and being unable to connect. Asymmetry from excessive AP
power.

An installation over the legal limit after an antenna upgrade. EIRP includes antenna
gain.

Poor performance on a long antenna cable run. Cable loss. Move the radio.

> **Network+ note.** Objective 2.4 expects antenna types. Over-learn: omnidirectional
> radiates in all horizontal directions; directional (patch, Yagi, parabolic) concentrates
> into a beam; **gain is directionality, not amplification, measured in dBi**;
> **polarisation must match**; and EIRP = transmit power − losses + antenna gain, and is
> what regulators limit.
