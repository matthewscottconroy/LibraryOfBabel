# Lab 06 — Subnetting and the Address Plan

**Corresponds to:** Chapters 25, 26, 27
**Week:** 6
**Time:** 120 minutes

---

## Objectives

- Compute network address, broadcast address, usable range and host count for
  any prefix, by hand, in under thirty seconds.
- Produce a complete VLSM allocation, largest-first and aligned for
  summarisation.
- Determine whether a set of prefixes can be aggregated, and produce the summary.
- Demonstrate empirically the connectivity symptom a wrong mask produces.
- Produce the addressing plan that becomes Project Deliverable 2.

---

## You will need

- Paper. This lab is deliberately paper-first; the tools are for checking.
- `netcalc.py` and `subnet_practice.py` from [tools/](../tools/).
- Two hosts and a switch for Part 4.

**No special equipment.** Parts 1–3 and 5 run anywhere.

---

## Procedure

### Part 1 — Drill (30 minutes, on paper)

**1.** Generate a worksheet and do it by hand. No calculator, no tool:

```bash
python3 subnet_practice.py --sheet 20 --topic subnet --seed 606
```

**2.** Time yourself on the last five. If any takes more than sixty seconds,
reread Appendix A §A.5 and do another ten.

**3.** Check with the answer key:

```bash
python3 subnet_practice.py --sheet 20 --topic subnet --seed 606 --answers
```

**4.** For every one you got wrong, write out the **binary** working in full.
Not the shortcut — the binary. The shortcut is a consequence and you should be
able to rederive it.

**5.** Now the reverse direction:

```bash
python3 subnet_practice.py --sheet 10 --topic design --seed 42
```

Given a host count, choose the prefix.

---

### Part 2 — One worked example in full

**6.** Take `172.19.140.219/26` and produce, **in binary**, showing every step:

- the address in binary, with the prefix boundary marked
- the mask in binary
- the AND, giving the network address
- the OR with the inverted mask, giving the broadcast address
- the first and last usable host
- the usable host count
- the next subnet's network address

**7.** Now do the same address by the block-size shortcut, in under thirty
seconds, and confirm the answers match.

**8.** Check both with:

```bash
python3 netcalc.py subnet 172.19.140.219/26 --binary
```

---

### Part 3 — VLSM

**9.** You hold `10.42.0.0/16`. Allocate, largest-first, on paper:

| Requirement | Hosts |
|---|---|
| Operations floor | 900 |
| Warehouse scanners | 60 |
| Finance | 40 |
| Servers | 25 |
| Management | 20 |
| Voice | 200 |
| Cameras | 30 |
| Guest wireless | 100 |
| Six point-to-point router links | 2 each |

**10.** For each, state: the prefix, the network address, the usable range, the
usable count, and the waste.

**11.** State the total consumed, the first free address, and the summary prefix
covering everything you allocated.

**12.** Check:

```bash
python3 netcalc.py vlsm 10.42.0.0/16 900 200 100 60 40 30 25 20 2 2 2 2 2 2
```

**13.** Now redo the allocation **smallest-first** on paper and record what goes
wrong. This is the point of the exercise, and the failure is not immediately
obvious until you reach the 900-host subnet.

---

### Part 4 — The wrong mask, demonstrated

**14.** Configure two hosts on a switch:

- Host A: `10.0.0.10/24`
- Host B: `10.0.0.200/24`

Confirm they can ping each other.

**15.** Change **host A only** to `/25`:

```bash
sudo ip addr change 10.0.0.10/25 dev <iface>
```

**16.** Ping A to B. Record the result. Ping B to A. Record that too, and note
that the two directions differ.

**17.** Add a third host at `10.0.0.50/24`. From A, ping it. Record.

**18.** Explain the pattern before changing anything back. Then verify your
explanation:

```bash
python3 netcalc.py local 10.0.0.10/25 10.0.0.200
python3 netcalc.py local 10.0.0.10/25 10.0.0.50
```

**19.** Examine host A's ARP cache during the failing ping. What is it trying to
resolve, and why?

---

### Part 5 — Summarisation

