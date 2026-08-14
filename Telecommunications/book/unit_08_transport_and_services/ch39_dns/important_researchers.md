# Chapter 39 — The People

**Elizabeth "Jake" Feinler (b. 1931).** Ran the Network Information Center at SRI from 1972
to 1989, and maintained HOSTS.TXT by hand.

She was, quite literally, the Internet's directory service — people telephoned her group
to register a hostname, and her team edited the file. She also ran the WHOIS database and
the RFC reference service.

**And she proposed the top-level domain structure.** The categories `.com`, `.edu`, `.gov`,
`.mil`, `.org` and `.net` are her team's work, arrived at by observing what kinds of
organisation were actually connecting.

> The person who ran the system that failed is the person who designed the categories of
> the system that replaced it, which is a fairer outcome than these histories usually
> produce. Her group's operational experience is why the TLD structure fitted the actual
> user population.

**Paul Mockapetris (b. 1948).** **DNS**, at USC/ISI in 1983. RFCs 882 and 883, revised as
**1034 and 1035** in 1987 — still the specification.

The design brief was unusually hard: it had to work before the thing it names is
reachable, be incrementally deployable alongside HOSTS.TXT, have no single point of
failure, and still produce one authoritative answer.

His account of the design is characteristically modest — he has said that the hardest
part was persuading people that a distributed system could be as reliable as a file, and
that most of the design decisions were about making delegation genuinely local so that
nobody had to ask permission.

**Two properties have aged extraordinarily well:** the hierarchy has absorbed growth of
seven orders of magnitude without structural change, and caching with an explicit TTL
turned out to be exactly the right consistency model for a system where staleness is
tolerable but unboundedness is not.

**One has aged badly:** there is no authentication anywhere, for the same reason ARP has
none (Chapter 18's notes) — in 1983 the threat did not exist.

**Paul Vixie (b. 1963).** **BIND** — for decades *the* DNS server, running the majority of
the Internet's zones — and the **Internet Systems Consortium**.

**His contributions to the protocol are substantial:** **DNS NOTIFY** (RFC 1996), **IXFR**
(incremental transfer), **EDNS0** (RFC 2671, extending the 512-byte limit), and much of
DNSSEC's operational design.

He is also the field's most vocal critic of DNS abuse — of resolvers that lie, of
registrars that hijack NXDOMAIN responses to serve advertising, and of the general erosion
of the assumption that a DNS answer is the truth.

BIND's security history is instructive: it is the most-attacked DNS implementation
because it is the most-deployed, and its vulnerability record drove the creation of
deliberately smaller alternatives — NSD, Knot, Unbound, PowerDNS — built on the
principle that an authoritative server and a recursive resolver should be **separate
programs**, because they have entirely different threat models.

**Dan Kaminsky (1979–2021).** The 2008 cache-poisoning attack.

What made it significant was not the mechanism but the reframing. Cache poisoning was
known and considered impractical — one attempt per record, then wait for the TTL.
Kaminsky's insight that querying for *nonexistent* names gives unlimited attempts, and
that the additional section poisons the whole delegation, turned it into a practical attack
against any domain in seconds.

His handling of the disclosure is the part worth emulating. He notified vendors
privately, coordinated a simultaneous multi-vendor patch release in July 2008, and
withheld details for a month afterwards — and then presented the full analysis publicly.

> One of the largest coordinated security responses in Internet history, and it worked
> because he chose to make it work rather than to publish first.

He died in 2021, aged 42. His broader work on DNS, on entropy, and on the general
observation that systems built on trust assumptions fail when the assumptions change,
runs through much of modern network security.

**Dan Bernstein (b. 1971).** **djbdns**, written in response to BIND's vulnerability record.

He had implemented source port randomisation years before Kaminsky's attack made it
mandatory — not because he had predicted that specific attack, but because he regarded
predictable values in a security-relevant protocol as obviously wrong.

**The general principle is worth extracting:** defending against the class rather than
the instance. Bernstein appears in Chapter 37 for SYN cookies, and the same instinct
produced both.

Steve Crocker, Russ Mundy, Olafur Gudmundsson and the DNSSEC working group. DNSSEC, over
more than fifteen years — RFC 2065 in 1997, substantially revised as RFC 4033–4035 in
2005, and the root signed in **2010**.

**The long gestation is itself informative.** The first design (RFC 2065) had a fatal
operational problem — a zone could not be updated without re-signing everything — and it had
to be substantially rebuilt. Getting cryptographic key management right for a hierarchical
system with independent operators at every level took the working group over a decade, and
the result is hard to operate, which is why §39.4's adoption figures are what they
are.

Verisign, ICANN and the root key ceremony participants. The July 2010 root signing.

**The ceremony's theatricality is deliberate and correct.** Multiple witnesses, physical key
shares held by trusted community representatives from several countries, a documented
script, video recording. The entire DNSSEC trust chain rests on one key, and the process
by which it is generated and used must be publicly credible in a way that no purely
technical control could achieve.

It is one of the few places in this book where the answer to a trust problem is
procedural rather than technical, and it works.

The Mirai botnet's authors, and the Dyn incident. October 2016, and the lesson is
architectural rather than technical.

Mirai compromised consumer IoT devices — cameras, recorders — using default credentials,
and directed them at Dyn. The devices were individually trivial and collectively
sufficient to take a major DNS provider offline, and with it Twitter, Spotify, Reddit,
GitHub and Netflix.

> None of those companies' infrastructure failed. They shared a dependency they had not
> counted as one.

Chapter 46's IoT security and Chapter 56's availability work both trace to this
incident, and the practical conclusion — use two DNS providers — is one of the cheapest
resilience measures in this book and is still not universal.
