# 4.3 Noise, SNR, and Decibels

Before we can state Shannon's theorem we need its second ingredient, and we need
the notation the entire physical layer is written in.

## Why decibels

Signal powers in telecommunications span an absurd range. A transmitter might emit
1 watt; a receiver might successfully decode a signal of 10⁻¹³ watts. That is a
ratio of ten trillion to one, and writing such numbers in linear form is miserable.

Worse, the operations you actually perform on them are *multiplications*. A signal
loses half its power in a connector, then a tenth in a length of cable, then is
amplified twentyfold. Chaining multiplications by hand is error-prone.

The decibel solves both problems by taking logarithms, turning multiplication into
addition and compressing the range.

$$\text{dB} = 10 \log_{10} \frac{P_1}{P_2}$$

Note the 10, and note that this is for **power** ratios. For **amplitude** or
voltage ratios the coefficient is 20, because power goes as the square of
amplitude and log(x²) = 2 log(x). Mixing the two up is the most common error in
this area and produces answers wrong by a factor of two in dB, which is a factor
of a hundred in linear terms.

## The table to know by heart

| Ratio | dB | | Ratio | dB |
|---|---|---|---|---|
| 1 | 0 | | 1/2 | −3 |
| 2 | +3 | | 1/4 | −6 |
| 4 | +6 | | 1/10 | −10 |
| 8 | +9 | | 1/100 | −20 |
| 10 | +10 | | 1/1000 | −30 |
| 100 | +20 | | | |
| 1000 | +30 | | | |

Two facts carry most of the practical work:

- **+3 dB is double. −3 dB is half.**
- **+10 dB is ten times. −10 dB is one tenth.**

Everything else is composition. 26 dB = 20 + 3 + 3 = 100 × 2 × 2 = 400×. A 16 dB
loss is 10 + 3 + 3, so 1/10 × 1/2 × 1/2 = 1/40, i.e. 2.5% of the original power
survives. You can do these in your head within a week of practice, and once you
can, every optical power budget and every wireless link budget becomes mental
arithmetic.

## dBm: absolute power

A decibel is a *ratio* and therefore dimensionless. To express absolute power we
need a reference, and the near-universal one in telecommunications is one
milliwatt:

$$\text{dBm} = 10 \log_{10} \frac{P}{1 \ \text{mW}}$$

| Power | dBm |
|---|---|
| 1 W | +30 |
| 100 mW | +20 |
| 20 mW | +13 |
| 1 mW | 0 |
| 1 µW | −30 |
| 1 nW | −60 |
| 1 pW | −90 |

Useful anchors: a Wi-Fi client transmits at roughly +15 to +20 dBm. A usable Wi-Fi
receive level is around −65 dBm; −80 dBm is marginal; −90 dBm is generally below
the noise. A single-mode fibre transceiver might launch at −3 dBm and have a
receiver sensitivity of −23 dBm, giving a 20 dB budget to spend on distance,
splices and connectors.

The arithmetic that makes dBm worth using: **dBm + dB = dBm.** Transmit at
+20 dBm, lose 60 dB in the path, gain 3 dBi from the receive antenna, and you have
+20 − 60 + 3 = **−37 dBm** at the receiver. One line, no multiplication. That is a
complete link budget and §4.4 and Chapter 42 do nothing more complicated.

## Where noise comes from

**Thermal (Johnson–Nyquist) noise** is the floor beneath everything. The random
thermal motion of charge carriers in any conductor above absolute zero generates a
fluctuating voltage. It was measured by John B. Johnson at Bell Labs in 1926 and
explained theoretically by Harry Nyquist the following year. Its power is

$$N = kTB$$

where *k* is Boltzmann's constant (1.380649 × 10⁻²³ J/K), *T* is absolute
temperature in kelvins, and *B* is bandwidth in hertz.

Three consequences, each important:

- **Noise power is proportional to bandwidth.** Doubling the bandwidth doubles the
  noise admitted. A wider Wi-Fi channel carries more data *and* hears more noise,
  and §4.4 shows how those trade against one another.
- **Noise depends on temperature.** This is why radio telescopes and deep-space
  receivers use cryogenically cooled front ends, and why a hot equipment cabinet
  performs measurably worse than a cool one.
- **You cannot design it away.** It is thermodynamics, not engineering.

**Computing the floor.** At room temperature (290 K, the standard reference),

$$kT = 1.380649 \times 10^{-23} \times 290 = 4.00 \times 10^{-21} \ \text{W/Hz}$$

In dBm per hertz:

$$10 \log_{10}\left(\frac{4.00 \times 10^{-21}}{10^{-3}}\right) = -174 \ \text{dBm/Hz}$$

**−174 dBm/Hz** is a number worth memorising. It is the thermal noise floor at
room temperature per hertz of bandwidth, and every receiver on the planet works
against it.

For a given bandwidth, add 10 log₁₀(*B*):

| Bandwidth | Noise floor |
|---|---|
| 1 Hz | −174 dBm |
| 1 kHz | −144 dBm |
| 1 MHz | −114 dBm |
| 20 MHz (Wi-Fi) | −101 dBm |
| 80 MHz (Wi-Fi) | −95 dBm |
| 160 MHz (Wi-Fi) | −92 dBm |

Real receivers do worse than this by their **noise figure** — the additional noise
the receiver's own electronics contribute, typically 4–10 dB for consumer
equipment. So a real 20 MHz Wi-Fi receiver has a noise floor near −95 dBm, which
matches what a Wi-Fi analyser reports on a quiet channel. When your analyser shows
−80 dBm of noise instead, something in the environment is radiating, and Chapter 43
goes looking for it.

**Other noise sources**, all covered in Chapter 6: shot noise (the granularity of
charge carriers, important in optical receivers), impulse noise (lightning,
switching transients, motors — the dominant killer on copper), intermodulation
(non-linear devices creating sums and differences of frequencies), and crosstalk
(a neighbouring pair's signal, which is why the pairs are twisted).

## Signal-to-noise ratio

$$\text{SNR} = \frac{P_{\text{signal}}}{P_{\text{noise}}}, \qquad \text{SNR}_{\text{dB}} = 10 \log_{10} \frac{P_{\text{signal}}}{P_{\text{noise}}}$$

Equivalently, and more usefully in practice: **SNR in dB is simply the received
signal level in dBm minus the noise floor in dBm.**

A Wi-Fi client seeing −65 dBm of signal against a −95 dBm noise floor has an SNR
of 30 dB. Walk further away, signal drops to −85 dBm, SNR falls to 10 dB, and the
radio drops to a lower modulation — fewer bits per symbol, more robust against
noise. That rate adaptation, which you experience as your connection getting
slower as you walk away, is the device tracking §4.4's capacity curve downward in
real time.

**SINR** — signal to interference *plus* noise ratio — is the honest measure in any
shared-medium environment, because a neighbouring access point on your channel is
not thermal noise but it degrades you identically. In practice, in the 2.4 GHz
band, interference usually dominates thermal noise by a wide margin, which is why
Chapter 43 spends more time on channel planning than on receiver sensitivity.

> **Network+ note.** N10-009 objective 5.5 expects familiarity with signal
> strength measurement in wireless troubleshooting, and RSSI values in dBm appear
> throughout the wireless objectives. The two things to carry: a *less negative*
> dBm is stronger (−65 is much better than −85), and signal strength alone is
> meaningless without the noise floor — SNR is what determines the achievable rate.
