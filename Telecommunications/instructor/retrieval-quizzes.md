# Retrieval Quizzes

Fifteen quizzes, one per week, covering the *previous* week's material.

Five questions, twenty minutes including the review. The review is the valuable
part: the questions are chosen to expose a specific misconception, and the twenty
minutes are spent on the misconception rather than on the score.

**Why retrieval rather than review.** The effect being exploited is that
*retrieving* information strengthens memory far more than re-reading it, and that
the strengthening is greatest when retrieval is effortful. A quiz that everyone
passes easily has done nothing. Aim for a mean around 60%.

Weight these at 5% of the course total, dropping the lowest two scores. Low
stakes is essential: a graded assessment produces cramming, and cramming produces
exactly the shallow encoding this is designed to defeat.

The **misconception targeted** column is the point of each question. Announce the
answer, then spend the time on the students who chose the distractor and why it
was attractive.

---

## Week 2 — on Chapters 1–3

| # | Question | Answer | Misconception targeted |
|---|---|---|---|
| 1 | A 6,600 km fibre path. Minimum round-trip time? | ~65 ms | One-way confused with round-trip |
| 2 | A 1 Gb/s link measures 940 Mb/s with iperf3. Is it faulty? | No — that is a perfect link; the rest is header overhead | Advertised rate treated as achievable payload rate |
| 3 | `ping` reports min 41, avg 48, max 312. What does avg−min estimate? | Typical queueing delay | Only the average is read |
| 4 | Name the four latency components. Which grows with load? | Propagation, transmission, processing, queueing — only queueing | All delay assumed load-dependent |
| 5 | Upgrading a 1 Gb/s link to 10 Gb/s changes an 80 ms RTT by how much? | ~11 µs per hop — essentially nothing | "Faster link means lower latency" |

## Week 3 — on Chapter 4

| # | Question | Answer | Misconception targeted |
|---|---|---|---|
| 1 | Capacity is linear in ___ and logarithmic in ___ | Bandwidth; SNR | Both assumed linear |
| 2 | +3 dB of transmit power buys how much capacity at high SNR? | One bit/symbol/Hz | Power assumed to scale capacity proportionally |
| 3 | Thermal noise floor per hertz at 290 K? | −174 dBm/Hz | — |
| 4 | Why did 33.6 kb/s modems stop improving? | They were at the Shannon bound for a 3.1 kHz, 30 dB channel | Assumed a commercial or regulatory limit |
| 5 | Does encrypted traffic compress? | No — maximum entropy by design | WAN optimisation assumed universally beneficial |

## Week 4 — on Chapters 5–14

| # | Question | Answer | Misconception targeted |
|---|---|---|---|
| 1 | Why does a long cable round off a square wave? | It attenuates the high harmonics that constitute the corners | Treated as unexplained "signal degradation" |
| 2 | What does a line code buy, and what does it cost? | Clock recovery and DC balance; channel capacity | Encoding assumed free |
| 3 | 100 users, 1 Mb/s each, 5% active. Circuit vs packet capacity? | 100 Mb/s vs ~20 Mb/s | The multiplexing gain not felt |
| 4 | Why is twisted pair twisted? | Interference couples equally into both conductors and cancels differentially | Stated as fact without mechanism |
| 5 | A hub-based network: physical and logical topology? | Star and bus respectively | The two conflated |

## Week 5 — on Chapters 15–18, 21–23

| # | Question | Answer | Misconception targeted |
|---|---|---|---|
| 1 | State the switch algorithm in three clauses | Learn from source; forward by destination; flood if unknown | Recited as "switches are smart hubs" |
| 2 | A switch breaks up which domain? | Collision, not broadcast | The two conflated — the single most examined confusion |
| 3 | Host at 192.168.10.70/27 sends to 192.168.10.100. ARPs for what? | The default gateway (different /27 blocks) | Assumed to ARP for the destination |
| 4 | Why does Ethernet have a 64-byte minimum? | Round-trip time of the longest permitted segment, so collisions are detectable | Treated as arbitrary |
| 5 | Ping by address works, by name fails. Layers exonerated? | 1 through 6 | Layered bisection not yet automatic |

## Week 6 — on Chapters 19–20

