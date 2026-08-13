# 8.1 Carriers and Keying

## Why a carrier is not optional

Chapter 7 put bits directly onto a wire and it worked. Radio cannot do that, and
the reason is not a protocol decision or a regulatory requirement. It is the size
of the antenna.

**The antenna argument, with numbers.**

An antenna radiates efficiently when its physical size is a substantial fraction of
the wavelength it is transmitting. The standard compromise is a quarter wave, and
the wavelength is

$$\lambda = \frac{c}{f}$$

Now take a baseband voice signal at 3 kHz:

$$\lambda = \frac{3 \times 10^8}{3 \times 10^3} = 100{,}000 \ \text{m} = 100 \ \text{km}$$

A quarter-wave antenna would be **25 kilometres tall**. This is not a matter of
engineering effort; it is a matter of the signal's wavelength being longer than any
structure humans build.

Move the same voice signal onto a 900 MHz carrier:

$$\lambda = \frac{3 \times 10^8}{9 \times 10^8} = 0.333 \ \text{m}$$

A quarter wave is **8.3 cm** — the whip on a car roof. At 2.4 GHz it is 3.1 cm, at
5 GHz 1.5 cm, and at 28 GHz 2.7 mm, which is why millimetre-wave phased arrays can
pack dozens of elements into a few square centimetres and why beamforming is
practical up there and impractical at 900 MHz.

**You modulate onto a carrier because you cannot build an antenna for baseband.**
That is the whole of the first reason.

**The sharing argument**, which is independent and equally important. Chapter 5 §5.4
established that a baseband signal occupies the spectrum from DC upward and cannot
be moved. Two baseband transmitters on one medium therefore occupy the same band
and interfere.

A modulated signal can be placed anywhere. Move one conversation to 900.2 MHz and
another to 900.4 MHz and they coexist, each recovered by a receiver tuned to its own
band, neither aware of the other. That is frequency-division multiplexing
(Chapter 9 §9.1), and it is why a thousand radio stations, forty television
channels and every mobile phone in a city can share the same air.

Neither argument applies to a wire between two switches, which is exactly why wired
Ethernet uses baseband line coding and radio uses modulation. The two chapters are
two answers to one question, differing because the constraints differ.

## The three parameters

A sinusoidal carrier is completely described by three numbers:

$$s(t) = A \cos(2\pi f t + \phi)$$

- **A** — the amplitude
- **f** — the frequency
- **φ** — the phase

That is all. There is no fourth parameter. Therefore there are exactly three
primitive things you can vary to carry information, and every modulation scheme
that has ever existed is a combination of them plus the decision to use several
carriers at once.

## Amplitude-shift keying

Vary the amplitude; hold frequency and phase constant. In the simplest binary form,
**on-off keying**: transmit the carrier for a `1`, transmit nothing for a `0`.

```
data:      1     0     1     1     0
       ╱╲╱╲╱╲        ╱╲╱╲╱╲╱╲╱╲╱╲
      ─      ──────                ──────
```

**Virtues:** trivially simple. A transmitter is a switch and a receiver is a diode
and a capacitor. This is why the earliest radio used it — Marconi's spark
transmitters were on-off keyed, and Morse code over radio is on-off keying by hand.

**Vices:** amplitude is exactly the property that noise, fading and attenuation
attack. A signal that has faded by 6 dB has half the power, and if the receiver's
threshold was set for the original level, every `1` now reads as a `0`. Amplitude
is the *least* robust of the three parameters, and this is why essentially nothing
modern uses ASK alone.

It survives in two places: **optical fibre**, where on-off keying of a laser is
standard because the optical channel does not fade the way radio does; and as a
*component* of QAM (§8.3), where it is combined with phase and where coherent
detection makes it viable.

## Frequency-shift keying

Vary the frequency; hold amplitude and phase constant. Binary FSK transmits one
frequency for `1` and another for `0`.

