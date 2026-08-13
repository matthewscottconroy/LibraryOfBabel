# Chapter 27 — Exercises

## A. Recall

**A1.** Give the three RFC 1918 ranges with prefixes and address counts.

**A2.** What is `100.64.0.0/10` for, and what does it mean if your WAN interface has an
address in it?

**A3.** Give the three documentation ranges from RFC 5737.

**A4.** What does an address in `169.254.0.0/16` tell you?

**A5.** Define unicast, broadcast, multicast and anycast in one sentence each.

**A6.** What is the multicast address range, and what is `224.0.0.0/24` reserved for?

**A7.** What special addressing does anycast require?

**A8.** List six fields an address allocation record must contain.

## B. Apply

**B1.** For each address, state whether it is private, public, or special-purpose, and if
special, which kind:

(a) `10.255.255.254`   (b) `172.15.0.1`   (c) `172.20.0.1`   (d) `172.32.0.1`
(e) `192.168.255.1`    (f) `169.254.9.9`  (g) `100.65.0.1`   (h) `100.128.0.1`
(i) `127.5.5.5`        (j) `203.0.113.7`  (k) `224.0.0.5`    (l) `240.1.1.1`

**B2.** A host has `169.254.88.12/16`, no gateway and no DNS. List, in the order you
would check them, the six most likely causes.

**B3.** Give the multicast MAC address for IP multicast group `239.1.2.3`. Then give
another IP group that maps to the same MAC and explain why.

**B4.** For each service, state which delivery model is appropriate and why:

(a) a DNS query   (b) OSPF hellos   (c) a video call between two people
(d) IPTV to 400 televisions   (e) DHCP discovery   (f) a public DNS resolver
(g) a file download   (h) service discovery on a home network

**B5.** Write the complete allocation record for a new subnet: 60 servers at the
Manchester site, VLAN 220, from `10.4.0.0/16`. Use every field from §27.4.

**B6.** Design the intra-subnet layout for a /24 serving 120 workstations, 8 printers,
4 access points and a gateway. Give the ranges and justify the reserved space.

## C. Analyse

**C1.** Explain why RFC 1918 plus NAT deferred IPv4 exhaustion by roughly fifteen years,
and argue whether that was good or bad for the Internet.

**C2.** `240.0.0.0/4` is 268 million unused addresses that cannot be released. Explain
why, and connect it to Chapter 23 §23.4's argument about the narrow waist.

**C3.** Explain why `192.168.1.0/24` is a poor choice for a corporate network, giving the
specific failure a remote worker would experience.

**C4.** Compare multicast and CDNs as solutions to one-to-many distribution. Explain why
CDNs won, and state the general principle.

**C5.** Anycast requires no new protocol, no new address range and no new configuration
keyword. Explain how it works, and connect its success to multicast's failure.

**C6.** Explain precisely why anycast suits DNS over UDP and not long TCP connections.
Then explain how Cloudflare runs anycast TCP at scale anyway.

**C7.** BCP 38 is twenty-five years old, universally recommended and incompletely
deployed. Explain the incentive structure that produces this, and name another mechanism
in this book with the same problem.

**C8.** "Undocumented free space is unusable space." Defend this, then explain what a
ping scan does and does not tell you about what is free.

## D. Design

**D1.** An organisation is merging with another. Both use `10.1.0.0/16` for their
headquarters. Enumerate the options, with cost and risk for each, and make a
recommendation.

**D2.** Design the complete address plan for a company with 4 sites, expecting to grow to
15. Choose a range and justify the choice, define the hierarchy, specify conventions for
gateways, loopbacks and point-to-point links, and show one site in full.

**D3.** For the semester project's network, produce the full IPAM record set: every
subnet with every field from §27.4, including reserved ranges and notes.

**D4.** Write the address plan conventions document for an organisation. It must be short
enough that people read it and specific enough that two engineers would produce the same
allocation from it.

**D5.** An organisation must renumber a site of 400 hosts from `192.168.0.0/16` to
`10.20.0.0/16`. Write the migration plan, staged, with rollback at every step.

## E. Troubleshoot

**E1.** Every host on one floor has a `169.254.x.x` address. Hosts on other floors are
fine. Give the two most likely causes and the command that distinguishes them.

**E2.** A single host has `169.254.x.x` while its neighbours on the same switch are fine.
Different cause set — give three.

**E3.** A user reports that a service on their own machine is "connection refused" even
though it is running and listening. `ss -tlnp` shows `127.0.0.1:8080`. Explain.

**E4.** A remote worker connects by VPN and can reach nothing at head office. Their home
network is `192.168.1.0/24`. Head office uses `192.168.1.0/24`. Explain exactly what
happens.

**E5.** Multicast video works on the VLAN where the source is and nowhere else. Give the
cause.

**E6.** Multicast video works everywhere, and every switch port shows heavy traffic
including ports with no subscribers. Give the cause.

**E7.** A user behind a residential connection cannot host a game server despite
configuring port forwarding correctly. Their router's WAN address is `100.82.14.9`.
Explain.

**E8.** After reassigning a decommissioned subnet to a new project, unrelated systems
begin failing intermittently. Explain and give the policy that prevents it.

## F. Extend

**F1.** Inventory your own network or lab: list every subnet, its purpose, and whether it
is documented anywhere. Compare the three lists of §27.4's audit.

**F2.** Install NetBox and model a small network in it — sites, VLANs, prefixes, devices.
Then use its API to answer "what is the next free /26 at site A?"

**F3.** Use `dig +nsid` or `dig CHAOS TXT id.server` against `8.8.8.8` and `1.1.1.1` from
two different networks. Explain the different answers in terms of anycast.

**F4.** Capture multicast traffic on a lab segment with and without IGMP snooping
enabled, and quantify the difference.

**F5.** Find the current status of the `240.0.0.0/4` reclamation proposals and summarise
the arguments on both sides.
