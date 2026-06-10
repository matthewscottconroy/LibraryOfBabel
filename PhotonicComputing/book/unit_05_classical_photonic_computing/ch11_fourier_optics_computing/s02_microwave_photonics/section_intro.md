# Section 11.2: Microwave Photonics

## What This Section Is About

Microwave photonics is the field that uses photonic techniques to generate, transmit, and process radio-frequency (RF) signals. Unlike the analog optical computing discussed in Section 11.1, which lost its advantage to digital FFTs for most applications, microwave photonics has found a genuine niche: at frequencies above 20–40 GHz, electronic sampling and processing become impractical (ADCs run out of dynamic range), while photonic systems can handle these signals naturally because the optical carrier frequency (hundreds of THz) is orders of magnitude above the RF signal frequency (tens of GHz).

This section covers three subsections:

**11.2.1: RF Signal Processing** — The photonic RF channelizer, time-delay beamforming, and optical filtering for wideband radar and electronic warfare (EW) applications.

**11.2.2: Photonic ADC** — The time-lens approach to analog-to-digital conversion; why photonics can achieve >100 GHz effective sampling rates; actual performance vs. electronic ADCs.

**11.2.3: Optical Beamforming Networks** — True time delay (TTD) beamforming for phased-array antennas; dispersion-based TTD; integrated photonic beamforming on chip.

Microwave photonics is one of the most mature and commercially deployed areas of photonic computing. Defense systems using photonic RF processing are in service today. Understanding this domain builds intuition for what analog optical processing is genuinely good at, in a domain where the physics unambiguously favors optics over electronics.
