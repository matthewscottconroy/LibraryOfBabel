# Chapter 53 — Exercises

## A. Recall

**A1.** Name the three diagrams, state the question each answers, and give one thing that
belongs on each and one that does not.

**A2.** Why is a single diagram containing everything worse than three separate ones? Give two
distinct reasons.

**A3.** What spanning tree information belongs on the logical diagram, and why does its absence
make the diagram misleading?

**A4.** State the labelling rule in one sentence, and explain why the alternative is worse than
no label at all.

**A5.** What five things should appear on every diagram regardless of type?

**A6.** List the fields an asset inventory must record, and say which one answers the only
strategic question anyone asks of it.

**A7.** Why does airflow direction belong on a rack elevation?

**A8.** Distinguish an address plan from IPAM.

**A9.** Describe the static-address-inside-a-DHCP-pool failure, including why the symptom
appears months later and why it affects two machines differently.

**A10.** What is a quarantine period for a released address, and what does it prevent?

**A11.** Name two ways IPv6 makes address management easier and three ways it makes it harder.

**A12.** State the test of a good runbook, and the reason most fail it.

**A13.** When should a runbook be written, and why then?

## B. Apply

**B1.** For a two-site network — head office with a core pair, four access switches and a
firewall; a branch with one switch and a router; connected by an MPLS circuit and a backup VPN
— list what belongs on each of the three diagrams. Then draw all three.

**B2.** Design a labelling scheme for an organisation with 4 buildings, up to 6 floors each, up
to 3 comms rooms per floor, up to 8 racks per room. Show the format, give three example labels,
and state how a patch cable is labelled.

**B3.** Critique these labels and rewrite each:

(a) `Server`
(b) `To upstairs`
(c) `HR printer — do not unplug`
(d) `Port 14`
(e) `Uplink 2`

**B4.** A rack has 42 U. It contains: 2 × 1 U switches, 1 × 2 U firewall pair, 4 × 2 U servers,
1 × 3 U UPS, 2 × 1 U patch panels, and a 1 U cable manager between each pair of adjacent
devices.

(a) How many rack units are consumed?
(b) How much is free?
(c) You are asked to add a 4 U chassis switch that draws 900 W. What three things do you check
before agreeing?

**B5.** A subnet 10.20.0.0/24 has gateway .1, a DHCP pool of .100–.200, and static assignments
at .10–.40.

(a) How many addresses are available for new static assignments?
(b) A colleague assigns .150 to a printer "because it responded to nothing". Explain what will
happen and when.
(c) Write the IPAM record that would have prevented it.

**B6.** An organisation has 900 devices across 12 subnets at 3 sites.

(a) Argue for or against dedicated IPAM tooling, with reasons.
(b) State the three fields whose absence would most damage an incident response.
(c) Describe how you would reconcile the current records against reality.

**B7.** Take the runbook skeleton in §53.4 and write a complete one for "an access switch is
unreachable from the management network." Include confirm, impact, the layer-below check, three
ordered causes, a disruptive action with its disruption stated, and escalation on a clock.

## C. Analyse

**C1.** The chapter describes documentation as "a cache". Develop the analogy: what is the hit
rate, what is the staleness problem, what is the eviction policy, and where does the analogy
break down?

**C2.** Analyse why calendar-based documentation review fails and change-triggered review works.
Is the difference about incentives, visibility, or something else?

**C3.** Automated discovery produces a current, ugly, complete picture; a hand-drawn diagram
produces an intelligible, curated, stale one. Analyse the trade and argue for a practice that
uses both. What exactly should the automation be used for?

**C4.** "An unlabelled cable is honestly unknown; a wrongly labelled one is confidently wrong."
Analyse this as a general principle about documentation and identify two other places in this
book where the same asymmetry appears.

**C5.** Analyse the argument that an integrated DDI system removes an error class "by
construction rather than by discipline". What other operational problems have the same shape,
and what does that suggest about where to spend effort?

**C6.** IPv6's SLAAC allows a host to obtain a valid global address that nothing recorded.
Analyse the consequences for address management, security monitoring and incident response, and
say what you would do about each.

