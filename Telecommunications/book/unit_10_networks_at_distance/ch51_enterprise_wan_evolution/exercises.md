# Chapter 51 — Exercises

## A. Recall

**A1.** State the full-mesh circuit count for *n* sites, and explain why hub-and-spoke
dominated enterprise networking for thirty years.

**A2.** Define PVC, DLCI, CIR and DE, and say what commercial mechanism CIR implements.

**A3.** What did an enterprise buy when it bought MPLS L3VPN? List five things, and say which
justified the price.

**A4.** Define tromboning and state the three distinct costs it imposes.

**A5.** Name the three forces that displaced MPLS, and say how many of them are technical.

**A6.** Describe SD-WAN in one sentence using only terms defined earlier in this book.

**A7.** State three things SD-WAN does not do.

**A8.** What is SASE, and what problem created by local breakout does it address?

**A9.** Give the four motivations for direct cloud interconnect, and say which is usually
decisive.

**A10.** Why does a single direct cloud connection typically carry no meaningful SLA?

**A11.** State the zero-trust principle in one sentence, and say what makes a VPN's premise
expired.

**A12.** What is the first diagnostic question to ask a remote worker reporting slowness, and
why?

## B. Apply

**B1.** Compute the full-mesh circuit count for 6, 12, 30 and 80 sites.

(a) At £450 per circuit per month, give the monthly cost of each full mesh.
(b) Give the hub-and-spoke count and cost for each.
(c) For 30 sites, express the saving as a percentage and state what it costs operationally.

**B2.** A branch has a 2 Mb/s access circuit with a 512 kb/s CIR.

(a) What is guaranteed?
(b) A sustained 1.5 Mb/s transfer runs during a congested period. What should the customer
expect?
(c) The carrier's network is quiet. What should they expect?
(d) Explain why the carrier can sell this profitably, referring to Chapter 9.

**B3.** Compare two options for a 40-site enterprise:

- MPLS: 20 Mb/s per site at £800/month
- Broadband + SD-WAN: 500 Mb/s per site at £80/month, plus £120/site/month for SD-WAN
licensing and support

(a) Give the annual cost of each.
(b) Give the cost per Mb/s for each.
(c) State three things the cheaper option does not provide.
(d) At what SD-WAN licence cost per site do the two break even?

**B4.** A Manchester branch reaches a cloud service in Dublin. Direct distance 350 km; the
backhauled path goes Manchester → London → Dublin, 1,100 km total.

(a) Compute the propagation delay each way for both paths, using fibre at 4.9 µs/km.
(b) Give the round-trip difference.
(c) The application performs 12 sequential round trips to load a page. What is the user-visible
difference?

**B5.** A company egresses 60 TB per month from AWS. Internet egress is $0.09/GB; Direct
Connect egress is $0.02/GB. A 1 Gb/s hosted connection costs $900/month all-in.

(a) Compute the monthly egress cost each way.
(b) Compute the total monthly cost each way.
(c) Find the break-even egress volume.
(d) The company also egresses 15 TB/month from Azure to AWS. Explain what this costs and why.

**B6.** Size the WAN circuit for a 120-person branch: 30% on concurrent HD video calls at
2.5 Mb/s, 15% on voice at 100 kb/s, and everyone else averaging 1.5 Mb/s.

(a) Compute the requirement.
(b) Add 40% headroom.
(c) The available service is 900 Mb/s down / 50 Mb/s up. Assess it.
(d) State what you would measure to check your assumptions after installation.

**B7.** Two WAN links each have 99.5% availability.

(a) Compute the combined availability assuming independence.
(b) Express both as minutes of downtime per month.
(c) The two links share a duct into the building, which accounts for 30% of outages. Recompute
the effective availability and comment.

## C. Analyse

**C1.** §51.1 states that only one of the three forces that displaced MPLS was technical.
Identify which, and argue whether a technically superior MPLS would have survived. What does
this suggest about how infrastructure decisions are actually made?

**C2.** Frame Relay defined FECN and BECN in 1990 and equipment largely ignored them; IP
defined ECN and deployment took twenty years. Analyse why explicit congestion notification is
so hard to deploy, referring to Chapter 38 §38.3.

**C3.** SD-WAN is described as containing nothing new. List its component mechanisms, name where
each appeared earlier, and argue whether "combining existing mechanisms under central policy"
constitutes genuine innovation.

**C4.** Duplicating a voice call across two paths doubles its bandwidth consumption to
eliminate loss. Analyse when this trade is correct and when it is not, and derive a general
rule from Chapter 5's error-control argument.

**C5.** SASE moves security inspection from branch appliances to a provider's cloud. Analyse
the consequences for: operational burden, latency, trust, availability, and what happens during
a provider outage. State whether you would adopt it and under what conditions.

