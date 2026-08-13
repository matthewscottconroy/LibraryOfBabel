# Appendix D — CompTIA Network+ (N10-009) Crosswalk

This appendix maps the N10-009 exam objectives to the chapters that develop them.

**Read it in the right direction.** The temptation is to use this table as a study
plan — find the objective, read the chapter, tick it off. That inverts the book's
method and produces exactly the outcome it was written to avoid: vocabulary without
mechanism.

Use it instead as a **coverage check**. Work through the book in order; when you reach
the end of a unit, consult this table to see which objectives that unit has equipped
you for. If an objective appears in a chapter you have read and you cannot answer a
question about it, that is the signal to go back — not to a summary, but to the section
that derives it.

The book covers more than the exam requires and, in places, less than a dedicated cram
guide would. It is not a substitute for CompTIA's own objectives document, which you
should download (it is free) and read once early and once shortly before sitting.

---

## Domain weights

| Domain | Weight | Primary units |
|---|---|---|
| 1.0 Networking Concepts | 23% | I, II, III, V, VI, VIII |
| 2.0 Network Implementation | 20% | IV, VI, VII, IX |
| 3.0 Network Operations | 19% | XI, X |
| 4.0 Network Security | 14% | XII |
| 5.0 Network Troubleshooting | 24% | XIII, plus every chapter's failure modes |

Note the largest domain. Troubleshooting is 24% and is examined partly through
performance-based questions, which reward the ability to reason from a symptom rather
than recall a fact. Unit XIII formalises the method; the raw material is the "what
breaks here" section that closes every chapter in the book.

---

## Domain 1.0 — Networking Concepts (23%)

| Objective | Topic | Chapters |
|---|---|---|
| 1.1 | OSI model layers and their functions | **22**; 21, 23 |
| 1.2 | Networking appliances, applications and functions | 17 (switch), 29 (router), 60 (firewall/IDS/IPS/proxy), 52 (load balancer/CDN), 40 (DHCP), 39 (DNS), 54 (SNMP/syslog) |
| 1.3 | Cloud concepts and connectivity options | **69**; 51 |
| 1.4 | Common networking ports, protocols, services and traffic types | **41**, 35, 36, 37, 39, 40; Appendix B |
| 1.5 | Transmission media and transceivers | **10**; 5, 6, 7, 42 |
| 1.6 | Network topologies, architectures and types | **11**, 14; 67 |
| 1.7 | IPv4 addressing: subnetting, VLSM, CIDR, address types | **25, 26, 27**; 2 (binary), Appendix A |
| 1.8 | Modern network environments: SDN, VXLAN, zero trust, IaC, IPv6 | **67, 68, 70**; 28, 59 |

**Where this book adds depth beyond the objective:** Chapters 1–4 (information theory,
Shannon capacity, performance measurement) are not examined and are the reason the
examined material makes sense. Chapter 2's binary fluency is the prerequisite for 1.7,
which is the objective candidates most often fail.

---

## Domain 2.0 — Network Implementation (20%)

| Objective | Topic | Chapters |
|---|---|---|
| 2.1 | Routing technologies and bandwidth management | **29, 30, 31, 32**; 52 (QoS, traffic shaping) |
| 2.2 | Switching technologies and features | **17, 19, 20**; 15, 16 |
| 2.3 | Wireless devices and technologies | **43, 44, 45**; 42, 46, 47 |
| 2.4 | Physical installations | **10**; 53 (rack diagrams, labelling), 56 (power, cooling) |

**Notes.** Objective 2.1's "bandwidth management" is Chapter 52's classification,
marking, policing and shaping. Objective 2.3 expects channel width, non-overlapping
channels, and encryption standards — Chapter 43 derives the 1/6/11 rule rather than
asserting it, which is the difference between remembering it and reconstructing it.

---

## Domain 3.0 — Network Operations (19%)

| Objective | Topic | Chapters |
|---|---|---|
| 3.1 | Documentation and organisational processes | **53, 55**; 72 |
| 3.2 | Monitoring, logging and alerting | **54**; 3 (metrics), 64 |
| 3.3 | Disaster recovery, high availability and testing | **56** |
| 3.4 | Remote access methods and security | **61**; 41 (SSH, RDP), 59 |
| 3.5 | Data centre architecture and IaC | **67, 70**; 56 |

**Notes.** This domain is the one most often under-prepared, because it is the least
technically glamorous and the most representative of the actual job. Chapter 53's three
diagrams, Chapter 54's baselines and alert design, Chapter 55's change control, and
Chapter 56's availability arithmetic map onto it directly.

