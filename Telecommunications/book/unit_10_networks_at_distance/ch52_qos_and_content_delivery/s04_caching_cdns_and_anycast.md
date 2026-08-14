# 52.4 Caching, CDNs and Anycast

Everything in §52.1 to §52.3 rearranges a queue. This section deletes the journey.

## The argument, quantified

Chapter 3 §3.1 separated delay into four components. QoS acts on exactly one of them.

| Component | What QoS can do | What a CDN can do |
|---|---|---|
| **Propagation** | **nothing** | **eliminate most of it** |
| **Serialisation** | nothing | **reduce it** (higher-rate local link) |
| **Queueing** | **this is the one** | reduce it (less congestion) |
| Processing | nothing | little |

**Work the numbers.**

| | |
|---|---|
| **A perfect QoS policy on a congested link** | saves perhaps **20–50 ms** of queueing |
| **Moving content 8,000 km closer** | saves **78 ms** of propagation, round trip |
| **London → Sydney, direct** | **166 ms round trip** |
| **London → a cache in London** | **under 1 ms** |

> A CDN's effect is larger than any QoS policy's, and it acts on the delay component that
> Chapter 3 §3.2 identified as otherwise irreducible. You cannot make light faster. **You can
> stop sending it so far.**

**And the effect compounds with protocol behaviour.** A TLS handshake plus a TCP handshake is
three or four round trips before any data flows (Chapter 37 §37.1, Chapter 41 §41.2). At
166 ms that is over half a second before the first byte; at 5 ms it is 20 ms. The saving is
multiplied by every round trip the protocol needs.

## Caching, before CDNs

The mechanism is older than the web and the principles are unchanged.

| Where | Example |
|---|---|
| **Browser** | local disk cache |
| **Forward proxy** | an organisation's shared cache — **largely obsolete now** |
| **Reverse proxy** | in front of an origin server, absorbing repeated requests |
| **CDN edge** | **thousands of locations, operated as a service** |

HTTP's cache control is the mechanism (Chapter 41 §41.1), and the three headers that matter:

| Header | Meaning |
|---|---|
| **`Cache-Control: max-age=3600`** | **fresh for an hour; use it without asking** |
| **`ETag` / `If-None-Match`** | **"has it changed?" → `304 Not Modified`, no body** |
| `Vary` | **this response depends on these request headers** — and it is where caching breaks |

> `Vary: User-Agent` on a resource that does not actually vary by user agent will fragment
> the cache into hundreds of copies and destroy the hit rate. It is the commonest
> cache-defeating mistake, and it is usually accidental.

Forward proxies are worth a note because they died. In 2005 an organisation's web proxy
cached a substantial fraction of its traffic and saved real bandwidth. With HTTPS
everywhere, a proxy cannot see or cache content without terminating TLS, which most
organisations rightly will not do for general browsing. The caching moved from the
organisation to the CDN, and that relocation is a small part of the concentration story
below.

## What a CDN actually is

```
                        ┌──────────────┐
                        │    Origin    │   your server; one place
                        └───────┬──────┘
                    ┌───────────┼───────────┐
              ┌─────┴────┐ ┌────┴─────┐ ┌───┴──────┐
              │  Shield  │ │  Shield  │ │  Shield  │  regional tier
              └─────┬────┘ └────┬─────┘ └───┬──────┘
         ┌──────────┼───────┐   │       ┌───┴────┬─────────┐
      ┌──┴──┐   ┌──┴──┐  ┌──┴──┐ …   ┌──┴──┐ ┌──┴──┐  ┌───┴──┐
      │ PoP │   │ PoP │  │ PoP │     │ PoP │ │ PoP │  │ PoP  │  edge: hundreds
      └──┬──┘   └──┬──┘  └──┬──┘     └──┬──┘ └──┬──┘  └───┬──┘  to thousands
         │         │        │           │       │         │
       users     users    users       users   users     users
```

Three things happen at the edge, and only the first is caching:

**Serving cached content.** Images, video segments, scripts, stylesheets — and increasingly,
whole pages assembled at the edge.

**Terminating TLS and the transport.** The handshake completes against a server 5 ms away
rather than 80, and the edge then reuses a warm, long-lived connection to the origin. This
benefits even completely uncacheable content — a database query still travels the long path,
but the connection setup does not.

**Optimising the path to origin.** The CDN's own backbone frequently beats the public
Internet between its edge and its shield, so even a cache miss is faster than going direct.

> **The commonest misunderstanding about CDNs is that they only help static content.** The
> transport termination alone is worth deploying one for, and it is why API endpoints and
> dynamic applications sit behind CDNs.

The shield tier exists to protect the origin. Without it, a thousand edge locations each
miss on the same object and send a thousand requests to origin — a self-inflicted denial of
service on a popular new object. The regional tier collapses those into one.

## Anycast

How a user reaches the nearest copy without anyone configuring anything.

One IP address, announced by BGP from many locations (Chapter 27 §27.3, Chapter 32 §32.1).
Each router in the Internet sends packets for that address to whichever announcement its own
routing policy prefers — which is, approximately, the nearest one.

```
   1.1.1.1 announced from London, Frankfurt, Singapore, São Paulo, …

   User in Manchester ──▶ BGP prefers the London announcement ──▶ London PoP
   User in Jakarta    ──▶ BGP prefers Singapore              ──▶ Singapore PoP
```

