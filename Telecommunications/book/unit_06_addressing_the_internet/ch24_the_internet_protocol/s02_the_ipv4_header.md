# 24.2 The IPv4 Header

Twenty bytes, thirteen fields, unchanged since RFC 791 in September 1981. Every packet
you have ever sent carried this structure.

It repays reading field by field, because **each field is a decision**, and several of
them are decisions the designers would make differently now.

## The header

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-------+-------+---------------+-------------------------------+
|Version|  IHL  |Type of Service|          Total Length         |
+-------+-------+---------------+-----+-------------------------+
|         Identification        |Flags|     Fragment Offset     |
+---------------+---------------+-----+-------------------------+
|  Time to Live |    Protocol   |        Header Checksum        |
+---------------+---------------+-------------------------------+
|                       Source Address                          |
+---------------------------------------------------------------+
|                    Destination Address                        |
+---------------------------------------------------------------+
|                    Options (if IHL > 5)          |   Padding  |
+--------------------------------------------------+------------+
```

## Field by field

### Version — 4 bits

Value **4**. The first four bits of every IPv4 packet are `0100`.

Four bits allowed sixteen versions; IPv6 uses 6 (`0110`). Versions 1–3 were
experimental and never deployed, and version 5 was assigned to the Internet Stream
Protocol, an experiment that did not survive. **IPv6 is version 6 because 5 was taken**,
which is the entire reason for the name.

The version field is what lets a single interface carry both protocols — a receiver
reads the first nibble and dispatches accordingly.

### IHL — Internet Header Length, 4 bits

Header length in **32-bit words**. Minimum **5** (= 20 bytes), maximum **15** (= 60
bytes).

It exists because of options. Without options the header is fixed at 20 bytes and the
field is always 5 — which it is, in essentially all traffic.

**The 60-byte maximum is why IP options are nearly useless**: 40 bytes of options is
not much, and it constrained the design permanently. IPv6 replaced the mechanism with
extension headers precisely because of this.

### Type of Service / DSCP — 8 bits

The most-redefined field in the header.

| Era | Interpretation |
|---|---|
| 1981 (RFC 791) | 3-bit precedence + delay/throughput/reliability flags |
| 1998 (RFC 2474) | **6-bit DSCP** + 2 bits reserved |
| 2001 (RFC 3168) | 6-bit DSCP + **2-bit ECN** |

The current layout:

```
   ┌───────────────────────┬───────┐
   │        DSCP (6)       │ ECN(2)│
   └───────────────────────┴───────┘
```

**DSCP** — Differentiated Services Code Point — marks a class of service, and Chapter 52
covers what it means and how little it means without configuration. Common values:

| Name | DSCP | Typical use |
|---|---|---|
| Default (BE) | 0 | everything |
| AF31 | 26 | signalling |
| **EF** | **46** | **voice** |
| CS6 | 48 | network control |

**ECN** — Explicit Congestion Notification — lets a router signal congestion by
**marking** a packet rather than dropping it (Chapter 38 §38.3). It is one of the few
genuinely successful post-hoc additions to IP, and its deployment took twenty years
because middleboxes cleared the bits.

The original TOS interpretation was almost never implemented and is why this field spent
seventeen years as dead space.

### Total Length — 16 bits

Header **plus** payload, in bytes. Maximum **65,535**.

In practice constrained far below that by the link MTU (§24.3): 1500 bytes on Ethernet,
so a typical packet's total length is at most 1500.

Note this is *total*, unlike IHL which is in words. **Mixing the units is a standard
error** when reading a header by hand.

### Identification — 16 bits

Identifies a packet for **reassembly**: all fragments of one original packet carry the
same ID.

It has a second life as a fingerprinting signal. Some stacks increment it globally, some
per destination, some randomise it — and the behaviour identifies the operating system,
which `nmap -O` exploits. **A field intended for reassembly leaks the sender's identity**,
which is a recurring lesson about fields whose values are observable.

### Flags — 3 bits

```
   ┌───┬───┬───┐
   │ 0 │DF │MF │
   └───┴───┴───┘
