# Lab 13 — The Troubleshooting Gauntlet

**Corresponds to:** Chapters 63, 64, 65, 66
**Week:** 13
**Time:** 150 minutes

---

## This lab is different

There is no procedure. There is a working network, an instructor who has broken
it in several places, and a clock.

**You are marked on process, not on speed.** A team that reaches fault 3
methodically and runs out of time scores higher than one that guesses fault 6
correctly. The marking is published below so that you optimise for the right
thing, because teams that do not know this reliably optimise for the wrong thing.

---

## Objectives

- Execute the seven-step methodology under time pressure without abandoning it.
- Choose tests that discriminate between hypotheses rather than confirm one.
- Change one thing at a time and observe the effect before changing another.
- Produce a written record **during** the incident, not after.
- Recognise, from symptom alone, the failure signatures catalogued in Chapter 65.

---

## Rules

1. **One change at a time.** Make it, observe, record, decide. A team that makes
   three changes and finds the fault gone has learned nothing and is marked
   accordingly.

2. **Write as you go.** Each team keeps a running log: time, observation, what it
   ruled out, hypothesis, test, result. The log is submitted and is 10% of the
   mark. Reconstructing it afterwards is visible and is not accepted.

3. **No configuration changes until you have stated the hypothesis** out loud to
   the instructor and it has been recorded. You may look at anything.

4. **You may restore a fault** you have fixed if you want to re-observe it. Say so.

5. **"We do not know" is allowed and costs nothing.** Guessing costs marks.

---

## The tools you have

Everything from Chapter 64. The ones that will earn their keep:

```bash
ping / ping -s 1400 -M do / ping -c 100
traceroute -n / mtr -n --report -c 50
ip addr / ip route / ip neigh / ip -s link
ss -tulnp / ss -tin
dig / dig +trace / dig @<server>
tcpdump -i any -n / wireshark
nc -zv host port
iperf3 -c host [-P 16] [-u]
```

On the switches and routers:

```
show interfaces status
show interfaces <if> | include error|collision|drop
show vlan brief
show mac address-table
show spanning-tree
show ip route
show ip arp
show ip dhcp snooping binding
show logging
```

---

## The procedure you follow

Chapter 63's seven steps, and you will be asked which one you are on:

1. **Identify** — gather information, question users, determine what changed,
   duplicate the problem.
2. **Theorise** — a probable cause, stated before any change.
3. **Test the theory** — an observation that *discriminates*, not an action that
   fixes.
4. **Plan** — including the side effects and the rollback.
5. **Implement** — one change.
6. **Verify** — full functionality, not just the reported symptom.
7. **Document** — as you go.

The two questions that earn the most in the least time:

> **When did it last work?**
> **What changed between then and now?**

---

## The faults

The instructor selects three to five from this list. Each has a distinctive
signature and a tempting wrong answer, and the tempting wrong answer is the one
your team will reach for under pressure.

| # | Fault | What a user reports | The trap |
|---|---|---|---|
| 1 | Access port moved to the wrong VLAN | "My machine has no network" | "DHCP is down" |
| 2 | Subnet mask changed /24 → /25 on one host | "Some things work, some don't" | "Firewall rule" |
| 3 | Default gateway off by one | "I can't get to the Internet" | "ISP outage" |
| 4 | Native VLAN mismatch on the trunk | Nothing — connectivity that shouldn't exist | Nobody notices |
| 5 | ICMP type 3 code 4 blocked | "It connects then freezes" | "Ping works, MTU is fine" |
| 6 | Speed/duplex hard-coded at one end | "It's slow, gets worse when busy" | "We need more bandwidth" |
| 7 | Root bridge priority lowered on an access switch | "Slow, and it comes and goes" | "Bad cable" |
| 8 | Static route to a decommissioned next hop | "One system is unreachable" | "That server is down" |
| 9 | DHCP relay removed from one subnet | "Everyone on floor 2 is down" | "Scope exhausted" |
| 10 | Two APs on channels 1 and 3 | "Wi-Fi is bad but signal is full" | "Increase transmit power" |
| 11 | Duplicate IP (static inside the DHCP pool) | "Two machines drop out randomly" | "Failing network card" |
| 12 | Wrong DNS server in the DHCP scope | "The Internet is down" | "The Internet is down" |

