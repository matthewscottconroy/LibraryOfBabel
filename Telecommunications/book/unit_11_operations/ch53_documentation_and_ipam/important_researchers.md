# Chapter 53 — The People

**This chapter has no protocol designers.** Its intellectual history comes from **medicine,
aviation and safety science** — fields that confronted the same problem earlier and with worse
consequences: **how does a competent person do the right thing under pressure, at 03:00, when
they are tired and the situation is unfamiliar?**

**Atul Gawande (b. 1965).** Surgeon, and **the person who made checklists respectable.**

**Gawande's 2009 work with the World Health Organization produced a nineteen-item surgical
safety checklist** — confirm the patient's identity, mark the site, confirm the antibiotic was
given, count the instruments — **items so obvious that surgeons found them insulting.**

**A trial across eight hospitals on four continents found that deaths fell from 1.5% to 0.8%
and major complications by around a third.**

> **The objection was always the same, and it is the same objection network engineers make:
> "I know how to do my job."** **They did. The checklist was not for the knowledge; it was for
> the moment when the knowledge is present and the step is skipped anyway** — because the
> theatre is busy, because it is late, because someone assumed someone else had done it.

**Gawande's distinction between two failure modes is the useful one:**

| | |
|---|---|
| **Ignorance** | **we do not know how** — the historical problem |
| **Ineptitude** | **we know how and fail to apply it** — the modern problem |

**A runbook addresses ineptitude, not ignorance.** **It does not teach; it ensures.**
**Understanding which of the two you are solving determines what you write** — and most
badly-written runbooks are attempts to teach, embedded in a document that will be read at 03:00
by someone who cannot learn just then.

**Richard I. Cook (1953–2022).** Anaesthesiologist and safety researcher, and **"How Complex
Systems Fail" (1998)** — **eighteen numbered observations, four pages, and the most useful
document about operations that anyone has written.**

**The ones that bear directly on this chapter:**

> **"Complex systems run in degraded mode."** **A system is always operating with multiple
> latent faults present**, because it is too expensive to fix them all. **Your network is
> currently broken in ways you do not know about, and it is working anyway.**

> **"Post-accident attribution to a root cause is fundamentally wrong."** There is no single
> cause; **there is a combination of conditions that each were individually insufficient.**
> **Which is why §53.4 prefers "what documentation would have made this shorter?" to "what was
> the root cause?"**

> **"Change introduces new forms of failure."** And **"safety is a characteristic of systems,
> not of their components."**

**Cook's work is the reason blameless post-incident review is now standard practice**, and it
is worth reading the four pages rather than the summaries.

**Sidney Dekker (b. 1969).** **The "new view" of human error**, and the argument that reframes
every incident review.

**Dekker's position:** **"human error" is not an explanation. It is a label applied after the
fact to a decision that made sense at the time.**

> **The question is never "why did they do that stupid thing?" It is "why did that action make
> sense to a competent person, given what they knew, what they could see, and what pressure
> they were under?"**

**And that reframing produces different fixes.** **"The engineer typed the wrong interface"
produces a rebuke and changes nothing. "The interface names differed by one character and the
confirmation prompt did not show which device" produces a change to the tooling**, and the next
person does not make the mistake.

**John Allspaw (b. 1974).** **Etsy, and the operationalisation of Dekker and Cook in software
and infrastructure.**

**Allspaw's contribution was practical:** **blameless post-mortems as a documented, mandatory,
routine process**, with the explicit rule that **the person who made the change is a source of
information and not a defendant.**

**His argument for it is not humanitarian but epistemic:** **an engineer who expects blame will
not tell you what actually happened**, so **a blaming process buys you a worse understanding of
your own system**, permanently.

> **"Blameless" does not mean "consequence-free". It means the review's purpose is
> understanding**, and if the organisation cannot separate those two things, **it will not
> learn from incidents regardless of what its process document says.**

**The Google SRE team, and the 2016 book.** **The most influential single document in modern
operations practice**, and it is free online.

**What it contributed to this chapter's material:**

**Toil as a measurable quantity.** **Manual, repetitive work that scales with service size** —
and the argument that it should be **capped as a percentage of an engineer's time**, because
otherwise it expands to fill everything and no improvement ever happens.

**Error budgets.** **If the availability target is 99.9%, then 0.1% of failure is budgeted, and
spending it is permitted.** **This converts an argument between operations ("no changes") and
development ("ship faster") into arithmetic**, and Chapter 56 §56.1 develops it.

**Documentation as a production artifact** — **reviewed, versioned, owned, and tested**, rather
than as a courtesy.

> **The SRE material's greatest value is that it is specific about costs.** Most operations
> writing is exhortation. **This is the rare body of work that says what a practice costs and
> when it is not worth doing.**

**The ITIL authors, and the UK's Central Computer and Telecommunications Agency.** **From the
late 1980s**, and it deserves an honest paragraph.

**ITIL defined the vocabulary this chapter uses** — incident, problem, change, configuration
item, the CMDB, known error — **and that shared vocabulary is a genuine contribution.**

**And it acquired a reputation for producing enormous process overhead**, largely deserved, in
organisations that implemented the documents rather than the intent.

> **The useful reading of ITIL is as a checklist of questions rather than a set of required
> procedures.** "Who owns this? What is the change process? Where is the configuration
> recorded? What is the difference between an incident and a problem?" **are all worth
> answering.** **Answering them with a forty-page procedure is where it goes wrong.**

**ANSI/TIA-606, and the labelling standards committees.** **Unloved, and correct.**

**TIA-606 specifies an administration scheme for telecommunications infrastructure** —
identifiers for spaces, pathways, cables, terminations and grounding, **with a defined format
and required records.**

**It is over-specified for a small organisation and it is right about the principle**, which
§53.2 states more briefly: **identifiers are hierarchical, location-based and permanent.**

## What the borrowed disciplines have in common

**Medicine, aviation and nuclear operations all arrived at the same three conclusions decades
before networking did.**

**Checklists work, and the people who resist them are the experts.** Gawande.

**There is no root cause, and looking for one prevents learning.** Cook, Dekker, Perrow.

**The system is always partly broken and running anyway.** Cook.

> **Networking's operational practice is thirty years behind aviation's and is catching up by
> importing it.** **Every idea in this chapter that seems obvious was contested, and was settled
> somewhere with a higher body count** — which is a reason to accept the conclusions without
> repeating the experiment.
