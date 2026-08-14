# 32.3 Peering, Transit and Money

BGP's selection algorithm (§32.2) is a mechanism for expressing preferences. This section
is about where the preferences come from, and the answer is money.

**The Internet's topology is a commercial artefact.** Understanding the money explains
routing decisions that make no technical sense, and it explains why the network looks the
way it does.

## The two relationships

Almost every BGP session is one of two kinds.

### Transit — one pays the other

```
   Customer AS ──── $$$ ────▶ Provider AS
```

**The provider gives the customer access to the entire Internet.** The customer pays,
usually per megabit of the 95th percentile of usage.

**What the provider advertises to the customer:** everything — the full table, or a
default route.

**What the customer advertises to the provider:** only its own prefixes and those of its
own customers.

### Peering — neither pays

```
   AS A ◀──── free ────▶ AS B
```

**Two networks exchange traffic between their own customers**, and typically neither pays
the other — *settlement-free peering*.

**What each advertises:** **only its own prefixes and its customers'.** Not the rest of
the Internet.

**That restriction is the whole of the arrangement**, and violating it is the route leak
of §32.4.

**Why peer at all?** Both save money. Traffic that would otherwise be carried by a paid
transit provider goes directly, at the cost of a cross-connect. If the volumes are roughly
balanced, both benefit and neither has leverage to charge.

**When they are not balanced, the larger network usually refuses** — which is why
"depeering" disputes happen and why a content network sending far more than it receives
often ends up paying, despite the traffic being requested by the other network's own
customers.

## The valley-free rule

**The most important principle in inter-domain routing**, and it follows entirely
from the money.

Classify every neighbour as **customer**, **peer**, or **provider**. Then:

| Route learned from | Advertise to |
|---|---|
| **Customer** | **everyone** — customers, peers, providers |
| **Peer** | **customers only** |
| **Provider** | **customers only** |

**The reason is economic, not technical:**

- **A customer's routes earn you money.** Advertise them everywhere; the more traffic
  arrives for your customer, the more your customer pays.
- **A peer's routes earn you nothing.** Advertising them to your provider means paying
  your provider to carry traffic for a network that pays you nothing. **You would be
  buying transit on someone else's behalf.**
- **A provider's routes cost you money.** Advertising them to a peer means becoming a
  free transit provider for your peer.

**The resulting path shape** — hence the name:

```
        provider                provider
             ╲                  ╱
              ╲                ╱
             peer ────────── peer          ← the "peak"
              ╱                ╲
             ╱                  ╲
       customer                customer

   A valid path goes UP, ACROSS at most once, then DOWN.
   It never goes down and up again — never into a "valley".
```

**A valley would mean an AS providing free transit**, which nobody does deliberately.

> **Almost every large BGP incident in history is a valley** — an AS advertising routes it
> learned from one provider or peer to another, becoming an unwilling and undersized
> transit provider for traffic it never wanted.

§32.4 is the catalogue.

## Internet exchange points

**A shared fabric where many networks peer at once.**

Rather than *n(n−1)/2* cross-connects between *n* networks in a city, everyone connects
once to a common switch and peers over it bilaterally.

| IXP | Peak traffic |
|---|---|
| **DE-CIX Frankfurt** | ~15+ Tb/s |
| AMS-IX Amsterdam | ~10+ Tb/s |
| LINX London | several Tb/s |
| Equinix, and hundreds of regional exchanges | |

**The economics are compelling.** A port at an exchange costs a few hundred to a few
thousand a month and may replace far more in transit. **Peering at an exchange is usually
the cheapest bandwidth an ISP can buy**, and it is why exchanges exist in almost every
significant city.

**Route servers** at exchanges let a network peer with everyone present through one
session rather than dozens — a multilateral arrangement that dramatically lowers the effort
of peering with small networks.

**And an exchange is a shared Layer 2 segment**, which means Chapter 20's concerns apply:
a misconfiguration at an exchange affects everyone on the fabric, and exchanges enforce
strict rules — no broadcast, no proxy ARP, one MAC per port, filtering — for exactly the
reasons in Unit IV.

