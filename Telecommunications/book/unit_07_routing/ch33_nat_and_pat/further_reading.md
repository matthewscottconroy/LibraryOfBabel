# Chapter 33 — Further Reading

## Primary sources

**RFC 1631 — Egevang, K. & Francis, P. (1994). *The IP Network Address Translator.***
**Read this.** Nine pages. It is the clearest example in the RFC series of engineers
proposing something they know is architecturally wrong, saying exactly why, and
recommending it anyway because the alternative is worse. Compare their predicted
consequences with §33.3's record.

**RFC 2663 — Srisuresh, P. & Holdrege, M. (1999). *IP Network Address Translator
Terminology and Considerations.***
The taxonomy — basic NAT, NAPT, and the behavioural classifications that STUN depends on.

**RFC 3022 — Srisuresh, P. & Egevang, K. (2001). *Traditional IP Network Address
Translator.***
The revised specification, and the one implementations follow.

**RFC 4787 / BCP 127 — Audet, F. & Jennings, C. (2007). *NAT Behavioral Requirements for
Unicast UDP.***
**What a NAT should do**, written after a decade of implementations doing different
things and breaking applications differently. Worth reading to see how much unspecified
behaviour there was.

**RFC 6598 — Weil, J. et al. (2012). *IANA-Reserved IPv4 Prefix for Shared Address
Space.***
CGNAT's `100.64.0.0/10`, and the justification — which is an admission that the previous
workaround needed a workaround.

**RFC 8489 (STUN), RFC 8656 (TURN), RFC 8445 (ICE).**
The NAT-traversal family. **RFC 8445's introduction** is the clearest statement of the
problem, and ICE's candidate-gathering procedure is a genuinely elegant response to an
environment nobody wanted.

**RFC 3948 — Huttunen, A. et al. (2005). *UDP Encapsulation of IPsec ESP Packets.***
NAT-T. Encapsulating a security protocol to hide it from a middlebox, which is Chapter 21
§21.4's ossification story in miniature.

**RFC 6296 — Wing, D. & Baker, F. (2011). *IPv6-to-IPv6 Network Prefix Translation.***
NPTv6 — the one legitimate remaining case, done statelessly.

## Papers

**Ford, B., Srisuresh, P. & Kegel, D. (2005). "Peer-to-Peer Communication Across Network
Address Translators." *USENIX Annual Technical Conference*.**
**The hole-punching paper.** Careful, systematic, and it is why peer-to-peer works across
NAT at all. §33.3's table of NAT types and their traversability comes from here.

**Guha, S. & Francis, P. (2005). "Characterization and Measurement of TCP Traversal
Through NATs and Firewalls." *ACM IMC*.**
What NATs actually do, measured rather than specified. The gap between the two is the
subject.

## Books

**Dutcher, B. (2001). *The NAT Handbook.* Wiley.**
Dated and thorough on the mechanics.

**Doyle, J. & Carroll, J. *Routing TCP/IP*, Volume 1.**
The NAT configuration chapters, with the platform detail.

**Hagen, S. (2014). *IPv6 Essentials*, 3rd ed.**
For §33.4's endgame — what replaces NAT and what has to be configured deliberately
instead.

**Cheswick, W., Bellovin, S. & Rubin, A. (2003). *Firewalls and Internet Security*,
2nd ed.**
For the §33.1 argument. Their treatment of what NAT does and does not provide is the
authoritative one, from people with no interest in overselling it.

## Applied

**`show ip nat translations`** on any Cisco router, or **`conntrack -L`** on Linux. **Look
at a real table.** Every NAT problem is visible in it, and knowing what a healthy one looks
like is the prerequisite for recognising an unhealthy one.

**`iptables -t nat -L -n -v`** or `nft list table nat` — the rules, with counters.

**Capture an FTP session in active mode across a NAT** (exercise F1), with the ALG on and
off. **Finding the `PORT` command as ASCII text in the payload, and watching the router
rewrite it, makes §33.3's layer violation concrete in a way no description does.**

**Use a STUN client** to discover your public address and determine your NAT type:

```bash
stunclient stun.l.google.com 19302
# or
pystun3
```

**Then predict whether peer-to-peer will work for you**, and test it. Exercise F2, and it
takes five minutes.

**`traceroute` from inside and outside a NAT** and compare. The translated addresses in
the ICMP payloads are a good demonstration of how deep the rewriting goes.

**Wireshark filters:** `ftp`, `sip`, `stun`, `classicstun`. A SIP call capture with the SDP
expanded shows the private address being advertised, which is §33.3's VoIP problem in one
packet.

**Lab 21** in this book's [labs/](../../../labs/) directory builds a PAT router, fills its
translation table deliberately, demonstrates the timer effects on a UDP application, then
breaks FTP and SIP across it and fixes each with an ALG and with STUN respectively.
**Lab 22** builds a double-NAT topology and catalogues what breaks.

## For the certification-minded

Objective 2.2 expects NAT, PAT and port forwarding. Objective 1.7 expects CGNAT space.
**NAT is examined directly and frequently.**

Seven things worth over-learning:

1. **Static NAT is one-to-one and permits inbound; dynamic NAT uses a pool; PAT/overload
   shares one address using ports.**
2. **Inside local** = the private address; **inside global** = its public appearance.
3. **PAT is what almost everyone runs**, and the terms NAT, PAT, NAT overload and NAPT
   are used interchangeably for it.
4. **Port forwarding is a manually-created translation entry.**
5. **`100.64.0.0/10` is CGNAT space**, and its presence means no public address.
6. **NAT breaks protocols that embed addresses** — FTP and SIP are the standard examples.
7. **NAT is not a firewall.** This appears in security questions and the expected answer
   is that NAT is not a security control.

Expect the four Cisco terms as a matching item, and a scenario asking why an FTP transfer
or a VoIP call fails.

And the two operational points worth more than the objective:

**When something works from outside and fails from inside, think hairpinning.** The fix is
split-horizon DNS, and the symptom is otherwise baffling.

**When an IPv6 deployment turns out to have inbound open, the team assumed NAT was doing
the firewall's job.** It never was, and on IPv6 there is nothing to fall back on.
