# 63.4 Documenting and Learning

The seventh step, skipped when the problem is solved and everyone is tired, and the one
with the largest compounding return of anything in this book.

## Why it is skipped, and why the reasons are wrong

| The reason | The answer |
|---|---|
| **"It's obvious what happened"** | **to you, now.** Not to anyone in six months, including you |
| **"Nobody reads it"** | **because nothing useful is in it.** That is a quality problem, not a value one |
| **"There's no time"** | **it is seven lines** (§63.2), and it takes four minutes |
| **"I'll do it tomorrow"** | **the detail is already gone.** Chapter 53 §53.4 |
| **"It won't happen again"** | **it will, or something adjacent will** |

> The record's value is not to you. It is to the person who meets this at 03:00 in eighteen
> months, and that person is frequently you, with no memory of it.

## What makes a record useful

§63.2 gave the seven-line template. Three properties distinguish records that get used.

**It records the mechanism, not the action.**

| Poor | Useful |
|---|---|
| "Restarted the service and it worked" | **"The service had exhausted its file descriptor limit (1,024); restarting cleared it; the limit has been raised to 65,536"** |
| "Reseated the fibre" | **"Rx power was −31 dBm against a −27 dBm sensitivity; cleaning the connector restored it to −18 dBm"** |
| "Fixed a firewall rule" | **"ACL entry 40 shadowed entry 80; moved 40 below 80"** |

The left column tells the next person what to try. The right column tells them what to look
for, and only the second helps when the symptom differs slightly.

**It records what was ruled out.**

> "We confirmed the circuit, the routing and DNS were all correct" saves the next person forty
> minutes, and it is almost never written down, because eliminating things does not feel
> like progress while you are doing it.

And it records the evidence, not just the conclusion. The counter value, the log line, the
capture filename. Because the conclusion may turn out to be wrong, and the evidence
remains useful when it does.

## The search problem

A knowledge base nobody searches is a knowledge base nobody has.

Records are searched by symptom, not by cause — the person searching has the symptom and
not the cause, which is the whole reason they are searching.

Which means the record must contain the symptom in the words a person would use:

```
   Title:  "HTTPS times out from one VLAN; other VLANs fine; ACL shadowing"
   Tags:   timeout, ACL, VLAN, firewall, shadowed rule
```

**Not:** `"CHG-9018 remediation"`.

And error messages should be quoted verbatim, because the first thing anyone does is paste
the error into a search box, and if your own knowledge base does not match it, they will find
someone else's.

## Two habits that compound

Chapter 53 §53.4 stated them and they belong here as practice.

**Write the runbook during the first incident.**

> While you are working it out, you are generating exactly the content the runbook needs —
> the commands, the healthy output, the false leads, the order you tried things in.
> Afterwards it has compressed into "oh, it's usually the ACL", and the detail is gone.

Keep a scratch file open and paste into it as you go. Tidying takes twenty minutes;
reconstruction takes two hours and produces something worse.

And after every significant incident, ask what documentation would have made it shorter.

Better than "what was the root cause?" for this purpose, because it produces an action
rather than an explanation, and everyone involved can answer it, not only the person who
found the fault.

## The blameless review

**Chapter 55's Dekker and Allspaw material, applied.**

The purpose is understanding, and an engineer who expects blame will not tell you what
actually happened — so a blaming process buys you a worse understanding of your own system,
permanently.

**What a useful review asks:**

| | |
|---|---|
| **What did each person believe at each moment, and why was that reasonable?** | **not "why did they do that?"** |
| **What information was available, and where was it?** | |
| **What made the diagnosis slow?** | **and this is where the documentation actions come from** |
| **What would have detected this earlier?** | Chapter 54 §54.4 |
| **What nearly went wrong but did not?** | **the near-misses are free information** |

**And what it does not ask:** who is at fault.

> "Blameless" does not mean "consequence-free". It means the review's purpose is
> understanding, and an organisation that cannot separate those two things will not learn
> from incidents regardless of what its process document says.

**The near-miss question is the under-used one.** "We nearly failed over to the standby and
then found it had not been patched" is a finding with no incident attached, and it is the
cheapest kind of learning available.

## Metrics that are worth keeping

Four, and they are the ones that change behaviour.

| Metric | Tells you |
|---|---|
| **Time to detect** | **whether monitoring works** (Chapter 54) — and it should fall |
| **Time to diagnose** | **whether documentation and tooling work** — this is the one §63.4 improves |
| **Time to resolve** | **the total, and Chapter 56 §56.1's MTTR** |
| **Recurrence** | **whether the fix addressed the cause** |

**Time to diagnose is the interesting one.**

> It is the fraction of MTTR that documentation, runbooks, monitoring and method actually
> reduce, and Chapter 56 §56.1 established that reducing MTTR is where the availability
> leverage is. Measuring it separately from time to resolve makes the value of this chapter
> visible, which is otherwise the hard part of justifying it.

And recurrence is the one that catches superficial fixes. A fault fixed three times is a
fault whose cause was never found, and the metric surfaces it where individual incident
records do not.

## Feeding it back

The record is not the end. Four things should come out of an incident besides the write-up.

| Output | Goes to |
|---|---|
| **A monitoring alert** that would have detected it earlier | Chapter 54 §54.4 |
| **A runbook** for the next occurrence | Chapter 53 §53.4 |
| **A configuration or design change** that prevents it | Chapter 55 |
| **A documentation update** — a diagram, an inventory field, a knowledge base entry | Chapter 53 |

And each should have an owner and a date, or it will not happen — the post-incident
action list is where good intentions go, and the difference between organisations that improve
and those that do not is entirely whether those actions are tracked like any other work.

> An incident that produces a write-up and no actions has been documented and not learned
> from.

## What breaks here

A knowledge base entry that says "restarted the service". Tells the next person what to
try, not what to look for.

**A record nobody can find.** Titled by change reference rather than by symptom.

**An error message paraphrased.** People search for the exact string.

**A record written a week later.** The detail is gone. Scratch file, during.

A review that identified who made the mistake. You will not be told what happened next
time.

**Post-incident actions with no owner.** They will not happen. Track them as work.

The same fault three times, each with a record. The records exist and the cause was never
found. The recurrence metric is what surfaces this.

**No time-to-diagnose metric.** Then the value of everything in this unit is invisible, and
it will not be funded.

> **Network+ note.** Objective 5.1's seventh step and Objective 3.1's documentation material.
> Over-learn: **document findings, actions and outcomes**; **update the knowledge base**;
> **incident records feed problem management**; and root cause analysis identifies the
> underlying cause rather than the symptom. The seventh step is examined by name, and the
> compounding argument is what makes it worth doing rather than worth remembering.
