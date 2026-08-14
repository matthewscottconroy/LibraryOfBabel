# Chapter 25 — The People

**Jon Postel (1943–1998) and the RFC 791 authors.** The classful scheme of §25.4 is
their design, and it deserves defending before it is criticised.

In 1981 there was no field available for a mask and no appetite for adding one — the
header was already twenty bytes and every byte was contested. Encoding the split in the
leading bits cost **nothing**: no field, no signalling, no configuration, no negotiation.
For a network of a few hundred hosts with an address space of four billion, three sizes
seemed generous.

**The failure was not of design but of forecast.** They expected the address space to
last indefinitely because the growth they were planning for was linear, and what
happened was exponential. Nobody in 1981 predicted a personal computer on every desk,
let alone in every pocket.

**Jeffrey Mogul and Jon Postel.** **RFC 950** (1985), *Internet Standard
Subnetting Procedure* — the mask as an explicit object, and the first crack in the
classful edifice.

The document's importance is larger than what it did. It solved a local problem — letting
an organisation divide its own allocation — and in doing so it **introduced the concept
that would make classes redundant eight years later.** Once a mask exists as a thing you
can write down and carry in a protocol, the class is doing no work.

**Vince Fuller, Tony Li, Jessica Yu and Kannan Varadhan.** Authors of **RFC 1519**
(1993), *Classless Inter-Domain Routing*. Also RFC 1338 (1992), which is the earlier
statement of the same argument.

CIDR is the most consequential change ever made to IP addressing, and what makes it
remarkable is the deployment. **It was rolled out across the operating Internet in about
two years**, requiring routing protocol changes, allocation policy changes, and
coordinated behaviour from every service provider — with no flag day and no central
authority able to compel anyone.

**And it worked.** The global routing table's growth curve bent visibly in 1994, which is
one of very few occasions in this book where a protocol change produced an immediately
measurable global effect. The people who did it were operators and engineers acting
under genuine time pressure — the projections said routers would run out of memory before
1996 — and the work was done in working groups over about eighteen months.

**Tony Li** went on to substantial routing work at Cisco and Juniper, including much of
the BGP scaling work of Chapter 32. **Vince Fuller** later worked on LISP and on the
locator/identifier separation problem, which is the same problem CIDR addressed one
layer up.

**Yakov Rekhter.** Co-author of RFC 1518 (the CIDR address allocation
architecture) and of **BGP** itself (Chapter 32). His contribution here is the
**provider-based allocation** argument: that addresses should be allocated by providers
from their own blocks, so that customer networks aggregate into the provider's
announcement.

This is what changed the routing table's *growth rate* rather than merely its size, and
it is why the table has under a million entries rather than tens of millions. The cost —
which Rekhter acknowledged clearly — is that **changing provider means renumbering**,
and this remains one of IPv4 and IPv6's genuinely unsolved problems. Chapter 27 §27.4
covers what organisations do about it.

Rekhter's other work — BGP, MPLS, VPN architectures — makes him one of the most
consequential people in the routing half of this book.

**Robert (Bob) Braden (1934–2018) and the Host Requirements work.** RFC 1122 states what
a host must do with masks, addresses and the local-or-remote decision of §25.3. The
document is where the behaviour that produces the mask-mismatch symptom is specified —
not as a bug, but as the correct and only sensible thing for a host to do with the
information it has.

**Understanding that the mask mismatch is correct behaviour by both hosts** is what makes
the diagnosis obvious rather than mysterious.

**Paul Francis and Kjeld Borch Egevang.** RFC 1631 (NAT), noted in Chapter 21, and
relevant here because NAT and CIDR were the two mechanisms that together deferred IPv4
exhaustion by decades. **CIDR reduced waste; NAT reduced demand.** Neither was intended
as permanent, and both are.

**Geoff Huston.** APNIC's chief scientist and the person who has measured all
of this most carefully — address consumption, allocation efficiency, routing table
growth, and the exhaustion projections that turned out to be right. His datasets are
what turn Chapter 27's exhaustion arithmetic from an argument into a measurement.

**Peter Loshin, Rick Graziani, Todd Lammle and the pedagogical tradition.** Worth naming
because subnetting is one of the few topics in computing with a genuine teaching
literature — decades of accumulated technique about how to make binary arithmetic
tractable under exam pressure.

**The "magic number" method of Chapter 26 §26.2 has no single author.** It emerged from
instructors solving the same problem repeatedly, and it is a good example of practical
knowledge that never entered a specification and is nonetheless universally known by
people who do the work.
