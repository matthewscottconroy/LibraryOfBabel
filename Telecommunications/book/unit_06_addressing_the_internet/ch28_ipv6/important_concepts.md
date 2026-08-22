# Chapter 28 — Important Concepts

**The IPv4 number** *(§28.1)* — 2³² = **4,294,967,296**, and about 14% is reserved
before anyone is allocated anything. Effective usable public addresses are on the order
of **3.2 billion**, against 8 billion people with several devices each.

**The dates** *(§28.1)* — IANA exhausted 3 February 2011; **APNIC first** in April
2011, RIPE 2012, LACNIC 2014, ARIN 2015, AFRINIC 2019. "Exhausted" means the free pool
reached the final /8 and severe rationing began — typically one /22 per new member, once.

**Addresses became property** *(§28.1)* — A transfer market, prices from $8 in 2011 to
**$50–60 at peak**; a /16 trades for two to three million dollars. MIT sold half of
`18.0.0.0/8` in 2017. Cost is the argument that a decade of advocacy did not
generate.

Why the response was slow — four reasons *(§28.1)*:

1. NAT worked too well. 10,000 employees need one public address. The crisis was
   deferred past the point where anyone felt urgency, and a deferred crisis generates no
   budget.
2. There is no benefit to being early. The benefit is entirely a network effect and
   the first mover captures none of it — unlike TLS, where deploying HTTPS protects
   your own users immediately.
3. It is not backward compatible. An IPv6-only host **cannot** talk to an IPv4-only
   host, so the transition requires dual-stack: twice the addresses, rules, routing,
   troubleshooting, and twice the security surface.
4. **Everything must change** — firewalls, monitoring, IPAM, logging, geolocation, and
   every application that assumed 32 bits.

What finally moved it *(§28.1)* — **Mobile carriers** running IPv6-only cores with
464XLAT; large content providers going dual-stack; **cost**; government mandates; and
hyperscalers for whom 10/8's 16.7 million addresses are **not enough**. Adoption is now
**45–50%** of Google's users, varying from over 70% to under 5% by country.

**The IPv6 number** *(§28.1)* — 2¹²⁸ ≈ 3.4 × 10³⁸, roughly 6.7 × 10²³ addresses per
square metre of the Earth's surface.

The real lesson of IPv6's size *(§28.1)* — Not "use bigger addresses" but "never
make address conservation a design constraint again." Every subnet is a /64 regardless
of host count; a site gets a /48; Chapter 26's VLSM arithmetic simply does not exist,
and that is a feature.

**The notation** *(§28.2)* — Eight groups of 16 bits in hexadecimal. **Rule 1:** drop
**leading** zeros per group. **Rule 2:** replace **one** run of zero groups with `::`.
**Rule 3:** canonical form — lowercase, compress the **longest** run, never `::` for a
single zero group.

**`::` only once** *(§28.2)* — Two would be unparseable: there would be no way to
distribute the missing groups between the gaps.

Non-canonical forms break string matching *(§28.2)* — `2001:DB8::1` and
`2001:db8:0:0:0:0:0:1` are the same address and will not match textually, which breaks log
searches and ACLs. **Follow RFC 5952.**

**Prefix notation only** *(§28.2)* — No dotted-decimal mask form exists. One notation,
no conversion table, no mask octets. The one place IPv6 is genuinely simpler.

**The recognition table** *(§28.2)* — `2` or `3` → global unicast; `fe80` → link-local;
`fd` → unique local; `ff` → multicast; `::1` → loopback. Four prefixes classify
anything you will meet.

The /64 boundary is architectural *(§28.2)* — Not conventional. SLAAC requires 64 host
bits. A /48 gives **65,536 subnets**, and the design question becomes *how many subnets*
rather than *how large*.

Link-local is normal in IPv6 *(§28.2)* — `fe80::` on every interface, always. The
opposite of IPv4, where `169.254.x.x` means failure. Confusing the two produces false
alarms during IPv6 deployments. Link-local addresses need a scope ID:
`ping6 fe80::1%eth0` — and forgetting `%interface` is the commonest command-line error.

Unique local uses random bits *(§28.2)* — `fd` + **40 random bits**. Generate them;
do not choose them. This solves Chapter 27's merger-collision problem
probabilistically — and everyone using `fd00::1` recreates the `192.168.1.0/24`
problem exactly.

IPv6 has no broadcast *(§28.2)* — Every use replaced by multicast: `ff02::1` all
nodes, `ff02::2` all routers, solicited-node for NDP. A broadcast interrupts every
host's CPU; a solicited-node multicast is filtered in NIC hardware and interrupts about
one.

**`ping6 ff02::2%eth0`** *(§28.2)* — Asks every router on the link to identify itself. No
useful IPv4 equivalent.

