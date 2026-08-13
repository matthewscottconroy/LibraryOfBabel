# The Semester Project

## Network Design and Technical Justification

One organisation, designed across the semester in seven staged deliverables, and
defended in a final paper.

This is not a report. A report describes what things are. **This is an
engineering argument**, and the sentence that appears throughout it is:

> **We chose X rather than Y because…**

Where the reason must be traceable to a stated requirement, a computed number, or
an explicit tradeoff. Not a preference, not a habit, not a vendor's
recommendation. That distinction — argued in Chapter 72 §72.4 — is the whole
purpose of the exercise, and it is what the marks are for.

---

## The organisation

**Meridian Logistics Ltd.** A freight-forwarding company, 78 employees, growing
to a planned 120 within three years, occupying two floors of a leased building.

### The premises

- **Ground floor** (~900 m²): reception, a 40-person open-plan operations room, four
  meeting rooms, a small warehouse and loading bay at the rear, and a comms room.
- **First floor** (~700 m²): finance (12 desks), management (8 offices), a 20-seat
  training room, and a second comms cupboard.
- The two floors are connected by a riser containing existing conduit. Nobody has
  documented what is in it.
- The building is 1970s reinforced concrete. The warehouse has metal racking to
  4 m and an overhead crane.

### The people and what they do

| Group | Count | What they need |
|---|---|---|
| Operations staff | 40 | A cloud logistics platform, all day, plus VoIP handsets |
| Warehouse | 14 | Handheld barcode scanners, on the move, must not drop mid-aisle |
| Finance | 12 | An on-premises accounting server, plus cloud services |
| Management | 8 | Everything, plus video conferencing from offices |
| Remote workers | 4 permanent, plus any staff from home 2 days/week | Full access from anywhere |
| Visitors | up to 20 concurrent | Internet only, isolated from everything |

### Systems

- **On premises:** an accounting server, a file server, a domain controller, a
  backup appliance, and a network video recorder for 22 cameras.
- **Cloud:** the logistics platform (SaaS), email and collaboration (SaaS), and a
  small IaaS deployment running a customs-declaration application that the
  on-premises accounting server must reach.
- **VoIP** for all staff, with a hosted PBX.
- **Guest Wi-Fi** in reception and the training room.
- **22 IP cameras**, PoE, on the ground floor and loading bay.

### Constraints

- The building lease has 6 years remaining.
- Two ISPs serve the business park: a 1 Gb/s symmetric fibre service and a
  500/50 Mb/s coaxial service.
- The IT team is **two people**, neither of whom has run a routing protocol.
- Customs data is subject to a retention requirement and must not traverse the
  public Internet unencrypted.
- Capital budget is constrained and will be questioned line by line. Operational
  budget is more flexible.

The gaps in that brief are deliberate. Part of the work is identifying what you
have not been told and either asking or stating an assumption — Chapter 72 §72.1
is about exactly this.

---

## The seven deliverables

Each is submitted at the stated week, marked, and returned. The final paper
incorporates all of them, revised in light of the feedback.

| # | Week | Deliverable | Chapters | Weight |
|---|---|---|---|---|
| 1 | 3 | Requirements analysis | 1, 3, 11, 14 | 10% |
| 2 | 6 | Addressing plan | 25, 26, 27 | 15% |
| 3 | 8 | Logical design | 17, 19, 20, 29, 30, 39, 40 | 15% |
| 4 | 9 | Physical and wireless design | 6, 10, 16, 42–45 | 15% |
| 5 | 12 | Security design | 57–62 | 15% |
| 6 | 13 | Operations and troubleshooting plan | 53–56, 63–66 | 10% |
| 7 | 14 | Final paper and defence | 72, all | 20% |

Full specifications are in **[deliverables.md](deliverables.md)**; the marking
criteria are in **[rubrics.md](rubrics.md)**.

---

## Working practice

**Teams of three.** Real network design is not solitary, and defending a choice
to a colleague before defending it to a marker is the point. Individual
submissions are permitted on request.

**Cite the book, and disagree with it where you have reason.** A justification
that says "Chapter 45 argues for more access points at lower power, but this
warehouse has a 4 m metal racking problem that the chapter does not address, so
we propose…" is worth more than one that quotes the book correctly and
thoughtlessly.

**Use the tools.** [`netcalc.py`](../tools/netcalc.py) will check your addressing
arithmetic; [`perfcalc.py`](../tools/perfcalc.py) will compute your link budgets
and capacity figures. Showing the computation is worth marks; asserting a number
is not.

**Every diagram gets a version and a date.** Chapter 53's three-diagram
discipline applies from Deliverable 3 onward, and stale diagrams lose marks in
the same way they cost time in practice.

---

## What distinguishes a good submission

Having marked a great many of these, the difference is consistent and worth
stating in advance.

**A weak submission** describes a design. It names technologies, gives a diagram,
and lists what was chosen. Every statement is true and nothing is argued.

**An adequate submission** justifies its choices against the requirements.

**A strong submission** does three further things:

1. **States what was rejected and why.** Considering only the option you chose is
   not consideration.
2. **Quantifies where quantification is possible.** "The 240 m warehouse run
   exceeds copper's 100 m limit" beats "the run is too long for copper".
3. **States the condition under which the answer would change.** "If headcount
   passes 200 or a second site opens, the collapsed core becomes insufficient and
   the addressing plan in §4 reserves space for a distribution layer" is the
   sentence that makes a design document useful to whoever inherits it.

The third is the one that separates a good student from a good engineer, and it
costs one sentence per decision.

---

## A note on scope

You are not expected to produce a bill of materials with part numbers, or
configuration files, or a project plan. You are expected to produce the design
and the argument for it.

Where a decision genuinely depends on information you do not have, say so, state
the assumption you are proceeding under, and say what you would ask. That is what
a competent engineer does, and it earns marks rather than losing them.