| # | Question | Answer | Misconception targeted |
|---|---|---|---|
| 1 | Why is a Layer 2 loop catastrophic and a Layer 3 loop merely wasteful? | Ethernet frames have no TTL to decrement | Assumed switches drop loops themselves |
| 2 | Bridge priorities at default: which switch becomes root? | Lowest MAC — usually the oldest switch in the building | Assumed the biggest or most central |
| 3 | A VLAN is a ___ | Broadcast domain, so inter-VLAN traffic must be routed | VLANs created without routing |
| 4 | Native VLAN 1 one end, 99 the other. Symptom? | Silent merging of two broadcast domains, no error | Assumed the trunk would fail |
| 5 | Does link aggregation speed up a single file transfer? | No — a flow hashes to one member link | Bundling assumed to add single-flow throughput |

## Week 7 — on Chapters 24–27

| # | Question | Answer | Misconception targeted |
|---|---|---|---|
| 1 | 192.168.10.70/27: network, broadcast, usable range | .64, .95, .65–.94 | Chart-dependence exposed at a non-octet boundary |
| 2 | Given only an address, can you find the network/host boundary? | No — the mask is external information | Classful thinking persisting |
| 3 | Wrong subnet mask: what is the symptom? | Some destinations reachable, others not | Assumed total failure |
| 4 | A host shows 169.254.13.7. What does that mean? | DHCP did not answer | Read as a configuration rather than a diagnosis |
| 5 | Smallest prefix for 500 hosts? | /23 (510 usable) | Off-by-one on the −2 |

## Week 8 — on Chapters 28–33

| # | Question | Answer | Misconception targeted |
|---|---|---|---|
| 1 | Table has /8, /16, /24 and default all matching. Which is used? | Longest prefix | Assumed first match or lowest metric |
| 2 | Static route's next hop dies. What happens? | Traffic is black-holed silently; the route stays | Assumed automatic withdrawal |
| 3 | Administrative distance chooses between ___; metric between ___ | Protocols; routes within one protocol | The two conflated |
| 4 | Is NAT a firewall? | No — no policy, no inspection, no outbound restriction | Very widely believed |
| 5 | Traceroute: 40% loss at hop 6, 0% at hops 7–15. Diagnosis? | ICMP rate limiting; the path is fine | Escalated to the provider |

## Week 9 — on Chapters 34–40

| # | Question | Answer | Misconception targeted |
|---|---|---|---|
| 1 | Why must ports exist? | A packet reaching the right host is still ambiguous among its processes | Ports learned as a table |
| 2 | Flow control protects ___; congestion control protects ___ | The receiver; the network | Conflated almost universally |
| 3 | Connection refused vs connection timeout: different diagnoses? | Yes — refused means something answered; timeout means nothing did | Both read as "failed" |
| 4 | Why is TCP's reliability harmful for voice? | A packet past its playout deadline is useless, and retransmitting blocks what follows | "Reliable is always better" |
| 5 | Clients on one subnet get APIPA, others are fine. Cause? | Missing DHCP relay on that subnet's router | Investigated the failing subnet, which is fine |

## Week 10 — on Chapters 41–46

| # | Question | Answer | Misconception targeted |
|---|---|---|---|
| 1 | Doubling distance costs how much path loss? | 6 dB | Assumed linear |
| 2 | Derive why 2.4 GHz has three non-overlapping channels | 5 MHz spacing, 22 MHz occupancy → 1, 6, 11 | Memorised without derivation |
| 3 | Is channel 3 better than sharing channel 1 with a neighbour? | No — partial overlap prevents deferral and corrupts both | Intuitively appealing and wrong |
| 4 | Why can Wi-Fi not detect collisions? | A radio cannot hear over its own transmission | Assumed a protocol choice |
| 5 | One legacy client at 6 Mb/s: effect on the cell? | Degrades everyone by consuming airtime | Assumed only that client suffers |

## Week 11 — on Chapters 47–51

