# Chapter 61 — Important Concepts

A VPN is tunnelling plus cryptography *(§61.1)* — Take a packet, encrypt it, and put it
inside another addressed to a device that will decrypt and forward it. Two addresses per
packet: the outer pair routable and public, the inner pair whatever you like.

Only one of the three things a tunnel buys requires cryptography *(§61.1)* —
Confidentiality across an untrusted path does; carrying RFC 1918 addresses, IPv6, Ethernet
frames or multicast over a network that would not is a property of encapsulation alone. A
GRE tunnel with no encryption is a perfectly sensible thing to build, and asking "what is this
tunnel for?" distinguishes the cases.

The third thing a tunnel buys is now regarded as a liability *(§61.1)* — A shared trust
domain is exactly the "being on the network confers privilege" assumption Chapter 59 §59.4
rejects.

GRE over IPsec exists because IPsec tunnel mode carries IP and not multicast *(§61.1)* — So
OSPF's hellos cannot run directly over it. A virtual tunnel interface removes the need,
and where the platform supports one, use it.

PPTP's authentication and encryption are broken *(§61.1)* — Its presence in a configuration
is a finding.

Split tunnelling is correct for most organisations and dilutes the inspection point
*(§61.1)* — In March 2020 full tunnelling meant every video call crossed the corporate link
twice, against concentrators sized for 10% of staff. The mitigation is that inspection
should not depend on the tunnel — an endpoint agent or a cloud service sees the traffic
regardless of path. And the "client as a bridge" concern is real and overstated: the
compromise is on the client either way, so host firewalling is the genuine mitigation.

Small packets work and large ones vanish *(§61.1)* — Every tunnel adds a header, and the
PMTUD black hole follows when ICMP Type 3 Code 4 is filtered somewhere you do not control.
MSS clamping fixes TCP reliably and requires nothing of the endpoints, which is why it is
deployed almost universally. Set the interface MTU too, and check both directions.

A tunnel that flaps rhythmically is usually recursive routing *(§61.1)* — The route to the
tunnel's own endpoint is learned through the tunnel. Ensure it is learned outside — a static
route, or route filtering.

AH cannot traverse NAT, ever *(§61.2)* — It authenticates the outer IP header, which is
exactly what NAT modifies. This single property made it irrelevant. Use ESP.

IPsec's complexity is the number of things two independently-configured devices must agree
on *(§61.2)* — **Not the cryptography.** Phase 1's encryption, integrity, DH group,
authentication and lifetime; phase 2's protocol, transform, PFS group, lifetime and traffic
selectors. WireGuard's argument is precisely that most of these choices should not exist.

IKEv1's aggressive mode sends the identity in the clear *(§61.2)* — And subjects a
pre-shared key to offline dictionary attack. It exists because main mode with a PSK cannot
support a dynamic-address peer; IKEv2 solves that properly, in four exchanges rather than six
or nine, with NAT traversal and dead peer detection built in.

**Use route-based tunnels** *(§61.2)* — A VTI is an interface: it carries a routing protocol,
it is monitored like an interface, and failover is a routing problem. Policy-based IPsec is
a large source of the protocol's reputation.

ESP has no ports, so NAT cannot map it *(§61.2)* — NAT-T encapsulates ESP in UDP 4500,
costing eight bytes and requiring keepalives — typically every 20 seconds — to hold the
mapping open. A tunnel that drops after a few minutes of idleness has a keepalive interval
longer than the NAT device's timeout.

A drop every 3,600 seconds is not a coincidence *(§61.2)* — Note the interval and compare
it with the configured lifetimes. Mismatched lifetimes, rekey collisions, or a volume-based
lifetime reached quickly.

Pre-shared keys fail at about twenty sites *(§61.2)* — Chapter 58 §58.1's key distribution
arithmetic. And the PSK is in every configuration backup (Chapter 55 §55.4), so rotating it
requires coordinated changes at both ends of every tunnel — which is why they are typically
the original values.

A TLS VPN's entire argument is that port 443 outbound is permitted everywhere *(§61.3)* —
Because blocking it breaks the web. Which is why remote access moved to TLS and site-to-site
did not.

Tunnelling TCP inside TCP produces meltdown *(§61.3)* — Both layers retransmit and their
timers interact, so throughput collapses under loss rather than degrading. Serious TLS VPNs
use DTLS or QUIC over UDP and fall back to TCP only when UDP is blocked — and if yours is on
TCP over a lossy link, that is the explanation.

TLS VPN concentrators are a recurring source of critical vulnerabilities *(§61.3)* —
Internet-facing, terminating TLS, parsing complex input, and attractive. On Chapter 55 §55.3's
emergency patching track by definition.

WireGuard is about 4,000 lines against IPsec's hundreds of thousands *(§61.3)* — One fixed
cipher suite, no negotiation, no downgrade, no mismatched-proposal failure. The trade is
agility, argued both ways; TLS 1.3 reduced its suites for the same reason.

A WireGuard endpoint does not respond to unauthenticated packets at all *(§61.3)* — Port
scanning finds nothing, and there is no handshake to start without a valid key — which removes
an entire class of pre-authentication attack surface.

`AllowedIPs` is both the route and the ingress filter *(§61.3)* — Routing and authorisation
are one configuration line, which eliminates the class of misconfiguration in which they
disagree.

WireGuard has no key distribution, deliberately *(§61.3)* — The protocol is incomplete,
and the products built on it supply the identity layer it declines to specify. "WireGuard is
simpler than IPsec" compares a transport protocol with a complete system; the fair comparison
still favours it, by less.

A VPN that is slow is almost never slow because of the cipher *(§61.3)* — Check MTU, then
whether the crypto is in the kernel, then the CPU — in that order. Kernel implementations
substantially outperform userspace ones because the copies cost more than the encryption.

Full tunnelling is roughly six times the bandwidth of split *(§61.4)* — 1,080 concurrent
users at 2.5 Mb/s is 2.7 Gb/s against 430 Mb/s. Organisations that full-tunnelled discovered
their concentrators, circuits and firewalls were all undersized simultaneously.

Size for the event that makes everyone remote *(§61.4)* — That event is exactly when the
VPN matters, and it is not a rare category: weather, transport, a building problem, a
pandemic. And check the licence count — many products licence by concurrent session and
produce a hard, confusing failure at the limit.

Password-only VPN authentication is the commonest initial access route into organisations
with a VPN *(§61.4)* — MFA on it is the highest-value control available, and it is how a
large share of ransomware incidents begin.

"What does a connected client reach?" is the question that matters most and is answered
least *(§61.4)* — The default in most deployments is "everything", and per-group policy at
the concentrator is supported by every product and configured by few.

A VPN that connects a user to a data centre so they can reach a cloud service is doing work
for no reason *(§61.4)* — Terminate near the users or near the applications, which for a
distributed organisation argues for a cloud-delivered service.

The realistic destination is a much smaller VPN, not no VPN *(§61.4)* — Legacy
applications still need one, and site-to-site is not going away — the zero trust argument is
about user access, not about connecting networks, and conflating them is a common error in
vendor material. A reduction in blast radius is achievable in a way that "remove the VPN" is
not.