## The flattening

**The hierarchy of §32.1 is decreasingly accurate**, and the reason is content.

**The old shape:** content lived in data centres, reached through a transit hierarchy, and
Tier 1 networks carried a large share of all traffic.

**The current shape:** a handful of content networks — Google, Meta, Netflix, Amazon,
Cloudflare, Akamai — account for the majority of consumer traffic, and they **peer directly
with access networks**, often placing caches **inside** them.

```
   Old:   Content ── Transit ── Transit ── Access ── User

   Now:   Content ────────── peer ────────── Access ── User
          or:   [cache inside the access network] ── User
```

**Netflix's Open Connect and Google's Global Cache** put physical servers in the ISP's own
facilities. The traffic never crosses the Internet at all.

**Consequences worth noting:**

- **A large fraction of traffic never touches a Tier 1.**
- **Tier 1 transit has become a commodity** with falling prices and declining strategic
  importance.
- The **AS path to popular content is short** — often one or two hops — which is why
  §32.2's AS path length is even less informative than it looks.
- **Concentration.** A small number of networks carry most traffic, which has consequences
  for resilience and for policy that are outside this book's scope and inside this book's
  concern.

## How the money shows in the routing

**This is the practical payoff of the section.** When a path makes no technical sense,
these are the reasons:

**Traffic taking a longer path.** LOCAL_PREF, set because the shorter path costs more.
**Cheaper beat shorter**, deliberately.

**Traffic to a nearby destination going a long way round.** No peering relationship
exists, so it goes up to a common provider and back down — the valley-free rule forbidding
the direct-looking path that does not exist commercially.

**Asymmetric paths.** Hot potato on both sides (§32.2), plus each network's independent
policy.

**A route that vanishes at a certain time.** A depeering — a commercial dispute expressed
as a configuration change.

**Traffic between two networks in one city crossing a continent.** They do not peer, and
their nearest common transit point is elsewhere. **This is common and it surprises
everyone the first time.**

## The customer's decisions

For an organisation multihoming to two providers, the practical policy questions:

**Outbound — which provider do we use?**

```
   route-map PREFER-A permit 10
    set local-preference 200
```

LOCAL_PREF, applied to routes from the preferred provider. **Easy, and entirely within
your control.**

**Inbound — which provider do others use to reach us?**

**Hard, and only partially controllable**, because it depends on decisions made by
thousands of other networks:

| Technique | Effect |
|---|---|
| **AS path prepending** | works against networks that decide on AS path — many decide earlier |
| **MED** | only to a single neighbouring AS, and usually ignored |
| **Communities** | if the provider publishes useful ones, the most effective |
| **Selective advertisement** | announce a more specific prefix to one provider only — effective and it adds a route to the global table |
| Not advertising at all | absolute, and it removes redundancy |

> **Outbound traffic engineering is easy; inbound is hard.** You control your own routers'
> decisions completely and other networks' not at all. Every network engineer learns this,
> usually while trying to balance two links.

**The asymmetry is structural**, not a deficiency in the tools: the decision about which
path to use is made by the sender, and you are not the sender.

## What breaks here

**Announcing a provider's routes to a peer.** A valley. You have become a transit provider
for traffic you cannot carry. §32.4.

**Traffic engineering that does not work.** Trying to control inbound with MED, or
prepending against a network that decides on LOCAL_PREF.

**Two links, and 90% of traffic on one.** Inbound is not controllable by wishing. Use
selective advertisement or the provider's communities.

**A path that makes no technical sense.** It probably makes commercial sense. Look for the
relationship rather than the topology.

**Losing connectivity to one network while everything else works.** A depeering, or a
filter change at a peer.

> **Network+ note.** Objectives 1.2 and 2.2 touch on ISP relationships lightly. The
> examinable content is thin; the operational content is not. Worth carrying: **transit is
> paid and gives you everything; peering is free and gives you only the peer's own
> customers**; **the valley-free rule follows from the money**; and **you can control
> outbound traffic and only influence inbound.**
