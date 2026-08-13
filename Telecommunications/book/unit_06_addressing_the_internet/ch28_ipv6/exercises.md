# Chapter 28 — Exercises

## A. Recall

**A1.** Give 2³² and 2¹²⁸ to three significant figures.

**A2.** When did IANA's IPv4 pool exhaust, and which regional registry exhausted first?

**A3.** State the three IPv6 notation rules.

**A4.** Give the prefix for each: global unicast, link-local, unique local, multicast,
loopback, unspecified.

**A5.** What is a /64 always, in IPv6, and why?

**A6.** What do the M, O and A flags in a Router Advertisement mean?

**A7.** What does DHCPv6 never provide, and where does it come from instead?

**A8.** What is Happy Eyeballs and what problem does it solve?

## B. Apply

**B1.** Compress fully, in canonical form:

(a) `2001:0db8:0000:0000:0000:0000:0000:0001`
(b) `fe80:0000:0000:0000:0202:b3ff:fe1e:8329`
(c) `2001:0db8:0000:0042:0000:8a2e:0370:7334`
(d) `0000:0000:0000:0000:0000:0000:0000:0000`
(e) `ff02:0000:0000:0000:0000:0000:0001:0002`
(f) `2001:0db8:0001:0000:0000:0001:0000:0001`

**B2.** Expand fully:

(a) `2001:db8::1`   (b) `::1`   (c) `fe80::1%eth0`   (d) `2001:db8:0:1::`
(e) `ff02::1:ff00:42`

**B3.** For each address give the type and whether it is routable on the Internet:

(a) `2001:db8::1`   (b) `fe80::1`   (c) `fd12:3456:789a::1`   (d) `ff02::1`
(e) `::1`   (f) `3fff::1`   (g) `2001:4860:4860::8888`   (h) `::ffff:192.0.2.1`

**B4.** Compute the EUI-64 interface ID and the resulting link-local address for MAC
`00:1b:21:3c:4d:5e`. Show every step.

**B5.** Repeat B4 for MAC `a4:5e:60:c1:02:9f`.

**B6.** Compute the solicited-node multicast address for `2001:db8:1:1::abcd:1234`, and
the destination MAC of the frame carrying a solicitation to it.

**B7.** An organisation receives `2001:db8:a000::/48`. How many /64 subnets does it have?
Give the first four and the last.

**B8.** For each M/O/A combination, state what a host does:
(0,0,1), (0,1,1), (1,0,0), (0,0,0).

## C. Analyse

**C1.** IPv6 was specified in 1998 and is at roughly half of traffic in 2026. Give the
four reasons from §28.1 and rank them by how much each contributed.

**C2.** "There is no benefit to being early." Explain this precisely, and contrast with a
protocol where the first mover does benefit.

**C3.** Explain why IPv6 was not made backward compatible, and assess whether the
decision was right given what we now know.

**C4.** Explain why every IPv6 subnet is a /64, and what breaks if you use a /120 to
"save space".

**C5.** EUI-64 addresses are a privacy problem. Explain the specific capability they give
an observer that IPv4 never did, and describe the two mechanisms that fix it.

**C6.** Explain why DHCPv6 does not provide a default gateway, and what this means for a
network that deploys stateful DHCPv6 without RAs.

**C7.** Android does not implement DHCPv6. Explain the operational consequence, and
assess whether Google's position is defensible.

**C8.** Happy Eyeballs is described as the mechanism that unblocked IPv6 adoption. Defend
this claim.

**C9.** 6to4 was automatic, required no configuration, and was deprecated. Explain why,
and state the general principle.

**C10.** IPv6 wins where one organisation controls both ends and lags where many must
coordinate. Support this with three examples and connect it to §28.1's incentive
argument.

## D. Design

**D1.** You have `2001:db8:abc0::/44` for an organisation with 8 sites. Design the
allocation: what does each site get, what does each site's internal structure look like,
and how much room remains?

**D2.** Design the IPv6 deployment model for an enterprise with Windows, Linux, macOS and
Android devices. Specify SLAAC/DHCPv6 choices per segment and justify each against a
device type.

**D3.** Write the IPv6 security policy for an organisation that has not deployed IPv6 and
does not intend to. It must address every source of unintended IPv6 in §28.4.

**D4.** For the semester project's network, add a complete IPv6 plan alongside the IPv4
one: prefixes, subnet structure, address assignment method per segment, and the firewall
approach.

**D5.** Design the migration from dual-stack to IPv6-only for a corporate network. State
what must be true before you start, what breaks, and how you would sequence it.

## E. Troubleshoot

**E1.** A host has only `fe80::...` and no global address. Give the diagnostic command
and the three most likely causes.

**E2.** A host has a global address and no default route. What was in the RA?

**E3.** A host has an address and a route but cannot resolve names. Give two causes and
the fix for each.

**E4.** Windows and Linux devices get IPv6 correctly; Android phones get nothing.
Diagnose.

**E5.** IPv6 worked and stopped after a firewall change. IPv4 is unaffected. What was
changed and what is the correct policy?

**E6.** A firewall ACL permitting `2001:db8:1:1:a1b2:c3d4:e5f6:7890` stopped matching
after two days. Explain and give the correct rule.

**E7.** A dual-stack site is intermittently slow for some users by exactly 250 ms.
Explain.

**E8.** A DHCPv6 reservation stopped working after a machine was reimaged. Explain.

**E9.** An IPv6-only client can reach `www.example.com` but not `http://198.51.100.5/`.
Explain and name the mechanism that would fix it.

## F. Extend

**F1.** Obtain IPv6 connectivity if you do not have it — natively, or via a tunnel
broker — and verify with `test-ipv6.com`. Document what you had to do.

**F2.** Capture the full SLAAC sequence on an interface coming up: RS, RA, DAD, and the
resulting addresses. Identify every message against §28.3's seven steps.

**F3.** Run `rdisc6` on a network with IPv6 and interpret every field, including the M
and O flags. Predict what a host will do, then verify.

**F4.** Compare `ip -6 addr` on a host before and after a day. Identify which addresses
rotated and which did not, and explain why.

**F5.** Check the current Google IPv6 adoption statistics and the per-country breakdown.
Explain the three highest and three lowest countries in terms of §28.1's drivers.

**F6.** Audit a network you have access to for unintended IPv6: link-local reachability
between hosts, any RAs present, any tunnels active, and whether the firewall has IPv6
rules.
