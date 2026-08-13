# Project Deliverables

Seven staged submissions for the Meridian Logistics design. Each is marked and
returned; the final paper incorporates all of them, revised.

Length limits are ceilings, not targets. A submission that makes its argument in
four pages beats one that pads to eight.

---

## Deliverable 1 — Requirements Analysis

**Due week 3 · 10% · 4 pages max · Chapters 1, 3, 11, 14**

Before any technology is named, establish what the network must do.

**Required content:**

1. **A communication matrix.** Who talks to whom, about what, and how much.
   Rows and columns for each group and each system in the brief; each populated
   cell states the traffic type, its approximate volume, and its sensitivity to
   latency and loss. This is the artefact everything downstream is derived from.

2. **Application requirements table.** For each application: bandwidth per user,
   latency tolerance, loss tolerance, and whether it is interactive. Use
   Chapter 3's distinctions — a table that says "high bandwidth" for a VoIP call
   demonstrates that §3.1 was not absorbed.

3. **Availability requirements**, per system, expressed as the cost per hour of
   unavailability and the resulting target. Chapter 56 §56.1 converts a target
   into a downtime budget; do that conversion and state whether the target is
   achievable given planned maintenance.

4. **Growth assumptions**, stated explicitly with the reasoning.

5. **Constraints and non-functional requirements**: regulatory, budgetary,
   operational (remember there are two IT staff), and physical.

6. **Questions you would ask, and assumptions you are proceeding under.** The
   brief is deliberately incomplete. Identifying the gaps is assessed.

