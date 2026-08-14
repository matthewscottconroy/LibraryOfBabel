# 48.2 Peering, Transit and IXPs

Chapter 32 gave the routing policy. This section gives the invoice, because the policy is
a consequence of the invoice and not the other way round.

## The two relationships

| | **Transit** | **Peering** |
|---|---|---|
| Shape | **customer buys from provider** | **lateral, between equals** |
| What you receive | **the whole Internet** | **that peer and its customers only** |
| What you send | anywhere | **to that peer's customers only** |
| Money | **you pay** | **usually nobody pays** |
| Contract | **a real one, with an SLA** | **frequently an informal agreement, or none** |
| BGP announcement | **your routes go everywhere** | **your routes go to that peer alone** |

**The critical asymmetry:** a transit provider carries your traffic to third parties; a
peer **does not.** If AS 100 peers with AS 200, AS 100 may reach AS 200's customers and
may not reach AS 200's other peers or AS 200's own transit providers through it.

A network that does forward such traffic is leaking (Chapter 32 §32.4), and it is the
commonest serious BGP misconfiguration — because the default behaviour of a router with no
export policy is to announce everything to everyone.

> **Peering is defined by what it excludes.** The routing filter is the relationship.

## Ninety-fifth percentile billing

The convention that governs transit pricing, and it is not obvious the first time you meet
it.

The provider samples the port's throughput every five minutes in both directions,
producing 8,640 samples per month. At the end of the month:

1. Discard the highest 5% — 432 samples, which is **36 hours** of the month
2. Bill the highest remaining sample
3. Bill inbound or outbound, whichever is larger — not the sum

```
   Mb/s
   9000 ┤        ╭╮                          ← discarded: brief spikes
   8000 ┤   ╭╮  ╭╯╰╮      ╭╮                   (top 5% = 36 hours free)
   7000 ┤╭─╮╯╰──╯  ╰──╮╭──╯╰─╮   ← 95th percentile: the billed figure
   6000 ┤╯ ╰          ╰╯     ╰──╮
        └────────────────────────────────▶
                     one month
```

**Two consequences that shape real engineering decisions:**

**Brief spikes are free.** A nightly backup, a software release, a flash crowd — 36 hours of
peaks per month cost nothing. So an operator does not need to provision for the peak; they
provision for the 95th percentile.

**Sustained load is what costs.** Shifting a transfer from a busy hour to a quiet one reduces
the bill directly, which is why bulk transfers are scheduled overnight and why some CDNs
deliberately pace their fill traffic.

And it explains a class of operational behaviour that otherwise looks strange: an operator
watching a graph closely at the end of the month, or moving traffic between providers for a
few days. They are managing the 95th percentile, not the capacity.

## The peering decision, done properly

**It is a spreadsheet.**

**A regional ISP:**

| | |
|---|---|
| 95th percentile transit today | **8 Gb/s** |
| Transit price | **$0.30 per Mb/s per month** |
| **Transit cost today** | **$2,400/month** |
| Traffic reachable via the local IXP's members | **60%** |
| 95th percentile after peering | **3.2 Gb/s** |
| **Transit cost after** | **$960/month** |
| **Transit saved** | **$1,440/month** |

**Against:**

| | |
|---|---|
| IXP port (10 G) | **$800/month** |
| Cross-connect and colocation | **$300/month** |
| Router capacity | **capital, amortised** |
| Engineering time | **real, and often underestimated** |

So it saves $340/month before capital and staff — **marginal**, and the decision turns on
things the arithmetic does not show:

Latency and quality improve whether or not the money works. One hop to a content network
beats three hops through a transit backbone.

**Resilience improves** — a transit failure no longer removes access to the majority of what
users want.

Transit prices fall as volume falls, but they fall in steps and commitments are usually
contractual for a year or more, so the saving may not be realisable immediately.

**And the traffic will grow.** At 20 Gb/s the same peering decision saves $3,600/month against
the same $1,100 of port cost. The decision is usually made ahead of the arithmetic, in
anticipation.

> The rule of thumb in the industry: peer early, because the cost of an IXP port is fixed
> and the cost of transit is not.

## Internet exchange points

An IXP is a Layer 2 switching fabric in a neutral facility. That is all it is.

```
   ┌─────────────────────────────────────────────────┐
   │            IXP switching fabric                 │
   └──┬────┬────┬────┬────┬────┬────┬────┬────┬──────┘
      │    │    │    │    │    │    │    │    │
     ISP  ISP  CDN  CDN cloud ISP  uni  gov  content
      A    B    P    Q    R    C    D    E     F
```

