# 29.1 The Forwarding Decision

A router does one thing. Everything else it does — running protocols, holding tables,
applying policy — exists to support this one thing, and stating it precisely is the whole
of this section.

> **Given a packet, choose the interface to send it out of, and the next device to
> address it to.**

That is it. Repeat several billion times per second and you have the Internet.

## The decision, in full

```
   A packet arrives.

   1. Read the destination IP address from the header.
   2. Find every route in the table whose prefix contains that address.
   3. Of those, choose the one with the LONGEST prefix.
   4. That route names an outgoing interface and a next hop.
   5. Decrement TTL. If it reaches zero, drop it and send ICMP Time Exceeded.
   6. Recompute the header checksum.
   7. Resolve the next hop's link-layer address (ARP / NDP).
   8. Build a new frame and transmit it.
```

Eight steps, and steps 2 and 3 are the interesting ones. §29.3 is about them.

## What the router does not do

The list is longer than the list of what it does, and each omission is a decision that
was made deliberately (Chapter 23 §23.4).

| The router does **not** | Because |
|---|---|
| Look at the source address | Forwarding is destination-based. **The source is irrelevant to where a packet goes.** |
| Look inside the payload | That is the endpoints' business — and a firewall's, which is why a firewall is not a router |
| Remember the packet | No per-flow state (Chapter 23 §23.1) |
| Know whether the packet arrives | Best-effort (Chapter 24 §24.1) |
| Know the whole path | **Only the next hop** |
| Guarantee the reply takes the same path | It frequently does not |

**Two of those deserve development.**

### Forwarding is destination-based only

The source address plays **no part** in the forwarding decision. A packet from
`10.1.1.1` to `8.8.8.8` and a packet from `192.0.2.99` to `8.8.8.8` take **exactly the
same path**, hop for hop.

This has consequences that run through the rest of the book:

- **Source spoofing works.** Nothing in normal forwarding checks that a source address is
  plausible, which is why BCP 38 (Chapter 27 §27.2) has to be configured deliberately and
  why reflection attacks are possible (Chapter 62).
- **Return traffic is a separate decision**, made by different routers with different
  tables — hence asymmetric routing.
- **Policy-based routing**, which *does* consider the source, is an explicit exception
  that must be configured, is handled outside the fast path on many platforms, and is
  used sparingly for exactly that reason.

### A router knows only the next hop

**This is the property that makes the Internet possible**, and it is worth stating
starkly.

No router anywhere knows the path a packet will take. Each knows only *"for this
destination, hand it to that neighbour"*. The path is not planned, not stored, not
agreed — it **emerges** from a chain of independent local decisions, each made by a
device that has no idea what the previous or next one decided.

```
   Host ──▶ R1 ──▶ R2 ──▶ R3 ──▶ R4 ──▶ Server

   R1 knows: "for 203.0.113.0/24, send to R2"
   R2 knows: "for 203.0.113.0/24, send to R3"
   R3 knows: "for 203.0.113.0/24, send to R4"

   Nobody knows the whole path. It is never written down anywhere.
```

Chapter 12's telephone network worked the opposite way: a circuit was **established end
to end before any data moved**, and the path was known, held as state at every switch,
and torn down afterwards. Hop-by-hop forwarding trades that certainty for the ability to
reroute instantly around a failure, with no signalling, because the next hop is the only
commitment anyone has made.

**It also means traceroute (Chapter 34 §34.3) is genuinely doing detective work**, and
that a path can change mid-conversation with nothing to announce it.

## Control plane and data plane

The distinction that organises everything in Unit VII.

| | Control plane | Data plane |
|---|---|---|
| **Job** | decide *what the table should say* | *use* the table |
| Runs | routing protocols, management | forwarding |
| Speed | seconds | **nanoseconds** |
| Implemented in | software, on a CPU | **hardware — ASIC or TCAM** |
| Frequency | on topology change | **every packet** |
| If it fails | the table goes stale | **traffic stops** |

**They operate on wildly different timescales**, and that separation is what makes a
router both flexible and fast: the complicated, slow, changeable part runs in software on
a general-purpose CPU, and the simple, fast, unchanging part runs in silicon.

**The practical consequences appear constantly:**

- **A router's CPU can be at 100% while forwarding is unaffected**, because forwarding
  is not on the CPU. Conversely, a device forwarding in software has no such separation
  and will drop traffic under control-plane load.
