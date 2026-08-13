# 5.4 Baseband and Broadband

Two words that the industry uses inconsistently, one of them badly, and the
confusion is worth clearing because the underlying distinction determines whether
a medium can be shared by frequency.

## The engineering definitions

**Baseband** signalling puts the data directly onto the medium, occupying the band
from DC (or near it) upward. The signal is not shifted in frequency; what you see
on the wire is the data, rendered as voltage or light or current.

**Broadband** signalling — in its original, correct sense — modulates the data onto
a carrier, shifting its spectrum to occupy a band somewhere other than at DC.
Multiple such signals can then coexist on one medium at different carrier
frequencies, which is frequency-division multiplexing (Chapter 9 §9.1).

Note that the distinction is about **where in the spectrum the signal sits**, not
about how fast it goes. This is exactly where the colloquial usage went wrong.

## The colloquial usage, and why it is wrong

Since roughly 1998, "broadband" in ordinary speech has meant "fast Internet
access, as opposed to dial-up". The FCC and other regulators have formalised this
with numerical thresholds that have been revised upward repeatedly.

That usage is now universal and this book does not fight it. But be aware that it
has nothing to do with the engineering sense, and that the two collide
occasionally:

- A **1 Gb/s Ethernet link is baseband** — the `BASE` in `1000BASE-T` says so
  explicitly — and it is unambiguously fast. Baseband does not mean slow.
- A **56 kb/s voiceband modem is broadband** in the engineering sense, since it
  modulates onto a carrier in the 300–3,400 Hz band, and it is unambiguously slow.
- A **DOCSIS cable modem is broadband in both senses**, which is a coincidence.

Ethernet's naming convention preserves the engineering sense and is worth reading
carefully. In `10BASE5`, `100BASE-TX`, `1000BASE-T`, `10GBASE-SR`, the `BASE`
means baseband. There was also `10BROAD36` — 10 Mb/s Ethernet modulated onto a
carrier for 3,600 m over CATV coax, standardised in 1985 and essentially never
deployed. Its existence is why the `BASE` is there at all: it distinguished a
variant that mattered from one that did not.

## Why the distinction determines sharing

Here is the practical consequence, and it is the reason this section exists.

**A baseband signal occupies the spectrum from DC upward and cannot be moved.**
If two baseband transmitters share a medium, they occupy the same band and
interfere. Sharing must therefore be arranged in **time** — one at a time, by
turns — which is what CSMA/CD (Chapter 16) and CSMA/CA (Chapter 44) do, and which
is why a shared baseband medium requires a medium access control protocol at all.

**A broadband signal can be placed anywhere in the spectrum.** Several can
coexist simultaneously on one medium at different frequencies, each with its own
receiver tuned to its own band, none of them aware of the others.

That is the entire architectural difference between a cable television plant
carrying two hundred channels simultaneously and an Ethernet segment where one
station transmits at a time.

## The four cases, tabulated

| Medium | Baseband or broadband | Sharing mechanism |
|---|---|---|
| Ethernet over copper or fibre | Baseband | Time (or, in switched networks, not shared at all) |
| Cable television and DOCSIS | Broadband | Frequency, plus time within each channel |
| DSL | Broadband | Frequency — voice below 4 kHz, data above |
| Wi-Fi and all radio | Broadband, necessarily | Frequency (channels) plus time (CSMA/CA) |
| DWDM on fibre | Broadband, at optical frequencies | Wavelength |

Radio is "necessarily" broadband for the antenna reason developed in Chapter 8
§8.1: you cannot build an antenna for a baseband signal whose wavelength is
kilometres. Every radio system in this book modulates, without exception, and the
choice of carrier frequency is one of its defining parameters.

## The DSL case, worked

DSL is the cleanest illustration, and it previews Chapter 49 §49.1.

A telephone local loop was carrying baseband voice — well, band-limited voice from
300 Hz to 3,400 Hz, which is not quite baseband but is close enough for the
argument. The copper itself passes megahertz.

ADSL leaves the voice band alone and puts data **above** it: upstream roughly
25–138 kHz, downstream roughly 138 kHz to 1.1 MHz, divided into 4.3125 kHz
subcarriers each independently modulated. The **splitter** at the customer premises
is a pair of filters — a low-pass to the telephone, a high-pass to the modem — and
it is the entire integration cost.

Two properties fall straight out of the frequency-domain view:

**Voice and data coexist without any coordination**, because they occupy disjoint
bands. Neither protocol knows the other exists.

**Attenuation rises with frequency**, so the highest subcarriers die first as loop
length increases. This is why DSL rate falls with distance in the smooth, continuous
way that Chapter 49's table records — the modem measures each subcarrier's SNR
individually and loads bits onto it accordingly, so a long loop simply carries fewer
bits on fewer usable subcarriers. It is a per-frequency application of Shannon's
formula, executed hundreds of times per line, and it is a genuinely elegant piece of
engineering.

## What breaks here

**Assuming "broadband" on a datasheet means the engineering sense.** Read the
context. If it appears next to a data rate in Mb/s, it means the colloquialism.

**Assuming a baseband medium can be shared by frequency.** It cannot, and every
attempt to do so on Ethernet has failed commercially — `10BROAD36` being the
instance.

**Forgetting that radio must modulate.** Students occasionally propose sending
data "directly" over radio without a carrier. The antenna arithmetic in Chapter 8
§8.1 shows why this is not an oversight in the standards.

> **Network+ note.** N10-009 uses "baseband" and "broadband" in the engineering
> sense in the media objectives, and in the colloquial sense in the WAN objectives.
> Both appear. The distinguishing question to ask when you meet either word on the
> exam: is this about *how the signal occupies spectrum*, or about *how fast the
> customer's Internet is*?
