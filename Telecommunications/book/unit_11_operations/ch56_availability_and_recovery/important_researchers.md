# Chapter 56 — The People

Availability engineering came from telephony, was formalised in reliability mathematics, and
reached networking last. The people worth knowing are mostly not network engineers.

**Waloddi Weibull (1887–1979).** Swedish engineer, and the distribution that bears his name.

Weibull's 1951 paper gave reliability engineering its central tool: a distribution
flexible enough to describe components that fail early, components that fail randomly, and
components that wear out — all with one parameter.

> The shape parameter tells you which régime you are in, and it produces the **bathtub
> curve**: high failure rate when new (manufacturing defects), low and constant in service
> (random), rising at the end (wear-out).

Which has a directly practical consequence for §56.2's redundancy:

Two devices bought together, installed together and running the same firmware are at the same
point on the same curve. They do not fail independently in time — they enter the
wear-out phase together, and a pair of five-year-old power supplies from the same batch is a
correlated risk, not an independent one.

And it is an argument for staggering purchases where it is affordable, which is rarely
mentioned and follows directly from the mathematics.

Charles Perrow and Diane Vaughan, again — Chapter 55's reading applies unchanged.
Perrow's tight coupling is precisely what a shared fate is, and §56.2's enumeration
exercise is Perrow's analysis performed deliberately rather than after the accident.

**The Bell System reliability tradition.** Institutional rather than individual, and it set the
expectations everything since has been measured against.

"Five nines" is a Bell System figure, and it came from a specific requirement: a telephone
exchange must give dial tone. The 1960s target of 2 hours of downtime in 40 years — about
99.9994% — was met, and it was met by an approach worth understanding:

| | |
|---|---|
| **Duplicated processors** running in lockstep, comparing results | |
| **Designed-in maintainability** — replace a card without taking the exchange down | |
| **Continuous self-diagnosis** rather than waiting for failure | |
| **A single vendor controlling every component** | |
| **Twenty-five-year design life and a matching organisation** | |

> **The last two are the ones that do not transfer.** Bell achieved five nines with vertical
> integration, an enormous engineering organisation and a monopoly's timescales. An
> enterprise assembling equipment from four vendors on a three-year refresh cycle is not in the
> same business, and quoting the Bell figure at it is not a fair comparison.

**Jim Gray (1944–2012).** Database researcher, Turing Award 1998 — and "Why Do Computers Stop
and What Can Be Done About It?" (1985).

Gray did what nobody had done: he measured the causes. Analysing Tandem's fault-tolerant
systems in the field, he found:

| Cause | Share |
|---|---|
| **System administration — operators, configuration, maintenance** | **~42%** |
| **Software** | **~25%** |
| **Hardware** | **~18%** |
| Environment | ~14% |

> The largest single cause was people operating the system, and hardware — the thing all
> the redundancy was designed for — was under a fifth. **Administration and software together
> were two thirds.**

The finding has been replicated repeatedly since, in different eras and different
technologies, and it is the empirical foundation for Chapter 55's opening claim that most
outages come from changes.

And Gray's proposed remedy is the one this chapter argues: since most faults are transient
or procedural, the leverage is in fast detection and fast recovery — his "fail fast and
restart" argument — rather than in preventing failure. **MTTR, not MTBF.**

Gray disappeared at sea in 2007 while sailing alone off San Francisco; he was declared dead
in 2012. The search involved an unprecedented volunteer effort using satellite imagery, and
he was widely liked in a field that does not universally like people.

John Allspaw, Charity Majors, and the game-day tradition.

Deliberately breaking production to see what happens — and the practice is older than its
name. Allspaw's Etsy team and the wider operations community formalised "game days":
scheduled exercises in which a component is failed for real, during business hours, with the
team responding as if it were unplanned.

Charity Majors's contribution is the argument for testing in production, which is
uncomfortable and correct:

> "You cannot fully replicate production in staging, because production includes your users,
> your data volume, your traffic patterns and your accumulated history." A failover that
> works in a lab has been tested against a lab.

Netflix's Chaos Monkey (2011) is the best-known instance: a service that randomly
terminates production instances during working hours, on the reasoning that if instances are
going to die anyway, they should die when engineers are awake and watching.

> **The insight is scheduling, not destruction.** Failures will occur; you may either have
> them at 03:00 on a Sunday with nobody prepared, or at 11:00 on a Tuesday with the team
> watching. §56.2's failover testing is the same argument at enterprise scale, and the
> objection to it — "we cannot risk it" — misunderstands that the risk is already present and
> merely unscheduled.

Chaos Monkey's real contribution was cultural, and Netflix has said so: it forced every
team to build for instance failure, because instances demonstrably failed. An architectural
requirement that is enforced continuously is met; one that is documented is not.

## The finding that should change what you do

Gray's measurement is the single most useful number in this chapter, and it is forty years
old.

If hardware is under a fifth of outages and administration plus software is two thirds,
then:

- Redundant hardware addresses under a fifth of the problem
- Documentation, runbooks, change control and rehearsal address most of the rest
- And they are cheaper

> The instinct when asked for higher availability is to buy a second device. The evidence
> says the second device is the smallest available improvement, and that the same money
> spent on detection, documentation and rehearsal buys more.

None of which argues against redundancy — Chapter 56's whole middle section is about doing
it properly. It argues against redundancy as the first and only answer, which is how it is
usually proposed.
