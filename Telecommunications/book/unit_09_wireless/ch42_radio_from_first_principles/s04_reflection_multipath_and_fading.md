# 42.4 Reflection, Multipath and Fading

§42.3's link budget assumed one path. **Indoors there are dozens**, and what happens when
several copies of the same signal arrive at slightly different times is the defining
problem of indoor wireless — **and, since 802.11n, its greatest asset.**

## What happens to a wave

**Five behaviours, and each has a practical consequence:**

| Behaviour | Occurs when | Effect |
|---|---|---|
| **Reflection** | the surface is large relative to λ | a second copy arrives later |
| **Refraction** | the wave crosses media of different density | the path bends |
| **Diffraction** | the wave passes an edge | it bends round corners — **partial coverage in shadow** |
| **Scattering** | the object is small relative to λ | energy spreads in many directions |
| **Absorption** | the material converts it to heat | §42.1's material table |

**Diffraction is why you have some signal round a corner** rather than a hard shadow, and
**it is stronger at lower frequencies** — another reason 2.4 GHz covers awkward geometry
better than 5 GHz.

**And metal reflects almost totally.** Filing cabinets, lift shafts, ductwork, foil-backed
insulation, metal shelving in a warehouse — **each is a mirror**, and a warehouse full of
metal racking is an extremely difficult radio environment for exactly this reason.

## Multipath

**Several copies of the signal arrive by different paths at different times.**

```
                    ╭─── reflected off ceiling ───╮
                   ╱                               ╲
   Transmitter ───┼──────── direct path ────────────┼─── Receiver
                   ╲                               ╱
                    ╰─── reflected off floor ─────╯
```

**The copies are delayed relative to each other by the difference in path length**, and at
the speed of light:

$$1\ \text{metre of extra path} = 3.3\ \text{nanoseconds of delay}$$

**Indoors, delay spreads of 50–300 ns are typical** — corresponding to path differences of
15 to 90 metres, which a few reflections in a large room easily produce.

### Constructive and destructive interference

**When the copies combine at the receiver, the result depends on their relative phase:**

```
   In phase (constructive):        Out of phase (destructive):

   ╱▔╲    ╱▔╲                      ╱▔╲    ╱▔╲
       ╱▔╲    ╱▔╲                      ╲▁╱    ╲▁╱
   ─────────────────                ─────────────────
   ═══════════════  stronger        ───────────────  cancelled
```

**Two copies of equal amplitude arriving exactly out of phase cancel completely.**

**And "exactly out of phase" is half a wavelength of extra path — 6.25 cm at 2.4 GHz.**

> **Moving a receiver a few centimetres can change the signal by 20 dB or more.**

**This is why holding a phone slightly differently changes the signal bars**, why an access
point works from one side of a desk and not the other, and why **a site survey taken while
standing still is misleading.**

### Fading

**The variation over time and space that multipath produces:**

| Type | Cause | Character |
|---|---|---|
| **Fast fading** | movement through the interference pattern | **rapid, deep, over centimetres** |
| **Slow fading** (shadowing) | obstructions coming and going | gradual, over metres |
| **Flat fading** | the whole channel affected equally | narrowband signals |
| **Frequency-selective fading** | **some frequencies cancelled, others not** | **wideband signals** |

**Frequency-selective fading is the one that matters for Wi-Fi**, and it follows directly:
the path difference is a fixed *distance*, so it corresponds to a different *phase* at
different frequencies. **So a 20 MHz channel may have deep nulls at some frequencies and be
perfectly clear at others.**

**Which is precisely why OFDM works** (Chapter 8 §8.4): the channel is divided into many
narrow subcarriers, **each individually experiencing flat fading**, and the ones in a null
are lost while the rest survive. **Forward error correction across the subcarriers then
reconstructs the lost data.**

> **OFDM does not defeat multipath by fighting it. It divides the channel into pieces small
> enough that each piece sees a simple problem.**

## Inter-symbol interference

**The other consequence of delay spread**, and it is what limits symbol rate.

**A delayed copy of symbol *n* arrives while symbol *n*+1 is being received**, and the two
overlap:

```
   Direct:      │ symbol 1 │ symbol 2 │ symbol 3 │
   Reflected:        │ symbol 1 │ symbol 2 │ symbol 3 │
                          ▲
                    overlap — symbol 1's echo corrupts symbol 2
```

**The faster the symbol rate, the shorter each symbol, and the worse the overlap.**

**The defence is a guard interval** — a gap between symbols, longer than the expected delay
spread:

