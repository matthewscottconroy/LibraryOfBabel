# 9.4 Code-Division and Wavelength-Division Multiplexing

Two remaining techniques. One divides by a mathematical property rather than a
physical one; the other is FDM applied to light and is the single most economically
important multiplexing technology in existence.

## Code-division multiplexing

Everybody transmits **simultaneously**, across the **whole band**, all the time.
Each transmitter multiplies its data by a distinct high-rate **spreading code**, and
each receiver correlates against the code it wants.

The mechanism depends on the codes being **orthogonal**: the correlation of a code
with itself is large, and with any other code is approximately zero. So a receiver
correlating against code *k* recovers transmitter *k*'s data and sees everyone
else's transmissions as noise.

The cocktail-party analogy is the standard one and is genuinely apt: many
conversations in one room, at once, and you can follow the one in a language you
speak while the rest is background.

**Processing gain** is what makes it work. If the spreading code runs *G* times
faster than the data, the correlation concentrates the wanted signal's energy by a
factor of *G* while the interference remains spread:

$$G_p = 10\log_{10}\left(\frac{\text{chip rate}}{\text{data rate}}\right) \ \text{dB}$$

For IS-95 cellular: 1.2288 Mchip/s spreading a 9.6 kb/s vocoder gives a processing
gain of 128, or 21 dB.

That gain has a striking consequence: **the wanted signal can be below the noise
floor and still be recovered.** GPS is the clearest example — its signals arrive at
around −130 dBm, roughly 20 dB *beneath* thermal noise in the receiver's bandwidth,
and 43 dB of processing gain lifts them out. A GPS receiver is decoding something
it cannot detect.

**Properties:**

- **Soft capacity.** Adding a user does not consume a fixed channel; it raises
  everyone's noise floor slightly. So the system degrades gradually rather than
  blocking, which is the opposite of FDM and TDM's hard limits.
- **Inherent resistance to narrowband interference**, which the correlator spreads
  out while concentrating the wanted signal.
- **Low probability of intercept**, which is why the technique originated in
  military communications.
- **Requires stringent power control.** A transmitter too close to the receiver
  drowns distant ones — the **near-far problem** — so CDMA cellular systems adjust
  transmit power hundreds of times per second. This is the technique's principal
  operational burden.

**Where it went.** CDMA dominated 3G — IS-95, CDMA2000 and UMTS all used it — and
5G does not. LTE and 5G NR use OFDMA instead, because OFDM's per-subcarrier
flexibility, its easier equalisation in wideband channels, and its lower receiver
complexity won out. CDMA survives in GPS and other GNSS systems, in some satellite
links, and as the spreading component inside other schemes.

### The Lamarr and Antheil story

Worth telling because it is true and frequently told badly.

In 1942, the actress **Hedy Lamarr** and the composer **George Antheil** were
granted US Patent 2,292,387 for a "Secret Communication System": a
frequency-hopping scheme to prevent radio-controlled torpedoes being jammed. Lamarr
had absorbed a great deal about weapons systems from her first marriage to an
Austrian armaments manufacturer; Antheil had experience synchronising multiple
player pianos for his *Ballet Mécanique*, and the patent's synchronisation
mechanism used a punched paper roll on exactly that principle.

The US Navy declined to use it. The patent expired unexploited. Frequency hopping
and related spread-spectrum techniques became central to military and then civilian
communications decades later, and Lamarr received essentially no recognition until
the 1990s.

Two caveats worth stating, because the story is often inflated. Frequency hopping is
**not** the same thing as CDMA — hopping is a spread-spectrum technique, and
direct-sequence CDMA is a different one, though both fall under the spread-spectrum
family. And the patent was one of several contemporaneous ideas rather than a
uniquely originating one. What is genuinely remarkable is that two people entirely
outside the field produced a sound and well-specified design that the professionals
ignored, and that the pattern of ignoring it took fifty years to correct.

## Wavelength-division multiplexing

FDM applied to light, and the economically decisive technology in this chapter.

Different wavelengths — different colours — propagate down one fibre
simultaneously without interacting. Each is modulated independently. At the far end,
optical filters separate them.

The frequencies involved are enormous: 1550 nm corresponds to about 193 THz. The
ITU-T G.694.1 grid specifies channel spacings of 100 GHz, 50 GHz or 25 GHz — which
at these frequencies is a spacing of about 0.8, 0.4 or 0.2 nanometres.

