# Chapter 32 — Important Concepts

**An autonomous system** *(§32.1)* — A collection of IP prefixes under a single,
clearly-defined routing policy. Defined by who decides where traffic goes — not by
ownership, technology or location. The boundary is administrative, not technical,
which is what makes the protocol crossing it different in kind.

**AS numbers** *(§32.1)* — 16-bit originally, **32-bit now** (RFC 6793); private
**64512–65534**. The 16-bit space exhausted like IPv4's — and the transition succeeded in
a few years because the population was small, technical and motivated, and because
**AS 23456 (`AS_TRANS`)** let old speakers carry 32-bit paths they did not understand.
The conditions IPv6 lacked.

When you need BGP *(§32.1)* — **When you multihome.** More precisely: when there is a
decision to make that only you should make. With one provider, a default route is the
correct answer.

**IGP versus EGP** *(§32.1)* — Interior: **shortest path**, technical metrics, full trust,
hundreds of routers, seconds to converge. Exterior: **the policy-preferred path**,
commercial criteria, **no trust**, 75,000 ASes and ~950,000 prefixes, minutes to converge.
There is only one EGP, and there has been no competitor since 1994.

Why an IGP cannot do it *(§32.1)* — (1) Shortest is the wrong objective — no metric
expresses "cheaper" or "not through a competitor". (2) Nobody will run your protocol —
accepting OSPF from another organisation means accepting their view of the world.
(3) The scale is different in kind — nothing can hold the Internet's topology.
(4) Policy must be expressible and hidden, and interior protocols advertise everything
to everyone.

**BGP's basics** *(§32.1)* — **TCP port 179**; **incremental updates only**; the **full AS
path**; selection by **policy sequence**; eBGP AD 20, iBGP 200; neighbours
configured by hand — no discovery, which suits a contractual relationship and is the one
place BGP's trust model is sound.

Running over TCP has a corollary *(§32.1, §32.2)* — Routes are held until explicitly
withdrawn, so a session may be silent for hours. But if the session drops, every route
learned over it is withdrawn at once, which makes a flapping BGP session far more
disruptive than a flapping OSPF adjacency.

Tier 1 is a commercial definition *(§32.1)* — A network that buys transit from
nobody, reaching everything through settlement-free peering. About a dozen exist.

Path vector solves the loop problem outright *(§32.2)* — If a router sees its own AS
in the AS_PATH, it discards the route. One rule replacing split horizon, poison reverse,
holddown and counting to infinity. The provenance distance vector lacked, carried
explicitly.

AS path length is a proxy for nothing physical *(§32.2)* — One AS may be a single
router; another a transcontinental network with forty internal hops.

**LOCAL_PREF** *(§32.2)* — **Higher wins**, default 100, not sent outside your AS, and
**compared first** — so it overrides AS path entirely. The primary tool for controlling
outbound traffic, and the mechanism by which commercial preference beats topology.

**MED** *(§32.2)* — Lower wins; a **hint** to a neighbouring AS about which of your
entry points to use. Compared late, usually ignored, and only between routes from the
same neighbour AS. The weakest tool and the one people reach for first.

**COMMUNITY** *(§32.2)* — A 32-bit tag with no meaning of its own — its meaning is
whatever the receiving network publishes. How customers signal policy to providers
without a phone call, including RFC 7999's `65535:666` BLACKHOLE, which triggers
RTBH at the provider's edge in seconds.

**The selection algorithm** *(§32.2)* — Weight, **LOCAL_PREF**, locally originated,
**shortest AS_PATH**, ORIGIN, MED, **eBGP over iBGP**, lowest IGP metric to next hop,
oldest, router ID. Read what is not in the list: no bandwidth, no latency, no loss, no
congestion. BGP has no idea whether a path is fast.

**Hot potato routing** *(§32.2)* — Step 8 prefers the **nearest exit**, so traffic leaves
your network as soon as possible. Rational, and it produces asymmetry — each direction
chooses its own sender's nearest exit, so the two paths differ routinely.

iBGP does not readvertise *(§32.2)* — A route from an iBGP peer is never sent to
another iBGP peer, because the AS_PATH does not change within an AS so the loop check
cannot work internally. Consequence: a full mesh of n(n−1)/2 sessions — 190 for twenty
routers.

**Route reflectors** *(§32.2)* — Reduce it to roughly *n*. Exactly OSPF's designated
router, applied to iBGP. Peer on **loopbacks** so a session survives any single link
failure.

