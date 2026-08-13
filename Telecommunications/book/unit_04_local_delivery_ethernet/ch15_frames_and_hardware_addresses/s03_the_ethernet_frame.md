# 15.3 The Ethernet Frame

Field by field, with the reason each exists. This is the structure you will decode
in every capture for the rest of your career.

## The layout

```
 ┌──────────┬─────┬──────────┬──────────┬───────────┬─────────────┬─────┐
 │ Preamble │ SFD │ Dest MAC │ Src MAC  │ EtherType │   Payload   │ FCS │
 │  7 bytes │  1  │ 6 bytes  │ 6 bytes  │  2 bytes  │ 46–1500 B   │  4  │
 └──────────┴─────┴──────────┴──────────┴───────────┴─────────────┴─────┘
 └──── not counted in ────┘  └──────────── the frame, 64–1518 bytes ────┘
      the frame length

 ← 12 bytes interframe gap → │ frame │ ← 12 bytes interframe gap →
```

**Total on the wire** for a maximum-size frame: 12 (gap) + 8 (preamble and SFD) +
1,518 (frame) = **1,538 bytes**, which is the figure Chapter 3 §3.1's efficiency
calculation uses.

## Preamble and start frame delimiter

Seven bytes of `10101010`, then one byte of `10101011`.

The alternating pattern gives the receiver's clock recovery circuit (Chapter 7 §7.1)
a known signal to lock onto before any real data arrives. Fifty-six bit times of
perfect alternation is ample for a phase-locked loop to acquire and stabilise.

The **SFD** ends `...11` rather than `...10`, and that final consecutive pair of ones
is the marker: everything after it is frame. The receiver has been synchronising on
alternation and now sees the pattern break, which is unambiguous.

Preamble and SFD are **not counted** in the frame length and are stripped by the
interface. You will not see them in a Wireshark capture, because the capture happens
above the point where they are removed — which surprises people expecting the byte
offsets to match the diagram.

## Destination address

Six bytes, and **first** in the frame. The ordering is deliberate and worth noting:
a receiver can decide whether a frame is for it after reading six bytes, rather than
after reading the whole thing. On a shared medium where every station sees every
frame, that matters — and it is what makes Chapter 17 §17.4's cut-through switching
possible, since a switch can begin forwarding after six bytes too.

Three cases:

- **Unicast** — one specific interface.
- **Multicast** — first byte odd (§15.2). Every station examines it; those subscribed
  to the group process it.
- **Broadcast** — `ff:ff:ff:ff:ff:ff`. Every station processes it, which is why
  broadcast domain size matters (Chapter 17 §17.3).

## Source address

Six bytes. Always unicast — a frame cannot originate from a group.

This field is what a switch **learns from** (Chapter 17 §17.2). It is also
trivially forgeable, which is the basis of MAC flooding and several other attacks in
Chapter 62.

## EtherType — or length

Two bytes, and this field has an interesting history that produces a rule worth
knowing.

**Ethernet II** (the DIX standard, 1982) uses these two bytes as a **type**
identifier saying what the payload is:

| Value | Payload |
|---|---|
| `0x0800` | IPv4 |
| `0x0806` | ARP |
| `0x86DD` | IPv6 |
| `0x8100` | 802.1Q VLAN tag |
| `0x8847` | MPLS unicast |
| `0x888E` | 802.1X EAPOL |

**IEEE 802.3** (1983) used the same two bytes as a **length** field, with the
protocol identified by an 802.2 LLC header inside the payload instead.

Both formats coexisted for years, so a receiver must distinguish them. The rule is
arithmetic:

> **If the value is ≤ 1500 (`0x05DC`), it is a length. If it is ≥ 1536 (`0x0600`),
> it is an EtherType.**

The gap between 1500 and 1536 is reserved to keep the two unambiguous, and every
assigned EtherType is above 1536 by construction.

In practice **Ethernet II won completely**. Essentially all modern traffic uses it,
and the 802.3 length format survives mainly in spanning tree BPDUs and a few legacy
protocols. When this book says "Ethernet frame" it means Ethernet II.

