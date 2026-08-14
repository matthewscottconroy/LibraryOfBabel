# Chapter 29 — Important Concepts

**The forwarding decision** *(§29.1)* — Given a packet, choose the interface to send it
out of and the next device to address it to. Everything else a router does exists to
support this. Repeat several billion times per second and you have the Internet.

**The eight steps** *(§29.1)* — Read the destination; find every matching route; choose
the **longest prefix**; take its interface and next hop; decrement TTL (drop and send
ICMP Time Exceeded at zero); recompute the header checksum; resolve the next hop's
link-layer address; build a new frame and transmit.

Forwarding is destination-based only *(§29.1)* — The source plays **no part**. Two
packets from different sources to the same destination take identical paths. Consequences:
**source spoofing works** (hence BCP 38); return traffic is a separate decision, hence
asymmetric routing; and policy-based routing is an explicit, off-fast-path exception.

A router knows only the next hop *(§29.1)* — No router anywhere knows the path.
The path is not planned, stored or agreed — it **emerges** from a chain of independent
local decisions. This is the trade against circuit switching: certainty given up in
exchange for instant rerouting with no signalling.

Control plane versus data plane *(§29.1)* — The control plane decides what the
table should say (software, CPU, seconds, on topology change); the data plane uses
it (hardware, nanoseconds, every packet). Consequences: a router's CPU can be at 100%
with forwarding unaffected, and ping to a router may be slow while traffic through it
is fast — control-plane response is low priority.

**RIB versus FIB** *(§29.1)* — The RIB is the control plane's full view including routes
it did not choose; the FIB is what hardware forwards on. `show ip route` shows the
RIB. When they disagree, forwarding follows the FIB. SDN (Chapter 68) is this
separation taken to its conclusion.

Every host is a router *(§29.1)* — Your laptop runs the same algorithm. Chapter 18
§18.1's local-or-remote decision is a routing table lookup, with two entries instead
of a million. The only difference is that a host does not forward packets not addressed
to it — and `ip_forward` removes even that.

**6.7 nanoseconds** *(§29.1)* — 100 Gb/s at minimum frame size is **148.8 Mpps**, leaving
about twenty instructions per packet. So forwarding is not done on a CPU. It is done
in **TCAM**, which compares against every entry simultaneously and returns the longest
match in constant time.

TCAM's properties shape Internet routing *(§29.1)* — Constant-time lookup, very
expensive, power-hungry, physically limited. Which is why aggregation matters and why a
router that exceeds its TCAM does not degrade gracefully — it falls back to software or
drops routes.

The five fields every route carries *(§29.2)* — Destination prefix, next hop,
interface, source, preference. Learn to read those in any format and platform
differences become cosmetic.

Directly connected versus via *(§29.2)* — A route **with** a next hop is indirect; a
route **without** one is directly connected, so the host ARPs for the destination
itself rather than a gateway. This is the mechanism behind the local-or-remote
decision.

Connected routes are automatic *(§29.2)* — `proto kernel` / `C`. Configuring an
address configures the route, on every platform. Nothing else works without them, and a
static route whose next hop is not on a connected subnet is invalid.

`ip route get` is the tool *(§29.2)* — It performs the real lookup — longest-prefix
match, policy rules and all — and reports the answer. It settles arguments that reading
the table by eye does not.

`[110/20]` is two different numbers *(§29.2)* — **Administrative distance** (trust in
the *source*, compared **between** protocols) then **metric** (path quality, compared
**only within** a protocol). `[90/2195456]` is not worse than `[110/20]` — 90 is
better, and the metrics are in different units and not comparable.

**`ip rule show`** *(§29.2)* — Policy rules are consulted **before** the main table and
are invisible in `ip route`. Traffic going somewhere inexplicable, with a table that says
otherwise, usually has one — VPN clients add them routinely.

**Longest-prefix match** *(§29.3)* — Several routes may match; the most specific wins.
Each longer prefix is a more informed statement, so ignoring it would discard better
information — and would make aggregation impossible, since an aggregate could never have
an exception.

