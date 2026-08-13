# Chapter 13 — Exercises

## A. Recall

**A1.** Name Baran's three network topologies and state the vulnerability of each.

**A2.** What redundancy level did Baran find sufficient for high survivability, and
why is that result surprising?

**A3.** State the two independent requirements that led Baran and Davies to the
same mechanism.

**A4.** Distinguish datagram from virtual-circuit packet switching in terms of what
is carried in each packet and what state the network holds.

**A5.** Why is store-and-forward used at all, given the delay it adds?

## B. Apply

**B1.** Follow Baran's derivation: starting from "any node may be destroyed", show
in four steps why each unit of data must carry its own destination address and be
small and standardised.

**B2.** A 1,500-byte packet crosses 12 hops. Compute the total store-and-forward
transmission delay for link rates of 2 Mb/s, 100 Mb/s and 10 Gb/s. Compare each
against a propagation delay of 25 ms and comment.

**B3.** Using *W* ∝ ρ/(1−ρ), compute the relative queueing delay at utilisations of
0.6, 0.75, 0.85, 0.92 and 0.97, normalised to the value at 0.5. At what utilisation
does the delay reach ten times the 50% figure?

**B4.** 150 users, 3 Mb/s each when active, 6% activity. Compute: the circuit-
switched capacity required; the mean and standard deviation of simultaneous active
users; a provisioned capacity giving six standard deviations of headroom; and the
multiplexing gain.

**B5.** Repeat 13.9 at 1,500 and 15,000 users. Tabulate the gain and explain the
trend.

**B6.** Verify with the tool and compare against your hand figures:

```bash
python3 tools/simnet.py statmux --users 150 --rate 3 --activity 0.06 --link <your figure>
```

**B7.** A DNS query is 60 bytes with a 180-byte reply. Compare the total packets
and round trips required over (a) a datagram network and (b) a virtual-circuit
network requiring setup and teardown. Express the overhead as a ratio.

**B8.** An ATM cell is 53 bytes, of which 5 are header. Compute the overhead
percentage. Then compute the effective throughput of a 155 Mb/s ATM link carrying
1,500-byte IP packets, accounting for both the cell tax and the padding of the final
cell.

## C. Analyse

**C1.** Baran's design was rejected by AT&T on grounds that were partly technically
valid and partly institutional. Separate the two, stating what was genuinely
impractical in 1964 and what was a consequence of the incumbent's position.
Then identify a contemporary technology dispute with the same structure.

**C2.** Argue that the convergence of Baran's and Davies's independent designs is
stronger evidence for packet switching than either design alone. Your answer should
identify what the two requirements have in common at a deeper level than
"survivability" and "efficiency".

**C3.** Explain why bufferbloat happens, and why the intuition that more buffer
means better performance is wrong. Your answer must reference TCP's congestion
signal explicitly, and must explain why the harm falls on flows that did not cause
the queue.

**C4.** The popular account says the ARPANET was built to survive nuclear war.
Explain precisely what is right and wrong about that claim, distinguishing Baran's
motivation from the ARPANET's, and identify what was actually borrowed.

**C5.** Consider a video conference on a link that becomes saturated when four
more participants join. Describe what a circuit-switched network would have done,
what the packet network does, and which is preferable — then argue the opposite
case. What property would the packet network need to offer the choice, and why does
it not have it?

## D. Design

**D1.** A regional health service is specifying a network to carry three traffic
types between 40 sites:

- **Clinical imaging** — large transfers, tolerant of delay, intolerant of
  corruption, bursty and unpredictable.
- **Telemedicine video consultations** — 2 Mb/s per session, up to 60 concurrent,
  intolerant of jitter above 30 ms, scheduled during clinic hours.
- **Patient monitoring telemetry** — small packets every 2 seconds from 4,000
  devices, must not be lost, must arrive within 5 seconds.

The service's existing WAN is MPLS with an SLA; the finance director has proposed
replacing it with commodity Internet circuits and SD-WAN at a third of the cost.

Analyse the proposal. For each traffic type, state which of the properties packet
switching abandoned (§13.4) actually matters, and whether the mechanisms of
Chapter 52 can recover it adequately. Recommend an architecture and defend it
against the cost argument. State explicitly what you would measure before committing.

## E. Troubleshoot

**E1.** A manufacturing site's WAN link to head office is 100 Mb/s. Users report
that the ERP application "freezes for a few seconds" several times an hour,
particularly in the afternoon.

Evidence:

- Five-minute average utilisation: 41% at peak.
- One-second sampling during a freeze: 100% for 3–8 seconds.
- `ping` to head office: min 12.1 ms, avg 68.4 ms, max 940 ms, mdev 121 ms, 0% loss.
- Output drop counter on the WAN interface: incrementing.
- A nightly backup was moved to run continuously in "trickle" mode six weeks ago,
  at the request of the backup team.
- The ERP application uses many small request/response exchanges.

Identify the mechanism precisely, referring to §13.3. Explain why the five-minute
average conceals the problem and why zero packet loss on `ping` is consistent with
the diagnosis rather than contradicting it. Explain why the ERP application is
affected more than the backup that is causing it. Give three remedies — one
immediate, one configuration, one architectural — and state which addresses the
cause.
