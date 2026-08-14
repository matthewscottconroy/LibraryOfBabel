# Chapter 48 — Internet Architecture

The Internet has no owner, no headquarters, no central router, and no off switch.
This is stated so often that it has become background noise, and it is worth
recovering the strangeness of it, because it is genuinely unusual among
infrastructures.

There is no organisation that operates the Internet. There are approximately
seventy-five thousand autonomous systems (Chapter 32), each independently operated,
each choosing its own equipment and policies, connected by bilateral agreements that
are individually negotiated. Nobody has a complete inventory. Nobody could produce
one. And the whole thing works well enough that its reliability is unremarkable
enough to be assumed.

## The shape, corrected

The traditional picture is a three-tier pyramid: tier 1 backbones at the top, tier 2
regional networks buying transit from them, tier 3 access providers at the bottom.
Traffic climbs the pyramid and descends the other side.

That picture was approximately right in 1998 and is now substantially wrong, and
§48.1 replaces it.

What changed is that **content moved**. In 2000, content was distributed across
millions of small servers, and reaching it required traversing the backbone. Today a
large majority of consumer traffic originates from a small number of very large
sources — video platforms, social networks, cloud providers, CDNs — and every one of
them has spent the last fifteen years deploying equipment as close to end users as
possible and peering directly with access networks.

The consequence is a **flattened** Internet. An access ISP in a mid-sized European
city typically peers directly with the major content networks at a nearby exchange,
or hosts their cache servers inside its own network. That traffic never touches a
transit backbone at all. Transit is still essential — for reaching the long tail of
everything else — but it carries a much smaller share of bytes than the pyramid
implies.

Two consequences that matter operationally: latency to popular content is far lower
than the geography would suggest (the content is not where you think it is), and a
larger share of the Internet's traffic depends on a smaller number of organisations
than at any point in its history, which is a resilience question the chapter takes
seriously rather than sloganeering about.

## The money, made concrete

Chapter 32 introduced transit and peering. §48.2 makes them operational.

**Transit** is a customer relationship. You pay a provider — typically per megabit
per second at the 95th percentile of measured usage, which is a billing convention
worth understanding because it means brief spikes are free and sustained load is not
— and they carry your traffic to and from the entire Internet. They advertise your
routes everywhere and everyone's routes to you.

**Peering** is a lateral relationship, usually settlement-free. Two networks exchange
traffic between their own customers only. Neither pays. Both save transit fees on
that traffic. Crucially, a peer does not provide transit *through* itself to third
parties, and a peer that begins doing so — accidentally, usually through a
configuration error — is committing the route leak of Chapter 32 §32.4.

**Internet exchange points** make peering cheap. Instead of running a cable to each
of forty peers, every participant connects once to a shared switching fabric in a
neutral facility and peers with everyone present over that single connection. DE-CIX
Frankfurt, AMS-IX Amsterdam and LINX London each carry many terabits per second at
peak. §48.2 covers the economics — why an IXP port plus forty peering sessions is
dramatically cheaper than the equivalent transit, and why the decision to peer or buy
is made on a spreadsheet rather than on principle.

The negotiation itself is worth knowing about because it is unlike most technical
work. Peering is agreed between engineers, often informally, sometimes at conferences
and sometimes over dinner, and the criteria — comparable traffic volumes, comparable
geographic footprints, a roughly balanced ratio — are conventions rather than rules.
A network that sends far more than it receives may be refused peering by one that
would then bear the cost of carrying it, which is why content networks and access
networks periodically have public disputes about who should pay whom.

## Where addresses come from

§48.3 traces the delegation chain, which mirrors DNS's structure (Chapter 39 §39.1)
and for the same reasons.

**IANA**, operated by PTI under ICANN, holds the top-level pools of IP address space, AS
numbers and protocol parameters. It allocates large blocks to the five **regional
Internet registries** — ARIN, RIPE NCC, APNIC, LACNIC and AFRINIC — which allocate to
local registries and end users within their regions, under regionally-set policies
made by their own member communities.

The IPv4 free pool is empty (Chapter 28), so allocations now come from transfer
markets and from reclaimed space, and the RIRs maintain transfer policies and
registries for this. The **WHOIS** and RDAP databases record who holds what, and are
the first place to look when identifying the operator of an address involved in an
incident.

## How a standard happens

§48.4 covers the process, because knowing it changes how you read a specification.

The **IETF** produces the Internet's protocol standards, and does so with no formal
membership, no voting, and no fees. Anyone may join a mailing list and contribute.
Documents progress from Internet-Draft to RFC through working group consensus, and
the guiding norm is Dave Clark's 1992 formulation: *"We reject: kings, presidents and
voting. We believe in: rough consensus and running code."*

That last clause is load-bearing. The IETF's culture strongly favours specifications
that have been implemented and tested over specifications that have been designed —
which is, per Chapter 22 §22.1, precisely the difference that decided OSI's fate.

**Not every RFC is a standard.** This surprises people. The RFC series includes
Proposed Standards, Internet Standards, Informational documents, Experimental
documents, Best Current Practices, Historic documents, and the annual April Fools
publications (RFC 1149, *IP over Avian Carriers*, is real, is numbered, and has been
implemented). Checking a document's status before citing it is a small habit that
prevents embarrassment.

Other bodies: **IEEE** for the 802 family (Ethernet, Wi-Fi); **ITU-T** for
international telecommunications; **3GPP** for cellular; **W3C** for the web;
**ICANN** for names and the root zone. Appendix C sets out who does what.

## By the end you will be able to

- Draw the modern Internet's structure and explain why the tier pyramid is outdated.
- Explain transit, peering and 95th-percentile billing, and compute which is cheaper
  for a given traffic profile.
- Explain what an IXP is and why it changes peering economics.
- Trace the address delegation chain from IANA to an end user, and look up who holds
  a given prefix.
- Explain how an IETF standard is made and what an RFC's status tells you.
- Read a traceroute and identify network boundaries, geography and transit
  relationships.
