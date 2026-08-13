# Chapter 36 — UDP

RFC 768, *User Datagram Protocol*, was written by Jon Postel and published in August
1980. It is **three pages long**, and one of those pages is mostly a diagram.

For comparison, TCP's specification — RFC 793, from the same author, the following
year — is eighty-five pages, and the modern consolidated version, RFC 9293 (2022),
runs to about a hundred and eighty.

That ratio is the chapter. UDP is what you get when you take IP, add the one thing
that was genuinely missing — process addressing — and then stop.

## The header

Eight bytes. Four fields. There is nothing else.

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-------------------------------+-------------------------------+
|          Source Port          |       Destination Port        |
+-------------------------------+-------------------------------+
|            Length             |           Checksum            |
+-------------------------------+-------------------------------+
```

**Source port** — who sent it, so a reply can be addressed. Optional; may be zero if
no reply is expected.

**Destination port** — which process it is for. This is the field UDP exists to
provide.

**Length** — of header plus data.

**Checksum** — over the header, the data, and a pseudo-header borrowed from IP.
Optional in IPv4 and mandatory in IPv6.

That is the entire protocol. No sequence numbers, because there is no ordering
guarantee to maintain. No acknowledgements, because nothing is promised. No
connection state, because there is no connection — every datagram is independent,
and a UDP "session" exists only in the application's imagination and possibly in a
firewall's state table.

## What UDP does not do, listed honestly

- Datagrams may be **lost**, and nobody will mention it.
- They may arrive **out of order**.
- They may be **duplicated**.
- There is **no flow control** — a fast sender will overwhelm a slow receiver and
  neither will be told.
- There is **no congestion control** — a UDP sender does not slow down when the
  network is full, which is why unrestrained UDP is capable of causing the kind of
  collapse Chapter 38 describes.

Presented as a list of deficiencies, it looks like a protocol nobody should use.
Presented correctly, it is a protocol that **declines to make decisions on the
application's behalf**, which is a different and more useful description.

## When declining is right

§36.2 develops four situations, and it is worth having them in advance because they
cover essentially every real use.

**When late data is useless.** In a voice call, a packet that arrives after its
playout deadline (Chapter 3 §3.3) cannot be used. TCP would retransmit it — spending
bandwidth and adding delay to deliver something that will be discarded, while
head-of-line blocking holds up the packets behind it. UDP simply loses it, the codec
conceals the gap, and the call continues. **For real-time media, TCP's reliability is
actively harmful**, and this is the cleanest example in the book of a "better"
guarantee being the wrong choice.

**When the exchange is one request and one reply.** A DNS query is ~50 bytes and its
answer ~200. Over TCP that costs a three-way handshake, the exchange, and a
four-way teardown: seven packets and at least two round trips, to move 250 bytes.
Over UDP it is two packets and one round trip. At the scale DNS operates, the
difference is enormous — and if the reply is lost, the application simply asks again,
which is a retransmission scheme costing zero bytes of protocol machinery.

**When the traffic is one-to-many.** Multicast and broadcast are inherently
unacknowledgeable — there is no single peer to acknowledge — so a reliable transport
is not merely wasteful but incoherent. Every multicast application therefore uses
UDP.

**When the application wants to build its own transport.** This is the modern case
and it is the interesting one. QUIC (Chapter 38 §38.4) runs over UDP and implements
its own reliability, ordering, flow control and congestion control — better ones,
tuned for HTTP's actual traffic pattern, deployable by updating a userspace library
rather than every kernel on the Internet. From QUIC's perspective UDP is not a
transport at all; it is **a way to get a datagram through the world's middleboxes to
a specific process**, and nothing more. That is a striking thing to have happened to
a protocol from 1980.

## The hazards

§36.4 covers what goes wrong, and two items deserve flagging now because they are
security-relevant.

**Amplification.** A UDP request is not authenticated, so its source address can be
forged. If a small query produces a large reply, an attacker sends a small forged
query and the *victim* receives the large reply. DNS, NTP and memcached have all
been used this way, with amplification factors ranging from about 50× for DNS to
over 50,000× for a badly configured memcached server. The 1.35 Tb/s attack on GitHub
in February 2018 was memcached amplification. Chapter 62 covers the mechanism and
the mitigations, of which source address validation (BCP 38) is the one that would
have prevented all of them and which remains incompletely deployed.

**Fragmentation.** A UDP datagram larger than the path MTU fragments at the IP layer
(Chapter 24 §24.3), with all the attendant problems. This is why DNS historically
capped responses at 512 bytes, why EDNS0 raised it carefully, and why DNSSEC's large
responses reintroduced the issue.

**No congestion response.** An application sending UDP as fast as it likes will not
back off. This is fine at small scale and is a genuine hazard at large scale; RFC
8085's guidance on UDP usage exists to tell application authors what they are
obliged to implement themselves.

## By the end you will be able to

- Draw the UDP header and state each field's purpose.
- Give four distinct situations in which UDP is the correct choice, with reasons.
- Explain why TCP's reliability is harmful for real-time media, in terms of playout
  deadlines and head-of-line blocking.
- Compute the packet and round-trip cost of a small exchange over TCP versus UDP.
- Explain UDP amplification, compute an amplification factor, and state the
  mitigation.
- Explain why QUIC is built on UDP rather than on IP directly.
