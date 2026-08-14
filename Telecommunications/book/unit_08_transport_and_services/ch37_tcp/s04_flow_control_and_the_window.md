# 37.4 Flow Control and the Window

**Flow control protects the receiver.** Congestion control (Chapter 38) protects the
network. They are different mechanisms solving different problems, and conflating them is
the most common confusion in this material.

| | Protects | Signalled by | Limit |
|---|---|---|---|
| **Flow control** | **the receiver** | the **window field** in every ACK | `rwnd` |
| **Congestion control** | **the network** | **loss, delay, or ECN marks** | `cwnd` |

**The sender is bound by both:**

$$\text{bytes in flight} \le \min(\text{rwnd},\ \text{cwnd})$$

Which of the two binds tells you where the bottleneck is, and it is directly
observable (§37.4's diagnostics below).

## The problem flow control solves

**A fast sender and a slow receiver.**

A 10 Gb/s server sending to a device that reads slowly — an embedded controller, a machine
under load, an application that is busy — will overwhelm the receiver's buffer, and the
data will be dropped **after** successfully crossing the network. Wasted bandwidth, and
the retransmission will be dropped too.

Stop-and-wait would solve it and would be catastrophically slow: one segment per round
trip, so on a 100 ms path with 1,460-byte segments:

$$\frac{1460 \times 8}{0.1} = 116{,}800 \text{ bits/s} \approx \mathbf{117\ kb/s}$$

**117 kb/s on a gigabit link.** The link is idle 99.99% of the time waiting for
acknowledgements.

## The sliding window

Allow several segments in flight at once, bounded by what the receiver can accept.

The receiver advertises a window in every ACK — *"beyond what I have acknowledged, I
can accept this many more bytes."*

```
   Byte stream:

   ...1000 | 1001 ─────────────── 5000 | 5001 ...
   ────────┴──────────────────────────┴────────
    sent &    sent, not yet ACKed  |  may send
    ACKed     ──── in flight ────  |  now
            └──── window = 4000 ───┘

   As ACKs arrive, the whole window slides right.
```

**Three quantities:**

| | |
|---|---|
| **`snd_una`** | oldest unacknowledged byte — the window's left edge |
| **`snd_nxt`** | next byte to send |
| **`snd_una + rwnd`** | the window's right edge — **cannot send past this** |

"Sliding" because the left edge advances as data is acknowledged and the right edge
advances as the window is re-advertised.

## Throughput and the bandwidth-delay product

The window determines the maximum throughput, and the relationship is the most
important formula in transport performance:

$$\text{throughput} \le \frac{\text{window}}{\text{RTT}}$$

Because you can have at most one window in flight per round trip.

Rearranged, this is the bandwidth-delay product — the window needed to fill a path:

$$\text{BDP} = \text{bandwidth} \times \text{RTT}$$

| Link | RTT | BDP | Window needed |
|---|---|---|---|
| 100 Mb/s | 10 ms | 125 KB | 125 KB |
| **1 Gb/s** | **10 ms** | **1.25 MB** | **1.25 MB** |
| 1 Gb/s | 100 ms | **12.5 MB** | 12.5 MB |
| 10 Gb/s | 100 ms | **125 MB** | 125 MB |
| 100 Mb/s | 600 ms (satellite) | **7.5 MB** | 7.5 MB |

And here is the problem the original TCP had:

## The 64 KB wall

**The window field is 16 bits.** Maximum **65,535 bytes**.

**Which caps throughput at:**

$$\frac{65{,}535 \times 8}{\text{RTT}}$$

| RTT | Maximum throughput with a 64 KB window |
|---|---|
| 1 ms | 524 Mb/s |
| **10 ms** | **52 Mb/s** |
| **50 ms** | **10.5 Mb/s** |
| **100 ms** | **5.2 Mb/s** |
| 600 ms | 874 kb/s |

> On a gigabit intercontinental path, an unscaled TCP connection achieves about
> 5 Mb/s — half a per cent of the link — **and it is not the network's fault.**

This is the single most important performance fact in this chapter, and it is why the
next mechanism exists.

## Window scaling

**RFC 7323**, negotiated in the handshake (§37.1).

A shift count, 0–14, applied to the window field:

$$\text{effective window} = \text{window field} \times 2^{\text{scale}}$$

With a scale of 14, the maximum window becomes

$$65{,}535 \times 2^{14} = \mathbf{1\ GB}$$

**Which is enough for any path.**

**Two properties matter operationally:**

**It is negotiated only in the handshake.** SYN and SYN-ACK carry the option; if either
is missing it, the connection runs unscaled for its entire life.

So a middlebox that strips the option imposes the 64 KB wall permanently (§37.1), and
the symptom is a connection that works perfectly and achieves 5 Mb/s on a gigabit path with
no errors, no loss, and nothing in any log.

This is the classic "the network is slow" complaint that is not the network, and
diagnosing it requires looking at the SYN.

## Buffers, and why autotuning matters

The advertised window comes from the receiver's buffer. An operating system that
allocates a small buffer advertises a small window and limits throughput regardless of the
network.

**Modern stacks autotune**, growing the buffer as the connection's BDP becomes apparent:

```bash
# Linux: min, default, max — in bytes
sysctl net.ipv4.tcp_rmem     # 4096  131072  6291456
sysctl net.ipv4.tcp_wmem     # 4096   16384  4194304
sysctl net.core.rmem_max     # the ceiling autotuning may reach
sysctl net.ipv4.tcp_moderate_rcvbuf   # 1 = autotuning enabled
```

The default maximum of ~6 MB supports a 1 Gb/s path at about 50 ms RTT. For anything
longer or faster, raise it:

```bash
# For a 10 Gb/s, 100 ms path: BDP = 125 MB
sysctl -w net.core.rmem_max=134217728
sysctl -w net.ipv4.tcp_rmem="4096 87380 134217728"
```

**And there is a trap:** an application that calls `setsockopt(SO_RCVBUF)` disables
autotuning for that socket. A well-meaning application that sets a "large" 256 KB buffer
has capped itself far below what autotuning would have provided. Do not set it unless
you know the BDP.

## Zero window

When the receiver's buffer fills, it advertises a window of zero:

```
   Receiver ── ACK, win 0 ──▶ Sender      "stop"
```

**The sender must stop.** It may send nothing but a **window probe** — a one-byte segment
sent periodically to ask whether the window has opened.

**The probe exists to prevent a deadlock:** if the receiver later advertises a non-zero
window and that ACK is lost, the sender would wait forever for a message that will
never be repeated — because ACKs are not retransmitted. The probe breaks the standoff by
having the sender ask.

Zero windows in a capture are diagnostic and unambiguous:

> **A zero window means the receiving *application* is not reading fast enough.** It is not
> a network problem. The data arrived; nobody consumed it.

The application is slow, blocked, deadlocked, or starved of CPU. Look at the receiver's
process, not the network.

Persistent zero windows are one of the clearest signals in packet analysis, and they
point away from the network entirely.

## Silly window syndrome

A pathology of the mechanism, and both ends have a fix.

**Receiver side:** the application reads one byte, so one byte of buffer frees, so the
receiver advertises a one-byte window, so the sender sends a 41-byte packet to carry one
byte of data. **Efficiency 2.4%.**

**David Clark's fix:** do not advertise a window increase until it is worth having — at
least one MSS, or half the buffer.

**Sender side:** the application writes one byte at a time, so TCP sends 41-byte packets
each carrying one byte. Telnet did exactly this.

**Nagle's algorithm (RFC 896):** *if there is unacknowledged data outstanding, buffer small
writes until it is acknowledged.* Small writes are coalesced into one segment.

### Nagle versus delayed ACK — the interaction that bites

Two independently sensible optimisations that combine badly, and it is worth
understanding because the symptom is distinctive.

**Delayed ACK:** a receiver waits up to 200 ms before acknowledging, hoping to piggyback
the ACK on outgoing data or to acknowledge two segments at once.

**Nagle:** a sender waits for an acknowledgement before sending a small segment.

Put them together with a request-response application that writes a header and then a
body:

```
   Sender:   writes header (small)  → Nagle sends it
   Sender:   writes body (small)    → Nagle: unacked data outstanding, HOLD
   Receiver: got the header, delayed ACK: WAIT up to 200 ms
   ...
   Receiver: 200 ms timer fires, sends ACK
   Sender:   receives ACK, sends the body
```

A 200 ms stall on every request, on a path whose RTT may be 1 ms.

**The fixes:**

| Fix | Where |
|---|---|
| **`TCP_NODELAY`** — disable Nagle | the application, and **this is the usual answer** |
| **Write once**, not twice | the application — often better than disabling Nagle |
| `TCP_QUICKACK` | disable delayed ACK, per-socket, Linux |

`TCP_NODELAY` is set by essentially every latency-sensitive application — databases,
RPC frameworks, games, interactive protocols. And "combine your writes" is usually the
better fix, because it removes the small segments rather than merely permitting them.

## Reading the window

```
$ ss -tni
ESTAB 0 0 10.0.0.5:44312 203.0.113.10:443
    cubic wscale:7,7 rto:236 rtt:35.5/2.1 mss:1448
    cwnd:24 ssthresh:18 bytes_sent:14200000
    send 7.8Mbps  rcv_space:14600  rcv_ssthresh:64088
```

| Field | Meaning |
|---|---|
| **`wscale:7,7`** | send and receive scale factors — **both present, so scaling is active** |
| **`cwnd:24`** | congestion window, **in segments** — 24 × 1448 ≈ 34 KB |
| `ssthresh:18` | slow-start threshold (Chapter 38 §38.2) |
| `rcv_space` | the receive window's current working size |

**In a capture:**

```
$ tcpdump -nn
203.0.113.10.443 > 10.0.0.5.51234: Flags [.], ack 5000, win 501
```

**`win 501` is the *field*.** With `wscale:7`, the real window is 501 × 128 = **64,128
bytes**.

> **Wireshark applies the scale automatically only if it saw the handshake.** A capture
> started mid-connection shows unscaled values and will mislead you badly — this is a
> genuine trap, and the fix is to capture from the SYN.

## Diagnosing which limit binds

**The practical payoff of the section:**

| Observation | Bottleneck |
|---|---|
| **`cwnd` ≪ `rwnd`** | **the network** — congestion control is limiting (Chapter 38) |
| **`rwnd` ≪ `cwnd`** | **the receiver** — its buffer or its application |
| **Zero windows** | **the receiving application** is not reading |
| Both large, throughput low | the **sender's** application is not writing, or the CPU is the limit |
| **No `wscale` in the handshake** | **the 64 KB wall** — check for a middlebox |

**Two commands answer this:**

```bash
ss -tni                              # cwnd, rcv_space, wscale
tcpdump -nn 'tcp[tcpflags] & tcp-syn != 0'   # did wscale survive?
```

## What breaks here

~5 Mb/s on a fast long path, with no loss and no errors. The 64 KB wall — window
scaling absent. **Capture the SYN.**

**Throughput capped well below the link rate.** Buffer limits. Check `tcp_rmem` and
`rmem_max`, and check whether the application set `SO_RCVBUF`.

**Persistent zero windows.** The receiving **application** is not reading. Not a network
problem.

**A consistent 200 ms delay per transaction.** Nagle interacting with delayed ACK. Set
`TCP_NODELAY`, or combine the writes.

A capture showing tiny windows that make no sense. The capture missed the handshake, so
the scale factor is unknown.

An application that set its own buffer size and got slower. Autotuning disabled.

> **Network+ note.** Objective 1.4 expects flow control and windowing. Over-learn:
> flow control protects the receiver and congestion control protects the network;
> **the receiver advertises a window in every ACK**; **throughput ≤ window ÷ RTT**;
> the window field is 16 bits and window scaling is negotiated in the handshake; and
> a zero window means the receiver's application is not reading.
