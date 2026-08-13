# 24.3 Fragmentation and MTU

IP must carry packets across networks with different maximum frame sizes. It has a
mechanism for this, the mechanism is fragmentation, and **it is generally agreed to have
been a mistake** — one the Internet is still paying for forty years later.

This section covers what it is, why it fails, what replaced it, and how the replacement
breaks.

## MTU

**Maximum Transmission Unit** — the largest payload a link can carry in one frame.

| Link | MTU |
|---|---|
| Ethernet | **1500** |
| Ethernet with 802.1Q | 1500 (frame grows to 1522) |
| Ethernet jumbo frames | 9000 |
| PPPoE (DSL) | **1492** |
| IPsec tunnel | ~1400 (varies) |
| GRE tunnel | 1476 |
| **VXLAN** | **1450** |
| WireGuard | 1420 |
| IPv6 minimum | 1280 |
| IPv4 minimum every host must accept | **576** |
| Loopback | 65536 |

**1500 is the number.** It comes from the original Ethernet specification (Chapter 15
§15.3), where it was chosen as a compromise between efficiency and the buffer memory
a 1980 interface could afford, and it has become an effectively permanent constant of
the Internet.

**The problem is the second column.** Every tunnel subtracts from it, and the modern
Internet is full of tunnels.

## Fragmentation

When a packet is larger than the outgoing link's MTU, IPv4 lets a **router** split it.

**A 4000-byte packet onto a 1500-byte link:**

| Fragment | IP header | Payload | Offset | MF | Total |
|---|---|---|---|---|---|
| 1 | 20 | 1480 | 0 | **1** | 1500 |
| 2 | 20 | 1480 | **185** | **1** | 1500 |
| 3 | 20 | 1020 | **370** | 0 | 1040 |

**The offsets are in 8-byte units:** 1480 ÷ 8 = 185, 2960 ÷ 8 = 370. This is why every
fragment but the last must be a multiple of 8 bytes (§24.2).

**Reassembly happens only at the destination.** Not at the next router, not at the far
end of the constrained link — at the final destination. Which is what makes the
mechanism as costly as it is.

## Why it is a mistake

Six reasons, and they compound.

**1. Losing one fragment loses the whole packet.**

There is no partial delivery. Lose fragment 2 of 3 and the destination discards
fragments 1 and 3 as well, because it can never complete the packet.

**The effective loss rate is multiplied by the fragment count.** With independent 1% per
packet loss and three fragments:

$$P(\text{delivered}) = 0.99^3 = 0.970$$

**A 1% link loss becomes a 3% packet loss.** And TCP throughput degrades roughly with
the square root of loss (Chapter 38 §38.2), so this is worse than it sounds.

**2. Reassembly costs the destination.**

The receiver must buffer fragments, hold them until the set is complete or a timer
expires, and cope with fragments arriving out of order or never. **This is state, held
on behalf of a sender who may be hostile.**

**3. It is an attack surface.**

| Attack | Mechanism |
|---|---|
| **Teardrop** | Overlapping offsets crash the reassembly code |
| **Ping of Death** | Fragments reassembling to over 65,535 bytes overflow buffers |
| **Fragment flood** | Send first fragments only; the target buffers them until timeout |
| **Firewall evasion** | Split a signature across fragments so inspection misses it |
| **Tiny fragment** | Put the TCP header across a boundary so port-based rules cannot match |

The last two are why **many firewalls simply drop all fragments**, which is a reasonable
policy with an important consequence: **anything relying on fragmentation stops
working**, and the failure is silent.

**4. It breaks stateless filtering.**

Only the **first** fragment carries the TCP or UDP header, so only the first has port
numbers. A stateless firewall filtering on ports cannot classify fragments 2 and 3 — it
must either pass them blindly or hold reassembly state, which is exactly what a
stateless device was supposed to avoid.

**5. It costs routers.**

Fragmentation is a slow-path operation on most hardware, so a router forced to fragment
does it in software at a fraction of line rate.

**6. Reassembly is at the wrong place.**

Fragments are created at a constrained link and reassembled at the destination, so they
traverse **every subsequent hop as separate packets** — more headers, more per-packet
processing, more opportunities for loss.

**The verdict:** RFC 8900 (2020), *IP Fragmentation Considered Fragile*, is the formal
statement. **IPv6 removed router fragmentation entirely** — only the source may
fragment, and it must discover the path MTU first.

## Path MTU Discovery

The replacement, and the mechanism nearly all traffic uses.

**How it works:**

```
   1. Send a packet with DF set, sized for the local MTU (1500)
   2. A router whose next link is smaller cannot fragment (DF set)
   3. It DROPS the packet and returns
        ICMP Type 3, Code 4 — "Fragmentation Needed and DF Set"
        including the MTU it could not exceed
   4. The sender reduces its estimate and retransmits
   5. Repeat until packets get through
```

