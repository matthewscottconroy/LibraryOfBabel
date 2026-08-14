# 37.2 Sequence Numbers and Acknowledgement

TCP delivers a **byte stream**: whatever the application writes, the peer's application
reads, in order, exactly once, with nothing missing.

**Over a network that loses, duplicates and reorders.** This section is the mechanism, and
it is one idea applied consistently.

## The idea

> **Number every byte. Acknowledge what you have received. Retransmit what is not
> acknowledged.**

**Every byte** — not every packet. This is the detail that makes everything else work, and
it is worth pausing on.

## Numbering bytes, not packets

**If TCP numbered packets, a retransmission would have to be the same size as the
original.** By numbering bytes, TCP can **repackage freely**:

```
   Sent:          [ 1000 bytes: seq 5000 ]  [ 1000 bytes: seq 6000 ]
   Lost:          the first

   Retransmit:    [ 500 bytes: seq 5000 ] [ 500 bytes: seq 5500 ]
                  or
                  [ 2000 bytes: seq 5000 ]   ← combining both, if the MTU allows
```

**Both are legal.** The receiver reassembles by byte position, so segment boundaries are
irrelevant to correctness.

**This matters in practice** because a retransmission may face a smaller path MTU than the
original (Chapter 34 §34.4), and TCP can simply send smaller segments. **A packet-numbered
protocol would be stuck.**

## The byte stream, and what it costs

**TCP has no message boundaries** (Chapter 36 §36.1). Three writes of 100 bytes may arrive
as:

- one read of 300
- two reads of 150
- 300 reads of 1
- any other division

The application must frame its own messages, and the standard approaches are:

| Method | Used by |
|---|---|
| **Length prefix** | most binary protocols |
| **Delimiter** | HTTP headers (`\r\n\r\n`), SMTP (`.` on a line) |
| **Fixed size** | simple protocols |
| Close the connection | HTTP/1.0's original approach |

> **"I called `send()` once so the peer will `recv()` once" is one of the most common
> beginner bugs in network programming**, and it works in testing on a LAN and fails in
> production.

## The header

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-------------------------------+-------------------------------+
|          Source Port          |       Destination Port        |
+---------------------------------------------------------------+
|                        Sequence Number                        |
+---------------------------------------------------------------+
|                    Acknowledgement Number                     |
+-------+-----------+-----------+-------------------------------+
| Offset| Reserved  |   Flags   |            Window             |
+-------+-----------+-----------+-------------------------------+
|           Checksum            |        Urgent Pointer         |
+-------------------------------+-------------------------------+
|                    Options (if Offset > 5)                    |
+---------------------------------------------------------------+
```

**Twenty bytes minimum, sixty maximum** — the Data Offset field counts 32-bit words, like
IP's IHL (Chapter 24 §24.2).

| Field | Purpose |
|---|---|
| **Sequence number** | the position of this segment's **first byte** in the stream |
| **Acknowledgement number** | **the next byte expected** — cumulative |
| Data offset | header length in 32-bit words |
| **Flags** | SYN, ACK, FIN, RST, PSH, URG, ECE, CWR, NS |
| **Window** | how much more the sender of this segment can receive (§37.4) |
| Checksum | covers header, payload **and a pseudo-header of the IP addresses** |
| Urgent pointer | essentially unused; see below |

### The flags

| Flag | Meaning |
|---|---|
| **SYN** | synchronise sequence numbers — connection setup |
| **ACK** | the acknowledgement field is meaningful — **set on all but the first SYN** |
| **FIN** | no more data from this side (§37.5) |
| **RST** | **abort** — tear down immediately |
| PSH | deliver to the application now; do not buffer |
| URG | urgent pointer is valid |
| **ECE / CWR** | explicit congestion notification (Chapter 38 §38.3) |

**URG and the urgent pointer are effectively dead.** Implementations disagreed about the
semantics, RFC 6093 recommends against using it, and its main modern relevance is as a
source of security bugs in middleboxes that try to parse it.

## Cumulative acknowledgement

**The acknowledgement number means: *"I have received everything up to but not including
this byte."***

```
   Received bytes 1–1000  →  ACK 1001
   Received bytes 1–2000  →  ACK 2001
```

**One number covers everything received.** Which has three consequences:

**Lost ACKs are harmless.** If the ACK for 1001 is lost and the ACK for 2001 arrives, the
sender learns everything it needed. **The acknowledgement is self-repairing.**

**ACKs need not be one-per-segment.** A receiver may acknowledge every second segment
(**delayed ACK**), reducing the reverse-path traffic by half.

**And it cannot express a gap.** This is the limitation:

```
   Received:  1–1000,  ✗ missing 1001–2000,  2001–3000

   ACK can only say:  1001
