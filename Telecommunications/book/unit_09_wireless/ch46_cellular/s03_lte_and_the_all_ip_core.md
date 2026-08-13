# 46.3 LTE and the All-IP Core

**LTE's significance is architectural rather than radio.** The air interface is better; the
change that mattered is that **the circuit-switched telephone network finally disappeared from
inside the mobile network.**

## The decision

**Every generation before LTE carried voice on circuits** (§46.2) and data on a separate
packet path bolted alongside.

**LTE has no circuit-switched domain at all.**

```
   3G:   ┌── circuit core ──▶ PSTN        (voice)
         └── packet core  ──▶ Internet    (data)
             two networks, two sets of equipment, two operational models

   LTE:  └── packet core ──▶ everything   (voice IS data)
```

> **Voice became an application over IP**, exactly like everything else — which is Chapter 23
> §23.4's end-to-end argument arriving in telephony seventy years after the telephone network
> was built the other way.

**And the operators' resistance was substantial**, because the circuit network was what they
knew, what their operational processes assumed, and — not least — **what their billing systems
were built around.** A minute of voice is easy to bill; a packet is not.

## The architecture

```
   ┌──────┐    ┌──────┐         ┌─────┐   ┌─────┐   ┌─────┐
   │  UE  │────│ eNB  │─────────│ MME │   │ SGW │───│ PGW │──▶ Internet
   └──────┘    └──────┘         └──┬──┘   └──┬──┘   └─────┘
   handset    base station         │         │
                                   └── HSS ──┘
                              subscriber database
```

| Element | Role |
|---|---|
| **UE** — User Equipment | the handset |
| **eNodeB** | **the base station** — and it does far more than 3G's did |
| **MME** — Mobility Management Entity | **control plane**: authentication, mobility, bearer setup |
| **SGW** — Serving Gateway | **user plane**: the mobility anchor |
| **PGW** — PDN Gateway | **the exit to the Internet**; assigns the IP address, does policy and charging |
| **HSS** | subscriber database — the successor to GSM's HLR |

**Two structural points:**

**The control and user planes are separate.** The MME handles signalling; the SGW and PGW
carry traffic. **This is Chapter 29 §29.1's separation, and Chapter 68's SDN argument, applied
to a mobile core** — and it is what allows the control plane to be scaled and located
independently of the traffic.

**The eNodeB is intelligent.** In 3G, base stations were controlled by a separate **Radio
Network Controller**; **LTE removed it** and moved scheduling, handover decisions and radio
resource management into the base station itself.

**Which flattened the architecture and reduced latency** — a handover no longer requires a
round trip to a controller — and it is why LTE achieves 30–50 ms where 3G achieved 100–200.

## OFDMA on the downlink

**LTE's air interface**, and it is the same mechanism Wi-Fi adopted a decade later
(Chapter 44 §44.4).

**Downlink: OFDMA.** The channel is divided into **resource blocks** — 12 subcarriers ×
0.5 ms — and **the base station schedules them among users**, every millisecond.

```
   Frequency ↑
      ┌────┬────┬────┬────┬────┐
      │ A  │ B  │ A  │ C  │ B  │   ← different users, same instant
      ├────┼────┼────┼────┼────┤
      │ C  │ C  │ B  │ A  │ A  │   ← reallocated 1 ms later
      └────┴────┴────┴────┴────┘
                Time →
```

