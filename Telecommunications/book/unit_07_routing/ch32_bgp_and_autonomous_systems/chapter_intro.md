# Chapter 32 — BGP and Autonomous Systems

The Border Gateway Protocol was sketched, according to its authors Kirk Lougheed and
Yakov Rekhter, on the backs of two napkins during an IETF meeting in 1989. It is
still referred to in the community as the two-napkin protocol, and RFC 1105 —
published in June 1989 — is seventeen pages long.

BGP now carries every route on the public Internet. The global table it maintains
passed a million IPv4 prefixes in 2021 and continues to grow. It is the protocol
that makes the Internet a single network rather than seventy thousand separate ones,
it has no central authority of any kind, and it works because approximately seventy
thousand independent organisations, many of them direct commercial competitors,
choose to tell each other approximately the truth.

That last sentence is the whole chapter. BGP's technical mechanisms are not
especially hard. What makes BGP different from everything in Chapter 31 is that it
operates in an environment of **conflicting interests and absent trust**, and its
design decisions only make sense in that light.

## Why interior protocols do not work here

Chapter 31's protocols find the shortest path. Between organisations, that is the
wrong objective, and often an actively unacceptable one.

Consider three networks: your ISP, a competitor, and a customer. The shortest path
from you to the customer may run through the competitor. Do you want to send traffic
that way? Perhaps — if the competitor has agreed to carry it, and if you are not
paying them per byte, and if their network is reliable, and if there is no
contractual reason not to. Those are commercial questions and no shortest-path
algorithm can express them.

Worse: does the competitor want to *carry* it? If they are not being paid, no. But
OSPF has no way to say "I can reach this destination but I decline to carry your
traffic to it." It advertises reachability, and reachability implies willingness.

So an inter-domain protocol must support:

- **Policy as the primary input**, overriding path length entirely.
- **Selective advertisement** — the ability to tell different neighbours different
  things about the same destination.
- **Loop detection without trust** — since you cannot assume neighbours are honest
  or even correct.
- **Enormous scale** with acceptable stability — a million prefixes, changing
  constantly, without recomputing everything.

BGP does all four, and it does the third with a mechanism worth naming: it is a
**path vector** protocol. An advertisement carries not a distance but the *entire
list of autonomous systems* the route has traversed. A router that sees its own AS
number in the path discards the advertisement, because accepting it would create a
loop. Loop detection requires no trust and no global view — merely the ability to
recognise your own name.

## The autonomous system

An **autonomous system** is a set of networks under one administrative and routing
policy. Not one organisation necessarily, and not one technology: an AS is a
*policy* boundary. It is identified by a number allocated by a regional registry —
originally 16 bits, extended to 32 bits by RFC 6793 when the space grew tight.

Within an AS, run whatever interior protocol you like; nobody outside can see it or
cares. Between autonomous systems, run BGP. That division is the architectural
statement of the whole unit, and it is why "IGP" and "EGP" are the categories they
are.

## The money

§32.3 is the section that most students find changes their picture of the Internet,
because it explains the shape of the network in terms of contracts rather than
topology.

**Transit** is a paid relationship: a customer pays a provider to carry its traffic
to and from the entire Internet. The provider advertises the customer's routes to
everyone and advertises everyone's routes to the customer.

**Peering** is usually settlement-free: two networks exchange traffic *between their
own customers only*, at no charge, because both benefit roughly equally and both
save on transit fees. Crucially, a peer does *not* advertise its other peers' or
providers' routes — peering is not transit, and a peer that starts behaving like one
is a **route leak**.

**Internet exchange points** are physical facilities where many networks meet to
peer cheaply over a shared fabric. DE-CIX in Frankfurt, AMS-IX in Amsterdam and
LINX in London each carry many terabits per second.

The resulting structure is not a hierarchy in any clean sense, and the once-standard
"tier 1 / tier 2 / tier 3" description has become misleading. A handful of networks
buy transit from nobody; a large number of content networks and CDNs peer extremely
widely and buy little; the traffic map has flattened enormously since 2010 as
content moved to a small number of very large sources. Chapter 48 draws the picture
properly.

## When it goes wrong

BGP's founding assumption — that participants tell approximately the truth — has no
enforcement mechanism, and §32.4 examines the consequences with real incidents.

**In February 2008**, Pakistan Telecom, attempting to block YouTube domestically,
advertised a more specific prefix for YouTube's address space. The advertisement
escaped to their upstream provider and propagated globally. Longest-prefix match
(Chapter 29 §29.3) did the rest: because the hijacked prefix was *more specific*
than YouTube's own, routers worldwide preferred it. YouTube was unreachable for
roughly two hours.

**In June 2019**, a small Pennsylvania network with a misconfigured
route optimiser leaked routes learned from one provider to another, and Verizon
accepted them without filtering. Large parts of Cloudflare, Amazon and others became
unreachable for two hours. The mitigation — filtering customer advertisements
against what they are authorised to announce — was standard practice that had not
been applied.

**In October 2021**, Facebook withdrew the BGP routes to its own DNS servers during
a maintenance error and disappeared from the Internet for six hours, including the
internal tooling and physical access systems its engineers needed to fix it.

The defences — RPKI origin validation, route object filtering, maximum-prefix limits,
and the MANRS norms — are covered in §32.4, along with an honest assessment of how
partially deployed they remain.

## By the end you will be able to

- Explain what an autonomous system is and why the boundary is administrative.
- Explain why path vector rather than distance vector or link state, in terms of
  loop detection without trust.
- Describe transit and peering and explain what each party advertises to whom.
- Trace the BGP path selection process and identify which attribute decided.
- Explain how a prefix hijack works, why longest-prefix match makes it effective,
  and what RPKI does about it.
- Explain a route leak and the filtering that prevents it.
