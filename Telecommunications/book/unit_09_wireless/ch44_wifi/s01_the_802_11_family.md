# 44.1 The 802.11 Family

Thirty years of amendments to one standard. **The letters are not a sequence and the
marketing names are not the standard names**, and sorting that out is the first task.

## The table

| Amendment | Wi-Fi name | Year | Band | Max rate | The key addition |
|---|---|---|---|---|---|
| **802.11** | — | 1997 | 2.4 | **2 Mb/s** | the original |
| **802.11b** | — | 1999 | 2.4 | **11 Mb/s** | DSSS; **made Wi-Fi commercial** |
| **802.11a** | — | 1999 | **5** | **54 Mb/s** | **OFDM**; ahead of its time |
| **802.11g** | — | 2003 | 2.4 | 54 Mb/s | OFDM at 2.4 GHz |
| **802.11n** | **Wi-Fi 4** | 2009 | 2.4 + 5 | **600 Mb/s** | **MIMO**, 40 MHz channels |
| **802.11ac** | **Wi-Fi 5** | 2013 | **5 only** | **~7 Gb/s** | 80/160 MHz, 256-QAM, **MU-MIMO down** |
| **802.11ax** | **Wi-Fi 6** | 2019 | 2.4 + 5 | ~9.6 Gb/s | **OFDMA**, 1024-QAM, **efficiency** |
| **802.11ax** | **Wi-Fi 6E** | 2021 | **+ 6 GHz** | ~9.6 Gb/s | the 6 GHz band |
| **802.11be** | **Wi-Fi 7** | 2024 | 2.4 + 5 + 6 | **~46 Gb/s** | 320 MHz, 4096-QAM, **MLO** |

**Note that `a` and `b` were ratified the same year.** The lettering follows the order
projects were *started*, not finished — **which is why the sequence is not chronological and
why nobody can remember it.**

**The Wi-Fi Alliance's numbering, from 2018, exists precisely because of that.** It is
marketing, it is clearer, and **the standard names remain what specifications and equipment
datasheets use.**

## What each one actually contributed

**Reading the table as a story rather than a list:**

### 802.11b (1999) — the commercial breakthrough

**11 Mb/s was slower than 802.11a's 54**, and **b succeeded and a did not** — for reasons
that have nothing to do with speed.

**2.4 GHz radios were cheap** and 5 GHz ones were not, in 1999. **And 2.4 GHz propagates
better** (Chapter 42 §42.3), so fewer access points covered a building.

**Apple's AirPort in 1999 was 802.11b at a consumer price**, and that is when wireless
networking stopped being exotic.

> **The technically superior standard lost to the cheaper one that shipped in volume.**
> Chapter 22 §22.1 and Chapter 23 §23.1 tell the same story about OSI, and it recurs.

### 802.11a (1999) — right, and early

**OFDM at 5 GHz, 54 Mb/s**, with none of 2.4 GHz's interference. **Everything about it was
correct** and it was commercially marginal until the band's advantages became compelling a
decade later.

**Its OFDM is the direct ancestor of everything since** (Chapter 8 §8.4, Chapter 42 §42.4).

### 802.11g (2003) — OFDM comes to 2.4

**802.11a's physical layer in the popular band.** 54 Mb/s, and **backward compatible with b**
— which was necessary commercially and expensive technically.

**The protection mechanisms** (§44.2) date from here: **a g network with one b client
associated must announce every transmission in a way b devices understand**, and the
overhead is substantial.

### 802.11n (2009) — MIMO

**The largest single jump**, and it came from Chapter 42 §42.4's reversal: **multipath became
the resource.**

| Addition | Effect |
|---|---|
| **MIMO, up to 4 spatial streams** | **4× the data in the same spectrum** |
| **40 MHz channels** | 2× |
| **Frame aggregation** (A-MPDU, A-MSDU) | **amortises the per-frame overhead** |
| Short guard interval | ~11% |
| Block acknowledgement | fewer ACKs |