The rule in one sentence *(§29.3)* — Longest-prefix match is what lets you say
"everything that way, except this." The same rule as firewall most-specific-match, CSS
specificity, and deepest-path filesystem matching.

Why masks must be contiguous *(§29.3)* — "Longest" is meaningful only if a prefix is a
leading run of bits. With a non-contiguous mask there is no length to compare.

A hole punched in an aggregate *(§29.3)* — A /26 inside a /24 captures its range;
everything outside it still follows the /24. Essentially every real routing table is
structured this way.

**The tie-breakers** *(§29.3)* — Same prefix length → **administrative distance**, then
**metric**, then **ECMP**. ECMP **hashes** rather than round-robins, for the same reason
link aggregation does: round-robin reorders within a flow and TCP reads reordering as
loss. So a single flow uses one path — four 10 Gb/s ECMP paths give one connection
10 Gb/s, not 40.

ECMP is why traceroute varies *(§29.3)* — Successive probes may take different paths,
and an intermittent problem may affect some flows and not others.

`/32` beats everything; `0.0.0.0/0` loses to everything *(§29.3)* — Host routes pull
one address down a specific path, blackhole it, or advertise anycast. The default route's
behaviour needs **no special case** — it falls out of the rule.

**More-specific hijacking** *(§29.3)* — Announce a longer prefix than the legitimate
holder and you attract their traffic, because every router applies the rule
correctly. BGP has no authentication of who owns what. Pakistan Telecom took YouTube
offline globally in 2008 this way; China Telecom carried 15% of prefixes in 2010;
MyEtherWallet users lost cryptocurrency in 2018. Providers filter announcements more
specific than /24, which is why /24 is the effective minimum announceable unit — a
convention with no protocol basis, enforced by mutual filtering. RPKI is the real fix.

**The default route** *(§29.4)* — Two entries cover four billion addresses. It matches
everything (zero network bits) and loses to everything (shortest prefix). No mechanism
of its own is required.

Recursive delegation, until it stops *(§29.4)* — Laptop → home router → ISP edge →
ISP core, and at the core the default disappears. A **default-free** router holds
~950,000 prefixes and no default; an unknown destination is **dropped**. The Internet is
a hierarchy of "ask upstream" until you reach the level where someone must actually
know.

Interface-only defaults on Ethernet are wrong *(§29.4)* — The router believes every
Internet destination is directly connected and ARPs for each one. Correct only on
genuine point-to-point links. Always give a next-hop address on a broadcast link.

How a host learns the gateway *(§29.4)* — **DHCP option 3** in IPv4; always the
Router Advertisement in IPv6, since DHCPv6 never supplies one.

**First-hop redundancy** *(§29.4)* — VRRP/HSRP/GLBP share a virtual IP and MAC;
failover works by **gratuitous ARP** within a second. IPv6 does not need it — NDP's
unreachability detection lets a host switch routers by itself, which is one of the better
answers to "what does IPv6 give me?"

**ICMP redirect** *(§29.4)* — *"Use that router instead."* Works, and is **disabled
everywhere**: a host accepting redirects accepts routing changes from anything on its
segment. If redirects are needed, the gateway assignment is wrong.

**"Network is unreachable"** *(§29.4)* — **No route matched** — the stack refused to send.
Distinct from a timeout, and local traffic still works, giving the distinctive *"I can
reach the printer and nothing else."*

Wrong gateway gives a timeout, not an error *(§29.4)* — The address exists, ARP
succeeds, Layer 2 looks fine, and packets go nowhere. The harder case.

A gateway outside the host's own subnet *(§29.4)* — The classic mask error. The host
cannot reach its own gateway.

**Two default routes** *(§29.4)* — Not an error; equal metrics give ECMP, so half of
connections fail and half succeed if one is stale. A very common laptop symptom after an
untidy VPN disconnect.
