# Chapter 39 — Important Concepts

**HOSTS.TXT** *(§39.1)* — Until 1983, one text file, one maintainer, distributed by
FTP. It worked for a decade, and its descendant `/etc/hosts` is still consulted before
DNS — which is occasionally useful and occasionally a fault nobody can find.

The four failures, and their answers *(§39.1)* — **Traffic** (load grew with the
product of file size and host count) → **distribute the data**. **Collisions** (a flat
namespace cannot delegate) → **hierarchy**. **Consistency** (everyone had a different copy,
with no way to know) → query on demand plus caching with an explicit expiry. The
administrative bottleneck (one organisation approving every name) → delegation.

The binding constraint was administrative, not technical *(§39.1)* — There is no number
of people who can approve every name for a global network.

**Zone versus domain** *(§39.1)* — A domain is a subtree; a zone is what one server is
authoritative for, stopping where delegation begins. The domain `example.com` includes
`build.eng.example.com`; the zone does not.

Hierarchy solves two problems at once *(§39.1)* — **State** (as in route aggregation and
OSPF areas) **and authority** (delegation). DNS is the clearest example in this book because
it does both visibly.

DNS became a general-purpose distributed database *(§39.1)* — Mail routing, service
discovery, anti-spam, **certificate issuance**, load steering. Because it is the one
lookup service every host already trusts.

**Recursive versus iterative** *(§39.2)* — The stub asks recursively — "get me the
answer". The resolver queries iteratively — "tell me who to ask next". Each referral moves
one level down the tree, which is hop-by-hop forwarding applied to a namespace.

**Glue records** *(§39.2)* — When a zone's nameserver is inside the zone it serves, the
parent must also return its address, or resolution loops. Not needed when the nameserver
is in a different zone. Stale glue at the registrar is a real and confusing fault.

Thirteen root addresses, 1,900+ instances *(§39.2)* — Thirteen because that is what fit
in a 512-byte response. Anycast is why the root survives volumetric attack, and long
TTLs mean a busy resolver queries it a handful of times a day.

Caching is where the cost actually goes *(§39.2)* — Typical hit rates exceed 80%, and
partial hits skip most of the walk. The hierarchy's apparent inefficiency is not one.

The stub's failure modes *(§39.2)* — Nameservers in `resolv.conf` are tried in order,
not load-balanced, with a **5-second default timeout** — so a dead primary makes every
lookup take 5 seconds. And the **`search` list** appends domains to unqualified names, so a
short name can resolve to something unexpected.

**The response flags** *(§39.2)* — `aa` absent means the answer came from a cache, which
is the first thing to check when an answer looks stale. `tc` means truncated — retry over
TCP.

`dig +trace` is the most instructive DNS command *(§39.2)* — It performs the iterative
walk and prints every referral.

**The size problem** *(§39.2)* — 512 bytes originally; **EDNS0** raises it; and large UDP
responses fragment and are dropped, so RFC 9715 now recommends ~1,232 bytes — a
retreat toward the original reasoning after two decades. A firewall permitting only
UDP/53 breaks large responses, zone transfers and DNSSEC.

**Reverse DNS** *(§39.2)* — `in-addr.arpa` with the address reversed, because DNS is
hierarchical from the right and IP addresses from the left — flipping aligns the two so
delegation of an address block matches delegation of a subtree. Controlled by the address
holder, not the domain owner.

A record's five fields *(§39.3)* — NAME, **TTL**, CLASS (always `IN`), TYPE, RDATA.

Round-robin DNS distributes load and does not provide failover *(§39.3)* — **No health
checking**, clients cache, clients choose, and recovery is bounded by the TTL rather than by
detection.

**CNAME's three rules** *(§39.3)* — It cannot coexist with other records at the same
name (a CNAME means the name is *entirely* an alias); therefore it cannot be at a zone
apex, which must carry SOA and NS; and chains cost round trips and can loop. The
standard fix for the apex is now **HTTPS/SVCB records**.

**MX** *(§39.3)* — Lower preference is tried first, which reverses most people's
intuition. An MX target must not be a CNAME. No MX record means mail goes to the A
record, which is a common cause of mail arriving at a web server.

TXT became the Internet's assertion mechanism *(§39.3)* — SPF, DKIM, DMARC, ACME
challenges, domain verification. In every case the logic is the same: only the domain's
owner can create a record under it, so a TXT record proves control. Certificate issuance
depends entirely on this.

