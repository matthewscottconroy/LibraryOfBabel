# Chapter 60 — Exercises

## A. Recall

**A1.** State what a firewall is in one sentence, using no marketing terms.

**A2.** State the two properties of ACL evaluation that cause most errors, and the diagnostic
each implies.

**A3.** Why should the final deny be written explicitly even though it is implicit?

**A4.** Explain why a stateless filter permitting outbound HTTP creates a security hole, and
what the `established` keyword does and does not fix.

**A5.** State the placement guidance for standard and extended ACLs, and the reason for each.

**A6.** Why does blocking all ICMPv6 break an IPv6 network? Name three message types that are
required.

**A7.** What does a stateful firewall record, and what does that let the policy omit?

**A8.** How does a stateful firewall track UDP, and what problem does that create?

**A9.** Give the three costs of stateful inspection, and the failure mode of each.

**A10.** Why does asymmetric routing break a stateful firewall?

**A11.** What is an ALG, why is it a layer violation, and what defeats it?

**A12.** Which data sheet figure actually limits a busy NAT or proxy device, and why is it not
throughput?

**A13.** Why did port-based classification stop working?

**A14.** List four costs of TLS inspection.

**A15.** State the false-positive asymmetry between IDS and IPS, and its usual consequence.

**A16.** Why is a VLAN not a security control?

**A17.** State the segmentation priority order and say which items deliver most of the benefit.

## B. Apply

**B1.** Examine this ACL and answer:

```
   10 deny   ip   10.20.0.0/16  any
   20 permit tcp  any           host 203.0.113.10 eq 443
   30 permit tcp  10.20.5.0/24  host 10.9.0.5     eq 1433
   40 permit udp  any           host 10.9.0.9     eq 53
   50 deny   ip   any           any
```

(a) Which rules are shadowed, and by what?
(b) Rewrite the list so that every rule can match.
(c) Add anti-spoofing entries and an explicit logged deny, and state where each goes.
(d) The organisation's prefix is 203.0.113.0/24. What anti-spoofing rule follows?

**B2.** Write the stateless ACL pair required to permit outbound HTTPS from 10.0.0.0/8, then
state precisely what an attacker can do through the return rule. Then write the stateful
equivalent and state what changed.

**B3.** A firewall has a connection table of 2,000,000 entries at 280 bytes each.

(a) How much memory does a full table consume?
(b) At 250,000 SYN per second, how long to fill it?
(c) Name three mitigations and state what each does.
(d) Half-open timeout is 30 seconds. What sustained SYN rate can be absorbed indefinitely?

**B4.** A firewall pair is deployed active/active with traffic load-balanced across both.
Sessions fail intermittently.

(a) Explain.
(b) Give three remedies in order of preference.
(c) State why the problem appears only under some conditions.

**B5.** A data sheet quotes: firewall throughput 40 Gb/s; IPS throughput 12 Gb/s; threat
prevention throughput 6 Gb/s; new sessions per second 400,000; concurrent sessions 8,000,000.

(a) Which figure applies to a deployment with application control, IPS and TLS inspection
enabled?
(b) A web proxy handles 4 Gb/s with an average object size of 30 KB and one connection per
object. Compute the new sessions per second and assess.
(c) State what you would ask the vendor before purchasing.

**B6.** Design the zone policy for a three-tier application: web, application and database, with
users and an Internet-facing load balancer. Write the permitted flows as a matrix, and justify
every "permit" cell.

**B7.** An organisation of 3,000 hosts is entirely flat.

(a) Compute how many hosts one compromise reaches.
(b) Design the first three segments you would create, with the flows permitted between them.
(c) Recompute the reachable count after your design.
(d) Estimate the work involved and state what you would do first.

**B8.** For each, state whether TLS inspection should be applied, and why:

(a) Traffic to a well-known SaaS provider used by the whole organisation
(b) Traffic to online banking
(c) Traffic to an uncategorised newly-registered domain
(d) Traffic from a server to a payment processor's API
(e) Traffic from a guest network
(f) Software update traffic from managed endpoints

## C. Analyse

**C1.** The chapter says the diagnosis for an ineffective rule is to look upward, and that
nearly everyone's first instinct is to re-read the rule they wrote. Analyse why, and identify a
general debugging principle.

**C2.** Analyse the `established` keyword as a security control. What does it actually verify,
what can an attacker do despite it, and what does its existence tell you about the evolution
from stateless to stateful?

**C3.** A stateful firewall requires that both directions of a flow take the same path, which
reintroduces a constraint packet switching removed. Analyse this trade, and identify two other
devices in this book with the same requirement and the same failure mode.

**C4.** Analyse TLS inspection as a security decision. Construct the strongest case for it and
the strongest case against, and state the position you would defend and under what conditions
you would change it.

