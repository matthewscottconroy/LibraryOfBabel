# Chapter 66 — Performance Problems

Everything is up. Every link light is green. Every `ping` succeeds. Every monitoring
dashboard is the colour it should be. Every service responds.

And the users say it is slow.

This is the hardest class of problem in networking, and it is hard for a structural
reason: the tests that detect faults do not detect degradation. A binary check —
is it up? — cannot distinguish 940 Mb/s from 94 Mb/s, and a system that is working
badly passes every test designed to find systems that are not working.

## The first question, always

> **Is it bandwidth, latency, or loss?**

Chapter 3 established that these are independent quantities with independent causes
and independent remedies, and that "slow" maps onto all three. Determining which one
you have is the first move and it eliminates most of the possibilities.

The three tests that answer it, in order:

**Bandwidth.** `iperf3` between the two points, with parallel streams. If a single
stream is slow and sixteen streams saturate the link, you do not have a bandwidth
problem — you have a per-stream limit, which means window, latency or loss.

**Latency.** `ping`, and read `min` and `avg − min`. A high `min` is propagation or a
path that is longer than you think. A large `avg − min` is queueing.

**Loss.** Extended `ping` or `mtr` over minutes rather than seconds. Chapter 3 §3.3's
Mathis table converts a loss percentage into a throughput ceiling, and 1% loss capping
a stream at under 2 Mb/s on an 80 ms path explains a great many complaints about links that
are only 20% utilised.

§66.1 works several complaint scenarios through this triage, and the point of each is
that a single well-chosen measurement eliminates entire families of cause.

## Duplex mismatch

§66.2 covers the classic, and it is classic because the symptom is so distinctive and
the cause is so consistently missed.

One end of a link is full duplex; the other is half. This happens when one end is
hard-coded and the other is left to autonegotiate — the autonegotiating end, receiving
no negotiation partner, falls back to half duplex by the standard's rules. Both ends
believe they are configured correctly, and the link comes up.

The behaviour: the full-duplex end transmits whenever it likes. The half-duplex end
interprets simultaneous transmission as a collision, aborts, and backs off. Under light
load this is nearly invisible. Under load it is catastrophic — throughput collapses to
a small fraction of the link rate, and gets *worse* as offered load increases, which is
the opposite of most performance problems and is diagnostic in itself.

The fingerprint is unambiguous and lives in the counters: late collisions on the
half-duplex end, alignment and CRC errors on both. A late collision — one detected
after the first 64 bytes — is never normal on a correctly configured link, and its
presence means either a duplex mismatch or a segment that exceeds the maximum length.

The remedy is to let both ends autonegotiate, which has been reliable for two decades
despite the folklore that says otherwise. Hard-coding one end is the cause, not the
cure.

## MTU, fragmentation, and black holes

§66.3 consolidates the material from Chapter 24 §24.3 and Chapter 34 §34.4, because
this is where it is applied.

The signature, worth memorising: small packets work, large packets do not. The
connection establishes — the handshake is small — and then hangs on the first
substantial transfer. SSH shows its banner and freezes. A web page's headers arrive and
the body does not. A file copy starts at zero bytes and stops.

The test takes ten seconds:

```
ping -M do -s 1472 <destination>     # 1472 + 28 = 1500, DF set
ping -M do -s 1400 <destination>
```

If the first fails and the second succeeds, the path MTU is below 1500 and the ICMP
message telling you so is being blocked.

Causes: a tunnel (Chapter 61 — every VPN reduces the usable MTU by its encapsulation
overhead), a PPPoE link (8 bytes), a mismatched jumbo frame configuration (Chapter 3
§3.1 — one device on the path not configured for it), or a firewall dropping ICMP
Type 3 Code 4.

Fixes, in order of preference: permit the ICMP message, which is the correct fix; set
the MTU correctly on the tunnel interface; or clamp the TCP MSS at the tunnel endpoint,
which is the pragmatic workaround that carriers use because it does not depend on
anyone else's firewall policy.

## Bufferbloat

§66.4 covers the problem that took the industry twenty years to notice, and it is a
genuinely interesting case of a well-intentioned decision producing a systemic
failure.

Memory got cheap, so device vendors added large buffers, reasoning that a bigger
buffer means fewer drops and fewer drops means better performance.

But Chapter 38 established that TCP uses loss as its congestion signal. A large
buffer does not prevent congestion; it *hides* it. Packets that would have been dropped
are instead queued — for hundreds of milliseconds, sometimes seconds — and TCP,
receiving no loss signal, keeps increasing its window. The queue grows until it finally
overflows, by which time every packet crossing that link, including packets from
completely unrelated latency-sensitive flows, is delayed by the full depth of the
queue.

The observable result is familiar to everyone: a large upload makes an unrelated
video call unusable, on a connection with plenty of capacity, and the effect
disappears the moment the upload finishes. Latency under load, measured properly,
can rise from 20 ms to over a second.

Jim Gettys named and characterised the problem in 2010–2011 after investigating exactly
this at home. The remedies are **active queue management** — CoDel, FQ-CoDel, PIE,
CAKE — which drop or mark packets based on *how long they have been queued* rather than
on queue occupancy, keeping latency bounded regardless of buffer size. FQ-CoDel is now
the default on Linux and in most consumer router firmware, and it is one of the more
satisfying instances of a research result reaching deployment.

The lesson worth generalising: more buffer is not better buffer. A buffer's job is
to absorb bursts, not to store a backlog, and the diagnostic that reveals the
difference is latency measured *under load* rather than on an idle link.

## By the end you will be able to

- Triage a "slow" complaint into bandwidth, latency or loss with three measurements.
- Recognise duplex mismatch from its counters and its load-dependent behaviour.
- Diagnose an MTU problem in two commands and choose among the three fixes.
- Explain bufferbloat mechanistically and identify it by measuring latency under load.
- Explain why more buffering made things worse, and what AQM does differently.
- Use the Mathis relation to convert an observed loss rate into an expected
  throughput, and compare against measurement.
