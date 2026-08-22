# 28.1 The Exhaustion Arithmetic

IPv4 ran out. This section gives the numbers, the dates, and — more interestingly — the
explanation of why a crisis that was correctly predicted in 1990 and correctly solved in
1998 was still not resolved in 2026.

## The number

$$2^{32} = 4{,}294{,}967{,}296$$

**Roughly 4.3 billion**, and the usable count is well below that.

| Reserved | Size |
|---|---|
| `0.0.0.0/8` | 16.8 M |
| `10.0.0.0/8` | 16.8 M |
| `127.0.0.0/8` | 16.8 M |
| `169.254.0.0/16` | 65 K |
| `172.16.0.0/12` | 1.0 M |
| `192.168.0.0/16` | 65 K |
| `100.64.0.0/10` | 4.2 M |
| `224.0.0.0/4` (multicast) | 268 M |
| `240.0.0.0/4` (reserved) | **268 M** |

**About 600 million addresses — 14% — are unavailable before anyone is allocated
anything.** And allocation itself is lossy: subnetting reserves two per subnet
(Chapter 26 §26.1), blocks are rounded up to powers of two, and organisations hold more
than they use.

**Effective usable public addresses: on the order of 3.2 billion**, against a world of
8 billion people holding several devices each.

## The dates

The exhaustion happened at two levels, and the distinction matters.

**IANA — the global pool — exhausted on 3 February 2011.** The last five /8s were
distributed one to each Regional Internet Registry, which was the agreed endgame policy
rather than an event.

**The regional registries then exhausted in turn:**

| Registry | Region | Exhausted |
|---|---|---|
| **APNIC** | Asia-Pacific | **15 April 2011** |
| **RIPE NCC** | Europe, Middle East | **14 September 2012** |
| **LACNIC** | Latin America | **10 June 2014** |
| **ARIN** | North America | **24 September 2015** |
| **AFRINIC** | Africa | **21 November 2019** (and contested since) |

**APNIC first**, by four years, which is the whole story of where growth was happening.

"Exhausted" means the free pool reached the final /8, at which point each registry
switched to a severe rationing policy — typically a single /22 (1,024 addresses) per new
member, once, ever.

## What happens now

IPv4 addresses did not stop existing. They became **property**.

**A transfer market.** Registries permit transfers between organisations, and addresses
are bought and sold:

| Period | Approximate price per address |
|---|---|
| 2011 | \$8 |
| 2015 | \$10 |
| 2019 | \$20 |
| 2021 | **\$50–60** |
| 2024–26 | \$30–50, volatile |

**A /16 — 65,536 addresses — trades for two to three million dollars.**

Which produced consequences worth noticing:

- **Organisations discovered they held assets.** Universities and corporations that
  received class Bs in the 1980s found themselves holding property worth millions.
  MIT sold half of its `18.0.0.0/8` in 2017.
- **IP addresses appear on balance sheets.**
- **Cost became the argument for IPv6**, where technical merit had not been. A provider
  needing a million addresses faces a fifty-million-dollar bill or an IPv6 deployment.

**The market is the pressure that a decade of advocacy did not generate.**

## Why the prediction was right and the response was slow

The exhaustion was predicted accurately and early. Frank Solensky's projections at the
IETF in 1990 put it at 1994; CIDR and RFC 1918 pushed it out; the 2011 date was
predicted within months, years in advance.

**IPv6 was specified in 1998.** So why, twenty-eight years later, is roughly half of
Internet traffic still IPv4?

### 1. NAT worked too well

Chapter 27 §27.1's argument. Private addressing plus translation meant an organisation of
10,000 employees needed **one** public address, not 10,000.

**This removed the pressure that would have driven the transition.** The crisis was
deferred past the point where anyone felt urgency — and a deferred crisis generates no
budget.

### 2. There is no benefit to being early

**The core problem, and it is structural.**

An organisation that deploys IPv6 gains **nothing** until the things it wants to reach
also have IPv6. And those things gain nothing until their users do. **The benefit is
entirely in the network effect, and the first mover captures none of it.**

Compare with something like TLS, where a site deploying HTTPS immediately protects its
own users regardless of what anyone else does. **A protocol whose benefit is
proportional to others' adoption deploys slowly**, and one whose benefit is immediate
deploys fast.

### 3. It is not backward compatible

**The decision that cost the most.**

An IPv6-only host **cannot talk to an IPv4-only host.** They are different protocols with
different address sizes; there is no partial understanding.

So the transition requires **dual-stack** — running both, for as long as the transition
lasts — which means:

- Twice the addresses to manage
- Twice the firewall rules
- Twice the routing configuration
- Twice the troubleshooting surface
- **Twice the opportunity for a security gap**, and the classic one is a firewall
  configured thoroughly for IPv4 and left open for IPv6

