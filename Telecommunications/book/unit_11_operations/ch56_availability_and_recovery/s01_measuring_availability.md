# 56.1 Measuring Availability

"We need five nines" is said in meetings constantly, usually by someone who has not
computed what it means. The arithmetic converts an aspiration into a budget, which is why it
is worth having at hand.

## The table

| Availability | **Per year** | Per month | Per week |
|---|---|---|---|
| 99% | **3.65 days** | 7.3 hours | 1.7 hours |
| **99.9%** | **8.76 hours** | **43.2 min** | **10.1 min** |
| 99.95% | 4.38 hours | 21.6 min | 5.0 min |
| **99.99%** | **52.6 min** | **4.3 min** | **1.0 min** |
| **99.999%** | **5.26 min** | **26 sec** | **6 sec** |
| 99.9999% | 31.6 sec | 2.6 sec | 0.6 sec |

Read the five-nines row again, and note the clause that defeats most claims:

> **5.26 minutes of downtime per year — including all planned maintenance.**

An organisation that reboots its firewalls quarterly for patching (Chapter 55 §55.3) has
already spent its annual budget several times over on planned work, before anything has
failed.

And the honest response to "we need five nines" is a question: does the target exclude
planned maintenance? If it does, it is a different and much weaker claim, and it should be
stated as one — "99.999% excluding scheduled windows" is achievable and is not what the
listener heard.

## What each nine costs

Roughly an order of magnitude more than the last, and the reason is that each successive
nine removes a different class of cause.

| Target | Requires |
|---|---|
| **99%** | **working equipment and someone to fix it during business hours** |
| **99.9%** | **redundant critical components; out-of-hours response; tested backups** |
| **99.99%** | **full redundancy including power; automated failover in seconds; no single maintenance window that takes anything offline; staff continuously available** |
| **99.999%** | **all of the above, plus site redundancy, plus a change process so rigorous it slows everything, plus rehearsed failover** |
| **99.9999%** | **realistically, not an enterprise network** |

> The step from 99.9% to 99.99% is where automation replaces people, because 43 minutes a
> month permits a human to be woken and to log in; 4 minutes does not.

And the step to 99.999% is where the change process itself becomes the limiting factor —
5 minutes a year does not permit a failed change, so every change must be reversible in
seconds or performed on a component that is out of service. This is why the highest-
availability systems change slowly, and why that is a cost rather than a virtue.

## The conversation that should happen instead

> The honest engineering conversation is not "how many nines can we have" but "what does an
> hour of downtime cost us, and at what point does prevention cost more than the outage?"

**Work it as arithmetic.**

A retail site turning over £2,000 an hour in trading:

| Availability | Downtime/year | **Cost of downtime** |
|---|---|---|
| 99% | 87.6 h | **£175,200** |
| **99.9%** | **8.76 h** | **£17,520** |
| 99.99% | 0.88 h | **£1,752** |

**Now the other column:**

| Step | Typical cost |
|---|---|
| 99% → 99.9% | **a second circuit and an LTE backup: ~£3,600/year** |
| 99.9% → 99.99% | **redundant hardware, UPS, out-of-hours cover: ~£40,000/year** |
| 99.99% → 99.999% | **a second site, or nothing at this scale** |

The first step saves £157,000 for £3,600. Obviously correct.

The second saves £15,768 for £40,000. Obviously wrong — **at this site.** At a distribution
centre turning over £40,000 an hour, the same step saves £315,000 and is obviously right.

> The number that decides the design is the cost of an hour, and it is a business figure
> rather than a technical one. **An engineer who designs without it is guessing**, and asking
> for it is the most useful question at the start of an availability discussion.

Frequently the correct answer is three nines with a fast, rehearsed recovery, and saying so
is better engineering than delivering four nines that nobody needed.

## MTBF, MTTR, and where the leverage is

$$A = \frac{\mathrm{MTBF}}{\mathrm{MTBF} + \mathrm{MTTR}}$$

| | |
|---|---|
| **MTBF** — mean time between failures | how often it breaks |
| **MTTR** — mean time to repair | **how long it stays broken** |

Two ways to raise availability, and they are not equally accessible.

**Increase MTBF.** Better equipment, better environment, fewer changes. Expensive, slow,
and largely bought rather than built — you cannot make a switch fail less often by trying
harder.

**Decrease MTTR.** Spares on site, documentation (Chapter 53), runbooks, monitoring that
detects rather than waits for a user (Chapter 54), automation, rehearsed procedures.

**Work the arithmetic:**

