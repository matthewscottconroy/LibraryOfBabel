# Chapter 72 — Exercises

## A. Recall

**A1.** State the chapter's claim about the origin of bad networks.

**A2.** Give three reasons the requirements a client volunteers are the wrong ones, and the
single response that addresses all three.

**A3.** State the seven elicitation questions.

**A4.** Why does "who communicates with whom, about what?" determine four separate design
outputs? Name them.

**A5.** Why should the cost per hour be asked for, and what does it convert?

**A6.** Why is "who will operate it?" the question most often omitted, and what should follow
from the answer?

**A7.** What distinguishes a requirement from an aspiration, and what is the test?

**A8.** Give the four columns of a usable requirements table and say why the "source" column
earns its place.

**A9.** State the six-stage derivation from requirements to topology, in order.

**A10.** What three things fall out of a traffic matrix immediately?

**A11.** Why should the upstream be sized first?

**A12.** Why install single-mode between buildings even where multimode would suffice?

**A13.** Give the recommended headroom for address space, ports, rack units, power and
bandwidth, with the reason for each.

**A14.** Why must addressing, services and security be designed together? Give two specific
dependencies.

**A15.** Give the four properties of a structured address plan and say which is most valuable.

**A16.** Name the five operational items that are consistently absent from designs.

**A17.** Give the four properties of a defensible design, and say which distinguishes a design
from a proposal.

**A18.** What does the "reversible?" column in a decision record achieve?

**A19.** Why is stating a design's limits in your own interest?

**A20.** What does a design review with no findings indicate?

## B. Apply

**B1.** For each volunteered requirement, state the question you would ask to get behind it, and
two possible actual requirements:

(a) "We need gigabit to the desktop"
(b) "We need five nines"
(c) "We need SD-WAN"
(d) "The Wi-Fi needs to be better"
(e) "We need a next-generation firewall"
(f) "We want to be cloud-first"

**B2.** Rewrite each aspiration as a testable requirement:

(a) "The network must be fast"
(b) "It must be secure"
(c) "It must be resilient"
(d) "It must be easy to manage"
(e) "It must support growth"

**B3.** Build the traffic matrix for a described organisation: a 90-person architecture practice
with a 12-person modelling team moving 20 GB files to a local render farm, a cloud CAD platform,
IP telephony, and a plotter room. Then state which three design decisions the matrix determines.

**B4.** Size the WAN circuit for a 350-person office: 30% concurrent video at 2.5 Mb/s, 12%
concurrent voice at 0.1 Mb/s, and 1.8 Mb/s average for everything else.

(a) Compute the requirement.
(b) Add 40% headroom.
(c) At 35% annual traffic growth, when is it exhausted?
(d) State what you would put in the design document about (c).

**B5.** Design the address plan for an organisation with 10.0.0.0/8, 25 sites growing to 60, and
seven functional categories per site.

(a) Give the allocation scheme.
(b) State how much is reserved at each level.
(c) Give the IPv6 equivalent from a /32.
(d) State the single ACL entry that would block all operational technology at all sites from
reaching all finance systems.

**B6.** Write the decision record rows for these five choices, including alternatives, reasoning
and reversibility:

(a) Routing protocol for a six-site organisation
(b) Wireless architecture for a warehouse with roaming scanners
(c) Internet edge redundancy for a business losing £1,200 per hour
(d) Whether to deploy 802.1X
(e) Whether to build a leaf–spine fabric for 40 servers in two racks

**B7.** For the design in B4, write the "what it does not do" section: at least five honest
limits.

**B8.** A client asks you to reduce a £140,000 design by £40,000. Produce the trade table: at
least four options, each with the saving and the consequence, expressed so the client can
decide.

## C. Analyse

**C1.** Analyse why "who will operate it?" constrains the design more than most technical
requirements. Construct a case where the correct engineering answer is a less capable
architecture.

**C2.** The chapter says a design that does not state when it expires will be described as having
failed. Analyse this, and propose how a design should express its own lifespan.

**C3.** Analyse the claim that addressing, services and security must be designed together. Take
one of the three, design it in isolation, and identify every subsequent decision it constrains.

**C4.** Analyse the "reversible?" column as a decision-making heuristic. Where else in this book
would it apply, and what does it imply about how review effort should be allocated?

