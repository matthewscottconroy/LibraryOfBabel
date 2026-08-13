# 71.2 Terabit Optics and Coherent Transmission

**Chapter 50 §50.3 established the mechanisms. This section is where they are running out**, and
**the interesting property is that the limit is Shannon's rather than engineering's.**

## Where the capacity has come from

**Four multipliers, applied over thirty years.**

| | Multiplier | Status |
|---|---|---|
| **More wavelengths** | **C-band's ~88 channels at 50 GHz** | **the band is full** |
| **More bandwidth per channel** | **50 → 75 → 150 GHz** | **flexible grid; and the band is still finite** |
| **More bits per symbol** | **QPSK → 16QAM → 64QAM → probabilistic shaping** | **SNR-limited** |
| **More baud** | **32 → 64 → 96 → 130+ Gbaud** | **electronics-limited, and advancing** |

**And the arithmetic of a modern transponder:**

$$\text{rate} = \text{baud} \times \text{bits/symbol} \times 2 \text{ (polarisations)}$$

| Baud | Modulation | **Per wavelength** |
|---|---|---|
| 64 Gbaud | DP-16QAM | **512 Gb/s** |
| **96 Gbaud** | **DP-64QAM** | **1.15 Tb/s** |
| **130 Gbaud** | **DP-256QAM** | **2.08 Tb/s** |

> **A single wavelength now carries more than an entire DWDM system did in 2000**, and **the
> increase has come almost entirely from the terminal equipment** — Chapter 50 §50.3's point
> about the fibre being unchanged.

## The limit, and it is Shannon's

**The uncomfortable observation for a field accustomed to exponential improvement.**

$$C = B \log_2(1 + \mathrm{SNR})$$

| Channel width | 15 dB OSNR | 20 dB | 25 dB |
|---|---|---|---|
| **75 GHz** | **0.38 Tb/s** | **0.50 Tb/s** | **0.62 Tb/s** |

**And the SNR cannot simply be raised**, because **the fibre is non-linear.**

> **Above a certain launch power, the Kerr effect — the refractive index varying with intensity
> — produces cross-phase modulation and four-wave mixing between channels.** **More power
> produces more noise, not more signal**, and **there is an optimal launch power beyond which
> capacity falls.**

**Which is the "non-linear Shannon limit"**, and it is the reason a modern system's capacity is
within a small factor of the theoretical maximum for its fibre and its band.

| Consequence | |
|---|---|
| **Capacity per fibre is bounded** | **and current systems are close to it** |
| **The trade is reach against rate** | Chapter 50 §50.3 — **and it is now the only trade** |
| **Improvement is incremental** | **a few per cent per generation, not doublings** |

> **The industry has moved from "how do we get more out of this fibre?" to "how do we use more
> fibre?"** — **and that is a different engineering problem with different economics.**

## The responses

**Four, in increasing order of departure from current practice.**

### More bands

**The C-band is 4.4 THz.** **The L-band adds a comparable amount**, and **S-band and beyond are
being developed.**

| Band | Wavelengths | Status |
|---|---|---|
| **C** | 1530–1565 nm | **full** |
| **L** | 1565–1625 nm | **deployed, doubling capacity** |
| **S** | 1460–1530 nm | **amplifier technology is the difficulty** |
| O, E, U | others | research |

**The obstacle is amplification** (Chapter 50 §50.3): **the EDFA works because erbium's gain band
happens to coincide with silica's loss minimum.** **Outside the C and L bands there is no
equivalent**, and **Raman amplification or semiconductor optical amplifiers are the alternatives
— each more complex, less efficient and more expensive.**

> **The C-band's convenience was a coincidence** (Chapter 50 §50.3), **and the bands beyond it do
> not have it.**

### More fibres

**Unglamorous and effective.** **Modern submarine cables carry 16 or 24 fibre pairs where earlier
ones carried 4 to 8** (Chapter 50 §50.5).

**And the constraint is power.** **A submarine cable's amplifiers are fed from shore
(Chapter 50 §50.5), and the available power is fixed** — **so more fibre pairs means less power
per pair.**

> **Which produced a genuine change in design philosophy: modern cables run each pair at lower
> spectral efficiency and gain more total capacity**, because **power per bit is minimised at a
> lower modulation order.** **The optimum is more pairs at QPSK rather than fewer at 16QAM**, and
> that inverts a decade of design instinct.

### Space-division multiplexing

