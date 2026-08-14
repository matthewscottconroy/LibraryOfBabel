# 47.3 LPWAN: LoRa, NB-IoT and the Trilemma

Low-Power Wide-Area Networks solve a problem the technologies so far cannot: a
battery-powered sensor, kilometres away, running for a decade.

And the way they solve it is by giving up the thing everything else optimises.

## The trilemma

**Three properties, and you may have two:**

```
                    RANGE
                     ╱ ╲
                    ╱   ╲
                   ╱     ╲
                  ╱       ╲
            POWER ─────────  DATA RATE
```

| Pick | Give up | Example |
|---|---|---|
| **Range + rate** | **power** | cellular LTE, Wi-Fi |
| **Rate + low power** | **range** | BLE, Zigbee, Thread |
| **Range + low power** | **rate** | **LPWAN** |

The physics is Shannon's (Chapter 4 §4.2). To be heard further with the same power, you
must either spread the energy over more time — sending fewer bits — or accept a lower
SNR, which by the capacity theorem also means fewer bits.

> **LPWAN's range is bought with data rate**, and the exchange rate is severe: hundreds of
> bits per second, over tens of kilometres.

Which is adequate for a very large class of application: a water meter reading, a soil
moisture sample, a parking-bay occupancy flag, a cattle tracker's position. A few bytes,
occasionally.

## LoRa and LoRaWAN

Two things with one name, and the distinction matters.

**LoRa** is the **physical layer** — a proprietary modulation owned by Semtech.
**LoRaWAN** is the **open network protocol** on top of it.

### The modulation

**Chirp Spread Spectrum.** The signal sweeps across the band — a chirp — and the receiver
correlates against the expected sweep.

**Why it works at low SNR:** the correlation gain means LoRa can decode signals below the
noise floor — down to about −20 dB SNR, where conventional modulation needs positive SNR
(Chapter 42 §42.1).

> **A LoRa signal you cannot see on a spectrum analyser is decodable.** This is the property
> that produces the range.

The spreading factor (SF7–SF12) is the trade, in one parameter:

| SF | Rate (125 kHz) | Range | Time on air (12 bytes) |
|---|---|---|---|
| **SF7** | **5.5 kb/s** | shortest | **~50 ms** |
| SF9 | 1.8 kb/s | | ~200 ms |
| **SF12** | **250 b/s** | **longest** | **~1.5 s** |

Each step up doubles the time on air and adds about 2.5 dB of link budget — so SF12
reaches perhaps four times as far as SF7 and takes thirty times as long to send the same
message.

And time on air is the binding constraint, because of duty cycle limits (below).

### LoRaWAN

| | |
|---|---|
| Bands | **sub-GHz ISM** — 868 MHz (EU), 915 MHz (US), 433 MHz |
| Range | **2–5 km urban, 15 km+ rural**, and far more with line of sight |
| Rate | **250 b/s – 50 kb/s** |
| Battery | **5–10 years** on a coin cell or AA |
| Topology | **star** — devices to gateways, gateways to a network server |
| Cost | **very low**; the network can be private |

**The architecture is worth noting:**

```
   Devices ──▶ Gateways ──▶ Network Server ──▶ Application Server
             (many, cheap)   (dedup, ADR)      (your data)
```

A device does not associate with a gateway. It transmits, and every gateway that hears it
forwards the message to the network server, which deduplicates. So there is no handover,
no association, and no roaming — a moving device simply is heard by different gateways.

Which is why LoRaWAN suits tracking: mobility is free, because there is no connection to
maintain.

### The device classes

| Class | Downlink | Power | Use |
|---|---|---|---|
| **A** | **only after an uplink** | **lowest** | **most sensors** |
| B | scheduled beacon slots | medium | occasional actuation |
| C | **always listening** | **highest** | mains-powered actuators |

Class A is the default and the reason for the battery life: the device transmits, opens
two brief receive windows, and sleeps. The network cannot reach it in between — if you
want to send it something, you wait until it next reports.

> **A Class A device is unreachable by design**, and applications must be built around that
> rather than against it.

### The duty cycle constraint

In Europe, 868 MHz sub-bands are limited to a 1% duty cycle — 36 seconds of transmission
per hour (Chapter 43 §43.1).

**Which interacts badly with high spreading factors:**

```
   SF12, 12-byte payload:   ~1.5 s time on air
   1% duty cycle:           36 s per hour
   ────────────────────────────────────────
   Maximum:                 24 messages per hour  ← and that is the whole budget
```