---

## Domain 4.0 — Network Security (14%)

| Objective | Topic | Chapters |
|---|---|---|
| 4.1 | Security concepts: CIA, AAA, zero trust, defence in depth | **57, 59**; 58 |
| 4.2 | Common attack types | **62**; 18, 19, 20, 36, 39 |
| 4.3 | Network security features, defence techniques and solutions | **60, 61**; 20 (segmentation), 59 (802.1X), 44 (WPA2/WPA3) |

**Notes.** Chapter 57 derives the CIA triad from the three things an adversary can do,
rather than presenting it as a definition. Chapter 62 walks back down the stack
attacking each mechanism, which doubles as a review of Units IV through VIII and is the
most efficient revision in the book.

---

## Domain 5.0 — Network Troubleshooting (24%)

| Objective | Topic | Chapters |
|---|---|---|
| 5.1 | Troubleshooting methodology | **63** |
| 5.2 | Cable connectivity issues and tools | **65 §65.1**; 6, 10, 64 §64.4 |
| 5.3 | Network services and issues | **65 §65.4**; 39, 40, 18 |
| 5.4 | Performance issues | **66**; 3, 38, 52 |
| 5.5 | Tools and protocols for troubleshooting | **64**; 34 |

**Notes.** This is the largest domain and the one where understanding pays the highest
return, because performance-based questions cannot be answered by recall. Chapter 65's
**symptom index** is organised the way an exam question is organised — here is what the
user sees, what is the cause — rather than the way reference material usually is.

The specific tools named in objective 5.5: `ping`, `traceroute`/`tracert`, `nslookup`,
`dig`, `ipconfig`/`ifconfig`/`ip`, `arp`, `netstat`, `tcpdump`, `nmap`, `hostname`,
`route`, `telnet`, cable testers, toner probes, taps, Wi-Fi analysers, visual fault
locators, and device `show` commands. All are in Chapter 64.

---

## Chapters not on the exam, and why they are here

| Chapters | Subject | Why included |
|---|---|---|
| 1–4 | Information, bits, performance metrics, Shannon | The foundation that makes media, wireless and performance reasoning derivable rather than memorised |
| 5–9 | Signals, impairments, coding, modulation, multiplexing | Objective 1.5 asks *which cable*; these chapters answer *why* |
| 12–13 | PSTN, packet switching history | Explains the shape of everything that followed, including 5G slicing and MPLS |
| 21, 23 | Layering rationale, end-to-end argument | Makes the OSI model of 1.1 an instrument rather than a list |
| 32 | BGP in depth | Beyond exam scope; essential for anyone working near the Internet edge |
| 46, 47 | Cellular, IoT radio | Lightly examined; increasingly central to real networks |
| 48, 50 | Internet architecture, optical transport | Context for objectives 1.3 and 2.4 |
| 71 | The frontier | Not examined; the reason to keep reading after certification |

---

## A study sequence

If you are working toward the exam alongside the book, a defensible order:

1. **Units I–V** (Chapters 1–23) — concepts, media, Ethernet, layering. Covers most of
   domain 1.0 and 2.2.
2. **Unit VI** (24–28) — addressing. Objective 1.7, and the one to over-prepare. Drill
   with `subnet_practice.py` in [tools/](../../tools/) until it is automatic.
3. **Unit VII–VIII** (29–41) — routing, transport, services. Objectives 1.2, 1.4, 2.1.
4. **Unit IX** (42–47) — wireless. Objective 2.3.
5. **Unit X–XI** (48–56) — WAN and operations. Domain 3.0.
6. **Unit XII** (57–62) — security. Domain 4.0.
7. **Unit XIII** (63–66) — troubleshooting. Domain 5.0, the largest.
8. **Chapter 72 and the project** — synthesis, which is what performance-based questions
   are testing.

Then read CompTIA's objectives document straight through. Anything on it that does not
prompt a recollection of a mechanism is a gap; go to the chapter, not to a summary.

---

## A note on what certification is and is not

CompTIA describes N10-009 as aimed at candidates with roughly 9–12 months of networking
experience. That framing is honest and worth taking seriously in both directions.

This book, and a course built on it, establishes the conceptual foundation and a good
deal of the hands-on grounding. It does not by itself substitute for the experience,
and no book does. What it should do is make the experience, when you get it,
*intelligible* — so that the first time you meet a duplex mismatch or a PMTUD black
hole in the wild, you recognise it rather than discovering it.

That is a better outcome than a pass mark, and it happens to produce one.
