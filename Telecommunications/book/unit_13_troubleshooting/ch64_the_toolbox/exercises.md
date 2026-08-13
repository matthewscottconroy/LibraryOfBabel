# Chapter 64 — Exercises

## A. Recall

**A1.** State four things a successful ping proves and three things it does not.

**A2.** Why is "it doesn't ping" not "it's down"?

**A3.** What must you add to `ping -s` to get the IP MTU, and why?

**A4.** Distinguish a local "message too long" error from a silent drop when MTU testing.

**A5.** How do you read the number of hops from a ping reply's TTL?

**A6.** Why is the latency at traceroute hop N not the latency of hop N?

**A7.** What do asterisks in a traceroute mean, and what do they not mean?

**A8.** State the rule for reading loss in `mtr` output.

**A9.** Give the default probe type for Linux `traceroute` and Windows `tracert`, and say what
to do when both are blocked.

**A10.** What does `169.254.x.x` indicate, and what does `UP` without `LOWER_UP` indicate?

**A11.** Give the five neighbour-table states and say what `FAILED` for the gateway means.

**A12.** Name five things worth reading in every `dig` output.

**A13.** Distinguish `NXDOMAIN`, `SERVFAIL` and `REFUSED`.

**A14.** State the difference between a capture filter and a display filter, and the practical
consequence.

**A15.** Give three differences between a SPAN port and a TAP.

**A16.** Why does a single-stream `iperf3` test understate a long path's capacity?

**A17.** Distinguish `open`, `closed` and `filtered` in `nmap` output, and say why the last two
matter.

**A18.** Name the physical tool the chapter calls the highest value per pound, and say why.

## B. Apply

**B1.** Interpret each result:

(a) `ping 10.9.0.5` → replies, `ttl=126`
(b) `ping 10.9.0.5` → `Destination Host Unreachable` from 10.20.0.1
(c) `ping app.example.com` → `unknown host`; `ping 203.0.113.10` → replies
(d) `ping -M do -s 1472` → no reply; `ping -M do -s 1400` → replies
(e) `ping 10.20.5.255` → several replies marked `DUP!`
(f) `ping 10.9.0.5` → replies with `time=` values of 2, 340, 3, 410, 2 ms

**B2.** A path has 1,500-byte MTU locally and a tunnel in the middle. Design the bisection
sequence to find the path MTU with `ping`, starting from 1472 and using at most six tests. Show
the values you would try and the decision at each step.

**B3.** Read this `mtr` output and state where, if anywhere, there is real loss:

```
   1. 10.20.0.1        0.0%  200
   2. 10.0.0.1         0.0%  200
   3. 198.51.100.1    28.0%  200
   4. 198.51.100.9     0.0%  200
   5. 198.51.100.17    6.0%  200
   6. 203.0.113.10     6.0%  200
```

**B4.** For each `dig` result, give the diagnosis and the next command:

(a) `status: NXDOMAIN` for a name that colleagues can resolve
(b) `status: SERVFAIL` from your resolver; correct answer from `@8.8.8.8`
(c) Correct answer with `SERVER: 192.168.1.1` when you expected 10.9.0.9
(d) An internal name resolving to a public address
(e) `status: NOERROR` with an ANSWER section containing only a CNAME

**B5.** Write the `tcpdump` command for each:

(a) Capture all traffic to and from 10.9.0.5 on port 443, 200 packets, no name resolution
(b) Capture continuously into 30 files of 200 MB, excluding your SSH session
(c) Capture only TCP SYN packets on any interface
(d) Capture ARP and ICMP only, with Ethernet headers shown
(e) Capture VLAN-tagged frames on eth1 and write to a file

**B6.** Write the Wireshark display filter for each:

(a) All TCP resets
(b) Retransmissions to or from 10.9.0.5
(c) DNS responses with a non-zero return code
(d) ICMP fragmentation-needed messages
(e) TLS ClientHello messages
(f) Frames more than one second after the previous displayed frame

**B7.** Compute the maximum single-stream TCP throughput with a 64 KB window for RTTs of 1, 10,
40 and 120 ms.

(a) Give the four figures.
(b) For a 1 Gb/s link at 40 ms, how many parallel streams would be needed to fill it?
(c) What window would a single stream need instead?
(d) State which of (b) and (c) you would report to a carrier and why.

**B8.** For each capture signature, state the diagnosis:

(a) SYN, SYN, SYN, no reply
(b) SYN → RST
(c) SYN → SYN/ACK → ACK → RST from the server, immediately
(d) The same segment retransmitted five times from the client
(e) Zero-window advertisements from the server
(f) A TLS ClientHello followed by nothing, on a path with a tunnel

**B9.** A capture taken on a SPAN port shows 4% packet loss. The same test taken with a TAP
shows none. Explain, and state what you would report.

## C. Analyse

**C1.** Analyse why ICMP filtering is so widespread given that it breaks ping, traceroute and
PMTUD. What is the security argument, is it sound, and what should the policy actually be?

