# Chapter 24 — Important Concepts

**Best-effort** *(§24.1)* — IP does not guarantee delivery, ordering, non-duplication,
timeliness, payload integrity, or notification of failure. It guarantees only that a
delivered packet's header passed its checksum and went to the address in it.

Dropping is normal operation *(§24.1)* — Congestion (the commonest by far), TTL
expiry, no route, header checksum failure, policy, buffer exhaustion. A network with
zero packet loss is underutilised, and TCP needs loss to find the available capacity.

Applications disagree about what they want *(§24.1)* — File transfer wants every
byte; live voice wants timeliness, and a retransmitted sample arrives after its
playout deadline — worse than useless. A network cannot serve both without serving one
badly, so it serves neither and lets each build what it needs.

Simple routers scale and survive *(§24.1)* — No per-flow state means a router can be
replaced mid-conversation, its memory scales with the routing table rather than the
traffic, forwarding can be a hardware table lookup, and there is no state to leak or
exhaust.

Graceful degradation is a property of promising little *(§24.1)* — A best-effort
network losing 2% still works. A guaranteed network that cannot guarantee has only bad
options. Under overload a telephone network blocks the call; the Internet slows down.

"Best-effort" is not an operational excuse *(§24.1)* — Loss on a **congested** link
is normal; loss on an **uncongested** link is a fault. Sustained loss above a fraction of
a per cent degrades TCP badly, and bursty loss hurts more than its average suggests.

**The IPv4 header** *(§24.2)* — 20 bytes minimum, 60 maximum, thirteen fields,
unchanged since RFC 791 (1981).

**Version and IHL** *(§24.2)* — Version 4 = `0100`; IPv6 is version 6 because 5 was
taken. IHL counts **32-bit words** (minimum 5 = 20 bytes), while Total Length counts
**bytes** — mixing the units is a standard error.

The 60-byte maximum killed IP options *(§24.2)* — Only 40 bytes available, processed
in software, and widely dropped for security (source routing especially). An extension
mechanism that is slow and filtered does not exist.

**DSCP and ECN** *(§24.2)* — The old Type of Service byte, redefined twice: **6-bit DSCP**
(EF = 46 is voice) plus **2-bit ECN**, which lets a router **mark** congestion rather
than dropping. ECN took twenty years to deploy because middleboxes cleared the bits.

**Identification leaks identity** *(§24.2)* — Intended for reassembly; its generation
pattern fingerprints the operating system. A recurring lesson about observable fields.

DF — Don't Fragment *(§24.2)* — Set on essentially all modern TCP traffic, and
the mechanism behind path MTU discovery. Which makes the corresponding ICMP message
load-bearing.

**The evil bit** *(§24.2)* — RFC 3514's joke with a serious point: you cannot ask an
attacker to declare themselves, and many proposed security mechanisms amount to
exactly that.

TTL is a hop count *(§24.2, §24.4)* — Despite the name. The temporal intent was
never implemented; IPv6 renamed it `Hop Limit`, which is what it always was. Initial
values: 64 Linux/macOS, 128 Windows, 255 network devices.

Protocol is the demultiplexing key *(§24.2)* — 1 ICMP, 6 TCP, 17 UDP, plus 41
IPv6-in-IPv4, 47 GRE, 50/51 IPsec, 89 OSPF. The binding constraint on new protocols is
not the 8-bit width but that middleboxes drop numbers they do not recognise — which
is why SCTP failed and QUIC hides in UDP.

The header checksum covers the header only *(§24.2)* — Because TTL changes every
hop, so it must be recomputed every hop. One's-complement sum, chosen for cheapness and
**incremental updatability**. IPv6 removed it entirely — Layer 2 checks frames,
Layer 4 checks end to end, so the middle check is redundant.

**MTU** *(§24.3)* — **Ethernet 1500**, PPPoE 1492, VXLAN 1450, WireGuard 1420, IPv6
minimum 1280, IPv4 minimum every host must accept **576**, jumbo 9000. Every tunnel
subtracts, and the modern Internet is full of tunnels.

**Fragmentation** *(§24.3)* — A router splits an oversized packet; offsets are in
8-byte units; reassembly happens only at the final destination.

Why fragmentation is a mistake *(§24.3)* — (1) Losing one fragment loses the whole
packet, so 1% link loss becomes 3% packet loss across three fragments; (2) reassembly
is state held on behalf of a possibly hostile sender; (3) it is an attack surface
(Teardrop, Ping of Death, fragment floods, firewall evasion, tiny fragments); (4) only
the first fragment has port numbers, so stateless filtering breaks; (5) it is a
slow-path operation; (6) reassembly is at the wrong place. RFC 8900 is the formal
verdict; IPv6 removed router fragmentation entirely.

**Path MTU Discovery** *(§24.3)* — Send with DF set; a router that cannot forward drops
and returns ICMP type 3 code 4 with the MTU. Elegant, and entirely dependent on
that ICMP arriving.

The PMTUD black hole *(§24.3)* — ICMP is blocked, the sender is never told, and
packets vanish. Symptom: small things work, large things hang — ping works, SSH
login works, `scp` hangs; the handshake completes and the first large segment does not.
It looks like an application problem, and people spend days on it.

**Diagnosing it** *(§24.3)* — `ping -M do -s 1472` and binary-search downward, or
`tracepath`. Fixes in order: stop blocking ICMP type 3 code 4; **MSS clamping** (a
layer violation, and what every VPN gateway does); lower the endpoint MTU; PLPMTUD.

**Tunnel overhead compounds** *(§24.3)* — VXLAN inside IPsec inside PPPoE leaves under
1400 bytes, each layer configured by someone unaware of the others. The data-centre
answer is jumbo frames on the underlay so encapsulation never drops the effective MTU
below 1500.

TTL makes loops survivable *(§24.4)* — Not prevented — **survivable**. A looping
packet dies within 255 hops instead of circulating forever, so a transient loop during
convergence is harmless. This one field is the fundamental difference between Layer 2
and Layer 3 forwarding, and it is what Perlman's route-don't-bridge argument is about.

TTL as a fingerprint *(§24.4)* — A reply with `ttl=56` most likely started at 64 and
crossed 8 hops, and is probably a Unix host. Free information in every reply.

**Traceroute** *(§24.4)* — Van Jacobson, 1987. Send deliberately small TTLs and
collect the errors: TTL=1 dies at hop 1 and its ICMP reveals hop 1's address, and so
on. Each hop is discovered by deliberately causing a failure.

**Reading traceroute correctly** *(§24.4)* — `* * *` means no reply, not no path —
usually ICMP rate-limiting, and traffic passes fine. Intermediate latency is control-
plane latency, not path latency, and a high middle hop with a low final hop means
nothing. ECMP means probes may take different paths. The reverse path is
invisible, which is why asymmetric routing is hard to see.

The probe types differ *(§24.4)* — Unix `traceroute` uses **UDP** (often filtered),
Windows `tracert` uses **ICMP**, `-I` uses ICMP, `-T` uses TCP SYN and gets through
firewalls. When traceroute shows nothing and connectivity works, use `-T`.

`mtr` is usually the better tool *(§24.4)* — Continuous probing with per-hop loss and
jitter, which distinguishes a genuinely lossy hop from one that rate-limits its own ICMP.
