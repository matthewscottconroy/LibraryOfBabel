# 14.1 Scopes: PAN, LAN, CAN, MAN, WAN

The categories are conventional, they are taught as a taxonomy, and treating them
as one is a mistake. What actually varies across the range is a set of continuous
parameters, and the useful skill is reading off **which parameter binds** rather
than deciding which box a network belongs in.

## The conventional taxonomy

| Scope | Span | Typical instance |
|---|---|---|
| **PAN** — personal area network | metres | Bluetooth headset, wearable sensors |
| **LAN** — local area network | building | Office Ethernet, home Wi-Fi |
| **CAN** — campus area network | site, several buildings | University, hospital, industrial site |
| **MAN** — metropolitan area network | city | Carrier metro Ethernet ring |
| **WAN** — wide area network | country, continent, global | Corporate WAN, the Internet |

Two further terms appear and are worth recognising: **WLAN** for the wireless
portion of a LAN, and **SAN** for a storage area network — which is a scope
classification in name and is really a *purpose* classification, since a SAN is a
LAN carrying block storage traffic with particular loss and latency requirements.

## What actually varies

Six parameters, and any real network sits somewhere on each independently.

| Parameter | PAN | LAN | CAN | MAN | WAN |
|---|---|---|---|---|---|
| Span | m | 100s of m | km | 10s of km | 100s–10,000s km |
| **Latency** | µs | µs | µs–ms | ms | **10s–100s of ms** |
| Typical rate | Mb/s | 1–100 Gb/s | 10–400 Gb/s | 1–400 Gb/s | 1–400 Gb/s |
| Who owns the medium | you | **you** | you | usually a carrier | **a carrier** |
| Cost per bit | ~0 | ~0 | low | metered | **metered** |
| Dominant design constraint | power, coexistence | switching, broadcast domains | fibre plant, routing | provider diversity | **latency, cost, availability** |

Read the table by column and you get the taxonomy. Read it by **row** and you find
that most of the rows have converged and one has not.

## What has eroded

**Speed.** In 1980 a LAN ran at 10 Mb/s and a WAN link at 9,600 bit/s — a factor of
a thousand. Today a 100 Gb/s wide-area circuit is unremarkable, and it is entirely
common for an organisation to have a **faster path to its cloud provider than
between two floors of its own building**. The gap has not merely closed; it has
inverted in places.

**Cost per bit.** Wide-area bandwidth was metered so tightly that protocol design
was shaped by it — X.25's per-packet charging, the effort that went into terminal
protocols that minimised bytes. For most organisations today, WAN bandwidth is no
longer the constraint that shapes application design.

**Ownership.** Cloud services and colocation blur it. Is the link from your building
to your virtual private cloud a LAN or a WAN? Your address plan treats it as one
network; a carrier bills you for it as another; and the answer is that the question
is not useful.

**Error rate.** Early WAN links were noisy enough that link-layer error recovery was
mandatory, which is why X.25 did per-hop error correction (Chapter 13 §13.2). Modern
digital links have error rates low enough to ignore, which is why Frame Relay
removed the correction and MPLS never had it.

## What has not eroded, and cannot

**Latency.**

Propagation delay is distance divided by the speed of light in the medium
(Chapter 1 §1.1). It is not an engineering parameter. Chicago to Frankfurt was about
35 ms one way in 1980, is about 35 ms one way now, and will be about 35 ms one way
in 2075.

This is the only one of the original distinctions that is as true today as it was,
and it is therefore the modern meaning of the LAN/WAN divide:

> **The LAN/WAN distinction is not about ownership, speed or cost. It is about
> latency, and therefore about round trips.**

Chapter 3 §3.4 established the consequence: any protocol requiring an acknowledgement
before proceeding is limited by data-in-flight ÷ RTT, and any application performing
*k* round trips per operation costs *k* × RTT before any data moves.

So an application that performs twenty round trips per screen load is fine on a LAN
— twenty times 0.2 ms is 4 ms, invisible — and unusable over a WAN, where twenty
times 35 ms is 700 ms, per screen, regardless of how much bandwidth is available.

**No amount of bandwidth fixes it**, and this is why "we upgraded the circuit and it
made no difference" is such a common report. Chapter 3's diagnostic exercise and
Chapter 51 §51.1's tromboning discussion are the same observation in different
settings.

## The practical consequence for design

When you meet a network problem, the productive question is not "is this a LAN or a
WAN?" but **which parameter binds**.

| If the binding constraint is | Then the design question is |
|---|---|
| Latency | How many round trips? Can they be removed or moved closer? |
| Bandwidth | Where is the bottleneck, and what is on it? |
| Loss | Physical layer, or congestion? (Chapter 6 versus Chapter 13) |
| Power | Duty cycle, protocol overhead per message (Chapter 47) |
| RF contention | Airtime, not signal (Chapter 45) |
| Provider diversity | Shared fate (Chapter 56 §56.2) |
| Cost | Which term dominates — path, capacity, or egress? |

A warehouse WLAN and a metropolitan fibre ring are both "networks", and they are
different engineering problems because different rows of the table bind. Naming the
scope tells you very little; naming the constraint tells you what to do.

## Where the categories still earn their keep

Three places, and it is worth being fair to them.

**Regulatory and commercial.** A carrier sells "metro Ethernet" and "wide-area
services" as distinct products with distinct pricing, SLAs and provisioning
timescales. The categories are real in a contract even where they are blurred in
engineering.

**Standards scope.** IEEE 802 divides into 802.3 (LAN), 802.11 (WLAN), 802.15
(WPAN) and 802.16 (WMAN), and the divisions shaped what each group standardised.

**Design defaults.** "This is a LAN" carries useful implications — you own the
medium, latency is negligible, bandwidth is cheap, broadcast works, and you can
change the cabling. Those defaults are usually right, and the failure mode is
applying them past the point where they hold.

## What breaks here

**Applying LAN assumptions over a WAN.** Chatty protocols, broadcast-dependent
service discovery, assumptions about sub-millisecond latency. This is the largest source of "it works in the office and not from the branch".

**Assuming the WAN is the bottleneck** because it historically was. Measure. On a
modern network the bottleneck is frequently the application's round-trip count, a
server, a firewall, or a Wi-Fi cell — and the WAN circuit is at 12%.

**Treating a cloud connection as a LAN because the address plan says so.** The
latency is a WAN's, whatever the subnet mask says, and applications will behave
accordingly.

**Designing a SAN as an ordinary LAN.** Storage traffic has loss and latency
requirements that ordinary Ethernet does not meet by default, which is why lossless
Ethernet and priority flow control exist (Chapter 71 §71.5).

> **Network+ note.** Objective 1.6 lists the scope types and expects you to identify
> them. The exam treats them as a taxonomy; treat them that way for the exam, and
> carry §14.1's actual lesson into practice — **latency is the durable distinction,
> and it is measured in round trips rather than in kilometres.**