**C2.** Traceroute's two universal misreadings — mid-path latency and asterisks — are both
consequences of how routers treat their own control plane. Analyse the design decision behind
that treatment and whether it is correct.

**C3.** Analyse the capture-broadly/filter-narrowly rule. What are its costs, and under what
circumstances would you violate it deliberately?

**C4.** A capture proving the network delivered the traffic correctly is described as one of the
most valuable outcomes. Analyse why this is undervalued, and how you would present such a result
to a team that believes the network is at fault.

**C5.** Analyse the single-stream `iperf3` trap as an instance of a general problem: a
measurement that is technically correct and answers a different question from the one asked.
Find two other examples in this book.

**C6.** Analyse the SPAN port's drop behaviour. Why does a switch prioritise forwarding over
mirroring, is that correct, and what would you have to know before trusting a SPAN-based
capture?

**C7.** The chapter says scanning without authorisation is both illegal and a waste of others'
time. Analyse the professional obligations here, and write the notification you would send before
an internal scan.

**C8.** Packet capture records other people's traffic. Analyse the tension between diagnostic
necessity and privacy, and propose a policy that a network team could actually follow.

## D. Design

**D1.** Design the standard diagnostic toolkit for a network team of six: software tools,
hardware tools, what is carried to site, what lives in the office, and a rough budget. Justify
each hardware item against a specific fault it diagnoses that nothing else does.

**D2.** Design the packet capture policy for an organisation: when capture is authorised, by
whom, what may be captured, where captures are stored, how long they are kept, who may read
them, and what is done with them when the incident closes. One page.

**D3.** Design a permanent capture capability for a data centre: capture points, SPAN or TAP,
storage sizing for seven days of retention at a stated rate, the trigger mechanism, and the
access control. Show the storage arithmetic.

**D4.** Design a standard throughput testing procedure that a junior engineer could follow to
produce results a carrier would accept: what is tested, from where to where, with what
parameters, in which directions, how many repetitions, and how the result is reported.

**D5.** Design the "first ten minutes" diagnostic script for a reported outage: the exact
commands to run, in order, on which devices, and what each rules out. It must be executable by
someone unfamiliar with the specific network.

## E. Troubleshoot

**E1.** A host cannot reach anything. `ip addr` shows 169.254.8.211/16. Give the diagnosis and
the next three checks.

**E2.** `ping` to a server succeeds and HTTPS times out. Give the next three commands and what
each would tell you.

**E3.** `traceroute` stops at hop 4 and the destination is reachable by other means. Explain and
give the command that would confirm it.

**E4.** A user reports intermittent failures. `mtr` over 500 cycles shows 3% loss at every hop
from 2 onwards. Diagnose.

**E5.** DNS resolution works from one workstation and not from another on the same subnet. Give
your diagnostic sequence.

**E6.** A capture on the client shows a request sent and no response. A capture on the server
shows the request arriving and a response sent. Diagnose and state what you would do next.

**E7.** `iperf3` between two sites measures 12 Mb/s on a 1 Gb/s circuit with a 90 ms RTT.
Diagnose, and state the two tests that would distinguish the possibilities.

**E8.** A fibre link is up and showing incrementing CRC errors. Give the three checks in order,
and say which is free.

**E9.** A capture is running and the intermittent fault occurs, and the relevant packets are not
in the file. Give three reasons.

**E10.** An `nmap` scan reports port 445 as `filtered` on one host and `closed` on another.
Explain what each tells you about the two hosts.

## F. Extend

**F1.** Determine the path MTU to three destinations — one local, one across the Internet, and
one across a tunnel if you have access to one — using `ping` bisection. Verify against the
arithmetic in Chapter 61 §61.1.

**F2.** Run `mtr` to a distant destination for 1,000 cycles and analyse the output. Identify
every hop showing loss, determine which is real, and explain each conclusion.

**F3.** Capture your own machine's traffic for five minutes with `tcpdump`, then open it in
Wireshark and use only Statistics → Protocol Hierarchy and Conversations to write a paragraph
describing what your machine was doing. Do not read individual packets.

**F4.** Deliberately create each of the capture signatures in B8 in a lab (a closed port, a
dropped port, a server that resets, a lossy link with `tc netem`) and capture each. Build a
personal reference of what each looks like.

**F5.** Measure throughput between two hosts with `iperf3` at 1, 8 and 32 parallel streams, and
with the window set to the bandwidth-delay product. Report all four figures and explain the
pattern in terms of Chapter 3 §3.4.

**F6.** Set up a SPAN port and a TAP on the same link if you can, or simulate the SPAN
oversubscription by mirroring a bidirectional 1 Gb/s link to a 1 Gb/s port. Demonstrate the
drop behaviour and quantify it.

**F7.** Use `dig +trace` for a domain and document each delegation step. Then deliberately break
a delegation in a lab zone and observe where the trace stops.

**F8.** Inspect a fibre connector with a scope if you can obtain one, before and after cleaning,
and record the transceiver's Rx power in both cases. If you cannot, read the transceiver power
on every fibre link you have access to and identify any that are within 3 dB of their documented
sensitivity.
