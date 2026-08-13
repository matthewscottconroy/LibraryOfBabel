# 21.3 The Cost of Abstraction

Layering is not free, and books that present it as an unalloyed good leave students
unable to explain why so much real engineering violates it. This section is the bill.

## Header overhead

Every layer adds bytes that are not the user's data.

A minimal TCP-over-IP-over-Ethernet frame:

| Layer | Header | Bytes |
|---|---|---|
| Ethernet | dest, src, type, FCS | 18 |
| Ethernet | preamble + interframe gap | 20 |
| IPv4 | minimum | 20 |
| TCP | minimum | 20 |
| | **total overhead** | **78** |

For a 1460-byte payload — the standard maximum on Ethernet — that is:

$$\frac{78}{1538} = 5.1\%$$

Acceptable. Now consider a **single keystroke** in an interactive session: **1 byte of
data, 78 bytes of overhead, 1.3% efficiency**. And it is worse in practice, because
that keystroke generates an echo and an acknowledgement.

Voice over IP is the case where this bites hardest in production. A G.729 codec
produces **20 bytes** of audio per 20 ms packet:

| Component | Bytes |
|---|---|
| Voice payload | 20 |
| RTP header | 12 |
| UDP header | 8 |
| IP header | 20 |
| Ethernet | 38 |
| **Total** | **98** |

**20 bytes of speech in a 98-byte frame — 20% efficiency.** The codec compresses audio
to 8 kb/s and the network carries it at 39 kb/s. Nearly five times the bandwidth,
entirely in headers.

Which is why **RTP header compression** exists, why **VoIP over satellite** is
expensive, and why Chapter 45 §45.3's capacity planning uses the *on-the-wire* rate rather
than the codec rate. The compression schemes work by exploiting the fact that
successive headers in a flow are nearly identical — which is only possible **by
violating layering**, since a compressor must understand all three headers at once.

## Copying and context switching

Traditionally, data crossing a layer boundary was **copied**.

An application calling `send()` copies from user memory to a kernel buffer. Add TCP's
buffering, IP's handling, and the driver's copy into the NIC's DMA region, and the same
bytes are moved several times. At 100 Gb/s, memory bandwidth becomes the constraint —
not the network.

The engineering response has been thirty years of removing copies, and every technique
is a controlled layer violation:

| Technique | What it does | What it violates |
|---|---|---|
| **Zero-copy** (`sendfile`, `splice`) | data never enters user space | the user/kernel boundary |
| **Scatter-gather DMA** | NIC reads headers and payload from separate places | header/payload separation |
| **TSO / LRO** | one large buffer segmented by hardware | the transport/link boundary |
| **Checksum offload** | NIC computes the transport checksum | the transport/link boundary |
| **RDMA** | the NIC writes into remote application memory | **almost everything** |
| **DPDK / XDP** | user space drives the NIC directly | the entire kernel stack |

**TSO (TCP Segmentation Offload)** is worth stating explicitly because it is
universal and almost invisible. The stack hands the NIC a 64 KB buffer and a template
header; the NIC produces forty-odd Ethernet frames with correct sequence numbers and
checksums. **A link-layer device is performing a transport-layer function**, and the
layer boundary that made the design comprehensible is gone in the implementation.

The practical consequence appears in packet captures: `tcpdump` on a sending host shows
64 KB "packets" that never existed on the wire. **Capture on the host and capture on
the wire disagree**, and Chapter 64 §64.3 covers why.

## The information barrier

The most consequential cost, and the least visible.

**A layer cannot see what it is not told**, and the interfaces are narrow by design.
So each layer makes decisions on incomplete information, and sometimes those decisions
are exactly wrong.

**The classic example: TCP over wireless.**

TCP interprets packet loss as congestion (Chapter 38 §38.2), because on a wired
network it almost always is. The correct response to congestion is to slow down.

On a wireless link, loss is frequently **corruption** — interference, fading, a
microwave oven. The correct response is to retransmit **immediately**, since the
network is not congested at all.

