# 6.2 Noise

Chapter 4 §4.3 established the thermal noise floor and computed it. This section
covers the noise sources that actually break links, which is a different list —
because thermal noise is small, predictable, and almost never the thing that
causes an outage.

## The five sources

**Thermal (Johnson–Nyquist) noise.** *N* = *kTB*. Unavoidable, flat across
frequency, proportional to bandwidth and temperature. −174 dBm/Hz at 290 K. It
sets the theoretical floor and it is the only one on this list you cannot reduce
by engineering the environment.

**Shot noise.** Current is carried by discrete charges arriving at random
intervals, so the current fluctuates. It matters in optical receivers, where the
photons themselves arrive as a Poisson process, and it becomes the dominant limit
in a well-designed optical front end. Its power is proportional to the signal
current, which makes it unusual: unlike thermal noise, it grows as the signal
grows.

**Intermodulation noise.** When two signals pass through a device that is not
perfectly linear — an amplifier driven near saturation, a corroded connection
acting as a diode — the output contains sums and differences of the input
frequencies. Two carriers at *f*₁ and *f*₂ produce products at *f*₁ ± *f*₂, at
2*f*₁ − *f*₂, and so on. The third-order products are the troublesome ones because
they land close to the originals and cannot be filtered out.

This is the impairment that appears when someone turns the transmit power up, and
it is a second reason — beyond Chapter 4's diminishing returns — that "more power"
is often the wrong answer.

**Crosstalk.** A neighbouring conductor's signal coupling into yours. §6.4 covers
it properly, because on twisted pair it is usually the binding constraint rather
than thermal noise.

**Impulse noise.** And this is the one that actually breaks things.

## Impulse noise, and why it dominates in practice

Impulse noise is brief, large, and irregular: a spike of energy lasting
microseconds to milliseconds, with an amplitude that can be tens of decibels above
the ambient floor.

Sources, all of them commonplace:

- **Switching transients.** Any inductive load being switched — a motor, a
  contactor, a fluorescent ballast, a lift — produces a fast, high-voltage spike
  as the field collapses. Buildings are full of them.
- **Lightning**, directly and by induction into long runs.
- **Power line disturbances**, and mains-frequency hum coupling in at 50 or 60 Hz
  and its harmonics.
- **Electrostatic discharge.**
- **Nearby radio transmitters** keying up.

Two properties make impulse noise categorically different from thermal noise.

**It is not Gaussian and not stationary.** Shannon's capacity formula assumes
additive white Gaussian noise, and impulse noise is neither white nor Gaussian.
The formula's prediction is therefore not a reliable guide to a link's behaviour in
an electrically noisy environment, and this is one of the honest caveats Chapter 4
§4.4 listed.

**It is bursty, which is worse than its average suggests.** A source producing 1%
average corruption spread evenly over time is very different from one producing
100% corruption for 1% of the time. The first damages one bit in a hundred and is
correctable; the second destroys entire frames and requires retransmission. Chapter
3 §3.3 made the same point about packet loss, and it is the same phenomenon a layer
down.

**The practical consequence:** in an industrial environment, impulse noise decides
the media choice. It is why shielded twisted pair or fibre is specified near
machinery, and why a link that is nowhere near its distance limit can still show
CRC errors that correlate with a production line's duty cycle rather than with
traffic load. That correlation — errors tracking a *machine* rather than the
network — is the diagnostic signature, and Chapter 65 §65.1 uses it.

## Noise figure: what the receiver adds

A real receiver contributes noise of its own. The **noise figure** quantifies how
much:

$$\text{NF (dB)} = \text{SNR}_{\text{in (dB)}} - \text{SNR}_{\text{out (dB)}}$$

A perfect receiver has NF = 0 dB. Real ones:

| Equipment | Typical NF |
|---|---|
| Cryogenically cooled radio astronomy front end | 0.1–0.5 dB |
| Good satellite receiver LNA | 0.5–1.5 dB |
| Cellular base station | 2–4 dB |
| Consumer Wi-Fi | 4–10 dB |

So a 20 MHz Wi-Fi receiver's practical noise floor is not the −101 dBm that
thermal noise alone gives, but roughly −95 dBm once a 6 dB noise figure is added —
which is why Chapter 43 §43.4 quotes that figure and why an analyser showing a
quiet channel reports something near it.

**Friis's formula** gives the noise figure of a chain, and it contains a lesson:

$$F_{\text{total}} = F_1 + \frac{F_2 - 1}{G_1} + \frac{F_3 - 1}{G_1 G_2} + \cdots$$

The **first stage dominates**, because every subsequent stage's contribution is
divided by the gain preceding it. This is why a low-noise amplifier goes at the
antenna rather than at the far end of the feeder cable, and why a long lossy cable
between antenna and LNA is a serious design error — the loss appears before the
gain and degrades the whole chain irrecoverably.

## Signal-to-noise ratio, and the two ways to improve it

$$\text{SNR}_{\text{dB}} = P_{\text{signal (dBm)}} - P_{\text{noise (dBm)}}$$

Two terms, so two levers, and their costs differ enormously:

**Raise the signal.** Increase transmit power, add antenna gain, shorten the path,
choose a lower-loss medium. Chapter 4 §4.4 showed the return is logarithmic in
capacity, and §6.2's intermodulation adds a second penalty at high power.

**Lower the noise.** Reduce the receiver's noise figure, narrow the bandwidth
(accepting less noise at the cost of a lower symbol rate), shield against impulse
sources, cool the front end, or — in a shared medium — remove the interferers.

In practice the second lever is usually cheaper and is usually the one ignored.
An administrator facing poor Wi-Fi reaches for transmit power; the productive
question is almost always what raised the noise floor. Chapter 45 develops this
into a diagnostic procedure, and it is the single most transferable idea in Unit IX.

## What breaks here

**CRC errors correlated with machinery rather than traffic.** Impulse noise. Look
for shielding, re-routing away from power runs, or fibre.

**A link that degrades when a specific piece of equipment starts.** Same, and the
correlation is the diagnosis. Chapter 64's flow and error data plotted against time
of day makes it visible immediately.

**Poor performance after "improving" a wireless network by raising power.**
Intermodulation, plus every other cell's noise floor raised, plus Chapter 45's
four mechanisms.

**A radio link with an LNA at the wrong end of the feeder.** Friis says the loss
before the gain is unrecoverable. Move the amplifier to the antenna.

**Hum at 50 or 60 Hz on an analog circuit.** Mains coupling, usually a grounding
problem. Almost extinct now that everything is digital, and still occasionally met
in audio and in legacy telephony.

> **Network+ note.** N10-009 expects EMI and interference as causes of cable
> connectivity problems (objective 5.2) and expects you to know that shielded
> cable and fibre are the remedies in electrically noisy environments. The
> mechanism — impulse noise being bursty and non-Gaussian, and therefore
> destroying whole frames — is what makes the remedy make sense.
