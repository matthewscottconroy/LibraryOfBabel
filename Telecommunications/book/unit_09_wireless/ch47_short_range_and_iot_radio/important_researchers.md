# Chapter 47 — The People

**Jaap Haartsen (b. 1963).** Ericsson, Lund, from 1994 — **the principal architect of
Bluetooth's radio.**

**The brief was narrow and the result was not:** replace the cable between a mobile telephone
and a headset. Haartsen's team chose **frequency hopping in the 2.4 GHz ISM band** for
robustness in a shared band, designed the piconet structure, and specified a system cheap
enough to put in a disposable accessory.

**The cost target was the design constraint that mattered.** A cable-replacement radio that
costs more than the device it connects to has no market — **so every decision, including the
short range and the modest data rate, was made to keep the silicon small.**

> **Bluetooth succeeded because it was designed to be cheap first and good second**, and it is
> in about five billion devices a year. **The name was Jim Kardach's** — an Intel engineer who
> had been reading a novel about **Harald Bluetooth, the tenth-century king who united Denmark
> and Norway**, and proposed it as a placeholder for a technology intended to unite
> communication protocols. **The placeholder was never replaced, and the logo is his initials
> in runes** — the bind rune of ᚼ and ᛒ.

**Haartsen was inducted into the National Inventors Hall of Fame in 2015**, and he has
observed since that the specification's growth from a cable replacement to a general-purpose
platform was neither planned nor entirely welcome.

**The Wibree team, Nokia Research, 2001–2006.** **The origin of Bluetooth Low Energy.**

**Wibree was a separate radio**, designed from the outset around the observation that a sensor
spends almost all of its life doing nothing. **Nokia contributed it to the Bluetooth SIG in
2007**, and it became Bluetooth 4.0's low-energy mode in 2010.

> **The significant idea was to stop optimising the active state.** Classic Bluetooth's
> engineers made transmission efficient; **BLE's made the radio absent** — and a coin cell that
> runs for years is a duty-cycle result, not an efficiency one.

**And it is worth noting that Nokia gave it away.** The company judged that a standard it did
not own was worth more than a proprietary radio it did — a judgement Chapter 20's Ethernet
history would have supported, and one Nokia's own later strategy did not repeat.

**Nicolas Sornin, Olivier Seller and François Sforza.** Cycleo, Grenoble, 2009 — **the
inventors of LoRa's chirp spread spectrum modulation.**

**The insight was to apply a technique from radar to a low-power link.** A chirp — a signal
sweeping across a band — **correlates against a known sweep with substantial processing gain**,
which permits decoding at around **−20 dB SNR**. Conventional modulation needs positive SNR.

**Semtech acquired Cycleo in 2012 for about $5 million**, which in retrospect was inexpensive
for a modulation now in hundreds of millions of devices.

> **LoRa's commercial structure is unusual and worth noticing.** **The modulation is
> proprietary and single-source** — Semtech — **while LoRaWAN, the network layer above it, is
> an open standard anyone may implement.** So the network can be owned by its operator and the
> silicon cannot be second-sourced, and organisations choosing LoRaWAN for independence from
> carriers accept a dependency on one chip vendor instead.

**Charles Walton (1921–2011).** **The first patent for a passive RFID transponder**, 1973 —
US 3,752,960, a **passive radio key card for door access.**

**Walton held some fifty patents and licensed the RFID work to lock manufacturers**, earning
modestly from it. **His patents expired in the early 1990s, before the technology's growth**,
and he received no royalties from the supply-chain and payment industries built on it.

**Mario Cardullo (b. 1935)** holds the 1973 patent usually cited as the first for an active
RFID tag with rewritable memory — and **Harry Stockman's 1948 paper "Communication by Means of
Reflected Power"** is the founding statement of backscatter, three decades before the
technology could be built.

> **Stockman closed his paper by observing that "considerable research and development work
> has to be done before the remaining basic problems in reflected-power communication are
> solved."** He was correct, and the interval was thirty years — **another entry in this
> book's list of correct designs waiting for a component** (Chapter 46's Ring, Chapter 19's
> Perlman).

**Léon Theremin (1896–1993)** deserves the historical footnote. **"The Thing"** — the listening
device concealed in a carved seal presented to the US ambassador in Moscow in 1945 — **was a
passive resonant cavity with no power source, energised by an external radio beam and
modulating its reflection with sound.**

**It hung in the ambassador's study for seven years undetected**, because there was nothing to
detect: no battery, no oscillator, and no emission unless illuminated. **It is passive
backscatter, built and deployed before Stockman described the principle in print.**

**The Thread Group and the Connectivity Standards Alliance.** Institutional rather than
individual, and the story of §47.2 is largely institutional.

**Thread was founded in 2014 by Nest (then newly acquired by Google), Samsung, ARM, Freescale
and Silicon Labs**, on the decision that low-power home devices should be IPv6 hosts rather
than sit behind translating gateways.

**The Zigbee Alliance renamed itself the Connectivity Standards Alliance in 2021** and
published **Matter in 2022** — **an organisation abandoning the network layer it had spent
eighteen years developing in favour of an application layer over someone else's transports.**

> **That is an unusual institutional act**, and it is the correct diagnosis: **Zigbee's
> failure was never its radio.** It was that a "Zigbee" light and a "Zigbee" hub from different
> vendors did not work together — **a semantics problem, which is an application-layer
> problem** (Chapter 21).

## A note on the pattern

**Three of this chapter's technologies were invented for one purpose and succeeded at
another.** Bluetooth was a cable replacement and became a platform; LoRa was a radar technique
and became a sensor network; **RFID was a door key and became the substrate of global
logistics.**

**And two of them — 802.15.4's mesh and Wibree's duty cycling — were given away by their
originators**, on the judgement that a standard is worth more than a product.

> **The chapter's technologies are the least visible in this book and among the most
> numerous.** Passive RFID tags are manufactured at a rate of tens of billions a year —
> **more each year than the world's people, Wi-Fi devices and cellular subscriptions
> combined** — and almost nobody thinks about them at all.
