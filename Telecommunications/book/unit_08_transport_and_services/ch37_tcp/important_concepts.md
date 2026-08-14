# Chapter 37 — Important Concepts

The handshake answers a narrow question *(§37.1)* — two hosts must agree on where
each byte stream starts. Three packets: **SYN** (client's ISN sent), **SYN-ACK**
(server's ISN sent *and* client's acknowledged), **ACK** (server's acknowledged). Two is
insufficient because the server's ISN would be unconfirmed; four is unnecessary
because the middle packet does two jobs.

SYN consumes a sequence number *(§37.1)* — Hence `ack = x+1`. So does FIN. Without it,
an ACK for a SYN would be indistinguishable from an ACK of nothing. An acknowledgement
number is always the next byte expected.

Initial sequence numbers must be unpredictable *(§37.1)* — Early stacks used a simple
counter, enabling **blind spoofing**: an attacker who can predict the server's ISN completes
the handshake from an address they do not control. This is the Mitnick attack of December
1994, described by Bellovin five years earlier. RFC 6528 makes the ISN a keyed hash of the
tuple.

The handshake is weak authentication as a side effect *(§37.1)* — An off-path
attacker cannot complete it, because they cannot guess the server's ISN. UDP has no
equivalent, which is why UDP spoofing is trivial.

The handshake negotiates four options, once, permanently *(§37.1)* — **MSS**, **window
scale**, **SACK permitted**, **timestamps**. If a middlebox strips one, the connection
runs without it for its entire life — and a stripped window scale means the 64 KB wall
forever, with the symptom being a connection that works and is inexplicably slow.

MSS is not the MTU *(§37.1)* — It is the largest **payload**: 1500 − 20 − 20 = **1460**.
Each side announces what it will accept and the smaller is used. MSS clamping rewrites
this option in transit, which is how VPN gateways prevent PMTUD black holes.

SYN flood and SYN cookies *(§37.1)* — A SYN costs the attacker one packet and the server
state held for a timeout. SYN cookies encode the state in the sequence number, so
the server holds nothing until the third ACK proves the handshake genuine. The cost is that
options may be lost, which is why they are a **fallback under attack**, not always on.

**Number every byte** *(§37.2)* — Not every packet. So retransmissions can be repackaged
freely — split, combined, or resized for a smaller path MTU. A packet-numbered protocol
could not.

TCP is a byte stream with no message boundaries *(§37.2)* — Three writes of 100 bytes
may arrive as one read of 300. The application must frame its own messages — length
prefix, delimiter, or fixed size. "I called `send()` once so the peer will `recv()` once"
is one of the commonest beginner bugs, and it works on a LAN and fails in production.

**Cumulative acknowledgement** *(§37.2)* — "I have received everything up to but not
including this byte." Consequences: lost ACKs are harmless (the next one covers
everything); ACKs need not be one-per-segment; and it cannot express a gap.

SACK expresses the gap *(§37.2)* — Lists the blocks that *have* arrived, so the sender
retransmits only what is missing rather than everything from the loss onward. Universally
supported; if a capture shows it absent, a middlebox stripped it and the connection will
perform badly under loss.

Three duplicate ACKs are the loss signal *(§37.2)* — Out-of-order arrival makes the
receiver repeat its last ACK. Three, because reordering also produces duplicates —
one or two is more likely reordering, and three has held up remarkably well as the
threshold.

**Sequence wraparound** *(§37.2)* — 4 GB of space wraps in 34 seconds at 1 Gb/s and
0.34 s at 100 Gb/s. **PAWS** uses the timestamp option to discard segments whose
timestamp is stale regardless of sequence number.

Two recovery mechanisms with very different costs *(§37.3)* — Fast retransmit ≈ 1
RTT; RTO ≥ 200 ms, often ~1 s, sometimes minutes. A single loss at the end of a
transfer costs an RTO; the same loss in the middle costs one RTT, because fast retransmit
needs data behind the loss to generate duplicates.

**Jacobson's estimator** *(§37.3)* — RFC 793 tracked the mean and ignored the variance, so
it fired constantly on erratic paths. RTO = SRTT + 4×RTTVAR adapts to the path's
*stability*: 20 ms mean with 2 ms variance gives RTO 28 ms; the same mean with 40 ms
variance gives 180 ms. The constants 1/8 and 1/4 make it shifts and adds with no
multiplication.

**Linux's RTO floor** *(§37.3)* — 200 ms minimum, so on a fast path `ss` shows the floor
rather than the formula. The formula only becomes visible when SRTT + 4×RTTVAR exceeds
it.

**Karn's algorithm** *(§37.3)* — An ACK after a retransmission is **ambiguous** — it may be
for either transmission, and guessing wrong corrupts the estimator in either direction.
Do not measure retransmitted segments at all — plus **exponential backoff**, because
refusing to measure means the RTO cannot adapt during exactly the period it needs to.
Timestamps remove the ambiguity and modern stacks use them.

**Exponential backoff** *(§37.3)* — 1, 2, 4, 8, 16, 32, 64 s. This is why a connection to
an unreachable host takes minutes to fail.

**Fast recovery** *(§37.3)* — Duplicate ACKs prove segments are still arriving, so the path
is lossy rather than broken. Halve the window and continue rather than restarting from
one segment.

**Tail loss** *(§37.3)* — Nothing behind the last segments to generate duplicates, so
recovery falls to the RTO. Short transfers are almost all tail, which is why page-load
latency is dominated by loss more than bandwidth suggests. **TLP** probes early; **RACK**
replaces duplicate-counting with time-based reasoning about what should have arrived by
now, handling both reordering and tail loss better. Now default in Linux and QUIC.

Flow control protects the receiver; congestion control protects the network *(§37.4)* —
Different mechanisms, different problems. The sender is bound by min(rwnd, cwnd), and
which one binds tells you where the bottleneck is.

Throughput ≤ window ÷ RTT *(§37.4)* — The most important formula in transport
performance. Rearranged, BDP = bandwidth × RTT is the window needed to fill a path.

The 64 KB wall *(§37.4)* — A 16-bit window field caps throughput at 5.2 Mb/s at
100 ms RTT. On a gigabit intercontinental path an unscaled connection achieves half a
per cent of the link, and it is not the network's fault.

**Window scaling** *(§37.4)* — A shift of up to 14, giving a 1 GB maximum. Negotiated
only in the handshake, so a middlebox that strips it imposes the wall permanently. This
is the classic "the network is slow" complaint that is not the network.

Autotuning, and how applications break it *(§37.4)* — Modern stacks grow buffers as the
BDP becomes apparent. An application calling `setsockopt(SO_RCVBUF)` disables autotuning
for that socket and usually caps itself far below what it would have got.

A zero window is definitively not a network problem *(§37.4)* — The receiving
*application* is not reading. The data arrived; nobody consumed it. Window probes
exist because ACKs are not retransmitted, so a lost window update would deadlock.

Nagle versus delayed ACK *(§37.4)* — Two sensible optimisations combining badly: the
sender holds a small write awaiting an ACK, the receiver delays the ACK up to 200 ms.
A 200 ms stall per request on a 1 ms path. Fix with `TCP_NODELAY`, or — usually
better — **combine the writes.**

**The four-way close** *(§37.5)* — FIN, ACK, FIN, ACK. Four rather than three because the
server may still have data to send — the **half-close** is a real feature, used by
`ssh host 'cat > file' < input`.

**TIME-WAIT** *(§37.5)* — Entered by whichever side closes first, held for 2×MSL
(60 s on Linux). Two reasons: late segments must not corrupt a new connection using the
same tuple, and the final ACK may be lost, so the closer must be able to resend it.

Where the TIME-WAIT cost falls *(§37.5)* — **On a client**, as ephemeral exhaustion
(~470 connections/s sustained). A server with many TIME-WAIT sockets is closing first,
which is worth investigating. `tcp_tw_reuse` is safe; `tcp_tw_recycle` broke every client
behind NAT and was removed in Linux 4.12; reducing MSL is wrong because it reintroduces
the hazard.

CLOSE-WAIT is never a network problem *(§37.5)* — The peer sent FIN and the local
application has not called `close()`. A growing count is a file-descriptor leak whose
endgame is a service refusing connections hours after the bug ran.

**RST** *(§37.5)* — Abrupt, unacknowledged, data discarded. Six different causes all
produce "connection reset by peer", and only a capture distinguishes them: immediately
after SYN means nothing listening; from a third party's address means injected;
**after idleness** means a middlebox forgot the session — which is why **TCP keepalives**
matter and why the two-hour default is far too long for middleboxes that forget in
5–30 minutes.

The state distribution is a diagnosis *(§37.5)* — `ss -tan | awk '{print $1}' | sort |
uniq -c`. CLOSE-WAIT means an application bug; SYN-SENT means a network problem;
SYN-RECV means possibly a flood; FIN-WAIT-2 means the peer is not closing. One command,
five different problems distinguished.
