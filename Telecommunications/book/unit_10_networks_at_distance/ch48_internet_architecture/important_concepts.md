# Chapter 48 — Important Concepts

A tier 1 network is defined negatively *(§48.1)* — It buys transit from nobody,
reaching everything through settlement-free peering and its own customers. There are perhaps
a dozen, and no authority certifies the list.

The pyramid described 1998 *(§48.1)* — It was accurate when content lived on millions of
small servers and reaching anything meant climbing to the backbone. It is not accurate now.

Content moved, and everything followed *(§48.1)* — Video volume made transit-carried
delivery uneconomic; the content networks built their own backbones and do not sell
transit; and caches went physically inside access networks, given free to the ISP because
both parties save. A large share of the bytes you consume travelled a few kilometres.

Transit is not unimportant; its share of bytes is *(§48.1)* — Transit reaches the long tail
of seventy-odd thousand autonomous systems, and an access network without transit is not on
the Internet. The head of the distribution goes direct.

Eyeball and content are the categories that predict behaviour *(§48.1)* — Eyeball
networks receive far more than they send and own the users; content networks send far more
than they receive and own what users want. Every peering dispute in the industry's history
is this asymmetry, and tiers predict nothing by comparison.

Latency no longer follows geography *(§48.1)* — Two sites "on the Internet" can differ
fifty-fold in latency, and the difference is whether the content network has built out to
you, not distance. Chapter 66's performance work depends on knowing this.

Technical decentralisation is intact; operational concentration is not *(§48.1)* — Any
single AS may fail unnoticed. A handful of CDN and DNS providers now sit in the path of a
large fraction of what users do — Fastly 2021, Akamai 2021, Cloudflare repeatedly — and
each took out thousands of unrelated sites in seconds without any attack.

Peering is defined by what it excludes *(§48.2)* — A peer carries traffic to its own
customers and no further. A transit provider carries yours to third parties. The routing
filter is the relationship, and a router with no export policy announces everything to
everyone — which is why route leaks are the commonest serious BGP misconfiguration.

95th-percentile billing: 8,640 samples, top 5% discarded *(§48.2)* — Five-minute samples,
432 discarded, which is 36 hours per month, and the larger of inbound or outbound is
billed. Brief spikes are free; sustained load is what costs — which is why bulk transfers
are scheduled overnight and why operators watch a graph at month end.

The peering decision is a spreadsheet, and then it isn't *(§48.2)* — The worked example
saves $340/month, which is marginal. The decision turns on latency, resilience, and the
knowledge that traffic will grow while the port cost will not. **Peer early** is the
industry's rule of thumb for exactly that reason.

An IXP is a Layer 2 fabric and nothing else *(§48.2)* — It carries no traffic of its own
and sets no policy about who peers. N participants need N connections instead of N(N−1)/2
cables, which is the entire economic argument. Route servers collapse hundreds of
sessions into one, at the cost of selective policy.

Private interconnect is where large flows go *(§48.2)* — Peer at the exchange, watch the
flow grow, move it to a PNI. Capacity, predictability, cost per bit and one-cable
troubleshooting all argue for it once a flow is large.

The peering disputes are about value capture, not engineering *(§48.2)* — The eyeball
network says the ratio is unbalanced; the content network says it is delivering what that
network's own paying customers requested. **Both are true.** **Comcast–Netflix (2013–14)** is
the case: not throttling, but the deliberate non-upgrading of a congested port — harder to
characterise and equally effective. The routing follows the contracts.

IANA holds the numbers, not just the addresses *(§48.3)* — Port numbers, DNS record types,
ICMP types, TLS cipher identifiers — "IANA-assigned" in a specification means "look it up in
a table maintained outside this document." Functions performed by **PTI under ICANN**, since
the 2016 stewardship transition.

The RIRs are membership organisations, not regulators *(§48.3)* — Policy is made by the
people it applies to, in open meetings, by consensus. Nobody voted for this system and it has
held for thirty years.

PA versus PI is the distinction with consequences *(§48.3)* — PA is the ISP's space, lent
to you, aggregated into one announcement — and you renumber when you leave. PI is yours,
portable, and costs a separate entry in every router's table on the Internet. Every PI
prefix is a small permanent tax on every router in the world, which is why policy restricts
it and why the table grew when restrictions loosened.

Exhaustion turned an allocated resource into a traded asset *(§48.3)* — IANA's pool went in
**February 2011**; all RIRs were effectively exhausted by 2020. Prices ran from ~$10 per
address in 2015 to $40–60 by 2021. MIT sold half of 18.0.0.0/8 in 2017. A new entrant
now pays what an incumbent received free, which is Chapter 28's argument for IPv6 in its
sharpest form.

Traded space is fragmented, so the market accelerates table growth *(§48.3)* — A /24 here
and a /22 there cannot be aggregated.

IPv6's scarce resource is routing table entries, not addresses *(§48.3)* — Policy gives
/32 minimum to an LIR, /48 to an end site, /56 or /48 to a household — deliberately sparse,
because aggregation is worth more than density. Giving a home a /56 is not wasteful; the
global unicast range holds $2^{53}$ of them.

WHOIS records who was given space; RPKI records who may announce it *(§48.3)* — During a
hijack those differ, and the RPKI answer is the one that matters. WHOIS is reliable for the
RIR, the registered organisation and the abuse contact; unreliable for geolocation, current
use and legitimacy.

Rough consensus is not a majority *(§48.4)* — It is the chair's judgement that objections
have been heard and addressed. A single well-argued technical objection can block a document;
a hundred people disliking it cannot. The distinction is objections with technical content
versus objections without.

Running code means an implementation is evidence and a design is a claim *(§48.4)* — The
IETF will change a specification to match what implementations do. This is exactly the
difference that decided OSI's fate (Chapter 22 §22.1).

"No membership" has costs as well as benefits *(§48.4)* — Influence accrues to those who
can attend three meetings a year and read a great deal of email, which skews towards large
employers; consensus is slow; and chairs hold real, lightly-reviewed power. Against which,
the process produced IP, TCP, DNS, HTTP, TLS and BGP.

An RFC is immutable and its number tells you nothing *(§48.4)* — Corrections come by
publishing another RFC. Check the status header and the "Obsoleted by" line before citing
anything — five seconds, and it prevents a specific class of embarrassment. RFC 793 was
obsoleted by RFC 9293 in 2022.

"Proposed Standard" is not provisional *(§48.4)* — TLS 1.3, HTTP/2 and QUIC are all
Proposed Standards, carrying most of the web. The formal progression is rarely completed
because it confers little benefit, so the label under-describes maturity almost universally.
Informational is where the traps are — a vendor's proprietary protocol documented with no
review of its merits is still a real RFC with a real number.

Specification accessibility affects who implements *(§48.4)* — IETF standards are free to
read; IEEE standards cost hundreds of dollars each, and 802.11's full text is thousands of
pages behind a paywall. The Internet's core protocols being free is not incidental to their
adoption.

Names are political because a name means something *(§48.4)* — ICANN's multistakeholder
model gives governments advice and not control, which several states have consistently
disliked; the alternative is the ITU's state-voting model, and WCIT 2012 was the flashpoint
that deferred rather than resolved the disagreement. The 2012 gTLD expansion also shows name
policy producing security consequences — `.zip` and `.mov` collide with file extensions.

How to read a specification *(§48.4)* — Status and obsoletion; abstract and introduction;
the packet format or state machine; the Security Considerations section, which is where
authors state what the protocol does not protect against; IANA Considerations; and ignore
the rest. RFCs are reference documents. Nobody reads one end to end.
