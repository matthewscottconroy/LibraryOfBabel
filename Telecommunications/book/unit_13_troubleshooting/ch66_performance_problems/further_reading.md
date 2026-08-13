# Chapter 66 — Further Reading

## Read these first

**Gettys, J. & Nichols, K. (2011). "Bufferbloat: Dark Buffers in the Internet." *ACM Queue*.**
**The naming paper**, and Gettys's account of finding it on his own connection is the best
introduction there is.

**Nichols, K. & Jacobson, V. (2012). "Controlling Queue Delay." *ACM Queue*.**
**CoDel, and it is genuinely well written.** **The argument that queue length is the wrong thing
to measure is the chapter's most transferable idea.**

**Mathis, M., Semke, J., Mahdavi, J. & Ott, T. (1997). "The Macroscopic Behavior of the TCP
Congestion Avoidance Algorithm."** *ACM SIGCOMM CCR*.
**Short.** **The derivation is followable and the consequence — square-root loss, inverse RTT —
is the argument that ends most "buy more bandwidth" discussions.**

**`bufferbloat.net`** and the **Bufferbloat Project's** material.
**The community's own documentation**, including test methodology, results and the history of
the fixes.

## Performance analysis

**Grigorik, I. — *High Performance Browser Networking*.**
**Free online.** **The chapters on latency, on TCP and on the application's view are the best
available treatment of §66.1's argument from the application side**, and the "latency is the new
bandwidth" framing is his.

**Gregg, B. — *Systems Performance* (2nd ed.), and the USE method.**
**Not primarily about networks and the method transfers directly**: **for every resource, check
Utilisation, Saturation and Errors.** **§66.2's counters are exactly that triple**, and Gregg's
methodology chapters are worth reading for the discipline.

**Fall, K. & Stevens, W. R. — *TCP/IP Illustrated, Volume 1*** (Chapter 65's reading) — **the
congestion control and timer chapters.**

**Ford, A. et al. and the wider `iccrg` material** — for where congestion control is going.

## MTU and PMTUD

**RFC 1191 (PMTUD), RFC 8201 (PMTUD for IPv6), RFC 4821 (Packetization Layer PMTUD).**
**RFC 4821's problem statement is the clearest description of the black hole in print**, and it
is Mathis's.

**RFC 2923 — "TCP Problems with Path MTU Discovery."**
**A catalogue of exactly the failures in §66.3**, written in 2000, **and still current** — which
is itself informative.

**RFC 8899** — datagram PLPMTUD, **for the UDP-based protocols that MSS clamping cannot help.**
**Relevant and increasingly so** (Chapter 38 §38.4).

**Cloudflare's and Google's engineering blog posts on MTU in production** — **practical accounts
of the problem at scale**, and both have written about it more than once.

## Queue management

**RFC 8289 (CoDel), RFC 8290 (FQ-CoDel), RFC 8033 (PIE), RFC 9332 (DualPI2).**
**FQ-CoDel's design rationale sections are the ones to read.**

**Høiland-Jørgensen, T. et al. (2018). "Piece of CAKE: A Comprehensive Queue Management Solution
for Home Gateways."**
**F8 uses it.** **The overhead accounting discussion is the practical part** and explains why a
shaper set from the marketing rate does not work.

**Floyd, S. & Jacobson, V. (1993). "Random Early Detection Gateways."**
**CoDel's ancestor** (Chapter 52's reading), **and reading it alongside CoDel shows exactly what
"no parameters" bought.**

**Appenzeller, G., Keslassy, I. & McKeown, N. (2004). "Sizing Router Buffers."**
Chapter 52's reading — **the result that buffers were over-provisioned by an order of
magnitude**, from the core router's side rather than the edge's.

**Cardwell, N. et al. — the BBR papers and the ongoing BBRv3 work.**
**Read the fairness discussion**, which is the honest and contested part.

## Duplex and the physical layer

**Seifert, R. & Edwards, J. — *The All-New Switch Book*** (Chapter 65's reading) — **the
auto-negotiation chapter.**

**IEEE 802.3 clause 28** — auto-negotiation. **Consult**, and the vendor summaries are more
readable.

**Your platform's counter documentation**, again. **"Input errors" is a different figure on
different platforms and the documentation is the only authority.**

## Tools

**`flent`** (flent.org) — **the tool for §66.4's measurement.** **Its `rrul` test loads the link
in both directions while measuring latency**, which is the whole diagnosis automated, and it
produces plots you can put in a report. **F1 and F2 use it.**

**The Waveform bufferbloat test** and **`dslreports`' speed test** — **browser-based, take
thirty seconds, and grade the result.** **The right tool for demonstrating the problem to
someone who does not run command-line tools.**

**`tc` with `netem` and `cake`** — **F5 uses `netem` to introduce known loss and verify the
Mathis relationship experimentally**, which is a satisfying afternoon.

**`iperf3`** (Chapter 64's reading) — **and read §64.4's warnings about single-stream
measurement before quoting a result.**

**`curl -w`** with a format file — **§66.1's decomposition.** **Write the format file once and
keep it**; it is the fastest first measurement for any web complaint.

**`ethtool -S`** — **the driver's own statistics**, which are far more detailed than the generic
counters and frequently name the exact drop reason.

**`ss -ti`** — **per-connection TCP state: cwnd, RTT, retransmissions, the congestion control
algorithm in use.** **The most under-used performance tool on Linux**, and it shows whether a
slow transfer is window-limited, loss-limited or application-limited in one line.

**SmokePing** — **continuous latency and loss to a set of targets, plotted.** **Chapter 54
§54.1's most under-collected baseline**, and it takes twenty minutes to set up.

## Following the field

**The `bloat` mailing list**, and the IETF's **`aqm`**, **`tsvwg`** and **`iccrg`** working
groups.

**Toke Høiland-Jørgensen's, Dave Täht's and Jim Gettys's writing** — **the current practitioner
commentary on queue management**, and consistently more useful than vendor material.

**The Broadband Forum's and cable industry's work on low-latency DOCSIS (L4S)** — **the current
attempt to solve this in the access network itself**, and it is where the next change will come
from.

## Where to look next

**Chapter 52** is this chapter's design counterpart — the same mechanisms, chosen deliberately
rather than diagnosed; **Chapter 3** is where the four delay components were established;
**Chapter 38** is the congestion control this chapter's arithmetic describes; **Chapter 54
§54.1** is how to see these faults before a user reports them; and **Chapter 72** returns to
performance as a design constraint rather than as a fault.