```
data:      1        0        1
       ╱╲╱╲╱╲   ╱‾╲_╱‾╲   ╱╲╱╲╱╲
        fast      slow      fast
```

**Virtues:** robust. The receiver measures *which* frequency arrived, not how big
it was, so amplitude variation from fading or attenuation is irrelevant. Detection
can be non-coherent — the receiver need not know the carrier's phase — which makes
the hardware simple and tolerant.

**Vices:** spectrally inefficient. Two frequencies must be far enough apart to be
distinguishable, which means the signal occupies more bandwidth than the data rate
strictly requires.

**Where it is used:** early modems (Bell 103 at 300 bit/s used FSK, and it is why
dial-up handshakes sounded the way they did); Bluetooth's basic rate uses Gaussian
FSK; LoRa uses a chirp variant; and essentially every low-cost, low-rate,
long-battery-life radio uses some form of it. When robustness matters more than
efficiency, FSK is the answer, and Chapter 47's IoT radios reflect that.

## Phase-shift keying

Vary the phase; hold amplitude and frequency constant. Binary PSK transmits the
carrier at 0° for one symbol and 180° for the other.

```
data:      1              0
       ╱╲╱╲╱╲╱╲    ╲╱╲╱╲╱╲╱
                   ↑
              phase reverses
```

**Virtues:** the most power-efficient of the three for a given error rate. Two
phases 180° apart are maximally distinguishable for a fixed amplitude, so BPSK
extracts the most reliability from the least power of any binary scheme. It also
generalises well: QPSK uses four phases at 90° spacing to carry two bits per
symbol, and the extension continues.

**Vices:** the receiver must know the carrier's phase reference, which requires
either a coherent detector (more complex) or differential encoding (which encodes
in phase *changes* rather than absolute phase, and pays a small penalty in error
rate — the same trick as differential Manchester in Chapter 7 §7.2, for the same
reason).

**Where it is used:** everywhere that matters. BPSK and QPSK are the robust modes
of every Wi-Fi standard, every cellular standard, every satellite link and every
digital broadcast system. When a link is marginal, the radio falls back to QPSK,
and when it is truly marginal it falls back to BPSK.

## Comparing the three

| | ASK | FSK | PSK |
|---|---|---|---|
| Varies | Amplitude | Frequency | Phase |
| Robust to fading | **No** | Yes | Yes |
| Bandwidth efficiency | Good | **Poor** | Good |
| Power efficiency | Poor | Moderate | **Best** |
| Receiver complexity | Lowest | Low | Higher |
| Modern use | Optical; inside QAM | Low-rate radio, Bluetooth, LoRa | Universal |

The pattern is that **phase is the most useful parameter and amplitude the least**,
which is initially surprising — amplitude is the intuitive one — and follows
directly from what the channel does to a signal. Attenuation and fading attack
amplitude. Frequency is robust but expensive in bandwidth. Phase is robust and
cheap, at the cost of needing a reference.

Hence §8.2's development: build a framework around phase, then add amplitude back
in as a second dimension once coherent detection has given us the reference we
need. That combination is QAM, and it is what every high-rate system uses.

## A note on why the "keying" language

The term is inherited from telegraphy. A Morse key was a switch, and "keying" a
transmitter meant switching it on and off. When engineers generalised to varying
other parameters, the word came along — amplitude-shift keying, frequency-shift
keying, phase-shift keying — and it now means "discrete modulation" in general.

The vocabulary is a fossil, like the baud that Chapter 4 inherited from Baudot and
the "dialling" that survives on telephones with no dial. Networking is full of
them, and they are usually a signal that some part of the system is older than it
looks.

> **Network+ note.** N10-009 does not examine modulation schemes by name. The
> connection that matters is that a wireless link's data rate depends on which
> modulation it has negotiated, and that the negotiation walks down toward more
> robust schemes as conditions worsen — which is objective 5.5's rate-adaptation
> behaviour, and §8.3 is where the ladder is set out.
