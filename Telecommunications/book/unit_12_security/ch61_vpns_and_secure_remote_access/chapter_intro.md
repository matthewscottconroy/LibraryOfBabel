# Chapter 61 — VPNs and Secure Remote Access

The idea is simple enough to state in one sentence: take a packet, encrypt it, and
put it inside another packet addressed to a device that will decrypt it and forward
the original.

That is tunnelling, and a virtual private network is tunnelling plus cryptography.
Everything else in this chapter is the specifics of how the encryption is negotiated,
what exactly gets wrapped, and what the resulting arrangement does to routing.

## What a tunnel actually buys

Three things, and it is worth separating them because different deployments want
different ones.

**Confidentiality and integrity across an untrusted path.** The point most people
think of. Traffic crossing the public Internet is readable by every network it
traverses; encryption makes it useless to them.

Connectivity that the underlying network would not provide. A tunnel can carry
private RFC 1918 addresses across a network that would drop them, or IPv6 across an
IPv4-only path, or Ethernet frames across a routed network (Chapter 67's overlays).
This is often the real reason a tunnel exists, and it has nothing to do with security.

**A shared trust domain.** Machines at both ends behave as though on one network. This
is convenient and, as §61.4 argues, is increasingly regarded as a liability rather
than a feature.

## IPsec

§61.2 covers the standard for site-to-site connectivity, and it has a reputation for
complexity that is largely deserved and mostly attributable to the number of choices
it presents.

Two protocols. **AH** provides authentication and integrity but not encryption, and it
authenticates the outer IP header — which means AH cannot traverse NAT at all,
since NAT modifies exactly what AH is protecting (Chapter 33 §33.3). It is
consequently almost never used. **ESP** provides encryption, integrity and
authentication of the payload, and is what everyone deploys.

Two modes. **Transport mode** protects the payload and leaves the original IP header,
used host-to-host. **Tunnel mode** encapsulates the entire original packet inside a new
one, used gateway-to-gateway, and it is what "site-to-site VPN" means.

**IKE** — Internet Key Exchange, now IKEv2 — does the negotiation: authenticate the
peers, perform the Diffie–Hellman exchange of Chapter 58 §58.2, and establish the
security associations. The negotiation has a large number of parameters that must
match at both ends — encryption algorithm, integrity algorithm, DH group, lifetime —
and the classic operational experience of IPsec is a tunnel that will not establish
because one parameter differs, with logs that describe the failure obscurely. §61.2
covers reading those logs, which is a genuinely useful skill.

**NAT traversal** encapsulates ESP inside UDP port 4500, because ESP is IP protocol
50 and many NAT devices cannot track it. It is standard and usually automatic.

## The alternatives

§61.3 covers the two that displaced IPsec for remote access.

**TLS VPNs** run over TCP or UDP 443, which is the point: they traverse any network
that permits web browsing, which is every network. IPsec is frequently blocked on
hotel and public Wi-Fi; a TLS VPN is indistinguishable from HTTPS at the packet level.
OpenVPN is the classic instance. The cost is TCP-over-TCP when using TCP mode, whose
interacting retransmission timers produce the well-documented meltdown behaviour that
makes UDP mode strongly preferable.

**WireGuard**, merged into the Linux kernel in 2020, deserves attention as a design
argument as much as a product. Its properties are a deliberate rejection of IPsec's
flexibility:

- About 4,000 lines of code, against IPsec/IKE implementations in the hundreds of
  thousands. Small enough to audit, and it has been.
- **No cryptographic agility.** One cipher suite — ChaCha20, Poly1305, Curve25519,
  BLAKE2s — with no negotiation. Nothing to downgrade, nothing to misconfigure, and if
  a primitive is broken the protocol version changes rather than the configuration.
- **Silent by default.** Unauthenticated packets are dropped without response, so the
  endpoint is undetectable by scanning.
- **Stateless-feeling roaming.** Peers are identified by public key rather than
  address, so a client moving between networks simply continues.

The design lesson, which generalises well beyond VPNs: configurability is an attack
surface. Every negotiable parameter is a parameter that can be negotiated badly, and
the history of TLS's downgrade attacks (Chapter 58 §58.4) is the same lesson learned
expensively.

## The remote access question, post-2020

§61.4 addresses the shift, and it is a genuine architectural argument rather than a
product comparison.

The traditional model: the remote worker establishes a VPN, receives an address on the
corporate network, and is *inside*. The perimeter extends to their kitchen table.

The problems with that, in 2026:

**The applications are not inside any more.** Routing a user's Microsoft 365 traffic
through a VPN concentrator and back out is the tromboning of Chapter 51 §51.1, applied
to every remote worker simultaneously — which is what caused the widely-reported VPN
capacity crises of March 2020.

**The device is not trusted.** A home computer, shared with a family, is now on the
corporate network with the same access as a managed laptop in the office.

**It is all-or-nothing.** A VPN grants network access, not application access. A
compromised remote device has whatever the network permits, which in a flat network is
everything.

**Split tunnelling** — sending only corporate-destined traffic through the tunnel —
addresses the capacity and performance problem and is standard practice, at the cost
that the device is simultaneously on the corporate network and the open Internet.
§61.4 treats this as the real tradeoff it is rather than pretending either position is
obviously right.

The direction of travel is zero trust network access (Chapter 59 §59.4): grant
access to a specific application, for a specific authenticated user, on a device whose
posture has been checked, without ever placing that device on the network. The
practical position for most organisations is a hybrid — ZTNA for the applications that
support it, VPN for the legacy systems that do not — and being able to articulate why
is a design skill worth having.

## By the end you will be able to

- Explain what a tunnel provides and separate the security purpose from the
  connectivity purpose.
- Choose between AH and ESP, and transport and tunnel mode, and explain why AH is
  unusable through NAT.
- Diagnose an IPsec tunnel that fails to establish, from a parameter mismatch.
- Explain why TLS VPNs traverse networks that IPsec cannot.
- State three design decisions in WireGuard and what each buys.
- Argue the split tunnelling tradeoff from both sides.
- Explain why ZTNA is displacing the VPN model for new deployments, and where the
  VPN still belongs.
