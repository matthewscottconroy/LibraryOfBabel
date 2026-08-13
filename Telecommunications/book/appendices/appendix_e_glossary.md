# Appendix E — Glossary

Every marked term in the book, with the chapter that introduces it. Where a term has a
common misuse, the misuse is noted, because the misuses cause real confusion.

---

**AAA** — Authentication, Authorization, Accounting. Three distinct functions routinely
conflated. *(Ch 59)*

**Administrative distance** — A number expressing how much a router trusts a route's
*source*. Chooses between protocols; **metric** chooses within one. *(Ch 30)*

**Anycast** — One address advertised from many locations; routing delivers to the
nearest. A routing technique, not an addressing feature. *(Ch 27, 52)*

**APIPA** — 169.254.0.0/16 link-local addressing, self-assigned when DHCP fails. Seeing
one is a diagnosis, not a configuration. *(Ch 27, 40)*

**ARP** — Address Resolution Protocol. Maps an IPv4 address to a MAC address on the
local segment. No authentication, by design. *(Ch 18)*

**Attenuation** — Loss of signal power over distance, measured in dB. Frequency-
dependent on copper. *(Ch 6)*

**Autonomous system (AS)** — A set of networks under one routing policy. An
administrative, not technical, boundary. *(Ch 32)*

**Bandwidth** — Physically, the width in hertz of a channel's passband. Colloquially,
a link's maximum data rate. This book uses **capacity** when the distinction matters.
*(Ch 3, 4)*

**Bandwidth–delay product (BDP)** — Capacity × RTT: the data in flight on a saturated
path. Determines the window a transport needs. *(Ch 3)*

**Baud** — Symbols per second. Equal to bits per second only with two symbols. Almost
always misused. *(Ch 4)*

**Best-effort** — IP's service model: attempt delivery, promise nothing. A deliberate
choice, not a limitation. *(Ch 24)*

**BGP** — Border Gateway Protocol. Path-vector routing between autonomous systems;
policy rather than shortest path. *(Ch 32)*

**Broadcast domain** — The set of interfaces receiving a broadcast from any of them.
Bounded by a router or a VLAN, **not** by a switch. *(Ch 17, 20)*

**Bufferbloat** — Excessive latency caused by oversized buffers hiding congestion from
TCP's loss signal. *(Ch 66)*

**CIDR** — Classless Inter-Domain Routing. Prefixes on arbitrary bit boundaries, with
aggregation. Abolished classful addressing in 1993. *(Ch 26)*

**Collision domain** — The set of interfaces that can collide. One per switch port on a
modern network. *(Ch 16, 17)*

**Congestion control** — Protects the **network** from a sender. Distinct from flow
control, which protects the **receiver**. *(Ch 38)*

**Control plane / data plane** — Deciding what the forwarding table contains, versus
using it. Software and hardware, seconds and nanoseconds. *(Ch 29, 68)*

**CRC** — Cyclic Redundancy Check. Ethernet's error *detection*; a failed frame is
discarded silently, not corrected. *(Ch 15)*

**CSMA/CA** — Collision avoidance, used by Wi-Fi because a radio cannot detect
collisions while transmitting. *(Ch 44)*

**CSMA/CD** — Collision detection, used by shared-medium Ethernet. Obsolete on
full-duplex links. *(Ch 16)*

**dB / dBm** — A power ratio in logarithmic form / absolute power referenced to 1 mW.
dBm + dB = dBm. *(Ch 4)*

**DHCP** — Dynamic Host Configuration Protocol. DORA, plus an options catalogue that
quietly configures the enterprise. *(Ch 40)*

**DNS** — Domain Name System. Hierarchy, delegation, caching. Cause of a
disproportionate share of outages that initially look like something else. *(Ch 39)*

**DSCP** — Differentiated Services Code Point. Six bits of the IPv4 ToS byte carrying
QoS classification. *(Ch 24, 52)*

**Duplex mismatch** — One end full duplex, the other half. Link works; throughput
collapses under load; late collisions in the counters. *(Ch 16, 66)*

**DWDM** — Dense Wavelength Division Multiplexing. 80–96 wavelengths on one fibre pair.
*(Ch 9, 50)*

**Encapsulation** — Wrapping a layer's PDU as the payload of the layer below. *(Ch 23)*

