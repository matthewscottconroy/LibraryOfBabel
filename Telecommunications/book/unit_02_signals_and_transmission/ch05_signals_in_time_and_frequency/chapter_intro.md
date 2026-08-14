# Chapter 5 — Signals in Time and Frequency

In 1807, Joseph Fourier submitted a paper to the Institut de France claiming that
any periodic function — including ones with corners, jumps, and discontinuities —
could be written as a sum of smooth sine waves. Lagrange, who was on the
committee, objected that this was obviously false, and the paper was rejected. It
took Fourier fifteen years and a rewritten treatise, *Théorie analytique de la
chaleur* (1822), to get the idea into print, and another century for mathematicians
to work out the precise conditions under which Lagrange's objection was and was
not correct.

Lagrange was right about a technical detail and wrong about everything that
mattered. Fourier's claim is essentially true, and it is the most useful
idea in signal processing, because it provides a second way of looking at any
signal — and the second view explains things that the first view merely displays.

## Two views of the same thing

Put an oscilloscope on a wire and you see a **time-domain** view: voltage against
time, the shape of the signal as it happens. This is intuitive. It is what a
signal "looks like."

Put a spectrum analyser on the same wire and you see a **frequency-domain** view:
how much energy the signal contains at each frequency. This is unintuitive on
first contact and is where every answer lives.

The two views contain identical information — the Fourier transform converts
between them losslessly, in both directions — but they make different facts
obvious. And nearly every fact a network engineer needs is obvious only in
frequency.

Consider: *why does a square wave get rounded off by a long cable?* In the time
domain this is a mysterious smearing. In the frequency domain it is trivial. A
square wave is a sum of a fundamental sine plus odd harmonics at 3×, 5×, 7× the
frequency and decreasing amplitude. The sharp corners *are* the high harmonics.
A cable attenuates high frequencies more than low ones. Remove the high harmonics
and you remove the corners. The rounding is not a mystery; it is a subtraction,
and you can predict exactly how much of it a given cable will produce.

That is the pattern for the whole chapter, and largely for the whole unit. **A
channel is characterised by what it does to each frequency.** Attenuation,
distortion, dispersion, filtering, bandwidth — all of them are statements about
frequency response, and all of them become simple arithmetic once you are looking
at the right picture.

## Analog and digital are not what people think

The chapter also has a definitional job to do, because "analog versus digital" is
one of the phrases the field uses most and defines least well.

The common belief is that analog signals are smooth waves and digital signals are
square. This is wrong, and believing it makes several later topics incoherent — in
particular it makes it impossible to understand what a modem does, or why a
digital signal on a real wire is a continuous voltage that only *represents*
discrete values.

§5.1 fixes this. The distinction is not about the shape of the waveform. It is
about whether the *set of meaningful values* is continuous or discrete, and that
is a property of the agreement (Chapter 2, §2.4), not of the physics. Every signal
on every wire in this book is a continuous physical quantity. Some of them are
interpreted as belonging to a discrete set. That is the whole difference, and it
is the reason a digital signal can be regenerated perfectly and an analog one
cannot.

## What this chapter does

§5.1 draws the analog/digital distinction properly and derives the enormous
practical consequence: regeneration versus amplification, and why the digital
telephone network of Chapter 12 could span a continent when the analog one could
not.

§5.2 develops the frequency domain: Fourier's decomposition, spectra, the square
wave worked example, and what a filter is.

§5.3 defines the **bandwidth of a channel** carefully — the 3 dB convention, why
it is where it is — and connects it back to Chapter 4's *B*.

§5.4 distinguishes **baseband** from **broadband** signalling, which is the
distinction between putting your signal directly on the wire and putting it on a
carrier, and which determines whether a medium can be shared by frequency.

## By the end you will be able to

- State what actually distinguishes an analog from a digital signal, and explain
  why the distinction permits regeneration.
- Sketch the frequency spectrum of a square wave and predict qualitatively what a
  band-limited channel does to it.
- Explain why a cable's bandwidth limits the bit rate it can carry, in terms of
  harmonics rather than by assertion.
- Define the 3 dB bandwidth and read it off a frequency-response curve.
- Distinguish baseband from broadband transmission and identify which one a given
  technology uses.

## Where this sits in the argument

Chapter 4 gave us *B* as a number in a formula. This chapter tells us what *B*
physically is, where it comes from, and what happens at its edges. Chapter 6 then
catalogues the ways a real channel departs from the ideal one assumed so far.