```

The receiver has bytes 2001–3000 and no way to say so. The sender knows only that 1001
is missing — not that anything after it arrived.

Without more information the sender might retransmit everything from 1001 onward,
including data the receiver already holds. **This is "go-back-N", and it wastes an entire
window on one loss.**

## SACK — expressing the gap

**Selective Acknowledgement**, RFC 2018, negotiated in the handshake (§37.1).

A SACK option lists the blocks that *have* arrived, alongside the cumulative ACK:

```
   ACK 1001,  SACK [2001–3000]

   meaning: "I need 1001 onward, AND I already have 2001–3000"
```

**So the sender retransmits only 1001–2000.**

The improvement is large on lossy or high-bandwidth-delay paths, where a window may
hold hundreds of segments and losing one should not mean resending all of them.

**SACK is universally supported and universally enabled.** If a capture shows it absent,
either a middlebox stripped the option (§37.1) or one end is very old — and in either case
the connection will perform badly under loss.

**D-SACK** (RFC 2883) extends it to report *duplicate* segments received, which lets a
sender learn that its retransmission was unnecessary and adjust.

## Duplicate ACKs and fast retransmit

The mechanism that recovers from loss without waiting for a timer (§37.3), and it
falls out of cumulative acknowledgement.

When a receiver gets an out-of-order segment, it re-sends the same ACK:

```
   Sender:                        Receiver:              ACK sent:
   seq 1000 (1000 bytes)  ──▶     received 1000–1999     ACK 2000
   seq 2000               ──▶     LOST                   —
   seq 3000               ──▶     out of order!          ACK 2000  ← duplicate
   seq 4000               ──▶     out of order!          ACK 2000  ← duplicate
   seq 5000               ──▶     out of order!          ACK 2000  ← duplicate
```

Three duplicate ACKs — four identical ACKs in total — is TCP's loss signal.

**Why three?** Because **reordering also produces duplicate ACKs** (Chapter 29 §29.3's
ECMP). One or two duplicates is more likely reordering than loss; **three is the threshold
chosen to distinguish them**, and it has held up remarkably well.

On receiving them, the sender retransmits immediately without waiting for the
retransmission timer — which is **fast retransmit**, and it is the difference between
recovering in one RTT and recovering in a timeout (§37.3).

## Sequence number wraparound

**The field is 32 bits**, so the space is 4,294,967,296 bytes ≈ **4 GB**.

**On a fast link this wraps quickly:**

| Rate | Time to wrap 4 GB |
|---|---|
| 100 Mb/s | ~5.7 minutes |
| 1 Gb/s | **~34 seconds** |
| 10 Gb/s | **~3.4 seconds** |
| 100 Gb/s | **~0.34 seconds** |

**The hazard:** a segment delayed in the network could arrive after the sequence space has
wrapped, and be accepted as valid current data.

PAWS — Protection Against Wrapped Sequence numbers (RFC 7323) solves it using the
**timestamp option**: every segment carries a timestamp, and a segment whose timestamp is
older than the connection's current one is discarded regardless of its sequence number.

Which is why the timestamp option matters beyond RTT measurement, and why disabling it
on a fast link is unwise.

## Reading it

```
$ tcpdump -nn -S 'host 203.0.113.10 and port 443'

10.0.0.5.51234 > 203.0.113.10.443: Flags [P.], seq 1043208216:1043209676,
    ack 2588791346, win 502, length 1460
203.0.113.10.443 > 10.0.0.5.51234: Flags [.],  ack 1043209676, win 509, length 0
```

**`-S` shows absolute sequence numbers**; without it `tcpdump` shows relative ones, which
are easier to read and hide the wraparound behaviour.

**`seq 1043208216:1043209676`** — this segment carries bytes at those positions;
1043209676 − 1043208216 = **1460**, the MSS.

**`ack 1043209676`** — **the next byte expected**, which is exactly where the previous
segment ended. Everything up to that point is received.

**`length 0`** on the ACK — a pure acknowledgement carrying no data.

## What breaks here

**An application receiving partial messages.** TCP is a byte stream; the application must
frame. **The commonest network-programming bug.**

**Terrible throughput under mild loss.** SACK is missing — check whether the option
survived the handshake.

**Sequence numbers that look wrong in a capture.** `tcpdump` shows relative numbers by
default. Use `-S`.

A connection failing on a very fast link after a few seconds. Sequence wraparound
without PAWS. Enable timestamps.

**Retransmission of data the receiver already has.** No SACK, so the sender is guessing.

> **Network+ note.** Objective 1.4 expects TCP's reliability mechanisms. Over-learn:
> **TCP numbers bytes, not packets**; **the acknowledgement number is the next byte
> expected**; **acknowledgement is cumulative**; **three duplicate ACKs signal loss and
> trigger fast retransmit**; and **the header is 20 bytes minimum.** The
> next-byte-expected definition is examined and is often misremembered as "the last byte
> received".
