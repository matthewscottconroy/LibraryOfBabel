# Chapter 27 — Important Concepts

The three private ranges *(§27.1)* — **`10.0.0.0/8`** (16.7M), **`172.16.0.0/12`**
(1M), **`192.168.0.0/16`** (65K). Memorise all three. **`172.32.5.1` is public**, and it
is the standard exam distractor.

**What "private" means** *(§27.1)* — Not routed on the public Internet, by universal
provider filtering. They are **not** secure, **not** unique, **not** hidden and **not**
unreachable. What they are is free, plentiful and locally meaningful.

**Why they exist** *(§27.1)* — Most connected hosts never need to be reached from the
Internet. Giving each a globally unique address wastes one, at the scale of billions.
RFC 1918 plus NAT bought roughly fifteen years — exhaustion arrived at IANA in 2011
rather than the late 1990s — and that deferral is the largest single reason IPv6's
transition has been so slow.

Which range to choose *(§27.1)* — **`10/8` by default**: room for generous
hierarchical structure. `172.16/12` if you expect partner VPNs, because its obscurity
means fewer collisions. Avoid `192.168/16` in enterprises, because every home router
uses it and a VPN user's local subnet always wins.

Randomise the second octet *(§27.1)* — `10.147.0.0/16` is as valid as `10.1.0.0/16`
and far less likely to collide in a merger or partner VPN. Costs nothing.

`100.64.0.0/10` is carrier-grade NAT *(§27.1)* — Neither public nor RFC 1918,
existing so the provider's private range cannot collide with the customer's. A WAN
interface with a `100.64.x.x` address means you have no public address: no inbound
connections, no port forwarding.

**Documentation ranges** *(§27.1)* — `192.0.2.0/24`, `198.51.100.0/24`,
`203.0.113.0/24`. Guaranteed never assigned, so an example cannot reference a real
network. Use them in your own diagrams.

`240.0.0.0/4` cannot be reclaimed *(§27.1)* — 268 million unused addresses, blocked
because essentially every IP stack hard-codes a check rejecting them. A change
requiring no protocol modification at all is still infeasible — the sharpest available
illustration that IP cannot be changed.

`169.254.0.0/16` means DHCP failed *(§27.2)* — The most useful single diagnostic in
networking. Not "slow", not "wrong" — no reply arrived. Causes in order: wrong
VLAN on the port (most common in enterprises), DHCP server down or out of leases, relay
not configured, cable/link, missing PortFast, rogue DHCP.

What APIPA can and cannot do *(§27.2)* — Reaches other link-local hosts on the same
segment; has no gateway and no DNS, so nothing off-segment works. Symptom: *"I can
see the other computer but not the internet."*

IPv4 and IPv6 link-local are opposite signals *(§27.2)* — In IPv4, a link-local
address means **something went wrong**. In IPv6, every interface always has one and it
means the interface is up. Confusing them produces false alarms during IPv6
deployments.

`127.0.0.0/8` is all loopback *(§27.2)* — The whole /8. Traffic never leaves the
host; the interface is always up; a `127.x.x.x` source arriving on a real interface is
**martian by definition**. `localhost` resolves to both `127.0.0.1` and `::1`, and a
service listening only on the IPv4 one is unreachable from a client that tries `::1`
first — "connection refused to my own machine".

`0.0.0.0` has four meanings *(§27.2)* — A source, meaning "no address yet"; a
destination prefix, meaning the **default route**; a **bind address**, meaning "all
interfaces"; and a next hop meaning "directly connected". Bound to `0.0.0.0` versus
`127.0.0.1` is a security control, and `ss -tlnp` is how you check it.

Directed vs limited broadcast *(§27.2)* — Directed (`192.168.10.255`) was routable
and enabled the **Smurf** amplification attack; RFC 2644 made not forwarding it the
required default. Limited (`255.255.255.255`) is never forwarded by any router.

Martians and BCP 38 *(§27.2)* — A network should not emit packets with source
addresses it does not own. Twenty-five years old, universally recommended,
incompletely deployed — which is why spoofing and amplification attacks still work. A
measure whose cost falls on one party and whose benefit falls on everyone else.

