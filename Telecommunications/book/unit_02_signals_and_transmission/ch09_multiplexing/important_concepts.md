# Chapter 9 — Important Concepts

**The economics that force multiplexing** *(chapter)* — The cost of a
communications link is overwhelmingly in the **path**, not the capacity. Trenching
costs the same for one fibre or 144; a submarine cable's terminal electronics are a
small fraction of the ships and permits. Capacity is cheap and paths are expensive,
so every generation re-solves the problem of getting more conversations onto paths
already paid for.

**Frequency-division multiplexing** *(§9.1)* — Each conversation gets its own band;
everybody transmits continuously. Requires modulation to shift each signal to its
allocated band.

**Guard band** *(§9.1)* — Spectrum separating adjacent FDM channels so that real
filters, which have a finite transition region, can separate them. Pure waste:
22% in analog carrier telephony (4 kHz allocated for a 3.1 kHz signal). Eliminated
by OFDM's orthogonality, which is why OFDM is so much more spectrally efficient.

**The L-carrier hierarchy** *(§9.1)* — Group (12 channels), supergroup (60),
mastergroup (600), jumbogroup (3,600), built recursively by treating each level's
output as a signal to be modulated again. Retired in the 1980s because analog
amplification accumulates noise and digital regeneration does not.

**Time-division multiplexing** *(§9.2)* — Each conversation gets the whole channel
in a recurring slot. Depends entirely on synchronisation, which is why the framing
bit exists.

**DS0** *(§9.2)* — 8,000 samples/s × 8 bits = **64 kb/s**. Derived from Nyquist
(3.4 kHz speech needs ≥6.8 kHz sampling; 8 kHz leaves filter headroom) plus a
quantisation decision using µ-law or A-law companding. The atom of the entire
digital telephone hierarchy.

**T1** *(§9.2)* — 24 DS0s + 1 framing bit = 193 bits/frame × 8,000 frames/s =
**1.544 Mb/s**. North America and Japan.

**E1** *(§9.2)* — 32 slots (30 voice, 1 framing, 1 signalling) × 8 bits × 8,000 =
**2.048 Mb/s**. Elsewhere. The incompatibility is a genuine historical accident that
persists in carrier contracts.

**Robbed-bit signalling** *(§9.2)* — T1's original practice of stealing the least
significant bit of every sixth frame for signalling. Inaudible in voice, fatal to
data — which is the origin of the **56 kb/s** figure that pervades older networking
material.

**Plesiochronous hierarchy** *(§9.2)* — "Almost synchronous." Tributaries come from
different exchanges with independent clocks, so a multiplexer runs slightly fast and
inserts **stuffed bits**. Hence DS2 is 6.312 Mb/s rather than 4 × 1.544 = 6.176.

**Why PDH forced full demultiplexing** *(§9.2)* — Stuffed-bit positions are known
only level by level, so extracting one DS0 from a DS3 requires unwrapping the entire
hierarchy. Ruinous for a carrier dropping channels along a route, and the reason
SONET/SDH — synchronous to a common atomic reference — was designed. The capability
it buys is **add-drop multiplexing**.

**Slips** *(§9.2)* — Buffer overflow or underflow when two ends of a TDM link derive
timing from sources that differ, repeating or dropping a frame. Inaudible in voice,
destructive to data, and the reason carrier networks distribute timing from a primary
reference source.

**Statistical multiplexing** *(§9.3)* — Allocating capacity **on demand** rather than
by reservation. Not a physical technique: it divides nothing in advance. The
argument that decided packet switching versus circuit switching.

**The core arithmetic** *(§9.3)* — 100 users at 1 Mb/s, 5% active: reserved needs
100 Mb/s at 5% utilisation; statistical needs ~20 Mb/s with an overflow probability
around 2 × 10⁻⁸. **Multiplexing gain 5×.**

**Gain grows with population** *(§9.3)* — The mean number active grows as *n* while
its standard deviation grows as √*n*, so relative variability falls as 1/√*n*. Gain
rises from ~2× at 10 users to ~18× at 100,000. The law of large numbers doing
engineering work, and the reason aggregation is valuable at every level — and why
the economics favour large operators.

**What statistical multiplexing gives up** *(§9.3)* — Guaranteed bandwidth;
constant delay; **admission control** (the busy signal is an honest refusal before
you invest effort); freedom from per-unit header overhead; and immunity to
congestion collapse. Chapter 13 argues the loss of admission control is the one most
regretted.

**The independence assumption** *(§9.3)* — The binomial model assumes users are
uncorrelated. A software update pushed at 09:00, a live event, or a synchronised
timetable destroys the assumption and with it the gain. The most common way a
capacity plan built on this arithmetic fails.

**Code-division multiplexing** *(§9.4)* — Everybody transmits simultaneously across
the whole band, each multiplied by an orthogonal **spreading code**; receivers
correlate to recover their own.

**Processing gain** *(§9.4)* — 10 log₁₀(chip rate ÷ data rate). 21 dB for IS-95.
Concentrates the wanted signal while interference stays spread, which is why **GPS
signals arriving 20 dB below the noise floor are decodable**.

**Soft capacity and the near-far problem** *(§9.4)* — CDMA degrades gradually rather
than blocking, since each added user raises everyone's noise floor slightly. But a
close transmitter drowns distant ones, so stringent power control — hundreds of
adjustments per second — is the technique's defining operational burden.

**Lamarr and Antheil** *(§9.4)* — The 1942 frequency-hopping patent, ignored by the
US Navy and unrecognised for fifty years. Worth telling accurately: hopping is
spread spectrum but is **not** the same as direct-sequence CDMA, and the patent was
one of several contemporaneous ideas. What is remarkable is that two outsiders
produced a sound design the professionals dismissed.

**Wavelength-division multiplexing** *(§9.4)* — FDM at optical frequencies. **CWDM**
at 20 nm spacing (8–18 channels, uncooled lasers, cheap); **DWDM** at 0.4–0.8 nm
(40–96 channels). At 96 × 400 Gb/s, a single fibre pair carries **38.4 Tb/s**.

**The EDFA's role** *(§9.4)* — Amplifies **the whole C-band at once**, optically,
so 96 wavelengths need one device rather than 96 regenerators every 40 km. Without
it, DWDM is economically impossible. It also fixes the C-band's location: erbium's
gain sits at 1530–1565 nm.

**WDM's economic consequence** *(§9.4)* — A fibre laid in 2001 for 10 Gb/s carries
tens of terabits today with **no change to the glass**. This is the largest
reason bandwidth prices collapsed, and why lighting another wavelength beats laying
new fibre until the fibre is full.

**Gain transients and nonlinearity** *(§9.4)* — Because an EDFA amplifies all
channels together with non-flat gain, adding or removing a channel changes what
every other channel sees. And at high power with close spacing, four-wave mixing
between channels becomes significant — which is why dispersion-shifted fibre, with
zero dispersion at 1550 nm, proved a poor choice for DWDM.
