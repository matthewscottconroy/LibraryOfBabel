# Chapter 38 — The People

**Van Jacobson (b. 1950) and Michael Karels.** The 1986 diagnosis and the 1988 fix. This
is the most consequential piece of work in this book.

What makes it remarkable is not the mechanisms but the diagnosis. The network was
failing in a way nobody had a name for, the failure was not in any component, and the
explanation — that correct local behaviour composed into a globally stable failure — had to
be arrived at from measurement.

**The paper's method is the lesson:** Jacobson instrumented the network, plotted what was
actually happening, and derived the mechanisms from what the data showed. Chapter 34's
`traceroute` and Chapter 37's RTT estimator come from the same habit — build the
instrument first.

**Conservation of packets** is the idea that makes the whole thing coherent. Everything
else — slow start, congestion avoidance, the estimator — is machinery for reaching and
maintaining an equilibrium that the ACK clock then sustains by itself.

Jacobson appears in Chapters 21, 24, 34, 37 and 38 of this book, which is more than anyone
else, and he later co-created **CoDel** (with Kathleen Nichols) and **BBR**.

**Dah-Ming Chiu and Raj Jain.** *Analysis of the Increase and Decrease Algorithms for
Congestion Avoidance in Computer Networks* (1989) — the proof that AIMD converges.

Their contribution is that TCP's behaviour is not merely empirically reasonable but
provably correct for the property it claims. The four combinations of
additive/multiplicative increase and decrease were analysed, and only AIMD converges to
both efficiency and fairness.

**Raj Jain** also gave us Jain's fairness index, which is how you measure whether an
allocation is fair rather than arguing about it.

**Sally Floyd (1950–2019).** Her second appearance, and she deserves more space than this
book gives her.

**RED** (Random Early Detection, with Jacobson, 1993) — drop packets *before* the queue is
full, probabilistically, so senders receive the congestion signal earlier and do not all
back off simultaneously. The direct ancestor of every active queue management scheme
since.

**ECN** (with Ramakrishnan) — §38.3's mechanism. **NewReno.** **SACK.** **TFRC** for
equation-based rate control. And a great deal of the analytical foundation on which
congestion control research rests.

Floyd's characteristic contribution was rigour: she took mechanisms that worked in
practice and established *why*, and where they failed. The transport layer behaves as
well as it does under stress substantially because of her.

**K. K. Ramakrishnan and Sally Floyd.** **ECN**, RFC 3168 (2001). The idea is obvious in
retrospect — ask the network rather than inferring from damage — and its twenty-year
deployment delay is one of the clearest measurements of ossification's cost in this book.

Matt Mathis, Jeffrey Semke, Jamshid Mahdavi and Teunis Ott. *The Macroscopic Behavior
of the TCP Congestion Avoidance Algorithm* (1997) — **the Mathis equation.**

Its value is that it makes an argument quantitative. "Loss hurts throughput" is a
statement anyone can make; "0.01% loss caps a 100 ms flow at 14 Mb/s regardless of link
capacity" ends a discussion.

It is the single most useful formula in this book for operational work, and Chapter 3
§3.3, Chapter 15 §15.4, Chapter 24 §24.3 and Chapter 66 all rest on it.

**Sangtae Ha, Injong Rhee and Lisong Xu.** **CUBIC** (2008), from North Carolina State
University.

The insight — make growth a function of elapsed time rather than round trips — removed
TCP's structural bias against long paths, and it did so with a modification small enough
to deploy. It is now the default on the majority of the world's machines, which is an
unusual outcome for an academic congestion-control proposal.

Neal Cardwell, Yuchung Cheng, C. Stephen Gunn, Soheil Hassas Yeganeh and Van Jacobson.
**BBR** (2016), at Google.

**The reframing is the contribution:** for thirty years congestion control asked *"has
something been dropped?"*; BBR asks *"what is this path's actual bandwidth and delay?"*

**And the motivation was measured, not theoretical.** At Google's scale it was demonstrable
that loss-based algorithms were operating at the point of maximum queueing delay, and that
on lossy international paths they were achieving a small fraction of available capacity.

The fairness criticism is legitimate and was taken seriously — BBRv2 and v3 exist
because the research community measured v1's behaviour against CUBIC and published the
results. This is the process working, and it is worth noting that the criticism came
from outside Google and was acted on.

**Yuchung Cheng** appears here and in Chapter 37 for RACK and TLP. The Google TCP team's
method is consistent: measure what actually limits performance at scale, then design for
that specifically.

**Kathleen Nichols and Van Jacobson.** **CoDel** (2012) — Controlled Delay, the queue
management algorithm that addresses bufferbloat by measuring how long packets sit in the
queue rather than how many there are.

The insight is that queue *length* is the wrong metric — a long queue that drains
quickly is fine, and a short queue that persists is not. CoDel targets the persistent
delay, and `fq_codel` (which adds fair queueing) is now the default on most Linux systems
and in OpenWrt.

**Jim Gettys (b. 1953).** **Bufferbloat** — his fourth appearance. The observation that
memory had become cheap enough that everyone added buffers, that no layer could see the
total, and that the result was multi-second queues that destroyed interactivity while every
component behaved reasonably.

**Naming the problem was the contribution.** Once "bufferbloat" existed as a word, the
measurements, the algorithms and the deployments followed within a few years.

**Jana Iyengar and Martin Thomson.** **QUIC** — RFC 9000, editors of the core
specification, after Jim Roskind's original design at Google in 2012.

**The design's coherence is the achievement.** QUIC is not a collection of improvements; it
is a set of decisions that reinforce each other — user space enables rapid iteration,
encryption prevents ossification, stream independence fixes head-of-line blocking, and the
combined handshake follows from doing transport and crypto together.

**Jim Roskind** deserves specific credit: the original Google QUIC was deployed and
measured at scale for **years** before standardisation, so the IETF work began with
operational evidence rather than argument.

Michio Honda, Yoshifumi Nishida, Costin Raiciu, Adam Greenhalgh, Mark Handley and Hideyuki
Tokuda. *Is it Still Possible to Extend TCP?* (IMC 2011).

**The measurement study that justified QUIC's existence.** They attempted to deploy new TCP
options across the real Internet and documented precisely how and where middleboxes broke
them.

> Without this paper, "TCP cannot be extended" is an assertion. With it, it is a
> measurement.

**Mark Handley** also co-authored Multipath TCP, which is the other answer to the same
question — extend TCP within what middleboxes tolerate — and the contrast between MPTCP's
constrained deployment and QUIC's rapid one is instructive.
