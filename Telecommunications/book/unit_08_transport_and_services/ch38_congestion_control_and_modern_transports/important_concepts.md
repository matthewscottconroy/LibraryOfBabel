# Chapter 38 — Important Concepts

**October 1986** *(§38.1)* — The LBL–Berkeley link, 400 metres apart, fell from 32 kb/s
to 40 bits per second. Nothing was broken.

**The collapse mechanism** *(§38.1)* — Load rises → a queue fills → packets drop → senders
time out and retransmit → the retransmissions add load → more drops → the link carries
almost entirely retransmissions of packets that will be dropped.

It is a stable state, not degradation *(§38.1)* — The network carries almost nothing
useful and is completely busy doing so, and it does not recover when load falls slightly,
because the retransmissions are themselves load.

Why 1981 TCP permitted it *(§38.1)* — It had flow control and no congestion control.
The assumption was that the receiver was the bottleneck, which was true when hosts were
slow and links dedicated. By 1986 links were shared and hosts were fast, and the
assumption had quietly inverted.

**Conservation of packets** *(§38.1)* — A connection in equilibrium should put a new
packet into the network only when an old one leaves it. The ACK is the signal that one
left, so a sender transmitting on ACKs is self-clocking — it automatically sends at the
rate the path sustains, without knowing what that rate is.

**The two windows** *(§38.1)* — `min(rwnd, cwnd)`. `cwnd` appears in no packet. No
receiver advertises it and no router requests it — every sender infers the state of a
shared resource independently, with no coordination, and the result is a stable roughly
fair allocation.

Loss as the signal, and its assumption *(§38.1)* — A drop means a queue overflowed.
Almost always true on fibre (error rates ~10⁻¹²); **false on wireless**, where loss is
interference and slowing down does not help; and false on very fast paths, where the
required loss rate becomes absurd. Every algorithm since is an attempt to get a better
signal than loss.

TCP's fairness is biased toward short paths *(§38.1)* — Equal `cwnd` among equal-RTT
flows is fair, but throughput is `cwnd`/RTT, so half the RTT gets twice the throughput.
And an application opening many connections gets many shares — which is why browsers
opened six per host, and why HTTP/2's single connection was initially slower on lossy
paths.

Slow start is exponential *(§38.1, §38.2)* — The name is misleading; it is the fastest
phase. `cwnd` += MSS per ACK doubles the window every RTT. "Slow" only relative to
starting at the receiver's full window, which is what caused the collapse.

The initial window is 10 segments *(§38.2)* — RFC 6928, 2013, raised from 1–4 after
Google measured that most web transfers were short enough for slow start to dominate
their entire lifetime. Ten segments delivers ~14 KB in the first round trip.

Congestion avoidance is linear *(§38.2)* — Roughly +1 MSS per RTT. The two phases
answer two questions: *how big is this path?* (exponential, get there fast) and *has it
got bigger?* (linear, probe gently).

AIMD converges and MIAD does not *(§38.2)* — Additive increase preserves the
difference between two flows; multiplicative decrease shrinks it. Iterated, they converge
to equal shares. AIMD is the only one of the four combinations that converges, which is
why everything since uses it or approximates it.

The sawtooth's two consequences *(§38.2)* — **Average utilisation ~75%**, because the
window oscillates between *W* and *W*/2 — a single flow cannot fill a link, and the
headroom is what lets other flows start. And **TCP needs loss**: without it there is no
signal to stop growing, so it grows until it causes loss.

**Tahoe, Reno, NewReno** *(§38.2)* — Tahoe drops to `cwnd`=1 on any loss; Reno's fast
recovery halves instead, because duplicate ACKs prove packets are still flowing; NewReno
handles multiple losses in one window. A timeout is always treated as severe, because
it means *nothing* got through.

**The Mathis equation** *(§38.2)* — throughput ≈ (MSS/RTT) × (C/√p), with C = √(3/2) ≈
1.22. Throughput falls with the square root of loss and is inversely proportional to
RTT — so a long path is penalised twice.

The numbers people disbelieve *(§38.2)* — MSS 1460, RTT 100 ms: 0.0001% loss →
142 Mb/s; 0.01% → 14 Mb/s; 1% → 1.4 Mb/s. Even one loss per million packets holds a
single classic flow to a seventh of a gigabit link. "The link is not full so the network
is fine" is wrong.

Reno cannot fill a long fat pipe *(§38.2)* — 10 Gb/s at 100 ms needs one loss in five
billion packets, and recovery from a single loss would take over an hour of linear
growth.

