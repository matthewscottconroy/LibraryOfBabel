# Chapter 14 — Further Reading

## Primary sources

**Clark, D. D. (1988). "The Design Philosophy of the DARPA Internet Protocols."
*ACM SIGCOMM Computer Communication Review* 18(4): 106–114.**
**The most useful document in this chapter's reading list.** Clark lists the
architecture's goals in priority order — survivability first, accountability last —
and explains what was traded for what. Eight pages, no mathematics, and it explains
more about why the Internet is bad at the things it is bad at than any other source.
Read it after Chapter 23 and again after Unit XII.

**Cerf, V. G. & Kahn, R. E. (1974). "A Protocol for Packet Network
Intercommunication." *IEEE Transactions on Communications* 22(5): 637–648.**
The internetworking paper. §I's problem statement — three networks, incompatible,
whose operators will not change — is the clearest possible motivation for the
black-box construction of §14.2.

**RFC 1958, *Architectural Principles of the Internet* (Carpenter, 1996).**
Eight pages. States the hourglass rule and the end-to-end principle as architectural
policy rather than as observation. §2's list of principles is worth reading
alongside Clark's paper.

**RFC 3439, *Some Internet Architectural Guidelines and Philosophy* (Bush &
Meyer, 2002).**
The sequel, and more sceptical. Its "Simplicity Principle" and the section on the
myth of end-to-end are a useful corrective to reading RFC 1958 as scripture.

**Leiner, B. et al. (1997). "A Brief History of the Internet."**
Available from the Internet Society. Written by the people who did it, and explicit
about the nuclear-war myth.

**ITU-T Recommendation G.114, *One-way transmission time*.**
Where §14.4's 150 ms budget comes from. Short, and worth reading to see that it is a
human-factors result rather than an engineering one.

## Books

**Peterson, L. & Davie, B. (2021). *Computer Networks: A Systems Approach*,
6th ed.** Free online at systemsapproach.org. Chapter 1's treatment of network
architecture and Chapter 4's internetworking material are the best textbook
complement to §14.2, and the book is unusually architecturally minded.

**Day, J. (2008). *Patterns in Network Architecture: A Return to Fundamentals.*
Prentice Hall.**
Difficult, opinionated, and worth the effort. Day argues that the Internet's layering
is historically contingent rather than principled, and that the recursive
construction of §14.2 should be applied uniformly rather than stopping where it does.
You need not agree; engaging with the argument sharpens what you think layering is
for.

**Abbate, J. (1999). *Inventing the Internet.* MIT Press.**
The institutional history, and good on how the internetworking decision was shaped by
the fact that ARPA could not compel the constituent networks to change.

**Blum, A. (2012). *Tubes: A Journey to the Center of the Internet.* Ecco.**
A journalist's account of visiting the physical Internet — exchange points, cable
landing stations, data centres. Useful precisely because §14.2's abstraction makes it
easy to forget that there is a physical thing, and Chapter 48 assumes you know there
is.

## On re-centralisation

**Zittrain, J. (2008). *The Future of the Internet — And How to Stop It.* Yale.**
Free online. The generativity argument: that the Internet's value came from
permitting unanticipated uses, and that the shift toward controlled platforms
forecloses that. §14.3's re-centralisation, considered as a policy question.

**Doctorow, C. (2023). *The Internet Con.* Verso.**
Polemical and technically informed on interoperability and lock-in. Read as an
argument rather than an analysis.

**Jacobson, V. et al. (2009). "Networking Named Content." *ACM CoNEXT*.**
The Content-Centric Networking paper. The most serious attempt to argue that the
hourglass's waist addresses the wrong thing — hosts rather than content — and that
CDNs are a workaround for that mismatch. Whether it is right is open; that a serious
attempt exists is worth knowing.

## On convergence

**Cisco, *Enterprise QoS Solution Reference Network Design Guide*.**
Free, long, and the most complete practical treatment of what §14.4's convergence
requires in configuration. The trust-boundary material is directly relevant to
exercise E1 of Chapter 14.

**Davidson, J., Peters, J. et al. (2006). *Voice over IP Fundamentals*, 2nd ed.
Cisco Press.**
Dated in its products and sound on the requirements: what voice needs, why, and what
a packet network must do to provide it. The chapters on delay budgets and jitter
buffers are the practical form of G.114.

**ETSI and 3GPP emergency-services requirements documents.**
Unglamorous, and the place to look for the regulatory obligations §14.4 mentions as
transferring to the enterprise. Location determination for a moved VoIP handset is a
genuinely unsolved problem in many jurisdictions.

## Tools

**Trace your own organisation's traffic.** Chapter 54 §54.4's flow data, aggregated
by destination, answers §14.3's design question — *where does the traffic actually
go* — and the answer is frequently not what the network was designed for. This is
the most useful exercise in the chapter.

**`perfcalc.py latency`** in this book's [tools/](../../../tools/) directory, for
exercise B1 of Chapter 14's round-trip arithmetic.

## For the certification-minded

Objective 1.6 expects the scope categories, client–server and peer-to-peer, and the
LAN/WAN distinction. Objective 1.2 expects VoIP infrastructure; objective 2.1
expects QoS.

Three things worth carrying beyond the exam:

1. **Latency is the durable LAN/WAN distinction**, measured in round trips.
2. **Best-effort is the only universally satisfiable contract**, which is why IP
   promises nothing and why that is a design choice rather than a shortcoming.
3. **Convergence exchanged physical separation for logical separation**, which is
   why VLANs, QoS and segmentation are not optional extras in a converged design —
   they are the thing that replaces what was removed.
