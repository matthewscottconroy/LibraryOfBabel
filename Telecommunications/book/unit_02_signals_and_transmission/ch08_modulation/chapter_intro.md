# Chapter 8 — Modulation

On the evening of 12 December 1901, in a disused hospital on Signal Hill in St
John's, Newfoundland, Guglielmo Marconi and his assistant George Kemp listened
through a telephone earpiece attached to a kite-borne aerial and heard — or
believed they heard — three faint clicks. Three clicks is the letter `S` in Morse.
It had been transmitted from Poldhu, in Cornwall, 3,400 kilometres away.

The claim was disputed at the time and has been disputed ever since. Marconi kept
no recording; the receiver was a coherer of notoriously erratic behaviour; the
frequency used was around 850 kHz, which should not have propagated that far in
daylight over that path; and Marconi had every commercial incentive to hear
something. Careful modern reconstructions suggest it was possible but marginal.
What is not disputed is that within a few years the technique unquestionably
worked, that Marconi received the Nobel Prize in 1909, and that the entire
subsequent century of wireless communication rests on the thing his apparatus
did — which was to take information and impress it upon a high-frequency carrier
wave.

That operation is **modulation**, and this chapter is about it.

## Why a carrier at all

Two reasons, and they are independent, which is worth separating because people
usually conflate them.

**The antenna reason.** An efficient antenna must be a substantial fraction of a
wavelength — a quarter wave is the usual compromise. The wavelength of a signal at
frequency *f* is λ = *c*/*f*. A baseband voice signal at 3 kHz has a wavelength of
100 kilometres, so a quarter-wave antenna would be 25 km tall, which is not
practical. Move that same voice onto a 900 MHz carrier and the wavelength is 33 cm,
so a quarter-wave antenna is 8.3 cm — the length of the whip on a car roof, and,
not coincidentally, roughly the length of the antenna hidden in the edge of your
laptop screen. **You modulate onto a carrier because you cannot build an antenna
for baseband.**

**The sharing reason.** Chapter 9's frequency-division multiplexing requires that
different conversations occupy different frequency bands. A baseband signal
occupies the band from DC upward and cannot be moved. Modulation shifts a signal's
spectrum to wherever you want it, which is what makes it possible for a thousand
radio stations, forty television channels, and every mobile phone in a city to
coexist.

Neither reason applies to a wire between two switches, which is precisely why wired
Ethernet uses baseband line coding (Chapter 7) and radio uses modulation. The two
chapters are the two halves of one question, answered differently because the
constraints differ.

## Three things you can vary

A sinusoidal carrier is fully described by three parameters:

$$s(t) = A \cos(2\pi f t + \phi)$$

Its **amplitude** *A*, its **frequency** *f*, and its **phase** *φ*. Those are the
only three, and therefore there are exactly three primitive things you can modulate.
Vary the amplitude and you have amplitude-shift keying; vary the frequency, FSK;
vary the phase, PSK. Every modulation scheme in existence — including the
extraordinarily sophisticated ones in 5G and Wi-Fi 7 — is a combination of these
three, plus the decision to use several carriers at once.

The chapter's argument builds in that order: the three primitives, then the
combination of amplitude and phase that gives QAM, then the use of many carriers
simultaneously that gives OFDM.

## Where this connects to Chapter 4

Directly and quantitatively. Chapter 4 said data rate = symbol rate × log₂ *M*,
and that the achievable *M* is limited by SNR. Modulation is where *M* is chosen,
and a **constellation diagram** is the picture of that choice.

A QPSK constellation has 4 points and carries 2 bits per symbol. 16-QAM has 16
points, 4 bits. 256-QAM has 256 points, 8 bits. 4096-QAM — in Wi-Fi 7 — has 4,096
points and carries 12 bits per symbol, which requires an SNR in the region of
42 dB, which in turn requires you to be sitting more or less on top of the access
point in a quiet radio environment.

That last sentence is the practical content of this chapter. When your phone shows
full signal and delivers 900 Mb/s in one room and 80 Mb/s in another, the radio has
walked down the constellation ladder — 4096-QAM to 1024-QAM to 256-QAM to 64-QAM to
QPSK — trading bits per symbol for noise immunity, exactly as Chapter 4's capacity
curve requires. **Rate adaptation is a constellation choice, made forty times a
second.**

## What this chapter does

§8.1 develops the three primitive keying schemes, with their waveforms, their
spectra, and their relative robustness — including why FSK dominated early modems
and why nothing modern uses ASK alone.

§8.2 introduces phase and the quadrature representation: the I/Q plane, why two
carriers at 90° are independent, and how every modern transmitter is built from
this decomposition.

§8.3 covers QAM and constellation diagrams properly: bits per symbol, minimum
distance, the required SNR for a given error rate, and how to read a measured
constellation to diagnose a fault — a skill directly useful in cable and satellite
work.

§8.4 covers OFDM: why splitting one fast channel into hundreds of slow subcarriers
defeats multipath, what a cyclic prefix is, and why every modern high-rate radio
system — Wi-Fi, LTE, 5G, DVB, DOCSIS 3.1 — converged on it.

## By the end you will be able to

- Explain why a carrier is necessary for radio, with the antenna arithmetic.
- Sketch and identify ASK, FSK and PSK waveforms.
- Read a constellation diagram: count its bits per symbol, and identify phase
  noise, amplitude compression, and interference from its distortions.
- Compute the data rate of a modulation scheme from symbol rate and constellation
  size, and estimate the SNR required.
- Explain what OFDM does to multipath and why it enabled high-rate indoor wireless.