**CUBIC** *(§38.3)* — Default in Linux since 2006 and Windows since 2019. Window growth is
a function of time since the last loss, not of round trips — cubic, flat near the previous
maximum, fast above and below it. This removed TCP's structural bias against long paths,
which is the most important thing about it. **Still loss-based**, so it inherits the
wireless misinference and bufferbloat.

**BBR's premise** *(§38.3)* — Loss is not congestion; loss is what happens after
congestion has already filled a queue. A loss-based algorithm therefore operates, by
construction, at the point of maximum queueing delay.

**What BBR measures** *(§38.3)* — **BtlBw** (maximum delivery rate) and **RTprop** (minimum
RTT), then sends at BtlBw with BDP in flight. It aims for the pipe full and the queue
empty, where loss-based algorithms aim for the queue full and something dropped. The two
cannot be measured simultaneously, so BBR alternates.

BBR's advantage and its controversy *(§38.3)* — Largely unaffected by random loss,
because loss is not the signal — dramatic on wireless and long international paths. And
BBRv1 was not fair to CUBIC; v2 and v3 respond to loss and ECN as supplementary signals.
An algorithm that measures the right thing but competes badly is not obviously better
than one that measures the wrong thing and competes fairly.

**ECN** *(§38.3)* — The router sets CE instead of dropping; the receiver echoes ECE; the
sender reduces as if it had seen loss and confirms with CWR. Congestion signalled with
nothing lost, and one RTT earlier than loss would have arrived.

**ECN's twenty-year deployment** *(§38.3)* — Middleboxes cleared the bits or dropped
marked packets entirely, so enabling it made some destinations unreachable. Broken by
RFC 8311's silent fallback plus operators enabling it where they controlled both ends.
Twenty years, for a two-bit field that already existed — Chapter 21 §21.4's ossification,
measured precisely.

Data centres invert the assumptions *(§38.3)* — RTT in **microseconds**, near-zero
loss, and one operator controlling both ends — so the middlebox problem that delayed
ECN does not exist. **DCTCP** responds proportionally to the fraction of packets marked,
keeping queues very short, because a data centre's problem is latency rather than
throughput.

**Choose deliberately** *(§38.3)* — CUBIC for general purpose and for competing with
unknown traffic; BBR for long lossy paths and content at scale; DCTCP where you own
everything. Do not change it because a blog post recommended it — BBR is better for some
traffic on some paths and is not universally better.

Why TCP could not be changed *(§38.4)* — **(1) Ossification**: middleboxes strip unknown
options, drop packets carrying them, or rewrite sequence numbers — and they fail
silently, so a feature is subtly broken for users you cannot identify. (2) The
kernel: changing TCP means shipping an OS update to every client on Earth. (3)
Head-of-line blocking is architectural: HTTP/2 knows its streams are independent and TCP
cannot be told.

HTTP/2 over TCP was worse than HTTP/1.1 on lossy paths *(§38.4)* — Because six
connections meant a loss stalled one sixth of the work; one connection meant it stalled all
of it.

**QUIC's four choices** *(§38.4)* — **Over UDP** (middleboxes pass it), **in user space**
(ships with the application), streams in the transport (independent loss recovery), and
**an encrypted header** — so middleboxes **cannot** inspect, modify, or ossify it. QUIC is
designed to be un-ossifiable.

**QUIC's capabilities** *(§38.4)* — **Independent streams**, the thing TCP structurally
cannot do. 1-RTT handshake, or 0-RTT resumed — but 0-RTT data is **replayable**, so it
is safe only for idempotent requests. **Connection migration**, because a connection is
identified by a Connection ID rather than the five-tuple, so a phone moving from Wi-Fi
to cellular keeps its download. **Mandatory encryption**, which is both a security decision
and an ossification defence.

QPACK exists because HPACK could not work *(§38.4)* — HTTP/2's header compression
assumes ordered arrival, which independent streams do not guarantee. A change at one layer
forcing changes above it.

QUIC deployed in five years; IPv6 took thirty *(§38.4)* — Because the deploying party
benefits immediately and needs nobody's cooperation. QUIC is the counter-example that
proves Chapter 28's point about network effects.

**QUIC's costs** *(§38.4)* — 2–3× the CPU per byte historically, no mature hardware
offload, **operational opacity** — you cannot inspect it on the wire, which is deliberate
and is a real loss for operators — and debugging requires endpoint logging (qlog) rather
than packet capture.

Blocking UDP/443 blocks HTTP/3 silently *(§38.4)* — The TCP fallback is automatic and
works, so the network makes its users' browsing slower and nobody notices.

The final form of the layering argument *(§38.4)* — Layering is a design discipline,
not a law. It is violated where the cost of honouring it exceeds the benefit — and QUIC is
the largest deliberate violation in modern networking, because the boundary had come to
cost more than it was worth.
