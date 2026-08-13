# Chapter 68 — Exercises

## A. Recall

**A1.** Distinguish the control, data and management planes by what each does and what its
failure means.

**A2.** Why was the control plane distributed in 1989? State the reason precisely.

**A3.** Give five costs of a distributed control plane.

**A4.** Name the three control-plane architectures and say which one dominates in practice.

**A5.** Give six places where control/data separation has genuinely succeeded, and say which one
arrived first.

**A6.** Why did wireless controllers separate control from data seven years before SDN was named?

**A7.** Give the four possible controller states and what the network does in each.

**A8.** What does an OpenFlow flow entry consist of, and why is matching across layers radical?

**A9.** Distinguish reactive from proactive flow installation and give the arithmetic that
killed the first.

**A10.** Give six reasons OpenFlow did not sweep the field.

**A11.** Why did OpenFlow's expressiveness map badly onto real ASICs?

**A12.** What was OpenFlow's most consequential outcome, and why is that ironic?

**A13.** What four things does a P4 program specify?

**A14.** What does in-band network telemetry answer that no monitoring system can?

**A15.** Why was the Tofino discontinuation significant, and what was the reason?

**A16.** Give the four verbs of intent-based networking and say which two distinguish it from
automation.

**A17.** Why can a system not translate "make the finance network secure" into a configuration?

**A18.** What is the difference between checking configuration and checking state, and why does
it matter?

## B. Apply

**B1.** For each system, state whether its control plane is distributed, hybrid or centralised,
and what happens when its controller is unreachable:

(a) An OSPF network
(b) A wireless network with a controller
(c) An SD-WAN deployment
(d) An EVPN/VXLAN fabric
(e) A public cloud VPC
(f) A pure OpenFlow deployment with reactive flow installation

**B2.** A switch has 4,000 ACL TCAM entries and sees 250,000 concurrent flows.

(a) What fraction of flows could have a per-flow entry?
(b) At 8,000 new flows per second with a 1.2 ms controller round trip, what is the controller's
request rate and the added latency on a new flow?
(c) Design the proactive alternative and state how many entries it would need.

**B3.** For each requirement, state whether it needs a programmable pipeline (P4), a
conventional switch, or a host, and justify:

(a) Forwarding a newly-invented encapsulation at line rate
(b) Recording per-hop queue depth in the packet
(c) Applying an ACL by source address
(d) Aggregating machine learning gradients in the network
(e) Load balancing across a pool of servers
(f) Terminating TLS

**B4.** Classify each claim about an intent-based product as real, bounded or marketing, with a
reason:

(a) "Verify that the guest network cannot reach the finance servers, exhaustively"
(b) "Describe your intent in plain English"
(c) "Detect and correct configuration drift automatically"
(d) "The system designs the network for you"
(e) "Continuously confirm the network matches the intent"
(f) "Self-healing infrastructure"

**B5.** Write the four intent statements you would define for a branch office network, in a form
that is testable. For each, state the check that would verify it and whether that check is
against configuration or against state.

**B6.** An organisation asks whether to deploy an SDN controller for its 40-switch campus.

(a) State the questions you would ask before answering.
(b) State the three most likely honest answers and their conditions.
(c) List what the organisation could do instead that captures most of the value.

**B7.** Google's B4 is described as succeeding under conditions most organisations do not have.

(a) List those conditions.
(b) For each, state what an ordinary enterprise has instead.
(c) State which single condition you consider most decisive.

**B8.** For each SDN promise, state what actually arrived and by what route:

(a) A central controller programming switches
(b) Commodity network hardware
(c) Network programmability
(d) Vendor independence
(e) A network with an API

## C. Analyse

**C1.** The chapter argues distribution was a constraint rather than a goal. Analyse whether
that remains true: could an Internet-scale control plane be centralised today, and what would
prevent it?

**C2.** Analyse the hybrid architecture's dominance. What does it get from each of the extremes,
and what does it give up?

**C3.** Wireless controllers arrived seven years before SDN and nobody called it SDN. Analyse
why the architecture emerged there first and what that says about how architectures actually
spread.

**C4.** Analyse OpenFlow's vendor-incentive problem rigorously. Was the outcome inevitable? What
would have had to be true for incumbent vendors to have implemented it well?

