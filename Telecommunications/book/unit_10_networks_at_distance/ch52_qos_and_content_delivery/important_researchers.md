# Chapter 52 — The People

**Van Jacobson (b. 1950), again.** **He appears in Chapter 38 for congestion control and here
for the diagnosis of what went wrong afterwards.**

**Jacobson's argument from about 2010 — the bufferbloat campaign — is that the network filled
with memory.**

**Memory became cheap. Equipment vendors added buffers because a deeper buffer drops fewer
packets, and dropping packets looks like a defect.** **The consequence is that loss-based
congestion control, which needs loss to work, now fills a buffer of hundreds of milliseconds
before it receives any signal at all.**

> **Bufferbloat is the network being broken by an attempt to make it better.** Every individual
> decision to add memory was locally reasonable; **the aggregate destroyed interactive
> performance across the Internet**, and it took a decade to name.

**With Kathleen Nichols, Jacobson produced CoDel (2012)**, and its design insight is the one
worth carrying:

> **Measure how long packets are staying, not how many there are.** Queue length depends on
> packet size and link rate; **queueing delay is what actually harms the application**, and it
> is what CoDel controls.

**Nichols and Jacobson also deserve credit for what CoDel does not need:** **no configuration,
no classification, no knowledge of the link rate, and no tuning.** **A parameterless algorithm
that works across five orders of magnitude of link speed** is a rare thing, and it was the
explicit design goal.

**Kathleen Nichols.** **Co-author of CoDel, and — earlier — a principal author of the DiffServ
architecture.**

**Which is a genuinely unusual career shape.** **Nichols helped design the classification-based
QoS model of §52.2 in the late 1990s** (RFC 2474, with Steven Blake, Fred Baker and David Black), **and then fifteen years later co-designed the algorithm that makes much of it
unnecessary on edge links.**

> **That is not a contradiction.** DiffServ solves the problem of allocating a scarce link
> among classes with different requirements; **CoDel solves the problem of a buffer that is
> too deep.** They address different failures, and **the reason CoDel often replaces a QoS
> policy in practice is that most edge "QoS problems" were bufferbloat all along.**

**Dave Täht.** **The bufferbloat project's organiser, and the reason the fixes
shipped.**

**Täht's contribution was not an algorithm.** It was **CeroWrt, then the sustained, unglamorous
work of getting FQ-CoDel and CAKE into Linux, into OpenWrt, into home routers and eventually
into vendor firmware.** **He wrote much of the code, ran the measurement infrastructure,
maintained the mailing lists, and argued with manufacturers for a decade.**

> **An excellent algorithm that ships in nothing changes nothing.** **The bufferbloat fixes
> exist in your operating system's default queue discipline because someone spent ten years on
> the deployment**, and that work is systematically undercredited relative to the papers.

**Sally Floyd (1950–2019).** **RED — Random Early Detection, 1993, with Van Jacobson — and much
of the theory underneath everything in this chapter.**

**RED's idea was CoDel's ancestor:** **do not wait until the queue is full. Drop probabilistically
as it grows**, so TCP receives a congestion signal early and the queue never reaches maximum
depth.

**It was correct and it was very hard to configure.** **RED's parameters — minimum threshold,
maximum threshold, drop probability — had to be tuned per link, and getting them wrong made
things worse.** **The result was that RED shipped everywhere and was enabled almost nowhere**,
which is exactly the failure CoDel's parameterless design was built to avoid.

**Floyd's wider contribution is larger than RED.** **ECN** (Chapter 38 §38.3), **TCP's SACK
behaviour, the analysis of TCP-friendly rate control, and a body of measurement work that
established how congestion control actually behaves in the wild** rather than in simulation.
**She is among the most cited researchers in networking**, and the citation count understates
her influence because much of it is in RFCs rather than papers.

**Guido Appenzeller, Isaac Keslassy and Nick McKeown, and the buffer sizing result.**

**The 2004 paper "Sizing Router Buffers" showed that the accepted rule — buffer equals
bandwidth times delay — overprovisions core routers by an order of magnitude.**

**The argument is a statistical one.** **The rule assumes one TCP flow, whose sawtooth needs a
full bandwidth-delay product of buffer to keep the link busy through a recovery.** **With $n$
independent flows, the sawtooths are not synchronised**, and their sum is far smoother —
**so $\mathrm{RTT} \times C / \sqrt{n}$ suffices.**

> **This was a commercially significant result.** Core router buffers were expensive fast
> memory, **and reducing them tenfold changed what could be built.** It is also a reminder that
> **a rule of thumb derived for one flow may be badly wrong for many**, which is a general
> hazard.

**Daniel Lewin (1970–2001) and Tom Leighton (b. 1956).** **Akamai, and the invention of the
CDN.**

**The origin is an MIT class.** **Tim Berners-Lee, then at MIT, posed the problem of Internet
congestion as the web grew.** **Leighton, an applied mathematician, and Lewin, his graduate
student, worked on distributed hashing and consistent hashing as an approach** — Lewin's
master's thesis contains the consistent hashing algorithm that is now a standard tool far
beyond CDNs.

**Akamai was founded in 1998 and by 1999 was serving a substantial fraction of the web's
images.**

**Lewin was killed on 11 September 2001**, aboard American Airlines Flight 11 — **he is
generally believed to have been the first victim of the attacks.** He was 31.

> **Consistent hashing is his lasting technical legacy and it is used far more widely than in
> content delivery** — **in distributed caches, in databases, in load balancers, in nearly every
> system that must map keys to a changing set of servers.** **Chapter 69's cloud systems are
> full of it.**

## A note on the shape of this chapter's history

**Two of this chapter's most important contributions were deletions or simplifications, not
additions.**

**CoDel removed the parameters that made RED unusable.** **Appenzeller's result removed
buffering that was thought necessary.** **Neither added a capability**, and both had larger
effects than most additions.

**And one contribution was neither research nor engineering.** **Täht's decade of deployment
work is why any of it reached a household**, and the discipline systematically undervalues that
kind of effort relative to the papers it makes real.

> **The gap between "solved in a paper" and "fixed in the world" is where most of this
> chapter's actual improvement happened**, and it is measured in years.
