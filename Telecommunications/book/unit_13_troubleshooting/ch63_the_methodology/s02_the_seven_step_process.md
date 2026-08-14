# 63.2 The Seven-Step Process

CompTIA's methodology, which N10-009 examines — and it is worth deriving from §63.1's
failure modes rather than presenting as a list to memorise, because each step exists to
prevent a specific error.

```
   1. Identify the problem
   2. Establish a theory of probable cause
   3. Test the theory
   4. Establish a plan of action
   5. Implement the solution or escalate
   6. Verify full system functionality
   7. Document findings, actions and outcomes
```

## 1. Identify the problem

Gather information, question users, identify symptoms, determine what changed — and
duplicate the problem if possible.

The questions, and §63.1's set is the core:

> The most valuable question, asked far too rarely: *when did it last work?* **Followed by:
> *what changed between then and now?***

Because Chapter 55 established that most unplanned outages are caused by planned changes,
and the change log is therefore the highest-yield place to look before anything else.

What "identify" produces, and it should be written down:

| | Example |
|---|---|
| **The symptom, in observable terms** | "HTTPS to `app.example.com` times out from VLAN 30" |
| **The scope** | "all of VLAN 30; VLAN 20 unaffected" |
| **The timeline** | "worked at 09:00, failing by 09:40" |
| **The change window** | "CHG-9012 applied at 09:15" |
| **Reproducibility** | "reproducible from any VLAN 30 host" |

And "duplicate the problem" is the step most often skipped, because it takes time and the
answer feels obvious.

> A fault you cannot reproduce is a fault you cannot confirm you have fixed. You will make
> a change, the symptom will not be present, and you will not know whether that is because you
> fixed it or because it was not happening at that moment.

## 2. Establish a theory of probable cause

**"Probable" is doing work in that phrase.**

**Question the obvious.** The step's own guidance, and it means: consider the simple
explanations first, and check them rather than dismissing them. A cable, a duplex mismatch,
a full disk, an expired certificate, a lease that ran out — the boring causes are the common
ones, and an engineer who skips them because they are beneath the complexity of the symptom
loses an hour regularly.

**Order theories by probability, not by interest.**

> The most interesting hypothesis is rarely the most probable one, and there is a
> professional temptation towards the interesting.

**And form more than one.** A single theory invites §63.1's confirmation bias. Two or
three competing theories force you to look for observations that distinguish them, which is
the useful kind of looking.

**A practical formulation:**

| Theory | **What I would see if true** | **What I would see if false** |
|---|---|---|
| Firewall rule | **deny counter increments on test** | counter static |
| Routing | **no route in the table for that prefix** | route present and correct |
| Server-side | **fails from every source network** | fails from one network only |

If two rows have the same right-hand column, the observation does not distinguish them.

## 3. Test the theory

**Confirm or eliminate, and eliminating is progress.**

**Two rules that matter:**

**Test non-destructively where possible.** A `show` command, a packet capture, a test from
another source — none of which changes anything. A test that requires a change is a change
and belongs in step 5, with its own rollback.

And when the theory is confirmed, establish the mechanism before acting. "The firewall is
blocking it" is a confirmed theory; "the firewall is blocking it because rule 40 was added at
09:15 and shadows rule 80" is a mechanism (Chapter 60 §60.1), and only the second tells you
what to do.

**If the theory is not confirmed:** the step's own guidance is "re-establish a new theory or
escalate", and escalation is a legitimate outcome rather than a failure — see step 5.

## 4. Establish a plan of action

The step people skip when they are confident, and it is Chapter 55 §55.2's change record,
compressed.

Four things, and they take two minutes to state:

| | |
|---|---|
| **What you will do**, exactly | the commands |
| **What it will affect** | **the blast radius** — and it is larger than you think |
| **How you will reverse it** | **and how long that takes** |
| **How you will know it worked** | **stated before, not after** |

> Under pressure this step is compressed into "I'll just try this." Which is acceptable
> for a `show` command and not for a configuration change, and the distinction is whether it
> can be undone in seconds.