---

## Marking

| Criterion | Weight | What earns it |
|---|---|---|
| Evidence before hypothesis | 30% | The log shows observations preceding each theory |
| Discriminating tests | 25% | Tests chosen to distinguish, not to confirm |
| One change at a time | 20% | Each change isolated, effect observed and recorded |
| Correct diagnosis | 15% | The fault named correctly |
| Written record produced during | 10% | Timestamps consistent with the session |

**Correct diagnosis is 15%.** Read that again before you start.

---

## Worked example of a good log entry

```
14:02  Reported: "the network is slow" from three users on floor 2.
14:03  Confirmed: iperf3 floor2 -> server = 8.4 Mb/s.
       Same test from floor 1 = 940 Mb/s.  => not the server, not the core.
14:05  ping server from floor2: min 0.9 avg 1.1 max 2.0 mdev 0.2, 0% loss.
       => latency and loss are NOT the problem. Bandwidth is.
       Rules out congestion on the path (would show avg >> min).
14:07  Hypothesis A: duplex mismatch on the floor-2 uplink.
       Hypothesis B: a rate-limit or shaper on that port.
       Discriminating test: interface counters. A predicts late collisions;
       B predicts output drops with no errors.
14:09  show interfaces gi0/24: 1,204 late collisions, incrementing.
       => Hypothesis A confirmed. B eliminated.
14:10  Checked config: speed 100 / duplex full hard-coded on switch side.
       Host side is autonegotiate -> falls back to half. Chapter 66 §66.2.
14:12  Plan: set switch port to autonegotiate. Side effect: link bounces ~3 s.
       Rollback: re-apply the hard-coded config.
14:13  Applied. Link renegotiated to 1000/full.
14:14  Verified: iperf3 = 938 Mb/s. Late collisions no longer incrementing.
       Verified from two other floor-2 hosts.
14:16  Root cause: port hard-coded during an unrelated change on 4 Nov
       (per change log) and never reverted.
       What made this hard: the link was UP at full speed in the interface
       summary, and monitoring alerted on "down" but not on "degraded".
```

Note what that log does. It **rules things out explicitly**. It states two
hypotheses and the test that separates them **before** running it. It records the
rollback. It verifies more than the one reported symptom. And the last two lines
are the ones that generate an improvement.

---

## Debrief

Held as a whole class after the clock stops. Every team answers.

**1.** For your first fault: state the symptom, the evidence you gathered before
forming a hypothesis, and the test that discriminated. If you formed a hypothesis
first, say so — that is the honest and more useful answer.

**2.** Name one hypothesis you held that turned out to be wrong, and the
observation that eliminated it. Teams that report no wrong hypotheses were either
lucky or are not being candid.

**3.** Which of the twelve traps did your team fall into, even briefly? What made
it attractive?

**4.** Fault 5 — connects then freezes — is the one most teams fail. If you had
it: at what point did you test with a large packet, and what would have prompted
you to do it sooner? If you did not have it, state the two-command test now.

**5.** For one fault you solved, write the incident record as it would be filed in
production. Include the field that matters most: **what made this hard to find**,
and what specific change — an alert, a diagram, a documented convention — would
make the next occurrence cheaper.

**6.** Rank the twelve faults by how quickly you believe you could now diagnose
each from the user's report alone. For the three you rank slowest, state what you
would need to have prepared in advance to speed them up.

---

## For the instructor

Fault selection notes are in
[../instructor/exam-blueprints.md](../instructor/exam-blueprints.md) §4, including
the full script for each fault and the marking sheet.

Two suggestions from experience. **Introduce fault 4 in every session**, and award
marks to any team that notices it unprompted — most will not, and the debrief
about *why* nobody noticed is the most valuable ten minutes of the day. And
**debrief fault 5 at length** regardless of whether any team drew it: the "ping
works so the MTU is fine" reasoning is precisely what Chapter 34 has been warning
about, and hearing a team articulate the trap out loud is what makes it stick.
