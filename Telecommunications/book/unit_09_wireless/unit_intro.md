# Unit IX — Networking Without Wires

Everything built so far has assumed a wire. Not always literally — Chapter 10 covered
free space, and Chapter 8 covered modulation onto a carrier — but the network we
constructed in Units IV through VIII behaved as though each link were a private,
reliable, full-duplex path between two endpoints.

Radio is not that, in four specific ways, and this unit is about each of them.

**The medium is shared and nobody owns it.** Your access point and your neighbour's
occupy the same physical space and the same frequencies, and there is no negotiation
between them, no arbiter, and often no awareness. A microwave oven, a wireless
security camera, and a badly shielded lighting ballast are all participants in your
network whether you like it or not. Chapter 17's switch gave every wired host its own
collision domain; radio takes that back, permanently, and Chapter 44's CSMA/CA is the
consequence.

**The medium is half duplex, always.** A radio cannot listen while it transmits at
the same frequency — its own signal is billions of times stronger than anything it is
trying to hear. Every Wi-Fi link, however fast, is fundamentally half duplex, which
means the advertised "1.2 Gb/s" is shared between both directions and among every
client, and which is a substantial part of why real throughput is so much lower than
the number on the box.

**Signal strength varies enormously and continuously.** A wired link works or does
not. A radio link works at 1,201 Mb/s here, 400 Mb/s three metres away, 54 Mb/s
behind a wall, and not at all in the lift. Chapter 4's capacity formula is not an
abstraction here; it is the thing your device is tracking, in real time, several
times a second, by walking up and down the constellation ladder of Chapter 8 §8.3.

**Radio does not respect walls, and it does not respect property lines.** Anyone
within range can hear every frame. On a wired network, physical access is a
meaningful security boundary; on a wireless network it is not, and encryption is
therefore not an enhancement but a structural requirement. Chapter 44's four-way
handshake and Chapter 45's WPA3 discussion follow directly from this.

## Why we build up from the field

This unit begins with Maxwell rather than with SSIDs, and the reason is practical
rather than aesthetic.

Wireless networking has more confident folklore per square metre than any other part
of the field. *Turn the power up to improve coverage.* *Use a 160 MHz channel to make
it faster.* *Channels 1, 6 and 11 — but nobody remembers why.* *Move the router to
the middle of the house.* Some of these are right, some are wrong, and some are right
in one circumstance and catastrophically wrong in another. Without the physics, they
are all equally plausible sentences, and there is no way to tell them apart.

With the physics, they become computable. §42.3 builds a complete link budget in dBm,
and once you can do that, "will this link work at 3 km" and "why is my throughput bad
in the back office" become the same kind of question — one with an answer you can
derive rather than an opinion you can hold.

## What the unit contains

**Chapter 42 — Radio from First Principles.** The electromagnetic wave; frequency,
wavelength and why an antenna is the size it is; free-space path loss and a full link
budget; reflection, multipath, fading and the Fresnel zone.

**Chapter 43 — Spectrum and Channels.** Regulation and the ISM accident that gave us
unlicensed Wi-Fi; channel width and the arithmetic of overlap; 2.4, 5 and 6 GHz
compared honestly; the noise floor and what raises it.

**Chapter 44 — Wi-Fi.** The 802.11 family from `a` to `be`; CSMA/CA, the hidden node
problem and RTS/CTS; frames, SSIDs, association and the four-way handshake; MIMO,
MU-MIMO, OFDMA and beamforming.

**Chapter 45 — WLAN Design and Troubleshooting.** Site surveys and AP placement;
roaming, controllers and cloud management; capacity versus coverage; and a diagnostic
procedure for the complaints users actually make.

**Chapter 46 — Cellular.** The 1947 cellular idea and frequency reuse; 1G through 3G;
LTE's all-IP core; 5G NR, mmWave, slicing, and private cellular in the enterprise.

**Chapter 47 — Short-Range and IoT Radio.** Bluetooth and BLE; Zigbee, Thread and
Matter; LoRaWAN and NB-IoT and the range/rate/power trilemma; NFC and RFID.

## The number that runs the unit

If you take one thing from Unit IX, take this: **the useful quantity is not signal
strength but signal-to-noise ratio**, and both terms are in dBm, and the arithmetic is
subtraction.

A client at −65 dBm against a −95 dBm noise floor has 30 dB of SNR and will run at a
high modulation. The same client at −65 dBm in a room where a neighbouring network
has raised the floor to −75 dBm has 10 dB, and will run at perhaps a fifth of the
rate — with an identical signal strength reading and full bars on the screen.

Every wireless problem in Chapter 45 is, underneath, a question about that
subtraction: is the signal too weak, or is the noise too loud, and what is making it
so? Users report the symptom as "bad Wi-Fi." The two causes have completely different
remedies, and choosing the wrong one — almost always by turning the power up — makes
things worse for everyone in the building.
