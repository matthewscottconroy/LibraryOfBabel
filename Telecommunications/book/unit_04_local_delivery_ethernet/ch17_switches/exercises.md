# Chapter 17 — Exercises

## A. Recall

**A1.** State the switch algorithm in three sentences.

**A2.** Complete the sentence: "A switch breaks up ___ domains. It does not break
up ___ domains. A ___ breaks up ___ domains."

**A3.** What does a hub do with a signal arriving on port 3?

**A4.** How long is the default MAC address table ageing timer, and what happens to
a device that has been silent longer than that?

**A5.** Name the three forwarding modes and state the latency of each at 1 Gb/s.

## B. Apply

**B1.** Trace the switch algorithm through this sequence on a switch with an empty
table. Stations: P on port 1, Q on port 2, R on port 3, S on port 4.

(a) P → R; (b) R → P; (c) Q → S; (d) P broadcasts; (e) S → Q; (f) P → S.

For each frame, state what is learned, which ports it exits, and why.

**B2.** Count collision domains and broadcast domains:

```
   Router ── Fa0/0 ── SW-1 (12 ports used) ── SW-2 (8 ports used)
          └─ Fa0/1 ── SW-3 (6 ports used, one port to a 4-port hub)
```

**B3.** Repeat 17.7 with three VLANs configured across SW-1 and SW-2, and the router
providing inter-VLAN routing on subinterfaces.

**B4.** A 24-port switch has 3 MB of shared buffer. Compute the average buffer per
port, and express it as microseconds of transmission time at 1 Gb/s and at 10 Gb/s.
Comment on what burst duration each can absorb.

**B5.** Compute the maximum packet rate of a 1 Gb/s port at minimum frame size,
including interframe gap and preamble. Then compute the aggregate line-rate packet
rate for a switch with 48 × 1 Gb/s and 4 × 10 Gb/s ports.

**B6.** A switch datasheet quotes 176 Gb/s switching capacity and 95 Mpps
forwarding rate for the configuration in 17.10. Is it non-blocking for bandwidth?
Is it non-blocking for packet rate? State the traffic pattern under which it would
fail.

**B7.** An interface shows: utilisation 22% five-minute average, output drops
incrementing at ~400/hour, zero CRC errors, zero runts, zero collisions. Diagnose it
and state the two measurements you would take to confirm.

## C. Analyse

**C1.** Explain why a switch can learn addresses at no cost, and why learning by
observation is the only mechanism available. Your answer must reference the property
of MAC addressing established in Chapter 15 §15.2.

**C2.** MAC flooding degrades a switch to a hub rather than breaking it. Explain
the mechanism, and identify two other attacks in this book with the same shape —
degradation to an earlier, weaker design rather than destruction.

**C3.** A host receives a constant stream of unicast frames addressed to a
different host in the same VLAN. Give two distinct causes, state how you would
distinguish them, and explain what each implies about the network.

**C4.** Derive the ~58% throughput ceiling for a switch with pure input queueing
and random traffic, at least qualitatively. Explain what virtual output queueing
changes and why it restores line-rate capability.

**C5.** Argue that "more buffer is better" is wrong, using TCP's congestion signal.
Then state what a buffer is actually for, what the right size is, and what
mechanism handles the cases a correctly-sized buffer cannot.

**C6.** Store-and-forward's latency penalty shrinks as link rates rise while
cut-through's advantage is fixed. Compute both at 1, 10 and 100 Gb/s for a
maximum-size frame, and use the result to explain why cut-through has become a
specialist choice.

## D. Design

**D1.** You are specifying access switches for a floor with 220 workstations,
28 access points, 40 IP telephones and 24 cameras. Requirements:

- Every device on its own switch port; no hubs.
- Four VLANs: workstations, voice, cameras, and access-point management.
- 10 Gb/s uplinks, dual-homed to two distribution switches.
- The security team requires that a compromised workstation cannot reach the camera
  VLAN, and that MAC flooding cannot succeed.

Specify: the number of switches and ports; the collision and broadcast domain counts
in your design; the port-security configuration on access ports, with the maximum
address count justified per device class; and the forwarding mode you would select
with a reason. State what you would do about the buffer question and why.

## E. Troubleshoot

**E1.** A finance department of 40 users reports intermittent slowness, worst
between 09:00 and 09:30 and again after lunch. Nobody can reproduce it on demand.

Evidence:

- All 40 are on one access switch, one VLAN, along with 180 other users on adjacent
  switches in the same VLAN.
- The VLAN contains 220 hosts plus about 400 virtual machines on six hypervisors.
- The access switch's MAC table shows 8,192 of 8,192 entries used.
- `show mac address-table count` reports the table is full.
- Packet capture on a finance workstation shows a substantial volume of unicast
  traffic addressed to other hosts.
- Broadcast traffic measured at the workstation: roughly 900 frames/second at peak.
- No CRC errors anywhere; no collisions; output drops present but low.
- The problem began about a month ago, coinciding with a virtualisation project.

Diagnose it. Explain the mechanism connecting the full MAC table to the observed
unicast traffic, and explain why finance is affected when the cause is elsewhere.
Explain why the timing pattern fits. State the immediate remedy, the correct remedy,
and what the broadcast figure tells you independently. Then state what monitoring
would have caught this before users did.