TCP cannot tell the difference. The link layer knows perfectly well, and has no way to
say so.

**Result:** throughput collapses on lossy wireless links for a reason that is a pure
artefact of the layer boundary.

The mitigations are all layer violations:

- **802.11 link-layer retransmission** — the link layer hides the loss from TCP by
  retransmitting itself (Chapter 43 §43.3). Effective, and it converts loss into
  latency and jitter, which causes different problems.
- **ECN** — routers mark congestion explicitly rather than signalling it by dropping,
  so loss and congestion become distinguishable (Chapter 38 §38.3).
- **BBR** — infers congestion from bandwidth and RTT measurement rather than from
  loss (Chapter 38 §38.3).

Each is an attempt to get information across a boundary that was drawn to keep it out.

## What cannot be optimised across

A boundary prevents an optimisation that would require knowing both sides.

- IP cannot **combine** small TCP segments, because it must not parse them.
- Ethernet cannot **prioritise** by application, because it does not know the
  application.
- TCP cannot know that the **first byte of a video frame** matters more than the
  fortieth, because it carries an undifferentiated byte stream.

That last one is the most expensive in practice. **A video player wants to abandon a
late frame; TCP will retransmit it regardless**, because TCP guarantees ordered
delivery of every byte and cannot be told that some bytes have expired.

This is why real-time media uses **UDP** and builds its own transport (Chapter 36
§36.2) — not because TCP is badly implemented, but because **its service is the wrong
service**, and the interface offers no way to ask for a different one.

**Head-of-line blocking** is the same problem in another form. HTTP/2 multiplexes many
streams over one TCP connection; one lost packet stalls **every** stream, because TCP
must deliver in order and does not know the streams are independent. HTTP/2 knows;
TCP cannot be told. QUIC's central design decision (Chapter 38 §38.4) is to move the
transport into user space so that the multiplexing layer and the reliability layer can
finally see each other.

**QUIC is the largest layer violation in modern networking, and it exists because the
boundary cost more than it was worth.**

## Latency

Each boundary adds processing: a function call, a queue, sometimes a context switch,
sometimes an interrupt.

Individually microseconds. Aggregated:

| Path | Typical latency |
|---|---|
| Full kernel stack, interrupt-driven | 20–50 µs |
| Kernel bypass (DPDK) | 2–5 µs |
| RDMA | **under 2 µs** |

An order of magnitude, obtained by **removing layers**. High-frequency trading firms
spend heavily on exactly this, and so do storage and HPC systems, and the technique in
every case is to shorten the stack.

## The verdict

Layering costs bandwidth, CPU, latency, and — most importantly — **information**. It
buys independent evolution, independent expertise, independent failure, and
substitutability.

**The trade is overwhelmingly worth it**, which is why every network is layered. But
noticing the cost explains a great deal that is otherwise mysterious:

- why offload engines exist
- why VoIP needs header compression
- why TCP behaves badly on wireless
- why QUIC was built
- why high-performance systems bypass the stack
- why every packet capture on a modern host shows frames that never existed

**The exceptions in §21.4 are not failures of discipline. They are places where the
cost exceeded the benefit, and engineers responded correctly.**

## What breaks here

**Assuming layering is free.** It is not, and a design that ignores the cost will be
surprised by VoIP bandwidth and by wireless throughput.

**Capture disagreeing with the wire.** TSO/LRO/GRO. Disable offloads before a capture
that must reflect reality.

**Poor TCP throughput on a lossy link with plenty of bandwidth.** The
loss-means-congestion inference. Not a fault to be found; a boundary to be worked
around.

**Video stalling rather than degrading.** TCP retransmitting frames that no longer
matter. The wrong transport for the job.

> **Network+ note.** Not examined as such, and it underpins several examined topics —
> **VoIP bandwidth calculations must include header overhead**, and **wireless
> throughput is worse than its rate suggests** partly for the reason given here. Both
> appear on the exam in applied form.
