# 48.1 The Shape of the Internet

The picture in most textbooks is a pyramid, and it is a picture of 1998.

## The pyramid, and what it described

```
              ┌──────────────┐
              │   Tier 1     │   settlement-free among themselves,
              │  backbones   │   sell transit to everyone
              └──────┬───────┘
              ┌──────┴───────┐
              │   Tier 2     │   regional; buy transit, peer laterally
              └──────┬───────┘
              ┌──────┴───────┐
              │   Tier 3     │   access ISPs; buy transit
              └──────────────┘
                     │
                  end users
```

The defining property of a tier 1 network is negative: it buys transit from nobody.
It reaches the entire Internet through settlement-free peering with the other tier 1s and
through its own customers. There are perhaps a dozen — Lumen, Arelion, Cogent, GTT,
Tata, NTT, Telia, Zayo and a handful of others, and the membership is disputed at the edges
because there is no authority that certifies it.

And the pyramid was a genuinely accurate description of an Internet where content was
scattered across millions of small servers. Reaching a web server in another country meant
climbing to the backbone and descending on the other side, because there was no other path.

## What changed

**Content moved**, and everything followed.

| | **~2000** | **now** |
|---|---|---|
| Where content lives | **millions of small servers** | **a few very large networks** |
| Share of traffic from top 10 sources | small | **the majority** |
| Path to popular content | **via the backbone** | **direct, or from a cache in your ISP** |
| Transit's share of bytes | **most** | **a minority — the long tail** |
| Distance to content | **geographic** | **often a few kilometres** |

Three forces produced this, and they reinforce each other.

**Video.** Streaming is the majority of consumer traffic by volume, and it comes from a small
number of platforms. Carrying it over transit is ruinously expensive at scale, so the
platforms built their own networks.

**The content networks built backbones.** Google, Meta, Netflix, Amazon, Microsoft and Apple
each operate global networks with capacity comparable to a tier 1's, and they do not sell
transit. They exist to deliver their own traffic to users as directly as possible.

**Caches went inside access networks.** Netflix's Open Connect, Google's GGC, Akamai's
clusters — physical servers, given to the ISP at no cost, installed in the ISP's own data
centre. The ISP saves transit; the content network saves backbone and gains performance.
Both parties benefit, which is why deployment was rapid.

> A large share of the bytes you consume travel a few kilometres from a machine inside your
> own ISP. They never crossed a backbone, never crossed an ocean, and never touched a
> transit provider.

## The shape now

```
      ┌────────────────────────────────────────────────────────┐
      │   Content / cloud networks (Google, Meta, Netflix,      │
      │   Amazon, Microsoft, Akamai, Cloudflare …)              │
      └───┬───────────────┬────────────────┬───────────────┬────┘
          │               │                │               │
          │        ┌──────┴──────┐         │      ┌────────┴─────┐
          │        │    IXP      │         │      │  cache inside│
          │        │  fabric     │         │      │   the ISP    │
          │        └──────┬──────┘         │      └────────┬─────┘
          │               │                │               │
      ┌───┴───────────────┴────────────────┴───────────────┴────┐
      │              Access / eyeball networks                   │
      └──────────────────────────┬───────────────────────────────┘
                                 │
                        ┌────────┴────────┐
                        │  Transit        │  ← still essential,
                        │  backbones      │    smaller share of bytes
                        └─────────────────┘
                                 │
                        the long tail of everything else
```

Flat, densely interconnected, and with the backbone off to one side rather than on top.

**Transit has not become unimportant.** It is how you reach the other seventy-four thousand
autonomous systems, and an access network without transit is not on the Internet. What has
changed is the **share of bytes**: transit carries the long tail, and the head goes direct.

## The two categories that actually predict behaviour

Forget tiers. The useful distinction is what a network's traffic does.

| | **Eyeball network** | **Content network** |
|---|---|---|
| Customers | **end users** | **nobody, or content owners** |
| Traffic ratio | **receives ≫ sends** | **sends ≫ receives** |
| Wants | cheap inbound capacity | **as close to eyeballs as possible** |
| Leverage | **owns the users** | **owns what users want** |

And this asymmetry is the source of every peering dispute in the industry's history.
A content network wants to hand traffic to an eyeball network as early as possible; the
eyeball network then carries it across its own backbone to the user and bears that cost.
Whether that exchange should be free is the argument, and §48.2 works through it.

## Why it matters operationally

**Latency does not follow geography.**

```
   $ ping www.example-cdn.com
   64 bytes from 203.0.113.9: time=3.1 ms      ← a cache 4 km away

   $ ping some-small-site.example.org
   64 bytes from 198.51.100.7: time=147 ms     ← across an ocean
```

Two sites that "are on the Internet" can differ by fifty times in latency, and the
difference is not distance but whether the content network has built out to you.
Chapter 66's performance work depends on knowing this: a slow site is frequently not slow
but distant, and the fix is not on your network.

**Traceroute reads differently.** A path that leaves your ISP, enters an exchange, and lands
in a content network in three hops is normal and healthy. A path that climbs to a transit
backbone, crosses a continent and comes back is a sign that a peering session is down.

**Concentration is a resilience property.** When a single CDN misconfigures, a visible
fraction of the web becomes unreachable simultaneously — Cloudflare in July 2019 and again
in 2020 and 2022, Fastly in June 2021, Akamai in July 2021. Each took out thousands of
unrelated sites within seconds, and none of them was an attack.

> **The Internet's technical decentralisation is intact and its operational concentration is
> not.** Any single autonomous system may fail without the network noticing. A handful of
> organisations now sit in the path of a large fraction of what users actually do, and that
> is a different property from the one the architecture guarantees.

And the flattening also concentrates the physical layer. Content networks, IXPs and cache
deployments cluster in a small number of buildings — Equinix Ashburn, Telehouse Docklands,
Interxion Frankfurt — so a fire or a power failure in one facility has effects out of all
proportion to its size. Chapter 50 §50.5 makes the same point about submarine landing
stations.

## What breaks here

A site that is fast for you and slow for a colleague fifty kilometres away. Different
ISPs, different cache deployments. **Nothing is wrong.**

Traffic to a major service suddenly routing via another continent. A peering session or a
local cache is down; the traffic fell back to transit. Check the AS path (Chapter 32).

**A regional outage taking out unrelated sites.** A shared CDN or DNS provider. The
correlation is the diagnosis.

A CDN cache serving stale or wrong content to one region. The cache is local; the
invalidation did not reach it.

**Speed test fast, real traffic slow.** The speed-test server is often inside your ISP.
It measures the access link, not the path to anything you use.

> **Network+ note.** Objective 1.6 touches network types and the Internet. Over-learn:
> an IXP is a shared fabric where networks peer; **transit is purchased and peering is
> usually settlement-free**; and **content delivery networks place content near users.** The
> tier model still appears in exam material — know it as a historical description and know
> why it no longer describes the traffic.
