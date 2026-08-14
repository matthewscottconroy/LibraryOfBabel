# Chapter 62 — Exercises

## A. Recall

**A1.** Explain MAC flooding, and state precisely why the switch's behaviour under attack is
correct.

**A2.** Give the three port security violation actions and say which is usually right.

**A3.** Describe the two VLAN hopping attacks and give the mitigation for each.

**A4.** Why is double tagging one-way only, and does that make it harmless?

**A5.** Why does a rogue DHCP server work, and what is the commonest cause of one?

**A6.** What does DHCP snooping build besides a filter, and what depends on it?

**A7.** Give the three spanning tree guards and say what each prevents.

**A8.** Name the IPv6 equivalents of DHCP snooping and DAI.

**A9.** State what IP spoofing enables, and the single mitigation that would eliminate most of
it.

**A10.** Explain SSL stripping and say why the user sees no certificate error.

**A11.** What do HSTS and HSTS preloading each address?

**A12.** Why is FREAK instructive beyond its technical detail?

**A13.** State Kaminsky's insight about DNS cache poisoning.

**A14.** Name three DNS attacks that require no protocol weakness at all.

**A15.** Why does session token theft defeat MFA?

**A16.** Give the three kinds of denial of service and the defence for each.

**A17.** State the two components of an amplification attack.

**A18.** What made memcached's amplification factor so extreme, and how was it closed?

**A19.** What does RTBH actually do, and why is it nonetheless useful?

**A20.** State what "defence in depth" actually means, precisely.

## B. Apply

**B1.** A switch has a 32,000-entry CAM table.

(a) At 80,000 frames per second of unique source addresses, how long to fill it?
(b) What is the observable symptom?
(c) Write the port security configuration for an access port serving a desk with a phone and a
PC, and justify the maximum.
(d) State what support problem your choice will cause and how you would handle it.

**B2.** For each frame, state whether the attack succeeds and why:

(a) A double-tagged frame with outer tag 1, on a trunk whose native VLAN is 1
(b) A double-tagged frame with outer tag 1, on a trunk whose native VLAN is 999
(c) A double-tagged frame with outer tag 999, on a trunk whose native VLAN is 999, with
`vlan dot1q tag native` configured
(d) A DTP negotiation request to a port configured `switchport mode access` with
`switchport nonegotiate`

**B3.** Write the complete hardening configuration for an access port, with a one-line comment
on each command stating which attack it prevents. Then write the equivalent for a trunk to
another switch.

**B4.** Compute the attacker bandwidth required for a 400 Gb/s attack using each of: SSDP (30×),
DNS ANY (54×), NTP monlist (556×), memcached (51,000×).

(a) Give each figure.
(b) State which are achievable from a single domestic connection.
(c) State what stops each vector, and who has to act.

**B5.** An organisation has a 2 Gb/s Internet circuit and a scrubbing contract that activates in
15 minutes.

(a) A 30 Gb/s attack begins. Describe the first 20 minutes.
(b) What would RTBH achieve, and what would it cost?
(c) A 400 Mb/s application-layer attack begins instead. Assess the same defences.

**B6.** For each attack, state the layer, the mechanism and the most effective
mitigation:

(a) An attacker receives all traffic on a VLAN
(b) Users receive an address from an unexpected server
(c) Two hosts' traffic passes through an attacker's machine
(d) A user reaches a plaintext version of a site that supports HTTPS
(e) An entire domain resolves to an attacker's address
(f) A service is unreachable due to 200 Gb/s of UDP traffic
(g) A service is unreachable due to 300 requests per second
(h) An authenticated session continues from a new device

**B7.** Audit this configuration and list every problem:

```
   interface GigabitEthernet1/0/12
    switchport access vlan 1
    switchport mode dynamic auto
    no shutdown
   !
   snmp-server community public RO
   snmp-server community private RW
   !
   line vty 0 4
    transport input telnet ssh
    password cisco
    login
   !
   no ip dhcp snooping
```

**B8.** Rank the ten controls in §62.4's spending order for a 300-person organisation with no
security programme, and estimate a first-year cost for the top four. State what you would defer
and why.

## C. Analyse

**C1.** Every Layer 2 protocol in this book authenticates nothing, and every mitigation is a
bolt-on. Analyse whether this could have been otherwise, what it would have cost in 1985, and
whether the same mistake is being made now in any protocol you know.

**C2.** Port security's maximum must be high enough not to break legitimate use and low enough
to matter. Analyse this trade, and propose an approach that resolves it better than a fixed
number.

**C3.** Analyse the observation that most rogue DHCP servers are accidents rather than attacks.
What does that imply about how the control should be justified, monitored and responded to?

**C4.** FREAK and Logjam exploited deliberately weakened cryptography mandated by export
control. Analyse this as a policy failure with a twenty-year delay, and relate it to current
proposals for lawful access to encrypted communications.

**C5.** The mitigations in §62.2 are described as falling into three categories, none of which
is "detect the attack". Analyse why detection-based approaches have been disappointing for this
class of attack, and identify where detection is nonetheless the right answer.

