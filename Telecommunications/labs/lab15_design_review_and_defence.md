# Lab 15 — Design Review and Defence

**Corresponds to:** Chapter 72, and everything before it
**Week:** 15
**Time:** 150 minutes

---

## This lab is different

You will not build anything. You will attack someone else's design, and defend
your own against people whose job for the afternoon is to find its weaknesses.

This is the last lab because it is the one that requires everything. It is also
the closest thing in the course to the actual work: a design that has never been
argued with is a design whose flaws are still in it, and the person who finds them
in a review is very much cheaper than the person who finds them in production.

---

## Objectives

- Read an unfamiliar network design and locate its weaknesses systematically.
- Ask a question that discriminates rather than one that scores points.
- Defend a design decision by reconstructing its reasoning, not by restating it.
- Concede a genuine flaw quickly and propose the remedy.
- Recognise, in someone else's work, the failure modes you cannot see in your own.

---

## You will need

- Each team's complete Deliverable 7 — the final paper — exchanged with another
  team **48 hours in advance**.
- The review checklist below.
- Nothing else.

---

## Procedure

### Part 1 — The structured read (before the session, 90 minutes)

Read the other team's design against this checklist. For every row, record either
"satisfied, evidence on page N" or a specific concern.

#### Requirements

- [ ] Are the requirements requirements, or are they solutions in disguise?
- [ ] Is there a communication matrix, and does the design actually follow from it?
- [ ] Are availability targets derived from a stated business cost, or asserted?
- [ ] Is the two-person operability constraint addressed anywhere?
- [ ] Are the gaps in the brief identified, with assumptions stated?

#### Addressing

- [ ] Does the arithmetic check out? Verify three subnets yourself with
      `netcalc.py`.
- [ ] Do subnet sizes match the host counts from the requirements?
- [ ] Was allocation largest-first? Check by looking for fragmentation.
- [ ] Will it summarise? Compute the summary for one site.
- [ ] Is there growth headroom, and is the assumption behind it stated?
- [ ] Would it survive a merger with a company using 10.0.0.0/8 casually?

#### Logical design

- [ ] Is the spanning tree root set explicitly, or left to default election?
- [ ] Does every VLAN have a routing plan?
- [ ] Are the DHCP options the phones and access points need actually present?
- [ ] Is there an answer to "what happens when the riser link fails"?
- [ ] Is the routing decision defended against the operability constraint?

#### Physical and wireless

- [ ] Does the PoE budget add up? Add it up.
- [ ] Is the channel plan derived, or asserted?
- [ ] Is the warehouse treated as a distinct problem from the office?
- [ ] Is there a link budget with a stated margin?
- [ ] Is media selection justified against distance, environment **and** power?

#### Security

- [ ] Is the threat model proportionate to a freight forwarder?
- [ ] Can the addressing plan actually express the segmentation policy?
- [ ] Does the firewall policy trace to the threat model?
- [ ] Is there a position on split tunnelling, argued rather than assumed?
- [ ] Is the customs-data constraint addressed explicitly?
- [ ] Are the devices that cannot do 802.1X handled honestly?

#### Operations

- [ ] Does every alert have a stated action?
- [ ] Do RPO and RTO vary by system, with reasoning?
- [ ] Is there shared-fate analysis on the two ISP services?
- [ ] Would the runbooks pass the 03:00 test?

#### The decisions register

- [ ] Is it complete?
- [ ] Does each entry name the alternative considered?
- [ ] Does each entry state the condition under which the answer would change?

---

### Part 2 — Preparing three questions (30 minutes, in session)

From your checklist, select **three questions**. Not thirty; three.

A good review question has all of these properties:

- **It is answerable.** "Why did you do it this way?" is not a question, it is a
  challenge. "You sized the operations VLAN as a /23 for 40 desks plus 40 phones
  plus growth — what growth figure did you use, and where does it come from?" is
  a question.
- **It discriminates.** The answer tells you whether the team reasoned or guessed.
- **It concerns something that matters.** A typo in a subnet mask is worth
  mentioning in writing and is not worth one of your three.

Write each question out. Beneath each, write what a good answer would contain and
what a bad answer would sound like. You will be marked on the questions as much as
on your answers.

---

### Part 3 — The defence (15 minutes per team)

**Format per team:**

| Time | Activity |
|---|---|
| 0–7 min | Present the design. No slides required; the paper is the artefact. |
| 7–12 min | The reviewing team asks its three questions. |
| 12–15 min | The instructor asks two. |

**Presenting:** cover the requirements that drove the design, the three or four
decisions you consider most consequential, and one thing you would do differently.
Do not walk through the document — everyone has read it.

**Answering.** The rules that matter:

- **Reconstruct the reasoning, do not restate the conclusion.** "We chose a
  collapsed core" is a restatement. "At 340 users on two floors with no expectation
  of exceeding 500, a distribution layer adds two devices and a failure domain for
  no measurable benefit; the addressing plan reserves space for one at §4 if the
  second building proceeds" is a reconstruction.

- **"We did not consider that" is a good answer.** It costs less than a confident
  wrong one, and it is the answer a marker most wants to hear when it is true.

- **Concede quickly when the point is good**, then say what you would change. A
  team that spots a genuine flaw and a team that concedes it gracefully both score
  well; a team that defends the indefensible scores worst of anyone in the room.

---

### Part 4 — The written review (30 minutes, in session)

Produce a one-page review of the design you examined, structured as:

1. **The single strongest thing about this design**, with the evidence.
2. **The three most consequential concerns**, in order, each with: what the
   concern is, what could go wrong, and what you would do instead.
3. **One thing you are taking back to your own design**, and what you will change.

Submit it. It goes to the team you reviewed.

---

## Marking

| Criterion | Weight |
|---|---|
| Quality of the three questions asked | 25% |
| Answers to questions about your own design | 30% |
| Ability to reconstruct reasoning on request | 20% |
| The written review | 25% |

Note the first row. Finding a real weakness in someone else's design, and asking
about it precisely, is assessed at nearly the same weight as defending your own —
because it is the same skill exercised from the other side, and because it is the
skill that makes you useful in a design review for the rest of your career.

---

## Debrief

Whole class, after all defences.

**1.** Every team: name the single best question you were asked, and say what it
revealed. If it revealed nothing, say that too and explain why the question
missed.

**2.** Name a weakness you found in someone else's design that you then discovered
in your own. This happens to nearly everyone, and being able to say so is the
point of the exercise.

**3.** Was there a decision, anywhere in the room, that two teams made
*differently* and both defended well? If so, work out what assumption differed.
This is the most valuable ten minutes of the session, because it demonstrates that
"defensible" and "correct" are not the same word, and that engineering questions
frequently have several right answers separated by a stated assumption.

**4.** Which section of the checklist found the most problems across all designs?
What does that suggest about where the course under-prepared you?

**5.** One sentence each: what would you do differently if you started this project
again in week 3?

---

## A closing note

You have now designed a network, argued for it, had it attacked, and attacked
someone else's. That sequence — design, justify, defend, revise — is not an
academic exercise dressed up as practice. It is the job, and the only thing that
changes when you are paid for it is that the consequences of an unexamined
assumption are borne by someone else.

Chapter 72 §72.4 said that producing a design is not difficult, and that producing
one you can defend is the skill. You have now done both, under someone else's
scrutiny, which is the only conditions under which the second claim can be tested.

The rest is practice.
