# Chapter 3 — Important Concepts

**Bandwidth (physical sense)** *(§3.1)* — The width in hertz of the frequency band
a channel passes. A property of the medium and the equipment. This is the sense
Nyquist and Shannon use and the one Chapter 4 requires.

**Bandwidth (colloquial) / capacity** *(§3.1)* — The maximum data rate of a link
in bits per second. Fixed by the link's standard and physics; does not vary with
what you do. What you buy.

**Throughput** *(§3.1)* — The data rate actually achieved over some interval.
Always ≤ capacity, and depends on protocol, peer, path, load, and application.
What you get.

**Goodput** *(§3.1)* — The rate of useful application payload delivered,
excluding all protocol headers and retransmissions. What the user experiences. On
gigabit Ethernet with 1,460-byte payloads, ~949 Mb/s is the theoretical ceiling
and ~940 Mb/s the practical one.

**Protocol overhead** *(§3.1)* — Bytes on the wire that are not application
payload: interframe gap, preamble, and the headers of every layer. Small for large
frames (5% at 1,460 bytes), dominant for small ones (33% for a 160-byte voice
payload).

**Jumbo frames** *(§3.1)* — Payloads up to ~9,000 bytes, raising efficiency to
99%. Require every device on the path to agree; a single dissenting hop produces a
black-hole failure (Chapter 66).

**Bit/byte and decimal/binary prefix conventions** *(§3.1)* — Rates in bits
(lowercase `b`) with strictly decimal SI prefixes; storage in bytes (uppercase
`B`), with IEC binary prefixes (KiB, MiB) where memory is meant. The factor of 8
and the 1000-vs-1024 difference account for a large share of user confusion.

**Latency** *(§3.2)* — One-way delay from send to arrival. **Round-trip time
(RTT)** is the delay until a response returns. Both decompose into four
independent components.

**Propagation delay** *(§3.2)* — Distance ÷ propagation velocity. ~5 µs/km in
fibre. Constant, load-independent, size-independent, and irreducible. 119 ms one
way to geostationary orbit.

**Transmission (serialisation) delay** *(§3.2)* — Packet length ÷ link rate. The
time to clock the bits out. Proportional to size, inversely proportional to rate,
distance-independent. 12 µs for a 1,500-byte frame at 1 Gb/s.

**Processing delay** *(§3.2)* — Time the device spends deciding what to do with a
packet. 0.5–5 µs in a hardware switch; far larger and load-dependent in software
forwarding or on a device's control plane.

**Control-plane rate limiting** *(§3.2, §3.3)* — Routers deprioritise and
rate-limit ICMP addressed to themselves. A high latency or loss figure at an
intermediate traceroute hop, with normal figures beyond it, is this artefact and
not a fault.

**Queueing delay** *(§3.2)* — Time spent waiting in a buffer. The only
load-dependent component, and non-linear: relative delay grows as ρ/(1−ρ), so
raising utilisation from 50% to 90% multiplies it ninefold. The reason capacity
planning targets 60–70%.

**Jitter** *(§3.3)* — Variation in latency between successive packets. Formally
defined for RTP in RFC 3550. Destroys real-time applications not by delaying them
but by making delay unpredictable.

**Jitter buffer** *(§3.3)* — A playout buffer that absorbs variation by adding a
fixed delay to every packet. Converts jitter into latency at a one-for-one
exchange rate, which is why the ITU-T G.114 budget of 150 ms one-way makes buffer
growth expensive.

**Packet loss** *(§3.3)* — Fraction of packets that never arrive. Three causes,
diagnostically distinct: buffer overflow (correlates with load), corruption
(correlates with the physical layer), and policy (correlates with a rule).

**Mathis relation** *(§3.3)* — Classic TCP throughput ≈ (MSS/RTT)·(C/√p).
Throughput falls as the inverse square root of loss, so 1% loss caps a stream at
**under 2 Mb/s** on an 80 ms path regardless of link capacity, and even 0.001%
loss holds it to ~57 Mb/s. Pessimistic for modern
CUBIC/BBR stacks; the shape still governs.

**avg − min** *(§3.3)* — The most underused diagnostic in ping output. The minimum
approximates the path's irreducible delay; the difference between average and
minimum estimates typical queueing.

**Bandwidth–delay product (BDP)** *(§3.4)* — Capacity × RTT, in bits. The amount
of data in flight on a fully utilised path. 12.5 MB on a 1 Gb/s, 100 ms path.

**Sliding window limit** *(§3.4)* — Max throughput = window ÷ RTT. With the
original 16-bit TCP window (64 KB max), a 100 ms path is capped at 5.2 Mb/s
irrespective of link capacity.

**Window scaling** *(§3.4)* — RFC 7323's SYN-negotiated left-shift extending the
TCP window to 1 GB. Fails in three characteristic ways: a too-low kernel ceiling,
a middlebox stripping the option, and a slow-reading application.

**Long fat network (LFN)** *(§3.4)* — A path with a large bandwidth–delay product.
More sensitive to loss, slower to reach capacity through slow start, and much
improved by parallel streams.

**Round-trip count** *(§3.4)* — The generalised lesson: any protocol requiring
acknowledgement before proceeding is limited by data-in-flight ÷ RTT. When
diagnosing an application that is fine on the LAN and unusable on the WAN, count
round trips before considering bandwidth.
