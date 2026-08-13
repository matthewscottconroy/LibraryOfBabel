# Chapter 66 — The People

**Matt Mathis**, and the equation that ends most bandwidth arguments.

**"The Macroscopic Behavior of the TCP Congestion Avoidance Algorithm" (Mathis, Semke,
Mahdavi and Ott, 1997)** derived, from the AIMD sawtooth's geometry, the relationship §66.1
uses:

$$\text{throughput} \approx \frac{\mathrm{MSS} \times C}{\mathrm{RTT}\sqrt{p}}, \qquad C = \sqrt{3/2}$$

> **The derivation is not complicated and the consequence is severe.** **Throughput falls with
> the square root of the loss rate and inversely with RTT**, which means **a small amount of
> loss on a long path caps a transfer at a rate that has nothing to do with the link.**

**Mathis's later work is equally practical.** **He was central to the Web100 and pathdiag
projects**, which instrumented TCP's own state so that a slow transfer could be diagnosed
rather than guessed at, **and to RFC 4821's Packetization Layer PMTUD** — §66.3's black-hole
detection, which exists because he documented how often the ICMP never arrives.

**And he coined the framing that this chapter's §66.1 borrows:** **that a wide-area transfer's
performance is determined by the worst of several independent limits, and that identifying which
one binds is the entire diagnostic task.**

**Jim Gettys (b. 1953).** **The person who named bufferbloat, and who found it on his own home
connection.**

**Gettys is not primarily a network researcher** — **he worked on the X Window System, on the
HTTP/1.1 specification, and on the One Laptop Per Child project.** **He noticed in 2010 that
his home connection's latency rose to seconds under load**, and **rather than accepting it, he
measured it, characterised it, and discovered that the same behaviour was present in almost
every device he tested.**

> **The finding was that the problem was everywhere and nobody had a name for it.** **Home
> routers, cable modems, DSL modems, wireless drivers, operating system queues, and the
> carriers' own edge equipment.** **All of them had accumulated buffers over a decade of
> cheapening memory, and none of the individual decisions had been wrong.**

**Naming it was the substantive contribution.** **"Bufferbloat" gave the phenomenon an identity
that could be measured against, argued about, and fixed** — and **the fixes followed within two
years**, which for a problem of that scope is remarkably fast.

**Gettys has written that the most frustrating part was how long it took to convince people the
problem was real**, because **every individual measurement people made — throughput — showed
nothing wrong.** §66.4's four reasons it is missed are his.

**Kathleen Nichols and Van Jacobson**, again — **CoDel, 2012, and the design constraint that
made it deployable.**

**Chapter 52's entry covers the algorithm.** **What belongs here is the constraint:**

> **RED had existed since 1993, was correct, and was enabled almost nowhere** — **because its
> parameters had to be tuned per link and getting them wrong made things worse.** **CoDel's
> explicit design goal was to have no parameters at all.**

**And it achieved it.** **CoDel works across five orders of magnitude of link rate with the same
two constants — a 5 ms target and a 100 ms interval — and requires no knowledge of the link's
capacity.**

> **"It must work with no configuration" is a stronger requirement than "it must work well",
> and Nichols and Jacobson were right to prioritise it.** **An algorithm that ships enabled beats
> a better one that ships disabled**, which is Chapter 52's Täht argument from the other
> direction.

**Dave Täht**, again, and this is where his contribution is most visible.

**Chapter 52 covered the deployment work.** **Its result is that FQ-CoDel is the default queue
discipline on Linux, that CAKE exists and is in OpenWrt, and that a domestic router bought today
is far less likely to bloat than one bought in 2012.**

> **Most people benefit from this without knowing it exists**, which is the correct outcome and
> the reason the work was undervalued while it was being done.

**Toke Høiland-Jørgensen and the CAKE authors.**

**CAKE (2018)** takes FQ-CoDel and adds **the shaper, the overhead accounting, DiffServ
awareness and per-host fairness** — **and the overhead accounting is the part that matters
practically.**

> **A shaper set to "50 Mb/s" on a DOCSIS or PPPoE link is not shaping to 50 Mb/s**, because
> **the encapsulation is not counted** (Chapter 49). **CAKE's `docsis`, `pppoe-ptm` and
> `ethernet` keywords account for it**, which is the difference between a shaper that works and
> one that is 5% too high and therefore does nothing.

**Høiland-Jørgensen's `flent` is the other half** — **the measurement tool that makes the
before-and-after comparison of F1 and F2 possible**, and its `rrul` test loads the link in both
directions while measuring latency, which is exactly §66.4's diagnosis automated.

**Rich Seifert**, again, for the duplex mismatch (Chapter 65's entry).

**And the IEEE 802.3 committees**, collectively, for **auto-negotiation's "do not participate"
option** — **which is the mechanism by which §66.2's fault exists at all.**

> **A negotiation protocol in which one party may decline to negotiate will produce
> mismatches**, and **the alternative — mandatory negotiation — was considered and rejected for
> backward compatibility with equipment that predated it.** **Which is a defensible decision
> whose cost has been paid over thirty years by everyone who has diagnosed a duplex mismatch.**

## What this chapter's history shows

**Three observations.**

**The equations were derived before the problems were widespread.** **Mathis's relationship
predates the wide-area transfers it constrains; Chapter 3's bandwidth-delay product predates
the long fat networks it describes.** **The theory was available and the practice ignored it for
a decade**, which is this book's recurring shape.

**Bufferbloat was found by someone measuring their own connection.** **Not by a research
programme, not by a vendor, not by a standards body** — **by one person who refused to accept
that his home internet was just like that, and had the expertise to characterise what he
found.**

**And the fixes succeeded by removing configuration.** **CoDel has no parameters. FQ-CoDel needs
no classification. CAKE needs one number.** **Each is a deliberate rejection of the tunability
that made RED and hand-built QoS policies fail in practice**, and **it is the same argument as
Chapter 58's about cryptographic primitives designed so that misuse is difficult.**

> **The pattern across this book's operational chapters is consistent: the mechanism that ships
> enabled and needs no tuning wins, and the better mechanism that requires expertise does
> not.** **Designing for that is a skill, and it is under-taught.**
