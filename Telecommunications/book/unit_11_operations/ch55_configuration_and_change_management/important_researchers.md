# Chapter 55 — The People

Like Chapter 53, this chapter's intellectual history is borrowed — from software
engineering, from safety science, and from the operations community's own hard-won practice.
There is no protocol here to attribute.

**Charles Perrow (1925–2019).** Sociologist, and *Normal Accidents* (1984).

Perrow's argument, developed after Three Mile Island, is that some systems produce accidents
as a normal property rather than as a malfunction, and he identified the two conditions:

| | |
|---|---|
| **Interactive complexity** | **components interact in ways the designers did not foresee** |
| **Tight coupling** | **there is no slack — a failure propagates before anyone can intervene** |

> **A network is both.** A VLAN change interacts with spanning tree, which interacts with a
> first-hop redundancy protocol, which interacts with a firewall's state table — and the
> propagation is at the speed of the control plane, which is faster than a human.

Perrow's uncomfortable conclusion is that in such systems, accidents cannot be engineered
away — and that adding safety systems adds complexity, which adds interactions, which adds
accidents. The recommendation is not more process but less coupling: failure domains,
staged rollouts, and slack.

Which is why §55.2's "change one thing" and "stage it" are more valuable than any approval
workflow. They reduce coupling. Approval workflows do not.

**Diane Vaughan.** *The Challenger Launch Decision* (1996), and the normalisation
of deviance.

Vaughan's finding was that the Challenger disaster was not caused by anyone ignoring a known
risk. It was caused by a sequence of small, individually reasonable decisions, each of
which accepted an observation slightly outside the original specification because the previous
flight had also been outside it and had been fine.

> The specification said no O-ring erosion. Erosion was observed. It did not cause a failure.
> So erosion became acceptable. Then more erosion. Then more. At no point did anyone decide
> to accept an unacceptable risk. The definition of acceptable moved.

**The network analogue is exact and constant:**

- "We always skip the change record for this kind of thing"
- "That alert always fires; ignore it" (Chapter 54 §54.4)
- "We have run out of support before and it was fine"
- "The rollback has never been tested and we have never needed it"

Each is reasonable given the last time. None is a decision to accept risk. And the drift
is only visible from outside, or afterwards.

> **Vaughan's contribution is the diagnostic:** when a practice has quietly become the norm,
> ask what the written standard says and when it last matched behaviour. **The gap is the
> finding.**

Gene Kim, Jez Humble, Patrick Debois, John Willis and the DevOps movement. From about
2009 — and the empirical result is the part that matters.

The *State of DevOps* research programme, later published as *Accelerate* (Forsgren,
Humble & Kim, 2018), measured what people had argued about for a decade.

**Its central and counter-intuitive finding:**

> Organisations that deploy more frequently have fewer failures, not more — and **recover
> faster when they do fail.**

The mechanism is the one this chapter argues. A small, frequent change has a small blast
radius, is easy to reason about, and is easy to reverse. A large, infrequent change is a
batch of interacting modifications released together, and when it fails you cannot tell
which part failed.

The four measures they settled on — deployment frequency, lead time for change, change
failure rate, and time to restore — are worth knowing because they are measurable in a
network too, and because the first two are the ones nobody measures.

And it produced the sharpest available rebuttal to heavyweight change control:

> Approval by a body external to the team was found to have no positive effect on change
> failure rate, while measurably slowing throughput. Slower change means larger batches,
> and larger batches are the risk — the exact mechanism §55.2 describes. **Peer review inside
> the team performed better.**

This is a genuinely uncomfortable finding for a discipline that has invested heavily in change
advisory boards. It does not say that review is worthless — it says that review by people
without context, at a distance from the work, buys delay rather than safety.

**Ward Cunningham (b. 1949).** The technical debt metaphor, 1992.

Cunningham's original formulation is more precise than the way it is usually used, and the
precision matters:

> "Shipping first-time code is like going into debt. A little debt speeds development so long
> as it is paid back promptly with a rewrite. The danger occurs when the debt is not repaid.
> Every minute spent on not-quite-right code counts as interest on that debt."

**Two things people get wrong about it.**

Debt is not the same as bad work. Cunningham was describing a deliberate trade — ship now,
correct later — not carelessness. The debt in §55.1 is mostly not deliberate, which makes
it worse: nobody chose to take it on, so nobody feels responsible for repaying it.

The interest is the cost of working with it, not the cost of fixing it. Which is exactly
§55.1's argument — the cost of accumulated configuration is paid on every subsequent change,
continuously, not as a one-off remediation bill.

**Mark Burgess (b. 1966).** CFEngine, from 1993 — and the idea of convergent configuration.

Burgess's insight was that configuration management should not be a sequence of actions but a
description of a desired state, with the system repeatedly converging on it.

| | |
|---|---|
| **Imperative** | **"run these commands"** — assumes the starting state |
| **Convergent / declarative** | **"this is what it should look like"** — corrects from any state |

> A convergent system has no drift, because drift is corrected on the next run. This is
> §55.4's inversion of authority, stated a decade before the network industry took it
> seriously, and it is the direct ancestor of Puppet, Chef, Ansible and Terraform
> (Chapter 70).

Burgess also formalised "promise theory" — that an agent can only make promises about its
own behaviour, not impose obligations on others — which is a surprisingly good model for
why centralised network configuration systems are harder than they look.

## What this chapter's borrowed history establishes

Three findings, each from a different field, each pointing the same way.

**Perrow: complexity and coupling produce accidents structurally.** Reduce coupling, not
merely errors.

Vaughan: the standard drifts silently, and each step is reasonable. Compare behaviour to
the written standard periodically, because nobody notices from inside.

Accelerate: smaller and more frequent is safer than larger and rarer, and external
approval makes things worse.

> The instinct that change is dangerous is correct. The instinct that follows from it — change
> less, and approve harder — is wrong, and it is wrong in a way that has been measured.
> **Change more, in smaller pieces, with better reversal**, and the evidence is on that side.
