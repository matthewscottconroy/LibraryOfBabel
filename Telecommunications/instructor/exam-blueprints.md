# Exam Blueprints

Three exams, progressively less dependent on recall. That progression is the
design: by the final, a student who has memorised the book without understanding
it should be unable to pass, and a student who understands it without having
memorised much should do well.

---

## The general principle

**Do not ask "what is OSI Layer 2?"** Ask "a switch receives this frame; what
does it do next, and why?"

The first question tests whether a fact was stored. The second tests whether a
mechanism is understood, and it cannot be answered by someone who has only stored
facts. It is also the form CompTIA's performance-based questions take, which
makes the alignment convenient rather than contrived.

Every exam in this blueprint is **open-book for the appendices only** —
Appendix A's tables and Appendix B's port list. Removing recall of lookup-table
material shifts the assessment onto reasoning, which is what you want to measure.
Students who object that this makes it easier discover otherwise.

---

## Exam 1 — Fundamentals and Layers

**Week 5 · 10% · 90 minutes · Chapters 1–23**

| Section | Marks | Form |
|---|---|---|
| A: Short answer | 20 | 10 questions, mechanism-level |
| B: Calculation | 30 | 5 problems with working shown |
| C: Frame and switch reasoning | 30 | Scenario-based |
| D: Extended answer | 20 | One question, choice of two |

### Section B — the calculations that must appear

1. Goodput efficiency for a given payload size and link rate.
2. Latency decomposition: propagation, transmission, processing for a stated path.
3. Shannon capacity from bandwidth and SNR in dB.
4. A binary conversion and bitwise AND.
5. Minimum frame size or collision-domain reasoning.

### Sample questions

**B3.** A wireless link has 40 MHz of bandwidth. The client measures −72 dBm of
signal against a −96 dBm noise floor. (a) Compute the SNR in dB. (b) Compute the
Shannon capacity. (c) The client reports a 200 Mb/s connection rate. Is that
plausible? Justify in two sentences. *(6 marks)*

**C1.** A switch with an empty MAC address table receives a frame on port 3 with
source `aa:bb:cc:00:00:01` and destination `aa:bb:cc:00:00:09`. Ninety seconds
later it receives a frame on port 7 with source `aa:bb:cc:00:00:09` and
destination `aa:bb:cc:00:00:01`.

For each frame, state (a) what the switch records, (b) which ports the frame
exits, and (c) why. Then state what the switch does with a third frame, arriving
on port 3 for `aa:bb:cc:00:00:09`. *(9 marks)*

**C4.** Three switches are cabled in a triangle. A host sends one ARP request.
Describe, in sequence, what happens over the following two seconds if spanning
tree is not running. Your answer must explain why the situation does not resolve
itself, with reference to a specific missing header field. *(8 marks)*

**D1.** *(Choose one.)* Ethernet was less capable than Token Ring on several
measures that mattered in 1985, and displaced it entirely. Explain why, and
identify the general principle about technology adoption that the case
illustrates. *(20 marks)*

**D2.** *(Choose one.)* Explain why the OSI model's protocols failed while the
model itself became universal, and describe with a worked example how the model
is used as a diagnostic instrument. *(20 marks)*

---

## Exam 2 — Addressing and Internetworking

**Week 10 · 15% · 120 minutes · Chapters 24–51, cumulative on Units I–V**

| Section | Marks | Form |
|---|---|---|
| A: Subnetting | 30 | 8 problems, working required |
| B: Routing table interpretation | 25 | Given tables and topologies |
| C: Transport and services | 25 | Capture excerpts, symptom analysis |
| D: Wireless and WAN | 20 | Calculation and design reasoning |

Heavy on diagrams. This is the exam where a student who learned subnetting by
chart is separated from one who learned it by binary, and the /27 and /29
boundaries do that separating.

### Sample questions

**A5.** An organisation holds `172.20.0.0/16` and needs subnets for 1,200 hosts,
400 hosts, 90 hosts, 50 hosts, and four point-to-point router links.
(a) Allocate them, largest-first, showing each prefix and usable range.
(b) State the summary prefix for the whole allocation.
(c) Explain, in one sentence, what would have gone wrong had you allocated the
point-to-point links first. *(10 marks)*

