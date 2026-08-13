# 72.1 Requirements Elicitation

> **Most bad networks are the result of bad requirements rather than bad engineering.**

**And the requirements a client volunteers are the wrong ones.** **"We need gigabit to the
desktop" and "we need five nines" are solutions and slogans**, and **the engineer's job is to get
behind them.**

## Why the volunteered requirements are wrong

**Three reasons, and each needs a different response.**

**They are solutions, not requirements.** **"Gigabit to the desktop" is an answer.** **The
question it answers might be "the design team's file transfers take too long"**, which has other
answers — a faster server, a local cache, a different workflow — **and might be nothing at all,
because the constraint is the WAN.**

**They are inherited.** **"We have always used vendor X" and "the last consultant said we needed
a core switch" are history rather than requirements**, and **they are frequently correct and
should still be examined.**

**And they are aspirational.** **"Five nines" is stated by people who have not computed what it
means** (Chapter 56 §56.1), **and the arithmetic — 5.26 minutes a year including planned
maintenance — usually ends the conversation productively.**

> **The response to all three is the same: ask what would be true if the requirement were met.**
> **"If we had gigabit to the desktop, what would be different?"** **The answer is the actual
> requirement.**

## The seven questions

### 1. Who communicates with whom, about what?

**Not "we have 200 staff."**

| Poor | Useful |
|---|---|
| "200 users" | **"Sales talks to a CRM in the cloud; the design team moves 5 GB files to a local server all day; the warehouse scanners send a few hundred bytes every few seconds and must never lose connectivity while the picker is walking."** |

**Which produces, directly:**

- **A traffic matrix** — who to whom, how much, in which direction (§72.2)
- **The applications' sensitivities** — bandwidth, latency, jitter or loss (Chapter 66 §66.1)
- **The segmentation boundaries** — because "who talks to whom" is also "who need not"
  (Chapter 60 §60.4)

> **This one question, answered properly, determines the topology, the capacity, the QoS policy
> and the security zones.** **It is the highest-value hour of the whole engagement**, and it is
> frequently replaced by a headcount.

**And the useful technique is to ask about a day rather than about the system:** **"walk me
through what a picker does from arriving to leaving"** produces information that "what are the
warehouse's requirements?" does not.

### 2. What breaks the business if it stops, and for how long?

**Ask for the cost per hour.**

> **Chapter 56 §56.1's argument: the number that decides the design is the cost of an hour, and
> it is a business figure rather than a technical one.** **Asking for it converts an argument
> about nines into an arithmetic problem.**

**And ask about duration separately from cost**, because they are different:

| System | Cost/hour | Tolerable outage |
|---|---|---|
| **Warehouse scanners** | **£4,000** — picking stops | **minutes** |
| Email | £900 | **hours** |
| **Finance system** | £600 most days | **and zero on the last day of the month** |
| Intranet | £50 | **a day** |

**The third row is the one that catches people:** **a system's criticality varies with time**,
and **a maintenance window that is safe in the second week is catastrophic in the fifth.**

### 3. What is the growth expectation, over what period?

**A network designed for today's headcount will be rebuilt in three years.**

**And the useful form is a rate rather than a target:** **"we expect 15% annual growth in staff
and 40% in traffic"** is designable; **"we might double" is not.**

**Ask separately about:**

| | |
|---|---|
| **People** | ports, wireless capacity, addresses |
| **Traffic** | **which grows faster than people, consistently** |
| **Sites** | **the one that invalidates a topology** |
| **Applications** | new patterns, new dependencies |

### 4. Who will operate it?

> **This is the question most often omitted and it should constrain the design heavily.**

**A sophisticated design that the available staff cannot operate will be misconfigured within a
year and will fail in ways nobody can diagnose.**

**What to establish:**

| | |
|---|---|
| **How many people, with what skills?** | |
| **Is there out-of-hours cover?** | **which determines the availability that is achievable** |
| **Is there a second person?** | **Chapter 53 §53.4's single point of knowledge** |
| **What do they already operate?** | **familiarity is a real engineering property** |
| **Is there appetite for automation?** | Chapter 70 — and the capability, which is different |

> **Design for the team you have**, and **if that means a simpler architecture with less
> theoretical elegance, that is the correct engineering answer, not a compromise.**

**And the corollary is uncomfortable and worth stating to the client:** **if the design requires
capabilities the team does not have, the options are to change the design, to train, to hire, or
to buy a managed service** — **and pretending the gap does not exist is the option that is
always chosen and never works.**

### 5. What must not happen?

**Regulatory constraints, data residency, segregation requirements** — **hard boundaries, and
discovering one late invalidates work.**

