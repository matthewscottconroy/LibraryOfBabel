# Chapter 20 — Exercises

## A. Recall

**A1.** State the single rule that a VLAN adds to the switch forwarding algorithm.

**A2.** Give the size of the 802.1Q tag, where it is inserted, and the value of the
TPID.

**A3.** How many usable VLAN IDs are there, and what are the two reserved values used
for?

**A4.** Define the native VLAN in one sentence. What is its default value?

**A5.** What is the difference between an access port and a trunk port, stated in
terms of tagging?

**A6.** Name the three ways of routing between VLANs, and state which is used in
practice.

**A7.** On a port configured with a voice VLAN, which traffic is tagged and which is
untagged?

## B. Apply

**B1.** A switch has 24 ports: 1–8 in VLAN 10, 9–16 in VLAN 20, 17–23 in VLAN 30, and
port 24 is a trunk to another switch.

(a) How many broadcast domains does this switch participate in?
(b) How many collision domains?
(c) A broadcast arrives on port 3. Out which ports does it leave?

**B2.** Write the complete configuration for:
(a) an access port for a workstation in VLAN 100, fully hardened
(b) a trunk carrying VLANs 100, 200 and 300 with a safe native VLAN
(c) an unused port

**B3.** Write the router-on-a-stick configuration for a router serving VLANs 10
(`10.1.10.0/24`), 20 (`10.1.20.0/24`) and 30 (`10.1.30.0/24`), with the router taking
the `.1` address in each.

**B4.** Rewrite B3 as SVIs on a Layer 3 switch. Identify the line whose omission
produces the most confusing failure in this chapter.

**B5.** A frame of maximum payload size traverses a link with 802.1Q, then a VXLAN
tunnel, then a QinQ provider link. Compute the frame size at each stage and state the
minimum MTU each link must support.

**B6.** An attacker on an access port in VLAN 1 attempts a double-tagging attack
against VLAN 30. Draw the frame at each of the three stages, and state precisely which
switch performs which action.

**B7.** The same attack is attempted against a trunk whose native VLAN is 999. Trace
what happens and identify where it fails.

## C. Analyse

**C1.** "One VLAN, one subnet" is described as a consequence rather than a convention.
Prove it, by describing what actually happens in each of the two violating
configurations: two subnets in one VLAN, and one subnet across two VLANs.

**C2.** Physical segmentation and VLANs achieve the same isolation. Explain why VLANs
displaced physical segmentation almost completely, giving both an economic and an
operational reason.

**C3.** The double-tagging attack exploits behaviour that is entirely correct
according to the standard. Explain what this implies about the relationship between
"specification-compliant" and "secure", and give one other example from earlier
chapters.

**C4.** VLAN hopping via DTP and VLAN hopping via double tagging are both called VLAN
hopping. Compare them: what the attacker needs, what they achieve, whether replies
return, and what stops each.

**C5.** Explain why the PCP field cannot provide end-to-end quality of service across
a routed network, and what does.

**C6.** The 802.1Q VID is 12 bits and VXLAN's VNI is 24. Explain what changed between
1998 and 2014 to make the wider field necessary, and identify one other field in this
book with the same history.

**C7.** "Segmentation creates enforcement points; it does not enforce anything."
Defend this statement, then identify a case where segmentation alone *does* provide
security.

## D. Design

**D1.** Design the VLAN scheme for a three-floor building of 200 staff: administration,
engineering, guests, IP telephony, wireless infrastructure, building management (HVAC
and cameras), and printers. Give VLAN IDs, subnets, and the policy between each pair.
Justify anything unusual.

**D2.** For the Meridian Logistics network of the semester project, produce the
complete VLAN design with an access-port template, a trunk template, and the
inter-VLAN policy. Every hardening line must be traceable to a specific attack.

**D3.** A colleague proposes putting the warehouse scanners and the finance
workstations in the same VLAN "because they're both wired and it's simpler". Write the
one-paragraph response you would send, and the one-paragraph version for a
non-technical manager.

**D4.** Design the migration from a flat 400-host network to a segmented one, in
stages, with each stage independently revertible. State what breaks at each stage if
you get it wrong.

## E. Troubleshoot

**E1.** Two workstations on adjacent ports of the same switch cannot ping each other.
Both have addresses in `192.168.5.0/24`. Give the first command you run and what you
expect to see.

**E2.** Hosts in VLAN 10 can reach each other, hosts in VLAN 20 can reach each other,
and no host can reach the other VLAN. The SVIs are configured with correct addresses
and are up. Diagnose.

**E3.** A new VLAN 60 was created and works on the access switch but not on the core.
Give two candidate causes and the single command that distinguishes them.

**E4.** After a change, hosts in VLAN 30 are receiving broadcasts from VLAN 40.
Nothing else is wrong. What was misconfigured, and on which device?

**E5.** All VLANs disappeared from a 30-switch campus at 14:20. Nobody made a change
to any switch configuration. What happened, what do you check first, and what is the
configuration change that prevents recurrence?

**E6.** An IP telephone boots and gets an address; the workstation behind it does not.
Give two causes and how to distinguish them.

**E7.** Inter-VLAN throughput is capped at about 480 Mb/s on a network with gigabit
links everywhere. Explain, and give the remedy.

**E8.** A VLAN's SVI shows as `down/down` although the switch is healthy and the VLAN
is defined. Explain the most likely cause.

## F. Extend

**F1.** Capture traffic on a trunk port and on an access port in the same VLAN
simultaneously. Compare the frames byte by byte and account for every difference.

**F2.** Build the double-tagging attack in an isolated lab and demonstrate both that it
works with a native VLAN of 1 and that it fails with `vlan dot1q tag native`. Document
the frames at each stage.

**F3.** Investigate private VLANs (RFC 5517). Explain what problem they solve that
ordinary VLANs cannot, and identify two deployments where they are the right answer.

**F4.** 802.1X dynamic VLAN assignment makes the VLAN follow the user rather than the
cable. Design a deployment for a campus, and identify three things that break during
the transition and how you would handle each.
