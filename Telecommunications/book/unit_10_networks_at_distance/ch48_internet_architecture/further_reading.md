# Chapter 48 — Further Reading

## Foundational papers

Clark, D. (1988). "The Design Philosophy of the DARPA Internet Protocols." SIGCOMM.
**Read this one.** Twelve pages, and it states the priority ordering the designers actually
used — survivability first, **accountability last** — which explains more about the Internet's
present difficulties than any amount of commentary.

Saltzer, J., Reed, D. & Clark, D. (1984). "End-to-End Arguments in System Design."
Already cited in Chapter 23, and it belongs here too: the architectural principle that
produced the shape §48.1 describes.

Cerf, V. (1998). RFC 2468, "I Remember IANA."
Five minutes, and it conveys what the institutional history cannot.

Crocker, S. (1969). RFC 1, "Host Software."
Read it for the tone, not the content. It is a graduate student's careful notes, and the
whole culture of §48.4 is visible in it.

## Governance and institutions

Mueller, M. — *Ruling the Root: Internet Governance and the Taming of Cyberspace* (2002),
and *Networks and States* (2010).
The serious scholarly treatment, from someone who was present. *Ruling the Root* on the
ICANN formation and the Postel incident is the definitive account.

Russell, A. — *Open Standards and the Digital Age* (2014).
**The standards-war history**, and unusually good on why OSI lost — which is Chapter 22's
argument with the political and institutional detail restored.

**Abbate, J. — *Inventing the Internet* (1999).**
The general history, and the best single volume on how the culture of §48.4 formed.

ICANN's and the RIRs' own policy archives (icann.org, ripe.net, arin.net).
Dry and genuinely useful. RIPE's policy proposal archive shows a technical community
arguing about resource allocation in public, which is a good corrective to abstractions about
governance.

The IANA Stewardship Transition proposal (2015) and ICANN's accountability documents.
**Long.** The executive summaries convey the shape of the problem — how to make a function
survive its performer without handing it to a state.

## Peering and economics

Norton, W. — *The Internet Peering Playbook*, and the DrPeering white papers
(drpeering.net).
The practical reference, and frank in a way industry documents are not. The peering
negotiation tactics chapters are unlike anything else published.

Faratin, P. et al. (2008). "The Growing Complexity of Internet Interconnection."
*Communications & Strategies*. The academic treatment of the disputes in §48.2, written
before the Netflix–Comcast case made them public.

Labovitz, C. et al. (2010). "Internet Inter-Domain Traffic." SIGCOMM.
The paper that documented the flattening, with two years of measured data from over a
hundred networks. §48.1's argument, established empirically, and it changed how the
industry described itself.

The FCC's 2015 and 2018 Open Internet proceedings, and the corresponding European BEREC
documents.
Read a filing from each side of an interconnection dispute. They are public, they are
adversarial, and reading both is the fastest way to understand that the engineering and the
argument are separable.

## Address policy

Huston, G. — the APNIC blog (blog.apnic.net) and **potaroo.net**.
The single best ongoing source on address policy, routing table growth and IPv6 adoption.
Huston has been measuring and writing about this for twenty-five years, with data rather
than opinion, and his annual "BGP in 20xx" reviews are the reference for Chapter 32 §32.2's
growth question.

The RIR policy manuals — ARIN's NRPM, the RIPE policy documents, APNIC's policies.
Consult rather than read. Knowing they exist and how to search them is the skill.

Edelman, B. (2009). "Running Out of Numbers: The Impending Scarcity of IP Addresses."
The economics of exhaustion, written while it was still impending.

## Measurement and tools

**RIPE Atlas** (atlas.ripe.net) — a global measurement network of thousands of small probes,
free to use. Host one and earn credits to run measurements from anywhere. The single best
practical tool for the questions in this chapter.

RIPE RIS and RouteViews — public BGP route collectors with decades of archived data.
**F1 uses these.**

bgp.tools, stat.ripe.net, bgpview.io, he.net/BGP — web front-ends for AS relationships,
prefix ownership and peering visibility. `bgp.tools` is the most readable.

**PeeringDB** (peeringdb.com) — the industry's own directory: who is present at which
exchange, with what capacity, and their peering policy. Maintained by the networks
themselves, and the first place to look when planning interconnection.

**Public looking glasses** — most large networks run one. Seeing your own prefix from four
other networks' perspectives is instructive, and it is F1.

`whois`, and RDAP clients — `whois 8.8.8.8` on any Unix; RDAP via `rdap.org` or the RIRs'
own endpoints for structured output.

## Following the field

**NANOG, RIPE, APRICOT and AfPIF meeting archives.**
Videos and slides are public. NANOG's outage post-mortems and RIPE's plenary talks are
where operational reality is discussed by the people it happened to, and they are more
current than any book including this one.

**The IETF datatracker** (datatracker.ietf.org) — every draft, every revision, every working
group, every mailing list. F4 uses it.

**Internet Society's Pulse** (pulse.internetsociety.org) — measurement of Internet resilience,
IPv6 adoption and market concentration by country.

## Where to look next

**Chapter 49** goes down to the access link that connects a user to the structure described
here; **Chapter 50** covers the physical transport underneath it; and **Chapter 52** takes up
content delivery and anycast, which is §48.1's flattening seen from the content network's
side.
