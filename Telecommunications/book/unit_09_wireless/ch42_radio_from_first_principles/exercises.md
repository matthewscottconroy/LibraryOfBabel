# Chapter 42 — Exercises

## A. Recall

**A1.** What two fields make up an electromagnetic wave, and how are they oriented relative
to each other and to the direction of travel?

**A2.** State the relationship between speed, frequency and wavelength, and give the
shortcut form for λ in metres from frequency in MHz.

**A3.** Give the three consequences of raising frequency, and state the fundamental trade in
one sentence.

**A4.** What is the design-target RSSI for a WLAN carrying voice?

**A5.** Why is SNR more important than absolute signal strength? Give the minimum SNR for
reliable data.

**A6.** What does antenna gain actually do? What is the reference for dBi?

**A7.** Give the formula for EIRP and state which quantity regulators limit.

**A8.** State the free-space path loss rule of thumb for doubling distance and for doubling
frequency.

**A9.** How much of the first Fresnel zone must be clear?

**A10.** What is multipath, and name its two harmful effects and its one beneficial use.

## B. Apply

**B1.** Compute the wavelength for 433 MHz, 868 MHz, 2.4 GHz, 5.8 GHz and 24 GHz. For each,
give the half-wave dipole length.

**B2.** Compute FSPL at 2.4 GHz and at 5 GHz for 50 m, 200 m, 2 km and 8 km. State the
difference between the bands at each distance.

**B3.** Complete this link budget and give the verdict:

```
   Transmit power        23 dBm
   Transmit cable        1.5 dB loss
   Transmit antenna      16 dBi
   Distance              3 km at 5.8 GHz
   Obstruction           4 dB
   Receive antenna       16 dBi
   Receive cable         1.5 dB loss
   Receiver sensitivity  −82 dBm
```

**B4.** For B3's link, compute the first Fresnel zone radius at the midpoint and the
required clearance.

**B5.** An access point transmits at 17 dBm through a 5 dBi antenna with 1 dB of cable loss.
The regulatory EIRP limit is 20 dBm. Is it compliant? What is the maximum antenna gain
allowed at this transmit power?

**B6.** A client measures RSSI −62 dBm and a noise floor of −88 dBm.

(a) What is the SNR?
(b) Would you expect high or low data rates?
(c) A microwave oven raises the noise floor to −70 dBm. Recompute, and state the effect.

**B7.** Two signals arrive with a path difference of 6.25 cm at 2.4 GHz. What happens? What
path difference would be needed for the same effect at 5 GHz?

**B8.** A 4×4 access point serves a 2×2 laptop and a 1×1 IoT sensor. How many spatial
streams can each achieve?

## C. Analyse

**C1.** Explain why higher frequencies have more bandwidth available and worse propagation,
and why these are both consequences of the same physics.

**C2.** Explain why a room that performs well when empty degrades when occupied, giving two
distinct mechanisms.

**C3.** Explain how a passive antenna can have gain without amplifying anything.

**C4.** Explain why a high-gain omnidirectional antenna may be a poor choice for a
multi-storey building.

**C5.** Explain reciprocity and use it to explain why raising an access point's transmit
power to fix a coverage complaint is usually wrong.

**C6.** Explain why line of sight is insufficient, using the Fresnel zone, and explain the
winter-to-spring failure it predicts.

**C7.** Explain frequency-selective fading and why OFDM is the appropriate response.

**C8.** Explain the guard interval trade-off, and state when you would allow a short guard
interval and when you would not.

**C9.** "Multipath went from the enemy to the enabling condition." Explain, and explain why
MIMO performs better indoors than on an outdoor line-of-sight link.

**C10.** Explain why moving a receiver a few centimetres can change the signal by 20 dB, and
what this implies for site surveys.

## D. Design

**D1.** Design a 3 km point-to-point link at 5.8 GHz between two buildings. Specify
antennas, transmit power, expected margin, mast height for Fresnel clearance, and the
failure modes you have designed against.

**D2.** A warehouse 100 m × 60 m × 12 m high has steel racking throughout. Explain why this
is a difficult radio environment and outline your approach, with reasoning from this
chapter.

**D3.** For the semester project's site, estimate the number of access points needed for
−67 dBm coverage at 5 GHz, stating every assumption.

**D4.** Write the antenna selection guidance for an organisation deploying to: open-plan
offices, a warehouse, a long corridor, an outdoor courtyard, and a lift shaft. Justify each.

## E. Troubleshoot

**E1.** A user reports full signal bars and unusable performance. Give the likely cause and
the measurement that would confirm it.

**E2.** A point-to-point link worked for eight months and degraded in April. Diagnose.

**E3.** Coverage is good throughout a floor except directly under one access point.
Diagnose.

**E4.** After replacing an access point's antennas with higher-gain ones, clients connect
from further away and cannot pass traffic. Explain.

**E5.** A 5 GHz deployment covers noticeably less area than the 2.4 GHz one it replaced,
with the same access-point positions. Is this a fault? Quantify.

**E6.** A link's received signal varies by 15 dB as a person walks past the antenna.
Explain.

**E7.** A survey conducted on a Sunday showed good coverage; Monday complaints are
widespread. Give two mechanisms.

**E8.** An outdoor MIMO link achieves only one spatial stream despite 3×3 radios at both
ends. Explain.

## F. Extend

**F1.** Use a Wi-Fi analyser to record RSSI while walking slowly across a room. Plot it and
identify the fading pattern. Then repeat standing still for 60 seconds and compare the
variance.

**F2.** Measure RSSI at the same point with a phone held vertically and horizontally.
Explain the difference.

**F3.** Compute a link budget for an existing link you have access to, then measure the
actual received signal and account for the discrepancy.

**F4.** Use [tools/perfcalc.py](../../../tools/perfcalc.py) `linkbudget` for three
scenarios of your choosing, and verify one by hand.

**F5.** Measure the attenuation of a wall by taking readings either side at the same
distance from the access point. Compare with §42.1's table.

**F6.** Find the regulatory EIRP limits for your country in the 2.4 GHz, 5 GHz and 6 GHz
bands. Determine the maximum antenna gain permitted at a typical transmit power in each.