The four load-bearing configuration lines *(§32.2)* — **outbound prefix-list** (the
single most important line in BGP), **inbound prefix-list**, **`maximum-prefix`**, and
**`next-hop-self`** — without which iBGP routes are present and unusable.

**Transit versus peering** *(§32.3)* — **Transit**: one pays, and receives **everything**.
**Peering**: neither pays, and each receives only the other's own and its customers'
prefixes. That restriction is the whole arrangement.

**The valley-free rule** *(§32.3)* — Routes from a **customer** go to **everyone**; routes
from a **peer or provider** go to **customers only**. The reason is economic: a customer's
routes earn money; advertising a peer's routes to your provider means paying to carry
traffic for a network that pays you nothing.

A valid path goes up, across at most once, then down *(§32.3)* — A valley would mean
providing free transit, which nobody does deliberately. Almost every large BGP incident
is a valley.

**IXPs** *(§32.3)* — A shared fabric replacing *n(n−1)/2* cross-connects with one port
each. Usually the cheapest bandwidth an ISP can buy. And a shared Layer 2 segment, so
Unit IV's concerns apply and exchanges enforce strict rules.

**The flattening** *(§32.3)* — Content networks peer directly with access networks and
place caches inside them, so a large fraction of traffic never touches a Tier 1. The
hierarchy picture is increasingly wrong.

How money shows in routing *(§32.3)* — A longer path chosen because it is cheaper; a
nearby destination reached the long way because no peering exists; asymmetric paths from
hot potato on both sides; two offices in one city communicating across a continent
because their networks do not peer.

Outbound is easy, inbound is hard *(§32.3)* — You control your own routers completely
and other networks' not at all. Structural, not a deficiency in the tools: the path
decision is made by the sender, and you are not the sender.

BGP has no way to tell whether an advertisement is true *(§32.4)* — The whole of the
section in one sentence.

**Leak versus hijack** *(§32.4)* — A **leak** announces real routes that should not have
been passed on — a policy failure, usually accidental, causing congestion and loss. A
**hijack** announces prefixes you do not hold — an ownership failure, causing
interception or a black hole.

A leak requires two failures *(§32.4)* — Someone must announce it and someone must
accept it. The second is the preventable one, and it is the one that keeps not being
prevented.

**AS 7007, 1997** *(§32.4)* — A misconfigured router re-advertised much of the global table
as its own /24s. The first demonstration that one misconfigured router could break the
Internet.

**Pakistan Telecom, 2008** *(§32.4)* — A domestic censorship order implemented as a
**more-specific /24** escaped into BGP, was not filtered outbound or inbound, and took
YouTube offline globally for two hours. Three separate defences would each have prevented
it, and none was present.

MyEtherWallet 2018 and KLAYswap 2022 *(§32.4)* — **Deliberate, targeted, profitable**
BGP attacks. They changed the conversation from accidents to attacks and made the case for
defences commercial.

Why it was built this way, and why it persists *(§32.4)* — In 1989 the participants
were known to each other and no ownership registry or cryptographic infrastructure
existed. It persists because there is no benefit to being early — a validator gains
nothing until others sign, and a signer gains nothing until others validate.

**Filtering** *(§32.4)* — Free, entirely within your control, and incomplete deployment
of it is why every incident happened. `maximum-prefix` is the blunt backstop: losing
one session beats propagating a leak.

**RPKI** *(§32.4)* — A signed **ROA** attests that a given AS may originate a given prefix
up to a given length; routers performing **Route Origin Validation** drop **Invalid**.
This stops the Pakistan Telecom case outright. Roughly half the table is now covered
and major networks drop Invalids — the tipping point came after 2019, helped by **MANRS**.

What RPKI does not do *(§32.4)* — It validates the origin, not the path, and it
does not prevent leaks at all, since a leaked route has a valid origin. ASPA would
attest provider relationships and catch valleys; **BGPsec** signs the path and is deployed
essentially nowhere.

Monitor your own prefixes *(§32.4)* — BGPalerter, BGPStream, RIPE RIS, RouteViews.
You cannot prevent someone announcing your prefix; you can find out in five minutes
instead of two hours, and that difference is most of the damage.

**The uncomfortable summary** *(§32.4)* — The Internet's routing works because most
participants are careful and honest most of the time. Simultaneously its greatest
weakness and the reason it scaled: a protocol requiring central authorisation would have
needed an authority every government and company trusted, which was never going to
exist.
