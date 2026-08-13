# 48.3 IANA and the RIRs

**Every IP address you use was delegated to you through a chain that begins in one place**, and
tracing it explains both how the system works and why it is under strain.

## The chain

```
                    ┌──────────────┐
                    │  IANA / PTI  │   the global pools
                    └──────┬───────┘
        ┌──────────┬───────┼────────┬──────────┐
     ┌──┴──┐   ┌───┴──┐ ┌──┴───┐ ┌──┴────┐ ┌───┴────┐
     │ARIN │   │ RIPE │ │APNIC │ │LACNIC │ │AFRINIC │   five RIRs
     └──┬──┘   └───┬──┘ └──┬───┘ └──┬────┘ └───┬────┘
        │          │       │        │          │
     LIRs / ISPs / large end users  ────────────┘
        │
     end users
```

**IANA** — the Internet Assigned Numbers Authority — **holds the top-level pools**: IPv4 and
IPv6 address space, autonomous system numbers, and **the registries of protocol parameters**
that every RFC refers to. Port numbers (Chapter 35), DNS record types (Chapter 39), ICMP
types (Chapter 34), TLS cipher suite identifiers — **all of them are IANA registries**, and
"IANA-assigned" in a specification means "look it up in a table maintained outside this
document."

**IANA's functions are performed by PTI, a subsidiary of ICANN**, under contract to the
Internet community. **Until 2016 the contract was with the US Department of Commerce**, and
the transition away from that arrangement — the **IANA stewardship transition** — was one of
the more consequential governance events in the Internet's history, and one of the least
noticed.

**The five RIRs** each serve a region, allocate to **local Internet registries** (ISPs and
large organisations) and directly to end users, **and set their own policies through their own
member communities.**

| RIR | Region | Note |
|---|---|---|
| **ARIN** | North America | |
| **RIPE NCC** | Europe, Middle East, Central Asia | **the largest membership** |
| **APNIC** | Asia-Pacific | **exhausted IPv4 first, in 2011** |
| **LACNIC** | Latin America, Caribbean | |
| **AFRINIC** | Africa | **in prolonged governance and legal difficulty** |

> **The RIRs are membership organisations, not regulators.** Their policies are made by the
> people they apply to, in open meetings, by consensus. **Nobody voted for this system and it
> has held for thirty years.**

## Allocation versus assignment, and why the words matter

| Term | Meaning |
|---|---|
| **Allocation** | address space given to an LIR **to sub-delegate** |
| **Assignment** | address space given to an end user **to use** |
| **PA** — provider aggregatable | **the ISP's space, lent to you; you return it when you leave** |
| **PI** — provider independent | **yours, portable between providers** |

**The PA/PI distinction is the one with operational consequences.**

**PA space is aggregated.** Your ISP announces one large prefix covering thousands of
customers, **and the global routing table stays small** (Chapter 32 §32.2). **You cannot take
it with you**, so changing provider means renumbering — which for a small organisation behind
NAT is trivial and for a large one is a project.

**PI space is yours.** You announce it yourself, from your own AS number, and **you can change
providers without renumbering.** The cost is a **separate entry in every router's table on the
Internet** — which is why RIR policy has always restricted PI allocations, and why the routing
table grew as those restrictions loosened.

> **Every PI prefix is a small permanent tax on every router in the world.** This is the
> tension at the heart of Chapter 32 §32.2's routing table growth, and it is a policy problem
> wearing an engineering problem's clothes.

## IPv4 exhaustion, and what replaced allocation

**The free pool is gone.**

| Event | Date |
|---|---|
| **IANA's last /8s distributed to the RIRs** | **February 2011** |
| APNIC's final /8 policy | April 2011 |
| RIPE NCC's final /8 policy | September 2012 |
| ARIN's free pool exhausted | September 2015 |
| **All RIRs effectively exhausted** | **by 2020** |

**What remains is a waiting list and a market.**

**The transfer market.** IPv4 space is now bought and sold, with the RIRs recording transfers
and applying policy to them. **Prices rose from roughly $10 per address in 2015 to $40–60 by
2021**, and have since softened somewhat — **but the direction over a decade has been sharply
upward.**

**Which means IPv4 addresses became an asset.** Organisations that received large allocations
in the 1980s — universities, early corporations, government departments — **discovered they
were holding property worth tens of millions of dollars.** Several sold. **MIT sold half of
18.0.0.0/8 in 2017**; Stanford, Merck, DEC's successor and others have made comparable
disposals.