Each participant connects once and can then establish a BGP session with any other
participant over that single port. N networks require N connections rather than N(N−1)/2
cables, which is the entire economic argument.

**The IXP itself does not route.** It carries no traffic of its own, sets no policy about who
peers with whom, and — in the European model — is typically a non-profit owned by its
members. The US model has more commercial operators.

| Exchange | Location | Scale |
|---|---|---|
| **DE-CIX** | Frankfurt | **peak traffic in the 10+ Tb/s range**; over 1,000 participants |
| **AMS-IX** | Amsterdam | comparable |
| **LINX** | London | comparable |
| IX.br | São Paulo | very large, and growing fastest |
| Equinix IX, DE-CIX NY | US | smaller relative to the market — the US peers more privately |

**Route servers** are the mechanism that makes a large IXP usable. Rather than configuring
1,000 individual BGP sessions, a participant peers with the exchange's route server and
receives the routes of every member that also uses it. One session, hundreds of peers.

**The trade is control:** route-server peering is open by default, so a network with selective
peering policy still configures bilateral sessions with the networks that matter. Most
operators do both — the route server for the long tail, bilateral sessions for the large
flows.

## Private interconnect

Once a flow is large enough, the IXP fabric stops being the right place for it.

A private network interconnect (PNI) is a direct cable between two networks in the same
facility — typically 100 G or multiples of it.

**Why move off the shared fabric:**

- **Capacity** — a 400 G flow should not share a fabric with everyone's traffic
- **Predictability** — no other participant's congestion affects it
- **Cost per bit** — a cross-connect is cheap; incremental IXP port capacity is not
- **Troubleshooting** — one cable, two ends, no shared infrastructure to eliminate

> The progression is standard: peer at the exchange, watch the flow grow, move it to a PNI,
> add more PNIs in more cities. Every large content-to-eyeball relationship has followed
> this path.

## The disputes

The recurring public argument, and it is worth being precise about it because both sides'
public statements are misleading.

**The setup:** a content network sends far more than it receives. An eyeball network says
this is unbalanced and asks to be paid; the content network says it is delivering what
that network's own customers requested, and the eyeball network has already been paid — by
those customers.

**Both statements are true.** They are arguments about who captures the value, not about
engineering.

**Comcast–Netflix (2013–14)** is the case worth knowing. Netflix traffic to Comcast
customers degraded severely during a peering dispute routed through congested transit;
Netflix eventually paid Comcast for direct interconnection, and performance recovered
immediately. The mechanism was not throttling but the deliberate non-upgrading of a
congested port, which is harder to characterise and equally effective.

**The traffic-ratio criterion** — a conventional requirement that peers exchange traffic within
some ratio, often 2:1 — is a rule invented by eyeball networks and it is not a technical
requirement. It exists because it gives leverage. Content networks argue that the ratio
is a consequence of consumer behaviour rather than of anything they control, which is
correct.

> **This is where network engineering meets commercial negotiation**, and an engineer who
> treats it as a purely technical question will be surprised by decisions that make no
> technical sense. **The routing follows the contracts.**

## What breaks here

Traffic to a peer routing via transit instead. The peering session is down, or the
prefixes are filtered. Check the BGP session state and the received prefix count.

**A transit bill much larger than expected.** Something sustained changed — a new backup
schedule, a leaked route attracting third-party traffic, a compromised host. Look at the
95th percentile graph, not the total.

Degradation at peak only, to one destination network. **A congested interconnect**, and
you may not own either end. This is the Comcast–Netflix pattern in miniature and it is
extremely common.

A peer's routes disappearing without the session dropping. A prefix filter or a max-prefix
limit tripped. `show ip bgp summary` shows the session up and the prefix count at zero.

**Announcing a peer's routes to another peer.** **A route leak.** It attracts traffic you
cannot carry and it is a serious incident. **Egress filters, always.**

An IXP port congested and no obvious cause. Someone's flow grew. The fix is a PNI, and
the lead time is weeks.

> **Network+ note.** Objective 1.6 and 1.8 touch these. Over-learn: **an IXP is a shared
> fabric for peering**; **transit is bought, peering is usually settlement-free**; and
> peering exchanges traffic between the two networks' own customers only. The
> 95th-percentile convention is not examinable and it is the thing you will actually use.