```

| Bit | Name | Meaning |
|---|---|---|
| 0 | reserved | must be zero |
| 1 | **DF** — Don't Fragment | **do not fragment; drop and report instead** |
| 2 | **MF** — More Fragments | more fragments follow |

**DF is the important one.** It is how **path MTU discovery** works (§24.3): send with
DF set, and if a router must fragment it drops the packet and returns ICMP *Fragmentation
Needed*, reporting the MTU it could not exceed.

**Modern stacks set DF on essentially all TCP traffic.** Which makes the ICMP message
load-bearing, and means that filtering ICMP breaks large transfers in a way that looks
nothing like a firewall problem.

Bit 0 was reserved and is the subject of RFC 3514, *The Security Flag in the IPv4
Header* — Steve Bellovin's April Fools' RFC proposing that malicious packets set the
"evil bit". It is a joke with a serious point: **you cannot ask an attacker to declare
themselves**, and a surprising number of proposed security mechanisms amount to
exactly that.

### Fragment Offset — 13 bits

Where this fragment sits in the original packet, **in units of 8 bytes**.

The 8-byte granularity is why **every fragment except the last must be a multiple of 8
bytes** — 13 bits × 8 = 65,536, exactly covering the maximum packet size. A tight
encoding, and the source of the constraint.

### Time to Live — 8 bits

A **hop count**, despite the name. Decremented by one at each router; the packet is
dropped at zero and ICMP *Time Exceeded* is returned.

Originally intended as seconds — a router holding a packet for more than a second was
meant to decrement by more — which nobody implemented. §24.4 covers it properly.

Typical initial values: **64** (Linux, macOS), **128** (Windows), **255** (Cisco, and
network devices generally). The initial value is another operating-system fingerprint,
and it is why `ping` output lets you guess what you are talking to.

### Protocol — 8 bits

**The demultiplexing key** (Chapter 23 §23.3): which protocol is in the payload.

| Number | Protocol |
|---|---|
| **1** | ICMP |
| 2 | IGMP |
| **6** | **TCP** |
| **17** | **UDP** |
| 41 | IPv6 (in 6in4 tunnels) |
| 47 | GRE |
| 50 | ESP (IPsec) |
| 51 | AH (IPsec) |
| 89 | OSPF |

**1, 6 and 17 are examined and worth memorising.**

The 8-bit width — 256 possible protocols — is not the constraint people assume. The
binding constraint is that **middleboxes drop protocol numbers they do not recognise**,
which is why SCTP and DCCP failed to deploy and why QUIC hides inside UDP (Chapter 23
§23.2).

### Header Checksum — 16 bits

Covers **the header only**, not the payload.

Why only the header? Because the header **changes at every hop** — TTL is decremented —
so the checksum must be recomputed at every hop. Making it cover the payload would mean
recomputing over the whole packet at every router, which was unaffordable in 1981 and
unnecessary, since the transport layer checksums the payload end to end.

The algorithm is the **one's complement sum of 16-bit words**, chosen because it is
cheap, order-independent, and — crucially — **incrementally updatable**: a router
changing only the TTL can adjust the checksum with a few operations rather than
recomputing it.

**IPv6 removed the header checksum entirely**, on the reasoning that Layer 2 checks
frames and Layer 4 checks end to end, so the middle check is redundant. This is a clean
application of the end-to-end argument and it makes IPv6 forwarding cheaper.

### Source and Destination Address — 32 bits each

Chapters 25 and 26 are about these. Note here only that they are **not changed by
routers** — except by NAT (Chapter 33), which is the violation.

### Options — variable

Record Route, Timestamp, Loose and Strict Source Routing, Router Alert.

**Essentially all of them are dead.** Options are:

- Limited to 40 bytes by IHL
- **Processed in software** on most routers, so an optioned packet takes the slow path
- **Dropped outright** by many networks, for security — source routing in particular
  allows an attacker to specify a path, which defeats routing-based security assumptions

**Strict and loose source routing were deprecated by RFC 7126** for exactly that reason.

The practical consequence: **if you see IP options in production traffic, something
unusual is happening.** And the lesson — an extension mechanism that is slow and
filtered is an extension mechanism that does not exist — is what motivated IPv6's
extension header redesign, which has had its own difficulties.

## Reading a header by hand

A real packet, in hexadecimal:

```
45 00 00 3c 1c 46 40 00 40 06 b1 e6 c0 a8 00 68 c0 a8 00 01
```

| Bytes | Field | Value |
|---|---|---|
| `45` | Version, IHL | 4, and 5 words = **20 bytes** |
| `00` | DSCP/ECN | 0 — best effort |
| `00 3c` | Total length | **60 bytes** |
| `1c 46` | Identification | 7238 |
| `40 00` | Flags, offset | `010` = **DF set**, offset 0 |
| `40` | TTL | **64** |
| `06` | Protocol | **6 = TCP** |
| `b1 e6` | Checksum | — |
| `c0 a8 00 68` | Source | **192.168.0.104** |
| `c0 a8 00 01` | Destination | **192.168.0.1** |

Do this by hand once. It takes ten minutes and it converts the header from a diagram
into something you can read in a capture when the tools are not helping.

## What breaks here

**Confusing IHL's units with Total Length's.** IHL is 32-bit words; Total Length is
bytes.

**Assuming the checksum protects the payload.** It does not. Transport checksums do.

**Filtering all ICMP.** DF is set on nearly all TCP traffic, so path MTU discovery
depends on ICMP getting back. Filtering it breaks large transfers and nothing else,
which makes it very hard to diagnose (§24.3).

**Expecting IP options to work.** They are slow and widely dropped.

**Reading TTL as time.** It is a hop count.

> **Network+ note.** Objective 1.4 expects the IPv4 header's key fields. Over-learn:
> **TTL is a hop count**; **protocol numbers 1 ICMP, 6 TCP, 17 UDP**; **the checksum
> covers only the header**; **DF drives path MTU discovery**; **DSCP is the QoS field**.
> Also know the typical initial TTL values, which appear in "what OS is this?" questions.
