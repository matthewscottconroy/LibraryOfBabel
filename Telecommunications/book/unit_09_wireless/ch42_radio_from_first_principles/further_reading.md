# Chapter 42 — Further Reading

## Foundations

**Maxwell, J. C. (1865). "A Dynamical Theory of the Electromagnetic Field."**
The original. Heavy going in its own notation, and worth looking at once to see that the
prediction genuinely preceded the observation.

**Feynman, R. *The Feynman Lectures on Physics*, Volume II.**
**Freely available at feynmanlectures.caltech.edu.** Chapters 18–21 on Maxwell's equations
and radiation. **The best physical explanation of what an electromagnetic wave *is* that
exists**, and it requires less mathematics than its reputation suggests.

**Hertz, H. (1893). *Electric Waves*.**
The experiments, described by the person who did them. Historically remarkable and
surprisingly readable.

## Radio engineering

**Balanis, C. (2016). *Antenna Theory: Analysis and Design*, 4th ed. Wiley.**
**The reference for antennas.** Mathematical, thorough, and the standard graduate text. Read
chapter 2 for the fundamental parameters — gain, beamwidth, polarisation — if nothing else.

**Rappaport, T. (2001). *Wireless Communications: Principles and Practice*, 2nd ed.**
**The standard text for propagation**, and the source of much of §42.3 and §42.4. The
multipath and fading chapters are the clearest available treatment.

**The ARRL Antenna Book** and **The ARRL Handbook.**
Written for amateur radio operators and consequently **practical rather than mathematical**.
If you will actually build or install antennas, these explain what the textbooks assume you
know. Updated annually.

**Molisch, A. (2010). *Wireless Communications*, 2nd ed.**
More modern than Rappaport on MIMO and OFDM specifically.

## MIMO and the modern physical layer

**Foschini, G. J. & Gans, M. J. (1998). "On Limits of Wireless Communications in a Fading
Environment When Using Multiple Antennas." *Wireless Personal Communications*.**
**The capacity result**, and it is worth reading the abstract at minimum to appreciate how
surprising the claim was: **capacity scaling linearly with antenna count.**

**Telatar, E. (1999). "Capacity of Multi-Antenna Gaussian Channels."**
The independent derivation, more mathematically direct.

**Tse, D. & Viswanath, P. (2005). *Fundamentals of Wireless Communication.***
**Freely available.** The rigorous treatment of MIMO, diversity and multiplexing, including
**the diversity–multiplexing trade-off** that determines how many streams a channel actually
supports.

## Practical

**Coleman, D. & Westcott, D. — *CWNA Certified Wireless Network Administrator Study
Guide.***
**The best practical wireless book for a network professional.** Covers everything in this
unit at the depth an engineer needs rather than a researcher, with the RF fundamentals done
properly. **If you read one book alongside Unit IX, this is it.**

**Bardwell, J. — the WLAN analysis papers.**
His work on what RSSI actually measures, on why vendor readings differ, and on interpreting
survey data is the antidote to a great deal of wireless folklore.

## Applied

**Get a Wi-Fi analyser and walk around.** On Android, **WiFiAnalyzer** (open source); on
macOS, the built-in **Wireless Diagnostics** (hold Option and click the Wi-Fi menu); on
Windows, **WiFi Explorer** or `netsh wlan show interfaces`.

**Exercise F1 is the one to do:** **record RSSI while walking slowly, then standing still.**
The variance while stationary is the fast fading of §42.4, and seeing it once ends any
belief that a single measurement characterises a location.

**Measure a wall.** Take a reading, walk through a doorway, take another at the same
distance. **The difference is the attenuation**, and comparing several wall types against
§42.1's table takes twenty minutes and is more convincing than the table.

**Rotate your phone** and watch RSSI change. That is polarisation (§42.2).

**[tools/perfcalc.py](../../../tools/perfcalc.py) `linkbudget`:**

```bash
python3 tools/perfcalc.py linkbudget \
    --freq 5800 --distance 3 \
    --tx-power 23 --tx-gain 16 --tx-loss 1.5 \
    --rx-gain 16 --rx-loss 1.5 --obstruction 4 \
    --sensitivity -82
```

**It reports the budget, the margin with a verdict, and the Fresnel clearance** — and
**working one by hand and checking against it** is the fastest way to become confident with
the arithmetic.

**Also `perfcalc.py shannon` and `perfcalc.py noise`** for the SNR-to-capacity relationship
of §42.1.

**A spectrum analyser**, if you can get one. **A cheap SDR — an RTL-SDR dongle is around
£25 — plus `gqrx` or `SDR#`** lets you *see* the spectrum, which changes how you think about
it. **Chapter 43 §43.4's noise floor stops being an abstraction the first time you watch a
microwave oven appear on a waterfall display.**

**Ekahau, Hamina, or NetSpot** for professional site surveys — expensive, and **NetSpot has
a usable free tier** for small spaces.

**Lab 31** in this book's [labs/](../../../labs/) directory measures wall attenuation,
records a fading profile while walking, demonstrates polarisation loss, and works a link
budget for a real path — then compares the prediction with the measurement and requires the
discrepancy to be accounted for.

## For the certification-minded

Objective 2.4 expects wireless standards and RF fundamentals; objective 5.4 expects
interference and coverage troubleshooting.

Eight things worth over-learning:

1. **Higher frequency = more bandwidth, shorter range, worse penetration.**
2. **2.4 GHz penetrates better; 5 GHz is faster and shorter-range.**
3. **RSSI in dBm, and −67 dBm is the voice design target.**
4. **SNR matters more than RSSI**, and 20 dB is the practical minimum.
5. **Antenna types**: omnidirectional for coverage, directional (patch, Yagi, parabolic)
   for point-to-point.
6. **Gain is directionality**, measured in dBi.
7. **EIRP = transmit power − loss + antenna gain**, and it is what is regulated.
8. **Multipath causes fading**, and **MIMO uses it** for spatial streams.

And the three practical facts worth more than the objective:

**Strong signal with poor performance means the noise floor**, not the signal.

**Turning transmit power up creates an asymmetric link.** Clients cannot answer. Add access
points or better antennas instead.

**Line of sight is not enough** — 60% of the first Fresnel zone must be clear, which at 1 km
is 3.4 metres.
