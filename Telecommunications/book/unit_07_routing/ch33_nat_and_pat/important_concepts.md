# Chapter 33 — Important Concepts

RFC 1631, May 1994 *(§33.1)* — Nine pages proposing a **short-term measure**,
cataloguing its architectural damage candidly, and recommending it anyway. Thirty-two
years later it is on every network on Earth.

**The observation** *(§33.1)* — Most connected hosts never need to be reachable from the
Internet. Giving each a globally unique address wastes one, at the scale of billions.

**The saving** *(§33.1)* — 10,000 employees need **one** public address. NAT plus CIDR
bought roughly fifteen years — exhaustion arrived at IANA in February 2011 rather than
the late 1990s.

**The four terms** *(§33.1)* — **Inside/outside** says whose address it is;
**local/global** says which side of the NAT you are standing on. **Inside local** is the
private address; **inside global** is how it appears outside. Those two are the
translation.

NAT is not a firewall *(§33.1)* — The protection it provides is an absence of
information: an inbound packet with no matching entry has nowhere to go. Real, and a
side effect — like the difficulty of capturing on a switched network. Side-effect
security is fragile.

What NAT does not stop *(§33.1)* — Malware phoning home, a user visiting a malicious
site, a compromised internal host, data exfiltration, anything that establishes an
outbound connection. Nearly every modern attack begins with one.

The Internet became client-server *(§33.1)* — The largest consequence, and it happened
gradually. Peer-to-peer applications did not fail because peer-to-peer is a bad idea;
they failed because most hosts stopped being addressable.

Stateful, so it is a single point of failure *(§33.1)* — Fate-sharing violated
(Chapter 23 §23.1). If the NAT device restarts, every connection through it dies.

**The honest verdict** *(§33.1)* — The correct engineering decision under the
constraints of 1994, made by people who understood the trade and wrote it down. The
mistake was not deploying it — it was that its success removed the pressure to finish the
real fix.

**Static NAT** *(§33.2)* — One-to-one, permanent, **conserves nothing**. Its purpose is
**inbound reachability**: the mapping exists before any traffic flows. Use it for servers.

**Dynamic NAT** *(§33.2)* — A pool, first come first served. Only *n* hosts at a time; the
*n*+1th gets nothing. **Inbound impossible.** Largely historical.

**PAT / overload** *(§33.2)* — Many private addresses share ONE public address,
distinguished by port number. The mechanism that saved IPv4.

**Why it works** *(§33.2)* — A conversation is identified by the **five-tuple**, so the
router can change the source port too and use it to remember which host the conversation
belonged to. 16 bits of port gives tens of thousands of simultaneous conversations per
address.

**Port collision** *(§33.2)* — Two internal hosts choosing the same source port to the
same destination: the router rewrites the second. The host never knows.

Both checksums are recomputed *(§33.2)* — The IP header checksum because it covers the
addresses, and the TCP/UDP checksum because it covers a pseudo-header containing the IP
addresses. A Layer 3 device recomputing a Layer 4 checksum — the 1981 pseudo-header
decision making NAT's job harder in 1994.

**Capacity** *(§33.2)* — ~64,000 usable ports per public address, but hosts open many
connections at once, so a few hundred to a thousand hosts per address in practice.

**The timers matter** *(§33.2)* — TCP established **24 hours**; TCP after FIN **60 s**;
**UDP 300 s**. The 24-hour default accumulates entries from short-lived connections. The
UDP timeout is why long-lived UDP applications send otherwise-pointless keepalives — a
VoIP call on hold or an idle VPN loses its entry and dies silently.

**Port forwarding** *(§33.2)* — A pre-populated entry, created by configuration, so the
outside can initiate. One external port maps to one internal host. **UPnP/NAT-PMP** let
applications open holes themselves, including malware — disable it on anything you care
about.

**Hairpinning** *(§33.2)* — Reaching an internal server by its public address from inside.
Many cheap routers do not implement it, and the symptom is distinctive: the service
works from outside and fails from inside. Fix with split-horizon DNS.

The root cause of everything NAT breaks *(§33.3)* — A host knows its own address, and
that address is what its peers see. The original design guaranteed it; NAT removes it.

**FTP** *(§33.3)* — Active mode sends the client's private address as ASCII text inside
the data stream, and the server cannot connect back. Passive mode fixes it for clients
and not for servers. An ALG must parse the control stream, rewrite the addresses, and
fix up TCP sequence numbers when the text length changes. FTPS defeats the ALG
entirely, which is why SFTP is the right answer.

**SIP and RTP** *(§33.3)* — SDP carries the phone's private address, giving the classic
symptom: the call sets up and there is no audio, or audio one way. **STUN** asks a
server what your public address is; **TURN** relays everything; **ICE** tries every
candidate. A meaningful fraction of calls — commonly 10–20% — require relaying, a
permanent infrastructure cost imposed entirely by NAT.

IPsec AH is broken by design *(§33.3)* — It authenticates the addresses NAT changes.
**NAT-T** wraps ESP in UDP 4500.

Hole punching and symmetric NAT *(§33.3)* — Both hosts send simultaneously so each
creates an entry the other's packet can use. Symmetric NAT allocates a new external port
per destination, so the observed port is not the port the peer will reach — and CGNAT is
usually symmetric, which is why peer-to-peer works worse on mobile networks.

Every mitigation but one is a workaround *(§33.3)* — ALGs, STUN/TURN/ICE, hole
punching, NAT-T, port forwarding, translation logging. The real cost of NAT is not CPU:
it is that a generation of protocol designers had to assume the network would break their
protocol, and design around it.

**CGNAT** *(§33.3, §33.4)* — The provider translates too, so many customers share one
public address, with **`100.64.0.0/10`** in the middle because the customer is already
using RFC 1918. A `100.64.x.x` WAN address means you have no public address.

**What CGNAT costs** *(§33.4)* — **Inbound is impossible**, not merely difficult;
peer-to-peer degrades because it is symmetric; **shared reputation** means one abuser gets
the address blocked for hundreds of subscribers with no recourse; and logging requires
port ranges at enormous volume.

**The provider's arithmetic** *(§33.4)* — A million subscribers at $40 per address is
**$40 million**; CGNAT equipment is a few hundred thousand. But IPv6 costs less still,
because the IPv4 share falls every year. CGNAT is a cost that grows; IPv6 is a cost that
ends.

IPv6 has no NAT, deliberately *(§33.4)* — Address conservation is not a design
constraint. The **stateful firewall** provides the same protection as a policy decision
rather than a side effect, and it can be relaxed per host, which NAT cannot.

What you get back *(§33.4)* — Global addressability, peer-to-peer, no ALGs, no STUN,
no TURN, no hole punching, no translation state to fail or exhaust, logs that identify
hosts, and the end-to-end principle.

The most common IPv6 security mistake *(§33.4)* — Assuming NAT was providing the
protection, and not configuring the firewall. Exactly the cost of side-effect security.

**The thirty-year summary** *(§33.4)* — NAT is the most successful temporary measure in
the history of computing, and its success is precisely what made it permanent. Whether
that is a triumph of pragmatic engineering or a cautionary tale about workarounds is worth
forming a view on, and the honest answer is both.
