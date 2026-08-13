# 63.1 Evidence Before Hypothesis

**Instincts under pressure are unreliable in specific, predictable ways.** **Recognising your
own behaviour is most of the fix**, which is why this section names the failure modes before it
offers a method.

## The six failure modes

### Confirmation bias

**You suspect the firewall. You look at the firewall.**

> **There is always something in the firewall logs.** **You find it, and you stop looking.**

**The evidence did not point to the firewall; you pointed at the firewall and then found
evidence.** **And a firewall log at any moment contains denied packets, dropped sessions and
anomalies**, because that is what a firewall does — **so "I found something suspicious" is not
information.**

**The countermeasure is to state, before looking, what you would expect to see if your
hypothesis were true and what you would expect if it were false.** **If the answer to the second
question is "the same thing", the observation cannot distinguish them and looking is a waste.**

### Recency bias

**The last three problems were DNS, so this is DNS.**

**And it is a reasonable heuristic that is wrong often enough to be dangerous**, because **the
base rate has changed and your memory has not.**

**The countermeasure is the change log** (§63.2): **what actually changed is evidence; what
recently broke is not.**

### Changing multiple things at once

**Under pressure you restart the service, clear the ARP cache and reseat the cable. It works.**

> **You now do not know what was wrong**, cannot prevent recurrence, and cannot write a useful
> record. **You have converted a diagnosable fault into a fault that will return.**

**This is Chapter 55 §55.2's "change one thing", under pressure**, and it is harder to obey
during an incident precisely because the incentive is to restore service. **Which is legitimate
— restoring service is the priority — and the compromise is to record what you did in what
order**, so that the sequence is at least reconstructible afterwards.

### Acting before observing

**Rebooting destroys the state that would have identified the cause.**

| Destroyed by a reboot | |
|---|---|
| **Interface counters** | errors, discards, the rate they were accumulating |
| **ARP and MAC tables** | who was where |
| **Connection tables** | what was established |
| **Routing table and adjacency state** | what the device believed |
| **Logs**, on devices with a volatile buffer | Chapter 54 §54.3 |
| **The condition itself** | which may not recur for weeks |

> **The reboot may fix the symptom and it guarantees you will meet the problem again with no
> more information than you have now.**

**The discipline is thirty seconds of capture before any restorative action:** **`show`
the counters, the log tail, the relevant table** — **and on a device where you can, a
`show tech-support` or its equivalent, which takes one command and captures everything.**

### Anchoring on the reporter's diagnosis

> **"The VPN is broken" is a hypothesis presented as an observation.** **The observation is "I
> cannot open the file share."**

**Those are different statements, and starting from the first can waste an hour** — during which
the actual fault, a permissions change on the file server, remains undiagnosed.

**The countermeasure is a question:** **"What exactly did you do, and what exactly happened?"**
— and **listening to the answer without translating it into your own hypothesis as it arrives.**

### The single-cause assumption

**The belief that there is one fault.**

**Frequently there are two**, and **the second is what makes the first hard to diagnose** —
a marginal cable and a duplex mismatch, a DNS problem and a firewall rule. **Chapter 55's Cook
reference: complex systems run in degraded mode**, with multiple latent faults present, **so an
incident is often one new fault interacting with one that was already there.**

**The signal is a fix that improves things and does not resolve them.** **Take that
seriously rather than concluding the fix was wrong.**

## Evidence, and what counts as some

**A working definition:**

> **Evidence is an observation that would have been different if your hypothesis were false.**

| Not evidence | Evidence |
|---|---|
| "There are errors in the log" | **"Error rate rose from 2/hour to 400/hour at 14:07"** |
| "It seems slow" | **"p95 latency to that host is 340 ms; last week it was 22"** |
| "The firewall might be blocking it" | **"The connection counter for that rule incremented when I tested"** |
| "It works from my machine" | **"It works from a machine on VLAN 20 and fails from VLAN 30"** |
| "The link is up" | **"The interface has 0 input errors and 14,000 output discards"** |

**Three properties distinguish the right-hand column:**

**It is a measurement, not an impression.**