**C6.** Analyse BCP 38's non-deployment as an economics problem. Model the incentives
explicitly, and propose three interventions that could change the outcome, assessing each.

**C7.** Every protocol denial-of-service attack exploits an asymmetry in work done. Analyse
three examples from §62.3 in these terms, and derive a design principle for protocol authors.

**C8.** Mirai compromised hundreds of thousands of devices using 62 default credential pairs.
Analyse the responsibility: manufacturers, operators, regulators, or the victims of the
resulting attacks? What would actually have prevented it?

**C9.** "Defence in depth" is described as frequently used to justify buying another product.
Analyse the difference between layered controls and independent controls, and design a test an
organisation could apply to its own stack.

## D. Design

**D1.** Design the complete Layer 2 hardening standard for a 2,000-port campus: the access port
template, the trunk template, the exceptions and how they are approved, the monitoring, and the
rollout sequence. Address what will break and how you will find out before users do.

**D2.** Design the DDoS response plan for an organisation with a 5 Gb/s circuit and a
public-facing service: preparation, detection, the decision tree during an attack, who is
authorised to invoke each measure, the contacts, and the post-incident review. Include the
rehearsal schedule.

**D3.** Design the DNS security posture for an organisation with 40 domains: registrar controls,
DNSSEC decision, resolver configuration, monitoring for unexpected certificates and records, and
the process for retiring a record so that subdomain takeover cannot occur.

**D4.** An organisation has been compromised by an attacker who obtained a VPN credential, moved
laterally and encrypted its file servers. Design the remediation programme in the terms of
§62.4's ordered list, with a 12-month sequence and the argument you would make for each stage.

**D5.** Design the instrumentation for the hardening checklist in §62.4: for each control, what
signal indicates it is working, what signal indicates it has been bypassed or disabled, and what
alert you would create. Present it as a table.

## E. Troubleshoot

**E1.** A switch begins flooding all traffic to all ports. Give three causes and how to
distinguish them.

**E2.** A user's port shuts down every time they connect a docking station. Diagnose and give
two fixes with their trade-offs.

**E3.** Users in one building receive addresses in the wrong range. Give your diagnostic
sequence and the permanent fix.

**E4.** Two servers experience intermittent connectivity, and their ARP entries for each other
change periodically. Give two possible causes and the command that distinguishes them.

**E5.** After a contractor connects equipment, the network reconverges and several links change
state. Diagnose and state which single configuration line would have prevented it.

**E6.** IPv6 hosts on a user VLAN begin routing through a workstation. Explain and give the
mitigation.

**E7.** A public website is reachable over HTTP as well as HTTPS, and users on a public Wi-Fi
network report unexpected content. Diagnose and give three remedies.

**E8.** A domain's traffic is redirected for six hours. The DNS records at the authoritative
servers are correct. Give three explanations.

**E9.** An account protected by hardware-key MFA is used by an attacker. Explain how, and state
the three mitigations.

**E10.** A service behind a CDN is taken offline by a volumetric attack. The CDN reports it
absorbed the attack successfully. Explain.

**E11.** A firewall's connection table fills during an attack while the protected servers remain
lightly loaded. Explain what is happening and give two configuration changes.

## F. Extend

**F1.** In an isolated lab you control, demonstrate MAC flooding against a switch with and
without port security, using `macof` or an equivalent. Record the CAM table size, the time to
fill it, and the observable effect. Do this only on equipment you own.

**F2.** Demonstrate ARP spoofing between two lab hosts with `arpspoof` or `bettercap`, then
enable DHCP snooping and dynamic ARP inspection and repeat. Capture the log entries DAI
produces.

**F3.** Set up a rogue DHCP server on a lab segment and observe which clients accept it and how
quickly. Then enable DHCP snooping and confirm the offer is dropped. Record the exact log
message so you would recognise it in production.

**F4.** Measure amplification factors yourself: query a DNS resolver you operate with a small
ANY query and measure the response size ratio. Do the same for any other UDP service you run.
Report the factors and identify anything on your network that could be reflected.

**F5.** Test your own network for open reflectors using a public scanning service's data (Shodan,
the Open Resolver Project, or Shadowserver reports for your ASN). Report what is exposed and
who owns it.

**F6.** Check a set of domains you own for: HSTS, HSTS preload status, DNSSEC, CAA records,
registry lock, and dangling CNAMEs. Report the findings as a remediation list.

**F7.** Audit a switch configuration you have access to against §62.4's data plane checklist.
Report the proportion of items present, and estimate the time to remediate the rest.

**F8.** Investigate one large DDoS incident in detail — GitHub 2018, Dyn 2016, or a more recent
one. Determine the vector, the peak volume, the duration, how it was mitigated, and what changed
afterwards. Assess whether the same attack would work today.

**F9.** Read RFC 2827 and RFC 3704, then determine whether your own network implements ingress
filtering, and whether your provider does (the Spoofer project measures this). Write the
paragraph you would send to your provider if it does not.
