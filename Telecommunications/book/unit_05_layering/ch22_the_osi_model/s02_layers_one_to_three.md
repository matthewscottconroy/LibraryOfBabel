# 22.2 Layers One to Three

This section contains no new technical material. Everything in it was built in Units
I–IV. The purpose is to attach the standard names to what you already have, which is
the correct order and the opposite of how the subject is usually taught.

## Layer 1 — Physical

**What it does:** move bits between two directly connected points.

**Unit:** the **bit**. (Sometimes "symbol" — Chapter 6 §6.2 explains why they are not
the same thing, and why the distinction matters for every rate calculation.)

**Everything at Layer 1:**

| Concern | Chapter |
|---|---|
| Voltage, current, optical power levels | 5, 8 |
| Line coding — NRZ, Manchester, 4B/5B, PAM-4 | 6 |
| Modulation — ASK, FSK, PSK, QAM, OFDM | 7 |
| Connectors, pinouts, cable specifications | 8 |
| Bit rate, symbol rate, baud | 6 |
| Attenuation, noise, interference, crosstalk | 5, 8 |
| Distance limits | 8, 16 |
| Clock recovery and synchronisation | 6 |

**Devices:** cables, connectors, repeaters, hubs, transceivers, media converters,
patch panels, antennas.

**Key idea:** Layer 1 has **no idea what the bits mean**. It carries a stream of
symbols. Chapter 6's line codes exist to make the stream recoverable — self-clocking,
DC-balanced — not to make it meaningful.

**Its faults:** cable damage, wrong cable type, exceeded distance, bad connector,
interference, dirty fibre, failed transceiver, no power. **Layer 1 faults are the most
common faults in networking and the cheapest to check**, which is why §22.4's method
starts there.

## Layer 2 — Data Link

**What it does:** deliver frames between devices **on the same network segment**.

**Unit:** the **frame**.

**Everything at Layer 2:**

| Concern | Chapter |
|---|---|
| Framing — where a frame starts and ends | 15 |
| MAC addressing, 48-bit, flat | 15 |
| Error **detection** — the FCS, CRC-32 | 15 |
| Medium access control — CSMA/CD, CSMA/CA | 9, 10 |
| Switching, learning, flooding | 17 |
| Address resolution (ARP, awkwardly) | 18 |
| Loop prevention — spanning tree | 19 |
| VLANs and 802.1Q tagging | 20 |
| Link aggregation | 19 |
| Flow control — 802.3x PAUSE | 16 |

**Devices:** switches, bridges, network interface cards, wireless access points.

**The two sublayers** — an IEEE 802 refinement worth knowing because it is examined:

| Sublayer | Standard | Function |
|---|---|---|
| **LLC** — Logical Link Control | 802.2 | Interface to Layer 3; identifies the carried protocol |
| **MAC** — Media Access Control | 802.3, 802.11 | Addressing and access to the medium |

The split let one LLC serve Ethernet, Token Ring, FDDI and Wi-Fi alike. In practice
Ethernet II framing with an EtherType displaced LLC for almost everything (Chapter 15
§15.3), so LLC survives mainly in 802.11 and in exam questions.

**Key idea:** Layer 2 delivers **on one link**. It has no concept of a network beyond
the segment, no routing, and — critically — **no way to reach anything a frame cannot
be addressed to directly**. That limitation is what Layer 3 exists to remove.

**Its faults:** duplex mismatch, wrong VLAN, spanning-tree blocking, MAC table
problems, ARP failures, switching loops, frame errors from a Layer 1 problem.

## Layer 3 — Network

**What it does:** deliver packets **between networks**, across intermediate devices.

**Unit:** the **packet**.

**Everything at Layer 3:**

| Concern | Chapter |
|---|---|
| IP addressing, hierarchical | 24, 25 |
| Subnetting, CIDR, VLSM | 26 |
| Routing and forwarding | 30, 31, 32 |
| Longest-prefix match | 30 |
| TTL and loop mitigation | 24 |
| Fragmentation and reassembly | 24 |
| ICMP | 33 |
| NAT | 33 |
| IPv6 | 28, 29 |

**Devices:** routers, Layer 3 switches, firewalls (in part).

**Key idea:** Layer 3 provides the **end-to-end** addressing that Layer 2 cannot,
because IP addresses are hierarchical and can therefore be aggregated (Chapter 18
§18.1). It is also **best-effort**: no delivery guarantee, no ordering guarantee, no
duplicate suppression. Those are Layer 4's problem, deliberately (Chapter 23 §23.4).

**Its faults:** wrong IP address, wrong subnet mask, wrong or missing default gateway,
missing route, routing loop, MTU and fragmentation problems, NAT misbehaviour.

## The relationship, in one paragraph

Layer 1 moves bits along a wire. Layer 2 groups them into frames and delivers them to a
device **on that wire**. Layer 3 carries packets **across many wires**, using Layer 2
on each one in turn.

Which is exactly the traced example of Chapter 18 §18.1: **one packet, many frames**.
The IP header rides unchanged from source to destination while a new Layer 2 header is
built and discarded at every hop. **Layer 3 is end-to-end; Layer 2 is hop-by-hop.**

## The address correspondence

The single most useful table in the model, and worth memorising:

| Layer | Address | Scope | Structure | Changes per hop? |
|---|---|---|---|---|
| 2 | MAC | one link | flat | **yes** |
| 3 | IP | global | hierarchical | **no** |
| 4 | port | one host | flat | no |

Every fact in that table has been derived in an earlier chapter. Together they answer
most questions about which layer is responsible for what.

## Where things sit — the awkward cases

| Thing | Layer | Note |
|---|---|---|
| Hub, repeater | **1** | no decisions, no addresses |
| Switch | **2** | MAC addresses |
| Router | **3** | IP addresses |
| Layer 3 switch | **3** | a router in switching silicon |
| Wireless access point | **2** | a switch with a radio; the radio is Layer 1 |
| **ARP** | **2.5** | resolves L3 addresses, carried in L2 frames |
| **ICMP** | **3** | inside IP, but part of IP's control plane |
| **MPLS** | **2.5** | between the frame header and the packet header |
| Firewall | **3–7** | depends entirely on the firewall |
| Load balancer | **4 or 7** | depends entirely on the load balancer |
| **NAT** | **3, and 4** | rewrites addresses *and* ports |

The awkward rows are §21.4's subject. **They are awkward because the model is a
description, not a constraint** — and being able to say *why* something does not fit is
better understanding than being able to assign it a number.

## What breaks here

**Confusing Layer 2 and Layer 3 addressing.** The commonest conceptual error in the
subject. Reread the address table until it is automatic.

**Assuming a switch does Layer 3 things.** A plain switch does not route, does not
break broadcast domains, and does not know what an IP address is.

**Placing ARP.** It does not fit. Say so.

**Assuming Layer 3 provides reliability.** It provides none, deliberately.

**Assuming a wireless access point is Layer 1** because radio is Layer 1. The radio is
Layer 1; the access point makes forwarding decisions on MAC addresses and is Layer 2.

> **Network+ note.** Objective 1.1 examines this constantly, most often as "at which
> layer does device X operate?" or "at which layer does protocol Y operate?" Over-learn
> the device table and the address table. The three most-missed items are **the access
> point (Layer 2, not 1)**, **the Layer 3 switch (Layer 3, despite the name)**, and
> **ARP (does not fit; answer Layer 2 if forced)**.