**C6.** "The cloud is cheap to enter and expensive to leave." Explain the pricing mechanism
behind this, and analyse its architectural consequences for multi-cloud strategies and for exit
planning.

**C7.** Assess the claim that zero trust makes the VPN obsolete. Be specific about which
workloads it covers, which it does not, and what a realistic transition looks like for an
organisation with twenty-year-old applications in a data centre.

**C8.** The remote worker is described as "a branch office with a budget of zero and no site
survey." Analyse which branch design principles transfer, which do not, and what replaces the
ones that do not.

## D. Design

**D1.** Design the WAN for a retailer with 180 stores, 3 distribution centres and 1 head office.
Stores run point-of-sale, card payment, stock lookup and staff Wi-Fi; an hour of downtime costs
a store roughly £2,000 in lost trading. Specify connectivity, redundancy, breakout policy and
security architecture. Justify the cost of your redundancy against the outage cost.

**D2.** An engineering firm with 12 sites is migrating from MPLS to SD-WAN. Two sites run a
real-time simulation cluster that requires deterministic sub-10 ms latency between them. Design
the transition, including what you would not migrate and why, and write the paragraph you would
give the finance director explaining the retained MPLS spend.

**D3.** Design cloud connectivity for an organisation using AWS, Azure and one SaaS provider,
with 80 TB of monthly egress and a compliance requirement that customer data must not traverse
the public Internet. Include redundancy, failover behaviour, and the interaction with an
existing SD-WAN.

**D4.** A healthcare organisation is moving 3,000 staff to permanent hybrid working, with
clinical systems split between a data centre and a SaaS provider. Design the remote access
architecture. Address: identity, device posture, the residual data centre applications, what
happens when the identity provider is unavailable, and how you would support users you cannot
see.

**D5.** Write the branch connectivity standard for an organisation with 60 sites of varying
size: define site tiers, and for each tier specify bandwidth, redundancy, breakout policy,
equipment and the testing regime. Keep it to two pages and make every requirement justifiable.

## E. Troubleshoot

**E1.** A branch reports poor performance to a cloud application while its WAN circuit shows
30% utilisation. Give the likely cause and the measurement that confirms it.

**E2.** After enabling SD-WAN, one application is consistently routed over the wrong path. Give
four possible causes, ordered by likelihood, and the check for each.

**E3.** A branch's LTE backup fails to carry traffic during a primary outage. List five
possible causes and say which is most common.

**E4.** Voice quality degrades across all sites simultaneously; SD-WAN dashboards show all paths
green. Explain how both can be true and what you would measure.

**E5.** A company installs Direct Connect and its AWS egress charges do not fall. Diagnose.

**E6.** After a direct connect failover to VPN, some connections work and others hang. Give the
likely mechanism and the specific device class to examine.

**E7.** A partner bank blocks access from three branches after an SD-WAN migration. Explain and
give two remedies.

**E8.** A remote user reports slowness. A speed test shows 400 Mb/s. Applications remain slow.
Give the three things you would check next, and say what the speed test has and has not ruled
out.

**E9.** An SD-WAN controller becomes unreachable for six hours. Describe what should still work,
what should not, and what you would verify afterwards.

## F. Extend

**F1.** Obtain your organisation's or a public example WAN circuit pricing, and build a
spreadsheet comparing MPLS, broadband + SD-WAN, and a hybrid, over five years for 25 sites.
Include installation, licensing, hardware refresh and support. Present the result as you would
to a finance director, in one page.

**F2.** Model the cloud egress cost for a workload you know or can specify. Compare Internet
egress, direct interconnect, and an interconnect exchange. Identify the break-even points and
the assumptions your answer is most sensitive to.

**F3.** Build a two-site SD-WAN in a lab using an open-source implementation (for example
OpenWrt with WireGuard and a policy script, or a vendor trial). Introduce loss on one path with
`tc netem` and observe the steering behaviour. Document what it did and how quickly.

**F4.** Read one SD-WAN vendor's published architecture document and one independent analysis of
the same product. List every claim in the vendor document that the independent analysis
qualifies, and write a paragraph on what that tells you about reading vendor material.

**F5.** Measure the tromboning penalty in a network you have access to: compare the latency to a
cloud service from a device behind a corporate VPN and from the same device without it. Explain
the difference in terms of the path taken.

**F6.** Read the NIST zero trust architecture publication (SP 800-207). Summarise its core
tenets in one page, and assess honestly how much of a network you know actually meets them.

**F7.** Investigate what an SD-WAN vendor's edge device does when it cannot reach the
controller. Find the answer in documentation rather than marketing, and write down exactly what
continues and what stops.
