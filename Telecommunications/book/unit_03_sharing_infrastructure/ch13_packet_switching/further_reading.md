# Chapter 13 — Further Reading

## Primary sources

**Baran, P. (1964). *On Distributed Communications*, RAND Memoranda RM-3420-PR
through RM-3428-PR.**
Freely available from RAND. **Volume I, *Introduction to Distributed Communications
Networks*, is the one to read** — about forty pages, non-mathematical in its main
argument, and containing the three-topology diagram and the redundancy-level
survivability plots. Volume V covers routing and Volume IX covers security,
including the end-to-end encryption argument seventeen years before Saltzer, Reed
and Clark.

**Baran, P. (1964). "On Distributed Communications Networks." *IEEE Transactions on
Communications Systems* 12(1): 1–9.**
The condensed journal version, if forty pages is too many.

**Davies, D. W. (1966). "Proposal for a Digital Communication Network."**
NPL internal paper, and the origin of the word *packet*. Available through the NPL
archives and various history collections.

**Roberts, L. G. (1967). "Multiple Computer Networks and Intercomputer
Communication." *ACM Symposium on Operating System Principles*.**
The ARPANET design, presented at the meeting where Scantlebury presented NPL's work
and Roberts learned of Baran's.

**Leland, W. E., Taqqu, M. S., Willinger, W. & Wilson, D. V. (1994). "On the
Self-Similar Nature of Ethernet Traffic (Extended Version)." *IEEE/ACM Transactions
on Networking* 2(1): 1–15.**
The measurement paper that showed Poisson models understate queueing. §1 and §4 are
readable without the heavy statistics, and the plots at multiple timescales are the
whole argument.

**Nichols, K. & Jacobson, V. (2012). "Controlling Queue Delay." *ACM Queue* 10(5).**
CoDel. Clear, opinionated, and the best single explanation of why buffer sizing by
occupancy fails and by sojourn time works.

**Gettys, J. & Nichols, K. (2011). "Bufferbloat: Dark Buffers in the Internet."
*ACM Queue* 9(11).**
The paper that named the problem and made the industry take it seriously.

## Books

**Hafner, K. & Lyon, M. (1996). *Where Wizards Stay Up Late: The Origins of the
Internet.* Simon & Schuster.**
The standard narrative history of the ARPANET. Good on the people and on the
practical constraints; explicit that the nuclear-war motivation is a myth as applied
to the ARPANET itself.

**Abbate, J. (1999). *Inventing the Internet.* MIT Press.**
More scholarly than Hafner and Lyon, and better on the institutional and political
context — including a careful treatment of the Baran/Davies/Kleinrock priority
question that avoids taking sides.

**Bertsekas, D. & Gallager, R. (1992). *Data Networks*, 2nd ed. Prentice Hall.**
Chapter 3 for the queueing mathematics of §13.3, derived properly. Chapter 5 for
routing. Demanding and definitive.

**Kleinrock, L. (1976). *Queueing Systems, Volume 2: Computer Applications.***
Wiley. The application of queueing theory to networks, by the person who did it
first.

**Peterson, L. & Davie, B. (2021). *Computer Networks: A Systems Approach*,
6th ed.** Free online at systemsapproach.org. Chapter 3's treatment of switching is
excellent on the datagram/virtual-circuit distinction, and the book as a whole is
more architecturally minded than most textbooks.

## Historical and biographical

**Baran, P. (1999). Oral history interview, Charles Babbage Institute.**
Freely available. Baran in his own words on the AT&T rejection, on why he
recommended cancelling his own programme, and on his consistent deflection of credit.
More candid than any secondary account.

**Campbell-Kelly, M. (1987). "Data Communications at the National Physical
Laboratory (1965–1975)." *Annals of the History of Computing* 9(3): 221–247.**
The British side, which is usually compressed to a footnote. Good on why the NPL
network did not become a national one.

**Pelkey, J. (2007). *Entrepreneurial Capitalism and Innovation: A History of
Computer Communications 1968–1988.*** Free online. Exhaustive, based on hundreds of
interviews, and particularly good on the commercial forces — X.25, Telenet, the
carriers' responses — that most histories skip.

## Applied

**RFC 970, *On Packet Switches with Infinite Storage* (Nagle, 1985).**
Six pages, and it predicts bufferbloat twenty-five years before it was named. Nagle
observes that a switch with unlimited buffering does not avoid congestion; it merely
converts loss into unbounded delay. Worth reading as an example of a warning
correctly issued and comprehensively ignored.

**RFC 8290, *The FlowQueue-CoDel Packet Scheduler and Active Queue Management
Algorithm*.**
The deployable answer, and the default in Linux.

**bufferbloat.net** and the **Flent** testing tool.
Measuring latency under load on your own connection takes ten minutes and is the
most direct way to make §13.3's argument concrete. Most connections show the effect
clearly.

## Tools

**`simnet.py statmux`** in this book's [tools/](../../../tools/) directory — for
Exercises 13.9–13.11 and for watching the multiplexing gain grow with population
size.

**`simnet.py queue`** — the ρ/(1−ρ) table, which is the most useful output in
the toolset for capacity-planning conversations.

## For the certification-minded

Objective 1.6 expects the circuit-switched versus packet-switched distinction, and
objective 1.2 expects MPLS. Nothing else in this chapter is examined directly.

Three of its ideas are examined constantly under other headings:

1. **Oversubscription is deliberate**, and §13.4's arithmetic is why it is sound
   engineering rather than corner-cutting.
2. **Capacity planning targets peak, not average**, because of ρ/(1−ρ).
3. **Packet loss on a congested link is a signal rather than a fault**, which
   changes what you do about it.

And one that is not examined and is worth more than the three: **packet switching
deliberately abandoned admission control**, which is why QoS exists, why it cannot
create capacity, and why every guarantee mechanism in this book is harder than it
looks.
