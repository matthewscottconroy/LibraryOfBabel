# 47.1 Bluetooth and BLE

**Bluetooth and Bluetooth Low Energy share a name, a band and almost nothing else.** They are
different radios with different protocols for different purposes, and treating them as
versions of one thing is the commonest confusion in this area.

## The origin

**Ericsson, 1994**, to replace the serial cable between a phone and a headset. **Named after
Harald "Bluetooth" Gormsson**, the tenth-century Danish king who united warring tribes — the
analogy being a standard uniting incompatible devices — **and the logo is his initials in
runes**, ᚼ and ᛒ, superimposed.

**The design constraints were unusual and they shaped everything:** it had to be **very cheap**
(cheaper than the cable it replaced), **very low power** (battery devices), and **very short
range** (a few metres), **in the 2.4 GHz ISM band** (Chapter 43 §43.1) because that was free.

## Classic Bluetooth

| | |
|---|---|
| Band | **2.4 GHz ISM** |
| Access | **frequency-hopping spread spectrum** — 79 channels, **1,600 hops/s** |
| Rate | 1–3 Mb/s (BR/EDR) |
| Range | **10 m typical** (Class 2), 100 m (Class 1) |
| Topology | **piconet**: one master, up to 7 active slaves |
| Power | tens of milliwatts |
| Connection | **persistent, streaming-oriented** |

**Frequency hopping is the interesting mechanism**, and it is Chapter 42's Lamarr–Antheil idea
in production: **the transmitter and receiver change frequency 1,600 times per second in a
pseudo-random sequence known to both.**

**Which gives three properties at once:**

**Interference resilience.** A hop that lands on a Wi-Fi channel is corrupted; **the next hop
is elsewhere**, and the loss is one packet rather than the connection.

**Coexistence with Wi-Fi.** Because Bluetooth spends only a fraction of its time on any
frequency, **it interferes with Wi-Fi mildly rather than continuously** (Chapter 43 §43.4).
**Adaptive Frequency Hopping** (from Bluetooth 1.2) improves it further by **learning which
channels are busy and excluding them from the sequence.**

**Security by obscurity of sequence** — weak, and it does make casual interception harder.

**What classic Bluetooth is for:** **audio, and continuous data.** A headset, a speaker, a car
kit, a keyboard, a serial replacement. **Anything that streams.**

## Bluetooth Low Energy

**A different radio, introduced in Bluetooth 4.0 (2010)**, derived from Nokia's Wibree.

| | Classic | **BLE** |
|---|---|---|
| Channels | 79, 1 MHz | **40, 2 MHz** |
| **Advertising channels** | — | **3** (37, 38, 39) |
| Rate | 1–3 Mb/s | **125 kb/s – 2 Mb/s** |
| **Connection setup** | ~100 ms | **~3 ms** |
| **Idle current** | mA | **µA** |
| **Battery** | days | **months to years** |
| Model | **streaming** | **small, infrequent transfers** |

> **BLE is not a slower Bluetooth. It is a radio designed to be off almost all the time.**

**The design principle is duty cycle.** A BLE sensor **wakes, advertises or exchanges a few
bytes, and sleeps** — and the sleep is where the battery saving is. A coin cell runs a
temperature sensor for years because **the radio is active for milliseconds per hour.**

**The three advertising channels are chosen deliberately** — 37, 38 and 39 sit **in the gaps
between Wi-Fi channels 1, 6 and 11** (Chapter 43 §43.2), so device discovery survives in a band
saturated with Wi-Fi.

### GATT — the data model

**BLE's application layer is a small hierarchical database**, and it is worth knowing because
every BLE device exposes one.

```
   Profile  (e.g. Heart Rate)
     └── Service            (Heart Rate Service, UUID 0x180D)
           ├── Characteristic  (Heart Rate Measurement, 0x2A37)  → notify
           └── Characteristic  (Body Sensor Location, 0x2A38)    → read
```

**A client reads, writes, or subscribes to notifications on characteristics.** Standard
services have assigned UUIDs; proprietary ones use 128-bit UUIDs.

**Which makes BLE devices self-describing** — a client can discover what a device offers
without prior knowledge, **and it is why a generic BLE scanner app can interact with almost
anything.**

## The versions

