# Chapter 26 — Exercises

*This is the heaviest exercise set in the book. Subnetting is a motor skill; there is no
substitute for repetition. Use
[tools/subnet_practice.py](../../../tools/subnet_practice.py) to generate more.*

## A. Recall

**A1.** State the two formulas: number of subnets, and usable hosts per subnet.

**A2.** How is block size computed from a mask octet?

**A3.** Where do subnets begin, always?

**A4.** State the two conditions a set of networks must satisfy to aggregate exactly.

**A5.** What does VLSM require of the routing protocol, and why?

**A6.** In what order must VLSM allocations be made, and why?

**A7.** What is `0.0.0.0/0`, and why does it never beat another matching route?

## B. Apply — the core drill

**B1.** For each, give network, broadcast, first usable, last usable and host count.
**Target: under 20 seconds each.**

(a) `192.168.1.100/26`   (b) `10.0.0.200/25`   (c) `172.16.4.50/28`
(d) `192.168.100.10/29`   (e) `10.1.1.1/30`   (f) `203.0.113.200/27`
(g) `192.168.20.130/25`   (h) `172.20.75.90/22`   (i) `10.50.100.7/21`
(j) `192.168.15.55/19`   (k) `172.16.155.100/20`   (l) `10.200.50.75/13`
(m) `192.168.7.200/29`   (n) `172.31.99.99/18`   (o) `10.10.10.10/23`

**B2.** Work `192.168.10.70/27` completely in binary, showing every step, then again by
the magic-number method. Confirm they agree.

**B3.** Divide `192.168.50.0/24` into /27 subnets. List all of them with ranges.

**B4.** Divide `172.16.0.0/16` into /20 subnets. List the first four and the last one.

**B5.** How many /26 subnets fit in a /22? How many /30s in a /24? How many /28s in a
/20?

**B6.** For each host requirement, give the smallest sufficient prefix and the wasted
capacity: 2, 6, 12, 25, 60, 120, 300, 1000, 5000.

## C. Aggregation

**C1.** Summarise each set to a single prefix, or state that it cannot be done exactly
and give the minimal covering set:

(a) `10.1.0.0/24`, `10.1.1.0/24`
(b) `192.168.8.0/24` through `192.168.15.0/24`
(c) `172.16.32.0/24` through `172.16.63.0/24`
(d) `10.5.4.0/24`, `10.5.5.0/24`, `10.5.6.0/24`, `10.5.8.0/24`
(e) `192.168.6.0/24` through `192.168.9.0/24`
(f) `10.0.0.0/16`, `10.1.0.0/16`, `10.2.0.0/16`, `10.3.0.0/16`

**C2.** Show the binary for C1(b) and identify the exact bit at which the common prefix
ends.

**C3.** A router advertises `192.168.4.0/22` but holds only `192.168.4.0/24` and
`192.168.5.0/24`. Describe what happens to traffic for `192.168.6.10`, and name the
phenomenon this would constitute on the public Internet.

**C4.** A routing table contains `10.0.0.0/8 → A`, `10.1.0.0/16 → B` and
`10.1.5.0/24 → C`. Where does each of these go: `10.9.9.9`, `10.1.9.9`, `10.1.5.9`?

## D. VLSM design

**D1.** Using `192.168.1.0/24`, allocate for: 100, 50, 25, 10, and three 2-host links.
Show the allocation table and the remaining space.

**D2.** Same requirements, but allocate **smallest first** and document exactly where it
fails.

**D3.** Using `172.16.0.0/20`, allocate for: 500 users, 200 servers, 100 voice, 60
guests, 25 management, and four point-to-point links. Verify the sizes sum correctly.

**D4.** Using `10.0.0.0/8`, design a plan for an organisation with 12 sites, each with
users, voice, servers, guests and management. Every site must advertise exactly one
prefix. Show the top level and one site in full.

**D5.** Redesign the following so that it aggregates, and state what would have to be
renumbered:

```
10.1.0.0/24   Site A floor 1        10.4.0.0/24   Site B floor 2
10.2.0.0/24   Site B floor 1        10.5.0.0/24   Site A floor 3
10.3.0.0/24   Site A floor 2        10.6.0.0/24   Site C
```

**D6.** For the semester project's network, produce the complete VLSM plan with the
documentation fields listed in §26.4, including reserved ranges.

## E. Analyse and troubleshoot

**E1.** Explain why allocating largest-first is necessary, using the alignment argument.
Give the general principle it is a case of.

**E2.** A /24 divided into 64 /30s yields how many usable addresses? What fraction of the
block is lost, and what would /31 give instead?

**E3.** Two subnets are configured as `10.1.0.0/23` and `10.1.1.0/24`. Describe the
overlap and what a host in each believes.

**E4.** An engineer configures `192.168.1.100/26` as a subnet's network address. What is
wrong, and what is the correct network address?

**E5.** A site's routing table has 340 entries where the design called for 12. Give three
plausible causes and how you would distinguish them.

**E6.** After adding a subnet on floor 4 of Building 1, routers at three other sites
recalculated. What does this tell you about the address plan?

**E7.** A summary route was configured and part of the range became unreachable. Explain
the likely cause.

**E8.** An organisation allocated subnets in the order they were requested over ten
years. Assess the cost of this and the options for fixing it.

## F. Extend

**F1.** Generate and complete five sheets from
[tools/subnet_practice.py](../../../tools/subnet_practice.py), one per topic. Record your
times and repeat weekly until every topic is under fifteen seconds per question.

**F2.** Write a program that takes a block and a list of host requirements and produces a
VLSM allocation. Compare its output with `netcalc.py vlsm` on ten cases, including one
that does not fit.

**F3.** Find the current global BGP table size (bgp.potaroo.net) and its growth rate.
Estimate the table size in ten years, and state what the estimate assumes.

**F4.** Take your own organisation's or lab's address plan and assess it against §26.4:
does it aggregate, is it documented, is there headroom, is the structure readable?

**F5.** Read RFC 1519's projections about routing table growth and compare them with what
actually happened. Which mechanism — CIDR or NAT — did more to defer the crisis?
