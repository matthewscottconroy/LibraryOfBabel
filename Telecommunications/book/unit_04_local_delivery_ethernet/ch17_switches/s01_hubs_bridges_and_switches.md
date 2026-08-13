# 17.1 Hubs, Bridges, and Switches

Four devices, one evolutionary line, and each step adds exactly one capability. The
progression is worth walking because it makes clear what a switch actually *is* —
which is a bridge with enough ports and enough silicon to be interesting.

## Repeater

**Regenerates the signal. Two ports.**

A repeater receives a degraded signal on one port and transmits a clean one on the
other. It operates purely on **symbols**, not on frames — it has no idea what a MAC
address is, does not buffer, and does not examine anything.

It is Chapter 5 §5.1's regeneration in a box: decide which discrete value was sent,
transmit it afresh, discard the accumulated noise.

**What it buys:** distance. Two 500 m coaxial segments joined by a repeater span
1,000 m.

**What it does not change:** anything logical. The two segments remain **one
collision domain** and **one broadcast domain**. Everybody still hears everybody;
CSMA/CD still arbitrates; the bandwidth is still shared.

And the repeater's own latency counts against the slot time (Chapter 16 §16.2),
which is why the original specification permitted only four repeaters between any
two stations — the **5-4-3 rule**: five segments, four repeaters, three populated.

## Hub

**A multi-port repeater.**

Identical to a repeater in every respect except port count. A signal arriving on any
port is regenerated onto **every other port**.

```
        A         B         C         D
        │         │         │         │
        └────┬────┴────┬────┴────┬────┘
             │         │         │
        ┌────┴─────────┴─────────┴────┐
        │            HUB              │
        │  every signal → every port  │
        └─────────────────────────────┘
```

The consequences, from Chapter 11 §11.3:

- **Physically a star. Logically a bus.** The cabling radiates from a point; the
  behaviour is identical to twelve stations tapped onto one coaxial cable.
- **One collision domain**, covering every port.
- **Bandwidth is shared.** Twelve stations on a 10 Mb/s hub have 10 Mb/s *between*
  them.
- **Half duplex only**, because a station must listen while transmitting.
- **No privacy at all.** Every frame reaches every station's interface, which
  discards those not addressed to it — unless placed in promiscuous mode, which is
  one command.

That last point is worth dwelling on. **On a hub network, packet capture requires no
special access.** Plug in, set promiscuous mode, and you see everything. This is why
network monitoring in the 1990s was trivially easy and why it became difficult
afterwards — the difficulty is a *side effect of a performance improvement*, not a
security feature that was designed.

Hubs disappeared entirely once switch prices fell, which happened around 1998–2002.

## Bridge

**Two or more ports, and it makes a forwarding decision.**

This is the step that matters. A bridge does not merely regenerate; it **receives an
entire frame**, examines its addresses, and decides whether to forward it.

Radia Perlman's characterisation is that a bridge is a device that improves the
network by *not* forwarding things.

The algorithm, which §17.2 develops:

1. **Learn.** Record the frame's **source** address against the port it arrived on.
2. **Forward.** Look up the **destination** address. If known, send it out that port
   only. If unknown, flood it everywhere except the arrival port.

The consequences are categorical rather than incremental:

- **Each port becomes its own collision domain.** A collision on port 3 does not
  reach port 7, because the bridge does not repeat the signal — it forwards frames,
  and it does so only after receiving them intact.
- **Traffic is contained.** A conversation between two stations on port 3's segment
  never reaches port 7 at all, so port 7's bandwidth is available for something else.
- **Segments may run at different speeds**, because the bridge buffers and forwards
  rather than repeating in real time.
- **The broadcast domain is unchanged.** Broadcasts are flooded by construction.

Early bridges were two-port devices implemented in software, forwarding perhaps a
few thousand frames per second, and they were used to join segments rather than to
connect stations.

## Switch

**A bridge with many ports, implemented in hardware.**