And it constrains the gateway more than the device, because a gateway's downlink shares the
same limit across every device it serves. This is why LoRaWAN downlink is scarce and
why Class A's model is not merely a power optimisation but a necessity.

**Adaptive Data Rate** lets the network server tell a device to use the lowest spreading factor
that works, saving both battery and duty cycle — and it is essential rather than optional
in any dense deployment.

## NB-IoT and LTE-M

**The cellular answer**, standardised by 3GPP and operating in **licensed spectrum.**

| | **NB-IoT** | **LTE-M** |
|---|---|---|
| Bandwidth | **180 kHz** | 1.4 MHz |
| Rate | **~26–66 kb/s** | **~375 kb/s – 1 Mb/s** |
| **Mobility** | **limited — no handover** | **full handover** |
| **Voice** | no | **yes (VoLTE)** |
| Latency | 1.6–10 s | 50–100 ms |
| Battery | **10+ years** | several years |
| Coverage | **+20 dB over LTE** | +15 dB |
| Cost | lowest | higher |

**The +20 dB is the point.** NB-IoT achieves it by **repetition** — sending the same data many
times and combining at the receiver — which is the same range-for-rate trade as LoRa's
spreading factor, implemented differently.

And +20 dB is roughly the difference between working and not working inside a basement or a
buried meter pit, which is exactly where utility meters live.

## Comparing them

| | **LoRaWAN** | **NB-IoT** | **Sigfox** |
|---|---|---|---|
| **Spectrum** | **unlicensed** | **licensed** | unlicensed |
| **Who operates it** | **you, or a carrier** | **a carrier** | a carrier |
| **Interference risk** | **yes** | **no** | yes |
| Rate | 250 b/s – 50 kb/s | 26–66 kb/s | **100 b/s, 140 msgs/day** |
| Range | 2–15 km | 1–10 km | 10–50 km |
| **Private deployment** | **yes — the key advantage** | **no** | no |
| Per-device cost | very low | low | very low |
| Recurring cost | **none, if private** | **subscription** | subscription |

**The deciding question is usually not technical:**

> **Do you want to run the network, or rent it?**

LoRaWAN is the only one you can own. Buy gateways, run a network server (or use The Things
Network), and there is no subscription and no dependency on a carrier's roadmap.

NB-IoT gives licensed spectrum and carrier coverage — no gateways to deploy, no
interference risk, and a per-device subscription forever plus exposure to the carrier's
technology decisions (§46.2's 2G shutdown is exactly this risk, realised).

Sigfox is worth mentioning as a cautionary tale: an ultra-narrowband proprietary network
with a single operator, which entered receivership in 2022 and was acquired. A network
technology with one commercial operator is a single point of failure at the business layer,
and customers with deployed devices discovered it.

## Choosing

| Requirement | Choose |
|---|---|
| **Sensors on your own site, no subscription** | **LoRaWAN, private** |
| Sensors across a country | **NB-IoT**, or a LoRaWAN operator |
| **Needs mobility across cells** | **LTE-M** |
| Needs voice | **LTE-M** |
| **Deep indoor, buried, basement** | **NB-IoT** (+20 dB) or LoRa at high SF |
| Highest data rate in the class | **LTE-M** |
| **Lowest possible cost per device** | LoRaWAN or NB-IoT |
| **Frequent, large messages** | **none of these** — use cellular or Wi-Fi |

And the last row is the common design error. LPWAN is for a few bytes, occasionally.
An application sending a kilobyte every minute is not an LPWAN application, and forcing it
produces duty-cycle exhaustion, battery drain and a network that does not work.

## What breaks here

A LoRaWAN device sending far fewer messages than expected. Duty cycle, at a high spreading
factor. Enable ADR, or reduce the payload.

**Downlink not reaching a device.** Class A — it is unreachable except after an uplink.
Redesign around it.

**Battery life far below specification.** Spreading factor too high, or the device is
retransmitting, or it is Class C.

**Range far below the datasheet.** Datasheets quote line of sight. Buildings, and the
antenna's position (Chapter 42 §42.2), dominate.

An NB-IoT deployment stranded by a network change. Carrier dependency — the same risk as
§46.2's 2G shutdown.

**A Sigfox deployment with no operator.** Business-layer single point of failure.

> **Network+ note.** Objective 2.4 mentions IoT connectivity. Over-learn: LPWAN trades data
> rate for range and battery life; LoRaWAN uses unlicensed sub-GHz spectrum and can be
> privately deployed; NB-IoT uses licensed cellular spectrum and requires a carrier; and
> these carry a few bytes occasionally, not continuous data.
