# 36.3 UDP Applications

The protocols that chose UDP, and — more usefully — why each one did. §36.2 gave four
criteria; this section applies them, because seeing the reasoning repeated across a dozen
protocols is what makes it transferable.

## DNS — port 53

**The archetype.** Chapter 39 covers it properly.

**Why UDP:**

| Criterion | Applies? |
|---|---|
| Short transaction | **yes** — one query, one response |
| Timeliness | **yes** — a resolver is on the critical path of every connection |
| Application does reliability better | **yes** — retry with a different server |
| One-to-many | no |

**The retry logic is the interesting part.** A resolver that gets no answer within a
timeout retries against a different server — which is better than TCP retransmitting to
the same unresponsive one. The application's knowledge (there are several servers) beats
TCP's generic mechanism.

And it uses TCP too, when the response exceeds what a datagram carries — historically
512 bytes, now negotiated up to ~4096 with EDNS0 (Chapter 39 §39.2). Zone transfers
always use TCP, because they are bulk.

> DNS is the clearest case of "use both, each where it fits." A firewall permitting
> only UDP/53 breaks large responses and DNSSEC.

## DHCP — ports 67 and 68

**Why UDP: because nothing else could work.**

**The client has no IP address.** It broadcasts from `0.0.0.0` to `255.255.255.255`
(Chapter 40 §40.2).

**TCP cannot do this.** A TCP handshake requires a source address to receive the SYN-ACK,
and the client does not have one. TCP is point-to-point and the client does not know who
the server is.

This is the bootstrap problem of Chapter 18 §18.2, and UDP over broadcast is the only
mechanism that solves it.

## TFTP — port 69

Trivial File Transfer Protocol, and it is worth a moment because it *does* implement
reliability over UDP — badly, and deliberately.

**Lock-step acknowledgement:** send a 512-byte block, wait for its ACK, send the next.

**Throughput is terrible** — one block per round trip, so on a 20 ms path that is
25 KB/s regardless of the link speed.

And it was the right choice, because TFTP exists to be implemented in a few kilobytes
of boot ROM on a device with no operating system (Chapter 41 §41.2). A TCP stack does not
fit; a lock-step UDP loop does.

It survives in PXE boot and network device firmware loading, and nowhere else.

## SNMP — ports 161 and 162

**Why UDP:** a monitoring system polls thousands of devices, frequently.

TCP's cost would be per-device state and a handshake per poll. For a poll every 60
seconds across 5,000 devices, that is 5,000 handshakes per minute for transactions of a few
hundred bytes.

And loss is acceptable — a missed poll is a gap in a graph, and the next poll is 60
seconds away. The application's tolerance for loss is high, which is §36.2's second
criterion.

Traps (162) are fire-and-forget, which is a genuine weakness: an SNMP trap that is
lost is lost silently, and the management system never knows the device tried to report
something. SNMPv2 added `INFORM`, which is acknowledged, for exactly this reason.

## Syslog — port 514

**Why UDP:** volume, and the same tolerance argument.

**And it is a real weakness.** Syslog over UDP:

- Loses messages under load, **silently**
- Has **no delivery confirmation**
- Is **trivially spoofable** — anything can send a log line claiming to be anything

Which matters most exactly when it matters most: during an incident, log volume spikes,
and UDP syslog drops messages precisely when you need them. An attacker who wants to hide
can flood the collector and the evidence is lost with no trace of the loss.

RFC 6587 defines syslog over TCP, and RFC 5425 over TLS. Use them for anything
security-relevant. UDP syslog is acceptable for high-volume operational logging where
loss is tolerable and is not acceptable as an audit trail.

## NTP — port 123

**Why UDP:** the transaction is a measurement, and TCP would corrupt it.

This is the most interesting case in the section.

NTP works by timestamping (Chapter 41 §41.3): the client records when it sent, the server
records when it received and replied, the client records when it received. From four
timestamps it computes the offset and the round-trip delay.

**TCP's retransmission would destroy the measurement.** A retransmitted packet arrives late
by a variable amount, and the client cannot tell a retransmitted packet from a slow
one — so the delay measurement would be wrong, and the clock would be set wrong.

> NTP needs to know how long the packet took. Any mechanism that hides delay makes the
> measurement useless.

A lost NTP packet is simply skipped and the next sample is used, which is exactly
right — the application's requirement is *an accurate measurement*, not *this particular
measurement*.

## RTP — voice and video

**Why UDP:** §36.2's second criterion, in its purest form.

RTP (RFC 3550) runs over UDP and carries a sequence number and a timestamp of its own —
so the application can detect loss and reordering and decide what to do about it, which
is the point.