**C5.** Open vSwitch is described as OpenFlow's most consequential outcome. Analyse why the model
succeeded in software and failed in hardware, and what that implies for P4.

**C6.** Analyse the Tofino discontinuation as an economics problem. Model the incentives for a
switch vendor, a merchant silicon vendor and a hyperscale buyer, and say what would change the
outcome.

**C7.** The chapter says the verification half of intent-based networking is real and least
emphasised in marketing. Analyse why demonstrable capabilities are undersold relative to
aspirational ones, and what a buyer should do about it.

**C8.** "The idea was correct and the implementation route was wrong." Analyse this as a general
pattern. Find two other examples in this book, and propose how to recognise it while it is
happening.

**C9.** Analyse the claim that the most SDN-like networks in existence are the public clouds,
built by companies that were not in the OpenFlow standards process. What does this say about the
relationship between standards efforts and implementation?

## D. Design

**D1.** Design the verification programme for a 200-device network: what properties you would
assert, how each is checked, what tool performs the check, how often, and what happens on a
violation. Include at least six properties and state which require state rather than
configuration.

**D2.** Design a control-plane architecture for a 60-site SD-WAN: what is centralised, what is
distributed, what happens during a controller outage, how policy is deployed, and how the design
is verified. Justify each centralisation decision.

**D3.** An organisation is evaluating an intent-based networking product. Design the evaluation:
the questions, the tests you would insist on, the failure scenarios you would require them to
demonstrate, and the criteria on which you would decide. Two pages.

**D4.** Design a use of in-band network telemetry for a data centre fabric: what is recorded,
where it is inserted and stripped, how the MTU is handled, what volume of data results, and what
questions it would answer that Chapter 54's tooling cannot.

**D5.** Write the two-page assessment you would give a CTO who has been told that SDN will halve
the network team's headcount. Be fair to the idea, honest about what arrived, and specific about
what would actually reduce operational effort.

## E. Troubleshoot

**E1.** An SDN controller becomes unreachable and the network stops forwarding entirely. Explain
what design decision produced this and what the alternative is.

**E2.** A pure OpenFlow deployment performs well in a lab and collapses in production at 6,000
new flows per second. Diagnose.

**E3.** A switch advertised as supporting OpenFlow 1.3 rejects a flow rule matching on both TCP
port and VLAN priority. Explain.

**E4.** Two controllers in a cluster are separated by a network partition and both continue
programming devices. Describe the consequences and the mechanism that should have prevented it.

**E5.** A P4 program compiles for `bmv2` and fails to compile for the target hardware. Give three
likely reasons.

**E6.** An intent-based system reports full compliance and a user cannot reach a service.
Explain how both can be true.

**E7.** An automatic remediation system disabled a port during an incident, worsening it.
Analyse the design failure and state the correct default.

**E8.** In-band telemetry is enabled and large transfers begin failing. Diagnose in one command.

## F. Extend

**F1.** Install Mininet and Ryu (or a similar teaching controller) and build a small OpenFlow
network. Implement a learning switch in the controller, then observe the flow table filling.
Then implement it proactively and compare the controller's load.

**F2.** Measure reactive flow setup latency in the lab from F1: time the first packet of a new
flow against a subsequent one. Relate the result to §68.2's argument.

**F3.** Install Batfish and run it against a set of configurations — your own, or the public
example set. Ask it three reachability questions and report what it found. Then deliberately
introduce a shadowed ACL rule and confirm it detects it.

**F4.** Write a P4 program for `bmv2` that parses a custom header of your own design and forwards
on a field in it. Send a packet with `scapy` and confirm it is forwarded. Document the parser
state machine.

**F5.** Read the B4 paper (Jain et al., 2013) and list every condition Google's environment
satisfied that a typical enterprise does not. Assess which are technical and which are
organisational.

**F6.** Write down five intent statements for a network you know, in testable form. For each,
determine whether it is currently true, using whatever means you have. Report which you could
not determine and why.

**F7.** Examine an SDN or intent product's documentation — not its marketing — and determine
precisely: what is centralised, what happens when the controller is unavailable, what it can
verify, and what it can change automatically. Present the answers as a table.

**F8.** Read the OpenFlow 1.0 and 1.3 specifications' table models and list the differences.
Assess what a vendor implementing 1.0 would have had to change, and relate it to §68.2's churn
argument.