**C5.** Studies find that TLS-inspecting middleboxes frequently negotiate weaker parameters than
the client would have. Analyse the consequences: who is harmed, who can detect it, and what the
security indicator in the browser is now measuring.

**C6.** An IPS deployed in blocking mode on day one produces an outage and a permanent loss of
trust in the device. Analyse this as an organisational rather than a technical problem, and
design the deployment that avoids it.

**C7.** Analyse the claim that a DMZ whose servers can reach the internal database is
decorative. Is this too strong? Construct the strongest counterargument and then assess it.

**C8.** Microsegmentation succeeded in cloud environments first. Analyse why, and say what a
traditional data centre would have to do to reach the same position.

**C9.** Segmentation for PCI scope reduction is described as a security control funding itself.
Analyse this as a general strategy: where else does a compliance requirement align with a
genuine control, and where does it diverge?

## D. Design

**D1.** Design the complete firewall policy for a small organisation: Internet edge, DMZ with a
web server and a mail relay, an internal user network, a server network, and a guest wireless
network. Present it as a zone matrix plus the rules for two of the interfaces in full, with
comments and review dates.

**D2.** Design the segmentation for a hospital: clinical workstations, medical devices,
building management, guest Wi-Fi, administrative staff, servers, and a research network.
Specify the segments, the permitted flows, the enforcement mechanism for each boundary, and
what you would do about devices that cannot be patched.

**D3.** Design the management plane for a 400-device estate: the management network or VRF, the
out-of-band path, the jump host arrangement, what the management systems may reach, and what
happens during an outage that removes in-band access. State the cost.

**D4.** Design a microsegmentation programme for a 600-workload data centre: the discovery
phase, the labelling scheme, how policy is authored, the monitor-mode period, the enforcement
sequence, and how you would handle an application nobody understands.

**D5.** Write the TLS inspection policy for an organisation: what is inspected, what is
bypassed and why, how the CA is deployed and protected, what is published to users, how the
device's own behaviour is verified, and what would cause you to recommend endpoint agents
instead.

## E. Troubleshoot

**E1.** A firewall rule permitting a new service is in place, verified correct, and the service
does not work. Give your diagnostic sequence.

**E2.** An engineer applies an ACL to an interface and the site loses connectivity. Explain and
state what should have been done.

**E3.** A network runs IPv4 correctly and IPv6 does not work at all after a firewall
deployment. Diagnose specifically.

**E4.** Small requests to a server succeed and large file transfers hang. Give the cause and the
specific filter to examine.

**E5.** Some connections to a service work and some do not, apparently at random, and it started
after a redundancy change. Diagnose.

**E6.** An IoT sensor works for several minutes after a reboot and then stops responding to
commands until rebooted again. Explain.

**E7.** A VoIP deployment sets up calls with no audio in one direction. List four possible
causes and how to distinguish them.

**E8.** A firewall rated at 20 Gb/s is dropping traffic at 5 Gb/s. Give three explanations.

**E9.** A banking application on managed laptops stopped working after a security deployment.
Diagnose and give two remedies.

**E10.** A ransomware incident spreads from one workstation to every server despite a
segmented network. Give three mechanisms by which this could happen.

## F. Extend

**F1.** Build a firewall policy on a Linux host using `nftables` (or `pf`), implementing:
stateful outbound access, a DMZ with one permitted inbound service, anti-spoofing, and a logged
default deny. Then attempt to reach a blocked service and find the log entry.

**F2.** Deliberately create a shadowed rule in a policy and observe the hit counters over a day.
Then use a policy analysis tool (Batfish, a vendor's shadow analysis, or a script) and compare
what it found with what you knew.

**F3.** Measure a stateful firewall's connection table behaviour: establish connections, observe
the table growing, and determine the idle timeout for TCP, UDP and ICMP experimentally. Report
the values and compare with the documented defaults.

**F4.** Generate a controlled SYN flood against your own lab firewall (`hping3` or equivalent,
on infrastructure you own) and observe the table filling. Then enable SYN cookies or SYN
proxying and repeat. Report the difference.

**F5.** Set up a TLS-inspecting proxy in a lab (`mitmproxy` will do) and examine what it
negotiates with the server compared with what the client offered. Report any downgrade, and
test what it does when the server presents an invalid certificate.

**F6.** Deploy Suricata or Zeek in detection mode against your own traffic for a week. Count the
alerts, categorise them, and determine how many you would have been willing to have blocked.
Report the implied false-positive rate.

**F7.** Take a network you have access to and map its actual segmentation: what can reach what.
Use scanning only where you are authorised. Present the result as a reachability matrix and
identify the three highest-value boundaries that do not exist.

**F8.** Read RFC 4890 and audit an IPv6 firewall policy against it. Report which required ICMPv6
types are blocked and what each would break.
