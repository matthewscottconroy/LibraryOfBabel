# 36.2 When Speed Beats Certainty

TCP is reliable and UDP is not, so TCP is better. That reasoning is wrong, it is extremely
common, and correcting it properly is what this section is for.

The correct framing is not "which is better" but "what does this application actually
need".

## The cost of reliability

Reliability is not free, and its costs are specific.

### One round trip before any data

TCP's handshake (Chapter 37 §37.1) is SYN, SYN-ACK, ACK. The client cannot send data
until the third packet, so one full round trip elapses before the request goes out.

| RTT | TCP handshake cost | Effect on a 1-packet query |
|---|---|---|
| 1 ms (LAN) | 1 ms | negligible |
| 20 ms (regional) | 20 ms | **doubles it** |
| 100 ms (intercontinental) | 100 ms | **doubles it** |
| 600 ms (geostationary satellite) | **600 ms** | **doubles it** |

For a transaction consisting of one question and one answer, the handshake doubles the
latency. And that is before TLS, which historically added two more round trips
(Chapter 41 §41.1).

### Head-of-line blocking

The cost that surprises people, and it is the reason real-time media cannot use TCP.

**TCP delivers bytes in order.** If segment 5 is lost, segments 6, 7 and 8 **may have
arrived** and TCP will not deliver them until 5 is retransmitted and received.

```
   Sent:      1  2  3  4  5  6  7  8
   Arrived:   1  2  3  4  ✗  6  7  8
   Delivered: 1  2  3  4  ─── nothing ───   ← waiting for 5
                              ↓
                   +1 RTT for retransmission
                              ↓
   Delivered:                 5  6  7  8    ← all at once
```

The application waits one round trip for data that is already in the receiver's
memory.

For a file transfer this is correct — you need byte 5 before byte 6 means anything.

**For live audio it is a disaster.** By the time segment 5 arrives, its moment has passed;
segments 6, 7 and 8 were the audio that should be playing **now**, and they were held back
for a sample that is already too late to use.

> TCP's retransmission does not merely fail to help real-time media. It actively harms
> it, by delaying data that is still useful in order to deliver data that is not.

### Retransmission of stale data

Following directly: TCP will retransmit anything it sent, regardless of whether it
still matters.

A video conferencing application that loses a frame wants to skip it and send the next
one. TCP has no way to be told this — its service is *deliver every byte in order*, and
there is no interface for *"forget byte 5, it has expired"* (Chapter 21 §21.3).

So the application must either accept the wrong behaviour or use a different transport.

### Connection state

TCP requires a control block per connection at both ends — sequence numbers, windows,
timers, buffers. A server with 100,000 connections holds 100,000 of them.

**A stateless UDP server holds nothing.** A DNS resolver answering 100,000 queries per
second maintains no per-client state at all, which is why one machine can do it.

### The teardown, and TIME-WAIT

Closing a TCP connection costs four packets and leaves the closer holding the tuple for
60 seconds (Chapter 35 §35.4). For short transactions this is a substantial tail cost —
and it is the mechanism behind ephemeral exhaustion (Chapter 35 §35.3).

## When UDP is right

Four criteria. If several apply, UDP deserves consideration.

### 1. The transaction is short — ideally one exchange

If the whole conversation is a question and an answer, TCP's handshake is pure
overhead: three packets of setup, two of data, four of teardown, to move two packets'
worth of information.

**DNS is the canonical case.** A query is ~30 bytes, a response ~100. TCP's overhead
would exceed the payload by an order of magnitude, and the round trip would double the
latency.

### 2. Timeliness matters more than completeness

**Voice, video, gaming, telemetry, live measurement.**

**The test:** *if this data arrives late, is it still useful?* If the answer is no,
retransmission is worthless and the delay it causes is harmful.

A voice sample from 200 ms ago cannot be played — the moment has gone, and playing it
would be worse than the gap. A gap of 20 ms is inaudible; a 200 ms stall is very
audible.

### 3. The application can do reliability better than TCP can

**The important criterion, and the least understood.**

TCP's reliability is **general** — every byte, in order, always. An application often
knows something TCP cannot:

| Application knows | TCP cannot exploit it |
|---|---|
| "This frame is a keyframe; the next twenty depend on it" | it treats all bytes equally |
| "This update supersedes the previous one" | it will deliver both |
| "This request is idempotent; retry freely" | it retries at the wrong granularity |
| "Loss of this is acceptable; loss of that is not" | it has one reliability setting |

An application with this knowledge can build reliability that is better for its purpose
than TCP's — retransmitting only what matters, at the right time, at the right
granularity.

QUIC is the industrial-scale version of this argument (Chapter 38 §38.4): it runs over
UDP and implements reliability **per stream**, so a loss in one stream does not block
others — which is precisely the head-of-line problem TCP cannot solve because it does not
know the streams exist.

### 4. One-to-many

**TCP is strictly point-to-point.** There is no multicast TCP and there cannot be — the
handshake, the acknowledgements and the windows are all pairwise.

Multicast and broadcast require UDP (Chapter 27 §27.3). Anything that must reach many
recipients at once — routing protocol updates, service discovery, IPTV, market data — uses
UDP because there is no alternative.

## When UDP is wrong

**Equally important, and the failure is common.**

**Bulk transfer.** Every byte matters and there is no deadline. Use TCP; you will end up
reimplementing it badly otherwise.

Anything where you would implement retransmission and ordering. If the application
needs *every byte, in order*, that is TCP's service. Building it on UDP means writing —
and debugging — sequence numbers, acknowledgements, timers, retransmission, and flow
control, all of which took the TCP community twenty years to get right (Chapter 38).

Anything over a congested or shared path, without congestion control. §36.4's argument,
and it is a matter of network citizenship rather than application correctness.

> "We used UDP because it is faster" is the most common bad reason. UDP is faster
> because it does less. If your application needs the things it does not do, you will do
> them yourself, worse.

## The comparison, honestly

| | TCP | UDP |
|---|---|---|
| Header | 20+ bytes | **8 bytes** |
| Setup | **1 RTT** | **none** |
| Reliable | **yes** | no |
| Ordered | **yes** | no |
| Duplicates suppressed | **yes** | no |
| Flow control | **yes** | no |
| **Congestion control** | **yes** | **no** |
| Message boundaries | **no** — a byte stream | **yes** |
| Head-of-line blocking | **yes** | **no** |
| Multicast | **no** | **yes** |
| Server state per client | one control block | **none** |
| Good for | bulk, transactional, anything needing every byte | short exchanges, real-time, one-to-many |

**Two rows are worth reading against expectation.** UDP's **message boundaries** are a
feature TCP lacks, and TCP's **head-of-line blocking** is a defect UDP lacks. The list is
not simply "TCP has more".

## The modern answer

The distinction is blurring, and the direction is worth knowing.

**QUIC** (Chapter 38 §38.4) runs over UDP and provides reliability, ordering, flow control,
congestion control and encryption — everything TCP does, plus per-stream independence
that TCP cannot do.

So "UDP" in a modern traffic profile increasingly means "QUIC", which is more reliable
than TCP in the way that matters. A firewall rule permitting TCP/443 and blocking UDP/443
is not blocking "unreliable traffic"; it is blocking HTTP/3.

**The framing that survives:**

> The question was never "reliable or not". It was "who implements reliability, and with
> what knowledge of the application's needs". TCP answers "the kernel, generically". UDP
> answers "you, however you like". QUIC answers "a library, with the application's
> knowledge available to it" — and that is why it won.

## What breaks here

Using TCP for a one-packet transaction over a long path. The handshake doubles the
latency.

**Using TCP for live audio or video.** Head-of-line blocking, and retransmission of data
that has expired.

**Using UDP for bulk transfer.** You will reimplement TCP, badly.

Choosing UDP "because it is faster" without asking what you lose. The most common bad
reason.

**Blocking UDP/443 believing it blocks something unreliable.** It blocks HTTP/3.

> **Network+ note.** Objective 1.4 expects the TCP/UDP comparison and which applications
> use which. Over-learn: UDP for DNS, DHCP, TFTP, SNMP, syslog, NTP, voice and video;
> TCP for HTTP, HTTPS, SSH, FTP, SMTP and anything needing every byte; and the
> reasoning — UDP when the transaction is short or timeliness beats completeness.
