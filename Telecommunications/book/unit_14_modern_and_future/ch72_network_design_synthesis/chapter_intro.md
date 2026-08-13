# Chapter 72 — Network Design: The Synthesis

Everything converges here.

Seventy-one chapters have built mechanisms. This one asks the question that a working
network engineer is actually paid to answer: **given a set of requirements, what should
we build, and why that rather than something else?**

The second half of that question is the chapter's real subject. Producing *a* design is
not difficult; a competent person with a catalogue can produce a design in an
afternoon. Producing a design you can **defend** — where every significant choice has a
reason, the rejected alternatives were genuinely considered, and the tradeoffs are
stated rather than hidden — is the skill that distinguishes an engineer from a
configurator.

## Requirements, and the questions nobody asks

§72.1 covers elicitation, and its central claim is that **most bad networks are the
result of bad requirements rather than bad engineering.**

The requirements a client volunteers are the wrong ones. They say "we need gigabit to
the desktop" and "we need five nines," which are solutions and slogans rather than
requirements. The engineer's job is to get behind them.

The questions that produce useful answers:

**Who communicates with whom, about what?** Not "we have 200 staff" but: sales talks to
the CRM which is in the cloud; the design team moves 5 GB files to a local server all
day; the warehouse scanners send a few hundred bytes every few seconds and must never
lose connectivity while the picker is walking.

**What breaks the business if it stops?** For how long? Ask for the cost per hour of
each system being unavailable. The answers determine the availability budget of
Chapter 56 §56.1, and asking for them converts an argument about nines into an
arithmetic problem.

**What is the growth expectation?** Over what period? A network designed for today's
headcount will be rebuilt in three years.

**Who will operate it?** This is the question most often omitted and it should
constrain the design heavily. A sophisticated design that the available staff cannot
operate will be misconfigured within a year and will fail in ways nobody can diagnose.
**Design for the team you have**, and if that means a simpler architecture with less
theoretical elegance, that is the correct engineering answer, not a compromise.

**What must not happen?** Regulatory constraints, data residency, segregation
requirements. These are hard boundaries, and discovering one late invalidates work.

**What is the budget, capital and operational?** Including the operational cost, which
clients consistently underestimate and which usually exceeds the capital cost over the
life of the network.

## From requirements to design

§72.2 and §72.3 work the derivation, and the point of the ordering is that each stage
constrains the next.

**Sites and their connectivity** — from geography and the WAN options of Chapter 51.
**Topology** — from Chapter 11's hierarchy or Chapter 67's fabric, chosen by traffic
pattern and scale. **Media** — from Chapter 10's decision procedure, driven by
distance, environment, rate and whether the far end needs power. **Capacity** — from
the traffic analysis, with the utilisation headroom that Chapter 3 §3.2's queueing
curve requires. **Addressing** — Chapter 27's plan, designed to summarise (Chapter 26
§26.4) and to accommodate growth. **Segmentation** — Chapter 20's VLANs and Chapter 60's
policy, driven by Chapter 57's threat model. **Services** — DHCP, DNS, NTP, and where
each lives and what happens when it is unreachable. **Wireless** — Chapter 45's
coverage-or-capacity determination. **Security** — designed in rather than added.
**Operations** — monitoring, documentation and change process from Unit XI, specified
as part of the design and not left as an exercise.

The discipline §72.3 insists on is that **addressing, segmentation, routing and
security are one design, not four.** An address plan that ignores the routing design
produces a network that cannot summarise (Chapter 31 §31.4). A segmentation scheme
designed after addressing produces VLANs whose subnets cannot be expressed as a
sensible firewall policy. These are the mistakes that are cheap to avoid at design time
and enormously expensive to correct later, and they are made constantly because the
four are usually done by different people at different times.

## The defence

§72.4 is the chapter's purpose, and it is the form your semester project's final paper
takes.

For every significant choice, the design document must contain the sentence:

> **We chose X rather than Y because...**

And the reason must be a *reason* — traceable to a stated requirement, a computed
number, or an explicit tradeoff — not a preference, a habit, or a vendor
recommendation.

Worked examples of what that looks like:

*"We chose a collapsed core rather than a three-tier design because the site has 340
users across two floors with no expectation of exceeding 500; a distribution layer would
add two devices, four uplinks and a failure domain for no measurable benefit at this
scale. If the second building proceeds in 2028, the distribution layer becomes
justified and the addressing plan in §4 reserves space for it."*

*"We chose Cat6A rather than Cat6 for horizontal cabling despite the current
requirement being 1 Gb/s, because the installation labour is identical, the material
difference is £0.31/m over 4,200 m (£1,302), and it supports 10GBASE-T to 100 m should
the requirement arise within the cabling's 15-year life. We chose not to install fibre
to the desk because no application requires it, no device supports it, and PoE
(Chapter 16) is required for the 34 access points and 51 telephones."*

*"We chose OSPF rather than static routing because the site count is 9 and rising, and
Chapter 30's arithmetic gives 72 route statements to maintain manually across a
topology that changes. We chose OSPF rather than BGP because we operate a single
autonomous system with no policy requirements and no external peering, and BGP's
expressiveness would be unused complexity."*

Note the structure common to all three: a decision, the specific requirement or number
that drove it, the alternative that was considered, and — where relevant — the
condition under which the answer would change. That last element is what makes a
design document useful to the person who inherits it in five years.

## The exercise this chapter is

The chapter's substance is not prose but a worked design, end to end, for the
organisation in the semester project brief: a 50–100 person company on two floors, with
workstations, servers, cloud services, printers, VoIP, employee and guest wireless,
Internet connectivity, remote workers, and room to grow.

Every unit of this book contributes to it, and the exercise is to produce the document
and then defend it against a reader whose job is to ask *why not the other thing?*

That is the job. Everything before this chapter was preparation for it.

## By the end you will be able to

- Elicit requirements that are requirements rather than solutions.
- Derive a design in dependency order, with each stage constrained by the last.
- Design addressing, segmentation, routing and security as one artefact.
- Justify every significant choice against a requirement, a number, or a stated
  tradeoff.
- Identify the condition under which each choice would change.
- Produce and defend a complete design document for a described organisation.
