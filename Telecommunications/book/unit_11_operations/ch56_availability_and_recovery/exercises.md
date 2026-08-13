# Chapter 56 — Exercises

## A. Recall

**A1.** Give the annual downtime for 99%, 99.9%, 99.99% and 99.999% availability, and state the
clause that defeats most five-nines claims.

**A2.** State the availability formula in terms of MTBF and MTTR, and say which of the two
offers more leverage and why.

**A3.** Give the series and parallel availability formulas, and state the assumption the
parallel one makes.

**A4.** What is an error budget, what argument does it replace, and what does an unspent budget
tell you?

**A5.** Define shared fate and give five distinct examples.

**A6.** State the four questions that must be answered before an availability figure means
anything.

**A7.** What problem does an FHRP solve, and why can a host not solve it itself?

**A8.** Why does the virtual MAC address move as well as the virtual IP?

**A9.** Give the default timers for VRRP and HSRP and say what each implies for a voice call.

**A10.** What is FHRP tracking, what failure does it prevent, and what should be tracked in
preference to link state?

**A11.** Why should preemption have a delay?

**A12.** What goes wrong when the spanning tree root and the FHRP master are on different
devices?

**A13.** State the purpose of a UPS when a generator is present, and when one is not.

**A14.** Why does a UPS self-test not establish its runtime?

**A15.** Define RPO and RTO, and state which points backwards and which forwards.

**A16.** Give the four DR test types and say what each finds that the others do not.

## B. Apply

**B1.** Compute the permitted downtime per year, month and week for 99.5%, 99.95% and
99.995%.

**B2.** An organisation patches its firewall pair quarterly. Each patch requires a 20-minute
window with a 3-minute traffic interruption during failover.

(a) What annual availability does the planned work alone permit?
(b) Which availability targets does this rule out?
(c) What would have to change to reach 99.99% including planned work?

**B3.** A service is composed of: access switch 99.99%, distribution 99.99%, firewall 99.95%,
WAN circuit 99.9%, and a server 99.95%.

(a) Compute the end-to-end availability.
(b) Identify the constraining component.
(c) The firewall is made redundant (assume independence). Recompute.
(d) The circuit is made redundant instead. Recompute, and say which investment is better.

**B4.** A device has an MTBF of 18 months.

(a) Compute availability for MTTR of 8 hours, 2 hours and 20 minutes.
(b) Compute availability if MTBF were doubled and MTTR left at 8 hours.
(c) State which intervention is cheaper and why the chapter argues for it.

**B5.** A distribution centre turns over £28,000 per hour and operates 16 hours a day, 6 days a
week.

(a) Compute the annual cost of downtime at 99%, 99.9% and 99.99%, assuming outages occur during
operating hours.
(b) The step from 99.9% to 99.99% costs £85,000 a year. Is it justified?
(c) Repeat (b) for a branch office turning over £900 an hour.

**B6.** A team has a 99.9% monthly target.

(a) What is the monthly error budget in minutes?
(b) After an 18-minute incident and two 4-minute maintenance interruptions, how much remains?
(c) State the policy you would apply for the rest of the month, and the policy if the budget
were entirely unspent by day 25.

**B7.** Design the VRRP configuration for a pair of routers serving VLANs 10, 20, 30 and 40,
with load sharing between them and correct alignment with spanning tree.

(a) State which router is master for which VLAN and which is STP root for which.
(b) Give the priorities.
(c) Specify tracking and the decrement value, showing that it causes a failover.
(d) Specify preemption and justify the delay.

**B8.** A rack contains equipment drawing 2,600 W and is fed by a 16 A, 230 V circuit through a
3,000 VA UPS rated at 0.9 power factor.

(a) What is the circuit's capacity in watts, and what headroom remains?
(b) What percentage of the UPS's capacity is loaded?
(c) Estimate the runtime using the table in §56.3, and comment.
(d) A new 48-port PoE switch supporting 30 access points at 22 W each is proposed. Assess.

**B9.** A 4 kW load sits in a 45 m³ comms room. Cooling fails.

(a) Compute the theoretical rate of temperature rise, ignoring thermal mass.
(b) How long from 21 °C to 40 °C?
(c) The monitoring polls temperature every five minutes with a threshold at 32 °C. Assess
whether this is adequate, and propose an alternative.

**B10.** For each system, propose an RPO and RTO with a one-line justification, and state the
mechanism each implies:

(a) A hospital's patient records system
(b) A university's course catalogue website
(c) A payment processor's transaction ledger
(d) An engineering firm's CAD file server
(e) The network's own configuration repository

**B11.** A synchronous replication link runs to a site 340 km away.

(a) Compute the round-trip propagation delay.
(b) An application performs 2,000 sequential writes to complete a transaction. What does the
replication add?
(c) The business wants the DR site 1,200 km away for regional-event protection. Recompute and
state the design consequence.

## C. Analyse

**C1.** Analyse why each additional nine costs roughly an order of magnitude more than the
last. Identify what class of cause each nine removes, and where the discontinuities are.

**C2.** The chapter argues that halving MTTR beats doubling MTBF. Analyse this rigorously: is it
always true? Construct a case where it is not, and state what determines which lever to pull.

