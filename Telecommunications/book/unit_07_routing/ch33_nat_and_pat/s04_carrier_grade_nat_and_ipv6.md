# 33.4 Carrier-Grade NAT and the IPv6 Endgame

NAT was meant to be temporary. It was not, and when the addresses ran out anyway,
providers did the only thing available: they added a second layer of it.

## Carrier-grade NAT

The provider translates too, so that many *customers* share one public address.

```
   Customer LAN          Home router         ISP's CGNAT          Internet
   192.168.1.50  ──────▶  100.64.5.12  ──────▶  203.0.113.7  ──────▶
    (RFC 1918)         (RFC 6598 shared)      (one public address
                                               for hundreds of
   ── NAT #1 ──────────── NAT #2 ──────────▶    subscribers)
```

Two layers of translation, and the customer's public-facing address is not public at
all.

The middle range is `100.64.0.0/10` (Chapter 27 §27.1), reserved by RFC 6598 for
exactly this. It exists because the provider cannot use RFC 1918 space — the customer is
already using it, and a collision would be unresolvable.

**Recognising it is a one-glance diagnosis:** a WAN interface holding a `100.64.x.x`
address means you have no public address, and everything below follows.

### What CGNAT costs

**Inbound is impossible.** Not difficult — impossible. There is no public address to
forward from, and the customer cannot configure the provider's NAT.

- No port forwarding
- No self-hosted services
- No inbound VPN
- Games and applications that need inbound connections fail, and their error messages
  blame the user's router

**Peer-to-peer degrades badly.** CGNAT is usually **symmetric** (§33.3), so hole punching
fails and traffic must be relayed. This is why video calls and multiplayer games work
worse on mobile networks, and the cause is invisible to the user.

**Shared reputation.** Hundreds of subscribers share one address, so one abuser gets the
address blocked for everyone. A CAPTCHA on every search, a forum ban, a service refusing
connections — all consequences of a neighbour's behaviour, with no recourse.

**Logging becomes a regulatory problem.** Identifying which subscriber used an address at
a given moment requires logging the **port range** as well, at enormous volume. Several
jurisdictions now mandate it, and the storage cost is substantial. Some providers allocate
**deterministic port blocks** per subscriber specifically to make the logs tractable.

**And it is expensive for the provider.** CGNAT is stateful, high-capacity, and must be
redundant — a failure drops every session for every subscriber behind it. The equipment
costs real money, which is the argument that finally moved IPv6.

## Why providers do it anyway

Because the alternative is not having addresses.

A provider adding a million subscribers needs a million addresses. At $40 each on the
transfer market (Chapter 28 §28.1), that is $40 million — for a resource that will only
get scarcer.

CGNAT costs a few hundred thousand in equipment. The arithmetic is not close.

But IPv6 costs less still, and this is the decisive point: a provider deploying IPv6
with **464XLAT** (Chapter 28 §28.4) runs an IPv6-only core, needs CGNAT only for
IPv4-destined traffic, and that traffic share falls every year.

> CGNAT is what you deploy when you have not deployed IPv6. It is a cost that grows;
> IPv6 is a cost that ends.

Which is why mobile carriers went IPv6-first, and why they are the most IPv6-advanced
part of the industry (Chapter 28 §28.1). They were the first to face the arithmetic at
scale.

## The endgame

**IPv6 removes the reason NAT exists.** With 2¹²⁸ addresses, address conservation is not
a design constraint (Chapter 28 §28.1).

So IPv6 has no NAT — deliberately, and Chapter 28 §28.4 gives the reasoning.

**And the objection is always the same:** *"but NAT protects us."*

**It does not** (§33.1), and the IPv6 answer is the correct one:

| Concern | IPv4 answer | IPv6 answer |
|---|---|---|
| Inbound connections blocked | NAT, incidentally | **a stateful firewall, deliberately** |
| Internal topology hidden | NAT, incidentally | **privacy addresses** (Chapter 28 §28.3) |
| Address conservation | NAT, deliberately | **not a problem** |

Every consumer IPv6 router ships with a default deny-inbound firewall. The protection
is identical, it is a policy decision rather than a side effect, and it can be relaxed
per host for the applications that need it — which NAT cannot do without port forwarding.

**What you get back:**

- **Every device globally addressable.** Peer-to-peer works.
- No ALGs, no STUN, no TURN, no hole punching. The workarounds of §33.3 become
  unnecessary.
- **No translation state**, so no single point of failure and no table to exhaust.
- **Logs identify hosts**, so abuse handling works.
- The end-to-end principle restored (Chapter 23 §23.4).

**NPTv6** (RFC 6296) exists for the one legitimate remaining case — multihoming without
provider-independent addresses — and it is stateless one-to-one prefix translation,
which preserves the end-to-end model in a way NAT does not. It is rarely needed and rarely
used.

## Where this leaves us

The honest current position, without advocacy in either direction:

**NAT will not disappear soon.** IPv4 will be reachable for decades, every one of those
connections will traverse NAT, and enterprise networks in particular have deep
institutional attachment to it.

CGNAT will get worse before it gets better, as providers add subscribers against a
fixed address supply.

IPv6 removes the need, and adoption is at roughly half of traffic and rising —
driven by cost rather than by architecture, which is Chapter 28 §28.1's point and the
only argument that has ever moved this.

The trajectory is clear and the timescale is long. A student today will spend a career
working with both, and the practical skill is not choosing a side but knowing which
problems are NAT's and which are not — because a large share of "weird network problems"
in the next twenty years will be NAT, and recognising them quickly is worth more than
having an opinion about it.

## The thirty-year summary

**1994:** a nine-page RFC proposes a short-term workaround, catalogues its architectural
damage honestly, and recommends it anyway.

**It works.** Exhaustion is deferred fifteen years. The Internet grows a thousandfold.
Billions of devices connect that could not otherwise have.

**And the costs arrive slowly and permanently:** the end-to-end principle broken,
peer-to-peer effectively dead, an entire protocol family invented to work around it, a
generation of applications built to relay through servers, and — most consequentially —
the urgency for IPv6 removed by the very success of the workaround.

> NAT is the most successful temporary measure in the history of computing, and its
> success is precisely what made it permanent.

Whether that is a triumph of pragmatic engineering or a cautionary tale about workarounds
is a question worth forming your own view on, and the honest answer is that it is both.

## What breaks here

No inbound connectivity at all, and port forwarding does nothing. CGNAT. Check for
`100.64.x.x` on the WAN interface.

A game or application failing on mobile and working on home broadband. Symmetric
CGNAT defeating hole punching.

CAPTCHAs everywhere, or a service blocking you for no reason. Shared address
reputation.

An abuse report you cannot trace to a subscriber. Port-level logging was not enabled.

**Deploying NAT66 out of habit.** Do not. The firewall provides the security; the addresses
are free.

An IPv6 deployment where inbound is unexpectedly open. The firewall was not configured
because the team assumed NAT was doing it. This is the most common IPv6 security
mistake and it is exactly the cost of side-effect security.

> **Network+ note.** Objective 1.7 expects `100.64.0.0/10` as CGNAT space; objective 2.2
> expects NAT. Over-learn: CGNAT means the customer has no public address, so inbound is
> impossible; **`100.64.x.x` on a WAN interface is the signature**; and IPv6 does not
> use NAT — a stateful firewall provides the protection deliberately rather than as a side
> effect.