| Property | |
|---|---|
| **Selection** | **automatic, by routing** — no DNS trickery, no client logic |
| **Failover** | **withdraw the announcement and traffic moves in seconds** |
| **DDoS absorption** | **an attack is spread across every location that announces it** |
| **Latency** | **near-optimal, most of the time** |

"Approximately the nearest" is doing real work in that sentence. BGP chooses by AS-path
length and local policy, not by latency or geography — so a user in Lisbon may be sent to
London rather than Madrid because their ISP's routing prefers it. Anycast is very good and
it is not optimal, and large operators supplement it with measurement-driven adjustments.

The classic anycast deployment is the DNS root (Chapter 39 §39.1): 13 root server
addresses, over 1,900 physical instances. The address `198.41.0.4` exists in hundreds of
places at once, and this is why the root has never been taken down despite being a very
attractive target.

**And the limitation:** anycast suits short, stateless exchanges. A long TCP connection
can break if routing changes mid-session and packets begin arriving at a different instance
with no knowledge of the connection. In practice this is rare enough that anycast TCP works
fine — Cloudflare, Google and others use it at scale — but it requires the routing to be
stable, and it is why DNS over UDP was anycast's first and most natural application.

## The two DNS-based alternatives

Worth distinguishing, because they are frequently confused with anycast.

**GeoDNS.** The authoritative server returns a different answer depending on the resolver's
location. Simple, and its accuracy depends on the resolver being near the user — which
broke when public resolvers became popular, since a user in Manchester using `8.8.8.8` may
present as being anywhere. **EDNS Client Subnet** partially fixes it by passing a truncated
client address, at a real privacy cost.

**HTTP redirect.** The user connects to a central location and is redirected to a nearby one.
Adds a round trip, and is precise.

| | **Anycast** | **GeoDNS** | **Redirect** |
|---|---|---|---|
| Extra round trips | **none** | **none** | **one** |
| Accuracy | **good** | **fair; depends on resolver** | **exact** |
| Failover speed | **seconds (BGP)** | **TTL-bound — minutes** | **immediate** |
| Complexity | **BGP everywhere** | low | low |

## The concentration problem

§48.1 raised it; this is where it is worth confronting.

The CDN model has been extraordinarily good for performance and it has concentrated a very
large share of the world's traffic in a handful of organisations.

| Incident | Effect |
|---|---|
| **Fastly, June 2021** | **a single customer's configuration change** took out a large fraction of major news, government and commerce sites **worldwide, within a minute** |
| **Akamai, July 2021** | a DNS configuration error, comparable effect |
| **Cloudflare, repeatedly** | 2019 (a regular expression), 2020, 2022 (a network change) |

None of these was an attack. Each was a configuration change.

> The Internet was designed so that no single failure could disconnect it, and that property
> still holds at the routing layer. It does not hold at the layer where users actually
> experience the network, because a large fraction of what they use depends on one of about
> five organisations.

**And the honest assessment has two halves.**

**The concentration is a genuine risk.** A handful of change-control processes now sit in the
critical path of a large fraction of global commerce, and the blast radius of a mistake is
measured in continents.

**The alternative is worse.** A world in which every site runs its own origin with no CDN is a
world with worse performance, worse availability and far worse DDoS resilience. The
concentration exists because the service is genuinely valuable, not because of a market
failure.

**What an engineer can actually do:** **multi-CDN**, with DNS or measurement-driven failover
between two providers. It is not free — it doubles the integration work, complicates cache
invalidation, and is worth it only above a certain scale — and it is the only real answer
available.

## What breaks here

**A CDN serving stale content.** Cache TTL, or an invalidation that did not reach every
location. Purges are eventually consistent, and "eventually" can be minutes.

Low cache hit rate on obviously cacheable content. Check `Vary`, `Set-Cookie` and
`Cache-Control` on the origin's responses. A `Set-Cookie` on a static asset makes it
uncacheable in most configurations, and it is usually accidental.

**One region slow and others fine.** A PoP problem, or anycast routing sending that region
somewhere distant. Test from that region — RIPE Atlas (Chapter 48's further reading) is
the tool.

**Anycast sending users to the wrong continent.** **BGP path selection**, not geography. The
user's ISP prefers a path you did not anticipate, and the fix is peering rather than
configuration.

**A long download failing intermittently over anycast.** **Routing changed mid-connection.**
Rare, real, and the reason some operators use anycast only for the initial exchange and unicast
for the transfer.

**A CDN outage taking your site down.** Have a documented, tested plan to fail to origin
directly — DNS pointing at the origin, with the origin able to survive the load. Most
organisations discover during the incident that origin cannot.

GeoDNS sending a user to the wrong place. They are using a public resolver. Expected;
anycast does not have this failure mode, which is one of its main advantages.

> **Network+ note.** Objective 1.8 and 3.3. Over-learn: **a CDN caches content close to
> users**; anycast advertises one address from many locations and routing selects the
> nearest; caching reduces load on the origin and latency for the user; and **load
> balancing distributes requests across servers.** The anycast concept is examined and is
> genuinely worth understanding rather than memorising.
