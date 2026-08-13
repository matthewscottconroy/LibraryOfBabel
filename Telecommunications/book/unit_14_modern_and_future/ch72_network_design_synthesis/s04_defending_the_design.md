# 72.4 Defending the Design

**The chapter's real subject, and the book's.**

> **Producing *a* design is not difficult.** **A competent person with a catalogue can produce a
> design in an afternoon.** **Producing a design you can defend — where every significant choice
> has a reason, the rejected alternatives were genuinely considered, and the trade-offs are
> stated rather than hidden — is the skill that distinguishes an engineer from a configurator.**

## What "defensible" means

**Four properties, and each is testable.**

| | |
|---|---|
| **Traceable** | **every choice traces to a requirement, or to a stated assumption** |
| **Comparative** | **the alternatives were considered and the rejection has a reason** |
| **Honest about trade-offs** | **what this design is worse at is stated** |
| **Falsifiable** | **the conditions under which it would be wrong are named** |

**And the fourth is the one that distinguishes a design from a proposal:**

> **"This design assumes traffic grows at 40% annually. At 70% the WAN capacity is exhausted in
> year one rather than year two, and the remedy is X, costing Y."** **A design that states the
> conditions under which it fails is a design that can be trusted**, and one that does not is
> asking to be believed.

## The document

**Ten sections, and the ordering is the argument.**

| | Section | Contains |
|---|---|---|
| **1** | **Executive summary** | **one page: what, why, what it costs, what it does not do** |
| **2** | **Requirements** | **§72.1's table, with sources and priorities** |
| **3** | **Assumptions** | **explicitly, and this is where growth rates and cost-per-hour go** |
| **4** | **Design** | **the three diagrams** (Chapter 53 §53.1) **plus the derivation** |
| **5** | **Decisions and alternatives** | **the section that makes it defensible — see below** |
| **6** | **What it does not do** | **the honest limits** |
| **7** | **Risks** | **with likelihood, impact and response** (Chapter 57 §57.3) |
| **8** | **Cost** | **capital and five-year operational** |
| **9** | **Implementation** | **sequence, dependencies, rollback at each stage** |
| **10** | **Operations** | **who runs it, with what, and what they need to learn** |

**Sections 5 and 6 are the ones that are omitted, and they are the ones that matter.**

### The decision record

**One table row per significant decision**, and **a design with fewer than about a dozen has not
recorded its reasoning.**

| Decision | Chosen | Alternatives | Why | Reversible? |
|---|---|---|---|---|
| **Core topology** | **Collapsed core, two switches** | three-tier; single switch | **200 users; three-tier is unwarranted complexity for the team (§72.1 q4); single switch fails the £900/hour target** | **hard** |
| **WAN** | **500 Mb/s + LTE backup** | 1 Gb/s; two fixed circuits | **capacity from the matrix; the second circuit's cost exceeds the outage cost at £900/hour** | **easy** |
| **Routing** | **Static, with a floating default** | OSPF | **four routes; OSPF is operational burden the team does not need** | easy |
| **Wireless** | **Controller-based, 802.1X** | cloud-managed; PSK | **scanner roaming (§72.1 q1); PSK cannot be rotated across 30 devices** | moderate |
| **Addressing** | **10.20.0.0/16, structured** | flat /22 | **§72.3's summarisation; renumbering is a project** | **very hard** |
| **Firewall** | **HA pair** | single unit | **it is the single point of failure on every flow** | hard |

**The "reversible?" column is the one nobody includes and the most useful.**

> **A decision that is easy to reverse deserves less analysis and can be made quickly.** **A
> decision that is very hard to reverse — the address plan, the topology, the vendor — deserves
> disproportionate attention**, and **conflating the two wastes effort on the wrong choices.**

### The "what it does not do" section

**Written by you, before it is written by someone else.**

| | |
|---|---|
| **"The WAN is sized to year two; year three requires an upgrade, costed at £X"** | |
| **"There is no microsegmentation within the server segment"** | |
| **"A failure of the firewall pair's shared configuration affects both"** | |
| **"The design assumes two engineers; with one, the recovery time doubles"** | |
| **"Server-to-server traffic is not inspected"** | |

> **A design that names its own limits is trusted.** **A design that is silent about them is
> either dishonest or unexamined, and a reviewer cannot tell which** — **which is why stating
> them is in your interest and not merely in the client's.**

## The presentation

**Different audiences need different arguments**, and the design is one document with several
readings.

| Audience | Cares about | Lead with |
|---|---|---|
| **Executive** | **cost, risk, timescale** | **section 1, and the risk table** |
| **Finance** | **capital and operational, over five years** | **section 8, and the alternatives' costs** |
| **Operations** | **can we run it?** | **section 10, and the decision record's reasoning** |
| **Security** | **what is exposed, what is enforced** | **§72.3, and section 6's limits** |
| **Peer engineer** | **is it right?** | **the derivation, and the alternatives** |

**And the executive summary's hardest sentence is the cost of the thing not being done**, which
is §72.1's question 2 restated: **"an hour of warehouse downtime costs £4,000; this design
reduces expected annual downtime from 40 hours to 4."**

> **Which converts the conversation from "why is the network so expensive?" to "is £144,000 of
> avoided loss worth £60,000 of investment?"** — **and the second is a question a finance
> director can answer.**

## Defending it under challenge

**Five challenges you will meet, and the response to each.**

### "Why not just use X?"

