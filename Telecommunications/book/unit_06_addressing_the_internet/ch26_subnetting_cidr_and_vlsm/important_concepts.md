# Chapter 26 — Important Concepts

**Subnetting is one idea** *(§26.1)* — **Take bits from the host portion and give them to
the network portion.** Everything else is arithmetic.

**The two formulas** *(§26.1)* — **subnets = 2^b**, **hosts per subnet = 2^h − 2**, where
*b* is bits borrowed and *h* is host bits remaining. The total is fixed: every bit given
to the network is taken from the host.

**Subnetting wastes addresses** *(§26.1)* — Each subnet loses two to its own network and
broadcast, so 64 subnets of a /24 lose 128 addresses — **half the block**. Worth paying
for separate broadcast domains, security boundaries, fault isolation and routing
hierarchy. It is also why **/31 exists** for point-to-point links.

**Sizing** *(§26.1)* — Take the host requirement, add 2, round **up** to the next power
of two. **Round up again when near a boundary**: 30 hosts in a /27 fits exactly and one
more device means renumbering — which touches every static address, DHCP reservation,
firewall rule and document.

**Block size = 256 − mask octet** *(§26.1, §26.2)* — The magic number. **Subnets begin at
multiples of the block size, always.**

**The prefix table** *(§26.1)* — /25:128, /26:64, /27:32, /28:16, /29:8, /30:4, and
upward /23:512, /22:1024, /21:2048, /20:4096. **Memorise it.**

**Subnet zero is legal** *(§26.1)* — RFC 1878 (1995), default on Cisco since IOS 12.0.
Older material uses **2^b − 2** for the subnet count; **modern practice and Network+ use
2^b**. The host formula's minus-two has **not** changed.

**Do the binary first** *(§26.2)* — The shortcut is faster and the binary is what lets
you handle the unusual case and verify a doubtful answer. **Students who learn only the
shortcut can pass an exam and cannot debug a /19.**

**The magic-number method** *(§26.2)* — (1) find the **interesting octet** — the one that
is neither 255 nor 0; (2) block size = 256 − that octet; (3) **count multiples until you
pass the address's value**; (4) network = that multiple, broadcast = next multiple − 1;
(5) usable = network+1 to broadcast−1. **Ten seconds, no binary.**

**Why it works** *(§26.2)* — The block size **is** 2^h, subnets partition the octet into
equal blocks from zero, so **every boundary is a multiple of the block size**. The
shortcut is the binary method with the writing skipped.

**When the interesting octet is not the fourth** *(§26.2)* — Identical method. **The step
people get wrong:** for a /20 the broadcast's fourth octet is **255**, not 0 — the host
portion extends past the interesting octet. Zero everything after it for the network;
set everything after it to 255 for the broadcast.

**Three verification checks** *(§26.2)* — Is the network a multiple of the block size? Is
(broadcast − network + 1) the block size? Does the original address fall in the usable
range? Any failure means the arithmetic is wrong. And **subnets × block size must equal
the parent block size**.

**Supernetting** *(§26.3)* — Combine several blocks into one shorter prefix. **A routing
operation**: nobody assigns a /22 to a segment; a router **advertises** one in place of
four /24s.

**Why aggregation matters** *(§26.3)* — The global BGP table holds roughly **950,000 IPv4
prefixes**; without aggregation it would be tens of millions, exceeding what TCAM can
hold. **Aggregation is not an optimisation — it is what makes a global routing table
possible.**

**Summarising by hand** *(§26.3)* — **Find the longest common prefix in binary.**
Shortcut: **new prefix = old prefix − log₂(number of networks)**. Four /24s → /22, eight
→ /21, sixteen → /20.

**The two conditions** *(§26.3)* — (1) **contiguous**, no gaps; (2) **the summary must
start on a boundary** — the starting network must be a multiple of the combined block
size. `192.168.5.0/22` is invalid because 5 is not a multiple of 4.

**Advertising what you do not hold** *(§26.3)* — Attracts traffic destined for someone
else's network, which you then drop. At Internet scale this is a **route hijack**,
intended or not, and it has taken large portions of the Internet offline.

**The common summarisation error** *(§26.3)* — A set that is contiguous but does not
begin on a boundary — `192.168.5.0/24` through `192.168.8.0/24`. No single prefix covers
exactly four such networks; the minimal exact set is
`192.168.5.0/24` + `192.168.6.0/23` + `192.168.8.0/24`.

**Aggregation isolates change** *(§26.3)* — Larger than the table-size benefit. A new
subnet on floor 4 is invisible to another site, because that site's route is a /20 that
already covers it. **A network whose routing table churns whenever a subnet is added is
one whose stability depends on nobody doing anything.**

**Longest-prefix match makes exceptions work** *(§26.3)* — A /16 aggregate and a /24
exception coexist; the /24 wins for its range. **Aggregation does not have to be perfect
to be valuable.** It is also the mechanism route hijacking exploits — a more specific
announcement always wins.

**`0.0.0.0/0`** *(§26.3)* — Zero network bits, matches everything, and therefore **loses
to every other route**. One entry replacing the entire Internet; aggregation taken to its
logical end.

**VLSM** *(§26.4)* — Different prefix lengths within one block, each subnet sized for
what it holds. Without it, every subnet uses the largest subnet's mask: a /24 for
100+50+25+10+three links becomes **two /25s where seven subnets are needed**.

**VLSM requires a classless routing protocol** *(§26.4)* — **The protocol must carry the
mask.** RIPv1 and IGRP do not; RIPv2, EIGRP, OSPF, IS-IS and BGP do. This is the whole
technical requirement.

**Allocate largest first** *(§26.4)* — **The single most common VLSM mistake is doing it
in any other order.** A /25 has only two valid starting positions in a /24; a /30 has
sixty-four. **Large blocks have few valid starting positions; small blocks fit almost
anywhere — so place the constrained things first.** The same principle as memory
allocation and disk partitioning.

**The five steps** *(§26.4)* — List requirements **descending**; allocate largest from
the block's start; continue in descending size, each on its own boundary; **record what
is left**; verify no overlaps, all on boundaries, and **sizes summing to the parent**.

**Undocumented free space is unusable space** *(§26.4)* — Nobody assigns from a range
they cannot prove is free.

**Design for aggregation** *(§26.4)* — Allocate a **contiguous block per site or
function**, top down, then subdivide within it. A site then advertises **one prefix,
ever**, and internal changes never leave it.

**Spend the addresses** *(§26.4)* — HQ holding 65,536 addresses for 1,000 hosts is
**irrelevant**: RFC 1918 gives 16.7 million and there is no prize for efficiency.
**Optimise for comprehensibility and aggregation.** The exceptions are **public address
space**, which is scarce and expensive, and **very large scale**, which is one reason
hyperscalers went to IPv6 internally.

**A readable plan** *(§26.4)* — `10.2.1.50` should tell you "Branch 1, voice" at a
glance, from a log line, without consulting documentation. Structure in the address is
worth more than the addresses it costs.