**B2.** A router's table contains:

```
0.0.0.0/0        via 203.0.113.1
10.0.0.0/8       via 10.1.1.2
10.1.0.0/16      via 10.1.1.6
10.1.5.0/24      via 10.1.1.10
192.168.0.0/16   via 10.1.1.14
```

State the next hop for packets to: `10.1.5.7`, `10.1.9.200`, `10.240.0.1`,
`192.168.4.4`, `8.8.8.8`. For each, name the rule that decided it. *(10 marks)*

**B4.** Host A (`10.1.1.50/24`, gateway `10.1.1.1`) can ping `10.1.1.1` and can
ping `8.8.8.8`, but cannot ping `10.1.2.15`. Host B on the same subnet has
identical symptoms. The router's table contains a route for `10.1.2.0/24`.
Give the two most likely causes, and state the single test that would
distinguish them. *(8 marks)*

**C3.** A packet capture of a file transfer shows: three duplicate ACKs, then a
retransmission, then normal progress; forty seconds later, TCP Zero Window
messages from the receiver.

(a) What caused the first sequence? (b) What caused the second? (c) Which of the
two indicates a network problem and which does not? Justify. *(9 marks)*

**C5.** A user reports that a web application "hangs after logging in". The
connection establishes, small requests succeed, and the failure occurs on the
first page containing images. `ping` to the server succeeds; `ping -s 1400 -M do`
fails. Name the fault, explain the mechanism, and give both the correct fix and
the workaround used when the correct fix is unavailable. *(9 marks)*

**D2.** A point-to-point 5.8 GHz link is proposed over 12 km. Transmit power
23 dBm, 24 dBi dishes at both ends, 1.5 dB of cable loss each end, 20 MHz
channel, receiver noise figure 6 dB, receiver sensitivity −82 dBm at the target
rate. (a) Compute FSPL. (b) Compute received power. (c) Compute the link margin
and assess it. (d) The client asks whether doubling transmit power would let the
link reach 24 km. Answer with arithmetic. *(12 marks)*

---

## Final Exam — Integrated Networking Reasoning

**Week 15 · 20% · 180 minutes · Cumulative, weighted toward Units XI–XIV**

| Section | Marks | Form |
|---|---|---|
| A: Diagnosis | 40 | 5 unfamiliar networks with symptoms |
| B: Design reasoning | 30 | 2 scenarios requiring a justified choice |
| C: Synthesis | 20 | Trace an operation end to end |
| D: Judgement | 10 | Evaluate a proposal and a claim |

**Section A is the heart of the exam.** Each question gives a network the student
has never seen, a set of symptoms, and a set of evidence, and asks for a
diagnosis with reasoning. Marks are for the reasoning; a correct diagnosis with
no reasoning scores half.

### Sample questions

**A2.** A branch office reports that "the network is slow in the afternoons".
Evidence:

- Circuit: 200 Mb/s. Five-minute average utilisation peaks at 71% at 14:00.
- `ping` to head office at 09:00: `min/avg/max/mdev = 18.2/18.9/21.0/0.6 ms`
- `ping` at 14:00: `min/avg/max/mdev = 18.2/94.7/610.3/88.1 ms`, 0.8% loss
- `iperf3` single stream at 14:00: 11 Mb/s. With `-P 16`: 138 Mb/s.
- Head office reports no complaints from other sites.

(a) Identify the dominant problem and the evidence that establishes it.
(b) Identify the secondary problem.
(c) State which piece of evidence rules out a physical-layer fault, and why.
(d) The branch manager proposes upgrading to 500 Mb/s. Explain, with reference to
a specific relationship, why this will help less than the utilisation graph
suggests, and name two mechanisms you would investigate first. *(12 marks)*

**A4.** Users on one floor of a building intermittently lose connectivity for
periods of 20–90 seconds, several times a day, at no predictable time. Wired and
wireless users are both affected. Other floors are unaffected. Interface counters
on the floor's access switch show no errors. The switch's uptime is 340 days.