**Frame aggregation deserves emphasis** because it is invisible and important. **802.11's
per-frame overhead is enormous** — the preamble, the contention, the interframe spaces, the
acknowledgement. **At high data rates the overhead dominates the payload**, and sending one
large aggregated frame instead of twenty small ones is most of where n's real-world gain came
from.

### 802.11ac (2013) — wider and denser

**5 GHz only**, which was the right call — 2.4 GHz cannot support the channel widths.

| Addition | Effect |
|---|---|
| **80 and 160 MHz channels** | 4× and 8× over 20 |
| **256-QAM** | 8 bits per symbol, up from 6 |
| **Up to 8 spatial streams** | |
| **MU-MIMO (downlink)** | **transmit to several clients simultaneously** |
| Beamforming, standardised | better SNR at range |

**MU-MIMO is the conceptual step.** Until it, an access point talked to one client at a time.
**With enough antennas and knowledge of the channel, it can direct separate beams to several
clients at once** — using spatial separation rather than time separation.

**Its practical benefit was smaller than advertised**, because it requires clients to be
spatially well separated and the channel information to be current. **Wi-Fi 6 improved it
substantially.**

### 802.11ax / Wi-Fi 6 (2019) — efficiency, not speed

**The change of goal.** Every previous amendment raised the peak rate; **ax was designed for
dense environments where the problem is contention rather than link speed.**

| Addition | Effect |
|---|---|
| **OFDMA** | **several clients share one transmission**, each on its own subcarriers |
| **Uplink MU-MIMO** | several clients transmit simultaneously |
| **1024-QAM** | 10 bits per symbol |
| **BSS colouring** | **distinguish your network's transmissions from a neighbour's** |
| **TWT** — target wake time | **scheduled wake-ups, for battery life** |
| Longer OFDM symbols | better outdoors and in high multipath |

**OFDMA is the significant one**, and §44.4 develops it. **In one sentence: instead of giving
one client the whole channel for a short time, give several clients part of the channel
simultaneously.**

**Which matters because most Wi-Fi traffic is small frames** — a sensor reading, an
acknowledgement, a keystroke — **and giving a 160 MHz channel to a 64-byte frame wastes
almost all of it.**

**BSS colouring** addresses §43.4's co-channel problem: **a device can tell whether an
overheard transmission is from its own network or a neighbour's**, and can apply a different
deferral threshold — **so a distant neighbour's transmission no longer stops you
transmitting.**

**TWT** is the IoT feature: **a device negotiates when it will wake**, sleeps in between, and
**battery life improves by an order of magnitude** for sensors that report occasionally.

### Wi-Fi 6E (2021) — the band

**No new physical layer.** 802.11ax, in 6 GHz (Chapter 43 §43.1). **The significance is
entirely the spectrum**, and it is substantial.

### 802.11be / Wi-Fi 7 (2024)

| Addition | Effect |
|---|---|
| **320 MHz channels** | 2× over 160 |
| **4096-QAM** | 12 bits per symbol |
| **MLO — Multi-Link Operation** | **use 2.4, 5 and 6 GHz simultaneously** |
| Multi-RU | more flexible OFDMA allocation |
| Preamble puncturing | use a wide channel with part of it blocked |

**MLO is the genuinely new idea.** A client associates over **several bands at once** and can
**aggregate them for throughput, or use one as a backup for reliability, or send the same
frame on two for latency.**

> **MLO changes association from a single radio relationship to a multi-radio one**, and it
> is the first structural change to how a client attaches since 802.11 began.

**Preamble puncturing** is worth knowing: a 320 MHz channel with an incumbent in part of it
can still be used, **with the occupied portion excluded** — which makes wide channels usable
in the real, partially-occupied spectrum of Chapter 43 §43.3.

## The rate numbers are theoretical

**A warning that matters**, because the headline figures are quoted constantly and are never
achieved.

**"9.6 Gb/s" for Wi-Fi 6 assumes:**

- **8 spatial streams** — no client has 8; laptops have 2, phones 2, most IoT 1
- **160 MHz channel** — often unavailable (Chapter 43 §43.2)
- **1024-QAM** — requires SNR above about 35 dB, so **very close to the access point**
- **Perfect conditions, one client, no interference, no overhead**