And it includes the question of whether to act now: a fix that requires a reboot of a
device carrying other services may be correct and may need to wait, and that is a decision
for the incident commander rather than for the engineer with the terminal open.

## 5. Implement the solution or escalate

Escalation is explicitly part of the method, which is worth noticing.

**Escalate when:**

- You have exhausted your theories and the next step requires knowledge you do not have
- The fix requires authority you do not have — a change to a system another team owns
- The impact is growing rather than contained
- A time limit has been reached — Chapter 53 §53.4's runbooks escalate on a clock, and the
  same discipline applies without one

> Escalating early is cheap and escalating late is expensive, and the reluctance is social
> rather than technical. An engineer who escalates after twenty minutes with good notes is
> more useful than one who escalates after three hours with none.

**And when implementing:**

One change at a time (§63.1). **Verify between changes.** Record what you did as you do
it, because you will not remember the order afterwards — Chapter 53 §53.4's scratch file.

## 6. Verify full system functionality

"Full system" is the operative phrase, and it means two things.

Verify the original symptom is gone — from the position of the person who reported it,
not from the network operations centre. "It works from here" is not verification.

**And verify nothing else broke.** The change had a blast radius; check it. Chapter 54
§54.1's graphs, compared against the same period on previous days, and a check of anything
that shares a device or a path with what you changed.

Then implement preventive measures where applicable — the step's own wording, and it means:
if this can recur, what stops it? A monitoring alert that would have caught it earlier, a
configuration change that prevents it, a runbook entry (§63.4).

## 7. Document findings, actions and outcomes

The step that is skipped when the problem is solved and everyone is tired, and §63.4 is
about why it is the one with the largest compounding return.

What a useful record contains, and it is short:

```
   Symptom:    HTTPS to app.example.com timed out from VLAN 30
   Scope:      VLAN 30 only; ~40 users; 09:40–10:25
   Cause:      CHG-9012 added ACL entry 40 (deny 10.30.0.0/16 any),
               which shadowed entry 80 permitting VLAN 30 to the app tier
   Evidence:   deny counter on entry 40 incremented on test; entry 80 static
   Fix:        moved entry 40 below entry 80 (CHG-9018)
   Verified:   tested from 10.30.5.14 and by the original reporter
   Prevent:    shadow analysis added to the ACL change checklist;
               alert on entry-80 hit count dropping to zero
```

Seven lines, and it answers everything the next person will ask.

## Where the method is over- and under-applied

Honesty about the seven steps, because they are frequently taught as universally applicable
and are not.

**Over-applied:** a user's laptop with no link light. Plug the cable in. Formally
executing seven steps for a trivial fault is theatre, and treating the method as mandatory
regardless of scale trains people to ignore it.

**Under-applied:** exactly the situations where it matters — high pressure, high impact,
multiple people involved, unfamiliar system. These are when instincts are least reliable and
the method is most likely to be abandoned.

> **The heuristic: the method's value scales with the cost of being wrong.** Use it in
> proportion, and use it fully when the impact is large or the system is unfamiliar — which
> is precisely when you will least want to.

## What breaks here

A theory confirmed and the fix does not work. The theory was confirmed and the mechanism
was not established. "Blocked by the firewall" does not tell you which rule or why.

A fix that resolved the symptom and broke something else. Step 6's second half was
skipped. Check the blast radius.

Verified from the operations centre and users still affected. Verify from the reporter's
position.

**An intermittent fault declared fixed.** You cannot verify a fault you cannot reproduce,
and step 1's duplication requirement exists for this.

**Three hours before escalation, with no notes.** Escalating early is cheap.

No record, and the same fault next quarter. §63.4.

Seven formal steps applied to a loose cable. **Proportion.**

> **Network+ note.** Objective 5.1 examines the seven steps by name and in order, and the exam
> tests both the sequence and what belongs in each. Over-learn: identify, theory, test theory,
> plan of action, implement or escalate, verify full functionality and implement preventive
> measures, document. **"Question the obvious" belongs to step 2** and **"duplicate the
> problem" to step 1** — both are frequently misassigned.
