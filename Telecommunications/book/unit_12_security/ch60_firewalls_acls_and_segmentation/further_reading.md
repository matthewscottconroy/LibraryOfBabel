# Chapter 60 — Further Reading

## Read these first

Cheswick, W., Bellovin, S. & Rubin, A. — *Firewalls and Internet Security* (2nd ed., 2003).
The book that defined the field, and it is largely about failure — what their own network
experienced and what did not work. Dated in its specifics and excellent in its reasoning.

Cheswick, W. (1992). "An Evening with Berferd, in Which a Cracker is Lured, Endured and
Studied." USENIX.
Read this one for pleasure. The first documented honeypot, and it is funny and slightly
uncomfortable.

Bellovin, S. (1989). "Security Problems in the TCP/IP Protocol Suite."
Almost every attack in Chapter 62 is in this paper, written when the Internet had a few
hundred thousand hosts. Read it and note the dates on the fixes.

Ranum, M. (2005). "The Six Dumbest Ideas in Computer Security."
**Four pages.** "Default permit" and "enumerating badness" are the two to internalise, and
the argument has aged extremely well.

## Practice

Zwicky, E., Cooper, S. & Chapman, D. — *Building Internet Firewalls* (2nd ed.).
Older, and still the most thorough treatment of firewall architecture and of designing
policy — as opposed to configuring a product.

Vendor hardening and architecture guides — Palo Alto, Fortinet, Check Point, Cisco. Read
two from different vendors; where they agree is the actual practice, and where they disagree is
usually a product feature rather than a principle.

**CIS Benchmarks** for your firewall platform (Chapter 55's reading) — directly usable as the
security portion of a golden configuration.

NIST SP 800-41 — "Guidelines on Firewalls and Firewall Policy."
Free, vendor-neutral, and a reasonable checklist — dated, and the architecture sections
hold.

**NIST SP 800-125B**, and the NSA/CISA guidance on network segmentation and on securing
network infrastructure devices.
The CISA network infrastructure guidance is short and directly actionable, and the
management-plane sections are §60.4's argument stated by people who investigate the
consequences.

## Segmentation and zero trust's boundary

PCI DSS's segmentation guidance and the associated scoping documents.
Read them even if you have no cardholder data — they are the clearest published statement
of what "adequately segmented" means and how it is assessed, and the reasoning transfers.

The Purdue Model / ISA-95 levels, and **IEC 62443**, for industrial and operational
technology segmentation.
Essential if you touch OT, and the availability-first inversion of Chapter 57 §57.2's
triad is assumed throughout.

VMware's, Illumio's and Cisco's microsegmentation material, read critically.
The discovery-then-label-then-monitor sequence in §60.4 is common to all of them, which
suggests it is real rather than marketing.

## TLS inspection, honestly

Durumeric, Z. et al. (2017). "The Security Impact of HTTPS Interception." NDSS.
The measurement study behind §60.3's claim that inspecting middleboxes frequently weaken the
connection. Read the results section, and take it to whoever is proposing the deployment.

de Carné de Carnavalet, X. & Mannan, M. — work on TLS-intercepting client software.
The endpoint half of the same problem.

Your own device's behaviour — **F5 tests it**, and the result should be checked rather than
assumed for any product you deploy.

## Detection

Roesch, M. (1999). "Snort — Lightweight Intrusion Detection for Networks." LISA.
**The design paper**, and it explains the rules language's shape.

Paxson, V. (1999). "Bro: A System for Detecting Network Intruders in Real-Time."
**The alternative philosophy** — parse protocols, produce structured records, let the analyst
program the detection. The argument that won.

Sanders, C. & Smith, J. — *Applied Network Security Monitoring*.
The practical book on running detection, and it is honest about tuning, false positives and
analyst time — the parts that determine whether a deployment works.

Bejtlich, R. — *The Practice of Network Security Monitoring*.
The case for collecting rich data rather than relying on alerts, argued clearly.

**MITRE ATT&CK** (Chapter 57's reading) — use it as a coverage check against your detection
rules.

## Tools

`nftables` / `iptables` / `pf` — **F1 uses one.** Building a complete policy by hand on a
Linux host teaches more about firewalls than any product course, and the concepts transfer
directly.

**`hping3`, `nmap`, `scapy`** — for testing your own policy. **F4 uses `hping3`.** Only
against infrastructure you own.

**Batfish** (Chapter 55's reading) — **F2.** Shadow detection, reachability analysis, and
differential analysis between two policy versions. It answers "will this change alter what is
reachable?" before the change is applied.

**Suricata and Zeek** — **F6.** Both run on a laptop against a span port or a capture file,
and a week of real traffic is more instructive than any documentation.

**`mitmproxy`** — **F5.** A TLS-inspecting proxy you can inspect, which is the only way to
see what interception actually negotiates.

`nmap` with `--script` for firewall evasion checks, and **firewalk-style tooling** — for
verifying that a policy does what the document says. F7 uses scanning; only where authorised.

A console server and an out-of-band path — not a research tool, and §60.4's argument is
that it is the highest-value unglamorous purchase in this chapter.

## Following the field

The vendors' threat research blogs — Palo Alto Unit 42, Cisco Talos, Fortinet FortiGuard,
Check Point Research. Read them for the technical detail and discount the conclusions, which
are always that you need more of their product.

**`ipSpace.net`** (Chapter 51's reading) — consistently the most sceptical writing available on
microsegmentation and on what firewall products actually do.

The `netfilter` and `suricata` mailing lists, for the open implementations.

## Where to look next

**Chapter 61** covers the tunnels that cross these boundaries and the remote access that
bypasses them; **Chapter 62** attacks everything in this chapter; **Chapter 65** uses the ACL
and the connection table as diagnostic instruments; and **Chapter 67 §67.3** covers the overlay
networks in which §60.4's microsegmentation is actually enforced.