- **`ping` to a router may be slow while traffic through it is fast.** Replying to ping
  is a control-plane task with low priority; forwarding is hardware. This is exactly
  Chapter 24 §24.4's traceroute observation, and it is why intermediate-hop latency means
  nothing.
- **"Routing table" and "forwarding table" are different objects.** The RIB (Routing
  Information Base) is the control plane's full view, including routes it did not choose.
  The FIB (Forwarding Information Base) is the distilled result that hardware uses.

```
   OSPF ──┐
   BGP  ──┼──▶  RIB  ──(best routes only)──▶  FIB  ──▶  hardware
   static ┘     (control plane)                (data plane)
```

`show ip route` shows the RIB. `show ip cef` — or the platform's equivalent — shows the
FIB. **When they disagree, forwarding follows the FIB**, and a mismatch between them is a
real and confusing class of fault.

**Chapter 68's software-defined networking is this separation taken to its conclusion:**
if the control plane is already logically distinct, move it off the device entirely and
give it a view of the whole network rather than one node's.

## Every host is a router

Not a metaphor. **Your laptop performs exactly the algorithm above** for every packet it
sends.

```
$ ip route
default via 192.168.1.1 dev wlan0
192.168.1.0/24 dev wlan0 proto kernel scope link src 192.168.1.50
```

Two routes. Sending to `192.168.1.99`: the second matches, `scope link` means directly
connected, so ARP for the destination and send.

Sending to `8.8.8.8`: only `default` matches, so ARP for `192.168.1.1` and send there.

**This is Chapter 18 §18.1's local-or-remote decision**, and it is now visible as what it
always was: **a routing table lookup**. The mask arithmetic of Chapter 25 §25.3 is the
same longest-prefix match a core router performs, with two entries instead of a million.

**The only difference between a host and a router** is that a host does not forward
packets **not addressed to it**. Enable `net.ipv4.ip_forward` and the difference
disappears — which is precisely how a Linux box becomes a router.

## Speed

The numbers are worth knowing because they explain why the design is what it is.

A 100 Gb/s interface at minimum packet size:

$$\frac{100 \times 10^9}{64 \times 8 + 160} \approx 148.8 \text{ million packets per second}$$

**148.8 Mpps means 6.7 nanoseconds per packet.**

In 6.7 ns you can perform roughly **twenty instructions** on a fast CPU. You cannot
search a million-entry table, cannot take an interrupt, cannot allocate memory.

**So forwarding is not done on a CPU.** It is done in **TCAM** — ternary content-
addressable memory — which compares a key against **every entry simultaneously** and
returns the longest match in a single lookup, in constant time regardless of table size.

TCAM is why longest-prefix match is feasible at all, and its properties shape the whole
of Internet routing:

| TCAM property | Consequence |
|---|---|
| Constant-time lookup | table size does not affect speed |
| **Very expensive** | table size is limited by cost |
| **Power-hungry** | every cell is active on every lookup |
| Physically limited | a platform has a hard entry count |

**This is why aggregation matters** (Chapter 26 §26.3). The global routing table's
~950,000 entries must fit in TCAM. A router whose TCAM is exceeded does not slow down
gracefully — it **falls back to software forwarding for the overflow, or drops the
routes**, and Chapter 32 §32.4 covers the several occasions this has taken large parts of
the Internet offline.

## What breaks here

**Assuming the router looks at the source.** It does not. This surprises people
debugging asymmetric routing and people expecting return traffic to follow the forward
path.

**Assuming the path is symmetric.** Forward and return are independent decisions by
different routers.

**Reading control-plane symptoms as data-plane problems.** High CPU, slow ping response
to the router itself, and slow ICMP generation are all control plane. Traffic *through*
the device may be perfect.

**Confusing the RIB with the FIB.** `show ip route` is not what the hardware uses.

**Expecting a router to know the path.** It knows one hop. Nothing knows the path.

> **Network+ note.** Objective 2.2 expects routing concepts and objective 1.2 expects the
> router as a device. Over-learn: **forwarding is destination-based**; **a router knows
> only the next hop**; and **control plane decides, data plane forwards**. The last
> distinction explains several troubleshooting scenarios that otherwise look
> contradictory.
