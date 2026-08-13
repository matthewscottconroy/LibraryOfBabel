# Chapter 30 — Exercises

## A. Recall

**A1.** What three things does every static route specify, and which are mandatory?

**A2.** State the rule about the next hop, and what happens when it is violated on
Cisco IOS and on Linux.

**A3.** Give the administrative distance of: connected, static, eBGP, EIGRP, OSPF, IS-IS,
RIP, iBGP, and "unusable".

**A4.** Distinguish administrative distance from metric in one sentence each.

**A5.** What is a floating static route, and what makes it float?

**A6.** What is RIP's metric, what is its maximum, and what does the value above the
maximum mean?

**A7.** Give the OSPF cost formula and the default reference bandwidth.

## B. Apply

**B1.** Write the static route for each, on both IOS and Linux:

(a) `10.20.0.0/16` via `192.168.1.9`
(b) A default route via `203.0.113.1` with administrative distance 250
(c) A blackhole for `172.20.0.0/14`
(d) `10.30.0.0/16` out `GigabitEthernet0/2` via `10.0.0.6`, explicit in both

**B2.** For each pair, state which route is installed and why:

(a) `10.1.0.0/16 [110/20]` and `10.1.0.0/16 [120/2]`
(b) `10.1.0.0/16 [1/0]` and `10.1.0.0/16 [110/5]`
(c) `10.1.0.0/16 [110/20]` and `10.1.0.0/16 [110/30]`
(d) `10.1.0.0/16 [1/0]` and `10.1.1.0/24 [120/8]`, for destination `10.1.1.50`
(e) `10.1.0.0/16 [90/2195456]` and `10.1.0.0/16 [110/20]`

**B3.** Compute OSPF's default cost for links of 10 Mb/s, 100 Mb/s, 1 Gb/s, 10 Gb/s and
40 Gb/s. Then recompute with a reference bandwidth of 100000.

**B4.** A network has 12 routers, each with 3 attached subnets, fully meshed logically.

(a) How many static route statements are needed in total?
(b) How many changes does adding one router with 3 subnets require?
(c) At what router count does the total exceed 1,000?

**B5.** Given this configuration, state where traffic to each destination goes:

```
ip route 0.0.0.0 0.0.0.0 203.0.113.1
ip route 10.0.0.0 255.0.0.0 10.1.1.2
ip route 10.99.0.0 255.255.0.0 203.0.113.1
ip route 10.99.5.0 255.255.255.0 10.1.1.2
```

(a) `8.8.8.8`  (b) `10.4.1.1`  (c) `10.99.1.1`  (d) `10.99.5.20`  (e) `172.16.1.1`

**B6.** Write a complete floating-static configuration with IP SLA tracking for a branch
with an MPLS primary (`10.1.1.2`) and an Internet backup (`203.0.113.1`), tracking a host
at `10.255.255.1` beyond the MPLS next hop.

## C. Analyse

**C1.** Explain why static routes have administrative distance 1, and what assumption
that encodes. Give a scenario where the assumption is wrong.

**C2.** eBGP is AD 20 and iBGP is AD 200 — the same protocol at opposite ends of the
scale. Explain the reasoning.

**C3.** Show, with a diagram and numbers, why RIP's hop-count metric can choose a path
that is fifteen thousand times slower. Explain what OSPF does instead and why it is still
imperfect by default.

**C4.** Explain what goes wrong when two routers in one OSPF domain have different
reference bandwidths. Be specific about the symptom.

**C5.** A floating static with tracking detects a failure in about fifteen seconds; BFD
does it in under a second. Explain the mechanism of each and why the difference is so
large.

**C6.** Explain completely why a floating static may fail to activate when a carrier's
network fails, including why the situation is common rather than exotic.

**C7.** "The argument for dynamic routing is about failure detection far more than path
calculation." Defend this using the four-router diamond of §30.4.

**C8.** Give six costs of dynamic routing. For each, say whether it applies to a
three-router network.

**C9.** `default-information originate always` is described as converting a working
failover into a black hole. Explain the mechanism and give the one case where `always` is
correct.

## D. Design

**D1.** Design the routing for a hub-and-spoke network: one head office, eight branches,
each branch with a single MPLS link. Static or dynamic? Justify with §30.4's two tests.

**D2.** Same network, but each branch now has an Internet backup link. Redo the design
and state precisely what detects a failure and how fast.

**D3.** For the semester project's network, write every static route required, on every
device, and verify by hand that every subnet reaches every other in both directions.

**D4.** An organisation runs 30 routers with entirely static routing and wants to migrate
to OSPF without an outage. Write the migration plan, in stages, with rollback at each.

**D5.** Design the blackhole strategy for a network under periodic volumetric attack:
what is blackholed, where, triggered by what, and removed when.

## E. Troubleshoot

**E1.** A static route was configured and does not appear in `show ip route`. Give four
causes and how to distinguish them.

**E2.** A static route appears in the table, and traffic to that destination is dropped.
The next hop pings fine. Give two causes.

**E3.** Traffic reaches a branch and nothing comes back. Both routers have "the right
routes". Diagnose.

**E4.** A carrier link failed. Every interface shows `up/up`. Traffic is black-holed and
the backup did not engage. Explain, and give two fixes.

**E5.** After a reboot, a Linux router lost three of its five static routes. Explain.

**E6.** A network runs OSPF correctly and traffic to one subnet takes an unexpected path.
`show ip route` shows the route as `S`. Explain.

**E7.** Both default routes are in the table and about half of user sessions fail.
Explain precisely what is happening.

**E8.** After enabling `default-information originate always` on a branch router during a
maintenance window, the whole network lost Internet access. Explain.

**E9.** A failover test succeeded for routing and applications still failed. Give three
causes that routing verification would not have caught.

## F. Extend

**F1.** Build a four-router lab with static routing only. Break a link and document
exactly how long connectivity stays broken and what restores it. Repeat with OSPF.

**F2.** Configure a floating static with IP SLA tracking and measure the actual failover
and failback times. Then break the link in a way that does *not* drop the interface, and
measure again.

**F3.** Take a real or lab configuration with more than twenty static routes and audit
it: find routes to non-existent networks, asymmetric pairs, and unintended splits.

**F4.** Write a script that reads several router configurations and reports every subnet
that is reachable from some routers and not others.

**F5.** Argue the case for running static routing on a fifteen-router network. Then argue
against. Identify the single fact that decides it.
