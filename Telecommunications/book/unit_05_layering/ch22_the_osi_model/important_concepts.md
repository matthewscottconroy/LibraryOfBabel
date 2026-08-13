# Chapter 22 — Important Concepts

**The most successful failure in computing** *(§22.1)* — OSI's protocols were
exhaustively specified, government-mandated, vendor-backed, and are used by essentially
nobody. **Its model is on every network operations centre wall.**

**What OSI was for** *(§22.1)* — Every manufacturer had its own incompatible
networking (SNA, DECnet, and the rest), and **the network was the vendor lock-in**. The
word *open* in the name is the entire political programme.

**Seven is negotiated, not derived** *(§22.1)* — The committee's principles for placing
a boundary were sensible and do not yield a number. **Layers 5 and 6 exist because they
were somebody's requirement.** Knowing this prevents treating seven as though it meant
something.

**What the committee got right** *(§22.1)* — **Layers 1–3 are excellent** and have no
serious alternative; the **service/protocol distinction**; **the vocabulary**, which is
how the whole industry speaks; and **connectionless service**, added after argument, of
which IS-IS survives.

**What it got wrong** *(§22.1)* — **Slow** (design by international committee);
**specified before implementing** (TCP/IP was implemented, found wanting, fixed, then
specified); **complex** (~1,000 pages against RFC 791+793's 130); **expensive** (ISO
standards cost money, **RFCs are free**); and **late**.

**GOSIP** *(§22.1)* — The US government mandated OSI procurement in 1990 and withdrew
it in 1995. **A government mandate, universal vendor support and a thorough
specification lost to free software that already worked** — a pattern that recurs with
SEND, with IPsec versus TLS, and with IPv6.

**What survived** *(§22.1)* — The **vocabulary**; **IS-IS**; **X.500/LDAP**;
**ASN.1**; and **X.509 — used by every TLS connection on the Internet**. The protocols
lost; the data formats and naming work are everywhere.

**Why still teach it** *(§22.1)* — It is the vocabulary; it is a **diagnostic
instrument** (§22.4); layers 1–3 are correct; and it is examined.

**Layer 1 — Physical** *(§22.2)* — Bits (or symbols) on a medium. Voltage, line coding,
modulation, connectors, distance, attenuation, clock recovery. Devices: cables, hubs,
repeaters, transceivers, media converters. **Layer 1 has no idea what the bits mean.**
**Its faults are the most common and the cheapest to check.**

**Layer 2 — Data Link** *(§22.2)* — Frames **on one segment**. Framing, MAC addressing,
FCS **detection** (not correction), medium access, switching, VLANs, spanning tree,
aggregation. Devices: switches, bridges, NICs, access points. Sublayers: **LLC (802.2)**
and **MAC (802.3/802.11)**. **It has no concept of a network beyond the segment.**

**Layer 3 — Network** *(§22.2)* — Packets **between networks**. IP addressing,
subnetting, routing, longest-prefix match, TTL, fragmentation, ICMP, NAT. Devices:
routers, Layer 3 switches. **Hierarchical, therefore aggregatable — and best-effort by
design.**

**One packet, many frames** *(§22.2)* — Layer 1 moves bits along a wire; Layer 2
delivers to a device on that wire; Layer 3 crosses many wires using Layer 2 on each in
turn. **Layer 3 is end-to-end, Layer 2 is hop-by-hop.**

**The address table** *(§22.2)* — MAC: one link, flat, **changes every hop**. IP:
global, hierarchical, **never changes**. Port: one host, flat. Memorise it; it answers
most "which layer" questions.

**The awkward cases** *(§22.2)* — ARP (2.5), MPLS (2.5, for a different reason), ICMP
(3, inside IP), firewalls (3–7), NAT (3 **and** 4). **They are awkward because the
model is a description, not a constraint**, and explaining why something does not fit is
better understanding than assigning it a number.

**Layer 4 — Transport** *(§22.3)* — Process-to-process. Ports, connections,
reliability, ordering, flow control, congestion control. **Layer 3 gets a packet to a
host; Layer 4 gets it to a process** — without ports, one network application per
machine. **This is the last layer where the model holds cleanly.**

**Layer 5 — Session** *(§22.3)* — Describes a real category of function that **was
never separated into a real layer**. The functions live in TCP or in applications. Exam
answers: NetBIOS, RPC, SMB, PPTP.

**Layer 6 — Presentation** *(§22.3)* — Encoding, encryption, compression. **The
historical problem was genuine** (EBCDIC to ASCII, endianness) and **the world solved
it by converging on Unicode, IEEE 754 and network byte order** rather than by
translating in the middle. Exam answers: TLS, JPEG, MPEG, ASCII.

**TLS does not fit** *(§22.3)* — Above TCP, below HTTP, encrypts (6), negotiates a
session (5), authenticates (nowhere), and **QUIC merges it into transport entirely**.
**Answer 6 on the exam and understand why the question is unanswerable.**

**Layer 7 is not the application** *(§22.3)* — Your browser is a program; **HTTP is
Layer 7**. The layer is the protocol that lets two applications communicate, not the
software with the buttons.

**The honest assessment** *(§22.3)* — **Layers 1–4 are real**: distinct
implementations, distinct headers, distinct devices. **Layers 5–7 are one layer in
practice**, which is exactly what the TCP/IP model says.

**"Layer 7" and "Layer 8"** *(§22.3)* — "Layer 7 firewall" means something precise and
useful: a device acting on **application protocol content**. "Layer 8" is the user, and
the joke persists because the diagnosis is so often correct.

**The diagnostic value** *(§22.4)* — **The model converts one unbounded problem into
seven bounded ones in a fixed order.** That is its entire diagnostic worth, and it is
enormous.

**Bottom-up** *(§22.4)* — Start at Layer 1, do not skip. Thorough. Use it when the
problem is new, when physical work happened recently, or when you are unfamiliar with
the system.

**`arping` is the Layer 2 key** *(§22.4)* — No IP, no ICMP, rarely filtered. **If
`arping` succeeds and `ping` fails, the problem is proved to be above Layer 2** —
eliminating cabling, switching, VLANs and spanning tree in one command.

**Ping the gateway first** *(§22.4)* — It separates "my segment is broken" from "the
world beyond is broken", and those have almost disjoint cause sets.

**RST versus silence** *(§22.4)* — **RST = the host is up and nothing is listening.
Silence = a firewall is dropping**, or the route is asymmetric. Very different problems,
distinguished by one observation.

**Divide-and-conquer** *(§22.4)* — Start at Layer 3; success sends you up, failure sends
you down. **log₂7 ≈ 3 tests instead of 7**, and the advantage grows with the problem
space.

**Top-down** *(§22.4)* — Right when the symptom is application-specific. **What the
working things prove is the most underused information in troubleshooting**: if the
user's email works, everything from the cable to Layer 4 is proven.

**The property that makes it work** *(§22.4)* — **A successful test at layer *n* proves
layers 1 through *n* are functioning.** One command, a large amount of information.

**Check DNS early** *(§22.4)* — A large share of "the network is broken". `ping 8.8.8.8`
succeeding while `ping google.com` fails identifies it in two commands.

**Check certificate expiry** *(§22.4)* — A scheduled outage nobody scheduled. "Worked
yesterday, nothing changed" matches almost nothing else.

**Change one thing at a time** *(§22.4)* — Otherwise you fix it without learning what
was wrong, which guarantees a recurrence.
