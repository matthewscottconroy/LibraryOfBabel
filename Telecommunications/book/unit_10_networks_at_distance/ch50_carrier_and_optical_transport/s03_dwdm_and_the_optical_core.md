# 50.3 DWDM and the Optical Core

A fibre pair laid in 2001 carries hundreds of times more traffic today than the day it was
lit, and nobody touched the glass. This section is how.

## The idea

Wavelength-division multiplexing is Chapter 9's frequency-division multiplexing, applied to
light.

A fibre carries one wavelength. It also carries a second, and a ninety-sixth, each modulated
independently, all in the same glass, none interacting — because the fibre is linear enough
over the relevant power range that superposition holds.

```
   λ1 ──┐                                          ┌── λ1
   λ2 ──┤                                          ├── λ2
   λ3 ──┼──▶ [mux] ═══one fibre═══ [demux] ──▶ ────┼── λ3
    :  ──┤                                          ├── :
   λ96──┘                                          └── λ96
```

The ITU grid (G.694.1) anchors the channels at **193.1 THz** and spaces them at 100, 50,
25 or 12.5 GHz. In the C-band — roughly 1530–1565 nm, about 4.4 THz wide — that gives
about 44 channels at 100 GHz spacing or 88 at 50 GHz.

| | Channels | At 400 Gb/s each |
|---|---|---|
| **C-band, 100 GHz** | ~44 | 17.6 Tb/s |
| **C-band, 50 GHz** | **~88** | **35 Tb/s** |
| C + L band, 50 GHz | ~190 | **76 Tb/s** |

> **One fibre pair. Tens of terabits.** And the same pair carried 2.5 Gb/s on a single
> wavelength when it was installed.

## Why 1550 nm

**Not a free choice.** Three windows exist in silica fibre, and the third one won for two
independent reasons.

| Window | Loss | Note |
|---|---|---|
| 850 nm | **~2 dB/km** | multimode, short reach only |
| 1310 nm | ~0.35 dB/km | **zero dispersion in standard fibre** |
| **1550 nm** | **~0.2 dB/km** | **minimum attenuation — and erbium amplifies here** |

1550 nm is the loss minimum for silica, set by Rayleigh scattering rising towards shorter
wavelengths and infrared absorption rising towards longer ones (Chapter 6). The minimum is a
property of the material, not of the manufacturing.

And erbium's gain band happens to sit at 1530–1565 nm.

> The C-band is the C-band because two unrelated pieces of physics — silica's loss minimum
> and erbium's fluorescence — coincide to within a few tens of nanometres. There is no
> engineering reason for this. It is luck, and the entire long-haul industry is built on it.

## The EDFA

The single enabling technology, and the reason DWDM is economic rather than merely
possible.

**Before 1987**, extending an optical link meant **regeneration**: convert light to
electricity, recover the clock, retime and reshape the bits, convert back to light.

Which meant, for a DWDM system, one regenerator per wavelength per site.

```
   Without EDFA — every 40 km:
   λ1 ──▶ [O/E] ──▶ [retime] ──▶ [E/O] ──▶ λ1
   λ2 ──▶ [O/E] ──▶ [retime] ──▶ [E/O] ──▶ λ2      96 of these,
    :                                               in a building,
   λ96──▶ [O/E] ──▶ [retime] ──▶ [E/O] ──▶ λ96      with power and cooling

   With EDFA — every 80–100 km:
   all λ ═══▶ [ EDFA ] ═══▶ all λ                   one device
```

An erbium-doped fibre amplifier is a length of fibre doped with erbium ions, pumped by a
laser at 980 or 1480 nm. The pump raises erbium into an excited state; an arriving signal
photon stimulates emission, and the signal is amplified as light, without ever becoming
electricity.

**Its three properties, in order of importance:**

**It amplifies the whole band at once.** All 96 wavelengths, in one device, because
stimulated emission does not care which photon triggered it. This is the property that
collapses the cost.

**It is transparent to modulation and rate.** An EDFA installed for 2.5 Gb/s per channel
amplifies 400 Gb/s per channel without modification — which is precisely why a fibre laid in
2001 can be upgraded by changing only the terminals.

**It is simple.** A coil of doped fibre, two pump lasers, some passive optics. No clock
recovery, no electronics in the signal path, nothing rate-specific to fail.

**The cost:** an amplifier amplifies noise as well as signal, and each one adds spontaneous
emission of its own. Optical SNR degrades with every span, which is what ultimately limits
reach — and it is why §50.2's FEC is worth so much.

> The EDFA did not make optical transmission possible. It made optical transmission
> divisible from the number of channels, and that is why the industry looks the way it does.

## Coherent detection

The second transformation, from about 2008, and it changed the economics again.

**Direct detection** — a photodiode measuring intensity — **discards phase.** So the only
usable modulation is on/off keying or simple intensity levels, and the achievable spectral
efficiency is low.

Coherent detection mixes the incoming signal with a local oscillator laser, recovering
**amplitude and phase** — which makes the entire QAM toolkit of Chapter 8 §8.3 available at
optical frequencies.