**What loses marks:** stating solutions instead of requirements ("we need
gigabit to the desktop" is not a requirement); omitting the operability
constraint; treating all applications as equivalent.

---

## Deliverable 2 — Addressing Plan

**Due week 6 · 15% · 5 pages max · Chapters 25, 26, 27**

**Required content:**

1. **Choice of address block**, with justification. Which RFC 1918 range, which
   slice of it, and why — including the merger argument from Chapter 27 §27.1.

2. **The allocation hierarchy.** How the space divides by floor, then by
   function, then by VLAN, designed so that each level can be summarised.
   State the summarisation that would be possible if a second site opened.

3. **A complete subnet table.** For every subnet: prefix, purpose, VLAN ID,
   gateway address, DHCP pool range, reserved static range, expected host count,
   and current utilisation percentage. Include the point-to-point links.

4. **Working shown for at least three subnets**, in binary, per Appendix A §A.5.
   Not because you cannot use a calculator, but because the marker needs to see
   that you can.

5. **Conventions**, stated and justified: where the gateway sits, where
   infrastructure sits, where DHCP begins and ends, and — critically — **why**,
   so a successor does not violate them by accident.

6. **Growth headroom**, explicit, with the assumption behind it.

7. **The IPv6 plan**, even though it will not be deployed on day one. A /48 from
   the ISP, divided into /64s, mapped onto the same VLAN structure.

**Verify with** `netcalc.py vlsm` and `netcalc.py summarise`, and say that you
did.

**What loses marks:** subnets sized without reference to Deliverable 1's counts;
allocation in an order that prevents summarisation; no growth space; a plan that
would collide with a plausible acquisition.

---

## Deliverable 3 — Logical Design

**Due week 8 · 15% · 6 pages max · Chapters 17, 19, 20, 29, 30, 39, 40**

**Required content:**

1. **The Layer 2 logical diagram**: VLANs, trunks, link aggregation, spanning
   tree topology with the root bridge identified and blocked ports marked.

2. **The Layer 3 diagram**: subnets, gateways, routing between them, the
   Internet edge, and the path to the cloud IaaS deployment.

3. **VLAN table**: ID, name, purpose, subnet, which ports or SSIDs map to it.
   Justify the segmentation scheme — by function, by department, by security
   zone, or a hybrid — against Chapter 20 §20.1 and Chapter 57's threat model.

4. **Inter-VLAN routing decision**, with the alternatives considered.
   Router-on-a-stick, SVIs on a Layer 3 switch, or routed ports — and why, given
   the traffic matrix from Deliverable 1.

5. **Routing decision.** Static or dynamic, and why. Note that Chapter 30 §30.1
   makes a serious case for static here, and that the IT team has never run a
   routing protocol. Defend whichever you choose against that.

6. **Spanning tree configuration**: root bridge placement (explicit, not
   default), protection features on access ports, and what happens when the
   inter-floor link fails.

7. **Core services**: where DHCP, DNS and NTP live, what their scopes and
   options contain (including the options the phones and access points need),
   how they are made redundant, and what breaks when each is unavailable.

**What loses marks:** leaving the root bridge to default election; VLANs with no
routing plan; DHCP options omitted for devices that need them; no answer to
"what happens when the riser link fails".

---

## Deliverable 4 — Physical and Wireless Design

**Due week 9 · 15% · 6 pages max · Chapters 6, 10, 16, 42–45**

**Required content:**

1. **The Layer 1 diagram**: rooms, comms rooms, cable routes, the riser, patch
   panels, and switch placement with port counts.

2. **Media selection**, per run, with justification against distance,
   environment, rate and power delivery. The warehouse run and the riser both
   need explicit treatment. Use Chapter 10 §10.5's procedure and state the
   alternatives rejected.

3. **Cable category decision**, with the labour-versus-material argument
   quantified (Chapter 10's chapter introduction gives the shape of it; supply
   your own figures and state their source or that they are estimates).

4. **PoE budget.** 22 cameras plus access points plus phones. Which standard,
   what the per-device draw is, what the switch must supply, and what headroom
   remains. This is arithmetic and it must be shown.

5. **Wireless design.** Coverage or capacity per area, with the determination
   justified per Chapter 45 §45.3 — reception, open-plan operations, training
   room and warehouse are four different problems. Access point count and
   placement, band and channel plan (derived, per Chapter 43 §43.2, not
   asserted), power settings with the argument from Chapter 45's introduction,
   and the SSID structure.

6. **The warehouse specifically.** Metal racking to 4 m, an overhead crane,
   handheld scanners that must not drop mid-aisle. Address multipath, roaming,
   and why the obvious answer of "more power" is wrong.

7. **A link budget** for at least one wireless path, computed with
   `perfcalc.py linkbudget`, with the margin stated and assessed.

**What loses marks:** a channel plan with no derivation; ignoring the racking;
a PoE budget that does not add up; treating the warehouse and the office as one
wireless problem.

---

## Deliverable 5 — Security Design

**Due week 12 · 15% · 6 pages max · Chapters 57–62**

**Required content:**

1. **A threat model.** Who would attack this organisation and what for, per
   Chapter 57 §57.1. Be proportionate: this is a freight forwarder, not a bank,
   and a design that assumes a state adversary will be marked down for
   innumeracy as readily as one that assumes none.

2. **Asset and risk register.** What matters, what the loss would cost, what the
   response is (mitigate, transfer, accept, avoid) — and where you accept, say so
   explicitly.

3. **Segmentation policy.** Which zones exist, what may reach what, and the
   blast-radius argument for each boundary. Cameras, guest Wi-Fi, warehouse
   scanners, finance and the customs application each need a position.

4. **Firewall rule set**, at policy level rather than syntax: a table of source
   zone, destination zone, service, action, and justification. Include the
   implicit deny and say where the trust boundary for QoS marking sits.

5. **Authentication design.** 802.1X where it can be deployed and what to do
   about the devices that cannot; wireless authentication per SSID; remote access;
   administrative access to network devices; and the MFA position.

6. **Remote access.** VPN or ZTNA or both, with the split-tunnelling position
   argued rather than assumed. Address the customs-data constraint explicitly.

7. **Encryption.** What is encrypted in transit, what is not, and why that is
   acceptable. The customs requirement is a hard constraint; identify it.

8. **The access-layer hardening checklist** from Chapter 62 §62.4, with each item
   tied to the attack it prevents.

**What loses marks:** a security design disconnected from the threat model;
segmentation that the addressing plan cannot express; ignoring the two-person IT
team's ability to operate what you propose.

---

## Deliverable 6 — Operations and Troubleshooting Plan

**Due week 13 · 10% · 5 pages max · Chapters 53–56, 63–66**

**Required content:**

1. **Documentation plan.** What documents exist, who maintains them, what
   triggers a review. The three diagrams from Deliverables 3 and 4 are part of
   this; state how they stay current.

2. **Monitoring design.** What is measured, at what interval, what the baseline
   collection plan is, and — assessed heavily — **the alert list**: each alert,
   its trigger, and the action a technician takes when it fires. Alerts with no
   action lose marks (Chapter 54's argument).

3. **Change management.** The process, the categories, and a worked example
   change record for a realistic change, including its rollback plan and how the
   rollback would be tested.

4. **Availability and recovery.** Per system: RPO, RTO, the redundancy provided,
   and the shared-fate analysis (Chapter 56 §56.2) for each redundant pair. The
   two ISP services are a redundant pair; analyse them.

5. **Failure scenarios.** Six specific failures — at minimum: the riser link,
   the primary ISP, a comms-room switch, the DHCP server, the wireless
   controller, and a warehouse coverage complaint. For each: expected symptom,
   detection method, diagnostic procedure, and remedy.

6. **Two runbooks**, written to the 03:00 test of Chapter 53 §53.4.

**What loses marks:** alerts with no action; RPO/RTO stated uniformly across all
systems; failure scenarios whose diagnosis is "check the logs".

---

## Deliverable 7 — Final Paper and Defence

**Due week 14 · 20% · 20 pages max excluding diagrams and appendices**

**The paper** integrates Deliverables 1–6, revised in light of feedback, into one
coherent document. It is not a stapled collection: the addressing plan must
support the segmentation, which must support the security policy, which must be
operable by the team described.

**Required additions beyond the earlier deliverables:**

1. **An executive summary**, one page, written for the managing director. No
   acronyms without expansion. State what is being built, what it costs in
   outline, what risks it addresses, and what it does not.

2. **A decisions register.** A table of every significant choice, in the form:
   decision · requirement or number that drove it · alternative considered ·
   condition under which the answer would change. This table is worth
   disproportionate marks because it is the artefact Chapter 72 §72.4 argues is
   the point of the exercise.

3. **A cost outline**, capital and operational, at the level of "four access-layer
   switches, one core switch, twelve access points" rather than part numbers.
   Identify the three largest line items and defend each.

4. **A phased implementation outline.** What is built first, what can wait, and
   what the migration from whatever exists today looks like.

5. **A statement of what you would do differently with more information**, naming
   the specific information.

**The defence** is a 15-minute presentation in week 15 followed by questions from
the marker and from another team, whose job is to find the design's weaknesses.
You are marked on the answers, not on the slides. "We did not consider that" is
an acceptable answer and loses fewer marks than a confident wrong one.
