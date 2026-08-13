# Chapter 6 — Impairments

Every chapter so far has assumed a channel that behaves. This one is the
catalogue of misbehaviour.

There is a useful way to think about the material here, and it is this: the world
has exactly four things it can do to your signal, and every physical-layer fault
you will ever diagnose is one of them or a combination.

1. It can make the signal **smaller** — attenuation.
2. It can **add** something to it — noise.
3. It can **reshape** it — distortion and dispersion.
4. It can let **someone else's signal** into it — crosstalk and interference.

That taxonomy is not merely tidy. It is diagnostic, because each of the four has a
different relationship to distance, frequency, temperature, and load, and those
relationships are how you tell them apart from a set of measurements.

## The scene

In 1962 a team at Bell Labs published the results of an exhaustive survey of the
American telephone plant, measuring what actually happened to signals on ordinary
subscriber loops. It is not a glamorous document. It is a list of impairments with
distributions attached: how much attenuation, how much noise, how much phase
distortion, how much impulse noise, on how many lines.

That document, and its successors, is why modems worked. Every modem standard from
Bell 103 in 1962 to V.90 in 1998 was designed against measured impairment
statistics rather than against an idealised channel, and the reason V.34 could
achieve 33.6 kb/s on a network built for voice is that its designers knew, to a
distribution, exactly what the channel would do to them.

The general lesson, which recurs in Chapter 45 when we survey a building for Wi-Fi
and in Chapter 66 when we read interface counters: **the channel you have is not
the channel in the specification, and the difference is measurable.**

## What each impairment does, in one line each

**Attenuation** — the signal loses power with distance. On copper it is worse at
high frequencies, which means it does not merely shrink the signal but reshapes
it. Measured in dB, and it is the reason every medium has a maximum length.

**Noise** — energy added by the world and by our own equipment. Thermal noise sets
the floor (Chapter 4); impulse noise from motors, switches and lightning is the
one that actually breaks copper links, because it is large, brief, and completely
unpredictable.

**Distortion** — the channel treats different frequencies differently, so the
signal's shape changes even without anything being added. Amplitude distortion
(some frequencies attenuated more) and delay distortion (some frequencies arriving
later) are the two forms, and the second is the one that produces intersymbol
interference and killed the 1858 Atlantic cable.

**Dispersion** — the optical version of delay distortion. Modal dispersion (rays
taking different paths through a multimode fibre) and chromatic dispersion
(different wavelengths travelling at different speeds) are why multimode fibre is
short-reach and why long-haul systems need dispersion compensation.

**Crosstalk** — a neighbouring conductor's signal coupling into yours. NEXT, FEXT,
and alien crosstalk are the measured forms, and defeating crosstalk is the entire
reason twisted pair is twisted — a fact usually stated and rarely explained.

**Interference** — someone else's transmitter, in the same band, at the same time.
The dominant impairment in every unlicensed wireless deployment, and the subject of
Chapter 43.

## Why this chapter is worth real attention

Because these are the faults that lie.

A failed cable is easy: no link light, and any technician finds it in a minute. A
*marginal* cable is the expensive one. It passes a continuity test. It works at
1 Gb/s and fails at 10. It works in the morning and fails in the afternoon when the
sun heats the riser. It produces 0.3% packet loss, which Chapter 3 told you is
enough to cap a TCP stream at a fraction of its capacity, while every dashboard in
the building shows a healthy green link at full speed.

The only way to find these is to know what to measure and what the measurement
means, and that is what this chapter supplies. When Chapter 66 tells you to read
the interface error counters — CRC errors, runts, giants, late collisions,
input errors — every one of those counters is a fingerprint of one of the six
impairments above, and knowing which is which is the difference between a
diagnosis and a cable swap.

## What this chapter does

§6.1 covers attenuation: the dB budget, frequency dependence, the skin effect, and
how maximum cable lengths are actually derived.

§6.2 covers noise in its several forms, extending Chapter 4's thermal floor to the
noise sources that dominate in practice.

§6.3 covers distortion and dispersion, deriving intersymbol interference properly
and explaining eye diagrams — the single most informative measurement in
high-speed signalling.

§6.4 covers crosstalk and interference, including why twisting works, what NEXT
and FEXT measure, and where EMI comes from in a real building.

## By the end you will be able to

- Compute a loss budget in dB and determine whether a proposed link will work.
- Explain why attenuation is frequency-dependent on copper and what that implies
  for the cable categories of Chapter 10.
- Distinguish the six impairments by their signature — how each varies with
  distance, frequency, temperature and load.
- Read an eye diagram and say what impairment is closing it.
- Explain, physically, why twisting a pair rejects crosstalk, and why the twist
  rates differ between pairs in the same jacket.
- Map each impairment onto the interface counter that reveals it.
