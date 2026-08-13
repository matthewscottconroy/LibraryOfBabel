# Chapter 25 — Exercises

## A. Recall

**A1.** How many bits is an IPv4 address, and how many addresses does that permit?

**A2.** List the nine valid mask octet values in order.

**A3.** State the formula for usable hosts, and explain the minus two.

**A4.** What operation combines an address and a mask, and what does it produce?

**A5.** Give the classful ranges for A, B, C, D and E by first octet.

**A6.** Give the three RFC 1918 private ranges with their prefix lengths.

**A7.** What does a wildcard mask of `0.0.0.63` correspond to as a prefix?

## B. Apply

**B1.** Convert to binary: 10, 172, 224, 192, 254, 100, 199, 45.

**B2.** Convert to decimal: `11010110`, `00111111`, `10000001`, `11111110`.

**B3.** Write `172.16.100.200` in full binary, and give its value as a single integer.

**B4.** For each, give the network address, broadcast address, first and last usable
address, and usable host count:

(a) `10.1.2.3/8`
(b) `172.16.50.100/16`
(c) `192.168.1.130/25`
(d) `192.168.10.70/26`
(e) `10.20.30.45/27`
(f) `203.0.113.100/28`
(g) `192.168.5.9/29`
(h) `172.16.8.130/30`
(i) `10.0.0.5/31`

**B5.** Convert between notations:

(a) /19 → dotted   (b) /21 → dotted   (c) 255.255.255.248 → prefix
(d) 255.255.192.0 → prefix   (e) /26 → wildcard   (f) 0.0.0.15 → prefix

**B6.** Show the full binary AND for `192.168.10.70` with masks /24, /26, /28 and /30,
and give the resulting network in each case.

**B7.** Are these pairs on the same network? Show the arithmetic.

(a) `10.1.1.1/24` and `10.1.2.1/24`
(b) `172.16.5.10/22` and `172.16.6.200/22`
(c) `192.168.1.100/26` and `192.168.1.200/26`
(d) `10.0.0.1/30` and `10.0.0.2/30`

**B8.** Which of these are valid subnet masks? For invalid ones, say why.

`255.255.255.0`, `255.255.0.255`, `255.255.248.0`, `255.255.255.7`, `255.0.0.0`,
`255.255.254.0`, `255.192.255.0`

## C. Analyse

**C1.** Explain why the network/host split makes global routing possible, using the
20-billion-to-1-million reduction. Then explain why MAC addresses cannot do the same.

**C2.** "An IP address is meaningless without its mask." Demonstrate with
`192.168.10.70` and four different prefixes, showing that the network differs each time.

**C3.** Work through a mask mismatch completely: host A `/24` and host B `/25` on the
same segment. Give each host's view, what happens in each direction, and the full symptom
set an operator would observe.

**C4.** Explain why only the sender's own mask matters for its local-or-remote decision,
and why this asymmetry is what makes a mask mismatch produce one-way connectivity.

**C5.** Classful addressing failed for three reasons. Rank them by urgency as of 1992
and justify the ranking.

**C6.** An organisation with 300 hosts under classful addressing. Enumerate their
options, compute the waste in each, and explain what they actually did and why.

**C7.** Explain how CIDR fixed both the allocation problem and the routing table problem
with one change. Be specific about the mechanism in each case.

**C8.** `172.16.0.0/12` is described as a fossil of classful thinking. Explain the
connection precisely, and say why this range is the most commonly forgotten.

## D. Design

**D1.** You have `10.50.0.0/16` and need to address a campus of six buildings, each with
between 200 and 900 hosts, plus point-to-point links between them. Propose a mask for
each and justify it.

**D2.** Write the addressing conventions document for an organisation: what the gateway
address is, where servers live in a subnet, where DHCP ranges start, and how
point-to-point links are numbered. Justify each convention.

**D3.** For the semester project's network, produce the complete address plan with
masks, and state for each subnet how much growth headroom it has.

**D4.** An organisation uses /24 for every subnet regardless of size, including
point-to-point links. Quantify the waste across a network of 40 subnets and write the
one-paragraph case for change.

## E. Troubleshoot

**E1.** Host A (`192.168.1.50`) can ping host B (`192.168.1.200`) but B cannot ping A.
Both are on the same switch, same VLAN. Give the diagnosis and the single command that
confirms it.

**E2.** A host has address `169.254.13.201/16`. What happened, and what do you check?

**E3.** A capture shows a host at `10.1.5.20` sending ARP requests for `10.1.9.4`. What
does this prove about the host's configuration?

**E4.** A router interface is configured `ip address 10.1.1.1` with no mask, and the
router now believes it is directly connected to 16 million addresses. Explain.

**E5.** Two point-to-point links exist between routers R1 and R2. R1's interface is
`10.0.0.1/30` and R2's is `10.0.0.2/31`. Do they communicate? Explain fully.

**E6.** An ACL `permit 10.1.1.0 255.255.255.0` on a Cisco router is not matching the
intended traffic. Explain the error.

**E7.** An organisation has `10.1.0.0/16` at two sites connected by a leased line
running EIGRP with auto-summary enabled. Traffic between sites is intermittent. Explain.

## F. Extend

**F1.** Write a program that takes an address and prefix and prints the binary form,
network, broadcast, usable range and count. Compare its output with
[tools/netcalc.py](../../../tools/netcalc.py) on twenty random inputs.

**F2.** Use [tools/subnet_practice.py](../../../tools/subnet_practice.py) to generate
fifty binary and mask questions. Time yourself. Repeat weekly until you average under
fifteen seconds per question.

**F3.** Read RFC 1519 (CIDR) and identify the specific projections that motivated it.
Compare them with what actually happened.

**F4.** Find the current size of the global BGP routing table (bgp.potaroo.net) and
estimate what it would be without CIDR aggregation. State your assumptions.

**F5.** Build a mask mismatch in a lab, capture the ARP traffic from both hosts, and
document the difference. Explain what each host believes.
