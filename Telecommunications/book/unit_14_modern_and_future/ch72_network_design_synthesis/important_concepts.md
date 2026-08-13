# Chapter 72 — Important Concepts

**Most bad networks are the result of bad requirements rather than bad engineering** *(§72.1)* —
**And the requirements a client volunteers are the wrong ones**: solutions ("gigabit to the
desktop"), slogans ("five nines") and inherited history. **The response to all three is the same:
ask what would be true if the requirement were met.**

**"Who communicates with whom, about what?" is the highest-value hour of the engagement**
*(§72.1)* — **It produces the traffic matrix, the applications' sensitivities and the
segmentation boundaries simultaneously**, and it is frequently replaced by a headcount. **Ask
about a day, not about the system**: "walk me through what a picker does" produces what "what are
the requirements?" does not.

**Ask for the cost per hour** *(§72.1)* — **It converts an argument about nines into an
arithmetic problem** (Chapter 56 §56.1). **And ask about tolerable duration separately**, because
a finance system costs £600/hour most days and is intolerable to lose on the last day of the
month.

**"Who will operate it?" should constrain the design heavily** *(§72.1)* — **A sophisticated
design that the available staff cannot operate will be misconfigured within a year and will fail
in ways nobody can diagnose.** **Design for the team you have**, and **if that means less
theoretical elegance, that is the correct engineering answer, not a compromise.**

**A requirement is testable; an aspiration is not** *(§72.1)* — **"The network must be fast"
against "a 5 GB file transfers to the design server in under 90 seconds."** **The test is
Chapter 63 §63.2's: can you state, in advance, the observation that would demonstrate it?**

**The source column earns its place** *(§72.1)* — **"Who asked for this?" is asked in every
project**, and a requirement whose source cannot be identified will be argued about. **And a
document in which everything is "must" has no priorities**, so the design will be
over-specified where it was easy.

**The response to unstated requirements is designed-in headroom** *(§72.1, §72.2)* — **There are
always some**, and **a design with no spare capacity, addresses or ports cannot absorb them.**
**50% of address space, 20% of ports, 25% of rack units, 30% of power, 40% of bandwidth** —
**cheapest at build time and most expensive to retrofit.**

**Each derivation stage's output is the next stage's input** *(§72.2)* — **Traffic matrix, sites,
topology, media and capacity, devices, physical** — **and a stage performed out of order produces
a design that must be revisited.**

**Three things fall out of the traffic matrix immediately** *(§72.2)* — **Where the bandwidth is**
(the design team's transfers are local, so they size the LAN and not the WAN); **where the
sensitivity is** (scanners need availability, voice needs jitter, neither needs throughput); and
**where the boundaries are** — **segmentation the requirements produced rather than the security
team imposed.**

**Redundancy is derived from the cost per hour** *(§72.2)* — £50 buys spares on a shelf; £900
buys a redundant core; £4,000 buys redundant everything on the critical path; £40,000 buys two
sites. **Stating it this way is what makes it defensible.**

**Size the upstream first** *(§72.2)* — Video is symmetric, and an asymmetric service's upload is
the number that fails (Chapter 51 §51.4).

**A design that does not state when it expires will be described as having failed** *(§72.2)* —
**Give the date, and what the upgrade costs.** At 40% traffic growth a circuit sized with 40%
headroom is exhausted in year two.

**Install single-mode between buildings even where multimode would suffice** *(§72.2)* — **The
fibre will be in the ground for twenty-five years and the optics will be replaced four times.**
The cheapest decision in the section.

**Buy switches on PoE budget, uplink capacity and security features, not on port count and
price** *(§72.2)* — **48 access points at 25 W is 1,200 W**, and cheap switches lack BPDU guard,
DHCP snooping and DAI (Chapter 62 §62.4).

**The 90 m constraint shapes buildings** *(§72.2)* — **It determines the number and placement of
comms rooms**, and it is discovered late by anyone who designs the logical network first.

**A security policy is trivial with a summarisable address plan and painful without one**
*(§72.3)* — **"The warehouse must not reach finance" is one ACL entry or forty**, and the address
plan was made before anyone asked about the policy.

**The same function at the same offset at every site** *(§72.3)* — **`10.x.64.0/20` is
operational technology everywhere**, so a policy written once applies everywhere and an engineer
reading an address knows what it is.

**Design the IPv6 plan when you design the IPv4 one** *(§72.3)* — **The cost is an afternoon;
retrofitting a coherent IPv6 plan onto a deployed network is the same project as renumbering.**

**"We will microsegment the servers" is frequently written and rarely done** *(§72.3)* — **A
design that says "server-to-server traffic is unrestricted" is better than one claiming a control
that will not exist.**

**What happens to each segment when the WAN fails is a design decision** *(§72.3)* — **A branch
whose DHCP relay points to HQ has no addressing when the circuit drops**, and both "the branch
survives" and "the branch stops" are defensible — **only one of them should be a surprise.**

**Five operational items are consistently absent and consistently needed** *(§72.3)* —
**Out-of-band management, the monitoring, the address record, the change process, the
documentation.** **All operational rather than architectural, which is why they are omitted** —
**and Chapter 56 §56.1's argument is that MTTR is where the availability leverage is**, so
omitting them omits the cheapest availability improvement in the design.

**Producing a design is not difficult; producing one you can defend is the skill** *(§72.4)* —
**Traceable, comparative, honest about trade-offs, and falsifiable** — **and the fourth
distinguishes a design from a proposal.** "At 70% growth the capacity is exhausted in year one,
and the remedy is X costing Y."

**The decision record is what makes it defensible** *(§72.4)* — **Decision, chosen, alternatives,
why, and reversible.** **A design with fewer than about a dozen rows has not recorded its
reasoning.**

**The "reversible?" column is the one nobody includes and the most useful** *(§72.4)* — **An
easily reversed decision deserves less analysis; the address plan, the topology and the vendor
deserve disproportionate attention** — **and conflating the two wastes effort on the wrong
choices.**

**Write "what it does not do" before someone else does** *(§72.4)* — **A design that names its
own limits is trusted; one that is silent is either dishonest or unexamined, and a reviewer
cannot tell which.**

**Convert the cost conversation** *(§72.4)* — **"Is £144,000 of avoided loss worth £60,000 of
investment?" is a question a finance director can answer**, and "why is the network so
expensive?" is not.

**If the alternative would also work, say so** *(§72.4)* — **"X would meet the requirements; we
chose this because of Z" is a stronger position than defending a preference as a necessity.**

**"Which requirement does this element serve, and what happens without it?"** *(§72.4)* — **The
over-engineering test.** **Building the sophisticated architecture for an environment that does
not need it is the most common design error in the field, and an engineer who has just read
seventy-one chapters is unusually susceptible to it.**

**Answer a cost challenge with a trade table, not a refusal** *(§72.4)* — **A client who accepts
a documented risk is in a different position from one who was not told.**

**A design review with no findings is a review that did not happen** *(§72.4)* — **Seek the
reviewer who will disagree**, and ask them what the single point of failure is, what breaks at
10× the traffic, and what this design is worse at than the current one.

**The book's purpose** *(§72.4)* — **Not a catalogue.** **So that when a requirement arrives you
can derive what it implies; when a proposal arrives you can identify which assumption it rests
on; when a fault arrives you can reason from the mechanism; and when someone asks "why that
rather than something else?", the answer is a derivation rather than a preference.**

**The five recurring shapes** *(§72.4)* — **The good idea that lost to economics; the compromise
that outlived its constraint; the mechanism reintroduced after being abandoned; the failure
documented decades before it mattered; and the thing that shipped beating the thing that was
correct.** **None is a criticism of the field** — **they are what engineering looks like when it
is done by people, over decades, under constraints that change** — **and recognising the shape is
what lets you predict which current claim will follow it.**

**The last observation** *(§72.4)* — **Every mechanism in this book was a reasonable answer to a
problem that existed at the time.** **Your job is to know which problem, whether it still exists,
and what it cost to solve — and then to make your own reasonable answer, and to be able to say
why.**