**Elegant, and it depends entirely on that ICMP message arriving.**

## The PMTUD black hole

The most consequential failure mode in this chapter, and one every network engineer
meets.

**Somebody blocks ICMP.** Often deliberately — "ICMP is a security risk" is a widely
repeated and largely wrong belief — and often as a side effect of a default-deny rule
that nobody thought about.

**The sequence:**

1. Sender transmits 1500-byte packets with DF set
2. A router downstream has a 1400-byte MTU
3. It drops the packets and sends ICMP Fragmentation Needed
4. **The ICMP is blocked and never arrives**
5. The sender keeps sending 1500-byte packets
6. They keep being dropped
7. **Nobody is told anything**

**The symptom is distinctive and misleading:**

| Works | Fails |
|---|---|
| `ping` | large file transfers |
| SSH login | `scp` of a large file |
| Small web pages | pages with images |
| The TCP handshake | the first large data segment |
| DNS queries | DNS responses over 1400 bytes (DNSSEC) |

**Small things work; large things hang.** And the connection *establishes* — the
handshake packets are small — so it looks like an application problem rather than a
network problem. People spend days on this.

**Diagnosis:**

```bash
# Find the largest packet that gets through.
# -M do sets DF; -s is payload size (add 28 for ICMP+IP headers)
ping -M do -s 1472 destination     # 1472 + 28 = 1500
ping -M do -s 1372 destination     # 1400
ping -M do -s 1272 destination     # 1300

# Or let the tool do it:
tracepath destination
```

If 1472 fails and 1372 succeeds, the path MTU is between 1400 and 1500, and the ICMP
that should have told you is being blocked.

**Fixes**, in order of preference:

1. **Stop blocking ICMP type 3 code 4.** The correct fix. RFC 4890 lists what must pass
   for IPv6; the IPv4 equivalent is at minimum this message.
2. **TCP MSS clamping** on the router: rewrite the MSS option in the TCP handshake so
   the endpoints negotiate a size that fits. Ugly — it is a layer violation (§21.4) — and
   it works without touching the endpoints, so it is what VPN gateways do universally.
   `ip tcp adjust-mss 1360` or the equivalent.
3. **Lower the MTU on the endpoints.** Works, and penalises all traffic including the
   traffic that did not need it.
4. **PLPMTUD** (RFC 4821): probe for the path MTU using the transport itself, without
   depending on ICMP at all. The robust answer, and slow to deploy.

## Where MTU problems come from

Overwhelmingly **tunnels**, because every tunnel adds a header:

| Tunnel | Overhead | Resulting MTU |
|---|---|---|
| GRE | 24 | 1476 |
| IPsec transport | ~30–40 | ~1460 |
| IPsec tunnel (ESP) | ~50–60 | ~1440 |
| **VXLAN** | **50** | **1450** |
| WireGuard | 60 | 1420 |
| GRE **over** IPsec | ~75 | ~1425 |
| PPPoE | 8 | 1492 |

**Nesting compounds.** A VXLAN inside an IPsec tunnel inside PPPoE leaves under 1400
bytes, and every layer was configured by someone who did not know about the others.

**The data-centre answer** is to run **jumbo frames (9000)** on the underlay, so that a
1500-byte tenant packet plus 50 bytes of VXLAN still fits comfortably. This is why jumbo
frames matter operationally: not for the efficiency of large transfers but so that
encapsulation never reduces the effective MTU below 1500.

## What breaks here

**Large transfers hang; small ones work.** PMTUD black hole. Test with
`ping -M do -s`.

**A VPN where you can log in but not transfer files.** Same thing, and the reason MSS
clamping is standard on VPN gateways.

**DNSSEC responses failing while ordinary DNS works.** DNSSEC responses exceed 1400
bytes and are usually fragmented or rejected. Chapter 39 §39.4.

**Fragments dropped by a firewall.** Reasonable policy, and anything depending on
fragmentation stops.

**MTU mismatch between two ends of a link.** One side sends 9000-byte frames the other
counts as giants. Both ends of a link must agree, and this is a very common jumbo-frame
mistake.

> **Network+ note.** Objective 5.2 expects MTU and fragmentation as a troubleshooting
> topic. Over-learn: **Ethernet MTU is 1500**; **DF triggers PMTUD**; **blocking ICMP
> breaks PMTUD and produces the small-works-large-hangs symptom**; **MSS clamping is
> the workaround**; and **jumbo frames must match at both ends**. The black-hole
> scenario appears on the exam in almost exactly the form described here.
