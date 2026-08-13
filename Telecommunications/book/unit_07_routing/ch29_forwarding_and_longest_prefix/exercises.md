# Chapter 29 — Exercises

## A. Recall

**A1.** State the forwarding decision in one sentence.

**A2.** List the eight steps a router performs on a packet.

**A3.** Name four things a router does **not** do, with a reason for each.

**A4.** Distinguish the control plane from the data plane in four respects.

**A5.** What is the difference between the RIB and the FIB, and which does hardware use?

**A6.** State the longest-prefix match rule, and say why a longer prefix is preferred.

**A7.** What are the three tie-breakers when two routes have the same prefix length?

**A8.** What does "default-free" mean, and roughly how many routes does such a router
hold?

## B. Apply

**B1.** Given this table, state the next hop for each destination and the prefix length
that decided it:

```
0.0.0.0/0        via A
172.16.0.0/12    via B
172.16.8.0/21    via C
172.16.10.0/24   via D
172.16.10.128/25 via E
172.16.10.192/26 via F
```

(a) `172.16.10.200`  (b) `172.16.10.150`  (c) `172.16.10.5`  (d) `172.16.12.9`
(e) `172.16.40.1`  (f) `192.0.2.1`  (g) `172.16.10.191`  (h) `172.31.255.254`

**B2.** Show the binary comparison that decides case (a) of B1, marking the bit at which
each candidate route stops matching.

**B3.** Read this Linux table and answer the questions below:

```
default via 10.0.0.1 dev eth0 proto dhcp metric 100
default via 10.0.5.1 dev tun0 proto static metric 50
10.0.0.0/24 dev eth0 proto kernel scope link src 10.0.0.42
10.0.5.0/24 dev tun0 proto kernel scope link src 10.0.5.7
198.51.100.0/24 via 10.0.0.1 dev eth0 proto static metric 100
```

(a) Where does traffic to `8.8.8.8` go, and why?
(b) Where does traffic to `198.51.100.20` go?
(c) What happens if `tun0` goes down?
(d) What is `src` doing on the two connected routes?

**B4.** Decode this IOS line completely: `O IA 10.4.0.0/16 [110/1180] via 10.255.0.9,
02:14:07, GigabitEthernet0/2`.

**B5.** Compute the packets per second and nanoseconds per packet for a 40 Gb/s interface
at minimum Ethernet frame size. Show the arithmetic.

**B6.** Four ECMP paths of 10 Gb/s each connect two routers. State the throughput of:
(a) one TCP connection; (b) 1,000 TCP connections between many hosts; (c) 1,000 TCP
connections between the same two hosts, hashing on IP only.

## C. Analyse

**C1.** Explain why forwarding uses only the destination address, and give three
consequences — one security, one operational, one diagnostic.

**C2.** "No router anywhere knows the path a packet will take." Explain, and contrast
with circuit switching (Chapter 12). What is gained and what is lost?

**C3.** Explain why a router's CPU can be at 100% while forwarding is unaffected, and
give two troubleshooting symptoms this explains.

**C4.** Prove that longest-prefix match is the only rule that permits "everything that
way, except this". What would break under a shortest-match or first-match rule?

**C5.** Explain the more-specific hijack completely: the mechanism, why every router
behaves correctly, why the protocol cannot detect it, and what partial defence the /24
filtering convention provides.

**C6.** ECMP and link aggregation both hash rather than round-robin. State the shared
reason, and the shared consequence.

**C7.** Explain why `0.0.0.0/0` requires no special case in the forwarding algorithm.

**C8.** ICMP redirect solves a real problem and is disabled everywhere. Give the problem,
the security objection, and what a redirect's presence tells you about the design.

**C9.** Explain why TCAM's physical properties make routing table size an Internet-wide
concern rather than a local one.

## D. Design

**D1.** A branch office has one Internet link and one MPLS link to head office. Write the
routing design: what routes exist, which is preferred for which traffic, and what happens
when each link fails.

**D2.** Design the first-hop redundancy for a site with two core switches serving twelve
VLANs. Specify what is active for what, and state the failover time and mechanism.

**D3.** For the semester project's network, produce the routing table you would expect on
each router, and verify by hand that every subnet is reachable from every other.

**D4.** An organisation wants to send traffic for one partner's subnet over a dedicated
link while everything else uses the Internet. Write the routes, and explain why
longest-prefix match makes this work without touching anything else.

## E. Troubleshoot

**E1.** `ping 8.8.8.8` returns "Network is unreachable" immediately. `ping` to a host on
the same subnet works. Diagnose in one step.

**E2.** `ping 8.8.8.8` times out with no error. Local traffic works, and the gateway
responds to ping. Give the two most likely causes.

**E3.** A host is configured `192.168.1.50/25` with gateway `192.168.1.200`. Explain what
is wrong and what the host will do.

**E4.** A laptop has intermittent connectivity: some sites load, others time out, and it
changes on reconnect. `ip route` shows two default routes with equal metrics. Explain
precisely.

**E5.** A static route to `10.5.0.0/16` was configured and does not appear in
`show ip route`. Give three causes.

**E6.** Traffic to `10.1.5.70` is taking an unexpected path. The configured static route
for `10.1.0.0/16` looks correct. What do you check, and with which command?

**E7.** A router shows the correct route in `show ip route` but traffic is not following
it. What is the next command?

**E8.** `traceroute` to a partner's network reaches your ISP's edge and stops. Traffic in
the other direction works. Diagnose.

**E9.** After a change, a router begins ARPing for large numbers of Internet addresses
and its ARP table fills. What was configured?

## F. Extend

**F1.** Read your own machine's routing table and account for **every** entry — what
created it, what it is for, and what would break without it.

**F2.** Use `ip route get` to test twenty destinations and predict each answer before
running it. Investigate any you get wrong.

**F3.** Build a three-router lab, configure overlapping routes of different prefix
lengths, and verify longest-prefix match empirically with `traceroute`.

**F4.** Research the 2008 Pakistan Telecom/YouTube incident in detail. Draw the
announcements, explain why every router behaved correctly, and state what would have
prevented it.

**F5.** Find the current global routing table size and the TCAM capacity of two current
router platforms. Calculate the headroom and comment.
