# 23.2 The Four-Layer Model

The TCP/IP model has four layers, and it differs from OSI in a way that is easy to
miss: **it was not designed.** It is a description, written after the fact, of what the
protocols already did.

RFC 1122 (1989) — *Requirements for Internet Hosts* — states it, and the document's
purpose is to tell implementers what a host must do, not to propose an architecture.
The model is the shape the requirements happened to have.

## The four layers

```
   ┌───────────────────┬────────────────────────────────────┐
   │   Application     │  HTTP, DNS, SMTP, SSH, DHCP, …     │
   ├───────────────────┼────────────────────────────────────┤
   │   Transport       │  TCP, UDP, (QUIC, SCTP, DCCP)      │
   ├───────────────────┼────────────────────────────────────┤
   │   Internet        │  IP, ICMP, IGMP                    │
   ├───────────────────┼────────────────────────────────────┤
   │   Link            │  Ethernet, Wi-Fi, PPP, …           │
   └───────────────────┴────────────────────────────────────┘
```

Mapped against OSI:

| OSI | TCP/IP | Comment |
|---|---|---|
| 7 Application | | |
| 6 Presentation | **Application** | **all three collapsed** |
| 5 Session | | |
| 4 Transport | **Transport** | identical |
| 3 Network | **Internet** | identical; different name |
| 2 Data Link | **Link** | **both collapsed** |
| 1 Physical | | |

Two collapses, and each is deliberate.

## Collapse 1: layers 5, 6, 7 → Application

Chapter 22 §22.3 made the case: **there is no separate implementation of a session
layer or a presentation layer.** An application does its own session management, its
own encoding, its own encryption, usually inside one program using libraries.

There is no boundary to point at. No header marks the transition. No device operates at
"layer 6". The functions exist; the layers do not.

**The TCP/IP model says so, and it is the more accurate description.**

## Collapse 2: layers 1, 2 → Link

Less obvious, and the reasoning is different — it is not that the distinction is
unreal, but that **it is not IP's business.**

IP requires exactly one thing of the layer below: *deliver this packet to the next hop
on this network.* How that happens — voltages, frames, addresses, medium access — is
entirely the link's affair, and IP is deliberately incurious about it.

RFC 1122's phrasing captures the attitude: the link layer is defined as whatever
"provides the means to transmit an IP datagram to the next hop". That is a functional
definition, not a structural one, and it accommodates Ethernet, Wi-Fi, PPP over a
serial line, a satellite link, or a carrier pigeon (RFC 1149, which is a joke that has
been implemented).

**The refusal to specify is the design.**

## The refusal to specify

This is the model's most important property and it is easy to miss because it looks
like absence.

OSI specified everything. Physical media, connectors, session semantics, presentation
encodings — all of it, in detail.

TCP/IP specified **IP, TCP and UDP precisely, and almost nothing else**. RFC 1122 tells
you what a host must do at the internet and transport layers. For the link layer it
says, in effect: *use whatever is there.*

**The consequence:**

| Year | New link technology | IP changes required |
|---|---|---|
| 1983 | Ethernet 10 Mb/s | none |
| 1990 | FDDI | none |
| 1997 | Wi-Fi 802.11 | none |
| 1999 | DSL | none |
| 2008 | LTE | none |
| 2019 | 5G | none |
| 2020s | 400 Gb/s Ethernet, LEO satellite | none |

Not one required a change to IP. Because IP never said what a link was, every new link
was already accommodated.

> **The specification's silence is its most valuable feature.** OSI's completeness
> made it correct and rigid. TCP/IP's minimalism made it adaptable, and adaptability
> is what a forty-year-old protocol needs.

The general principle: **a specification constrains the future.** Anything you specify
now, someone must comply with later, or work around. Specifying less leaves more room —
at the cost of interoperability problems that must be solved elsewhere.

## What is actually at each layer

