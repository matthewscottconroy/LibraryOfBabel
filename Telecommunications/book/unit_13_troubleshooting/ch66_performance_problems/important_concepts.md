# Chapter 66 — Important Concepts

Bandwidth, latency and loss are independent, and "it's slow" says nothing about which
*(§66.1)* — A link can have enormous bandwidth and terrible latency, low latency and no
bandwidth, or plenty of both and 2% loss that destroys everything. The reflexive response of
adding bandwidth addresses one of the three.

Run the ping while the throughput test runs *(§66.1, §66.4)* — Latency on an idle link is
the propagation; latency under load is the queueing, and the difference between the two is
the most informative measurement in this chapter.

A 64 KB window on an 80 ms path gives 6.6 Mb/s on a 10 Gb/s link *(§66.1)* — The window,
not the link. Adding bandwidth changes nothing.

0.1% loss caps a single stream at 23 Mb/s on a 20 ms path *(§66.1)* — Mathis, with
$C=\sqrt{3/2}$: throughput $\approx \mathrm{MSS}\times C / (\mathrm{RTT}\sqrt{p})$. Regardless
of whether the link is 100 Mb/s or 100 Gb/s. This arithmetic explains most "we upgraded the
circuit and nothing improved" reports.

`curl -w` localises a web complaint to one of five places in five seconds *(§66.1)* — DNS,
connect, TLS handshake, time to first byte, transfer. A `ttfb` of 2.9 seconds with an 83 ms
connect is a server problem, stated as a measurement.

The first latency question is whether the figure is close to the propagation minimum
*(§66.1)* — If it is, the network is doing everything it can; if it is three times the minimum,
the excess is queueing or a path that is not direct. London–Singapore is 147 ms and no
equipment reduces it.

For anything interactive, the average is the wrong statistic *(§66.1)* — `8.1/47.2/380/91.4`
has an average that satisfies any threshold and a maximum that makes voice unusable. Measure
p95 and p99, and alert on those (Chapter 54 §54.1).

An error is a frame that arrived damaged; a discard is a frame that arrived intact and was
thrown away *(§66.2)* — **Entirely different faults**: errors are Layer 1, discards are
congestion. And output discards on an interface averaging 30% is the microburst signature —
the strongest indicator that a five-minute graph is hiding something.

CRC with late collisions is a duplex mismatch; CRC without is physical *(§66.2)* — A late
collision cannot occur on a correctly-operating full-duplex link, so its presence proves one
end believes it is half duplex.

A forced port advertises nothing, so the auto port falls back to half duplex *(§66.2)* —
The link comes up, the configuration looks reasonable at both ends, and one side is full and
the other half. Both auto, or both forced identically — never one of each, and auto should
be the default everywhere.

Duplex mismatch collapses sustained transfer to 1–5% of the link rate while small transfers
work perfectly *(§66.2)* — Which is why it is diagnosed late: users report "the file server
is slow", not "the network is broken".

802.3x pause frames stop everything on the link, including traffic that was not causing the
congestion *(§66.2)* — Head-of-line blocking at Layer 2, and it propagates upstream.
Disabled by default on modern equipment, and finding it enabled on a general-purpose network is
a finding. Priority flow control (802.1Qbb) pauses per class and is the correct mechanism where
one is needed.

A counter's absolute value is nearly useless; the rate is the evidence *(§66.2)* — 10,000
CRC errors over four years is nothing; 10,000 in an hour is a fault. Clear and re-read over a
measured interval — and record that you cleared them, because you destroyed evidence.

A SPAN port does not show errors *(§66.2)* — The switch discards errored frames before
mirroring, so a capture cannot substitute for the counters.

Small packets work, large packets vanish *(§66.3)* — `ping` succeeds, SSH connects and
hangs on the first long output, a page's HTML loads and its images do not, a transfer stalls at
a few kilobytes, a VPN establishes and carries nothing. All the same fault, diagnosed in one
command.

The black hole is the filtered ICMP, not the MTU *(§66.3)* — The tunnel should send
Fragmentation Needed; something filters it (Chapter 60 §60.1), so the sender retransmits the
same oversized packet with exponential backoff until it gives up. That capture signature is
an MTU black hole and nothing else.

IPv6 routers do not fragment, so PMTUD is mandatory *(§66.3)* — Filtering ICMPv6 Packet Too
Big breaks IPv6 completely, and there is no fallback.

MSS clamping fixes TCP reliably and requires nothing of the endpoints *(§66.3)* — The
device rewrites the MSS option in passing SYNs and neither end knows. MTU − 40 for IPv4, − 60
for IPv6. Its limitation is exact: TCP only — QUIC, UDP VPNs and large DNS responses are
unaffected, which matters increasingly (Chapter 38 §38.4).

Jumbo frames must be configured on every device in the path *(§66.3)* — One device at the
default breaks it for everyone, and the symptom is "giants" counted and discarded.

Fragmentation is avoided rather than made to work *(§66.3)* — Losing one fragment loses the
whole packet, only the first carries the transport header so firewalls cannot classify the rest,
and many firewalls drop fragments outright.

Bufferbloat is the network being broken by an attempt to improve it *(§66.4)* — Memory
became cheap, deeper buffers drop fewer packets, and dropping looks like a defect. And
loss-based TCP needs loss, so it fills the buffer entirely before receiving any signal. Every
individual decision was locally reasonable.

The same buffer is harmless at 100 Mb/s and catastrophic at 1 Mb/s *(§66.4)* — 256 KB adds
5 ms at 100 Mb/s and 2,048 ms at 1 Mb/s — which is why the problem concentrates on slow
uplinks, and why an asymmetric connection bloats on the upload first.

Four reasons bufferbloat is missed *(§66.4)* — The throughput is fine (a speed test
measures throughput); the idle latency is fine (monitoring pings an idle link); the
utilisation graph looks reasonable (five-minute averages); and the symptom is attributed to
Teams, the VPN or the Wi-Fi — all correct observations with a cause on a different device.

CoDel measures how long packets are staying, not how many there are *(§66.4)* — Above 5 ms
of minimum queueing delay for longer than 100 ms, it begins dropping. And it has no parameters
to tune, deliberately — because RED's parameters were why RED shipped everywhere and was
enabled almost nowhere.

FQ-CoDel works without any classification at all *(§66.4)* — Per-flow sub-queues mean a
bulk transfer cannot delay a voice packet regardless of markings, which sidesteps Chapter 52
§52.2's trust boundary problem entirely.

`tc qdisc replace dev eth0 root cake bandwidth 47500kbit` is one line and the most effective
change available to most people *(§66.4)* — 800 ms of loaded latency becoming 15 ms is
routine, and almost nobody makes it. The `bandwidth` parameter is the shaper — about 95% of
the real rate — so the queue forms in your device rather than the carrier's.

AQM without a shaper achieves nothing *(§66.4)* — The queue is still forming in the
carrier's device, and this is the commonest disappointment. Chapter 52 §52.3's argument,
arriving as a diagnosis.

BBR is a sender-side mitigation; AQM is a network-side fix *(§66.4)* — A BBR sender does
not bloat the buffer and does not fix it for anyone else, and the network-side answer helps
every flow including the ones you do not control.
