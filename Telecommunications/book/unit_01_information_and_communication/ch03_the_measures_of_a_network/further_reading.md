# Chapter 3 — Further Reading

## Primary sources

**Cheshire, S. (1996). "It's the Latency, Stupid."**
Two pages. Read it today. It is the origin of the framing this whole chapter
uses, it is funny, and it has aged so well that the only dated thing about it is
the modem. Available at stuartcheshire.org.

**Jacobson, V., Braden, R. & Borman, D. (1992). RFC 1323, *TCP Extensions for High
Performance*. Superseded by Borman, Braden, Jacobson & Scheffenegger (2014),
RFC 7323.**
Window scaling and timestamps. §1 states the long-fat-network problem more
crisply than any textbook, and contains the "elephan(t)" joke. Read §1 and §2 of
RFC 7323; the rest is implementation detail.

**Mathis, M., Semke, J., Mahdavi, J. & Ott, T. (1997). "The Macroscopic Behavior of
the TCP Congestion Avoidance Algorithm." *ACM SIGCOMM Computer Communication
Review* 27(3): 67–82.**
The inverse-square-root loss relationship derived. The derivation is accessible
after Chapter 38; the result is usable now.

**ITU-T Recommendation G.114, *One-way transmission time* (2003).**
The 150/400 ms delay budget for interactive speech. Short, and the source of the
numbers every VoIP design document quotes second-hand.

**Schulzrinne, H., Casner, S., Frederick, R. & Jacobson, V. (2003). RFC 3550,
*RTP: A Transport Protocol for Real-Time Applications*, §6.4.1.**
The formal definition of interarrival jitter and the smoothing filter used to
compute it. Worth seeing that "jitter" has a precise definition and not merely a
vibe.

**Nagle, J. (1984). RFC 896, *Congestion Control in IP/TCP Internetworks*.**
Three pages, written two years before the collapse it predicts. A good example of
someone seeing a systems failure coming and being unable to get anyone to act.

## Books

**Grigorik, I. (2013). *High Performance Browser Networking.* O'Reilly.**
Free online at hpbn.co. Chapter 1 is the best treatment anywhere of latency versus
bandwidth for people who need to make decisions rather than pass exams, with
excellent real measurements. Chapters 2–4 cover TCP, UDP and TLS at exactly the
level this book's Unit VIII will reach. If you read one supplementary book
alongside this one, make it this.

**Bertsekas, D. & Gallager, R. (1992). *Data Networks*, 2nd ed. Prentice Hall.**
Chapter 3 is the classic rigorous treatment of queueing in networks — M/M/1,
M/G/1, networks of queues, Little's theorem. Mathematically demanding and worth
it if you want the ρ/(1−ρ) result derived rather than asserted.

**Kleinrock, L. (1975/1976). *Queueing Systems*, Volumes 1 and 2. Wiley.**
The standard reference. Volume 2 applies the theory to computer networks
specifically. Not a first read, but the place the results come from.

**Stevens, W. R. (1994). *TCP/IP Illustrated, Volume 1: The Protocols.*
Addison-Wesley. (2nd ed. with Fall, 2011.)**
The book that taught a generation to read packet captures. Its method — state a
mechanism, then show the actual trace — is the method this book uses in Unit VIII.
The window-scaling and performance chapters are directly relevant here.

## Tools and practice

**`iperf3`** — the standard throughput measurement tool. Learn `-P` (parallel
streams), `-u` (UDP, for jitter and loss), `-R` (reverse direction), and `-t`
(duration). Most wrong conclusions about throughput come from running the default
10-second single-stream TCP test and treating it as the path's capacity.

**`mtr`** — combines ping and traceroute continuously, giving per-hop loss and
latency over time. The right first tool for an intermittent WAN complaint, and the
right tool for demonstrating §3.3's point about non-propagating loss.

**`ss -i`** (Linux) — shows per-socket congestion window, RTT estimate, and
negotiated window scale factor. The direct way to confirm or eliminate a
bandwidth–delay-product problem on a live connection.

**`bdp_calculator.py`** in this book's [tools/](../../../tools/) directory —
computes BDP, required window, and achievable throughput from capacity, RTT and
loss, and plots the Mathis curve.

## For the certification-minded

N10-009 covers these under objective 5.5 (troubleshooting general networking
issues) and in the performance-based questions, which favour scenarios where the
naive answer is "add bandwidth." Objective 3.1 (monitoring) expects baselines, and
§3.3's point about reading all five numbers in a `ping` result — not just the
average — is directly applicable. Latency, jitter and packet loss are named
explicitly in the objectives as performance metrics; the exam does not require the
mathematics here, but the mathematics is what makes the scenarios obvious rather
than memorised.