SRV returns a port *(§39.3)* — Which an A record cannot. Active Directory's domain
controller location is entirely SRV. HTTP does not use it, which is why the web needed
SVCB.

**The SOA serial** *(§39.3)* — Secondaries compare it, and forgetting to increment it is
the classic zone-file mistake: the primary has the change, the secondaries do not, and
answers differ by which server was asked. Intermittent and baffling, and the fix is one
number.

NS records exist in two places *(§39.3)* — The parent (creating the delegation) and the
child (the authoritative list). When they disagree, resolution is unpredictable.

**The trailing dot** *(§39.3)* — A name without one has `$ORIGIN` appended, so
`www.example.com` in a zone file becomes **`www.example.com.example.com.`** — the single
most common zone-file mistake.

**Zone transfers** *(§39.3)* — AXFR (full), IXFR (incremental), NOTIFY. **Over TCP.** An
open AXFR hands an attacker your complete network map — restrict by address and
authenticate with TSIG.

TTL is a contract *(§39.4)* — The only mechanism controlling propagation speed. Short
TTL costs query volume and buys agility.

Lower the TTL *before* the change, and wait out the *old* TTL *(§39.4)* — The most
useful operational fact in the chapter. Lowering it at the moment of the change achieves
nothing, because caches already hold the old record with the old long TTL.

**Negative caching** *(§39.4)* — "Does not exist" is cached too, using the **SOA minimum**.
So a newly-created record does not work until that expires, everything looks right at
the authority and wrong everywhere else, and the only remedy is waiting. Keep the
minimum modest.

**TTLs are advisory** *(§39.4)* — Resolvers impose minimums and maximums, browsers cache
separately, and Java historically cached **forever**.

DNS has no authentication *(§39.4)* — A response is believed because it arrived and
matched: **16-bit query ID**, the question, and originally a **fixed source port**.

**The Kaminsky attack** *(§39.4)* — Query for names that do not exist, so every attempt
is a fresh cache miss with a fresh ID to guess — **unlimited attempts** — and the forged
response's additional section poisons the whole zone's delegation rather than one name.
Turned a slow one-shot attack into one that succeeds in seconds and takes the entire
domain. The July 2008 coordinated patch release is one of the largest such efforts in
Internet history.

Source port randomisation is a mitigation, not a solution *(§39.4)* — It raises the
guess from 2¹⁶ to about 2³². Kaminsky said so at the time.

**DNSSEC** *(§39.4)* — **RRSIG** (signature), **DNSKEY** (public key), **DS** (a hash of the
child's key published in the parent), **NSEC/NSEC3** (authenticated denial). Each level
vouches for the next; the root's key is the trust anchor, signed in a deliberately
theatrical ceremony in July 2010.

What DNSSEC does and does not do *(§39.4)* — Authenticity, integrity and authenticated
denial. Not confidentiality, and not the last hop — between the validating resolver
and your machine the answer is still plaintext. Which is where most users' actual exposure
is, and why encrypted transport moved faster.

**Why DNSSEC stalled** *(§39.4)* — No benefit to being early (the same obstacle as
IPv6); **operationally dangerous** — an expired signature makes your domain vanish, a
harder failure than not signing; key management is real work; and responses grow into the
fragmentation problem.

**Encrypted DNS** *(§39.4)* — DoT on port 853 is visible as DNS and therefore blockable;
DoH on port 443 is indistinguishable from web traffic — which is the point for a user
evading surveillance and the problem for an enterprise needing visibility. Both interests
are legitimate, and canary domains are the partial resolution.

**Dyn, October 2016** *(§39.4)* — A DDoS against one managed DNS provider took Twitter,
Spotify, Reddit, GitHub and Netflix offline **simultaneously**, with none of their own
infrastructure failing. Concentration risk: a shared dependency is a single point of
failure for all of them, invisible until it fails. Use two providers.

**Facebook, October 2021** *(§39.4)* — BGP routes to their own DNS servers were withdrawn,
so healthy servers became unreachable and every property became unresolvable for six hours.
And the failure compounded — internal tools depended on the same DNS, badge readers on
internal tools, and engineers could not enter the building. A dependency you did not
know you had is still a dependency.

**The diagnostic sequence** *(§39.4)* — Does it resolve; is it right at the authoritative
server; if yes but wrong at the resolver, it is caching; is the delegation correct
(`+trace`); is it my resolver specifically (`@8.8.8.8`). Steps 2 and 5 together
localise almost everything.