Give three candidate causes consistent with all the evidence, rank them, and for
each state the single observation that would confirm or eliminate it. *(10 marks)*

**A5.** An application works from the office and fails from home over the VPN.
The VPN connects; other applications work over it; the application's server is
reachable by `ping` from the VPN client; `nc -zv server 8443` from the VPN client
times out, while from the office it connects immediately.

State what the timeout (rather than a refusal) tells you, and give the two most
likely causes. *(8 marks)*

**B1.** A 60-person architecture practice is moving to a new floor. They run a
large on-premises file server holding 40 TB of drawings; designers routinely open
2–5 GB files. There is no in-house IT. Two options are proposed: (i) 10 Gb/s
copper to every desk, (ii) 1 Gb/s to the desk with 10 Gb/s uplinks and a faster
server.

Recommend one, with reasoning that addresses cost, the actual bottleneck, the
operability constraint, and the condition under which your answer would change.
*(15 marks)*

**C1.** Trace a cold-start load of `https://example.com` from a laptop that has
just been switched on and connected to an enterprise wireless network. Name every
protocol involved, in order, state what each accomplishes, and count the round
trips before the first byte of HTML arrives. Then name three techniques that
reduce that count and state what each removes. *(20 marks)*

**D1.** A vendor claims their product "eliminates the need for network
segmentation by using AI to detect lateral movement". Evaluate the claim in
150 words, identifying what is plausible, what is not, and what question you
would ask before purchasing. *(10 marks)*

---

## §4 — Week 13: the troubleshooting gauntlet

Teams diagnose against the clock on a network the instructor has broken. Marked
on diagnostic *process* — evidence gathered before hypotheses formed, tests that
discriminate, one change at a time — rather than on time to resolution.

### Fault scripts

Introduce three to five per session. Each is chosen to have a distinctive
signature and a tempting wrong answer.

| # | Fault | Symptom presented to the team | Tempting wrong answer |
|---|---|---|---|
| 1 | Access port moved to wrong VLAN | One host: link up, APIPA address | "DHCP server is down" |
| 2 | Subnet mask changed /24 → /25 on one host | Reaches some local hosts, not others; Internet works | "Firewall rule" |
| 3 | Default gateway address wrong by one | Local fine, remote fails entirely | "ISP outage" |
| 4 | Native VLAN mismatch on the inter-switch trunk | Two segments merged; unexpected reachability | Nobody notices — award marks for noticing |
| 5 | ICMP type 3 code 4 blocked on the firewall | SSH connects then hangs; small pings fine | "MTU is fine, ping works" |
| 6 | Speed/duplex hard-coded at one end | Works; throughput collapses under load | "Congestion, need more bandwidth" |
| 7 | Root bridge priority lowered on an access switch | Traffic takes a bizarre path; intermittent slowness | "Bad cable" |
| 8 | Static route left pointing at a decommissioned next hop | One destination black-holed silently | "Destination is down" |
| 9 | DHCP relay removed from one subnet's router | That subnet only: APIPA everywhere | "Scope exhausted" |
| 10 | Two APs set to channels 1 and 3 | Wireless throughput poor, signal strong | "Increase transmit power" |
| 11 | Duplicate IP address (static inside the DHCP pool) | Two hosts intermittently unreachable, unpredictably | "Failing NIC" |
| 12 | Wrong DNS server address in the DHCP scope | Everything by IP works, nothing by name | "Internet is down" |

### Marking the gauntlet

| Criterion | Weight |
|---|---|
| Evidence gathered before the first hypothesis | 30% |
| Tests that discriminate between hypotheses rather than confirm one | 25% |
| One change at a time, with the effect observed | 20% |
| Correct diagnosis | 15% |
| Written record produced during, not after | 10% |

Note that correct diagnosis is worth 15%. A team that diagnoses fault 6 correctly
by guessing scores less than one that reaches fault 6 methodically and runs out of
time. Announce this before starting, or teams will optimise for the wrong thing.

Fault 5 is the one most teams fail, and it is worth debriefing at length — the
"ping works so MTU is fine" reasoning is exactly the trap the book has been
warning about since Chapter 34.
