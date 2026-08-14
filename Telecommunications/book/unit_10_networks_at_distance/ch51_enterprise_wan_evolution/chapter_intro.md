# Chapter 51 — Enterprise WAN Evolution

In 1995 a branch office needed to reach exactly one place: the corporate data centre.
Email lived there, the file server lived there, the application lived there, and the
handful of employees who used the Internet did so through a proxy in the same
building. A leased line or a Frame Relay circuit to head office served every need
the office had, and the WAN's job description was one sentence long.

In 2026 the same branch office needs to reach Microsoft 365, Salesforce, a
warehousing application in AWS, a video conferencing service, a security gateway
operated by a third party, and — occasionally, for one legacy system nobody has
replaced — the corporate data centre.

The traffic that used to define the WAN is now a small minority of it. Everything
in this chapter follows from that change, and the technologies form a coherent story
of an industry adapting to it rather than a list of products.

## The old model and why it was rational

**Leased lines** gave a dedicated circuit between two sites: guaranteed bandwidth,
constant latency, no sharing. They were also priced per circuit per distance, so a
full mesh among *n* sites cost *n*(*n*−1)/2 circuits — Chapter 11's arithmetic
turning up on an invoice.

**Frame Relay** (and ATM) reduced that with virtual circuits over shared carrier
infrastructure: one physical connection per site, many logical circuits over it,
priced far below dedicated lines. It was Chapter 13 §13.2's virtual-circuit model,
sold commercially.

**MPLS L3VPN** (Chapter 50 §50.4) replaced both from the early 2000s and became the
default enterprise WAN for roughly fifteen years. One connection per site to the
carrier; the carrier's routing handles any-to-any connectivity; overlapping private
address space stays separate; and the service comes with a contractual SLA covering
availability, latency and packet loss.

That SLA is the reason MPLS commanded — and still commands — a substantial price
premium over commodity Internet access, sometimes ten times per megabit. The
enterprise was not buying bandwidth. It was buying a **promise**, and Chapter 13's
account of what packet switching gave up explains exactly why the promise was worth
paying for.

## What broke it

Three things at once.

**The traffic went elsewhere.** Backhauling cloud-destined traffic across an MPLS
circuit to a central data centre, out through a corporate Internet gateway, and back
again — the **tromboning** or hairpin pattern — adds latency to precisely the traffic
that is most sensitive to it, and consumes the expensive circuit carrying traffic that
was never going to the data centre.

**The price gap became indefensible.** A branch could buy 1 Gb/s of business
broadband for a fraction of the cost of 20 Mb/s of MPLS. Even accounting honestly for
the missing SLA, the arithmetic became difficult to defend to a finance director.

**Provisioning was slow.** An MPLS circuit takes weeks to months to install. A
broadband connection or an LTE modem takes days. For an organisation opening retail
sites, that difference is strategic rather than merely inconvenient.

## SD-WAN

The response, from around 2015, and §51.2 treats it as an application of principles
already established rather than as a product category.

An SD-WAN builds an **encrypted overlay** across whatever underlay connections a site
has — broadband, MPLS, LTE, satellite — and makes forwarding decisions per
application, in software, driven by centrally-defined policy and by continuous
measurement of each path's latency, loss and jitter.

The pieces are all familiar. It is an overlay (Chapter 67's idea, applied to the
WAN). It separates a control plane from a data plane (Chapter 68's idea). It makes
policy-based rather than destination-based forwarding decisions. And it uses IPsec
tunnels (Chapter 61) for the encryption that makes an untrusted underlay acceptable.

What it genuinely delivers: cloud traffic breaks out locally instead of tromboning;
several inexpensive links are used simultaneously rather than one expensive one with
an idle backup; voice can be steered onto the path currently measuring lowest jitter
while a backup transfer takes the other; and a new site is configured centrally rather
than by sending an engineer.

What it does not deliver, and §51.2 is explicit about this because vendors are not:
an SLA on the public Internet. Measuring paths and choosing the best one is not
the same as a carrier committing to a latency figure. When every available path is
congested, SD-WAN chooses the least bad one. For most traffic that is entirely
sufficient; for a small class of applications it is not, and the honest design keeps
an MPLS or dedicated circuit for those.

## Reaching the cloud directly

§51.3 covers the third leg. Cloud providers sell **direct interconnect** — AWS Direct
Connect, Azure ExpressRoute, Google Cloud Interconnect — a private circuit from your
network into theirs, bypassing the public Internet entirely.

The motivations are predictable: consistent latency, higher and more predictable
throughput, egress charges that are substantially lower than over the Internet (which
is frequently the deciding factor, and is worth modelling before designing), and
compliance requirements that prohibit certain traffic from traversing the public
Internet.

The design questions are where it terminates (your data centre, or a colocation
facility where the provider has a presence), whether you need redundant circuits in
different facilities, and how it interacts with your SD-WAN policy so that traffic
takes the private path when it should.

## Designing for the branch and the person

§51.4 addresses the change that 2020 accelerated and did not cause: a large fraction
of users are no longer in any office at all.

The branch office design questions — how much bandwidth, what redundancy, local
breakout or not, what happens when the link fails — now have a companion set for the
individual remote worker, whose "branch office" is a domestic broadband connection
shared with a household and whose "network" is a laptop.

This is where Chapter 59's zero trust becomes an architecture rather than a slogan:
if the user is not on your network, and the application is not in your data centre,
then a perimeter is not merely inadequate but irrelevant, and security must attach to
identity and device rather than to location. §51.4 draws the connection, and Chapter
59 develops it.

## By the end you will be able to

- Explain what an enterprise bought when it bought MPLS, and why it cost what it did.
- Explain tromboning and compute the latency penalty for a stated topology.
- Explain what SD-WAN is in terms of overlay, control-plane separation and
  policy-based forwarding.
- State honestly what SD-WAN does and does not replace.
- Decide when direct cloud interconnect is justified, including on egress cost.
- Design a branch connectivity plan for a stated site, with a defended position on
  local breakout.