**Realistically:**

| Claimed | Typical achieved |
|---|---|
| Wi-Fi 5, 1.3 Gb/s | **300–500 Mb/s** |
| Wi-Fi 6, 9.6 Gb/s | **500 Mb/s – 1.2 Gb/s** |
| Wi-Fi 7, 46 Gb/s | **1–2 Gb/s** |

**And the medium is shared** (§44.2). **A "1 Gb/s" access point serving thirty clients gives
each about 30 Mb/s at best**, and less once contention overhead is counted.

> **The rate on the box is the fastest single frame under ideal conditions with hardware
> nobody owns. Design against measured throughput, not against datasheets.**

## Modulation and coding schemes

**The MCS index is what a client actually negotiates**, and it is more informative than the
rate.

**An MCS specifies a modulation and a coding rate**, and the achievable index depends on SNR
(Chapter 42 §42.3):

| MCS | Modulation | Coding | Bits/symbol | Approx SNR needed |
|---|---|---|---|---|
| 0 | BPSK | 1/2 | 0.5 | **~5 dB** |
| 2 | QPSK | 3/4 | 1.5 | ~11 dB |
| 4 | 16-QAM | 3/4 | 3 | ~18 dB |
| 7 | 64-QAM | 5/6 | 5 | **~25 dB** |
| 9 | 256-QAM | 5/6 | 6.67 | **~31 dB** |
| 11 | 1024-QAM | 5/6 | 8.33 | **~35 dB** |
| 13 | 4096-QAM | 5/6 | 10 | **~40 dB** |

**Reading it:** **MCS 11 requires 35 dB SNR**, which in practice means **a client within a
few metres of the access point in a clean environment.** Most clients most of the time run at
MCS 4–7.

**And this is why Chapter 42 §42.1's SNR discussion matters operationally:** the MCS index a
client achieves is a direct function of its SNR, and **the rate follows from the MCS.**

**Rate adaptation** — the client and access point continuously adjust the MCS based on
success rate. **A client dropping from MCS 9 to MCS 4 has encountered conditions that will
not support the higher modulation**, and forcing it back up produces errors and
retransmissions that make throughput worse.

## Which to deploy

| Situation | Choose |
|---|---|
| **New enterprise deployment** | **Wi-Fi 6E or 7** — the 6 GHz band is the reason |
| Existing Wi-Fi 5, working | no urgency; upgrade at refresh |
| **High density** (lecture halls, stadiums) | **Wi-Fi 6 minimum** — OFDMA and BSS colouring are the point |
| IoT-heavy | Wi-Fi 6 for TWT |
| Home, few clients | Wi-Fi 6 is ample; 7 is not yet worth the premium |

**And the constraint is always the clients.** **A Wi-Fi 7 access point serving Wi-Fi 5
laptops delivers Wi-Fi 5** — the standards' benefits require both ends, and an estate refresh
is usually the binding item rather than the access points.

## What breaks here

**Throughput far below the datasheet.** The datasheet assumes hardware and conditions that do
not exist. Design against measurement.

**A single 802.11b or g client degrading everything.** Protection mechanisms (§44.2). Disable
low data rates.

**A Wi-Fi 6 upgrade producing no improvement.** The clients are older, or the environment's
problem was interference rather than protocol efficiency.

**Clients negotiating low MCS with strong signal.** SNR, not RSSI — something is raising the
noise floor (Chapter 43 §43.4).

**160 MHz channels unavailable.** Chapter 43 §43.2 — there are only two in 5 GHz, and DFS may
be disabled.

> **Network+ note.** Objective 2.4 expects the 802.11 standards, and **the table is examined
> directly.** Over-learn: **a = 5 GHz 54 Mb/s, b = 2.4 GHz 11 Mb/s, g = 2.4 GHz 54 Mb/s,
> n = both bands with MIMO, ac = 5 GHz only, ax = Wi-Fi 6 with OFDMA**; and the **Wi-Fi 4/5/6
> naming.** The a-versus-b band and rate confusion is the most-missed item.