**Could it have been designed compatibly?** It was argued at length in 1993–94. The
proposals that preserved compatibility (notably **TUBA** and various IPv4-with-options
schemes) had their own severe problems, and the working group concluded that a clean
design was worth the transition cost.

**The estimate of that cost was wrong by decades.** Deering has been candid that the
transition difficulty was underestimated, and it is the most-debated design decision in
this book.

### 4. Everything must change

Not only routers. **Firewalls, load balancers, monitoring systems, address management,
application code that assumed 32 bits, logging systems, geolocation databases, DDoS
mitigation, licence servers, and every runbook.**

Chapter 21 §21.2's point: **the applications that broke were the ones that assumed a
32-bit address** — that stored it in an integer, or allocated 16 bytes for text. The
socket interface was general; a great deal of software was not.

## What finally moved it

Adoption is now substantial and rising, and the drivers were not the ones advocates
expected.

| Driver | Effect |
|---|---|
| **Mobile networks** | T-Mobile US, Verizon, Jio and others run **IPv6-only** cores with translation for IPv4. Hundreds of millions of devices. |
| **Large content providers** | Google, Facebook, Netflix, Cloudflare all dual-stack. If the content is there, the demand exists. |
| **Cost** | \$40 per address makes IPv6 the cheaper option at scale |
| **Government mandates** | US federal agencies required to be IPv6-only by 2025 |
| **Hyperscale internal networks** | 10/8's 16.7 million addresses are **not enough** for a large cloud provider |

**Mobile did the most.** A carrier adding ten million subscribers cannot buy ten million
addresses, and running IPv6-only with **464XLAT** (§28.4) is cheaper and simpler than
multiple layers of carrier-grade NAT.

**Current adoption** — measured by Google as the fraction of users reaching it over IPv6
— passed 20% in 2016, 30% in 2020, and is around **45–50%** now. It varies enormously by
country: India and several others exceed 70%; much of Europe is 50–60%; some regions
remain under 5%.

## The IPv6 number

$$2^{128} = 340{,}282{,}366{,}920{,}938{,}463{,}463{,}374{,}607{,}431{,}768{,}211{,}456$$

**340 undecillion.** The comparisons are all absurd and one is worth making because it
conveys the right intuition:

$$\frac{2^{128}}{5.1 \times 10^{14} \text{ m}^2} \approx 6.7 \times 10^{23} \text{ addresses per square metre}$$

**Roughly 6.7 × 10²³ addresses for every square metre of the planet's surface, ocean
included** — which is, to within a factor of a few, **Avogadro's number of addresses per
square metre.** If that comparison helps, use it; if it does not, the point is simply
that the space is not going to run out.

But the number is misleading, and understanding why is the important part.

**IPv6 is not allocated efficiently, by design.** Standard practice:

| Allocation | Prefix | Contains |
|---|---|---|
| A single subnet | **/64** | 18 quintillion addresses |
| A home or small site | **/56** | 256 subnets |
| An organisation | **/48** | 65,536 subnets |
| An ISP | /32 or shorter | 65,536 sites at /48 |

**Every subnet is a /64**, regardless of whether it holds two hosts or two thousand.
This is not waste in any meaningful sense — it is a deliberate choice that makes SLAAC
work (Chapter 29 §29.2) and eliminates subnet sizing as a design activity entirely.

**The practical space is `2000::/3`** — one eighth of the total — which is what is
currently being allocated from. **Even at deliberately extravagant allocation rates, that
is enough for centuries**, and the conservative allocation policies of IPv4 were
consciously abandoned.

> **The lesson the IPv6 designers took from IPv4 was not "use bigger addresses". It was
> "never make address conservation a design constraint again."** Chapter 26's careful
> VLSM arithmetic simply does not exist in IPv6, and that is a feature.

## What breaks here

**Assuming IPv4 exhaustion is theoretical.** It happened. New allocations are rationed
and addresses cost money.

**Assuming IPv6 is optional.** For an enterprise, arguably still true. For a mobile
carrier or hyperscaler it has not been true for a decade.

**Assuming dual-stack is free.** It doubles the operational surface, and the most common
IPv6 security failure is a firewall configured only for IPv4.

**Applying IPv4 conservation instincts to IPv6.** Using a /120 to "save space" breaks
SLAAC and gains nothing. **Use /64s.**

> **Network+ note.** Objective 1.8 expects IPv6 and the reasons for it. Over-learn:
> **IPv4 has 2³² ≈ 4.3 billion addresses; IPv6 has 2¹²⁸**; **IANA exhausted in 2011**;
> **NAT deferred the crisis**; and **IPv6 is not backward compatible with IPv4, which is
> why dual-stack exists.** The last is examined and is the key to every transition
> question.