**SLAAC** *(§28.3)* — Generate link-local → DAD → Router Solicitation to `ff02::2` →
Router Advertisement with the /64 → form a global address → DAD → default route is the
RA's source. A host with no server, no configuration and no administrator gets a
working global address and a route.

**EUI-64** *(§28.3)* — Split the MAC, insert **`ff:fe`**, flip the 7th bit of the
first byte. `ff:fe` in the middle of an interface ID is the signature. And it is a
privacy disaster: the low 64 bits are constant everywhere the device goes, making it
trackable across networks in a way IPv4 never permitted.

**Privacy addresses** *(§28.3)* — RFC 4941: random interface IDs, rotated daily. Now
the default everywhere, so a host has several global addresses at once — temporary for
outbound, stable for inbound. RFC 7217's stable-privacy is stable per network and
different on each. Consequence: filter on the /64, not the address.

The RA flag matrix *(§28.3)* — **M** = use DHCPv6 for addresses; **O** = use DHCPv6
for other information; **A** = autoconfigure from this prefix. (0,0,1) pure SLAAC;
(0,1,1) SLAAC + stateless DHCPv6, the common enterprise choice; (1,0,0) stateful
DHCPv6.

**The RDNSS gap** *(§28.3)* — For years SLAAC could not convey **DNS servers**, so DHCPv6
was required even when SLAAC handled addressing. RFC 8106 closed it.

Android does not implement DHCPv6 *(§28.3)* — Deliberately. A stateful-DHCPv6
network works for Windows and Linux and silently fails for Android. If phones cannot
get IPv6, this is why.

**DHCPv6 details** *(§28.3)* — Ports **546/547**; multicast `ff02::1:2`;
SOLICIT/ADVERTISE/REQUEST/REPLY; identity by **DUID** not MAC — which you cannot read
off a label, and which may change on reimage. A genuine operational regression.

DHCPv6 never provides a default gateway *(§28.3)* — Deliberate: the router is the
authority on whether it is a router. So RAs are always required, even in fully
stateful deployments.

**Prefix delegation** *(§28.3)* — The router requests a **prefix**, not an address. A home
gets a /56 and subnets it, so every device has a globally routable address and there is
no NAT anywhere. This is what IPv6 is actually for, and the firewall — default
outbound-permit, inbound-deny — provides what NAT provided accidentally.

**Dual-stack** *(§28.4)* — Both protocols everywhere; IPv6 preferred by RFC 6724. It
does not solve exhaustion, because you still need IPv4 addresses — which is why it is a
transition strategy and not a destination, and why mobile carriers skipped it.

Happy Eyeballs (RFC 8305) *(§28.4)* — Start IPv6, start IPv4 **250 ms later**, use
whichever completes first. This is why broken IPv6 stopped being catastrophic — a
black-holed AAAA went from total failure to a 250 ms delay — and it is arguably the
mechanism that made IPv6 safe to attempt.

**NAT64 / DNS64** *(§28.4)* — DNS64 **synthesises** a AAAA by embedding the IPv4 address
in `64:ff9b::/96`; NAT64 translates at the gateway. Limitation: an application using an
IPv4 literal bypasses DNS entirely and fails.

**464XLAT** *(§28.4)* — **CLAT** on the phone presents a fake IPv4 interface to
applications; **PLAT** in the carrier network is a NAT64. The carrier operates only
IPv6; applications see IPv4 and work unmodified, literals included. The largest IPv6
deployment in the world, and most of its users have never heard of IPv6 — the correct
outcome for infrastructure.

Apple's App Store IPv6 requirement (June 2016) *(§28.4)* — One policy decision that
did more for IPv6 application readiness than a decade of advocacy.

**6to4's failure** *(§28.4)* — Automatic, zero-configuration, and **deprecated** (RFC
7526) because it relied on **anonymous public relays** with no accountability and
asymmetric paths. A mechanism with no accountable operator has no one to fix it.

There is no NAT66 *(§28.4)* — Deliberately. The two reasons for IPv4 NAT were address
conservation (irrelevant) and accidental security (never NAT's job). Do not deploy IPv6
with NAT out of habit.

**The security gap** *(§28.4)* — Every modern OS has IPv6 enabled and prefers it.
Unintended IPv6 arrives via **rogue RAs**, automatic tunnels, link-local — which is
always present and cannot be turned off — and providers enabling it by default. Three
positions: deploy it properly, block it deliberately, or ignore it and have an
unmonitored unfiltered path. Most organisations are in the third.

IPv6 wins where one party controls both ends *(§28.4)* — Mobile carriers,
hyperscaler fabrics, content-provider edges. It lags where many parties must
coordinate, which is the enterprise LAN. AWS charging ~\$43/year per public IPv4
address from February 2024 moved more workloads in a year than the preceding decade of
advocacy.
