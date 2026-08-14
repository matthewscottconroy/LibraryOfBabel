# 36.1 The Minimal Header

RFC 768 is **three pages**, including the title page and the references. Jon Postel wrote
it in August 1980 — the design owes much to David Reed's end-to-end thinking — and it has
never been revised.

It is the most restrained specification in the Internet suite, and reading it is the
best possible introduction to what a transport layer is obliged to do — because UDP does
only that and nothing more.

## The header

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-------------------------------+-------------------------------+
|          Source Port          |       Destination Port        |
+-------------------------------+-------------------------------+
|            Length             |           Checksum            |
+-------------------------------+-------------------------------+
|                            data                               |
+---------------------------------------------------------------+
```

**Eight bytes. Four fields.**

| Field | Bits | Purpose |
|---|---|---|
| **Source port** | 16 | who sent it — **optional; may be 0** |
| **Destination port** | 16 | **the demultiplexing key** (Chapter 35 §35.1) |
| **Length** | 16 | header + data, in bytes; **minimum 8** |
| **Checksum** | 16 | integrity — **optional in IPv4, mandatory in IPv6** |

Compare with TCP's twenty bytes minimum (Chapter 37 §37.1) and the difference is the
whole of this chapter.

## Field by field

### Source port — optional

**It may be zero**, meaning *"no reply expected"*.

**This is unusual and it is deliberate.** A protocol that only ever pushes data one way —
some logging, some telemetry — has no use for a return address, and UDP does not insist on
one.

In practice almost everything sets it, because almost everything wants a reply, and the
receiving application needs somewhere to send it.

### Destination port — the only mandatory identifier

**The demultiplexing key.** Without it UDP would be indistinguishable from raw IP, and
Chapter 35 §35.1's problem would be unsolved.

**This is UDP's core contribution**, and arguably its only essential one: **it adds
process-level addressing to IP and nothing else that is strictly required.**

### Length — and its redundancy

**Header plus data**, minimum 8 (a header with no payload — which is legal and is used as a
keepalive or a probe).

It duplicates information already in the IP header, which carries a total length from
which UDP's length is derivable. **The redundancy was deliberate**: it lets the UDP layer
validate independently of IP, and it costs two bytes.

**Maximum 65,535**, so the largest UDP payload is 65,507 bytes (65,535 − 8 UDP − 20 IP).
In practice the path MTU binds long before this (Chapter 34 §34.4), and a UDP datagram
larger than ~1,472 bytes will be fragmented — which §36.4 argues you should avoid.

### Checksum — optional, and the option is a trap

In IPv4, the UDP checksum may be zero, meaning *"not computed"*.

**This was a 1980 performance concession.** Computing a checksum over the payload cost real
CPU on a PDP-11, and for applications on a reliable local network it seemed a reasonable
saving.

It is a bad idea now, for a reason worth understanding:

The IP header checksum covers only the header (Chapter 24 §24.2). **Nothing else
checks the payload end to end.** Ethernet's FCS (Chapter 15 §15.4) is recomputed at every
hop — so corruption *inside a router*, in memory or on a bus, is caught by nothing.

> **With the UDP checksum disabled, a bit flip in a router's memory reaches the application
> undetected.**

This is not hypothetical; measured rates of undetected corruption in the Internet's core
are low and non-zero, and the applications that discovered this the hard way tended to be
storage protocols.

**IPv6 makes the checksum mandatory**, and the reason is exactly the above: IPv6 removed
the IP header checksum entirely (Chapter 24 §24.2), so **UDP's checksum is the only
integrity check between the link layer and the application.**

**The one exception** is tunnelling protocols such as VXLAN, where RFC 6935 permits a zero
UDP checksum in IPv6 because the encapsulated payload has its own — a narrow, argued-for
carve-out rather than a general licence.

### The pseudo-header

UDP's checksum, like TCP's, covers a pseudo-header containing the IP addresses,
protocol and length (Chapter 21 §21.4):

```
   ┌─────────────────────────────┐
   │      Source IP address      │  ← from the IP header
   ├─────────────────────────────┤
   │   Destination IP address    │  ← from the IP header
   ├────────┬────────┬───────────┤
   │  zero  │  17    │ UDP len   │
   └────────┴────────┴───────────┘