| # | Question | Answer | Misconception targeted |
|---|---|---|---|
| 1 | Minimum RTT to geostationary orbit? | ~477 ms | Underestimated by an order of magnitude |
| 2 | DSL vs DOCSIS: architectural difference? | DSL dedicated per subscriber; DOCSIS shared among 100–500 | "Slow at 8 p.m." unexplained |
| 3 | Why is PON downstream encrypted? | Every subscriber on the split physically receives every frame | Assumed a policy choice |
| 4 | What does QoS do when a link is oversubscribed? | Decides who suffers; it does not create bandwidth | Sold as a capacity solution |
| 5 | Shape what you ___, police what you ___ | Send; receive | Reversed, or treated as equivalent |

## Week 12 — on Chapters 52–55

| # | Question | Answer | Misconception targeted |
|---|---|---|---|
| 1 | What causes most unplanned outages? | Planned changes | Assumed hardware or attacks |
| 2 | 99.999% permits how much annual downtime, including maintenance? | 5.26 minutes | Claimed casually in meetings |
| 3 | Why does redundancy arithmetic overstate availability? | Shared fate — same rack, duct, circuit, firmware | Independence assumed |
| 4 | An alert with no defined action is what? | A graph, not an alert | Alert volume mistaken for coverage |
| 5 | A backup you have not restored is ___ | Not a backup | Learned expensively otherwise |

## Week 13 — on Chapters 56–61

| # | Question | Answer | Misconception targeted |
|---|---|---|---|
| 1 | Derive the CIA triad from what an adversary can do | Listen, alter, prevent | Memorised as a definition |
| 2 | Target 2013: which control failed? | Authorization — authentication worked correctly | The two conflated |
| 3 | Why is segmentation the specific answer to ransomware? | It makes lateral movement expensive | Treated as generic good practice |
| 4 | Firewall rule is present, correct, and has no effect. Where do you look? | Above it — a broader rule shadows it | Rule itself re-examined repeatedly |
| 5 | Traffic works one way, drops the other, config correct. Suspect? | Asymmetric routing breaking stateful inspection | Not considered |

## Week 14 — on Chapters 62–65

| # | Question | Answer | Misconception targeted |
|---|---|---|---|
| 1 | Why gather evidence before forming a hypothesis? | Acting destroys the evidence that would identify the cause | "Try the likely fix first" |
| 2 | Highest-yield opening question? | When did it last work, and what changed? | Skipped in favour of symptoms |
| 3 | Small packets work, large ones fail. Fault? | PMTUD black hole | "Ping works, so MTU is fine" |
| 4 | Late collisions on one end: two possible causes? | Duplex mismatch, or a segment too long | Read as generic congestion |
| 5 | Why did bigger buffers make things worse? | They hide congestion from TCP's loss signal, converting loss into delay | "More buffer is more headroom" |

## Week 15 — on Chapters 66–70

| # | Question | Answer | Misconception targeted |
|---|---|---|---|
| 1 | Why is the VXLAN VNI 24 bits? | 802.1Q's 4,094 VLANs were genuinely exhausted at cloud scale | Treated as arbitrary |
| 2 | Why did the OpenFlow vision not displace traditional networking? | Controller single point of failure, scale, installed base, vendor adaptation, wrong abstraction level | "It was hype" |
| 3 | What makes a cloud subnet "public"? | A default route to an internet gateway — it is routing | Assumed a subnet property |
| 4 | Security groups are stateful; network ACLs are ___ | Not — return traffic needs an explicit rule | Most common cloud fault |
| 5 | Best test of a claim about a future technology? | What does adoption cost, who bears it, who benefits, can it be incremental? | Judged on technical merit alone |

---

## Running them well

**Individually first, then in pairs.** Two minutes alone, two minutes comparing
with a neighbour, then reveal. The peer discussion converts a low score from an
embarrassment into a conversation, and the arguing is where the learning is.

**Never grade for accuracy in the moment.** Collect if you must for the 5%, but
announce the answers immediately. Delayed feedback wastes the effect entirely.

**Spend the time on distractors.** "Who chose the gateway rather than the
destination in question 3? Good — that is the reading almost everyone has on
first contact, and here is what the mask actually does." The students who chose
correctly by luck learn as much from that as the ones who did not.

**Track which questions the cohort fails.** If more than half miss the same one
two weeks running, the lecture that covered it needs rewriting rather than the
students needing reminding.