**End-to-end argument** — Functions belong at the endpoints; in the network only as a
performance optimisation. The Internet's constitutional principle. *(Ch 23)*

**Entropy** — Average information per symbol, −Σ p log₂ p. Bounds lossless compression.
*(Ch 4)*

**EtherType** — The Ethernet field naming the payload's protocol. Self-describing
framing; the physical implementation of layering. *(Ch 15, 23)*

**EVPN** — BGP-based control plane for VXLAN overlays, replacing flood-and-learn.
*(Ch 67)*

**FHRP** — First Hop Redundancy Protocol (VRRP, HSRP). Two routers share a virtual IP
so a host's single default gateway is not a single point of failure. *(Ch 56)*

**Five-tuple** — Protocol, source address, source port, destination address,
destination port. Identifies a connection. *(Ch 35)*

**Flow control** — Protects the **receiver** from a fast sender, via the advertised
window. Not congestion control. *(Ch 37)*

**Forward secrecy** — Compromise of a long-term key does not decrypt past sessions.
Requires ephemeral key exchange; mandatory in TLS 1.3. *(Ch 58)*

**Fragmentation** — Splitting an IP packet for a link with a smaller MTU. Router
fragmentation removed in IPv6. *(Ch 24)*

**Frame / packet / segment / datagram** — The PDU at Layer 2 / Layer 3 / TCP / UDP.
Used strictly in this book. *(Ch 23)*

**FSPL** — Free-space path loss. 32.45 + 20log f(MHz) + 20log d(km). Doubling either
distance or frequency costs 6 dB. *(Ch 42)*

**Goodput** — Application payload rate, excluding all headers and retransmissions.
~940 Mb/s on a healthy gigabit link. *(Ch 3)*

**Hourglass** — The Internet's architecture: many applications, many link technologies,
exactly one protocol at the waist. *(Ch 23)*

**ICMP** — IP's error and diagnostic channel. Blocking it indiscriminately breaks Path
MTU Discovery. *(Ch 34)*

**Idempotence** — Applying an operation twice has the same effect as once. What makes
declarative automation safe to run continuously. *(Ch 70)*

**Intersymbol interference (ISI)** — One symbol smearing into its neighbours' time
slots. Killed the 1858 Atlantic cable; still the limit at high speed. *(Ch 1, 6)*

**IPAM** — IP address management: the live record of allocations, as opposed to the
plan. *(Ch 53)*

**Jitter** — Variation in latency. Converted into latency by a jitter buffer, at a
one-for-one exchange rate. *(Ch 3)*

**Latency** — Delay, decomposing into propagation, transmission, processing and
queueing. Only the last varies with load; only the first is irreducible. *(Ch 3)*

**Layer 2.5** — Informal designation for mechanisms that fit the OSI model badly: ARP,
MPLS. A known limitation of the map, not a gap in understanding. *(Ch 22)*

**Leaf–spine** — Data centre fabric in which every leaf connects to every spine, making
all servers two hops apart, with ECMP and no blocked links. *(Ch 67)*

**Least privilege** — Minimum access required for the function. The principal limiter
of blast radius. *(Ch 59)*

**Line coding** — Transforming data before transmission to guarantee transitions for
clock recovery and DC balance. Manchester, 4B/5B, 8B/10B, 64B/66B. *(Ch 7)*

**Link-local** — Valid on one link only, never routed. 169.254.0.0/16 in IPv4,
fe80::/10 in IPv6 — where it is always present. *(Ch 27, 28)*

**Longest-prefix match** — The router's rule: among matching routes, use the most
specific. What makes hierarchical addressing usable. *(Ch 29)*

**MAC address** — 48-bit hardware address. Flat, and therefore not aggregatable, and
therefore not usable globally. *(Ch 15)*

**Mathis relation** — Throughput ≈ (MSS/RTT)·(C/√p). Throughput falls as the inverse
square root of loss. *(Ch 3)*

**MPLS** — Label switching. Survived its original speed justification because of
traffic engineering and VPNs. A virtual circuit on packet infrastructure. *(Ch 50)*

**MTU** — Maximum Transmission Unit. Mismatches produce the small-packets-work,
large-packets-fail signature. *(Ch 24, 66)*

**Multicast** — One sender, a subscribed group, one copy per link. Efficient and
operationally demanding. *(Ch 27)*

