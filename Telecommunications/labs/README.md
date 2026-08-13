# Labs

Fifteen hands-on labs, one per week of a semester course, each tied to the
chapters it exercises.

Every lab follows the same structure, and the structure is the pedagogy:

| Section | Purpose |
|---|---|
| **Corresponds to** | The chapters this lab makes concrete |
| **Objectives** | What you will be able to do afterwards |
| **You will need** | Equipment, software, access |
| **Procedure** | Numbered steps, with the commands |
| **Expected observations** | What you should see — so you know when something is wrong |
| **Break it** | A deliberate fault to introduce and diagnose |
| **Debrief** | Questions to answer *in writing* before you leave |

## The debrief is the assessment

Read this part even if you skip the rest of this file.

The temptation in a networking lab is to get the green light and stop. That
produces students who can follow a procedure and cannot diagnose a network they
did not build — which is, per Chapter 63, most of the job.

**These labs are assessed on the debrief, not on the outcome.** A student whose
lab did not work and who can explain precisely why has done better work than a
student whose lab worked and who cannot explain what happened. The debrief
questions are written to be unanswerable by someone who followed the steps
without watching.

The "Break it" section exists for the same reason. Every lab deliberately
introduces a fault, because a mechanism you have only ever seen working is a
mechanism you do not yet understand. You learn what ARP is for by watching what
happens when the cache is wrong.

## Equipment

The labs are written to degrade gracefully. Each states the ideal equipment and
a fallback, because most courses do not have a rack of switches per student.

**Minimum viable:** one Linux machine (a virtual machine is fine) with `ip`,
`tcpdump`, `dig`, `ss`, `iperf3`, and Wireshark. Roughly two thirds of the labs
run entirely on this.

**Better:** two or more machines on a shared segment, plus a managed switch and
a router. Labs 3–7 and 11–13 use these where available.

**Simulation:** where physical equipment is unavailable, Cisco Packet Tracer,
GNS3 or Containerlab substitute for the switching and routing labs. Each lab
notes which parts survive simulation and which do not — you cannot learn to
recognise a marginal cable in Packet Tracer, and the lab says so rather than
pretending.

**Wireless labs** (9) need a Wi-Fi analyser. A phone application is sufficient
for most of it; a spectrum analyser is much better and the lab explains what it
shows that the phone cannot.

## Safety and ethics

Two rules, and they are not formalities.

**Do not scan, capture, or test against networks you do not administer.** Lab 8
uses `nmap`; use it only against your own lab hosts. Packet capture on a network
you do not own may be unlawful in your jurisdiction regardless of intent.

**Capture responsibly.** A packet capture on a shared network records other
people's traffic. Capture on your own interface, filter narrowly, and delete
captures when the lab is finished.

## The lab sequence

| # | Week | Title | Chapters |
|---|---|---|---|
| 01 | 1 | Inspecting a Working Host | 1, 2, 3, 23 |
| 02 | 2 | Media, Signals, and Link Budgets | 4, 5, 6, 10, 42 |
| 03 | 3 | Building a LAN; Hubs, Switches and Sharing | 9, 11, 13, 17 |
| 04 | 4 | Frames, MAC Addresses, and ARP | 15, 16, 17, 18 |
| 05 | 5 | VLANs, Trunks, and Spanning Tree | 19, 20 |
| 06 | 6 | Subnetting and the Address Plan | 25, 26, 27 |
| 07 | 7 | Routing Between Networks | 29, 30, 31, 33, 34 |
| 08 | 8 | Ports, TCP, DNS, and DHCP | 35, 36, 37, 39, 40, 41 |
| 09 | 9 | Wireless Survey and Analysis | 42, 43, 44, 45 |
| 10 | 10 | WAN Behaviour, Latency, and Tunnels | 3, 49, 51, 61 |
| 11 | 11 | Documentation, Monitoring, and Baselines | 53, 54 |
| 12 | 12 | Segmentation, Firewalls, and TLS | 58, 60, 62 |
| 13 | 13 | The Troubleshooting Gauntlet | 63, 64, 65, 66 |
| 14 | 14 | Overlays, Cloud, and Automation | 67, 69, 70 |
| 15 | 15 | Design Review and Defence | 72 |

Labs 13 and 15 are different in kind. Lab 13 is a competition: the instructor
breaks a working network in several ways and teams diagnose against the clock.
Lab 15 is a structured critique of another team's design from the semester
project — because defending a design against someone who wants to find its
weaknesses is the skill Chapter 72 is about, and it cannot be practised alone.

## A note on Wireshark

Six of these labs use packet capture, and it is worth investing an hour early to
get comfortable with it. The three things that repay learning immediately:

- **Display filters** — `tcp.port == 443`, `dns`, `arp`, `tcp.analysis.flags`,
  `ip.addr == 192.168.1.1`. Type them into the bar; the syntax autocompletes.
- **Follow → TCP Stream**, which reassembles a conversation into readable form.
- **Statistics → Conversations**, which answers "what is actually on this link"
  faster than anything else.

Chapter 64 §64.3 covers the rest, including where to capture, which matters more
than how.