**What a codec does with loss:**

| Loss | Response |
|---|---|
| One audio packet (20 ms) | **conceal it** — interpolate; usually inaudible |
| A video P-frame | **skip it**; brief artefacts |
| A video keyframe | **request a new one** — application-level retransmission of the thing that matters |

**Notice the third row.** The application *does* retransmit — but only the keyframe, and
only because it knows a keyframe is worth retransmitting and a P-frame is not.

TCP cannot make this distinction, because it does not know what a frame is (Chapter 21
§21.3). This is §36.2's third criterion made concrete: the application knows something
the transport cannot.

**RTCP** (the control channel) carries loss and jitter statistics back to the sender, which
uses them to adapt the codec rate — congestion response implemented in the application,
because the transport does not provide it. §36.4 argues this is an obligation, not an
option.

## Routing protocols

**RIP** uses UDP 520 (Chapter 31 §31.1) — periodic multicast updates, and a lost update is
resent in 30 seconds.

**OSPF** and **EIGRP** use **neither** — they run directly on IP (protocols 89 and 88) and
implement their own reliability, because they need acknowledgements with properties neither
TCP nor UDP provides.

**BGP uses TCP** (Chapter 32 §32.1), because its updates are incremental and must never be
lost — once sent, a route is assumed held until withdrawn, which requires reliable
delivery.

Three protocols, three different answers, each derived from what the protocol needs.
That is the pattern this section is teaching.

## The modern ones

### QUIC — port 443/UDP

The most important UDP application by volume, and it inverts the chapter's framing.

QUIC provides reliability, ordering, flow control and congestion control — everything
TCP does — **over UDP**.

**Why not just use TCP?** Chapter 38 §38.4 gives the full answer; briefly:

- **Per-stream reliability** — a loss in one stream does not block others, which TCP
  cannot do
- **Faster handshake** — transport and cryptographic setup combined, 1-RTT or 0-RTT
- **Connection migration** — survives a change of IP address, so a phone moving from Wi-Fi
  to cellular keeps its connection
- **Deployable** — it is in user space, so it can be updated without changing kernels, and
  middleboxes cannot ossify it (Chapter 21 §21.4)

UDP is not being used for its unreliability here. It is being used as a substrate,
because deploying a genuinely new IP protocol is impossible (Chapter 23 §23.2).

### WireGuard, VXLAN, GENEVE, GRE-in-UDP

Tunnelling protocols use UDP as an encapsulation layer — for the same reason as QUIC:
it traverses NAT and firewalls, and its 8-byte header is the cheapest available wrapper.

The payload has its own reliability, so UDP's absence of it is exactly what is wanted.

## The summary table

| Protocol | Port | Why UDP |
|---|---|---|
| **DNS** | 53 | short transaction, latency-critical, better retry logic |
| **DHCP** | 67/68 | **client has no address; must broadcast** |
| **TFTP** | 69 | must fit in boot ROM |
| **NTP** | 123 | **retransmission would corrupt the measurement** |
| **SNMP** | 161/162 | high poll volume, loss tolerable |
| **syslog** | 514 | volume — **and it is a real weakness** |
| **RTP** | dynamic | **timeliness beats completeness; the app knows what to retry** |
| **QUIC** | 443 | **a substrate — reliability is implemented above it** |
| **VXLAN, WireGuard** | 4789, 51820 | cheap encapsulation that traverses NAT |
| RIP | 520 | periodic; loss self-corrects |
| **Multicast anything** | — | **TCP cannot do one-to-many** |

## What breaks here

**A firewall permitting only UDP/53.** Large DNS responses and DNSSEC fail.

**UDP syslog as an audit trail.** Messages are lost silently, exactly when volume spikes,
and the log is spoofable. Use TCP or TLS syslog.

**An SNMP trap that never arrived.** Fire-and-forget. Use `INFORM` if it matters.

NTP behind something that retransmits or queues variably. The measurement is corrupted,
and the clock will be wrong in a way that is hard to attribute.

**TFTP over a long path.** Lock-step, so throughput is bounded by RTT regardless of
bandwidth.

**Blocking UDP/443 to "block unreliable traffic".** It blocks HTTP/3, which is more capable
than TCP.

> **Network+ note.** Objective 1.4 expects which protocols use UDP. Over-learn: DNS,
> DHCP, TFTP, SNMP, syslog, NTP, and voice/video (RTP). And the reasoning, which makes
> the list derivable rather than memorised: short transactions, real-time data,
> broadcast, and applications that can do better than TCP.