| Scheme | Spacing | Channels | Typical use |
|---|---|---|---|
| CWDM | 20 nm | 8–18 | Metro, enterprise; uncooled lasers, cheap |
| **DWDM** | 0.4–0.8 nm | **40–96** | Long haul, submarine |
| Flexible grid | variable | 100+ | Modern coherent systems |

At 96 channels × 400 Gb/s per channel, a single fibre pair carries **38.4 Tb/s**.
Chapter 50 §50.3 covers the systems; the point here is the multiplication factor.

### The two technologies that made it possible

**The erbium-doped fibre amplifier.** Without it, each of 96 wavelengths would need
its own regenerator — an optical-to-electrical-to-optical conversion — every 40 km,
which is 96 regenerators per hut and economically absurd.

The EDFA amplifies light *as light*, and crucially it amplifies **the whole C-band
at once**. One device, 96 wavelengths, all amplified together. Developed by David
Payne's group at Southampton and independently at Bell Labs from 1987, it is the
enabling technology for long-haul WDM and it is why the C-band is the C-band:
erbium's gain happens to sit at 1530–1565 nm.

**Wavelength-selective filters** — thin-film filters, fibre Bragg gratings, and
arrayed waveguide gratings — which separate the channels at the receiver, and
**reconfigurable optical add-drop multiplexers** which let a wavelength be dropped
at an intermediate site without disturbing the others. That capability is the
optical equivalent of SONET's add-drop (Chapter 50 §50.2) and is what makes a
wavelength a sellable product.

### The economics

This is why WDM matters more than its technical elegance suggests.

Chapter 10 and Chapter 49 both establish that **the path costs and the capacity is
nearly free**. Laying a cable costs money for trenching, permits, ships and labour;
the terminal electronics are a small fraction of it.

WDM means a fibre laid in 2001 for 10 Gb/s carries 38 Tb/s today **with no change to
the glass** — only the equipment at the ends. That is a capacity increase of nearly
four thousand times on infrastructure already paid for, and it is the single largest
reason bandwidth prices collapsed between 1995 and 2015.

It is also why "just lay more fibre" is rarely the answer to a capacity problem:
lighting another wavelength on existing fibre is dramatically cheaper than
installing new fibre, right up until the fibre is full.

## Comparing the four techniques

| | FDM | TDM | CDMA | WDM |
|---|---|---|---|---|
| Divides | Frequency | Time | Code space | Wavelength |
| Transmit when | Always | In your slot | Always | Always |
| Occupies | Your band | Whole band | Whole band | Your wavelength |
| Guard overhead | Guard bands | Framing bits | None | Channel spacing |
| Capacity limit | Hard | Hard | **Soft** | Hard |
| Key requirement | Filters | Synchronisation | Power control | Amplifiers, filters |
| Modern instance | DSL, cable, broadcast | T1/E1, SONET | GPS | Long-haul fibre |

And **statistical multiplexing** (§9.3) sits outside the table entirely, because it
divides nothing in advance — which is exactly why it behaves so differently.

## What breaks here

**A CDMA system with broken power control.** One nearby transmitter at full power
denies service to every distant one. The near-far problem is not a corner case; it
is the technique's defining operational constraint.

**A DWDM system with an amplifier problem.** Because the EDFA amplifies all
channels together, its gain is not flat across the band and it tilts with input
power. A channel added or removed changes the gain seen by every other channel —
**gain transients** — and systems need dynamic gain equalisation to manage it. A
single channel failure can therefore degrade its neighbours.

**Wavelength drift.** A laser whose wavelength drifts with temperature moves into
its neighbour's channel. DWDM lasers are temperature-stabilised for this reason,
and CWDM's 20 nm spacing exists precisely so that uncooled lasers can be used.

**Fibre nonlinearity at high channel counts.** Four-wave mixing and cross-phase
modulation between channels become significant at high power and close spacing.
This is why dispersion-shifted fibre — which put zero dispersion at 1550 nm — turned
out to be a mistake for DWDM: zero dispersion allows the channels to stay in phase
long enough to mix efficiently.

> **Network+ note.** N10-009 expects CWDM and DWDM by name under the fibre and WAN
> objectives, and expects you to know that they multiply a fibre's capacity by
> carrying multiple wavelengths. CDMA appears only as cellular history. The
> transferable point is §9.4's economics: **capacity on installed fibre is cheap
> and new fibre is expensive**, which shapes every carrier's upgrade decision.
