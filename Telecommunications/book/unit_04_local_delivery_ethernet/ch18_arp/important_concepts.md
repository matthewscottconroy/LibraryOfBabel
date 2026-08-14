# Chapter 18 — Important Concepts

The two address worlds *(§18.1)* — MAC addresses are **flat**, factory-assigned,
and travel with the hardware; IP addresses are **hierarchical**, administratively
assigned, and belong to a location. Neither can do the other's job, and there is no
function from one to the other.

Why both are needed *(§18.1)* — The hardware reads the MAC address, so
nothing moves on a segment without one. MAC addresses cannot be aggregated, so a
global network addressed by them would need one routing entry per device on Earth —
not expensive, arithmetically impossible.

**The scoping sentence** *(§18.1)* — MAC addresses are hop-by-hop; IP addresses are
end-to-end. A packet crossing five routers is carried by five different frames with
five different destination MAC addresses; the destination IP never changes.

**The local-or-remote decision** *(§18.1)* — Performed before ARP, for every packet,
by ANDing both addresses with the local mask. Equal → ARP for the destination. Not
equal → ARP for the gateway, but keep the destination IP unchanged.

**ARP's question** *(§18.1)* — Exactly one: *given an IPv4 address on my own link,
what MAC address holds it?* Not routing, not discovery, not naming.

**Layer 2.5** *(§18.1)* — ARP is carried directly in Ethernet (EtherType `0x0806`),
not inside IP, because it must work before IP does. It therefore fits the OSI model
badly. A student who cannot place ARP in the model has understood correctly.

**The exchange** *(§18.2)* — **Request broadcast** (the asker does not know whom to
ask), **reply unicast** (the replier does). Two frames, once per cache lifetime.

**RFC 826** *(§18.2)* — Plummer, 1982. Four pages, never revised. Deliberately
generic — HTYPE/PTYPE/HLEN/PLEN allow any hardware and any protocol — though in
practice it means IPv4 over Ethernet.

**Everyone learns** *(§18.2)* — A broadcast request teaches every station the
*asker's* mapping, not just the replier's. Efficient, and also the vulnerability.

**Never ARP off-subnet** *(§18.2)* — A host ARPs only for addresses it believes are
local. An ARP request for an off-subnet address proves a wrong subnet mask, which
makes a capture a mask diagnosis.

**The broadcast bootstrap** *(§18.2)* — ARP works before any resolution exists
because `ff:ff:ff:ff:ff:ff` needs no lookup. The same trick bootstraps DHCP.

**The n² problem** *(§18.2)* — Every request is processed by every host in the
broadcast domain, and worst-case request volume grows with the square of host count.
One of the two arguments for bounding broadcast domains.

**The cache** *(§18.3)* — Ageing is typically 30 s on Linux, 15–45 s on Windows, and
4 hours on Cisco routers. **Randomised** to prevent synchronised re-resolution
across a room of machines — the same deliberate desynchronisation used by DHCP and
routing timers.

The router/switch timer mismatch *(§18.3)* — A router's 4-hour ARP timeout
outlives a switch's 5-minute MAC ageing, so the router unicasts to an address the
switch has forgotten and the switch floods. The most common benign cause of
persistent unicast flooding.

**`STALE` and optimism** *(§18.3)* — An expired entry is used immediately and
revalidated in parallel, because the mapping is almost always still correct. Same
optimism as DNS serve-stale and HTTP stale-while-revalidate.

**`INCOMPLETE`** *(§18.3)* — Request sent, nothing answered. Not a cache fault; a
reachability fault, reported accurately.

**Gratuitous ARP** *(§18.3)* — An unprompted announcement of one's own mapping
(sender IP = target IP). Three legitimate uses: **duplicate detection**, cache
update after a MAC change, and **switch table population**. It is what makes
VRRP/HSRP failover sub-second.

ARP has no authentication *(§18.3)* — A reply is believed because it arrived.
Therefore anything on the broadcast domain can claim any address on it — which is
the concrete argument for segmentation.

**ARP spoofing** *(§18.3)* — Unsolicited replies poison both victim and gateway,
placing the attacker in the middle of both directions. Mature, packaged tooling; no
privilege required beyond presence on the segment.

Gratuitous ARP and ARP spoof are the same message *(§18.3)* — Distinguished only
by whether the claim is true. No content inspection can separate them; DAI works by
checking the claim against the DHCP snooping binding table, which is external
evidence.

DAI depends on DHCP snooping *(§18.3)* — DAI validates against the binding table
that snooping builds. Enabling DAI alone drops everything; statically-addressed
servers need explicit ARP ACLs.

Encryption does not prevent the attack *(§18.3)* — TLS reduces a total compromise
to metadata exposure. Not nothing, but far less — which is why ubiquitous encryption
devalued a whole class of local attacks.

**Proxy ARP** *(§18.3)* — A router answering for addresses that are not its own,
originally for hosts that did not understand subnetting. Hides topology, bloats
caches, enlarges failure domains. Symptom: connectivity that works but whose path
makes no sense. Turn it off except in specific VPN and wireless-controller uses.

**`arping`** *(§18.3)* — Tests reachability at Layer 2 only, bypassing IP, ICMP and
most filtering. Distinguishes *"not there"* from *"there and broken above Layer 2"*.
`arping -D` identifies duplicate addresses in seconds.

**NDP** *(§18.4)* — IPv6's replacement, RFC 4861. Carried in **ICMPv6**, so it is
properly Layer 3. NS/NA (types 135/136) replace ARP request/reply; RS/RA (133/134)
replace DHCP's role; Redirect (137) replaces ICMP redirect.

**Solicited-node multicast** *(§18.4)* — `ff02::1:ff` plus the target's low 24 bits,
mapping to a `33:33:…` MAC address that **NIC hardware filters**. An ARP request costs
every host an interrupt; a neighbour solicitation costs one host in sixteen million
segments.

**Mandatory DAD** *(§18.4)* — Every address is `tentative` until proven unique. IPv6
reports at configuration time what IPv4 lets you discover through weeks of
intermittent failures.

**Neighbour Unreachability Detection** *(§18.4)* — NDP tracks whether a neighbour is
*currently* working, accepting **upper-layer confirmation** (TCP ACKs) as evidence.
Consequence: a host detects a dead default router and switches by itself, with no
VRRP/HSRP equivalent required.

The hop limit 255 check (GTSM) *(§18.4)* — Every NDP message is sent at hop limit
255 and discarded if received lower. Since routers decrement, this proves the sender
is on-link using a field that already existed. Free, and it eliminates every off-link
attacker. It does **not** authenticate on-link attackers. The same trick protects BGP.

Router Advertisements carry more than routing *(§18.4)* — Prefix, lifetime, MTU,
M/O flags, and (RFC 8106) DNS servers. Essentially DHCP's payload, unsolicited.
**Rogue RAs** black-hole a segment; **RA Guard** is IPv6's DAI.

Blocking ICMPv6 breaks IPv6 *(§18.4)* — No NDP, no resolution, no router
discovery, no path MTU discovery. RFC 4890 lists what must pass. The most common
real-world IPv6 deployment mistake, made by applying IPv4 firewall habits.

**SEND** *(§18.4)* — Cryptographically generated addresses solve the authentication
gap properly; deployment is essentially zero. A better mechanism lost to a
good-enough one deployed at a different layer — the same story as IPsec versus TLS.
