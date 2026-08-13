# 23.3 Encapsulation and PDUs

Encapsulation is the mechanism that makes layering real on the wire. This section
traces one HTTP request from a browser to a server and back, byte by byte, at every
layer and every hop.

It is the most concrete thing in this unit, and everything in Units I–IV appears in it.

## The PDU names

A **Protocol Data Unit** is the unit of data at a given layer. Each layer has its own
name, and the names are examined:

| Layer | PDU | Contains |
|---|---|---|
| Application | **data** / message | the actual content |
| Transport | **segment** (TCP) / **datagram** (UDP) | data + transport header |
| Internet | **packet** (or IP datagram) | segment + IP header |
| Link | **frame** | packet + frame header + trailer |
| Physical | **bits** / symbols | the frame, encoded |

Use them precisely. Saying "packet" for everything is common and it makes conversations
about where a fault lives much harder than they need to be.

## The nesting

Each layer wraps what it receives and treats it as **opaque**:

```
   ┌─────────────────────────────────────────────────────────────┐
   │ Eth │ IP │ TCP │            HTTP data              │ FCS   │
   └─────────────────────────────────────────────────────────────┘
     14    20    20                                        4
     └── frame ────────────────────────────────────────────────┘
           └── packet ──────────────────────────────────┘
                 └── segment ──────────────────────────┘
                        └── data ────────────────────┘
```

**The nesting property:** each layer adds a header, hands the result down, and the
layer below neither knows nor cares what it contains.

## The trace

**The setup.** A user types `http://web.example.com` in a browser on a laptop.

| | |
|---|---|
| Laptop | `192.168.1.50/24`, MAC `aa:aa:aa:aa:aa:aa` |
| Gateway | `192.168.1.1`, MAC `rr:rr:rr:rr:rr:rr` |
| Server | `203.0.113.10`, MAC `ss:ss:ss:ss:ss:ss` |

### Step 0 — DNS first

Nothing can happen until `web.example.com` becomes an address. The browser asks the
resolver, which is itself a full encapsulated exchange over UDP port 53 (Chapter 39).

**Result:** `web.example.com` → `203.0.113.10`.

Worth noting because **DNS failure is the most common cause of "the network is
broken"** (Chapter 22 §22.4), and it happens before anything else in this trace.

### Step 1 — Application layer

The browser constructs an HTTP request:

```
GET / HTTP/1.1
Host: web.example.com
User-Agent: Mozilla/5.0
Accept: text/html
Connection: keep-alive

```

**Roughly 120 bytes of ASCII.** That is the *data*. Everything from here on is
machinery to move it.

### Step 2 — Transport layer

TCP wraps it. (The three-way handshake happened first; assume the connection is
established.)

| Field | Value |
|---|---|
| Source port | **49152** — ephemeral, chosen by the OS |
| Destination port | **80** — HTTP's well-known port |
| Sequence number | 1 |
| Acknowledgement | 1 |
| Flags | PSH, ACK |
| Window | 64240 |
| Checksum | computed over the segment **and a pseudo-header of the IP addresses** (§21.4) |

**Segment: 20 + 120 = 140 bytes.**

The port numbers are the layer's entire contribution to addressing: **the IP address
identifies the host; the port identifies the process** (Chapter 35).

### Step 3 — Internet layer

IP wraps the segment.

| Field | Value |
|---|---|
| Version | 4 |
| IHL | 5 (× 4 = 20 bytes) |
| Total length | **160** |
| TTL | **64** |
| Protocol | **6** (TCP) |
| Header checksum | computed over the IP header only |
| Source | `192.168.1.50` |
| Destination | `203.0.113.10` |

**Packet: 20 + 140 = 160 bytes.**

Two fields to notice:

- **Protocol = 6** tells the receiving IP layer to hand the payload to TCP. This is the
  **demultiplexing key**, and every layer has one.
- **TTL = 64** is the loop protection Ethernet lacks (Chapter 19 §19.1), decremented at
  every router.

### Step 4 — Link layer

Now the local-or-remote decision (Chapter 18 §18.1):

```
   192.168.1.50  AND  255.255.255.0  =  192.168.1.0
   203.0.113.10  AND  255.255.255.0  =  203.0.113.0
   different  →  send to the gateway
```

So the host ARPs for `192.168.1.1` — not for `203.0.113.10` — and builds the frame:

| Field | Value |
|---|---|
| Destination MAC | `rr:rr:rr:rr:rr:rr` ← **the gateway** |
| Source MAC | `aa:aa:aa:aa:aa:aa` |
| EtherType | **`0x0800`** (IPv4) |
| Payload | the 160-byte packet |
| FCS | CRC-32 |

