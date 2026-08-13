# 6.1 Attenuation

The signal gets smaller. This is the simplest of the four impairments and the one
with the most immediate practical consequences, because it is what sets every
maximum distance figure you will ever look up.

## The measure

Attenuation is a power ratio, so it is expressed in decibels (Chapter 4 §4.3):

$$\text{loss (dB)} = 10\log_{10}\frac{P_{\text{in}}}{P_{\text{out}}}$$

and because decibels add, a link's total loss is simply the sum of its parts:

```
  transmitter  →  connector  →  100 m of cable  →  patch panel  →  connector  →  receiver
                    0.3 dB        22 dB               0.5 dB          0.3 dB
                                     total: 23.1 dB
```

The received power is the transmitted power minus that total, and the link works
if what remains exceeds the receiver's sensitivity with margin to spare. That
single sum is the **loss budget**, and it is the calculation behind every media
decision in Chapter 10.

## Why copper attenuates more at high frequencies

This is the fact that matters, and it has two distinct causes that both worsen with
frequency.

**The skin effect.** At DC, current flows through the whole cross-section of a
conductor. At higher frequencies, the changing magnetic field inside the conductor
induces eddy currents that oppose flow in the centre, pushing the current toward
the surface. The effective conducting area shrinks, so the effective resistance
rises.

The **skin depth** — where current density has fallen to 1/*e* of its surface value
— is

$$\delta = \sqrt{\frac{\rho}{\pi f \mu}}$$

For copper this gives roughly 8.5 mm at 60 Hz, 66 µm at 1 MHz, 2.1 µm at 1 GHz.
At a gigahertz, essentially all the current is flowing in a layer two microns
thick. Resistance rises as √*f*, and so does this component of the loss.

**Dielectric loss.** The insulation between the conductors is not a perfect
insulator; the alternating field flexes its molecules and some energy is lost as
heat. This component rises roughly linearly with frequency and eventually dominates.

Together they produce the characteristic copper attenuation curve, and the numbers
are worth having:

| Frequency | Cat5e loss per 100 m |
|---|---|
| 1 MHz | ~2.0 dB |
| 10 MHz | ~6.5 dB |
| 31.25 MHz | ~11.7 dB |
| 62.5 MHz | ~17.0 dB |
| 100 MHz | ~22.0 dB |

Note the shape: not linear in frequency but not far off, and the loss at 100 MHz is
eleven times that at 1 MHz. Cat6a extends the specification to 500 MHz, and it does
so with tighter twists, better dielectric and often shielding — all attacking the
same two mechanisms.

## The consequence: distance limits

Every "100 metres" you have ever read is the output of a loss budget.

The 100 m limit for twisted-pair Ethernet is not a physical boundary at which
signals stop. It is the distance at which the loss at the highest frequency the
standard uses brings the signal close enough to the receiver's sensitivity that
the specified error rate can no longer be guaranteed with margin. Actual cables
frequently work somewhat beyond it, and the standard's figure exists so that
equipment from different manufacturers interoperates without anyone having to
measure.

The reason it is 100 m and not 130 m is a committee decision about how much margin
to leave, made once and inherited by every subsequent standard — which is why
10BASE-T, 100BASE-TX, 1000BASE-T, 2.5GBASE-T and 10GBASE-T all say 100 m despite
spanning three orders of magnitude in rate. Each new standard was designed to fit
the existing distance rather than to extend it, because the installed cable plant
is the fixed thing.

## Fibre, and the shape of its loss

Optical fibre's attenuation is far lower and depends on wavelength rather than
frequency in any way you would recognise from copper. The mechanisms are different:

**Rayleigh scattering** — light scattering off microscopic density fluctuations
frozen into the glass when it solidified. It falls as 1/λ⁴, so it dominates at
short wavelengths and is the reason fibre systems moved to longer wavelengths as
the technology allowed.

**Infrared absorption** by the silica lattice itself, rising sharply beyond about
1,600 nm, which sets the long-wavelength boundary.

**Hydroxyl (OH⁻) absorption** — the water peak — producing a loss spike around
1,383 nm from residual moisture in the glass. Modern "low water peak" fibre
(ITU-T G.652.D) largely eliminates it, opening the band between the traditional
windows.

The result is a loss curve with minima in two places, and the industry built
around them:

| Window | Wavelength | Typical loss | Used for |
|---|---|---|---|
| O-band | 1310 nm | ~0.35 dB/km | Short and medium reach, no dispersion |
| C-band | 1550 nm | **~0.17 dB/km** | Long haul and DWDM |
| L-band | 1565–1625 nm | ~0.2 dB/km | DWDM expansion |
| (850 nm) | multimode only | ~3 dB/km | Short reach, cheap VCSELs |

**0.17 dB/km.** After 80 km, the loss is 13.6 dB, so about 4% of the light
survives — which a good receiver can read, and which is exactly why the amplifier
huts along a long-haul route are spaced where they are (Chapter 50 §50.3).

Compare copper's 22 dB per **100 metres** at 100 MHz. Fibre is better by something
like four orders of magnitude per unit distance, and that ratio is the whole reason
the long-haul network is made of glass.

## Working a budget

The standard calculation, which you will do in Chapter 10's exercises and in the
project:

```
  Transmitter launch power       −3.0 dBm
  Receiver sensitivity          −23.0 dBm
  ─────────────────────────────────────────
  Available budget               20.0 dB

  Fibre:  40 km × 0.25 dB/km    −10.0 dB
  Splices: 6 × 0.1 dB            −0.6 dB
  Connectors: 4 × 0.5 dB         −2.0 dB
  ─────────────────────────────────────────
  Total loss                     12.6 dB
  Margin                          7.4 dB
```

Seven decibels of margin is comfortable — it accommodates ageing, a future splice
after a cable cut, and a connector that is not quite clean. Under 3 dB is
marginal and will produce a link that works in the lab and fails in service.

The same arithmetic, with different terms, is the wireless link budget of
Chapter 42 §42.3. It is the same operation: add the gains, subtract the losses,
compare against sensitivity, and look at what is left.

## What breaks here

**A link that works at one rate and not a higher one, with no physical change.**
Higher rates use higher frequencies (copper) or tighter timing margins (fibre), so
the same cable presents more loss and less tolerance. This is the diagnosis for
Chapter 5's exercise E1 and it is common at every equipment refresh.

**A fibre link with a dirty connector.** A fingerprint on a ferrule can cost
several dB, which is a substantial fraction of a typical budget. It is the most
common fibre fault by a wide margin, it is invisible without inspection, and the
remedy costs almost nothing. Chapter 64 §64.4 covers the meter that finds it.

**A copper run "just a bit over" 100 m.** It will probably link. It will show CRC
errors under load, intermittently, and worse when the riser is warm. The counters
are the evidence (Chapter 65 §65.1).

**Buying margin with transmit power.** Chapter 1's lesson, and it applies here with
a twist: on fibre, too *much* power overloads the receiver and degrades
performance, so an optical link can fail for being too short as well as too long.
Attenuators exist for exactly this.

> **Network+ note.** Objective 1.5 expects the distance limits and the media
> characteristics; objective 5.2 expects you to diagnose cable connectivity
> issues, of which attenuation-related ones are the largest class. The figure to
> carry is that copper's loss rises with frequency, which is why a cable's category
> is a *bandwidth* rating and why a longer run fails at a higher rate first.
