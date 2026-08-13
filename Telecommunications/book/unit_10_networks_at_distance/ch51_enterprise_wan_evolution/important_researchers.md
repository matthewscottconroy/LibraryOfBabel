# Chapter 51 — The People

**This chapter's history is corporate rather than academic**, and pretending otherwise would
misrepresent it. **The enterprise WAN was shaped by product managers, standards committees and
finance directors at least as much as by engineers**, and the people worth knowing are a mixture
of the three.

**The Frame Relay "Gang of Four" — Cisco, DEC, Northern Telecom and StrataCom.** The protocol
came out of ITU-T's ISDN work around 1988, **and these four vendors produced the extensions in
1990 that made it a deployable product.** Its design principle is the one worth extracting.

**Frame Relay is X.25 with the error correction removed.**

**X.25 (1976) performed error detection and retransmission at every hop**, because the links of
the 1970s were analogue, noisy and unreliable. **By 1990 the links were digital fibre with error
rates a million times lower** — and per-hop error correction had become **pure overhead that
bought nothing.**

> **Frame Relay's contribution was a deletion.** Detect errors, discard the frame, and **let
> the endpoints sort it out** — which is Chapter 23 §23.4's end-to-end argument arriving in
> carrier networking, driven not by principle but by the observation that the links had got
> good.

**And the same deletion happened twice more:** ATM removed even more, **and MPLS removed ATM's
cell structure.** **Each generation deleted a compensation for a problem that had stopped
existing.**

**Yakov Rekhter and Eric Rosen, again.** **RFC 4364 — BGP/MPLS IP VPNs** — is the document that
defined the product this chapter is about the displacement of.

**Its design achievement is worth restating because it is easy to miss:** **the carrier's core
routers know nothing about any customer.** Ten thousand enterprises with overlapping 10.0.0.0/8
share one infrastructure, **and the P routers hold one routing table.**

> **That is a scaling property, and it is why MPLS L3VPN survived fifteen years of
> alternatives.** The technologies that replaced it did not beat it on scaling. **They beat it
> on price and lead time**, which Chapter 20's Ethernet history should have led everyone to
> expect.

**Martin Casado, Nick McKeown and Scott Shenker.** **Not SD-WAN's authors, and its
intellectual source.**

**Their 2007–2008 work on separating control from forwarding** (Chapter 68 covers it properly)
**produced the idea that a network's policy should live in one place and be pushed to simple
forwarding elements.** **SD-WAN is that idea applied to the WAN**, and it reached production
there **before it reached production in the data centre** — because the WAN's pain was worse and
the devices were fewer.

> **A recurring pattern: an architectural idea is proven first where the existing solution is
> most expensive**, not where it is most elegant. **Chapter 45 §45.2 makes the same observation
> about wireless controllers arriving before data centre SDN.**

**Viptela, VeloCloud, CloudGenix, Silver Peak and Versa.** **Between about 2012 and 2015**,
several independent startups built substantially the same product without coordination.

**That simultaneity is the interesting fact.** **When several unconnected teams build the same
thing at the same time, the cause is usually a change in the environment rather than an
insight** — and here the change was **broadband becoming good enough, and cloud traffic becoming
the majority.**

**Most were acquired within five years** — **Viptela by Cisco, VeloCloud by VMware, CloudGenix
by Palo Alto Networks, Silver Peak by HPE** — **which tells you what the incumbents thought was
about to happen to their MPLS revenue.**

**John Kindervag.** **Forrester Research, 2010 — and the term "zero trust".**

**Kindervag's argument was that the perimeter model's core assumption — that internal traffic
is trustworthy — had been false for years and nobody had said so.** Once an attacker is inside,
**a hard shell around a soft interior offers nothing**, and the breaches of the period
demonstrated it repeatedly.

**His formulation — "never trust, always verify" — is a slogan**, and the substance beneath it
is not: **remove implicit trust from location, authenticate every request, and grant the
minimum access required.**

> **The idea was not novel.** The Jericho Forum had argued for "de-perimeterisation" from 2004,
> and the underlying principle is Saltzer and Schroeder's least privilege from 1975. **What
> Kindervag supplied was a name that a board would fund**, and that turns out to matter.

**BeyondCorp — Google, 2014.** **Zero trust implemented at scale, and documented.**

**Google published a series of papers describing an internal network where being on the
corporate network conferred no privilege at all**: every request authenticated, every device
inventoried and its posture checked, **access decided per application.**

> **The papers are unusually valuable because they describe the migration**, not just the
> destination — **including how long it took (years), what broke, and which legacy systems had
> to be handled specially.** Most zero trust material describes an architecture; **BeyondCorp
> describes a project.**

## What this chapter's history actually shows

**Three observations that generalise.**

**Deletion is a design contribution.** **X.25 → Frame Relay → ATM → MPLS is a sequence of
removals**, each valid because the problem the removed mechanism solved had ceased to exist.
**Recognising that a compensation is no longer needed is harder than adding one**, because
nothing fails when you leave it in.

**Price and lead time beat technical merit, reliably.** **MPLS was better at what it did than
what replaced it.** It was displaced anyway. **Chapter 20's Ethernet story, Chapter 22's OSI
story and this one all have the same shape.**

**Simultaneous independent invention indicates an environmental change.** **Five SD-WAN
startups in three years** were not five insights. **They were five responses to broadband
becoming good and traffic moving to the cloud**, and the useful question when you see that
pattern is "what changed?" rather than "who was first?"
