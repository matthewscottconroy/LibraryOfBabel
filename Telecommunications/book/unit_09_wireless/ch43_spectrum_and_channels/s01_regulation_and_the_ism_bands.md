# 43.1 Regulation and the ISM Bands

Spectrum is a shared physical resource that cannot be manufactured, and unlike a cable
you cannot lay another one. Everyone within range shares it, and there is no technical
mechanism preventing anyone from transmitting.

So spectrum is governed by law rather than by protocol, and understanding that is
necessary to understand why Wi-Fi is the way it is.

## Why regulation exists

**The failure mode is complete and mutual.** Two transmitters on the same frequency in the
same place interfere; neither can fix it unilaterally, and raising power escalates rather
than resolves.

**And the early history demonstrated it.** By 1910 ship-to-shore radio was chaotic —
operators transmitting over each other, deliberately jamming competitors, and distress
calls lost in the noise. The **Titanic** in 1912 is the case usually cited: nearby ships
either were not listening or could not hear through the traffic.

The Radio Act of 1912 followed within months, and the pattern — a disaster, then
regulation — has repeated at every subsequent expansion of radio use.

## Who decides

| Body | Scope |
|---|---|
| **ITU-R** | **global** — allocates bands to services by region |
| **Region 1** | Europe, Africa, Middle East, former USSR |
| **Region 2** | the Americas |
| **Region 3** | Asia-Pacific |
| **FCC** | United States |
| **Ofcom** | United Kingdom |
| **ETSI / CEPT** | Europe |
| National regulators | everywhere else |

The ITU allocates internationally and national regulators implement, which means the
rules differ by country — and a device legal in one jurisdiction may not be in another.

> This is why an access point has a country setting, and why setting it wrongly is a legal
> matter rather than a preference. Channel availability, maximum power and whether radar
> detection is required all follow from it.

## Licensed versus unlicensed

**The fundamental division:**

| | Licensed | Unlicensed |
|---|---|---|
| **Access** | **exclusive**, by payment | **anyone**, free |
| Interference | **legally protected** | **no protection whatsoever** |
| Cost | **enormous** — billions at auction | zero |
| Examples | cellular, broadcast, satellite | **Wi-Fi, Bluetooth, Zigbee** |
| Quality guarantee | possible | **impossible** |
| Innovation barrier | very high | **none** |

The trade is exactly the one you would expect. Licensed spectrum can be engineered to a
guarantee because the licensee controls it; unlicensed spectrum cannot, because anyone may
transmit at any moment for any reason.

And the auction figures are worth knowing to appreciate what unlicensed access is worth:
the US C-band auction in 2021 raised **\$81 billion**, and European 5G auctions have raised
tens of billions. That is the price of exclusivity.

## The ISM accident

Wi-Fi exists because of a decision made for an entirely unrelated reason.

ISM — Industrial, Scientific and Medical — bands were set aside for equipment that
emits radio energy as a by-product rather than for communication:

- **Microwave ovens** — 2.45 GHz, because water absorbs there
- Industrial heaters and welders
- Medical diathermy equipment

**The bands were essentially dumping grounds.** They were allocated to noise sources, so
nobody wanted them for communication, and the regulators' position was that anything using
them must tolerate whatever interference it found.

In 1985 the FCC made a small change that turned out to matter enormously: it permitted
unlicensed spread-spectrum communication in the ISM bands, at low power, on the condition
that devices tolerate interference and cause none that anyone is obliged to accept.

Michael Marcus, an FCC engineer, drove the decision. The reasoning was that
spread-spectrum techniques — developed for military anti-jamming — could coexist with noise
sources and with each other in a way that narrowband signals could not.

> The 2.4 GHz band was given away because it was considered worthless, and it became the
> most economically significant spectrum allocation in history.

Wi-Fi, Bluetooth, Zigbee, cordless telephones, wireless microphones, baby monitors, garage
door openers, and most of the Internet of Things exist in the space nobody wanted — and
the crowding of Chapter 43 §43.3 is the direct consequence.

**The lesson worth extracting:** the allocation succeeded because it required nothing of
anyone. No licence, no coordination, no permission — exactly the property that Chapter 28
§28.1 identifies as the precondition for rapid adoption.

## The rules for unlicensed use

**Unlicensed is not unregulated.** Devices must comply with:

| Constraint | Typical |
|---|---|
| **Maximum EIRP** | **20 dBm (100 mW)** in Europe at 2.4 GHz; **30 dBm (1 W)** in the US at 5 GHz with restrictions |
| **Maximum transmit power** | separately limited on some bands |
| **Out-of-band emissions** | strict masks — you must not spill into neighbours |
| **Duty cycle** | limited in some sub-GHz bands |
| **DFS** — dynamic frequency selection | **mandatory on some 5 GHz channels** |
| **TPC** — transmit power control | required alongside DFS in Europe |
| **Listen before talk** | required in Europe generally |

The EIRP figure is the one that matters operationally (Chapter 42 §42.2), and it is
why "just turn the power up" is often not available — a compliant access point at
20 dBm EIRP with a 6 dBi antenna is already transmitting at 14 dBm and cannot legally do
more.

### DFS — sharing with radar

The 5 GHz band's central complication, and it produces a real operational surprise.

Large parts of 5 GHz are shared with weather radar, military radar and aviation radar,
which have priority. A Wi-Fi device using those channels must:

1. Listen for 60 seconds before using the channel (10 minutes on some weather-radar
   channels)
2. Monitor continuously while in use
3. Vacate within 10 seconds if a radar pattern is detected, and not return for 30
   minutes

**The consequence:**

> A DFS radar event moves every client off the channel with no warning, and the access
> point cannot return for half an hour.

Which appears as an unexplained mass disconnection, and — because false detections
happen — may occur when there is no radar. Airport-adjacent sites and coastal areas see
this most.

And it is why non-DFS channels are congested: many deployments avoid DFS entirely,
concentrating everyone onto the handful of channels that do not require it. Which is a
rational individual choice producing a bad collective outcome, and Chapter 43 §43.3
returns to it.

## The 6 GHz addition

The largest spectrum allocation for unlicensed use in decades, and it changes the
arithmetic of Chapter 43 §43.2 substantially.

| Region | 6 GHz allocation | Approximate |
|---|---|---|
| **United States** | 5.925–7.125 GHz | **1,200 MHz** |
| **Europe** | 5.945–6.425 GHz | **480 MHz** |
| UK | 5.925–6.425 GHz | 500 MHz |
| Others | varies considerably | — |

For comparison: the entire 2.4 GHz ISM band is 83.5 MHz. The US 6 GHz allocation is
**fourteen times larger**, and it arrives without a legacy of existing devices.

Its rules are different, and the difference matters:

Low Power Indoor (LPI) — indoor only, no external antennas, low power, and **no DFS
required.** This is what most enterprise Wi-Fi 6E and 7 uses.

Standard Power with AFC — higher power outdoors, but the device must consult an
**Automated Frequency Coordination** service, which knows where incumbent licensed users
(fixed microwave links) operate and tells the device which channels it may use at what power.

> **AFC is a database-driven approach to spectrum sharing**, and it is a genuinely new
> regulatory model — neither exclusive licensing nor free-for-all, but **coordinated
> sharing mediated by a service.** Chapter 72 §72.3 argues it is where spectrum policy is
> heading generally.

## What this means for a network engineer

**Four practical consequences:**

**You cannot fix interference by transmitting harder.** Regulation limits you, and Chapter
42 §42.2's reciprocity means it would not work anyway.

**You have no legal protection.** A neighbour's access point on your channel is entirely
lawful, and there is no authority to appeal to.

**Country configuration is a compliance matter.** Setting an access point to a country with
more permissive rules than yours is an offence, and it will also produce channels your
clients cannot use.

Spectrum is finite and shared, so the design problem is coexistence rather than
exclusion — which is the whole of Chapter 43 §43.2 and §43.3.

## What breaks here

Clients disconnecting en masse with no cause in any log. A DFS radar event. Check
whether the channel is DFS and look for a radar-detection message.

An access point refusing a channel you configured. Country setting, or DFS availability
check in progress (60 seconds, or 10 minutes).

A device that works in one country and not another. Different channel availability.

Coverage complaints that cannot be fixed with power. You are already at the regulatory
limit, or the link is asymmetric.

**6 GHz not appearing on client devices.** Country regulations, client hardware age, or a
band that is not enabled.

> **Network+ note.** Objective 2.4 expects the frequency bands and regulatory concepts.
> Over-learn: 2.4 GHz and 5 GHz are unlicensed ISM/UNII bands; **unlicensed means no
> interference protection**; DFS requires vacating a channel when radar is detected; and
> **regulatory domain determines available channels and power.** The DFS behaviour appears
> in troubleshooting scenarios.