The EtherType is Chapter 2 §2.4's **self-describing framing** and Chapter 23 §23.3's
layering made physical: each layer's header contains a field naming the protocol of
the layer above, so the receiver knows how to interpret what follows without prior
arrangement.

## Payload

**46 to 1,500 bytes.**

The minimum of 46 follows from the 64-byte minimum frame: 64 − 6 − 6 − 2 − 4 = 46.
If the actual payload is shorter, **padding** is added, and the padding is
meaningless bytes that the receiver must discard.

How does the receiver know how much to discard? It does not, from the Ethernet
header — an Ethernet II frame carries no length field. **The upper-layer protocol
must supply it**, which is why the IP header has a Total Length field (Chapter 24
§24.2) and why a protocol without one cannot safely ride in a padded frame.

The maximum of 1,500 is the **MTU**, and Chapter 24 §24.3's fragmentation and
Chapter 66 §66.3's black holes are both consequences of it.

## Frame check sequence

Four bytes of CRC-32, computed over everything from the destination address through
the end of the payload. §15.4 covers it.

## Decoding a real frame

From Chapter 2 §2.3's dump:

```
0x0000:  0050 5601 2345 001b 4411 3ab7 0800 4500
0x0010:  003c 1c46 4000 4006 b1e6 c0a8 0a46 c0a8
0x0020:  0a01 dcbc 0050 4a1b 8f2c 0000 0000 a002
```

| Offset | Bytes | Field | Value |
|---|---|---|---|
| 0–5 | `0050 5601 2345` | Destination MAC | `00:50:56:01:23:45` |
| 6–11 | `001b 4411 3ab7` | Source MAC | `00:1b:44:11:3a:b7` |
| 12–13 | `0800` | EtherType | **IPv4** |
| 14 | `45` | IP version 4, IHL 5 (20 bytes) | |
| 22 | `40` | TTL | 64 |
| 23 | `06` | Protocol | **TCP** |
| 26–29 | `c0a8 0a46` | Source IP | 192.168.10.70 |
| 30–33 | `c0a8 0a01` | Destination IP | 192.168.10.1 |
| 34–35 | `dcbc` | Source port | 56508 |
| 36–37 | `0050` | Destination port | **80** |

Note the OUI `00:50:56` on the destination — that is VMware, so the destination is a
virtual machine. The source `00:1b:44` is a physical vendor. That observation costs
one lookup and frequently tells you something about the topology that no diagram
recorded.

Reading a frame by hand like this, once, is worth more than any amount of clicking
through Wireshark's decode pane, because it establishes that the tool is a
convenience rather than a translator.

## The frame, tagged

With an 802.1Q tag (Chapter 20 §20.2), four bytes are inserted **between the source
address and the EtherType**:

```
 ┌──────────┬──────────┬───────────────────────┬───────────┬─────────┬─────┐
 │ Dest MAC │ Src MAC  │  TPID 0x8100 │ TCI    │ EtherType │ Payload │ FCS │
 │ 6 bytes  │ 6 bytes  │   2 bytes    │ 2 B    │  2 bytes  │         │  4  │
 └──────────┴──────────┴───────────────────────┴───────────┴─────────┴─────┘
```

The TPID `0x8100` sits where the EtherType would be, which is how a receiver knows a
tag is present; the real EtherType follows the tag. The frame's maximum becomes
**1,522 bytes**, and equipment not expecting this reports **giants**.

## What breaks here

**Expecting the preamble in a capture.** It is stripped by the interface. Offsets in
a capture begin at the destination address.

**Assuming padding is data.** A protocol without its own length field cannot
distinguish payload from padding, which is why IP carries Total Length.

**A device rejecting 1,522-byte frames.** Unexpected 802.1Q tag at an administrative
boundary. Shows as giants or baby giants.

**Confusing EtherType with length.** The ≤1500/≥1536 rule resolves it, and modern
traffic is essentially all EtherType.

**Trusting the source address.** It is whatever the sender chose to write.

> **Network+ note.** Objective 1.4 expects the Ethernet frame structure and the
> common EtherTypes; objective 5.2 expects giants and runts. The field to know
> cold is the **EtherType**, because it is how you identify what a frame carries
> before decoding anything else, and `0800` / `0806` / `86DD` cover most of what you
> will meet.