| Version | Year | Addition |
|---|---|---|
| 4.0 | 2010 | **BLE** |
| 4.2 | 2014 | better privacy, IPv6 support |
| **5.0** | 2016 | **2× rate, 4× range (coded PHY), 8× advertising capacity** |
| 5.1 | 2019 | **direction finding** — angle of arrival/departure |
| **5.2** | 2020 | **LE Audio, Isochronous Channels, Auracast** |
| 5.3–6.0 | 2021– | efficiency, channel sounding for ranging |

**Two deserve note.**

**BLE 5's coded PHY** trades rate for range: **125 kb/s with forward error correction gives
roughly four times the distance** of the 1 Mb/s mode. **The same Shannon trade as everywhere
else** (Chapter 4) — spend bits on redundancy, buy link budget.

**LE Audio (5.2)** moves audio to BLE with the **LC3 codec**, which is substantially more
efficient than classic's SBC. **It enables hearing aids with usable battery life**, multi-stream
stereo, and **Auracast** — broadcast audio that any number of receivers can join, for public
venues and assistive listening.

## Bluetooth mesh

**A separate specification (2017)**, built on BLE advertising rather than connections.

**Managed flooding**: a message is relayed by every node that hears it, **with a time-to-live**
and a message cache to prevent loops (Chapter 24 §24.4's argument, in a mesh).

**Used for lighting and building control**, where hundreds of nodes must be reachable and the
traffic is small. **Flooding is inefficient and it is robust and simple**, which is the right
trade for a lighting network.

## Security, honestly

**Bluetooth's security history is poor**, and the pattern is instructive.

**Pairing** establishes a shared key, and the mechanisms have improved:

| Method | Security |
|---|---|
| **Legacy PIN (2.0 and earlier)** | **weak** — often 0000 or 1234, and crackable |
| **Secure Simple Pairing (2.1)** | better; "Just Works" mode has **no MITM protection** |
| **LE Secure Connections (4.2)** | **ECDH key exchange — proper** |

**The vulnerabilities worth knowing:**

**BlueBorne (2017)** — remote code execution with **no pairing and no user interaction**,
across Android, iOS, Windows and Linux. **A device with Bluetooth on was exploitable by
proximity alone.**

**KNOB (2019)** — an attacker forces the encryption key negotiation down to **one byte of
entropy**, which is then trivially brute-forced. **The specification permitted key lengths from
1 to 16 bytes and did not require a minimum**, which is exactly TLS's downgrade problem
(Chapter 41 §41.1) in another protocol.

**BLURtooth, BIAS, and others** since.

> **The recurring shape is a specification that permits a weak option for compatibility, and
> an attacker forcing it.** Chapter 41 §41.1's argument that **TLS 1.3's design principle was
> to remove options** applies directly, and Bluetooth has been slower to learn it.

**And there is a privacy dimension:** a device advertising a **static address** is trackable
everywhere it goes. **BLE privacy uses resolvable private addresses** that rotate — and **many
devices do not implement it**, so retail and analytics companies track shoppers by their
phones' and wearables' Bluetooth advertisements.

## Where it fits

| Use | Technology |
|---|---|
| **Audio streaming** | **Classic (A2DP), or LE Audio** |
| **Wearables, sensors** | **BLE** |
| Keyboards, mice | BLE (or classic HID) |
| **Beacons, presence** | **BLE advertising** |
| **Lighting, building control** | **Bluetooth mesh** |
| File transfer | rarely — Wi-Fi Direct is better |
| **Anything needing IP** | **not Bluetooth** — see §47.2 |

**The last row matters.** **Bluetooth is not an IP network.** BLE can carry IPv6 over 6LoWPAN
and almost nothing does; **in practice a Bluetooth device talks to a phone or a gateway, which
talks to the Internet.**

## What breaks here

**Audio stuttering near a busy Wi-Fi network.** 2.4 GHz contention. **AFH mitigates it and
does not eliminate it.**

**A BLE sensor's battery lasting weeks instead of years.** The connection interval or
advertising interval is too aggressive. **Duty cycle is the whole design.**

**A device that pairs and will not reconnect.** Bonding information lost on one side; remove
the pairing on both.

**Two devices that will not pair.** Incompatible profiles, or one is BLE and the other classic
— **they are different radios and do not interoperate.**

**A tracker that follows a phone.** Static advertising addresses. Privacy features not
implemented.

> **Network+ note.** Objective 2.4 mentions Bluetooth. Over-learn: **2.4 GHz, frequency
> hopping, short range**; **BLE is for low-power intermittent data and classic is for
> streaming**; and **a piconet has one master and up to seven active slaves.** The
> BLE-versus-classic distinction is the useful part.
