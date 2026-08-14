# Chapter 51 — Further Reading

## Specifications and standards

RFC 4364 — "BGP/MPLS IP Virtual Private Networks."
The document that defined the product this chapter describes the displacement of. Read the
RD/RT sections; they are a small model of how to specify a service.

ITU-T I.122 and the Frame Relay Forum implementation agreements.
Historical, and worth ten minutes to see what a virtual-circuit WAN service looked like as
a specification.

RFC 4301, 4303 — IPsec architecture and ESP.
The encryption every SD-WAN overlay uses. Chapter 61 covers it properly; the architecture
document is the one to skim now.

MEF 70.x — SD-WAN Service Attributes and Services.
The only vendor-neutral SD-WAN specification that exists, from the Metro Ethernet Forum.
It defines the terminology precisely, which is genuinely useful when comparing products
whose marketing uses the same words differently. It has not produced interoperability, and
the reasons are instructive.

**NIST SP 800-207 — Zero Trust Architecture.**
The reference document, and short. F6 uses it. Read the tenets and then honestly assess a
network you know against them.

## Books and long-form

Yates, J. & Chandrasekaran, A. — vendor-neutral SD-WAN treatments are scarce; the best
available material is:

Segeč, P. et al., and the various *SD-WAN* practitioner books from the major publishers.
Read at least two, from authors aligned with different vendors, because the differences are
where the real design questions live.

Beyer, B. et al. — the BeyondCorp papers (research.google, 2014–2018).
Six papers, freely available. The migration paper is the valuable one — how long it
took, what broke, what had to be handled specially. Most zero trust material describes an
architecture; this describes a project.

Gilman, E. & Barth, D. — *Zero Trust Networks*.
**The practical book**, and it is honest about what the model does not solve.

Doyle, J. & Carroll, J. — *Routing TCP/IP*.
Not about SD-WAN, and the BGP and policy chapters are the foundation for understanding what
an SD-WAN's route policy is actually doing — and what your direct cloud interconnect's BGP
session needs.

## Papers and analysis

Kindervag, J. (2010). "No More Chewy Centers: Introducing the Zero Trust Model of
Information Security." Forrester.
The paper that named it. Short, and the argument is clearer than most of what followed.

Saltzer, J. & Schroeder, M. (1975). "The Protection of Information in Computer Systems."
Where least privilege comes from, and it predates zero trust by thirty-five years. Read
sections 1 and 2 and note how much of the modern discussion is in them already.

**The Jericho Forum's de-perimeterisation papers (2004–2007).**
The argument, made six years before it had a fundable name.

Cloud providers' own architecture centres — the AWS Well-Architected Framework's networking
pillar, Azure's Cloud Adoption Framework, Google's architecture centre.
Vendor material, and unusually good on the interconnect design questions in §51.3 — the
redundancy requirements and SLA conditions are stated precisely because they are contractual.

## Pricing and economics

The cloud providers' current pricing pages — and use them rather than any figure in this
book, including the ones in §51.3, which are illustrative and will be out of date.

**Cloud cost calculators**, used sceptically. **Model egress explicitly**; it is the line most
often omitted and most often decisive.

**TeleGeography's WAN and IP transit pricing reports.**
Actual market pricing for MPLS, Ethernet and Internet transit by region, updated annually.
The single best source for the arithmetic in §51.1, and F1 is much easier with it.

## Practical work

**A lab SD-WAN.** F3 builds one. WireGuard plus a policy script on two Linux boxes will
demonstrate the core behaviour — measure both paths, steer per application, fail over — and
building it teaches more than any product evaluation. Add `tc netem` to introduce loss and
latency deliberately.

**`tc` and `netem`** — Linux traffic control. Essential for any WAN experiment, and worth
learning independently of this chapter.

**Vendor trial licences.** Most SD-WAN vendors offer them. Evaluate the controller-outage
behaviour specifically (F7), because it is the question the sales process will not answer.

Digital experience monitoring trials — ThousandEyes, Catchpoint and open alternatives.
§51.4's argument is that endpoint measurement is now the only visibility you have; trying
one makes the point concrete.

**Your own VPN, measured.** F5 needs nothing but a laptop: compare latency to a cloud service
with and without the corporate tunnel, and the tromboning penalty becomes a number rather
than a diagram.

## Following the field

Gartner's Magic Quadrant coverage of SD-WAN and SASE, read as a market document rather than
a technical one. It is useful for knowing who exists and what the category boundaries are
claimed to be, and for nothing else.

Independent analyst and practitioner writing — Ivan Pepelnjak's *ipSpace* material is the
standard recommendation, and it is consistently the most sceptical and best-reasoned source
on SD-WAN and network overlays available. Read it against any vendor document (F4).

NANOG and RIPE operator presentations on enterprise interconnection and cloud on-ramps.

The cloud providers' networking release notes — because interconnect products, egress
pricing and route limits change, and an architecture built on last year's limits will
eventually meet this year's.

## Where to look next

**Chapter 52** covers what happens to traffic when the links of this chapter are congested;
**Chapter 59** develops the identity and access model §51.4 introduces; **Chapter 60** covers
the segmentation that local breakout makes necessary; **Chapter 61** covers the IPsec
underneath every overlay here; and **Chapter 69** covers the cloud networking that the
interconnects of §51.3 terminate in.
