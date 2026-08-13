# Chapter 34 — Exercises

## A. Recall

**A1.** What IP protocol number is ICMP? What is ICMPv6's?

**A2.** Give the type numbers for echo request, echo reply, destination unreachable and
time exceeded.

**A3.** Give the meaning of destination-unreachable codes 0, 1, 3, 4 and 9.

**A4.** What does an ICMP error message carry from the packet that caused it, and why
exactly that much?

**A5.** What does a successful ping prove? Give four things, and four things it does not
prove.

**A6.** What probe type does Unix `traceroute` use by default? Windows `tracert`? What
does each expect from the destination?

**A7.** What does `* * *` mean in traceroute output?

**A8.** Give the three ICMP types that must not be blocked, and the consequence of
blocking each.

## B. Apply

**B1.** For each observed result, give the ICMP type and code, and the diagnosis:

(a) A router says it has no route to the network
(b) A router has a route but ARP fails
(c) A host is up with nothing listening on the UDP port
(d) A packet is too large and DF is set
(e) A firewall drops the packet and admits it
(f) TTL reached zero

**B2.** Compute the payload size for `ping -M do -s N` to test each path MTU: 1500, 1492,
1476, 1450, 1420, 1280.

**B3.** Given this `mtr` output, state whether hop 4 is genuinely lossy and justify:

```
HOST              Loss%   Snt   Avg
1. 10.0.0.1        0.0%   200   0.8
2. 10.1.0.1        0.0%   200   4.2
3. 203.0.113.1     0.0%   200  11.9
4. 198.51.100.1   18.0%   200  47.3
5. 198.51.100.9    0.0%   200  12.4
6. 93.184.216.34   0.0%   200  88.1
```

**B4.** Same output, but hop 4 shows 18% and hops 5 and 6 show 18% and 19%. Redo the
analysis.

**B5.** A `ping` reply shows `ttl=118`. Give the likely initial TTL, the hop count, and
the probable operating system.

**B6.** Compute the effective MTU for: Ethernet with a GRE tunnel inside an IPsec tunnel
inside PPPoE. Then state the MSS you would clamp to.

**B7.** For each symptom, state whether PMTUD is the likely cause and give the one command
that would confirm it:

(a) SSH connects, `scp` hangs  (b) `ping` fails entirely  (c) a web page loads text and
not images  (d) DNSSEC validation fails for one zone  (e) a VPN connects and carries
nothing  (f) traceroute shows stars at hop 3

## C. Analyse

**C1.** Explain why an ICMP error carries the first 8 bytes of the original payload, what
that enables, and why it is a layer violation.

**C2.** Explain why a NAT device must rewrite inside the ICMP payload, and what fails when
it does not.

**C3.** "A successful ping is strong evidence; a failed ping is weak evidence." Explain
the asymmetry, giving three reasons a working host might not answer.

**C4.** Explain completely why intermediate traceroute latency is not path latency, using
Chapter 29 §29.1's control-plane distinction.

**C5.** Explain the rule that loss at one hop which does not appear at later hops is not
real loss. Prove it.

**C6.** Give the full argument against blocking all ICMP, listing what breaks and ranking
the items by severity.

**C7.** Explain why the PMTUD black hole's symptom looks like an application fault, and
why that costs so much diagnostic time.

**C8.** MSS clamping is described as ugly, universal and limited. Explain each of the
three.

**C9.** IPv6's PMTUD situation is described as both worse and better than IPv4's. Explain
both halves.

**C10.** Explain why QUIC implements its own path MTU discovery rather than relying on
ICMP, and connect it to Chapter 21 §21.4.

## D. Design

**D1.** Write the complete ICMP firewall policy for an Internet edge — inbound and
outbound, IPv4 and IPv6 — with a one-line justification for every permit and every deny.

**D2.** Design the MTU and MSS configuration for a site connecting over an IPsec VPN to
head office and running VXLAN internally. Specify every interface.

**D3.** For the semester project's network, write the diagnostic runbook for "small things
work, large things hang", suitable for a first-line technician.

**D4.** Design a monitoring check that would detect a PMTUD black hole before users report
it. What does it test, from where, and how often?

**D5.** An organisation blocks all ICMP by policy and will not change it. Write the
mitigation plan and state precisely what remains broken.

## E. Troubleshoot

**E1.** Users can log into a remote server over SSH and every file transfer hangs at
about 4 KB. Diagnose in two commands.

**E2.** A newly-built web server is reachable and its pages load without images. `ping`
works. Diagnose.

**E3.** `traceroute` to a partner site shows stars from hop 4 onward and never reaches the
destination, but `curl https://partner` works. Explain and give the command that traces
the real path.

**E4.** `mtr` shows 40% loss at hop 2 and 0% at every hop after it. A user insists the
network is dropping packets. Explain, with the reasoning.

**E5.** After a firewall upgrade, IPv6 stopped working entirely while IPv4 is unaffected.
Give the cause and the reference that specifies the fix.

**E6.** An application works from the office and hangs from the VPN. Both use the same
server. Give the most likely cause and the one-line fix on the VPN gateway.

**E7.** A ping sweep of `10.1.5.0/24` finds 40 hosts. The IPAM says 95 are allocated.
Explain the discrepancy and state which number to trust for what purpose.

**E8.** A monitoring system reports a server as down. `arping` to it succeeds. What has
been proved, and what is the next step?

**E9.** A `traceroute` shows the same two addresses alternating for fifteen hops. Diagnose
precisely.

## F. Extend

**F1.** Build a lab with a deliberately reduced MTU on a middle link. Verify PMTUD works.
Then block ICMP type 3 code 4 and reproduce the black hole. Document both, including the
exact application symptom.

**F2.** Fix the black hole three different ways — unblocking ICMP, MSS clamping, and
`tcp_mtu_probing` — and compare what each does and does not solve.

**F3.** Run `mtr --report` to five distant destinations for 200 cycles each. For every hop
showing loss, determine whether it is real, and justify each conclusion.

**F4.** Compare `traceroute`, `traceroute -I`, `traceroute -T -p 443`, and `tracert` to
the same destination from the same host. Explain every difference.

**F5.** Read RFC 4890 and write your organisation's IPv6 ICMP policy from it. Identify the
types where you disagree with the recommendation and say why.

**F6.** Capture an ICMP type 3 code 4 message and decode the embedded original header by
hand. Confirm that the first 8 bytes of payload contain the ports.