**Frame: 14 + 160 + 4 = 178 bytes.**

### Step 5 — Physical layer

The frame is encoded — 4B/5B or 64B/66B depending on the rate (Chapter 6) — a preamble
and start-frame delimiter are prepended, and the symbols go on the wire.

**On the wire: 178 + 8 (preamble/SFD) + 12 (interframe gap) = 198 byte-times.**

**Efficiency: 120 bytes of HTTP in 198 byte-times = 61%.** §21.3's argument, made
concrete.

### Step 6 — The router

The gateway receives the frame:

1. **Layer 1:** recovers the bits.
2. **Layer 2:** checks the FCS, sees the destination MAC is its own, **discards the
   frame header entirely**, reads EtherType `0x0800`, hands the packet to IP.
3. **Layer 3:** looks up `203.0.113.10` in the routing table by longest-prefix match (Chapter 29), **decrements TTL to 63**, recomputes the header checksum.
4. **Layer 2 again:** ARPs on the outgoing interface for the next hop, builds an
   **entirely new frame**, and transmits.

**Critical observation.** The router did **not** examine the TCP header. It did not
know or care that the packet contained TCP, let alone HTTP. It read the IP header,
made a decision, and rebuilt the frame.

> **The frame is rebuilt at every hop; the packet is not.** Only TTL and the header
> checksum change.

This is Chapter 18 §18.1's *MAC is hop-by-hop, IP is end-to-end*, seen at the moment it
happens. It is also why a router is a Layer 3 device: it operates on the layer whose
header it reads.

### Step 7 — Arrival

After several hops the packet reaches the server's link. The last router ARPs for
`203.0.113.10`, builds a final frame to `ss:ss:ss:ss:ss:ss`, and transmits.

The server **de-encapsulates**, and each layer uses a demultiplexing key to decide who
gets the payload:

| Layer | Check | Key | Hand to |
|---|---|---|---|
| 1 | recover bits | — | Layer 2 |
| 2 | FCS valid; MAC is mine | **EtherType `0x0800`** | IP |
| 3 | checksum valid; address is mine | **Protocol 6** | TCP |
| 4 | checksum valid; sequence in order | **Port 80** | the web server process |
| 7 | parse | — | application logic |

**Every layer has a demultiplexing key**, and they form a chain:

$$\texttt{EtherType} \rightarrow \texttt{Protocol} \rightarrow \texttt{Port} \rightarrow \text{process}$$

Knowing this chain is worth a great deal in a packet capture: it is exactly how
Wireshark decides which dissector to apply, and it is how you reason about what a
device can and cannot see.

### Step 8 — The reply

The server sends a response — headers and HTML, typically far more than 1460 bytes, so
TCP **segments** it into multiple packets (Chapter 37 §37.2).

Each is encapsulated identically, with source and destination reversed, and travels
back — possibly by a **different route** (Chapter 29 §29.3). The browser's TCP
reassembles them in order and hands a complete byte stream to the application.

## What this shows

**Five headers for 120 bytes of content**, and every one earns its place: the FCS
detects corruption, the EtherType demultiplexes, the IP addresses route, the TTL
prevents loops, the ports demultiplex to a process, the sequence numbers order and
detect loss.

**Each layer is genuinely independent.** The router did not parse TCP; TCP did not know
which link it crossed; HTTP did not know any of it happened.

**The demultiplexing chain is the mechanism** by which the receiving stack reverses the
sender's construction, with no ambiguity at any step.

**And a fault at any layer is a fault at every layer above it**, which is §22.4's
method restated as a property of encapsulation rather than a troubleshooting heuristic.

## What breaks here

**Using "packet" for everything.** Frame, packet, segment. The distinction is where
faults live.

**Expecting the destination MAC to be the destination host.** On any routed traffic it
is the next hop.

**Forgetting DNS happens first.** Step 0 is invisible in most descriptions and is the
most common failure point.

**Forgetting the frame is rebuilt every hop.** It explains why MAC addresses do not
appear in traceroute output, why a capture at one point differs from a capture at
another, and why Layer 2 problems are local by nature.

**Assuming a capture at one point shows the whole path.** It shows that hop. Chapter 64
§64.2 is about choosing where to capture, and the answer follows from this section.

> **Network+ note.** Objective 1.1 expects **encapsulation and the PDU names** —
> **bit, frame, packet, segment/datagram** — and this is examined directly. Also
> over-learn the **demultiplexing chain**: EtherType `0x0800` = IPv4, `0x86DD` = IPv6,
> `0x0806` = ARP; IP protocol 6 = TCP, 17 = UDP, 1 = ICMP. These specific numbers
> appear on the exam.
