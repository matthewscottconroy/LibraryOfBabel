# 9.3.3 ROADMs: Reconfigurable Optical Networks

## The Problem with Fixed Multiplexers

The first WDM systems used fixed wavelength multiplexers (AWGs or thin-film filters) at every network node: a fixed set of channels was added or dropped at each site, and the rest passed through. This worked for static traffic patterns, but as network traffic became increasingly dynamic (driven by cloud computing, CDN, and high-bandwidth applications), the inability to reconfigure optical channels without manual splicing became a major operational problem.

The solution is the **ROADM** (reconfigurable optical add-drop multiplexer): an optical switch that can add, drop, or pass-through any subset of WDM channels under software control, without any manual intervention.

## ROADM Architecture

A ROADM node consists of:

1. **Pre-amplifier**: Boosts the received signal to a working level.
2. **Wavelength selective switch (WSS)**: A MEMS-based or LCoS (liquid crystal on silicon) device that routes individual wavelengths to different output ports. A 1×N WSS has one input and N outputs; it can route each wavelength independently to any output.
3. **Add/drop ports**: Connect to local optical transceivers for channels that terminate at this node.
4. **Booster amplifier**: Amplifies the combined signal before transmission.

The WSS is the key component. Modern WSSs use **LCoS** (liquid crystal on silicon) spatial light modulators to diffract different wavelengths to different output ports:
- Port count: 1×20 to 1×32 per WSS
- Wavelength resolution: 6.25–12.5 GHz (compatible with flex-grid)
- Insertion loss: 4–7 dB per WSS
- Switching time: 10–100 ms (liquid crystal reorientation)

A **colorless, directionless, contentionless (CDC) ROADM** can route any wavelength from any input to any output, enabling maximum flexibility. This is the target architecture for disaggregated optical networks.

## ROADMs in the Context of Photonic Computing

ROADMs are not photonic computers in the sense of performing matrix operations. However, they are directly relevant to photonic computing infrastructure in two ways:

First, ROADMs implement the **optical circuit switching** layer in data center networks (Chapter 10). An optical circuit switch re-routes entire WDM channels between servers or GPU clusters based on traffic demand. Unlike packet-switched electronic networks, optical circuit switching provides dedicated bandwidth without buffering overhead.

Second, the WSS technology inside ROADMs — particularly the LCoS spatial light modulator — is related to the reconfigurable metasurface concept of Section 8.2.3. A 1D or 2D array of pixels that can be individually addressed to apply phase shifts is exactly a programmable diffractive element. The engineering that has made WSSs reliable and manufacturable provides a direct path to large-scale programmable optical computing elements.

---

## References

[1] Saleh, A.A.M. & Simmons, J.M. (2011). "All-optical networking — Evolution, benefits, challenges, and future vision." *Proceedings of the IEEE*, 100(5), 1105–1117. [Overview of ROADM architecture and reconfigurable optical network evolution.]

[2] Fontaine, N.K., Scott, R.P., Chandrasekhar, S., & Yoo, S.J.B. (2013). "Flexible optical spectrum control for next-generation WDM networks." *IEEE Communications Magazine*, 51(2), 52–59. [ROADM and flex-grid implementation details.]
