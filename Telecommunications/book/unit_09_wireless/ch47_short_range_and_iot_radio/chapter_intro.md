# Chapter 47 — Short-Range and IoT Radio

Wi-Fi and cellular are designed to move a lot of data to devices that have a power
supply or a battery you charge daily. An enormous and growing category of devices
wants neither: a door sensor that sends twelve bytes a day and must run for ten years
on a coin cell; a livestock tag that must report from eight kilometres away; a
payment card that has no battery at all and must work when waved near a terminal.

These are different problems, and they are solved by different radios. This chapter
is about the design space they occupy.

## The trilemma

Three properties, and you get approximately two:

```
                    RANGE
                     ╱ ╲
                    ╱   ╲
                   ╱     ╲
              RATE ─────── POWER
                 (efficiency)
```

**Long range** requires either high transmit power (costing battery) or a very low
data rate (so that energy per bit is high and the receiver can integrate over a long
symbol — Chapter 4 §4.4's low-SNR regime, where capacity is linear in SNR and
bandwidth is nearly free).

**High rate** requires bandwidth and SNR, which means either close proximity or
significant power.

**Low power** requires transmitting rarely, briefly, and weakly.

Every technology in this chapter is a point in that space, and knowing *which* point
is how you choose. LoRaWAN buys 10 km range and ten-year battery life by accepting a
data rate of a few hundred bits per second. Bluetooth LE buys years of battery life
and megabit rates by accepting ten metres. NFC dispenses with the battery entirely by
accepting four centimetres.

## The families

**Bluetooth and BLE.** Classic Bluetooth (1998) was designed to replace cables —
headsets, keyboards, file transfer — with a continuous connection and moderate power.
**Bluetooth Low Energy**, introduced in 4.0 (2010), is a genuinely different protocol
sharing a name: it is optimised for devices that wake, send a few bytes, and sleep,
and it achieves multi-year coin-cell operation. Essentially every fitness tracker,
sensor, beacon and smart lock uses BLE. §47.1 covers the GATT model, advertising and
connection, and the privacy machinery (rotating random addresses) added after
researchers demonstrated that fixed BLE addresses made devices trivially trackable
through public spaces.

**Zigbee, Thread, and Matter.** Both Zigbee (2003) and Thread (2014) use IEEE
802.15.4 radios and both build **mesh** networks — devices relay for one another,
extending range beyond any single link and providing redundancy. The distinction that
matters: Zigbee defined its own application layer, so Zigbee devices from different
vendors frequently could not interoperate despite sharing a radio. Thread carries
**IPv6** directly (over 6LoWPAN header compression), making every device a routable
IP node.

**Matter** (2022) is the application layer built on top of Thread, Wi-Fi and
Ethernet, backed by Apple, Google, Amazon and the Connectivity Standards Alliance,
and it exists because the smart home spent fifteen years as a collection of
incompatible ecosystems. §47.2 treats it as a case study in why standardising the
radio is insufficient — the same lesson Chapter 21 §21.1 drew about interfaces.

**LPWAN: LoRaWAN and NB-IoT.** Low-power wide-area networks take the trilemma to its
extreme: kilometres of range, years of battery, and data rates measured in hundreds
of bits per second.

**LoRa** uses chirp spread spectrum, which trades bandwidth for processing gain and
allows reception well below the noise floor (Chapter 4 §4.4 again). It operates in
unlicensed sub-GHz bands, so anyone can deploy a gateway — and a single gateway can
serve thousands of devices over 5–15 km in open country. The Things Network is a
community-operated global LoRaWAN infrastructure. The constraint is duty-cycle
regulation: in Europe a device may transmit for only 1% of the time in the 868 MHz
band, which bounds how much any device can ever say.

**NB-IoT** does the same job in *licensed* cellular spectrum, deployed by mobile
operators inside existing LTE/5G carriers. Better reliability and coverage guarantees,
at the cost of a subscription and dependence on an operator. §47.3 compares them on
the axes that decide real deployments: cost per device, cost per message, coverage
control, and who you have to negotiate with.

**NFC and RFID.** The very short range, and the one whose physics is different: at a
few centimetres the reader and tag are in each other's **near field**, coupled
inductively rather than by propagating waves. This permits the reader to *power* the
tag by induction, which is why a contactless card, a passport chip, and a warehouse
inventory tag need no battery at all. §47.4 covers passive versus active tags, the
frequency bands and their ranges, and the security model — including why "it only
works at 4 cm" is a weaker guarantee than it sounds, since a determined attacker with
a large antenna has repeatedly extended that range in published research.

## Why a networking student should care

Two reasons beyond completeness.

These devices are on your network and they are the weakest thing on it. A camera,
a sensor gateway, a smart lock and a building management controller are all computers
running old firmware that will never be patched, frequently with hardcoded
credentials, sitting inside your perimeter. The Mirai botnet in 2016 assembled
hundreds of thousands of such devices and generated attacks exceeding 1 Tb/s. The
network-level answer is segmentation — Chapter 20's VLANs and Chapter 60's
microsegmentation — and IoT is the single most common justification for both in
modern enterprise design.

**They share the spectrum.** Zigbee and BLE occupy 2.4 GHz alongside Wi-Fi. A dense
deployment of building sensors is, from Chapter 43's perspective, part of your noise
floor. A wireless survey that ignores them will mispredict.

## By the end you will be able to

- State the range/rate/power trilemma and place any technology within it.
- Distinguish Classic Bluetooth from BLE by design intent and use case.
- Explain mesh networking's advantages and the routing cost it incurs.
- Explain why Thread carries IPv6 and what Matter adds above it.
- Compare LoRaWAN and NB-IoT on coverage, cost, control and regulatory constraint.
- Explain how a passive NFC tag is powered and what that implies for range.
- Justify a segmentation design for a building full of IoT devices.