**It has a comparison** — against a baseline (Chapter 54 §54.1), against another location,
against a previous time. **A number with nothing to compare it to is not evidence.**

**And it would have come out differently under the alternative hypothesis.**

## The observations to make first

**A defensible opening set, in this order, before forming any hypothesis.**

| | Question | Why first |
|---|---|---|
| **1** | **What exactly is the symptom?** | **not the reporter's diagnosis** |
| **2** | **Who and where is affected?** | **one user, one site, everyone — this eliminates most of the search space** |
| **3** | **When did it last work?** | **bounds the change window** |
| **4** | **What changed?** | **Chapter 55: most outages are caused by changes** |
| **5** | **Is it constant or intermittent?** | **entirely different diagnostic approaches** |
| **6** | **Can I reproduce it?** | **a fault you cannot reproduce is one you cannot confirm you fixed** |

**Question 2 is the highest-value one and is frequently skipped.**

> **"One user" means the client or their port. "One site" means that site's link or a device in
> it. "Everyone, everywhere" means something central.** **Three questions to the service desk
> eliminate more of the search space than an hour of packet capture.**

**And question 5 determines the method entirely:**

| | **Constant** | **Intermittent** |
|---|---|---|
| Reproduce | **easy** | **the hard part** |
| Approach | **bisect the path** | **instrument and wait** |
| Evidence | **direct observation** | **counters, logs, and correlation over time** |
| Danger | — | **a coincidental fix appears to work** |

**The danger in the right-hand column is the one that costs weeks:** **an intermittent fault
that recurs every few days will appear to be fixed by whatever was done most recently**, and
**only a period substantially longer than the fault's interval demonstrates otherwise.**

## Working the problem under pressure

**Aviation's discipline, and it transfers.**

**Aviate, navigate, communicate** becomes:

| | |
|---|---|
| **Stabilise** | **restore service if you can do so without destroying evidence** |
| **Diagnose** | **systematically** |
| **Communicate** | **and this is not optional** |

**Two practical points about the third.**

**Someone must talk to the organisation and it should not be the person diagnosing.**
**An engineer answering "any update?" every four minutes is an engineer not diagnosing**, and
**the incident commander role exists for this reason** — even in a team of three, and even
informally.

**And state what you know, what you do not, and when you will next report.** **"We know the
Manchester site cannot reach the data centre, we do not yet know why, and I will update at
14:30" is a good status.** **"We are investigating" is not.**

## The cost of the reverse order

**Hypothesis before evidence is not merely inefficient; it produces specific, expensive
outcomes.**

**Time spent confirming a wrong theory.** **Which is unbounded, because there is always
something to find.**

**Changes made to a healthy system.** **A firewall rule added to fix a problem the firewall was
not causing** is now a permanent piece of unexplained configuration (Chapter 55 §55.1), **and it
will outlive everyone who remembers why.**

**And a record that misleads the next person.** **"Fixed by restarting the DNS service" enters
the knowledge base** (Chapter 53 §53.4), **and the next occurrence begins by restarting the DNS
service.**

> **The recurring cost of guessing is not the wasted hour. It is the configuration and the
> documentation left behind**, both of which persist.

## What breaks here

**An hour spent on the reporter's stated cause.** **Anchoring.** Ask what happened, not what
they think is wrong.

**Three changes made and the problem resolved.** **You do not know which.** Record the sequence
at minimum.

**A reboot that fixed it, and it returns next month.** **The evidence was destroyed.** Thirty
seconds of capture first.

**A fix that improved things without resolving them.** **Two faults.** Do not conclude the fix
was wrong.

**An intermittent fault "fixed" three times.** **The interval is longer than your observation
period.** Instrument and wait.

**A firewall rule added during an incident that nobody can now justify.** **The cost of guessing,
persisting.**

**"It works from my machine."** **Not evidence until you say which machine, on which network,
to which destination.**

> **Network+ note.** Objective 5.1 covers the methodology and the exam takes it seriously.
> Over-learn: **gather information and identify symptoms before theorising**; **question users
> and determine what changed**; **duplicate the problem if possible**; and **do not change
> multiple things at once.** The "what changed?" question is examined and is the single most
> productive question in practice.
