# Project Rubrics

## The general criterion

Every deliverable is marked against four dimensions, weighted differently by
stage. The dimensions are the same throughout, because the skill being developed
is the same throughout.

| Dimension | What is assessed |
|---|---|
| **Technical correctness** | Does it work? Is the arithmetic right? Would the design function as described? |
| **Justification** | Is every significant choice argued from a requirement, a number, or a stated tradeoff? |
| **Alternatives** | Were the options actually considered, and is the rejection reasoned? |
| **Communication** | Could a competent engineer who was not in the room build this? Could a manager understand the summary? |

The second and third dimensions are what distinguish this project from a report,
and together they carry more weight than correctness at every stage after
Deliverable 2. A design that works and is unargued scores lower than one with a
defensible flaw and a clear argument — because the second is recoverable and the
first is luck.

---

## The band descriptors

Applied to each dimension.

**Distinction (70–100).** Choices are argued from computed numbers and stated
requirements. Rejected alternatives are named with reasons. The conditions under
which each choice would change are stated. Where the brief is silent, the
assumption is explicit. The document would be genuinely useful to a successor.
Disagreements with the textbook, where present, are reasoned rather than
careless.

**Merit (60–69).** Choices are justified against requirements and mostly
quantified. Some alternatives considered. Assumptions mostly explicit. A
competent engineer could build from it with a few questions.

**Pass (50–59).** The design would function. Justification is present but thin —
"we chose Cat6a because it supports 10 Gb/s" without asking whether 10 Gb/s is
required. Alternatives largely absent. Communication adequate.

**Marginal (40–49).** The design is incomplete or contains errors that would
require rework. Justification is assertion. The document describes rather than
argues.

**Fail (below 40).** The design would not function as described, or the
submission does not engage with the brief's requirements.

---

## Per-deliverable weighting

### D1 — Requirements Analysis (10%)

| Dimension | Weight | Specific expectations |
|---|---|---|
| Technical correctness | 20% | Traffic estimates plausible; latency and loss tolerances correct per application type |
| Justification | 30% | Availability targets derived from stated business cost, not asserted |
| Alternatives | 10% | — |
| Communication | 40% | The communication matrix is legible and complete; gaps and assumptions explicit |

**Automatic deductions:** stating a solution as a requirement (−5 each, max −15);
omitting the two-person operability constraint (−10); treating VoIP as a
bandwidth-sensitive rather than latency-sensitive application (−10).

### D2 — Addressing Plan (15%)

| Dimension | Weight | Specific expectations |
|---|---|---|
| Technical correctness | 45% | Every subnet arithmetically correct; no overlaps; sizes match D1's counts |
| Justification | 25% | Block choice, hierarchy and conventions all argued |
| Alternatives | 10% | Why this range and this hierarchy rather than others |
| Communication | 20% | Table complete and usable as an operational document |

**Automatic deductions:** any arithmetic error (−5 each); an allocation order
that prevents summarisation (−10); no growth headroom (−10); binary working
absent (−10). **Full marks are not available** without the IPv6 plan.

### D3 — Logical Design (15%)

| Dimension | Weight |
|---|---|
| Technical correctness | 35% |
| Justification | 30% |
| Alternatives | 20% |
| Communication | 15% |

**Specific expectations:** root bridge placed explicitly; inter-VLAN routing
method argued against at least one alternative; the static-versus-dynamic routing
decision defended against the operability constraint; DHCP options present for
phones and access points; an answer to "what happens when the riser link fails".

**Automatic deductions:** root bridge left to default election (−10); VLANs
created with no routing plan (−15); diagrams without version and date (−5).

### D4 — Physical and Wireless Design (15%)

| Dimension | Weight |
|---|---|
| Technical correctness | 40% |
| Justification | 25% |
| Alternatives | 20% |
| Communication | 15% |

**Specific expectations:** PoE budget adds up and is shown; channel plan derived
rather than asserted; the warehouse treated as a distinct problem; at least one
link budget computed with margin assessed; media choices reference distance,
environment and power delivery.

**Automatic deductions:** PoE budget arithmetic wrong or absent (−15); channel
plan asserted without derivation (−10); warehouse racking not addressed (−10);
recommending increased transmit power as a coverage remedy without addressing the
counterargument (−10).

### D5 — Security Design (15%)

| Dimension | Weight |
|---|---|
| Technical correctness | 30% |
| Justification | 35% |
| Alternatives | 20% |
| Communication | 15% |

**Specific expectations:** threat model proportionate to the organisation;
segmentation expressible in the D2 addressing plan; firewall policy traceable to
the threat model; an explicit position on split tunnelling; the customs-data
constraint addressed; devices that cannot do 802.1X handled honestly.

**Automatic deductions:** security design that the addressing plan cannot express
(−15); a threat model assuming a state adversary without justification (−10);
proposing controls the two-person team demonstrably cannot operate, without
acknowledging it (−10); describing NAT as a security control (−5).

### D6 — Operations and Troubleshooting (10%)

| Dimension | Weight |
|---|---|
| Technical correctness | 25% |
| Justification | 25% |
| Alternatives | 10% |
| Communication | 40% |

**Specific expectations:** every alert has a stated action; RPO and RTO vary by
system with reasoning; shared-fate analysis performed on the two ISP services;
six failure scenarios with symptom, detection, diagnosis and remedy; two runbooks
executable by an unfamiliar technician.

**Automatic deductions:** any alert with no action (−5 each); uniform RPO/RTO
across all systems (−10); a runbook that fails the 03:00 test (−10); a rollback
plan reading "restore from backup" without a tested procedure (−10).

### D7 — Final Paper and Defence (20%)

**Paper (14%)**

| Dimension | Weight |
|---|---|
| Integration and coherence | 30% |
| Decisions register | 25% |
| Technical correctness | 20% |
| Communication, including the executive summary | 25% |

The **integration** mark is for whether the six deliverables have become one
design. A stapled collection with contradictions between the addressing plan and
the security policy fails this dimension regardless of the quality of the parts.

The **decisions register** is marked on completeness and on the fourth column —
the condition under which the answer would change. A register with that column
blank scores at most half.

**Defence (6%)**

| Criterion | Weight |
|---|---|
| Answers to questions about the design's weaknesses | 50% |
| Ability to reconstruct the reasoning behind a choice on request | 30% |
| Questions asked of the other team | 20% |

Note the last row. Finding a real weakness in someone else's design, and asking
about it precisely, is assessed — because it is the same skill as defending your
own, exercised from the other side.

**"We did not consider that"** is an acceptable answer and costs less than a
confident wrong one. Attempting to defend an indefensible choice rather than
conceding it costs the most.

---

## A note to markers

The most common failure in marking this project is rewarding volume. A team that
produces forty pages of accurate description will feel like it has worked harder
than one that produces twelve pages of tight argument, and will have learned
less.

The question to hold throughout: **for each significant choice, can I find the
sentence that says why, and does that sentence point at a requirement or a
number?** If not, the mark belongs in the Pass band regardless of how much is
there.

The second most common failure is penalising a defensible unconventional choice.
A team that argues for static routing here, against the textbook's general
preference for dynamic protocols at scale, and grounds it in the two-person
operability constraint, has done exactly what was asked. Reward it.