> **This is the exhaustion's real consequence: an address is no longer a technical resource
> allocated on need, but an asset traded on price** — **and a new entrant now pays what an
> incumbent received free.** Chapter 28's argument for IPv6 is fundamentally this one.

**And it produces a second-order effect worth knowing.** Traded space is often **fragmented** —
a /24 here, a /22 there — **which cannot be aggregated**, so the transfer market directly
accelerates routing table growth.

## IPv6 allocation, which is deliberately different

**The RIRs learned from IPv4 and set policy to prevent the same outcome.**

| Recipient | Typical allocation |
|---|---|
| **LIR / ISP** | **/32 minimum**, larger on justification — often /29 or shorter |
| **End site** | **/48** — 65,536 subnets |
| **Home customer** | **/56 or /48** — never a single address |
| Point-to-point link | /64 (or /127 for the link addresses) |

**The design principle is that sparse allocation is a feature.** IPv4 policy required
justifying every address, which produced fragmentation and constant renumbering. **IPv6 policy
gives everyone far more than they need**, on the reasoning that **aggregation is worth more
than density** (Chapter 28 §28.2).

**A common misunderstanding worth correcting:** giving a household a /56 is **not** wasteful.
The current global unicast range, `2000::/3`, contains $2^{53}$ — about nine thousand million million — distinct /56s, and
**the constraint IPv6 was designed around is routing table size rather than address supply.**

> **The scarce resource in IPv6 is not addresses. It is routing table entries** — and every
> policy decision follows from that.

## Looking things up

**Which is the practical skill this section exists for.**

**WHOIS** — the old protocol, plain text, port 43. **RDAP** — the modern replacement, JSON
over HTTPS, structured and internationalised. **Both answer the same question.**

```
   $ whois 8.8.8.8
   NetRange:       8.8.8.0 - 8.8.8.255
   CIDR:           8.8.8.0/24
   OrgName:        Google LLC
   Country:        US
   OriginAS:       AS15169
   Abuse contact:  network-abuse@google.com
```

**What it tells you, and what it does not:**

| Reliable | **Unreliable** |
|---|---|
| **Which RIR holds the range** | **Geolocation** — the registered country is the holder's, not the host's |
| **The registered organisation** | **Current use** — space may be leased or sub-delegated invisibly |
| **The abuse contact** | **Whether the announcement is legitimate** — check RPKI (Chapter 32 §32.4) |
| The allocation date | The actual operator, for delegated blocks |

**Three habits worth acquiring:**

**Check the origin AS against RPKI**, not against WHOIS. **WHOIS records who was given the
space; RPKI records who is authorised to announce it**, and during a hijack those differ.

**Use the RIR's own service** rather than an aggregator, for authoritative data —
`rdap.arin.net`, `stat.ripe.net`, and APNIC's equivalents.

**Read the `inetnum` hierarchy**, not just the first match. A /24 inside a /16 will show the
sub-delegation, and **the abuse contact you want is usually the more specific one.**

## What breaks here

**A WHOIS lookup naming a country that contradicts the traffic's behaviour.** The registration
is the holder's address. **Geolocation databases are separate, commercial, and frequently
wrong.**

**An abuse report to a WHOIS contact bouncing.** Extremely common; the records decay. **Try
the more specific delegation, or the upstream.**

**Space announced by an AS that WHOIS does not associate with it.** Could be legitimate — a
customer announcing PA space — **or a hijack.** RPKI settles it.

**Losing addresses when changing ISP.** PA space. **Expected, and the reason to consider PI
or, better, IPv6.**

**A `/24` refused by upstreams.** Many networks filter prefixes longer than /24 in IPv4 and
/48 in IPv6. **An announcement smaller than the accepted minimum will not propagate.**

**IPv6 space requested and a /64 offered for a whole site.** **Push back.** A site gets a /48
or at minimum a /56; a /64 is one subnet and it breaks SLAAC on everything behind it
(Chapter 28 §28.3).

> **Network+ note.** Objective 1.7 touches address management. Over-learn: **IANA delegates to
> five RIRs, which delegate to ISPs and end users**; **public addresses are allocated, not
> chosen**; and **the IPv4 free pool is exhausted.** The PA/PI distinction is not examinable
> and it is the one that will matter in your first job.
