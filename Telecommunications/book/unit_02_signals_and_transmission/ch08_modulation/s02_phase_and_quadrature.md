# 8.2 Phase and Quadrature

§8.1 concluded that phase is the most useful parameter. This section develops the
framework that makes phase tractable, and it is the single most important
conceptual tool in radio engineering.

## The problem with thinking about phase directly

Phase is awkward. It is an angle, it wraps around at 360°, and "add a bit more
phase" is not an operation that maps onto anything a circuit does naturally.
Building a transmitter that produces an arbitrary phase by *rotating* something is
not how electronics works.

The quadrature representation solves this by turning a phase-and-amplitude problem
into two independent amplitude problems.

## The trigonometric identity that makes it work

Start with a carrier at arbitrary amplitude and phase:

$$s(t) = A\cos(2\pi f t + \phi)$$

Expand using the angle addition formula:

$$s(t) = A\cos\phi\cos(2\pi f t) - A\sin\phi\sin(2\pi f t)$$

Now define two constants:

$$I = A\cos\phi \qquad Q = -A\sin\phi$$

and the signal becomes

$$s(t) = I\cos(2\pi f t) + Q\sin(2\pi f t)$$

Read what that says. **Any carrier of any amplitude and any phase is the sum of a
cosine and a sine at the same frequency, with appropriate amplitudes.** The phase
information has been converted entirely into the *ratio* of two amplitudes.

*I* is the **in-phase** component; *Q* is the **quadrature** component.
"Quadrature" means 90° apart, which is what a sine is relative to a cosine.

## Why this is the useful move

Three consequences, and each is a piece of practical engineering.

**A transmitter becomes two multipliers and an adder.** Generate a cosine and a
sine at the carrier frequency — easy, from one oscillator and a 90° phase shifter.
Multiply each by a voltage. Add. Any point in amplitude-phase space is reachable
by choosing two voltages, and choosing voltages is what electronics does well.

```
     I ──────►[×]──┐
                   │
   cos(2πft) ──────┘   ├──►[+]──► s(t)
                       │
     Q ──────►[×]──┐   │
                   │───┘
   sin(2πft) ──────┘
```

**A receiver becomes two multipliers and two filters.** Multiply the received
signal by a local cosine and low-pass filter: the result is *I*. Multiply by a
local sine and filter: the result is *Q*. Two numbers, from which amplitude and
phase follow:

$$A = \sqrt{I^2 + Q^2} \qquad \phi = \arctan\left(\frac{-Q}{I}\right)$$

**The two components are independent.** Because sine and cosine at the same
frequency are orthogonal — their product integrates to zero over a cycle — the
*I* channel carries no *Q* information and vice versa. **Two independent data
streams on one carrier, at one frequency, in the same bandwidth.**

That last point is worth pausing on. It is not a trick or an approximation. It
doubles the information a carrier carries, exactly, for free, and it falls directly
out of orthogonality.

## The I/Q plane

Plot *I* horizontally and *Q* vertically and every possible carrier state is a
point:

```
                Q
                │
          •     │     •
                │
    ────────────┼────────────  I
                │
          •     │     •
                │
```

- **Distance from the origin** = amplitude
- **Angle from the I axis** = phase

A modulation scheme is now simply **a chosen set of points in this plane**. That
set is a **constellation**, and §8.3 is about choosing one.

The schemes of §8.1 all appear here:

| Scheme | Points | Where |
|---|---|---|
| BPSK | 2 | On the I axis, at ±*A* |
| QPSK | 4 | At the four diagonals, all at radius *A* |
| ASK | 2+ | On the I axis at different radii |
| 16-QAM | 16 | A 4×4 grid |

**BPSK** uses only *I*, with two values. **QPSK** uses *I* and *Q* with two values
each — and because they are independent, QPSK carries **two bits per symbol at the
same power and bandwidth as BPSK carries one**. Four points at 90° spacing, all at
the same radius from the origin, and the minimum distance between adjacent points
is not much less than BPSK's.