**20.** Determine, on paper, whether each set can be summarised into a single
prefix, and if so what it is:

- (a) `192.168.4.0/24`, `192.168.5.0/24`, `192.168.6.0/24`, `192.168.7.0/24`
- (b) `192.168.4.0/24`, `192.168.5.0/24`, `192.168.6.0/24`
- (c) `10.1.8.0/22`, `10.1.12.0/22`
- (d) `172.16.30.0/24`, `172.16.31.0/24`, `172.16.32.0/24`, `172.16.33.0/24`

**21.** For each that cannot be exactly aggregated, state **why** — contiguity or
alignment — and give the minimal exact covering set.

**22.** Check:

```bash
python3 netcalc.py summarise 192.168.4.0/24 192.168.5.0/24 192.168.6.0/24 192.168.7.0/24
```

**23.** Set (d) is the instructive one. Explain why four contiguous /24s do not
summarise into a /22 here, and what the addresses would have to have been.

---

## Expected observations

- **Step 16: A cannot reach B, and B *can* reach A** — or rather, B's ARP request
  arrives and A replies, but A's own traffic to B goes to a gateway that may not
  exist. The asymmetry is diagnostic and confuses people badly.
- **Step 17: A reaches `.50` fine.** With a /25, A's network is `10.0.0.0/25`,
  covering `.0`–`.127`. `.50` is local; `.200` is not.
- **Step 19: A ARPs for its gateway** when trying to reach `.200`, and there
  probably is not one — so the failure is an ARP failure with a locally generated
  unreachable, not a routing error.
- **Step 13: smallest-first fails.** By the time you reach the 900-host subnet,
  the space is fragmented into pieces none of which is a contiguous, aligned /22.
- **Step 20(d): does not summarise.** `172.16.30.0/24` through `.33.0/24` spans a
  block boundary — 30, 31, 32, 33 is contiguous but not aligned, since a /22 must
  start at a multiple of 4. The minimal exact set is `172.16.30.0/23` plus
  `172.16.32.0/23`.

---

## Break it

**A.** Give a host the **network address** of its own subnet — `10.0.0.0/24` on a
/24. Record what the operating system does. Some refuse; some accept and fail
strangely.

**B.** Give a host the **broadcast address** — `10.0.0.255/24`. Record.

**C.** Configure a host with a **valid address and a mask of `255.255.255.255`**
(/32). Predict what it can reach before testing.

**D.** Set up two subnets that **overlap** — `10.0.0.0/24` and `10.0.0.128/25` on
different VLANs — and put a host in each. Record which one wins and why.

---

## Debrief

**1.** Show, in binary, the full working for `172.19.140.219/26`. Then state the
block-size shortcut and demonstrate that it is a *consequence* of the binary
rather than an independent rule.

**2.** In step 13, allocating smallest-first failed. State exactly where it failed
and why. Then state the general principle in one sentence, and name the
non-networking problem it is identical to.

**3.** In Part 4 you produced the selective-connectivity symptom. Describe it as a
*user* would report it, then explain the mechanism, then state the single command
that would confirm the diagnosis in ten seconds.

**4.** Host A ARPed for its gateway when trying to reach `10.0.0.200`. Explain the
full chain of reasoning that led it there, starting from the AND operation. Why
is the resulting error message misleading?

**5.** Set (d) in Part 5 is four contiguous /24s that do not summarise. Explain
why, and state what the four networks would have had to be for the aggregation to
work. Then explain what this implies for the *order* in which you allocate address
space in a design, connecting it to Chapter 31 §31.4.

**6.** Your VLSM allocation left free space. State how much, where it starts, and
what you deliberately reserved it for. Then state one growth scenario your plan
handles gracefully and one it does not, and what you would have done differently
knowing the second.

---

## Feeds the project

This lab's Part 3 and Part 5 are directly reusable in **Deliverable 2**, due this
week. The Meridian brief's host counts differ from the practice figures above,
but the method is identical and the marking criteria reward exactly what you
practised here: largest-first allocation, explicit growth headroom, alignment for
summarisation, and binary working shown for at least three subnets.