There is no protocol difference. A switch runs the same learn-and-forward algorithm
a bridge does, standardised in the same document (IEEE 802.1D). The differences are
engineering:

| | Bridge (c. 1990) | Switch (modern) |
|---|---|---|
| Ports | 2–4 | 8–96, or thousands in a chassis |
| Forwarding | software | **ASIC, wire speed on every port** |
| Rate | thousands of frames/s | hundreds of millions |
| Latency | milliseconds | **~1 µs** store-and-forward, ~500 ns cut-through |
| Cost per port | high | trivial |
| Typical use | joining segments | **one station per port** |

That last row is the transformation. When a bridge had four ports and a station cost
more than a port, you attached *segments*. When a switch has forty-eight ports and a
port costs less than a metre of cable, you attach **stations** — one per port.

At which point:

- **Every station has its own collision domain**, containing exactly itself and the
  switch.
- **There is nobody to collide with**, so **full duplex** becomes possible
  (Chapter 16 §16.4).
- **CSMA/CD becomes vestigial** and is eventually removed from the standard.
- **Each station gets the full link rate**, in both directions, simultaneously.
- **A switch forwards many frames at once** between different port pairs — a 48-port
  gigabit switch has an aggregate forwarding capacity of 96 Gb/s, not 1.

**This is why replacing a hub with a switch produces an order-of-magnitude
improvement.** It is not a faster hub; it is a different thing.

## The comparison, assembled

| | Repeater/Hub | Bridge/Switch | Router |
|---|---|---|---|
| Operates on | signals | **frames** | packets |
| OSI layer | 1 | **2** | 3 |
| Examines | nothing | MAC addresses | IP addresses |
| Collision domains | **one** | **one per port** | one per port |
| Broadcast domains | one | **one** | **one per interface** |
| Forwarding decision | none | learned table | routing table |
| Address structure | — | flat | **hierarchical** |
| Speed | wire | wire (ASIC) | wire (ASIC) |

The two rows in bold across the table are the ones to hold:

> **A switch breaks up collision domains. It does not break up broadcast domains.
> A router breaks up broadcast domains.**

§17.3 develops this, and it is the most examined and most confused pair of facts in
the chapter.

## Layer 3 switches, and the marketing

A **Layer 3 switch** is a router implemented in switching silicon, in a switch
chassis, with many ports.

The term arose because "router" implied the slow, software-forwarding, low-port-count
devices of the early 1990s, and vendors selling fast hardware-forwarding routers with
forty-eight ports wanted a word that did not carry that baggage.

Functionally, a Layer 3 switch **is a router**. It maintains a routing table, does
longest-prefix match, decrements TTL, rewrites MAC addresses, and breaks up broadcast
domains. Chapter 20 §20.4's inter-VLAN routing on a Layer 3 switch is routing, done
by a device the marketing calls a switch.

The distinction that survives is about **where the device is designed to sit**:
Layer 3 switches have many identical high-speed ports and are optimised for
LAN-facing aggregation; routers have fewer, more varied interfaces and richer WAN
and policy features. Both route.

**Multilayer switch** is the same thing with a broader name, sometimes including
Layer 4 awareness for load balancing or ACLs.

## What breaks here

**Expecting a hub to isolate anything.** It does not, in any respect. If you find one
in production, the network has one collision domain and no privacy.

**Expecting a switch to break up a broadcast domain.** It does not. That requires a
router or a VLAN.

**Assuming a Layer 3 switch is "not really a router".** It is, and the routing
misconfigurations of Chapter 65 §65.3 apply to it identically.

**Exceeding the repeater count on a legacy segment.** The 5-4-3 rule exists because
repeater latency eats the slot time, and violating it produces late collisions
(Chapter 66 §66.2) — which on a modern network almost always means duplex mismatch
instead, but the other cause is real.

> **Network+ note.** Objective 1.2 expects the device taxonomy — hub, switch,
> router, and their layers. The sentence to memorise verbatim is the bolded one
> above about collision and broadcast domains, and §17.3 is where the counting is
> practised.
