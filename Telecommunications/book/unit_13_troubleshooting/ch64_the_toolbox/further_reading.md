# Chapter 64 — Further Reading

## Read these first

Sanders, C. — *Practical Packet Analysis* (3rd ed.).
The book for §64.3. It teaches reading captures rather than using Wireshark, which is
the harder and more useful skill, and the case studies are realistic.

Chappell, L. — *Wireshark Network Analysis*, and *Troubleshooting with Wireshark*.
More comprehensive and more Wireshark-specific. Chappell's material on the Expert
Information system and on time-based analysis is the best available.

**The `tcpdump` and `pcap-filter` manual pages.**
**Genuinely.** `man pcap-filter` is the complete capture filter language in four pages, and
reading it once removes the need to search for syntax ever again.

Wireshark's own documentation and the display filter reference (wireshark.org).
Free, comprehensive, and the display filter reference is a lookup table you will use
constantly.

## The tools' own documentation

`man ping`, `man traceroute`, `man mtr`, `man dig`, `man ip`, `man ss`, `man nmap`.
Each is short and each contains options you do not know about. `man ip` in particular is
frequently skipped by people who have used `ip` for years, and `ip route get`, `ip -s -s link`
and `ip monitor` are all in it.

**Lyon, G. — *Nmap Network Scanning*.**
The reference, by the author. Free online in substantial part. Read the "Legal Issues"
chapter before running anything against something you do not own — it is more careful and more
useful than most treatments.

The `iperf3` documentation and ESnet's fasterdata guides (fasterdata.es.net).
ESnet's material on measuring high-bandwidth-delay paths is the best available, and it
explains the single-stream trap of §64.4 properly with worked examples.

## Reading captures well

Stevens, W. R. — *TCP/IP Illustrated, Volume 1* (2nd ed., Fall & Stevens).
The book that teaches what the packets mean. Its method is to show the capture and explain
it, chapter after chapter, and it remains the best way to learn to read TCP behaviour.

**Wireshark's sample captures** (wiki.wireshark.org/SampleCaptures).
Dozens of protocols, many with known faults deliberately included. F4 can be done against
these before creating your own.

**PacketLife.net's cheat sheets** — `tcpdump`, Wireshark display filters, and the protocol
headers, on single pages. Print the display filter one.

`pcapr`, `malware-traffic-analysis.net`, and the various capture challenge sites.
Practice material with answers. The Wireshark "packet challenges" in particular are a good
way to build the pattern library of §64.3.

## Physical layer

Fluke Networks' and Viavi's application notes on cable certification and OTDR
interpretation.
Vendor material, and unusually good — reading an OTDR trace is a genuine skill and their
guides teach it better than any textbook.

The BICSI Telecommunications Distribution Methods Manual, and **ANSI/TIA-568**.
Consult rather than read. Relevant when you must decide whether a cable plant meets a
standard rather than merely working.

**Your transceivers' data sheets.** The Rx sensitivity figure in §64.4 is in them, and
knowing it turns `show interface transceiver` from a number into a diagnosis.

## Measurement, properly

Paxson, V. — "Strategies for Sound Internet Measurement" (2004), and "End-to-End Internet
Packet Dynamics" (1997).
How to measure without fooling yourself, from the person who established the field. The
2004 paper's list of ways measurements go wrong is directly applicable to §64.4.

**RIPE Atlas** (Chapter 48's reading) — for measuring from somewhere other than where you
are, which is frequently the observation that settles a dispute.

**`flent`, `netperf`, `smokeping`** — for continuous and structured measurement rather than
one-off tests. SmokePing in particular plots latency and loss over time to a set of targets
and takes twenty minutes to set up, and it is Chapter 54 §54.1's most under-collected
baseline.

## Tools worth knowing exist

**`tshark`** — **Wireshark's command-line version.** `tshark -r file.pcap -Y '<filter>' -T
fields -e <field>` extracts columns for analysis in a spreadsheet or a script, which is how
large captures are actually analysed.

**`termshark`** — a terminal interface to the same, for when there is no display.

**`ngrep`** — `grep` for packet payloads, and quicker than a full capture when you know the
string.

**`socat` and `nc`** — for constructing a test connection by hand, which frequently answers
a question faster than reasoning about it.

**`hping3`, `scapy`** — for constructing packets that do not otherwise exist. Scapy in
particular turns "what would happen if…" into a two-line experiment.

**`ethtool`** — the Linux tool for interface hardware: negotiated speed and duplex, error
counters, ring buffer sizes, offload settings. Chapter 66 §66.2's duplex investigation runs
on it.

**`arping`** — **ARP-level reachability**, which works when ICMP is filtered and confirms Layer 2
independently of Layer 3.

**`mtr --report --json`**, and the various JSON output modes — for putting results into a
ticket or a script rather than a screenshot.

**PathPing** (Windows) — `tracert` and `ping` combined, and it is `mtr`'s less capable
built-in equivalent.

## On the legal and ethical side

Your jurisdiction's computer misuse legislation, and your organisation's monitoring
policy.
Know both before you need them. In the UK the Computer Misuse Act and the Investigatory
Powers Act both bear on capture; in the US the Wiretap Act and the CFAA; in the EU, GDPR
applies to captured content.

**Your organisation's data protection officer.** Chapter 54 §54.4's advice applies with more
force here: they will not object to diagnostic capture and they will want it documented,
and doing so in advance is substantially cheaper.

## Where to look next

**Chapter 65** is the catalogue of what these tools find, organised by layer; **Chapter 66**
uses them for the specific and difficult case of performance complaints; **Chapter 54** is where
the same data is collected continuously rather than on demand; and **Chapter 63** is the method
that determines which of these tools to reach for.