**C3.** Analyse the error budget as a management device. Whose behaviour does it change, in what
direction, and what could go wrong with it? Would it work in an organisation without a strong
measurement culture?

**C4.** "Nothing monitors the component that is not carrying traffic." Analyse this as a general
problem in system design, and identify two other places in this book where the same blind spot
appears.

**C5.** Analyse the trade-off in staggering firmware versions across a redundant pair. What
class of failure does it protect against, what does it cost, and is the chapter's compromise
(stagger in time) actually sound?

**C6.** Analyse why stretched Layer 2 between data centres is repeatedly proposed for disaster
recovery despite being known to be poor. What makes it attractive, what does it actually cost,
and what would you say to someone proposing it?

**C7.** The chapter claims a DR test that finds nothing was not a test. Analyse the incentive
structure that produces such tests, and design a process that resists it.

**C8.** RPO and RTO are described as independent. Analyse the cases where they are coupled in
practice, and say what causes the coupling.

**C9.** Analyse the choice between a DR site 20 km away and one 1,200 km away. Enumerate the
events each protects against and each does not, and argue for a position for a named type of
organisation.

## D. Design

**D1.** Design the availability architecture for a 24-hour manufacturing site where an hour of
network downtime halts production at a cost of £15,000. Specify the target, the redundancy at
each layer, the power arrangement, and the testing regime. Show the arithmetic that justifies
your target rather than a higher or lower one.

**D2.** Conduct a shared-fate analysis for a two-site design: two data centres 8 km apart, each
with dual carriers, dual power feeds and redundant equipment. List every shared fate you can
identify, rank by likelihood × impact, and propose mitigations with rough costs.

**D3.** Design the first-hop redundancy for a campus with a collapsed core: two switch-routers,
eight VLANs, voice on two of them. Specify protocol, priorities, timers, tracking, preemption
and STP alignment. Justify the timer choice against the voice requirement.

**D4.** Specify the power and environmental design for a new comms room supporting 12 kW of
equipment: UPS sizing, generator or not, circuit arrangement, cooling, monitoring and the
testing schedule. State the assumptions you would need the building's owner to confirm.

**D5.** Write a disaster recovery plan for a 400-person organisation with a primary data centre
and a colocation facility 30 km away. Include: RPO/RTO per system class, declaration criteria
and authority, restoration order, the network's addressing approach, the manual workaround, and
the annual test schedule. Keep it to three pages.

**D6.** Design a failover test programme: what is tested, how often, by whom, in what
environment, and how findings are handled. Include the cultural element explicitly and say how
you would prevent tests from being arranged to succeed.

## E. Troubleshoot

**E1.** A redundant pair of WAN circuits both failed at 09:12. Both are from different providers.
Give five possible shared fates and how you would investigate each.

**E2.** A firewall failover occurs and traffic does not resume. The standby is running. Give
three causes and how to distinguish them.

**E3.** Hosts have a working default gateway and cannot reach anything outside their subnet.
The FHRP shows one router as master. Diagnose.

**E4.** Both routers in a VRRP pair report themselves as master. Explain the mechanism, the
symptom users see, and the three most likely causes.

**E5.** A brief outage occurs every time a previously failed router comes back into service.
Explain and give the configuration change.

**E6.** The link between two distribution switches runs at 70% while all other links are below
20%. The design predicts it should be lightly loaded. Diagnose.

**E7.** A power cut occurs. The UPS carries the load for 100 seconds and the equipment shuts
down. The generator did not start. List what should have been tested and when.

**E8.** A switch reports high temperature. The room's temperature sensor reads 20 °C. Explain
both readings.

**E9.** A DR failover completes and users cannot log in, although the application servers are
running. Identify the dependency failure and give the correct restoration order.

**E10.** A DR test is declared successful having found no issues. State three things you would
ask about the test before accepting that conclusion.

## F. Extend

**F1.** Compute your own organisation's or a hypothetical service's cost per hour of downtime,
using real inputs where you can obtain them. Then price the redundancy steps from §56.1 and
determine the availability target the arithmetic supports. Present it in one page.

**F2.** Perform a shared-fate audit on a network you have access to — your home network is
sufficient and will surprise you. Trace power feeds physically, identify what shares a duct,
and list every single point of failure. Rank them by what it would cost to remove each.

**F3.** Configure VRRP or an equivalent between two routers in a lab, with tracking. Fail the
uplink (not the router) and observe whether failover occurs. Then remove tracking and repeat,
and document the black hole.

**F4.** Measure a real FHRP failover: capture traffic during a failover and determine how long
traffic was lost, using packet timestamps rather than the protocol's own logs. Compare with the
configured timers.

**F5.** Load-test a UPS if you have access to one, or read its documentation and determine what
its self-test actually measures. Report the difference between the rated and the tested runtime.

**F6.** Write and execute a walkthrough test of a DR plan — your own, your organisation's, or a
published example. Record every point at which the plan is ambiguous, out of date, or assumes
knowledge. Report the count.

**F7.** Investigate a published outage in which redundancy failed because of shared fate. Map
the failure, identify what analysis would have found it in advance, and estimate what the
mitigation would have cost.

**F8.** Read a cloud provider's availability zone documentation and determine what is genuinely
independent between zones and what is shared. Then find a published incident in which the shared
component failed.
