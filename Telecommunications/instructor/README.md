# Instructor Materials

A fifteen-week schedule, exam blueprints, and retrieval-quiz banks for a course
built on this book.

Assumes one five-hour weekly meeting (three lecture hours plus two lab hours, as
a conceptual allocation rather than a timetable). Adjust freely; the dependency
order is the part that matters, and the labs and project stages are pinned to it.

---

## The fifteen weeks

| Wk | Central question | Chapters | Lab | Project |
|---|---|---|---|---|
| 1 | What actually is a network? | 1–3 | 01 Inspecting a working host | — |
| 2 | How do bits travel through the real world? | 4–7, 10 | 02 Media, signals, link budgets | — |
| 3 | How can many devices share infrastructure? | 8, 9, 11–14 | 03 Building a LAN; hubs vs switches | **D1 Requirements** |
| 4 | How does a LAN deliver data? | 15–18, 21–23 | 04 Frames, MAC addresses, ARP | — |
| 5 | How do we divide and control LANs? | 19, 20 | 05 VLANs, trunks, spanning tree | **Exam 1** |
| 6 | How does an address describe a network? | 24–27 | 06 Subnetting and the address plan | **D2 Addressing** |
| 7 | How do packets cross networks? | 28–33 | 07 Routing between networks | — |
| 8 | How does a packet reach an application? | 34–40 | 08 Ports, TCP, DNS, DHCP | **D3 Logical design** |
| 9 | How does networking work without wires? | 41–46 | 09 Wireless survey and analysis | **D4 Physical/wireless** |
| 10 | How do we connect over distance? | 47–51 | 10 WAN behaviour, latency, tunnels | **Exam 2** |
| 11 | How do administrators operate networks? | 52–55 | 11 Documentation, monitoring, baselines | — |
| 12 | How do we make networks secure? | 56–61 | 12 Segmentation, firewalls, TLS | **D5 Security** |
| 13 | How do we find what's broken? | 62–65 | 13 **The troubleshooting gauntlet** | **D6 Operations** |
| 14 | What do modern networks look like? | 66–70 | 14 Overlays, cloud, automation | **D7 Final paper** |
| 15 | Can you reason about an unfamiliar network? | 71, synthesis | 15 Design review and defence | **Defence · Final exam** |

Chapter numbers are 1-based as printed in the book; the quiz engine's indices are
zero-based (Chapter 1 is index 0).

### Two departures worth defending to a curriculum committee

**The OSI model is week 4, not week 1.** Students meet it after they have solved
four distinct problems that layering solves. This is the book's central
pedagogical bet, and in practice it converts the seven layers from a chart to be
recited into a diagnostic instrument students reach for unprompted.

**Troubleshooting is a thread, not a unit.** Every chapter closes with failure
modes, and every lab includes a deliberate "break it" stage from week 1. Week 13
formalises a method students have already been using.

---

## The weekly rhythm

Five hours is a long time and student fatigue is real. A workable shape:

| Time | Activity |
|---|---|
| 0:00–0:20 | Retrieval quiz on last week, then review of the misconceptions it exposed |
| 0:20–1:20 | First-principles lecture |
| 1:20–1:30 | Break |
| 1:30–2:20 | Worked problems, diagrams, Socratic reasoning |
| 2:20–2:30 | Break |
| 2:30–3:10 | Live demonstration leading into the lab |
| 3:10–3:20 | Break |
| 3:20–4:40 | Hands-on lab |
| 4:40–5:00 | Lab debrief and an "explain what happened" exit exercise |

The **demo → lab → explanation** cycle is the part that matters. Students are
graded on explaining *why* something happened, not on getting a green light.

The **retrieval quiz** at the start is the highest-return twenty minutes in the
week. Use the banks in [retrieval-quizzes.md](retrieval-quizzes.md), or generate
them from the question bank:

```bash
cd quiz && cargo run -p quiz-cli -- --subject ../Telecommunications
```

---

## Assessment weighting

A defensible distribution, matching the book's emphasis:

| Component | Weight |
|---|---|
| Exam 1 (week 5) | 10% |
| Exam 2 (week 10) | 15% |
| Final exam (week 15) | 20% |
| Semester project (7 deliverables + defence) | 35% |
| Labs and debriefs | 15% |
| Retrieval quizzes | 5% |

Note that the project is the largest single component. That is deliberate: it is
the only component that assesses synthesis, and Chapter 72 argues that synthesis
is the job.

**Labs are marked on the debrief, not the outcome.** A student whose lab did not
work and who can explain precisely why has done better work than one whose lab
worked and who cannot. State this explicitly in week 1 or students will not
believe it.

---

## Equipment, and what to do without it

The labs are written to degrade gracefully and each states its fallback.

**Minimum viable:** one Linux VM per student with `ip`, `tcpdump`, `dig`, `ss`,
`iperf3` and Wireshark. Roughly two thirds of the labs run entirely on this.

**Better:** two or more machines per team on a shared segment, a managed switch,
and a router. Labs 3–7 and 11–13 use these.

**Simulation:** Packet Tracer, GNS3 or Containerlab substitute for the switching
and routing labs. Each lab notes what survives simulation and what does not — you
cannot learn to recognise a marginal cable in Packet Tracer and the labs say so
rather than pretending.

**Wireless (lab 9):** a phone-based analyser suffices for most of it. A spectrum
analyser is much better and the lab explains what it reveals that a Wi-Fi
analyser cannot.

**Week 13's gauntlet** needs real or simulated equipment you are willing to break.
Fault scripts are in [exam-blueprints.md](exam-blueprints.md) §4.

---

## Where students reliably struggle

Six things, from the shape of the material rather than from any one cohort.

**Binary fluency (week 6).** The single largest predictor of who copes with
subnetting is whether Chapter 2 §2.2 was actually absorbed in week 1. Set
`subnet_practice.py --topic binary` as low-stakes homework from week 1 onward,
not as revision in week 6.

**The local-or-remote decision (week 6).** Students learn to compute a network
address and do not connect it to what a host *does*. `netcalc.py local` shows both
AND operations side by side; run it on the projector.

**Broadcast versus collision domains (week 5).** Drill it with diagrams until it
is automatic. It is worth ten minutes a week for three weeks.

**Flow control versus congestion control (week 8).** Conflated almost universally.
Two windows, two purposes, two owners — say it three times.

**"More power fixes coverage" (week 9).** The intuition is strong and wrong. Lab
09 is designed to demonstrate it empirically; let the measurement do the arguing.

**Justification in the project.** Students describe rather than argue for the
first two deliverables regardless of instruction. Mark D1 and D2 hard on this
dimension and the message lands before the weight increases.

---

## Files

- **[exam-blueprints.md](exam-blueprints.md)** — three exam specifications with
  sample questions, plus the week 13 fault scripts.
- **[retrieval-quizzes.md](retrieval-quizzes.md)** — fifteen weekly retrieval
  quizzes with answers and the misconception each is designed to expose.
- **[../project/](../project/)** — the semester project brief, deliverables and
  rubrics.
- **[../labs/](../labs/)** — the lab guides.
- **[../book/appendices/appendix_d_network_plus_crosswalk.md](../book/appendices/appendix_d_network_plus_crosswalk.md)**
  — the N10-009 objective mapping, for students taking the certification
  alongside.