**C7.** The chapter claims a single point of knowledge is an availability risk in the same
category as a single power feed. Assess this comparison rigorously: is it fair? Compute or
estimate the comparison if you can.

**C8.** Analyse why runbooks written by experts fail the 03:00 test. Is this a writing problem,
a review problem, or an incentive problem? Design a process that fixes it.

## D. Design

**D1.** Design the complete documentation set for a 12-site retail organisation: what documents
exist, who owns each, what triggers each to be updated, where they live, and how they remain
accessible during an outage. Keep it to two pages.

**D2.** Design an IPAM structure for an organisation of 40 sites growing to 100, addressing
both IPv4 (with a 10.0.0.0/8 allocation) and IPv6 (with a /32). Specify the hierarchy, the
allocation policy, what is recorded, and how records are kept consistent with DNS and DHCP.

**D3.** Design the process by which a network change updates documentation automatically or
verifiably. Specify what is captured, by what mechanism, what is verified and how a discrepancy
is surfaced. State what you would not attempt to automate and why.

**D4.** Write the runbook index for a mid-sized enterprise: which runbooks should exist, ordered
by priority of writing them, with a one-line justification for each of the top ten.

**D5.** Design a programme to eliminate single points of knowledge in a team of six engineers
where one person holds most of the firewall and routing knowledge. Address: what you would
measure, what you would change, over what period, and how you would know it had worked — without
making that person defensive.

## E. Troubleshoot

**E1.** During an incident, the diagram shows a redundant pair of links between two buildings.
Both are down. Explain the most likely cause and what the documentation should have recorded.

**E2.** A device appears in no diagram, no inventory and no IPAM record, and is passing
production traffic. Describe how you would establish what it is, and what process failure it
indicates.

**E3.** Two machines experience intermittent, protocol-dependent connectivity failures at
unpredictable times. Neither configuration is wrong. Diagnose, and give the specific command
that confirms it.

**E4.** A subnet's addresses are exhausted, but IPAM shows 40% free. Give three causes and how
you would reconcile.

**E5.** After a device is replaced, monitoring alerts on the old hostname and the new device
appears unmonitored. Identify the process failure and the structural fix.

**E6.** An engineer follows a runbook at 02:40 and reaches a step that says "restart the
service". They do not know whether this will affect other sites. Analyse what the runbook should
have said and why its absence is dangerous rather than merely inconvenient.

**E7.** The corporate wiki containing all runbooks is unreachable because it authenticates
against a domain controller in the failed data centre. Describe the immediate response and the
permanent fix.

**E8.** An address freed last week and reissued this week causes a firewall to permit traffic it
should not. Explain the mechanism and the policy that prevents it.

## F. Extend

**F1.** Produce the three diagrams for a network you have access to — your home network is
sufficient and will be more instructive than you expect. Include the physical routes of any
cable that leaves a room. Then have someone else attempt to find a specific cable's far end
using only your physical diagram.

**F2.** Install NetBox or Nautobot and model a small network in it: sites, racks, devices,
interfaces, cables, prefixes and IP addresses. Then generate a report of unused addresses.
Comment on what the tool forced you to know that you had not written down.

**F3.** Collect LLDP or CDP neighbour information from every device on a network you administer
and generate a topology automatically (`lldpctl`, SNMP, or a script). Compare it with the
existing diagram and list every discrepancy.

**F4.** Read the ANSI/TIA-606 labelling standard, or a published summary of it. Compare its
scheme with the one in §53.2, and write a paragraph on what the standard adds and what it
over-specifies for a small organisation.

**F5.** Take a runbook from your organisation (or write one from a procedure you know) and have
someone unfamiliar execute it in a test environment while you watch silently and take notes.
Record every point at which they hesitate or ask a question. Rewrite it and repeat.

**F6.** Audit an environment you have access to for single points of knowledge: for each of
addressing, firewall policy, routing, wireless and backups, determine how many people could
answer a substantive question. Present the result as a table without naming individuals.

**F7.** Find three configurations in a live network that would make a competent engineer ask
"why is that there?". Establish the answer for each, and write the knowledge base entry. Record
how long each took to establish and who you had to ask.