| Standard | Guard interval | Tolerates delay spread up to |
|---|---|---|
| 802.11a/g | **800 ns** | ~240 m of path difference |
| 802.11n/ac short GI | **400 ns** | ~120 m |
| **802.11ax** | **800 ns / 1.6 µs / 3.2 µs** | **selectable — long GI for outdoor** |

**The guard interval is pure overhead** — time in which no data is sent — so **there is a
direct trade between throughput and multipath tolerance.**

**Short guard interval gives about 11% more throughput and fails in high-multipath
environments.** Which is why it is negotiable, and why a warehouse deployment should not
force it.

**802.11ax's long guard intervals are for outdoor and large-cell use**, where delay spreads
are much larger.

## Then it became an asset

**The reversal that defines modern Wi-Fi.**

**Until 802.11n (2009), multipath was purely a problem** to be mitigated with equalisers,
guard intervals and OFDM.

**MIMO — Multiple Input Multiple Output — uses it.**

**The insight:** if there are several independent paths between transmitter and receiver,
and both ends have several antennas, **the paths can carry different data simultaneously.**

```
   Tx antenna 1 ──┬─── path A ───┬── Rx antenna 1
                  ├─── path B ───┤
   Tx antenna 2 ──┴─── path C ───┴── Rx antenna 2

   Two independent spatial streams over the same frequency, at the same time.
```

**The receiver solves a system of linear equations** — it knows the channel between each
transmit and receive antenna pair, and can therefore separate the streams.

> **Multipath went from the enemy to the enabling condition.** A rich scattering environment
> — an office full of walls and furniture — supports *more* spatial streams than an open
> field, because the paths are more independent.

**Which produces a genuinely counter-intuitive result: MIMO works better indoors.** A
line-of-sight outdoor link has essentially one path, so the antennas see nearly the same
channel and the equations are ill-conditioned — **there is nothing to separate.**

**The stream count is bounded by the smaller antenna count:**

$$\text{streams} \le \min(N_{\text{tx}}, N_{\text{rx}})$$

**A 4×4 access point talking to a 2×2 laptop gets two streams**, not four — which is why AP
antenna counts beyond what clients have gives diminishing returns for throughput, though it
still helps for beamforming and diversity.

**Chapter 44 §44.4 develops MIMO, MU-MIMO and beamforming properly.**

## Diversity

**The simpler predecessor, and it is still used.**

**Two antennas, a small distance apart** — a few centimetres, so their multipath patterns
differ — **and the receiver uses whichever is better.**

**Because a null at one antenna is unlikely to coincide with a null at the other**, and
switching between them removes most deep fades.

| Diversity | Mechanism |
|---|---|
| **Spatial** | antennas separated in space |
| **Polarisation** | antennas at different polarisations |
| Frequency | the same data on different frequencies |
| Time | retransmission after the fade has passed |

**Even simple selection diversity gives several dB of effective gain** on a fading channel,
which is why two antennas appeared on access points long before MIMO.

## What this means in practice

**Five consequences worth carrying:**

**A survey must be taken while moving.** A stationary measurement samples one point in an
interference pattern that varies over centimetres.

**A room's behaviour changes when it is occupied.** Bodies absorb (§42.1) *and* alter the
reflection geometry.

**Metal is the dominant feature of any environment containing it.** Racking, lifts,
ductwork, foil insulation — plan around them rather than through them.

**Rate adaptation is doing something real.** A client dropping from MCS 9 to MCS 4 has
encountered a channel that will not support the higher modulation, and forcing the rate up
makes it worse rather than better.

**Multipath is not a fault.** In a MIMO deployment it is what makes the throughput possible,
and a deployment that eliminated all reflections would perform worse.

## What breaks here

**Signal varying wildly over small distances.** Fast fading. Normal, and it is why surveys
move.

**A location that works and a location one metre away that does not.** A null. Move the
access point rather than raising power.

**Poor performance in a warehouse with metal racking.** Reflections and shadowing. It is a
hard environment and needs more access points, not stronger ones.

**Short guard interval causing errors.** High delay spread. Allow the long interval.

**MIMO underperforming on an outdoor point-to-point link.** Too few independent paths.
Expected — MIMO wants scattering.

**A survey that looked fine and a deployment that does not.** The survey was taken in an
empty building, standing still.

> **Network+ note.** Objective 5.4 expects wireless interference and its causes.
> Over-learn: **multipath is the same signal arriving by several paths**; **it causes fading
> and inter-symbol interference**; **OFDM and guard intervals mitigate it**; and **MIMO
> exploits it to carry several spatial streams.** The reversal — that multipath became
> useful — is worth knowing because it explains why MIMO performs better indoors.