**The scheduler is the system's intelligence.** Every millisecond it decides which users get
which resource blocks, using **channel quality reports from every handset** — so it can give a
user resource blocks on the subcarriers where **that user's channel is currently good**
(Chapter 42 §42.4's frequency-selective fading).

> **This is frequency-selective scheduling: exploiting the fact that different users
> experience fading differently at different frequencies.** With enough users, some user is
> always experiencing good conditions on any given block — **so multi-user diversity turns
> fading from a problem into a source of gain.**

**Contrast with Wi-Fi's contention** (Chapter 44 §44.2): **LTE is centrally scheduled, so
there are no collisions and no backoff**, and its efficiency under load is correspondingly
much higher. **The cost is that it requires an infrastructure that owns the spectrum** and can
schedule authoritatively — which is exactly what unlicensed operation cannot have (Chapter 43
§43.1).

**Uplink uses SC-FDMA** rather than OFDMA, for a specific reason: **OFDM has a high
peak-to-average power ratio**, which requires a linear and therefore inefficient amplifier.
**SC-FDMA's lower PAPR means better handset battery life** — a design choice made entirely for
the device rather than the network.

## Voice over LTE

**Having removed the circuit network, voice had to be rebuilt.**

**Two approaches, and the interim one persisted for years:**

**CSFB — Circuit Switched Fallback.** When a call arrives, **drop the handset back to 3G or
2G** for the call, then return.

**It works and it is poor:** call setup takes several seconds, **data drops to 3G speeds
during the call**, and it requires the legacy network to remain in service — which is exactly
what operators wanted to decommission.

**VoLTE — Voice over LTE.** Voice as **IP packets over LTE**, using **IMS** (IP Multimedia
Subsystem) for signalling — which is SIP (Chapter 41 §41.4) with a telecoms architecture around
it.

| | CSFB | VoLTE |
|---|---|---|
| Call setup | **several seconds** | **under 2 s** |
| Data during a call | **drops to 3G** | **full LTE** |
| Voice quality | narrowband | **wideband (AMR-WB)** — noticeably better |
| Requires 2G/3G | **yes** | **no** |

**VoLTE's wideband audio is the audible difference.** A VoLTE call carries roughly twice the
audio bandwidth of a traditional call, **and it is the first improvement in telephone audio
quality since the 300–3400 Hz standard was set in the 1930s** (Chapter 12 §12.2).

**And VoLTE is what permits 2G and 3G shutdown**, which is why operators pushed it — the
legacy networks occupy spectrum that LTE and 5G use far more efficiently.

## The numbers

| | |
|---|---|
| **Bandwidth** | 1.4 – 20 MHz per carrier, **aggregatable to 100 MHz+** |
| **Peak downlink** | 100 Mb/s (Cat 3) to **1 Gb/s+** (LTE-Advanced Pro) |
| **Typical real** | **20–80 Mb/s** |
| **Latency** | **30–50 ms** |
| Reuse | **1** |
| MIMO | up to 8×8 |

**Carrier aggregation** is how LTE reached gigabit rates: **combine several carriers, possibly
in different bands**, and use them as one. **A handset may aggregate a 20 MHz carrier at
800 MHz with two at 2.6 GHz** — using the low band for coverage and the high bands for
capacity simultaneously.

**Which is Wi-Fi 7's MLO** (Chapter 44 §44.1) arriving in cellular a decade earlier, for the
same reason.

## What LTE got right

**Worth stating explicitly, because 5G's marketing tends to imply LTE was inadequate:**

**All-IP.** One network, one operational model, and everything is an application.

**Flat architecture.** Removing the RNC cut latency and simplified the system.

**Central scheduling with channel awareness.** Far higher spectral efficiency than
contention-based access.

**A genuine global standard.** Unlike 2G's GSM/CDMA split and 3G's fragmentation, **LTE is
essentially universal** — which made roaming and handset economics work properly for the first
time.

**And it is still carrying most of the world's mobile data.** 5G's coverage is patchy outside
cities; **LTE is the network most traffic actually uses**, and it will be for years.

## What breaks here

**Data dropping to 3G speeds during a call.** CSFB. Enable VoLTE.

**Poor voice quality on a network that supports VoLTE.** The call fell back, or one end does
not support it, or the codec negotiated narrowband.

**A device losing service when 3G was switched off.** It relied on CSFB and does not support
VoLTE. **This is a widespread real problem** as operators decommission legacy networks.

**Good signal and poor throughput on LTE.** The cell is congested — **the scheduler is sharing
resource blocks among many users**, and signal strength does not indicate load.

**A handset showing LTE and performing badly.** Check the reported **RSRP and SINR** rather
than the bars: RSRP is signal, SINR is the ratio, and Chapter 42 §42.1's argument applies
unchanged.

> **Network+ note.** Objective 2.4 expects cellular technologies. Over-learn: **LTE is
> all-IP with no circuit-switched domain**; **voice over LTE is VoLTE, using IMS**; **typical
> LTE latency is 30–50 ms**; and **carrier aggregation combines carriers for higher rates.**
