# Chapter 8 — Further Reading

## Primary sources

**Chang, R. W. (1966). "Synthesis of Band-Limited Orthogonal Signals for
Multichannel Data Transmission." *Bell System Technical Journal* 45(10):
1775–1796.** (And US Patent 3,488,445.)
OFDM, twenty-nine years before 802.11a. Worth seeing how completely the idea was
described before it was implementable.

**Weinstein, S. B. & Ebert, P. M. (1971). "Data Transmission by
Frequency-Division Multiplexing Using the Discrete Fourier Transform."
*IEEE Transactions on Communication Technology* 19(5): 628–634.**
The paper that made Chang's idea practical by identifying the DFT implementation.
The link from Chapter 5's FFT to this chapter's OFDM runs through here.

**Armstrong, E. H. (1936). "A Method of Reducing Disturbances in Radio Signaling by
a System of Frequency Modulation." *Proceedings of the IRE* 24(5): 689–740.**
The FM paper. Long, and the experimental sections are a model of demonstrating
something against a consensus that says it cannot work.

**IEEE 802.11-2020, Clause 17 (OFDM PHY) and Clause 27 (HE PHY).**
The specifications of the OFDM parameters quoted in §8.4 — subcarrier count,
spacing, symbol duration, guard intervals, and the modulation and coding scheme
tables that §8.3's ladder summarises. Clause 27's MCS table is worth looking at
directly: it is the rate ladder, in the standard's own words.

## Books

**Sklar, B. & Harris, F. J. (2020). *Digital Communications: Fundamentals and
Applications*, 3rd ed. Pearson.**
Chapters 4 and 9 cover bandpass modulation and modulation/coding tradeoffs. The
best accessible treatment of why phase is the most useful parameter, with the
error-probability derivations that §8.3's SNR table summarises.

**Proakis, J. G. & Salehi, M. (2007). *Digital Communications*, 5th ed.
McGraw-Hill.**
The standard graduate reference. Chapter 4 on modulation and Chapter 13 on
multichannel and multicarrier systems. Demanding and complete.

**Bahai, A. R. S., Saltzberg, B. R. & Ergen, M. (2004). *Multi-Carrier Digital
Communications: Theory and Applications of OFDM*, 2nd ed. Springer.**
The dedicated OFDM book. Chapters 2–4 cover orthogonality, the cyclic prefix and
the FFT implementation; chapter 5 covers the peak-to-average problem honestly. Go
here when §8.4 is insufficient.

**Rappaport, T. S. (2024). *Wireless Communications: Principles and Practice*,
3rd ed. Cambridge.**
The standard wireless text. Chapters 5 and 6 on propagation and multipath supply
the delay-spread measurements that §8.4's table quotes, with the measurement
methodology rather than just the numbers.

**Lathi, B. P. & Ding, Z. (2018). *Modern Digital and Analog Communication
Systems*, 5th ed. Oxford.**
More approachable than Proakis and unusually good on the quadrature representation.
Chapter 4's development of §8.2's identity is worth reading if the algebra there
felt like a trick rather than a decomposition.

## Applied and practitioner

**Keysight, Rohde & Schwarz, and Anritsu application notes on EVM and constellation
analysis.**
Freely available, vendor-written, and genuinely excellent on §8.2's diagnostic
table — what each constellation distortion looks like and what causes it. Written
for people holding an instrument, which makes them more concrete than any textbook.
Search for "EVM measurement" or "constellation diagram troubleshooting".

**CableLabs DOCSIS 3.1 Physical Layer Specification.**
The constellation and OFDM parameters for cable, and a good example of the same
techniques applied in a completely different medium. Chapter 49 §49.2 uses it.

**3GPP TS 38.211 (5G NR Physical channels and modulation).**
Free. §5 covers the modulation mapping including the exact constellation
definitions, and §4 covers the OFDM numerology — subcarrier spacing options,
cyclic prefix lengths — which shows how the tradeoffs of §8.4 were parameterised
rather than fixed.

## Interactive and hands-on

**GNU Radio, with any inexpensive SDR dongle.**
The single best investment for making this chapter concrete. Build a QPSK
transmitter and receiver in an afternoon; watch the constellation form; introduce
a frequency offset and watch it rotate; add noise and watch it smear. Everything in
§8.2's diagnostic table can be reproduced deliberately, which is a different kind
of understanding from reading about it.

**DSP Illustrations and similar interactive OFDM demonstrations.**
Several exist online. The ones that let you set delay spread and guard interval
independently and watch the error rate respond make §8.4's cyclic-prefix argument
unmissable.

**`perfcalc.py shannon` and `perfcalc.py db`** in this book's
[tools/](../../../tools/) directory — for checking §8.3's SNR ladder against the
capacity bound, and for the dB conversions throughout.

## For the certification-minded

N10-009 does not examine modulation schemes by name, and three of this chapter's
consequences are directly examinable:

- **Wireless data rates vary with conditions** and the advertised maximum requires
  ideal ones (objectives 2.3, 5.5). §8.3's ladder is the mechanism.
- **OFDMA is a Wi-Fi 6 feature** and its benefit is in dense environments
  (objective 2.3). §8.4 explains why, which is more useful than the fact.
- **Channel width affects throughput** — more subcarriers, more capacity, and more
  noise admitted (Chapter 4 §4.4 and Chapter 43 §43.3).

The single most transferable idea for practical work is §8.3's: **SNR determines
which modulation a radio can use, and modulation determines the rate.** Signal
strength alone determines neither, which is why Chapter 45's troubleshooting
procedure asks about noise before it asks about signal.
