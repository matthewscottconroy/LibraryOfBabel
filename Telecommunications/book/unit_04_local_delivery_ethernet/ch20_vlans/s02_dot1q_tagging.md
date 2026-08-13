# 20.2 802.1Q Tagging

VLANs on one switch need no protocol — the switch simply consults its own port
configuration. The moment a VLAN spans two switches, a question appears that cannot be
answered by configuration alone.

## The problem

Two switches, both carrying VLANs 10 and 20, connected by one cable.

```
   SW-1                              SW-2
   [10][10][20][20]───── ? ─────[10][20][20][10]
```

A frame arrives at SW-2 across that cable. **Which VLAN is it in?**

SW-2 cannot tell. The frame is an ordinary Ethernet frame: destination, source, type,
payload (Chapter 15 §15.3). Nothing in it says anything about VLANs, because VLANs are
a property of the switch's configuration and configuration does not travel with
frames.

Three possible answers:

**One cable per VLAN.** Correct, and it does not scale: 30 VLANs means 30 cables
between every pair of switches, and 30 ports consumed on each. This is what was
actually done before 802.1Q, and it is why 802.1Q mattered.

**Infer from the source MAC.** Requires every switch to know every host's VLAN.
Fragile, and it fails for broadcasts and unknown addresses.

**Put the VLAN number in the frame.** Which is what 802.1Q does.

## The tag

IEEE 802.1Q (1998) inserts **four bytes** into the Ethernet header, between the source
address and the EtherType:

```
   Untagged frame:
   ┌──────────┬──────────┬──────────┬─────────────┬─────┐
   │ Dest MAC │ Src MAC  │ EtherType│   Payload   │ FCS │
   │    6     │    6     │    2     │  46 – 1500  │  4  │
   └──────────┴──────────┴──────────┴─────────────┴─────┘

   802.1Q-tagged frame:
   ┌──────────┬──────────┬═════════╦═══════════┬──────────┬─────────────┬─────┐
   │ Dest MAC │ Src MAC  ║  TPID   ║    TCI    │ EtherType│   Payload   │ FCS │
   │    6     │    6     ║ 0x8100  ║  2 bytes  │    2     │  46 – 1500  │  4  │
   └──────────┴──────────┴═════════╩═══════════┴──────────┴─────────────┴─────┘
                          └──────── 4-byte tag ─────────┘
```

**TPID — Tag Protocol Identifier, `0x8100`.** It sits where the EtherType normally
sits, and it is a *reserved* EtherType value. A receiver reading `0x8100` knows the
next two bytes are a tag and the real EtherType follows. **A device that does not
understand 802.1Q sees an unknown EtherType and discards the frame** — which is
important, and which §20.3's native VLAN exists to work around.

**TCI — Tag Control Information, 16 bits**, subdivided:

```
    15  13 12                              0
   ┌───────┬─┬─────────────────────────────┐
   │  PCP  │D│           VID               │
   │ 3 bits│1│         12 bits             │
   └───────┴─┴─────────────────────────────┘
```

| Field | Bits | Purpose |
|---|---|---|
| **PCP** — Priority Code Point | 3 | Class of service, 0–7. **802.1p.** |
| **DEI** — Drop Eligible Indicator | 1 | May be dropped preferentially under congestion. Originally CFI, meaning Token Ring encapsulation. |
| **VID** — VLAN Identifier | 12 | **The VLAN number, 0–4095** |

## The 12 bits, and their consequences

$$2^{12} = 4096$$

Two values are reserved:

| VID | Meaning |
|---|---|
| **0** | No VLAN — **the tag carries priority only** |
| 1–4094 | Usable VLAN identifiers |
| **4095** | Reserved |

So **4,094 usable VLANs**, and VID 1 is by convention the default VLAN, which by
convention should not be used for anything (§20.3).

**VID 0 is genuinely useful and often forgotten.** A priority-tagged frame carries PCP
but no VLAN membership, letting a device request a class of service without asserting
a VLAN. IP telephony uses it in some configurations.

**The 4,094 limit is the reason VXLAN exists.** A campus never approaches it. A cloud
provider with 50,000 tenants needing isolated networks exhausts it immediately, which
is why VXLAN's identifier is **24 bits — 16.7 million** (Chapter 67 §67.2). Whenever
you see a 12-bit field in a 1998 standard, expect a successor with a wider one.

## PCP — the priority field nobody uses correctly

Three bits, eight classes, and the IEEE's recommended assignment:

| PCP | Traffic type |
|---|---|
| 7 | Network control |
| 6 | Internetwork control |
| 5 | **Voice** (< 10 ms latency) |
| 4 | Video (< 100 ms latency) |
| 3 | Critical applications |
| 2 | Excellent effort |
| 0 | **Best effort (default)** |
| 1 | Background |

Note that **1 is lower than 0** — a historical accident from the original ordering,
and a small trap.

Two things matter about PCP in practice.

**It is Layer 2 only.** The tag is stripped at every routed hop, so PCP has no
end-to-end meaning across a routed network. IP's DSCP field (Chapter 24 §24.2,
Chapter 52 §52.2) is the Layer 3 equivalent, and the two must be explicitly mapped at
boundaries. **Marking traffic at Layer 2 and expecting it to survive routing is a
recurring and disappointing mistake.**

**A marking is a request, not a guarantee.** If no device is configured to act on PCP,
setting it changes nothing. Chapter 52 develops this: **quality of service is a
scheduling policy on congested interfaces, and marking is only the input to it.**

## The MTU consequence

Four extra bytes on a frame with a maximum payload of 1500 gives **1522 bytes** rather
than 1518.

Which was a real problem in 1998, because 1518 was the specified maximum and equipment
enforced it. Frames of 1522 bytes were counted as **"baby giants"** and dropped.

The resolution: **802.3ac (1998)** formally extended the maximum to 1522 for tagged
frames, and every switch built since accommodates it. But the issue reappears whenever
tags stack:

| Encapsulation | Overhead | Frame size |
|---|---|---|
| Untagged | — | 1518 |
| 802.1Q | 4 | 1522 |
| QinQ (802.1ad) | 8 | 1526 |
| VXLAN | 50 | 1568 |
| MPLS (2 labels) | 8 | 1526 |

**Every tunnel and every tag costs MTU**, and the resulting fragmentation and path-MTU
failures are one of the most persistent categories of network fault (Chapter 24 §24.3,
Chapter 66 §66.3). The standard operational answer in data centres is to configure
**jumbo frames (9000 bytes)** on the infrastructure so the overhead never matters.

## QinQ — 802.1ad

Service providers have a specific problem: a customer hands over already-tagged
traffic, using VLAN numbers the provider does not control and which may collide with
another customer's.

**802.1ad ("provider bridging", "QinQ", "stacked VLANs")** adds a *second* tag:

```
   ┌──────────┬──────────┬═══════════╦═══════════╦──────────┬─────────┐
   │ Dest MAC │ Src MAC  ║ outer tag ║ inner tag ║ EtherType│ Payload │
   │          │          ║  0x88a8   ║  0x8100   │          │         │
   └──────────┴──────────┴═══════════╩═══════════╩──────────┴─────────┘
                          provider     customer
                          (S-VLAN)     (C-VLAN)
```

The **outer (service) tag** identifies the customer to the provider; the **inner
(customer) tag** is the customer's own, carried transparently. The outer TPID is
`0x88a8` to distinguish it.

The provider needs 4,094 customer identifiers; each customer gets their own 4,094
internal VLANs. **The same trick as NAT** (Chapter 33): when an identifier space is
exhausted, add a level of hierarchy and reuse it in each scope.

## What breaks here

**Frames dropped as oversize between switches.** MTU not accounting for the tag. Look
for "giants" or "oversize" counters on the interface.

**A device connected to a trunk port receives nothing.** Most end devices do not
understand tags and discard them as unknown EtherTypes. This is what §20.3's native
VLAN was designed to accommodate — and what makes it a security problem.

**QoS marking that has no effect after a router.** PCP is Layer 2 and was stripped.
Mark with DSCP for anything that must cross a routed hop.

**"We need more than 4,094 VLANs."** You need VXLAN, or you need to reconsider the
design. Both are legitimate answers, and Chapter 66 covers the first.

**Wireshark shows a VLAN tag on some frames and not others.** Correct — captures taken
on a trunk show tags; captures on an access port do not, because the tag was stripped
on egress. **Where you capture determines what you see**, which is Chapter 64's
recurring theme.

> **Network+ note.** Objective 2.3 expects 802.1Q. Over-learn: **the tag is 4 bytes,
> inserted after the source MAC**; **TPID `0x8100`**; **12-bit VID gives 4,094 usable
> VLANs**; **3-bit PCP is 802.1p priority**; **tagged frames are 1522 bytes**. The
> VLAN count and the tag size are both examined directly.