This is the free lunch that quadrature provides, and it is why QPSK rather than
BPSK is the workhorse robust mode in every modern standard.

## Constellation diagrams as measurements

The I/Q plane is not merely a way of drawing a scheme. It is a **measurement**, and
this is where it becomes a diagnostic tool rather than a teaching aid.

A receiver demodulates each symbol into an *I* value and a *Q* value. Plot
thousands of received symbols on the plane and the result should be tight clusters
at the ideal constellation points. What you actually see is diagnostic:

| What you see | What it means |
|---|---|
| Tight, round clusters at the ideal points | Healthy link; the spread is thermal noise |
| Clusters **smeared radially** (in and out) | Amplitude noise, or AGC instability |
| Clusters **smeared tangentially** (arcs around the origin) | **Phase noise** — an unstable oscillator |
| Whole constellation **rotating slowly** | Frequency offset between transmitter and receiver |
| **Outer points pulled inward** | Amplitude compression — the transmitter's amplifier is saturating |
| A **second faint constellation** offset from the first | An interfering signal, or a multipath reflection |
| Clusters **elongated along one axis** | I/Q imbalance in the modulator |

This is a genuinely practical skill. Cable technicians, satellite engineers and
anyone working with DOCSIS or DVB read constellations daily, and the pattern tells
them what to fix. A radially smeared constellation and a tangentially smeared one
have completely different causes and completely different remedies, and both look
like "poor signal quality" on any simpler measurement.

**Error vector magnitude** is the summary statistic: the RMS distance between where
each symbol landed and where it should have landed, expressed as a percentage of
the constellation's scale. It is the single number quoted on transmitter datasheets
and in test reports, and it captures everything the diagram shows without
distinguishing the causes.

## Why every modern radio is I/Q

The architecture has become universal, and it is worth knowing that this is a
recent development.

Older radios modulated directly, with circuits specific to the scheme in use — an
FSK modulator was a different piece of hardware from a PSK modulator. Modern radios
generate *I* and *Q* **digitally**, in a processor, and feed them to a pair of
digital-to-analog converters and a quadrature modulator that is entirely
scheme-agnostic.

The consequence: **changing modulation scheme is a software change.** The same
radio hardware transmits BPSK, QPSK, 16-QAM, 256-QAM or 4096-QAM depending on what
the processor puts on the *I* and *Q* lines. This is what makes rate adaptation
(§8.3) possible at all — a Wi-Fi radio changing modulation forty times a second is
changing numbers in a buffer, not switching circuits.

It is also the foundation of **software-defined radio**: if the hardware is a
generic quadrature up-converter and down-converter, then the entire radio is
software, and a single device can be a Wi-Fi transceiver, a cellular modem, a
satellite receiver or a spectrum analyser depending on what it is running. GNU
Radio and the inexpensive SDR dongles that Chapter 5's further reading recommends
are exactly this.

## What breaks here

**A constellation rotating slowly** — frequency offset. Either the local
oscillator is off, or there is Doppler shift, or the reference is not locked.

**Outer points pulled in** — the power amplifier is being driven into compression.
Reduce transmit power. This is a case where turning the power *down* improves the
link, which surprises people, and it is a distinct mechanism from Chapter 4's
diminishing returns and Chapter 6's intermodulation, though all three are consequences
of the same non-linearity.

**Tangential smearing** — phase noise, usually from a poor or unlocked oscillator,
or from a reference that is drifting with temperature.

**I/Q imbalance** — the two paths have unequal gain or are not exactly 90° apart.
Produces an elongated constellation and an image of the signal on the wrong side of
the carrier. Modern transceivers calibrate this out; older ones need adjustment.

> **Network+ note.** Constellations are not examined. What is worth carrying is
> that a wireless link's quality has more dimensions than "signal strength", and
> that two links with identical RSSI can perform completely differently because
> one has a clean constellation and the other does not. Chapter 45's diagnostic
> procedure — signal, noise, airtime, or client — is the exam-level expression of
> the same idea.
