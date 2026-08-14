# Chapter 65 — Further Reading

## Read these first

Fall, K. & Stevens, W. R. — *TCP/IP Illustrated, Volume 1* (2nd ed., 2011).
The book that teaches what correct behaviour looks like, which is the prerequisite for
recognising incorrect behaviour. Its method — show the capture, then explain it — is this
chapter's method.

Seifert, R. & Edwards, J. — *The All-New Switch Book* (2nd ed.).
The reference for §65.2. The chapters on auto-negotiation and on spanning tree operation
explain why the faults happen rather than merely listing them, and Seifert is unusually good on
the standards' compromises.

**Your platform's interface counter documentation.**
**Genuinely.** Every vendor documents what each counter means and they differ — "input
errors" is not the same figure on two platforms — and reading it once prevents a recurring
category of misdiagnosis.

## Layer 1

BICSI's Telecommunications Distribution Methods Manual, and **ANSI/TIA-568** and **-569**.
Consult rather than read. The distance limits, the pinouts and the installation practice
of §65.1 are specified here, and when a cable "works" but is out of specification this is what
says so.

Fluke Networks' and Viavi's application notes — on certification results, NEXT, return
loss and OTDR trace interpretation. Vendor material, and the best available teaching on
reading a certifier's output.

The IEEE 802.3 clauses for your media type, and — more practically — **transceiver data
sheets.** The Rx sensitivity figure that turns `show interface transceiver` into a diagnosis
is in the data sheet and nowhere else.

**IEC 61300-3-35** — the fibre connector cleanliness standard, and the reason inspection
scopes have pass/fail overlays. A summary is sufficient and the images are the useful part.

Cisco's, Juniper's and Arista's optical troubleshooting guides — all publish the
"read the DOM values first" procedure, and comparing three shows what is universal.

## Layer 2

IEEE 802.1D and 802.1w, and **802.1Q.**
**Consult.** The state machine and timer material explains §65.2's convergence symptoms, and
the vendor implementation guides are more readable.

**Perlman, R. — *Interconnections* (2nd ed.).**
Chapter 19's reading, and it belongs here for the design rationale behind the faults.

Vendor spanning tree troubleshooting guides — Cisco's in particular is thorough on
topology change causes, which is §65.2's most useful diagnostic.

The `bridge` and `mstpctl` manual pages on Linux, for the same mechanisms without a vendor
in the way.

## Layer 3 and above

Doyle, J. & Carroll, J. — *Routing TCP/IP*, Volumes 1 and 2.
The reference for §65.3's adjacency and convergence faults, and the troubleshooting sections
at the end of each protocol chapter are the parts to read.

**RFC 2328 (OSPF)** §10 and the neighbour state machine, and **RFC 4271 (BGP)** §8 — the state
machines whose names are the diagnoses. "Stuck in ExStart" is meaningful only if you know
what ExStart is for.

RFC 1122 and RFC 1812 — host and router requirements. Old, and they specify a great deal of
behaviour that is assumed rather than documented elsewhere, including much of what §65.3's
faults violate.

Chapter 39's and Chapter 58's reading, for the DNS and certificate material of §65.4.

## Practical catalogues

**Vendor knowledge bases**, used deliberately. Cisco's, Juniper's and Microsoft's are large,
searchable, and contain the accumulated record of the faults in this chapter — and searching
the exact error string is the right first move (Chapter 63 §63.4's argument, from the
consumer's side).

NANOG, RIPE and UKNOF presentation archives — operators describing faults they found,
and consistently more useful than anything written for a certification.

The `serverfault` and `networkengineering.stackexchange` archives, used with the usual
scepticism. The accepted answer is frequently wrong and the comments are frequently right.

Your own organisation's incident records — **F8 uses them.** The distribution of where
faults actually are, in your environment, is more useful than any general catalogue, and
nobody has ever computed it.

## Tools

Chapter 64's toolbox applies unchanged, and three additions specific to this chapter:

**`ethtool`** — negotiated speed and duplex, the advertised capabilities of both sides,
detailed error counters, and `ethtool -S` for the driver's own statistics, which are far more
detailed than the generic ones. The duplex investigation of §65.2 runs on it.

**`arping`** — Layer 2 reachability independent of Layer 3, which distinguishes an ARP
problem from a routing problem in one command.

**`ip monitor`** — watches routing, address and neighbour changes as they happen, which is
how you catch a flapping route or an address being reassigned.

**A loopback plug**, and a known-good patch lead kept deliberately — the two physical items
that convert an argument into a test.

## On the pattern

Petroski, H. — *To Engineer Is Human: The Role of Failure in Successful Design*.
**Not about networks.** The argument that failure is where engineering knowledge comes from,
and it reframes a fault catalogue as a knowledge base rather than a list of embarrassments.

**Chiles, J. — *Inviting Disaster*.**
Case studies of system failures across industries, and the recurring shapes — boundaries,
mismatches, compensating mechanisms that hide faults — are §65's closing observation, with
examples from outside computing.

## Where to look next

**Chapter 66** is the fourth category this chapter defers: performance complaints, which have
no single symptom and resist the layered method; **Chapter 64** supplies the tools; Chapter
63 §63.4 is where the faults you find should end up; and Chapter 54 §54.1 is how you
detect them before a user does.