```

So UDP reads the IP header, for the same reason TCP does — to detect a datagram
misdelivered to the wrong host — and with the same consequence: **a NAT that rewrites
addresses must recompute the UDP checksum** (Chapter 33 §33.2).

## What UDP does not have

The list is longer than the header, and every omission is deliberate:

| Absent | Consequence |
|---|---|
| **Connection setup** | **no handshake — the first packet carries data** |
| Acknowledgements | the sender never learns whether it arrived |
| **Sequence numbers** | **no ordering, no duplicate detection** |
| Retransmission | lost is lost |
| **Flow control** | a fast sender can overwhelm a slow receiver |
| **Congestion control** | **a UDP sender does not slow down** |
| Segmentation | the application must fit the datagram to the path |
| Connection teardown | nothing to tear down |

The last two rows in bold are the ones with consequences beyond the application, and
§36.4 develops them.

## The service it provides

**Exactly two things beyond raw IP:**

1. **Process-level addressing** — the destination port
2. **Optional payload integrity** — the checksum

And one property that is not a feature but is essential to understand:

3. **Datagram boundaries are preserved.**

A `send()` of 100 bytes arrives as a `recv()` of 100 bytes. One datagram, one read.

**TCP does not do this.** TCP is a **byte stream** (Chapter 37 §37.2): three writes of 100
bytes may arrive as one read of 300, or two reads of 150, or any other division. **The
application must frame its own messages.**

> **This is a genuine feature, not merely an absence.** For a protocol whose unit is a
> message — a DNS query, a log line, an audio sample — UDP's boundary preservation removes
> an entire category of work.

It is the most under-appreciated reason to choose UDP, and it is the reason a DNS
resolver's code is simpler than an HTTP client's.

## Reading it

```
$ tcpdump -i eth0 -nn udp port 53
14:23:01.123456 IP 10.0.0.5.51234 > 1.1.1.1.53: 39847+ A? example.com. (29)
14:23:01.145678 IP 1.1.1.1.53 > 10.0.0.5.51234: 39847 1/0/0 A 93.184.216.34 (45)
```

Two packets. A query and a response. That is the entire transaction — no handshake, no
teardown, no acknowledgement.

**Compare with TCP**, where the same exchange would be SYN, SYN-ACK, ACK, query, ACK,
response, ACK, FIN, ACK, FIN, ACK — **eleven packets for one question.**

**The wire cost:**

| | UDP | TCP |
|---|---|---|
| Packets for one query/response | **2** | ~11 |
| Round trips before data | **0** | **1** (the handshake) |
| Header overhead | 8 bytes | 20+ bytes |
| State at the server | **none** | one control block per connection |

**The round-trip column is the one that matters.** On a path with 50 ms RTT, TCP's
handshake costs 50 ms before the first byte of the query is sent. For a DNS lookup
that would otherwise complete in 50 ms, that is a doubling.

This is why DNS uses UDP, and it is the clearest possible illustration of §36.2's
argument.

## What breaks here

**A UDP checksum of zero in IPv4.** Legal, and it means nothing checks the payload
end to end. Enable it.

**A UDP datagram larger than the path MTU.** It fragments, and Chapter 24 §24.3's problems
follow. §36.4.

**Expecting ordering.** There is none.

**Expecting delivery.** There is none.

**Assuming a `recv()` matches a `send()` in TCP.** It does not — that is UDP's property,
not TCP's, and confusing them is a common source of application bugs in the other
direction.

> **Network+ note.** Objective 1.4 expects UDP and the TCP/UDP comparison. Over-learn:
> **the header is 8 bytes with four fields**; **UDP is connectionless and unreliable**;
> **no handshake, no acknowledgement, no ordering, no congestion control**; and **the
> checksum is optional in IPv4 and mandatory in IPv6.** The 8-versus-20-byte header
> comparison is examined directly.
