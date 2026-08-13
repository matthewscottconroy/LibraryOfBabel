# Chapter 16 — Further Reading

## Primary sources

**Abramson, N. (1970). "The ALOHA System — Another Alternative for Computer
Communications." *AFIPS Fall Joint Computer Conference*, 281–285.**
Five pages. The random-access idea, the vulnerable-period argument, and the 18.4%
result, presented by someone who knew the figure was poor and had a better reason
than efficiency. Freely available.

**Metcalfe, R. M. & Boggs, D. R. (1976). "Ethernet: Distributed Packet Switching for
Local Computer Networks." *Communications of the ACM* 19(7): 395–404.**
**The paper.** Ten pages, and the frame format it defines is unchanged. §2's design
principles and §6's analysis of the protocol under load are the substance; §8's
measurements from the running PARC network are what made it persuasive.

**Metcalfe, R. M. (1973). "Ether Acquisition." PARC memo, 22 May 1973.**
The original memo, reproduced in various histories and on Metcalfe's own pages.
Short, and worth reading for the diagram — hand-drawn, with the aether joke already
in place.

**Boggs, D. R., Mogul, J. C. & Kent, C. A. (1988). "Measured Capacity of an Ethernet:
Myths and Reality." *ACM SIGCOMM*.**
An important corrective. The received wisdom of the 1980s was that Ethernet
collapsed above about 37% utilisation; Boggs and colleagues measured a real network
and found it sustained far higher utilisation than the folklore claimed. A good
lesson in the difference between a model's worst case and a system's behaviour.

**IEEE 802.3-2022.** Clause 4 (MAC operation, including the backoff algorithm),
Clause 28 (autonegotiation), Clause 33 and Clause 145 (PoE and 802.3bt). Freely
available six months after publication via the IEEE GET program. Clause 28's
description of parallel detection is the authoritative statement of §16.4's duplex
mismatch mechanism.

## Books

**Spurgeon, C. & Zimmerman, J. (2014). *Ethernet: The Definitive Guide*, 2nd ed.
O'Reilly.**
The best single reference. Comprehensive on the standards ladder, unusually good on
autonegotiation and its failure modes, and honest about which parts of the standard
are vestigial. Chapter 5's treatment of autonegotiation is the clearest available.

**Seifert, R. & Edwards, J. (2008). *The All-New Switch Book*, 2nd ed. Wiley.**
By an 802.3 working group participant. Deeper than Spurgeon on the mechanisms and
on why the committee decided what it did. Chapters 2 and 3 cover the MAC and full
duplex.

**Perlman, R. (1999). *Interconnections*, 2nd ed. Addison-Wesley.**
Characteristically direct on the Ethernet/Token Ring comparison and on which
arguments were real. Her assessment of what determinism was worth, and of what
switching did to the debate, is worth reading against §16.3's.

**Hafner, K. & Lyon, M. (1996). *Where Wizards Stay Up Late.* Simon & Schuster.**
Context for ALOHAnet and the ARPANET connection, and good on the personalities.

**Hiltzik, M. (1999). *Dealers of Lightning: Xerox PARC and the Dawn of the Computer
Age.* HarperBusiness.**
The PARC context: the Alto, the laser printer, and why a network was needed. Good on
Metcalfe, Thacker and Lampson, and on Xerox's failure to commercialise almost any of
it.

## Historical and analytical

**Von Burg, U. (2001). *The Triumph of Ethernet: Technological Communities and the
Battle for the LAN Standard.* Stanford University Press.**
An economic and sociological analysis of exactly §16.3's question. Argues that the
open, multi-vendor community around Ethernet beat IBM's proprietary control of Token
Ring, and that this mattered more than any technical property. Read alongside the
cost argument rather than instead of it.

**Kleinrock, L. & Tobagi, F. (1975). "Packet Switching in Radio Channels: Part I —
Carrier Sense Multiple-Access Modes and Their Throughput-Delay Characteristics."
*IEEE Transactions on Communications* 23(12): 1400–1416.**
The analytical treatment of CSMA that Metcalfe's design anticipated empirically.
Where the throughput curves come from.

## Applied

**Cisco, "Troubleshooting Ethernet Collisions"** and the equivalent notes from other
vendors.
Freely available, and the practical companion to §16.4's duplex mismatch discussion.
The counter interpretation tables are directly useful during an incident.

**Any vendor's PoE planning guide.**
The budgeting arithmetic of §16.4, with the per-platform figures and the derating
tables for bundled cable and ambient temperature. Do the arithmetic from the guide
for the platform you are actually buying, not from the standard's headline numbers.

**IEEE 802.3 working group archives, ieee802.org/3/.**
Public presentations from every task force. The 802.3bz material in particular shows
the channel measurements that determined what Cat5e could support — a standards body
reasoning in the open, which is instructive to watch.

## Tools

**`simnet.py aloha`, `simnet.py csma` and `simnet.py minframe`** in this book's
[tools/](../../../tools/) directory — reproduce the 18.4% and 36.8% ceilings, compare
CSMA/CD against pure ALOHA under load, and derive the minimum frame size from segment
length and rate.

**`ip -s link` / `show interface`** on any switch. The counters of §16.4 are visible
on live equipment, and looking at a healthy port's counters — so you know what normal
looks like — is worth doing before you need to recognise abnormal.

**Lab 03 and Lab 05** in this book's [labs/](../../../labs/) directory build a
hub-versus-switch comparison and a deliberate duplex mismatch respectively.

## For the certification-minded

Objective 1.5 expects the Ethernet standards, media and distances. Objective 1.6
expects CSMA/CD and CSMA/CA. Objectives 1.5 and 2.4 expect PoE. Objective 5.2
expects duplex mismatch and collision counters.

Five things worth over-learning:

1. **Read the standard names** rather than memorising a table.
2. **100 m for all twisted-pair Ethernet**, and 10GBASE-T needs Cat6a.
3. **Let both ends autonegotiate**, and hard-coding one end causes the fault it is
   meant to prevent.
4. **Late collisions are never normal** — duplex mismatch, or a segment too long.
5. **PoE budgets are per switch, not per port.**

And the one that is not examined and generalises furthest: **standardise the
interface, not the mechanism.** Ethernet's forty-three years are the demonstration.
