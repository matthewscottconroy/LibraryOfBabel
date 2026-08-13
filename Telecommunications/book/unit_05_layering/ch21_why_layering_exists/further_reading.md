# Chapter 21 — Further Reading

## Primary sources

**Dijkstra, E. W. (1968). "The Structure of the 'THE' Multiprogramming System."
*Communications of the ACM*, 11(5).**
Where layering as a design discipline is first argued rigorously. Six pages, and the
argument — that strict levels are what make a system's correctness provable rather than
plausible — applies unchanged to networks.

**Zimmermann, H. (1980). "OSI Reference Model — The ISO Model of Architecture for Open
Systems Interconnection." *IEEE Transactions on Communications*, 28(4).**
The service/protocol distinction and the four service primitives of §21.2, stated
cleanly. The protocols did not survive; **this vocabulary did**.

**Saltzer, J., Reed, D. & Clark, D. (1984). "End-to-End Arguments in System Design."
*ACM TOCS*, 2(4).**
**Read this.** The most important paper in network architecture, and it answers the
question §21.1 leaves open: not whether to layer, but where a function belongs.
Chapter 23 §23.4 develops it; read the original first.

**Clark, D. (1988). "The Design Philosophy of the DARPA Internet Protocols."
*ACM SIGCOMM*.**
What the designers were actually optimising for, in priority order, written by one of
them. The ranking explains a great deal that otherwise looks arbitrary.

**RFC 1631 — Egevang, K. & Francis, P. (1994). *The IP Network Address Translator.***
Worth reading for its tone: NAT presented as a temporary measure, its architectural
damage catalogued honestly, and recommended regardless. Thirty years on, the
temporary measure is universal.

**RFC 3234 — Carpenter, B. & Brim, S. (2002). *Middleboxes: Taxonomy and Issues.***
The systematic catalogue of §21.4. Reads as an obituary for the end-to-end Internet,
written by people who understood exactly what was being lost.

**RFC 1144 — Jacobson, V. (1990). *Compressing TCP/IP Headers for Low-Speed Serial
Links.***
A textbook performance-motivated layer violation, contained to one link, still in use
as ROHC on cellular. Short and clever.

## Books

**Day, J. (2008). *Patterns in Network Architecture: A Return to Fundamentals.*
Prentice Hall.**
The most sustained critique of how layering was actually done, from someone who worked
on OSI. His argument is that the field settled on the wrong decomposition and has been
paying for it since. **Contentious, sometimes cranky, and the most intellectually
serious book about layering that exists.** Read it after you are comfortable with the
conventional account, and disagree with it productively.

**Tanenbaum, A. & Wetherall, D. (2011). *Computer Networks*, 5th ed.**
Chapter 1's treatment of layering is the standard reference and is very good on service
primitives and on the OSI/TCP-IP comparison.

**Kurose, J. & Ross, K. *Computer Networking: A Top-Down Approach.***
The opposite pedagogical order to this book's, argued explicitly in its preface. Worth
reading for the disagreement — the two orders make different things easy, and seeing
both is more useful than committing to either.

**Peterson, L. & Davie, B. *Computer Networks: A Systems Approach.***
Chapter 1 is unusually good on the costs of abstraction and on why performance-driven
violations exist. Freely available online in recent editions.

## On the costs

**Appenzeller, G., Keslassy, I. & McKeown, N. (2004). "Sizing Router Buffers."**
and **Gettys, J. & Nichols, K. (2011). "Bufferbloat: Dark Buffers in the Internet."
*ACM Queue*.**
Bufferbloat as a pure information-barrier failure: buffers added independently at every
layer, each locally reasonable, and no layer able to see the total.

**Balakrishnan, H. et al. (1997). "A Comparison of Mechanisms for Improving TCP
Performance over Wireless Links." *ACM SIGCOMM*.**
The definitive treatment of §21.3's TCP-over-wireless problem, comparing every
mitigation. Still the clearest statement of what the layer boundary costs.

**Langley, A. et al. (2017). "The QUIC Transport Protocol: Design and Internet-Scale
Deployment." *ACM SIGCOMM*.**
The deliberate layer violation, with the reasoning and with production measurements
from Google's deployment. Section 2 states the ossification argument of §21.4 in the
authors' own words.

**Honda, M. et al. (2011). "Is it Still Possible to Extend TCP?" *ACM IMC*.**
The measurement study behind the ossification claim: they tried to deploy new TCP
options across the real Internet and documented exactly where and how middleboxes broke
them. **The empirical foundation for QUIC's existence.**

## Applied

**`ethtool -k eth0`**, to see which offloads are enabled, and `ethtool -K eth0 tso off
gso off gro off lro off` to disable them before a capture that must reflect the wire.
Do this once and compare the captures; §21.3's claim becomes concrete immediately.

**A VoIP capture**, measured against the codec's nominal rate. Exercise F2. The gap
between 8 kb/s and 39 kb/s is the clearest demonstration of header overhead available.

**`strace` on a simple network client**, to watch the eight socket calls of §21.2 in
sequence. Forty years of interface stability, visible in one command.

**Lab 09** in this book's [labs/](../../../labs/) directory measures header overhead
empirically across payload sizes, then demonstrates offload effects on captures and the
TCP-over-lossy-link collapse using `tc netem`.

**`tc qdisc add dev eth0 root netem loss 1%`** — introduce loss deliberately and watch
TCP throughput. Then try it as delay instead. The asymmetry between the two is §21.3's
argument, measurable in five minutes.

## For the certification-minded

Network+ does not examine this chapter directly. It underpins several things that are
examined:

1. **VoIP bandwidth calculations must include header overhead**, and the exam does ask
   for these.
2. **Wireless throughput is well below its nominal rate**, partly for the reason in
   §21.3.
3. **NAT operates on Layer 4 ports as well as Layer 3 addresses** — examined, and
   frequently misremembered.
4. **Offloads make host captures differ from the wire**, which appears in
   troubleshooting scenarios.

And the general point, worth more than any of them: **you can derive what belongs at a
layer instead of memorising it.** A layer exists to serve many things above with one
implementation below. Any function that does not do that is misplaced, and any question
about where something belongs can be answered from that sentence.
