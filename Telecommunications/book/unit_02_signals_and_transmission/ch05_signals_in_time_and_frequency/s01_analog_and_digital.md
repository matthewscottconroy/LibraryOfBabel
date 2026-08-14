# 5.1 Analog and Digital

Ask a room of students what the difference is between an analog and a digital
signal and you will get, almost without exception, an answer about shape. Analog
signals are smooth and wavy. Digital signals are square.

This is wrong, it is wrong in a way that matters, and it makes several later
topics incoherent — in particular it makes it impossible to understand what a
modem does, or why a "digital" signal on a real wire is a continuously varying
voltage that spends most of its time between the two values it is supposed to
have.

## The actual distinction

Every signal on every wire in this book is a **continuous physical quantity**. A
voltage does not jump; it ramps, with a slope set by the circuit's capacitance
and the driver's current. An electromagnetic field does not switch; it varies.
Nature does not do discontinuities.

What distinguishes analog from digital is not the physics. It is the **agreement
about which values are meaningful**:

> An **analog** signal is one in which the entire continuous range of values is
> meaningful — the value *is* the information.
>
> A **digital** signal is one in which only a discrete set of values is
> meaningful, and everything between them is understood as an imperfect rendering
> of the nearest permitted value.

That is Chapter 2 §2.4's principle again — meaning is supplied by agreement, not
by the bits — applied one layer down, to voltages instead of bit patterns.

Consider what this implies. If I put 4.1 volts on a wire and we have agreed that
this is an analog signal, then the information is *4.1 volts*, and 4.0 volts would
have been different information. If we have agreed it is a digital signal with
levels at 0 V and 5 V, then 4.1 volts is a `1` that has been knocked about a bit,
and so is 4.7, and so is 3.9. The receiver's job is not to measure but to
**decide**.

## Why the decision is everything

That difference — measure versus decide — produces the most consequential
property in this book.

An analog receiver measures. It cannot distinguish signal from noise, because it
has no criterion for what the signal was supposed to be. If a 4.1 V signal arrives
with 0.2 V of noise on it, the receiver faithfully reports 4.3 V, and the error is
now permanently part of the signal.

A digital receiver decides. It measures 4.3 V, observes that the permitted values
are 0 V and 5 V, and outputs `1`. **The noise is discarded.** Not attenuated —
discarded, completely, because it was never part of what the receiver was looking
for.

This is what permits **regeneration**.

## Amplification versus regeneration

An **amplifier** multiplies its input. Signal and noise alike:

```
  in:   signal 1.0 V + noise 0.1 V     →  amplify ×5  →  signal 5.0 V + noise 0.5 V
```

The signal-to-noise ratio is unchanged — that is what "amplify" means — and each
subsequent span adds fresh noise on top of noise that is now permanent. Chain ten
amplifiers and the accumulated noise dominates. This is why a transcontinental
analog telephone call in 1930 required enormous engineering care and still sounded
like a transcontinental analog telephone call.

A **regenerator** decides, then transmits afresh:

```
  in:   nominal 5 V, arrived at 3.2 V with 0.4 V of noise
        → decide: closer to 5 than to 0 → it was a 1
        → transmit: a clean 5.0 V
```

The output is **not a better copy of the input. It is a new signal**, indistinguishable
from what the original transmitter sent. Chain ten thousand regenerators and the
last one emits exactly what the first one did, provided every individual decision
was correct.

That proviso is the whole of the rest of Unit II. The decisions are correct if the
noise never exceeds half the gap between levels, which is a condition you can
compute — Chapter 4 §4.4 computed it — and engineer against.

## The consequence, stated plainly

**Digital transmission converts a gradual degradation into a threshold.**

Analog quality falls off smoothly with distance: a bit worse at 100 km, noticeably
worse at 500, unusable at 2,000, with no clear line anywhere. Digital transmission
is perfect, perfect, perfect, and then — past the point where noise exceeds the
decision threshold — catastrophically broken.

This is why a digital television picture is flawless or absent rather than snowy,
why a marginal Ethernet link works entirely or drops frames rather than delivering
slightly damaged ones, and why the diagnostic techniques in Unit XIII look for
error *counters* rather than for quality measurements. There is no "quality" to
measure. There are correct decisions and incorrect ones.

It is also the reason Chapter 12 identifies digitisation as the largest
improvement in the telephone network's history, and why Chapter 6's impairments
matter in a very specific way: they do not degrade your signal a little, they eat
into the margin between where you are and where the decisions start going wrong.

## Two things this does not mean

**Digital is not automatically better.** It costs bandwidth. Chapter 4 §4.2's
Nyquist limit says a channel of bandwidth *B* carries 2*B* symbols per second;
representing an analog waveform digitally at adequate fidelity requires many
symbols per waveform cycle. The telephone network spends 64 kb/s to carry a 3.4 kHz
voice signal that the analog local loop carried directly. That is a large
multiplier, and it was worth paying only once the digital electronics became cheap
enough.

**Digital does not mean square.** A 10 Gb/s signal on a real backplane, viewed on
a fast oscilloscope, does not look square at all — it looks like a blur of
overlapping ramps, and the receiver's job is to sample it at exactly the right
instant. Chapter 6 §6.3's eye diagram is the standard way of looking at this, and
the "eye" it refers to is the region in the middle where a decision can still be
made confidently. As impairments accumulate, the eye closes, and when it closes
entirely the link stops working — abruptly, per the threshold behaviour above.

> **Network+ note.** N10-009 does not ask for this distinction directly. It does
> expect you to know that fibre and modern copper carry digital signalling and
> that repeaters regenerate rather than amplify, and it expects you to reason
> about why a link is either working or not rather than partially working. The
> underlying reason is here.

## Where the boundary is crossed

Two conversions matter, and both introduce errors of their own.

**Analog to digital** requires sampling (at what rate? Chapter 4 §4.2 answers) and
quantising (to how many levels?). Quantisation introduces **quantisation error** —
the difference between the true value and the nearest representable one — which is
noise that we manufacture ourselves, as Chapter 1 §1.2 noted. It is bounded, which
is its great virtue: it never gets worse with distance, because after the first
conversion the signal is digital and regenerable.

**Digital to analog** is the reverse and happens at every playback device.

The telephone network's choice — 8,000 samples per second at 8 bits per sample —
was a judgement about how much quantisation error is tolerable for speech, and it
produced the 64 kb/s DS0 that Chapter 12 traces through the entire digital
hierarchy. Different judgements produce different numbers: a music CD samples at
44.1 kHz with 16 bits because the ear is far more discriminating about music than
about speech, and a modern voice codec uses far fewer bits than 64 kb/s because
fifty years of research went into modelling what the ear does not notice.