| MTBF | MTTR | Availability |
|---|---|---|
| 1 year | **4 hours** | **99.954%** |
| 1 year | **30 min** | **99.994%** |
| **2 years** | 4 hours | 99.977% |
| 1 year | **5 min** | **99.9990%** |

> **Halving MTTR does more than doubling MTBF, and it is far cheaper.** Reducing repair time
> from four hours to thirty minutes takes a device from 99.95% to 99.99% — a full nine — and
> the mechanisms are documentation, spares and rehearsal rather than better hardware.

Which reframes most of this book's operational chapters. Chapter 53's runbooks,
Chapter 54's alerting and Chapter 55's tested rollbacks are all MTTR reductions, and MTTR is
where the leverage is.

## Measuring it honestly

Four questions that must be answered before a number means anything.

**Available to whom?** A service that is up in the data centre and unreachable from a branch
is down, and measuring at the data centre says otherwise. Measure from where the user is
(Chapter 54 §54.4).

**Available for what?** A web server that responds to a health check and returns errors to
real requests is "up" by most monitoring. Synthetic transactions that exercise the actual
function are the answer, and they are more work.

**Partial counts how?** A service degraded for everyone, or fully down for 10% of users?
Neither is captured by a binary up/down, and user-minutes lost is the better measure:

$$\text{user-minutes lost} = \text{affected users} \times \text{duration}$$

Which correctly ranks "everyone down for 5 minutes" against "10% down for an hour".

**Planned included?** Stated above, and it is the question most often left ambiguous —
deliberately, in some vendor SLAs.

## Error budgets

The most useful framing to come out of the last decade, and it converts an argument into
arithmetic.

> If the target is 99.9%, then 0.1% of failure is budgeted, and spending it is permitted.

| | |
|---|---|
| **Target** | 99.9% |
| **Budget** | **43.2 minutes per month** |
| **Spent so far this month** | 12 minutes |
| **Remaining** | **31 minutes** |

**And the policy attached to it:**

- **Budget remaining** → ship changes; take risks; the budget exists to be used
- **Budget exhausted** → freeze non-essential change until the next period

**Why this is better than the alternative:** the usual argument is operations saying "no
changes, they cause outages" against delivery saying "we must ship." Both positions are
reasonable and neither is falsifiable. The error budget makes it a measurement.

And it removes the perverse incentive of over-delivery. A team consistently achieving
99.99% against a 99.9% target is not doing well; it is over-investing — spending money on
reliability nobody asked for, and probably shipping too slowly. The budget being unspent is
information.

## Availability of a system, not a component

The arithmetic that surprises people, and it is why component-level targets mislead.

**Components in series — all must work:**

$$A_{\text{total}} = A_1 \times A_2 \times \cdots \times A_n$$

| Path | Availability |
|---|---|
| Switch 99.99% × router 99.99% × firewall 99.99% × circuit 99.9% | **99.87%** |

> Four components at very high availability produce a system below the weakest of them.
> The circuit at 99.9% dominates, and improving the switches achieves nothing.

**Components in parallel — any one suffices:**

$$A_{\text{total}} = 1 - (1 - A_1)(1 - A_2)$$

| | |
|---|---|
| Two circuits at **99.5%** each | **99.9975%** |
| Two circuits at 99.9% each | **99.9999%** |

Which looks wonderful, and §56.2 is entirely about why it is usually not true.

> **The parallel formula assumes independence.** In practice the two circuits share a duct, a
> building entry, a power feed or a carrier, and the real availability is far closer to the
> single-component figure than the arithmetic suggests.

## What breaks here

A five-nines requirement with a quarterly patching window. **Arithmetically impossible.**
Say so, with the table.

A vendor SLA of 99.99% that excludes planned maintenance, force majeure, and anything
attributable to the customer. **Read the exclusions.** The headline number is frequently the
smallest part of the document.

Availability measured at the data centre and users complaining. Measured in the wrong
place.

A health check passing while the service fails. The check tests reachability, not
function. Synthetic transactions.

An SLA credit that does not cover the loss. Chapter 51 §51.1 — credits are a percentage of
the circuit charge, not compensation. The SLA buys attention, not indemnity.

Investment in a component that is not the constraint. **Series arithmetic.** Find the
weakest term first.

A team hitting 99.99% against a 99.9% target and shipping slowly. **Over-investment.** The
unspent error budget is telling you something.

> **Network+ note.** Objective 3.3 covers availability concepts. Over-learn: availability is
> expressed in nines and converts to downtime per year; MTBF measures time between
> failures and MTTR time to repair; **RPO and RTO are recovery objectives** (§56.4); and **an
> SLA defines the committed level.** The nines-to-downtime conversion is examined regularly and
> is worth being able to compute rather than recall.