**NAT / PAT** — Address translation, with ports to disambiguate. A 1994 stopgap that
became permanent infrastructure. Not a firewall. *(Ch 33)*

**Native VLAN** — The VLAN whose frames cross a trunk untagged. Source of silent
segment merging and of the double-tagging attack. *(Ch 20)*

**Network byte order** — Big-endian, mandatory in all Internet protocol headers.
*(Ch 2)*

**Nyquist limit** — A channel of bandwidth *B* carries at most 2*B* symbols per second
without ISI. *(Ch 4)*

**OFDM** — Splitting one fast channel into many slow subcarriers; defeats multipath.
In Wi-Fi, LTE, 5G, DOCSIS 3.1, DVB. *(Ch 8)*

**OSI model** — Seven layers. Its protocols are dead; its value is as a diagnostic
instrument for bisecting a fault. *(Ch 22)*

**Overlay** — A virtual network built by tunnelling over another. VXLAN, GENEVE,
SD-WAN. *(Ch 67)*

**PDU** — Protocol Data Unit. The unit at a given layer. *(Ch 23)*

**PMTUD black hole** — Path MTU Discovery defeated by blocked ICMP. Connection
establishes, then hangs on large transfers. *(Ch 34, 66)*

**PoE** — Power over Ethernet. Up to 90 W under 802.3bt. Frequently decides the media
choice, since fibre cannot deliver power. *(Ch 10, 16)*

**Policing / shaping** — Enforcing a rate by dropping / by buffering. Shape what you
send, police what you receive. *(Ch 52)*

**Propagation delay** — Distance ÷ velocity. ~204 km/ms in fibre. Irreducible.
*(Ch 1, 3)*

**QUIC** — Transport over UDP, in userspace, with integrated TLS 1.3 and independent
streams. HTTP/3's substrate. *(Ch 38)*

**RPO / RTO** — Recovery point objective (data loss tolerance, in time) / recovery time
objective (restoration deadline). Independent, and separately costed. *(Ch 56)*

**Shannon–Hartley theorem** — *C* = *B* log₂(1 + SNR). Linear in bandwidth,
logarithmic in SNR. The reason the industry chases spectrum, not watts. *(Ch 4)*

**Shared fate** — When nominally redundant components can fail together. The reason
naive redundancy arithmetic overstates availability. *(Ch 56)*

**SLAAC** — IPv6 stateless address autoconfiguration, from a router advertisement, with
no server. *(Ch 28)*

**SNR / SINR** — Signal-to-noise ratio; with interference included. In dB, simply
received dBm minus floor dBm. The quantity that determines achievable rate. *(Ch 4, 42)*

**Spanning tree** — Perlman's algorithm: block links to eliminate loops while retaining
physical redundancy. Necessary because Ethernet frames have no TTL. *(Ch 19)*

**Split horizon** — Do not advertise a route back toward the neighbour you learned it
from. A partial fix for count-to-infinity. *(Ch 31)*

**Statistical multiplexing** — Allocating capacity on demand rather than by
reservation. The economic argument that decided packet versus circuit. *(Ch 9, 13)*

**Store-and-forward** — Receive the whole frame, verify it, then forward. Adds latency,
prevents propagating corruption. *(Ch 13, 17)*

**Throughput** — The rate actually achieved. Always ≤ capacity, frequently far below.
*(Ch 3)*

**TTL** — Time to Live. A hop count, decremented at each router; loop insurance, and
the mechanism `traceroute` exploits. *(Ch 24, 34)*

**VLAN** — A logically separate broadcast domain on shared physical infrastructure.
Traffic between VLANs must be routed. *(Ch 20)*

**VLSM** — Variable-length subnet masking: different prefix lengths within one block,
sized to need. Allocate largest first. *(Ch 26)*

**VNI** — VXLAN Network Identifier. 24 bits, 16.7 million segments, against 802.1Q's
4,094. *(Ch 67)*

**Window scaling** — RFC 7323's extension beyond the 16-bit TCP window. Without it, a
100 ms path caps at 5.2 Mb/s regardless of capacity. *(Ch 3, 37)*

**Zero trust** — Architecture assuming no network location confers trust; every request
authenticated and authorised on its merits. *(Ch 59)*