**Link.** Ethernet, Wi-Fi, PPP, and the addressing and framing of Units II–IV. ARP
lives here, awkwardly (Chapter 18 §18.1). MTU is a link property with consequences all
the way up.

**Internet.** **IP, and essentially nothing else.** ICMP (Chapter 34 §34.1) is IP's
control protocol; IGMP handles multicast group membership. That is the layer.

The thinness is the point (§23.4). IP does addressing, forwarding, fragmentation and a
hop limit. It does not do reliability, ordering, security, congestion control, or
quality of service. **Everything IP does not do was a decision, and each one is defended
by the end-to-end argument.**

**Transport.** TCP and UDP, and — increasingly — others:

| Protocol | Status |
|---|---|
| TCP | universal |
| UDP | universal |
| **QUIC** | major and growing; **inside UDP**, in user space |
| SCTP | telecoms signalling, WebRTC data channels |
| DCCP | essentially unused |

**QUIC's position is worth noting** (Chapter 38). It is a transport protocol carried
inside another transport protocol, because deploying a genuinely new IP protocol number
is impossible — middleboxes drop what they do not recognise. **The ossification of
§21.4 forced a new transport to disguise itself as UDP**, and it worked.

**Application.** Everything else. HTTP, DNS, SMTP, SSH, DHCP, SNMP, NTP, TLS, and the
tens of thousands of protocols nobody standardised.

## The five-layer compromise

Many textbooks teach **five** layers — TCP/IP's four with the link layer split back
into physical and data link:

```
   Application
   Transport
   Network
   Data Link
   Physical
```

It is not a standard. It is pedagogy, and it is defensible: the physical/data-link
distinction is real and useful — cable faults and duplex mismatches are genuinely
different problems — even though IP does not care about it.

**Three models, and which to use:**

| Model | Use it for |
|---|---|
| **OSI (7)** | vocabulary, certification, **troubleshooting method** |
| **TCP/IP (4)** | describing what actually runs |
| **Five-layer** | teaching, and everyday reasoning |

**They are not in conflict.** They are different decompositions of the same system for
different purposes, and a competent engineer moves between them without noticing. Being
troubled by the disagreement is a sign of taking the models too literally.

## What the model does not have

Worth listing, because their absence is informative:

**No session layer.** TCP connections and application-level sessions cover it.

**No presentation layer.** Applications encode their own data; the world converged on
Unicode and IEEE 754.

**No security layer.** This one is a genuine gap, and it was a deliberate omission that
proved costly. Security was assumed to be an application concern, and the result is
that **security was added to nearly every protocol individually, decades later**: TLS
for TCP, DNSSEC for DNS, RPKI for BGP, IPsec for IP, WPA for Wi-Fi.

Each was retrofitted, each was harder than it would have been by design, and several
are still incomplete. Chapter 57 §57.1 makes the argument that **the original
architecture's largest mistake was assuming a trustworthy network**, and this is where
that mistake is visible in the model.

**No management layer.** SNMP is an application protocol. Network management was an
afterthought, and Chapter 53 lives with the consequences.

## What breaks here

**Insisting on one model.** Use whichever fits the purpose.

**Assuming four layers means less structure.** It means less *specified* structure. The
structure is there; it is just not mandated.

**Expecting the model to accommodate everything.** QUIC does not fit. TLS does not fit.
Chapter 21 §21.4 explains why models do not accommodate everything.

**Reading the absence of a security layer as an oversight.** It was a decision, made in
a context where the network was trusted, and it is the most expensive decision in the
suite.

> **Network+ note.** Objective 1.1 expects both models and the mapping between them.
> Over-learn: **TCP/IP has four layers**; **Application = OSI 5+6+7**; **Link = OSI
> 1+2**; **Transport and Internet map one-to-one onto OSI 4 and 3**. The five-layer
> variant appears in some materials — recognise it, and know it is pedagogy rather than
> a standard.