| Modulation | Bits/symbol | × 2 polarisations | At 64 Gbaud |
|---|---|---|---|
| **DP-QPSK** | 2 | **4** | **256 Gb/s** |
| **DP-16QAM** | 4 | **8** | **512 Gb/s** |
| DP-64QAM | 6 | 12 | 768 Gb/s |

Polarisation multiplexing doubles it again: two independent signals on orthogonal
polarisations of the same wavelength, separated in the receiver's DSP. It is MIMO
(Chapter 44 §44.4) with two spatial streams, in a fibre.

### The part that mattered most

**Coherent receivers compensate chromatic dispersion electronically.**

**Chromatic dispersion** — different wavelengths travelling at slightly different speeds,
spreading a pulse (Chapter 6) — used to be corrected optically, with spools of
dispersion-compensating fibre at every amplifier site. Bulky, lossy, and specific to the
route's length.

A coherent receiver has the full complex field in digital form, so it can simply undo the
dispersion in a digital filter.

> This removed the dispersion compensation modules from the line, and with them the
> route-specific engineering. A coherent system can be dropped onto a fibre route with no
> knowledge of its dispersion, and the receiver works it out. That is why upgrading a
> route became a terminal-equipment purchase rather than a construction project.

And the same DSP handles polarisation-mode dispersion, laser phase noise and nonlinear
compensation — all problems that were previously addressed with physical hardware in the
line.

## Flexible grid and the Shannon limit again

Fixed 50 GHz channels waste spectrum when a signal needs 37.5 GHz and waste capacity when it
needs 75.

Flexible grid (G.694.1 revision) allocates in **12.5 GHz slices**, so a 400 Gb/s signal
takes what it needs. Combined with adaptive modulation — QPSK for long reach, 16QAM or
64QAM for short — a modern transponder is set per route.

And the trade is exactly Shannon's (Chapter 4 §4.2):

$$C = B \log_2(1 + \mathrm{SNR})$$

A 50 GHz channel at 15 dB optical SNR has a theoretical capacity around 250 Gb/s; at 20 dB,
about 330. Higher-order modulation demands more SNR, which means fewer spans before
regeneration.

| Route | Choice |
|---|---|
| **Metro, 80 km** | **64QAM — maximum bits, SNR is plentiful** |
| Regional, 600 km | 16QAM |
| **Transatlantic, 6,600 km** | **QPSK, or 8QAM — reach dominates** |

> The reach–capacity trade is the whole of optical engineering at this scale, and it is the
> same trade as Chapter 47's LoRa spreading factors and Chapter 44's MCS index. **Three
> completely different media, one Shannon curve.**

## ROADMs and the optical layer as a network

A reconfigurable optical add-drop multiplexer switches wavelengths without converting them to
electricity.

```
              ┌──────────────┐
   West ══════│              │══════ East
              │    ROADM     │
              └──┬────────┬──┘
                 │        │
              drop λ7   add λ7
                 │        │
              local router / transponder
```

A wavelength can pass straight through a node — "express" — or be dropped locally. With
colourless, directionless and contentionless (CDC) ROADMs, any wavelength can be dropped
to any port in any direction, and the wavelength plan becomes software.

Which turns the optical layer into a network in its own right, with its own control plane,
its own restoration, and its own capacity planning — separate from and underneath the IP
network that rides on it.

> A packet that crosses a continent may pass through twenty amplifier sites and four ROADMs
> without once becoming electricity. The IP layer sees one link.

And that invisibility is a genuine operational hazard. Two IP links that appear
diversely routed may share a fibre, a duct or a ROADM — a shared risk link group — and
the IP layer has no way to know. Chapter 56 §56.2 returns to it; the lesson here is that
diversity must be verified at the optical layer, on paper, against actual route maps.

## What breaks here

**One wavelength errored, the rest clean.** The transponder or the client optics, not the
line. The line amplifies everything equally.

**All wavelengths degrading together.** **The line** — a bend, a dirty connector, a failing
amplifier pump laser, or fibre damage.

**Pre-FEC errors rising slowly over months.** **Margin being consumed** — ageing, a
deteriorating splice, or added connectors. This is the early warning and it is the most
valuable number on the system.

A new wavelength turning up and degrading existing ones. **Nonlinear effects** — four-wave
mixing or cross-phase modulation — or the amplifier's gain being redistributed when
channel count changes. Amplifier gain must be re-levelled when channels are added.

**Protection switching that does not protect.** **Shared risk.** The "diverse" route shares a
bridge, a duct, or a river crossing. Verify against physical route maps, not against circuit
IDs.

**Distance suddenly mattering after an upgrade.** A higher-order modulation was selected,
which needs more SNR than the route provides. Drop to 16QAM or QPSK and the reach returns.

> **Network+ note.** Objective 1.5 covers fibre and touches WDM. Over-learn: **WDM carries
> multiple wavelengths on one fibre**; CWDM uses wide spacing and few channels, DWDM narrow
> spacing and many; **single-mode fibre is used for long distance**; and **optical amplifiers
> avoid electrical regeneration.** The CWDM/DWDM distinction is the examinable content.
