# Chapter 33 — NAT and PAT

RFC 1631, published in May 1994 by Kjeld Egevang and Paul Francis, is titled *The
IP Network Address Translator*. It is ten pages long, and its second section is
headed, with a candour that has not aged badly, "Motivation."

The motivation was that IPv4 addresses were running out and IPv6 was not ready. The
authors were explicit that what they were proposing was a stopgap — the RFC calls
it a "short-term solution" and devotes a full section to its drawbacks, including
the observation that it breaks the end-to-end model and that applications carrying
addresses in their payloads will fail.

Thirty-two years later it is in essentially every home, every mobile network, and
most enterprises on Earth.

There is a lesson in that, and it is not that the authors were wrong. It is that
**a workaround that removes the pain of a problem removes the pressure to solve it**,
and that temporary solutions have a way of becoming permanent infrastructure. NAT
bought IPv4 an extra two decades, which was genuinely valuable, and it did so by
making the exhaustion crisis survivable — which is precisely why Chapter 28's
transition has taken thirty years.

## The mechanism

A router with one public address and a private network behind it rewrites the source
address of outbound packets to its own, remembers what it did, and reverses the
translation on the return traffic.

The subtlety, and the reason plain address translation is not enough: if two internal
hosts both contact the same server, the router has two conversations sharing one
public address and must distinguish the replies. Address alone cannot do it.

So the translation includes the **port number** as well:

| Inside address:port | Outside address:port | Destination |
|---|---|---|
| 192.168.10.70:51234 | 203.0.113.5:51234 | 198.51.100.10:443 |
| 192.168.10.71:51234 | 203.0.113.5:51235 | 198.51.100.10:443 |

Two hosts that happened to choose the same source port are given different
translated ports, and the return traffic is unambiguous. This is **Port Address
Translation**, also called **NAT overload** or, in Linux, **masquerading**, and it is
what essentially everyone means by "NAT" in practice.

The capacity is large: 65,535 ports minus the reserved range, per public address,
which is why a single address can serve thousands of hosts. It is not unlimited, and
§33.4 covers what happens at carrier scale when it is not.

Note what has just happened architecturally. **The router is now stateful.** It must
remember every active translation, and if it forgets — a reboot, a failover to a
device without synchronised state — every connection through it breaks
simultaneously. Chapter 24 §24.1 argued that statelessness was what let routers
scale and survive; NAT gives that up. Chapter 60's stateful firewall makes the same
trade for the same kind of reason.

## What NAT breaks, and why it matters

§33.3 is the substantial section, because the breakages are not edge cases; they
shaped two decades of application design.

**Inbound connections are impossible without configuration.** There is no
translation entry until an internal host creates one, so an unsolicited packet from
outside has nowhere to go. This is why port forwarding exists, and it is the single
biggest change NAT made to the Internet: it converted a network of peers into a
network of clients and servers, in which ordinary users can consume services but
cannot offer them. That is an architectural change of the first magnitude, achieved
as a side effect of an addressing workaround.

**Protocols carrying addresses in their payload break.** FTP's `PORT` command
contains an IP address in ASCII. SIP carries addresses in its headers and in the SDP
body. IPsec's AH authenticates the IP header, which NAT modifies, so AH cannot
traverse NAT at all. The responses — application-layer gateways that inspect and
rewrite payloads, and NAT traversal protocols such as STUN, TURN and ICE — are a
large body of engineering that exists solely to work around NAT, and §33.3 covers
them.

**Peer-to-peer requires hole punching.** Two hosts both behind NAT cannot connect
directly without a rendezvous server and a set of tricks whose reliability depends on
the specific behaviour of each NAT device — behaviour which RFC 4787 had to
*classify* because implementations varied so much. Every video-calling application
you have used contains this machinery.

**Logging and attribution become hard.** Hundreds of subscribers behind one public
address means an abuse report identifying an address identifies a neighbourhood
rather than a person, and correlating requires port-level logs with accurate
timestamps. This has real consequences for law enforcement and for abuse handling.

**Troubleshooting is harder.** Captures on either side of the NAT show different
addresses for the same conversation, and correlating them requires the translation
table.

## The honest ledger

§33.1 also credits what NAT genuinely provides, because the usual claim is
overstated in both directions.

It does provide a degree of protection: unsolicited inbound traffic has no
translation entry and is dropped. This is real and it has prevented a great deal of
opportunistic attack against home networks.

It is **not a firewall**, and treating it as one is a mistake. It applies no policy,
inspects nothing, and does not restrict outbound traffic at all. An internal host
that initiates a connection to an attacker creates a translation entry and the
attacker has a bidirectional channel. Most modern malware works precisely this way.
"We have NAT so we are safe" is a sentence to be argued with.

## Carrier-grade NAT and the endgame

When a provider runs out of public addresses for its own subscribers, it NATs its
subscribers behind a shared pool — **CGNAT**, using the `100.64.0.0/10` space from
Chapter 27. The subscriber is now behind two layers of translation, cannot port
forward at all, and shares a public address with hundreds of others. Gaming, VPNs,
and anything requiring inbound connections degrade accordingly.

CGNAT is the point at which NAT's costs become visible to end users rather than only
to engineers, and it is one of the more effective arguments for IPv6 deployment — a
network that is IPv6-native needs no translation, and increasingly mobile operators
run IPv6-only cores with NAT64 only for legacy destinations.

## By the end you will be able to

- Explain static NAT, dynamic NAT and PAT, and construct a translation table for a
  described exchange.
- Explain why port translation is necessary and compute the capacity of a single
  public address.
- Enumerate what NAT breaks and name the workaround for each.
- Explain why NAT is not a firewall, with a concrete counterexample.
- Explain CGNAT, identify its address range, and describe its user-visible effects.
- Argue the connection between NAT's success and IPv6's slow deployment.