**Answer with the requirement it fails.** **Not "X is bad" but "X does not provide Y, which
requirement 4 requires."**

**And if X would work, say so.** **"X would meet the requirements; we chose this because of Z,
and if Z is not persuasive, X is a reasonable alternative"** — **which is a stronger position
than defending a preference as a necessity.**

### "This is over-engineered"

**Frequently correct, and worth testing honestly.**

> **The test: which requirement does this element serve, and what happens without it?** **If the
> answer is "nothing measurable", remove it.**

**And Chapter 67 §67.4's warning applies generally:** **building the sophisticated architecture
for an environment that does not need it is the most common design error in this field**, and
**an engineer who has just read seventy-one chapters is unusually susceptible to it.**

### "Can we do it more cheaply?"

**Always yes, and the answer is what is given up.**

**Present it as a table rather than as a refusal:**

| Saving | Cost |
|---|---|
| **Single firewall instead of a pair** | **−£18,000 capital; expected downtime +12 hours/year = £10,800/year** |
| **No LTE backup** | **−£1,800/year; expected downtime +6 hours/year at £4,000 = £24,000** |
| **Unmanaged edge switches** | **−£11,000; no 802.1X, no DHCP snooping, no diagnostics** |

> **Which lets the client make the decision, with the consequence stated** — **and a client who
> accepts a documented risk is in a different position from one who was not told.**

### "The last consultant said…"

**Ask what problem that recommendation solved.** **It was frequently correct for a requirement
that has changed**, and **saying so is more useful than contradicting a predecessor.**

### "What if you are wrong?"

**The best challenge, and the answer is section 3 and the falsifiability property.**

> **"The design's load-bearing assumptions are these four. Here is what happens if each is
> wrong, and what it costs to correct."**

## The design review

**Chapter 55 §55.2's peer review, at architecture scale**, and it should be requested rather than
endured.

**What a reviewer should be asked to check:**

| | |
|---|---|
| **Does every element trace to a requirement?** | **and is there anything unexplained?** |
| **Does every requirement have an element?** | **the reverse direction, which is the one missed** |
| **What is the single point of failure?** | **there is always one** (Chapter 56 §56.2) |
| **What breaks at 3× the traffic? At 10×?** | |
| **How is it operated by the team described?** | **§72.1 q4** |
| **What was rejected, and would you have rejected it?** | |
| **What is this design worse at than the current one?** | **there is always something** |

**And the reviewer to seek is the one who will disagree.** **A review that produces no findings
is a review that did not happen** (Chapter 56 §56.4's argument about tests, applied to
documents).

## What this book has been for

**The closing argument, and it is short.**

**Seventy-one chapters built mechanisms.** **Each was introduced because it answered part of one
question** — **how to get information from one process on one computer to another, reliably,
efficiently, securely and at scale** — **and each was accompanied by what it makes possible and
what it makes go wrong.**

**The purpose of that was not to produce a catalogue.** **It was to make the following possible:**

> **When a requirement arrives, you can derive what it implies rather than recognising it.**
> **When a proposal arrives, you can identify which assumption it rests on.** **When a fault
> arrives, you can reason from the mechanism rather than from memory.** **And when someone asks
> "why that rather than something else?", you have an answer that is a derivation rather than a
> preference.**

**Which is why this book put layering in Unit V rather than Chapter 1** (Chapter 21), **why every
chapter closed with what breaks** (Chapter 63), **why the history was load-bearing** — **because
knowing that a mechanism was a compromise, and what it was a compromise with, is what lets you
judge whether the compromise still applies.**

**And the recurring shape, which has appeared in almost every unit:**

| | |
|---|---|
| **The good idea that lost to economics** | Chapters 22, 57, 68 |
| **The compromise that outlived its constraint** | Chapters 50, 54, 59 |
| **The mechanism that was reintroduced after being abandoned** | Chapters 50, 52, 71 |
| **The failure that was documented decades before it mattered** | Chapters 57, 62 |
| **The thing that shipped beating the thing that was correct** | Chapters 22, 51, 58 |

> **None of those is a criticism of the field.** **They are what engineering looks like when it
> is done by people, over decades, under constraints that change** — **and recognising the shape
> is what lets you predict which current claim will follow it.**

**The last observation is the one to carry:**

> **Every mechanism in this book was a reasonable answer to a problem that existed at the
> time.** **Your job is to know which problem, whether it still exists, and what it cost to
> solve** — **and then to make your own reasonable answer, and to be able to say why.**

## What breaks here

**A design with no decision record.** **It cannot be defended, only asserted** — and it will be
challenged.

**A design that does not state what it does not do.** **Someone else will state it**, at a worse
moment.

**Effort spent on an easily reversible decision and none on the address plan.** **The
reversibility column exists for this.**

**A cost challenge answered with a refusal.** **Answer with the trade**, and let the client
decide.

**An over-engineered design produced by a well-read engineer.** **The commonest failure of the
competent**, and the test is which requirement each element serves.

**A design review with no findings.** **It did not happen.**

**A design defended as a preference.** **"I prefer X" is not an engineering argument**, and if
the alternative would also work, saying so is the stronger position.

> **Network+ note.** Objective 3.1 and the exam's scenario questions. Over-learn: **design begins
> with requirements**; **document decisions and their rationale**; **consider scalability,
> availability, security and cost together**; and **implementation follows a plan with defined
> rollback.** The examinable content is a fraction of this section; **the defensibility argument
> is the whole of the profession.**