The four delivery models *(§27.3)* — **Unicast** one-to-one; **broadcast**
one-to-all-on-segment; **multicast** one-to-subscribers; **anycast** **one-to-nearest**.

Broadcast is always a bootstrap mechanism *(§27.3)* — ARP, DHCP, Wake-on-LAN: in
every case something must be found before enough is known to address it directly. Never
crosses a router, interrupts every host, scales badly, and IPv6 removed it entirely
in favour of multicast.

**Multicast addressing** *(§27.3)* — `224.0.0.0/4`. `224.0.0.0/24` is link-local
control and never forwarded — OSPF at `.5`/`.6`, VRRP at `.18`, mDNS at `.251`.
`239.0.0.0/8` is administratively scoped, the multicast equivalent of RFC 1918.

The 23-bit MAC mapping *(§27.3)* — `01:00:5e` plus the **low 23 bits**, so 32
multicast IP groups share one MAC address. Occasionally a host receives frames for a
group it did not join.

IGMP snooping is not optional *(§27.3)* — Without it a switch floods multicast to
every port. A multicast deployment without IGMP snooping is a broadcast deployment with
extra steps.

Multicast failed on the Internet *(§27.3)* — No business model, hard inter-domain
operation, no congestion control, no security model — and CDNs solved the problem by
placing unicast copies near users, which requires no cooperation from anyone else's
network. A solution requiring every network to change loses to one requiring only
your own.

**Anycast** *(§27.3)* — Advertise the same prefix from many locations and let routing
do the rest. No protocol, no address range, no keyword — it is a consequence of how
routing already works. The thirteen DNS root addresses are served by 1,900+ physical
instances.

**What anycast buys** *(§27.3)* — Latency without client configuration; load
distribution by topology; DDoS resilience — its most valuable property, since an
attack is absorbed near the attacker; and failure handling with no health-check system,
because withdrawing the route is the failover mechanism.

**Anycast's catch** *(§27.3)* — Ideal for stateless short exchanges, risky for
long TCP connections, because a routing change mid-connection reaches an instance with
no state and produces a reset. This is why DNS is the canonical application.

The mechanism that required nothing new is the one that deployed *(§27.3)* —
Multicast has its own addresses, protocols and switch features and does not work across
the Internet; anycast has none and does.

Documentation is the plan *(§27.4)* — Without it, "what is free?" is unanswerable. A
ping scan finds live hosts, not allocations, so an idle-but-allocated range looks
free. The failure is an overlap, which produces intermittent connectivity where nothing
is broken and two things are both correct and incompatible.

**The record fields** *(§27.4)* — Prefix, purpose, site, VLAN, gateway, DHCP range,
static range, **reserved**, owner, date, **status**. Deprecated ranges must lie fallow
for six months, because old configurations, firewall rules and documentation still
reference them.

Conventions matter more than which convention *(§27.4)* — Gateway at `.1` or `.254`,
consistently. A layout with an explicit **reserved block**, so the DHCP pool never grows
into the static range. Point-to-point links from one dedicated block so they
aggregate. **Encode the structure** so `10.3.21.45` is readable without documentation.

Every router needs a loopback /32 *(§27.4)* — Always up, independent of any
physical interface, so it is the correct source for management, logging, SNMP, routing
IDs and BGP peering. It does not go down when one link does.

**IPAM tools** *(§27.4)* — NetBox is the one to know — the de-facto standard source of
truth for network automation. What a tool gives over a spreadsheet: **overlap detection**,
next-free-subnet, an **API**, audit history, DNS/DHCP integration, and discovery. A
spreadsheet in version control recovers most of the benefit for free.

**DDI** *(§27.4)* — DNS, DHCP and IPAM are the same data, and maintaining them separately
guarantees they will disagree. **They do**, and stale DNS pointing at reassigned
addresses causes both outages and security findings.

**Renumbering is expensive** *(§27.4)* — Static hosts, reservations, DNS, firewall rules,
ACLs, application configuration, monitoring, documentation — and partner allow-lists,
which you do not control. The mitigation is to make it unnecessary: allocate
generously, plan hierarchically, reserve space, choose unlikely ranges. All free at design
time.

**The audit** *(§27.4)* — Compare documented, configured and live. The three lists
will not agree, and every disagreement is an undocumented allocation, an abandoned one, or
a mistake.