**C5.** The chapter says a well-read engineer is unusually susceptible to over-engineering.
Analyse why, and propose a personal check that would catch it.

**C6.** Analyse the five recurring shapes named in §72.4. For each, find an example from this
book other than the ones cited, and state what recognising the shape would let you predict.

**C7.** "Every mechanism in this book was a reasonable answer to a problem that existed at the
time." Analyse this claim against three mechanisms you consider poorly designed, and say whether
it holds.

**C8.** Analyse the relationship between this chapter and Chapter 63. Both are about reasoning
under uncertainty with incomplete information. What do they share, and where do they differ?

## D. Design

**These five are the chapter's real assessment. Each should produce a document of the shape
§72.4 describes, and the decision record and the limits section are what is being assessed.**

**D1.** Design the complete network for a 220-person professional services firm on three floors
of one building, with an office of 15 in another city, a cloud-first application strategy, one
legacy on-premises system, and two IT staff. An hour of complete outage costs approximately
£3,500. Growth expectation: 10% annually in staff, 35% in traffic, one additional office within
three years.

**D2.** Design the network for a manufacturing site: 400 staff, a production hall with 60
machines on an existing fieldbus, 40 roaming scanners, environmental monitoring, a 24-hour
operation where a production stoppage costs £18,000 per hour, and an OT team that will not permit
IT to touch the production network. Address the OT/IT boundary explicitly.

**D3.** Design the network for a 900-pupil secondary school: high-density wireless, a
requirement to filter and log, a strict capital budget, one technician, and a summer holiday as
the only maintenance window. State what you would deliberately not do.

**D4.** An organisation has grown by acquisition to 14 sites with four different vendors, three
overlapping address plans and no documentation. Design the remediation programme rather than the
end state: what you would do first, in what order, over two years, and how you would demonstrate
progress.

**D5.** Take a network you know — your employer's, your institution's, or your own home — and
write the design document that should exist for it. Include the decision record for the choices
that were actually made, the limits, and the risks. Where you cannot determine why a decision was
made, record that as a finding.

## E. Troubleshoot

**These are design failures rather than operational ones.**

**E1.** A design meets every stated requirement and the client rejects it. Give four possible
causes and the elicitation question that addresses each.

**E2.** A network is delivered on budget and is unaffordable to operate within two years.
Diagnose the design failure.

**E3.** A sophisticated design is misconfigured within a year and the team cannot diagnose faults
in it. Identify the omitted question and state what should have been done differently.

**E4.** A regulatory requirement is discovered during implementation and invalidates the
segmentation design. Analyse what was missed and what it costs.

**E5.** A design's WAN capacity is exhausted eight months after delivery. The growth assumption
was documented. Assess whether this is a design failure.

**E6.** A design claims microsegmentation and it was never implemented. Analyse the consequences
— technical, and for the engineer's credibility.

**E7.** A design review produces no findings and the network fails in production six months
later. Analyse the review process.

**E8.** A design is challenged with "why not just use X?" and the engineer cannot answer. Analyse
what is missing from their document.

## F. Extend

**F1.** Conduct a real requirements elicitation: interview someone about a system or a network
they depend on, using §72.1's seven questions. Record what they volunteered first, what the
questions produced, and the gap between the two.

**F2.** Take a published network design — a vendor's reference architecture, a validated design
guide — and produce its decision record: for each significant choice, infer the reasoning and
the alternatives. Report which decisions you could not explain.

**F3.** Find a network design you or a colleague produced previously. Write the "what it does not
do" section that was omitted. Assess whether writing it at the time would have changed the
design.

**F4.** Cost a design over five years, capital and operational, including the items §72.1 lists
as frequently omitted. Compare the total with the capital figure and report the ratio.

**F5.** Present a design to someone from a non-technical background and record every question
they asked. Assess how many were answerable from your document and how many required you to
explain something the document should have contained.

**F6.** Review a peer's design against §72.4's seven review questions. Produce findings. Then
have them review yours, and compare what each of you found that the other did not.

**F7.** Take three decisions from a design you have made and classify each on the reversibility
scale. Then estimate the effort you actually spent on each, and assess whether it was allocated
correctly.

**F8.** Choose one of the five recurring shapes in §72.4 and write a short essay tracing it
through three chapters of this book. Then identify a current claim in the industry that has the
same shape, and predict its outcome.
