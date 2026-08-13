# Chapter 37 — TCP

Here is the problem, stated as starkly as it deserves.

You have a channel that loses messages, duplicates them, delivers them out of
order, corrupts them, and delays them by an amount that varies unpredictably by
three orders of magnitude. It provides no notification of any of these events. The
two parties have never met, share no clock, cannot see each other, and are separated
by a path that may change mid-conversation.

**Build a reliable, ordered byte stream on top of it.**

That is the Transmission Control Protocol, and it is the most consequential piece of
software engineering in this book. It carries the overwhelming majority of Internet
traffic. It was specified in 1981, has been extended perhaps forty times, and the
original header format is unchanged. Every operating system implements it; they
interoperate; and the reason they interoperate is a body of specification work —
RFC 793, then 1122, then 5681, 6298, 7323, 9293 — that is one of the field's genuine
achievements.

## The five problems, and the five mechanisms

The chapter is organised around the observation that TCP is not one idea but five,
each solving a distinct problem, each comprehensible on its own.

**How do two parties agree to start?** Both must know the other is there, both must
exchange initial state, and both must be protected against an old duplicate packet
from a previous conversation being mistaken for a new request. The **three-way
handshake** (§37.1) solves all three, and the reason it is three messages rather
than two is worth understanding rather than memorising.

**How does the receiver know what it missed and in what order?** Every byte is
numbered. The receiver acknowledges the highest contiguous byte received.
**Sequence numbers and cumulative acknowledgement** (§37.2) give ordering and gap
detection from one counter.

**What if data is lost?** The sender keeps a copy until acknowledged and resends
after a timeout — but choosing that timeout is subtle, because too short wastes
capacity and too long stalls the transfer, and the correct value depends on an RTT
that varies. **Retransmission with adaptive timers** (§37.3), including Karn's
algorithm and Jacobson's RTT estimator, plus the fast retransmit that avoids waiting
for a timer at all.

**What if the sender is faster than the receiver?** The receiver advertises how much
buffer space it has; the sender never exceeds it. **Flow control via the sliding
window** (§37.4) — and this is Chapter 3 §3.4's bandwidth–delay product made
concrete, including the zero-window and silly-window situations.

**How do they agree to stop?** Symmetrically and independently in each direction,
with a state machine that must handle the last acknowledgement being lost.
**Teardown and TIME_WAIT** (§37.5).

Note that **congestion control is not in this list.** It is a sixth problem — what
if the *network* is the bottleneck rather than the receiver — and it was not in the
original TCP at all. It was added in 1988, after the network collapsed. Chapter 38
is that story, and separating it from flow control is one of the more useful
distinctions in this unit, because students conflate them constantly: flow control
protects the receiver, congestion control protects the network, and they are
independent mechanisms with independent windows.

## Why three messages

A small thing, asked in every interview, and worth getting right for the reason
rather than the answer.

Two messages establish that A can reach B and that B knows A wants to talk. They do
not establish that **B's response reached A**, so B does not know whether A is
listening. Three messages close that loop: after the third, both parties know that
both parties know the connection exists.

There is a second, subtler reason, and it is the one Tomlinson identified in 1975.
Initial sequence numbers must be exchanged and confirmed. If A's SYN carrying its
ISN were not acknowledged, a *delayed duplicate* of an old SYN from a previous
connection could establish a new one with stale state. The three-way handshake makes
both sides confirm both sequence numbers before any data flows.

Initial sequence numbers are also, for that reason, randomised — RFC 6528 —
because predictable ISNs allowed connection spoofing, an attack Robert Morris
described in 1985 and Kevin Mitnick used in 1994.

## What a capture teaches

§37.1 through §37.5 each pair the mechanism with a real Wireshark trace, because TCP
is a protocol you learn to *see*.

The visible signatures are worth previewing, since recognising them is most of what
Chapter 66 asks:

- **Duplicate ACKs** in a row — a gap in the sequence space. Three of them trigger
  fast retransmit. Seeing them means loss, and their count tells you roughly how
  much.
- **A retransmission with a growing gap before it** — the RTO timer firing and
  backing off exponentially. Slower and more damaging than fast retransmit.
- **TCP Zero Window** — the receiving *application* is not reading fast enough. The
  network is innocent; the problem is on the host.
- **A window that never grows past 64 KB** on a high-latency path — window scaling
  absent or stripped (Chapter 3 §3.4).
- **SYN, SYN, SYN with no reply** — nothing listening, or a firewall dropping
  silently. Compare with an immediate RST, which means the host is there and
  actively refusing.

That last distinction — silent drop versus RST — is worth internalising early. A
**RST** means a machine answered. A **timeout** means nothing answered. They are
different diagnoses with different next steps, and a great deal of time is wasted by
treating "connection failed" as one symptom.

## By the end you will be able to

- Trace a three-way handshake in a capture, identify the ISNs, and explain why three
  messages are required.
- Explain cumulative acknowledgement and predict the ACK numbers for a given
  sequence of segments, including a gap.
- Explain the retransmission timer, why RTT estimation is needed, and what Karn's
  algorithm fixes.
- Distinguish flow control from congestion control precisely.
- Compute maximum throughput from window size and RTT, and identify a
  window-limited transfer from a capture.
- Draw the TCP state machine and explain TIME_WAIT's purpose and duration.
- Diagnose the five common capture signatures above.