**The research direction:** **multi-core fibre — several cores in one cladding — or few-mode
fibre carrying several spatial modes.**

| | |
|---|---|
| **Multi-core** | **4, 7, 19 cores in one fibre** — demonstrated at high capacity |
| **Few-mode** | **several propagation modes, separated by MIMO processing** |
| **Coupled-core** | **the modes mix and are separated computationally** — **which is Chapter 44's MIMO, in glass** |

**And the obstacle is the installed base.** **The world's fibre is single-core single-mode**, and
**space-division multiplexing requires new fibre** — **which for a submarine cable means a new
cable and for terrestrial routes means new duct work.**

> **Which is why it is a research direction rather than a deployment.** **The economics favour
> lighting more of the existing fibre before laying new fibre of a different kind**, and there is
> a great deal of unlit fibre.

### Better coding

**Incremental and real.**

**Probabilistic constellation shaping** — **transmitting outer constellation points less often
than inner ones** — **recovers a substantial fraction of the gap to Shannon**, and it is
deployed.

**And soft-decision FEC with higher overhead** (Chapter 50 §50.2) **continues to improve**, at
the cost of the overhead itself.

> **Both are approaching a limit rather than removing one**, and **the gains are single-digit
> percentages per generation.**

## What this means operationally

**Three consequences for someone operating a network rather than designing an optical system.**

**Capacity growth is slowing and cost per bit is falling more slowly than it did.** **The
assumption that bandwidth becomes cheaper indefinitely is weakening**, and **capacity planning
(Chapter 54 §54.1) should not assume that a future upgrade will be as cheap as the last.**

**Reach and rate are the trade, and it is now sharp.** **A 400 Gb/s wavelength that crosses an
ocean and a 800 Gb/s one that does not are different products**, and **the route determines the
rate** (Chapter 50 §50.3).

**And the margin is thinner.** **A system running near its Shannon limit has less headroom for
ageing, for a dirty connector or for an added span** (Chapter 65 §65.1) — **so the pre-FEC error
rate becomes a more important operational signal than it was** (Chapter 50 §50.2).

> **Which is a practical consequence of a theoretical limit: as systems approach it, they become
> more sensitive to the things that were previously absorbed by margin.**

## The 800G and 1.6T generation

**Where deployment actually is.**

| | Status |
|---|---|
| **400G** | **mature, and the current volume** |
| **800G** | **shipping; 2 × 400G lanes, or a single higher-baud carrier** |
| **1.6T** | **standardised, early deployment** |
| **3.2T** | in development |

**And the client-side story matters as much as the line side:** **an 800G optic in a switch port
is frequently 8 × 100G lanes electrically**, and **the electrical interface — 100 Gb/s per lane,
then 200 — is currently the harder engineering problem than the optical one.**

**Which produces a practical concern:** **power.** **A 1.6T optic dissipates 25–30 W**, and **a
switch with 32 of them dissipates a kilowatt in optics alone** (Chapter 56 §56.3, Chapter 67
§67.4's cabling and optics budget) — **and cooling a faceplate carrying that is a genuine
mechanical constraint.**

> **Which is why linear-drive and co-packaged optics are being developed**: **removing the DSP
> from the optic, or moving the optics onto the switch package, to reduce the power.** **The
> constraint on the next generation is thermal rather than optical**, which would have been a
> surprising statement fifteen years ago.

## What breaks here

**A capacity plan assuming the next upgrade costs what the last did.** **The exponential is
flattening.**

**A wavelength that will not reach after a modulation change.** **The reach-rate trade**
(Chapter 50 §50.3), and it is sharper now.

**A system that worked and now shows pre-FEC errors after a splice was added.** **The margin was
thin.** Near the limit, less is absorbed.

**Raising launch power to improve SNR and making it worse.** **The non-linear limit.** There is
an optimum.

**A switch faceplate that cannot be cooled.** **The optics' power.** Chapter 56 §56.3's argument,
at a new scale.

**An expectation that space-division multiplexing will arrive soon.** **It requires new fibre**,
and there is unlit fibre available first.

> **Network+ note.** Beyond the syllabus. The transferable content is Chapter 4's: **Shannon's
> limit is a property of the channel and not of the equipment**, and **a system operating near it
> improves only by widening the channel or improving the signal-to-noise ratio.** **Optical
> networking is the clearest large-scale demonstration of that constraint being reached.**
