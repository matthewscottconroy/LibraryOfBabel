# Chapter 28 — IPv6

On 3 February 2011, IANA allocated the last five unreserved `/8` blocks of IPv4
address space, one to each regional registry, and the global free pool was empty.
APNIC, covering the Asia-Pacific region, reached its final `/8` fifteen weeks later.
RIPE NCC exhausted in September 2012, LACNIC in 2014, ARIN in September 2015, and
AFRINIC in 2017. There has been no meaningful IPv4 free pool anywhere in the world
since 2019, and addresses now change hands on a transfer market at prices that have
ranged between roughly $20 and $60 each.

None of this was a surprise. The IETF had seen it coming in 1990, formed a working
group in 1993, and published the IPv6 specification — RFC 1883, later revised as
RFC 2460 and now RFC 8200 — in December 1995.

**A replacement protocol was standardised sixteen years before the resource it
replaced ran out, and thirty-one years later the transition is still incomplete.**
That is the most interesting fact in this chapter, and §28.4 takes it seriously
rather than treating it as an embarrassment, because the reasons are structural and
they teach something about how infrastructure actually changes.

## The arithmetic

IPv4: 32 bits, 2³² ≈ 4.3 × 10⁹ addresses. Fewer than there are people.

IPv6: 128 bits, 2¹²⁸ ≈ 3.4 × 10³⁸ addresses.

That number resists intuition, so: it is approximately 6.7 × 10¹⁷ addresses for
every square millimetre of the Earth's surface, oceans included. If you had been
assigning a billion addresses per second since the Big Bang you would have used
about 4 × 10²⁶ of them, which is roughly one part in a trillion of the space.

The extravagance is deliberate, and it is not primarily about running out. It is
about **making address space cheap enough that hierarchy can be wasteful**. A
standard end-site allocation is a `/48`, giving 65,536 subnets, and each subnet is a
`/64` with 18 quintillion addresses in it. Nobody needs 18 quintillion hosts on a
LAN. The /64 exists because SLAAC (§28.3) generates the host portion from a 64-bit
interface identifier, and because a fixed subnet size means **no subnetting
arithmetic within a site at all** — you allocate whole /64s and never compute a host
count again.

That is the real gift of IPv6 to an operator, and it is undersold: the daily
arithmetic of Chapter 26 largely disappears.

## Notation, and the rules that make it readable

An IPv6 address is 128 bits written as eight groups of four hex digits (Chapter 2
§2.3 explains why hex and not decimal), separated by colons:

```
2001:0db8:0000:0000:0000:ff00:0042:8329
```

Two compression rules, standardised in RFC 5952 so that everyone writes them the
same way:

1. **Drop leading zeros** in each group: `2001:db8:0:0:0:ff00:42:8329`
2. **Replace one run of consecutive all-zero groups with `::`** — once only, since
   two would be ambiguous: `2001:db8::ff00:42:8329`

RFC 5952 additionally requires lowercase hex, requires `::` to be used when it can
be, and requires it to compress the *longest* run. These rules exist so that text
comparison, log searching and configuration diffing work, and violating them is a
minor but real operational nuisance.

Learn to read `::1` as loopback and `fe80::` as link-local on sight; they are the
two you will meet constantly.

## The address types

IPv6 replaces IPv4's flat "an address is an address" with explicit scoping, and
this is one of its genuine improvements:

- **Global unicast (2000::/3)** — publicly routable. The equivalent of a public
  IPv4 address.
- **Link-local (fe80::/10)** — valid only on one link, never routed, and
  **automatically configured on every IPv6 interface always**. Routing protocols use
  these for neighbour relationships, which means IPv6 routing works before any
  global addressing exists.
- **Unique local (fc00::/7, in practice fd00::/8)** — the RFC 1918 analogue, for
  internal use, with a randomly generated 40-bit global ID that makes accidental
  collision on merger essentially impossible.
- **Multicast (ff00::/8)** — with scope encoded in the address itself.
- **No broadcast at all.** Deliberately removed; its functions are served by
  well-known multicast groups such as `ff02::1` (all nodes) and `ff02::2` (all
  routers).

A single interface normally holds several addresses simultaneously — at minimum a
link-local and a global — which surprises people arriving from IPv4 and is entirely
normal.

## Autoconfiguration, and the thing IPv4 never had

**SLAAC** — Stateless Address Autoconfiguration, RFC 4862 — lets a host configure
itself with no server at all. It sends a Router Solicitation to `ff02::2`; a router
replies with a Router Advertisement containing the /64 prefix; the host generates
its own interface identifier and combines the two. It then verifies uniqueness by
Duplicate Address Detection before using the address.

No DHCP server, no lease database, no scope exhaustion. For a network of sensors or
a conference Wi-Fi this is a substantial simplification.

The interface identifier was originally derived from the MAC address (modified
EUI-64), which meant a device's address contained its hardware identity and followed
it between networks — an obvious tracking problem. RFC 8981 privacy extensions now
generate random, periodically rotating identifiers instead, and this is default
behaviour on every major operating system.

**DHCPv6** still exists for networks that need centralised control, address
recording, or options that RAs cannot carry. In practice most enterprises run both,
in one of several combinations that §28.3 sets out, and the interaction between the
RA's flags and the DHCPv6 server's behaviour is a reliable source of confusion.

## Why it took thirty years

§28.4's argument, in brief, because it is the chapter's most useful lesson:

**IPv6 is not backwards compatible.** An IPv4-only host cannot talk to an IPv6-only
host, full stop. So there was never a moment when adopting IPv6 alone let you reach
anything new — you had to run both, which is more work than running one, for no
immediate benefit.

**NAT relieved the pressure.** Chapter 33's translation, combined with RFC 1918
space, meant organisations could keep growing without more public addresses. The
crisis that would have forced adoption was successfully deferred.

**The waist of the hourglass is the hardest thing to change** (Chapter 23 §23.1).
Every application and every network technology depends on it.

**The incentives are misaligned.** The party who bears the cost of deploying IPv6 is
usually not the party who benefits.

Deployment is nonetheless now substantial — Google's measurements have global IPv6
adoption above 45% of their users, with several countries above 70% — and mobile
networks are largely IPv6-native with translation at the edge. The transition is
happening; it is simply happening on infrastructure timescales rather than software
ones.

## By the end you will be able to

- Compress and expand IPv6 addresses per RFC 5952, correctly and consistently.
- Identify an address's type from its prefix on sight.
- Explain why /64 is the universal subnet size and what that eliminates.
- Trace a SLAAC exchange including Router Solicitation, Advertisement, and DAD.
- Choose between SLAAC, stateless DHCPv6 and stateful DHCPv6 for a stated
  requirement.
- Explain dual-stack, tunnelling and NAT64/DNS64, and state which is appropriate
  when.
- Give a structural explanation for the length of the transition.