| | Example |
|---|---|
| **Data residency** | **traffic or storage must not leave a jurisdiction** (Chapter 69 §69.1) |
| **Segregation** | **card data, clinical data, classified material** (Chapter 60 §60.4) |
| **Certification** | **the equipment must be on an approved list** |
| **Inspection** | **traffic must be inspectable** (Chapter 60 §60.3), or must not be |
| **Retention** | **logs kept for a stated period** (Chapter 54 §54.3) |
| **Physical** | **a room, a country, a supplier** |

**Ask for the actual obligation, not the interpretation.** **"We must be PCI compliant" and "the
cardholder data environment must be segmented from the rest of the network" are different
statements**, and the second is designable.

### 6. What is the budget, capital and operational?

**And the operational cost usually exceeds the capital cost over the network's life.**

| Frequently omitted | |
|---|---|
| **Support contracts** | **and they renew** |
| **Circuits** | **the largest recurring item in most networks** |
| **Licensing** | **increasingly subscription** |
| **Power and cooling** | Chapter 56 §56.3 |
| **Staff** | **including the on-call** |
| **The refresh** | **Chapter 55 §55.3's EOL, five years out** |
| **Cloud egress** | **Chapter 69 §69.1 — the one nobody models** |

> **A design that fits the capital budget and exceeds the operational one will be dismantled**,
> and the engineer will be blamed.

### 7. What did the last one do wrong?

**The question nobody asks and everybody can answer.**

> **"What frustrates you about the current network?"** and **"what happened the last time it
> failed?"** **produce requirements that no structured questionnaire elicits** — and they are
> requirements the client cares about, which matters for the design being accepted (§72.4).

## Turning answers into requirements

**A requirement is testable. An aspiration is not.**

| Aspiration | **Requirement** |
|---|---|
| "The network must be fast" | **"A 5 GB file transfers to the design server in under 90 seconds"** |
| "It must be reliable" | **"No more than 4 hours of unplanned outage per year at the warehouse"** |
| "It must be secure" | **"A compromised workstation cannot reach the finance server"** |
| "It must be scalable" | **"Add a site of 40 people without redesigning the addressing"** |
| "It must be manageable" | **"Two engineers, one of whom is on holiday, can restore any site within 4 hours"** |

**And the test of a requirement is Chapter 63 §63.2's:** **can you state, in advance, the
observation that would demonstrate it is met?** **If not, it is not a requirement.**

## Recording them

**A requirements document that is used has four columns.**

| Requirement | Source | Priority | Verification |
|---|---|---|---|
| **stated testably** | **who said it, and when** | **must / should / could** | **how it will be demonstrated** |

**The "source" column earns its place**, because **requirements are contested later** — **"who
asked for this?" is asked in every project** — and **a requirement whose source cannot be
identified is one that will be argued about.**

**And the priority column must be enforced.** **A document in which everything is "must" has no
priorities**, and **the design will therefore be over-specified in the places that were easy and
under-specified in the places that were hard.**

## The requirements that emerge later

**Honesty, because no elicitation is complete.**

**Three categories appear after the design is agreed:**

**The unstated assumption.** **"Obviously the printers must work" — nobody mentioned printers.**

**The discovered dependency.** **A system nobody knew about, or a dependency nobody knew
existed** (Chapter 53 §53.2's "why does this exist?").

**And the changed circumstance.** **An acquisition, a regulation, a pandemic** (Chapter 51
§51.4).

> **The response is not better elicitation — it is designed-in headroom** (§72.2). **A design
> with no spare capacity, no spare addresses and no spare ports is a design that cannot absorb
> the requirements that were not stated**, and there are always some.

## What breaks here

**A design that meets every stated requirement and is rejected.** **The unstated ones**, and
question 7 would have found several.

**Gigabit to the desktop delivered and the complaint unchanged.** **The requirement was a
solution**, and the constraint was elsewhere.

**Five nines specified and quarterly patching planned.** **Arithmetically impossible**
(Chapter 56 §56.1). The table ends it.

**A sophisticated design misconfigured within a year.** **Question 4 was not asked**, or was
asked and ignored.

**A regulatory constraint discovered during implementation.** **Question 5**, and it invalidates
work.

**A design within the capital budget and unaffordable to run.** **The operational cost was not
modelled**, and circuits and egress are the usual omissions.

**Everything marked "must".** **No priorities**, and the design will be wrong in the places that
mattered.

**A requirement nobody can attribute.** **The source column.**

> **Network+ note.** Objective 3.1 and the design material. Over-learn: **requirements gathering
> precedes design**; **business needs drive technical requirements**; **documentation records
> what was agreed**; and **stakeholder input must be obtained before implementation.** The
> examinable content is thin; **the seven questions are the professional content, and question 4
> is the one that most often determines whether a design succeeds.**
