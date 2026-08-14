# Chapter 32 — Further Reading

## Primary sources

**RFC 4271 — Rekhter, Y., Li, T. & Hares, S. (2006). *A Border Gateway Protocol 4.***
The specification. **§9.1 is the decision process** of §32.2 — read that even if you read
nothing else, and notice what is absent from it.

**RFC 1105 — Lougheed, K. & Rekhter, Y. (1989). *A Border Gateway Protocol.***
The napkin protocol, as first written. Worth reading for its brevity and for how much of
it survives.

**RFC 1930 — Hawkinson, J. & Bates, T. (1996). *Guidelines for Creation, Selection, and
Registration of an Autonomous System.***
Where §32.1's definition comes from, and the guidance on when you actually need one.

**RFC 7908 — Sriram, K. et al. (2016). *Problem Definition and Classification of BGP
Route Leaks.***
The taxonomy of §32.4, made precise. Six leak types, each with a real-world example.

**RFC 6480 — Lepinski, M. & Kent, S. (2012). *An Infrastructure to Support Secure
Internet Routing.***
RPKI's architecture. Also **RFC 6811** for origin validation in BGP, which is the part a
router implements.

**RFC 7454 / BCP 194 — Durand, J., Pepelnjak, I. & Doering, G. (2015). *BGP Operations
and Security.***
**The practical one.** What to filter, what to authenticate, what to limit — the
operational checklist behind §32.2's configuration. If you will ever configure BGP, read
this.

**RFC 7999 — King, T. et al. (2016). *BLACKHOLE Community.***
Four pages standardising `65535:666`, so that RTBH works across providers.

**RFC 6793 — Vohra, Q. & Chen, E. (2012). *BGP Support for Four-Octet AS Number Space.***
The transition that worked, including `AS_TRANS`. Read it against Chapter 28 and ask what
was different.

## Books

**Doyle, J. & Carroll, J. (2001). *Routing TCP/IP, Volume 2*. Cisco Press.**
BGP in depth, with the policy tools and worked configurations. The reference for anyone
who will operate it.

**Halabi, S. & McPherson, D. (2000). *Internet Routing Architectures*, 2nd ed. Cisco
Press.**
**The book on the commercial side of §32.3** — peering, transit, multihoming, and the
economics that produce the policies. Dated in its figures and correct in its structure.

**Zhang, R. & Bartell, M. (2003). *BGP Design and Implementation.* Cisco Press.**
Scaling: route reflectors, confederations, and large-network design.

**van Beijnum, I. (2002). *BGP.* O'Reilly.**
Short, readable, and unusually good at explaining *why* rather than *how*.

**Pepelnjak, I. — ipSpace.net.**
Not a book. His writing on BGP design, and on why most people should not run it, is the
most useful current commentary available, and he is refreshingly willing to say that a
technology is being used where it should not be.

## On the incidents

**Cloudflare's post-mortem of the 24 June 2019 leak.**
**Read this one.** It names the mechanism, the parties, and what each should have done. A
model of how to publish an incident.

**RIPE NCC's and Renesys/Oracle Dyn's analyses** of the 2008 YouTube, 2010 China Telecom
and 2018 MyEtherWallet incidents.
The forensic reconstructions, with the actual announcements and timings.

**Ballani, H., Francis, P. & Zhang, X. (2007). "A Study of Prefix Hijacking and
Interception in the Internet." *ACM SIGCOMM*.**
The systematic treatment: what is possible, how detectable it is, and how effective
partial defences are.

**MANRS** (manrs.org).
The norms, the participant list, and the observatory. Check whether your own providers are
listed; it is a reasonable question to ask them.

## Applied

**Look up your own network.** `bgp.he.net`, RIPEstat, or `whois -h whois.radb.net`. Find
your AS, your prefixes, your upstreams, and **whether your prefixes have ROAs**. Exercise
F1, and most people are surprised by at least one answer.

**RIPE RIS and RouteViews.** Public BGP data from dozens of vantage points worldwide. You
can see your own prefixes as the rest of the world sees them, which is the only way to
verify that your traffic engineering did what you intended.

**`bgpq4`** — generates prefix lists and AS-path filters from IRR data. The tool that makes
§32.2's mandatory filtering practical rather than manual.

**Routinator** or **rpki-client** — run a validator. It takes an afternoon and it makes
RPKI concrete.

**BGPalerter** — monitor a prefix and see what happens over a week. Free, and it is the
difference between five minutes and two hours.

**FRRouting or BIRD in containers**, for a lab. **Exercise F3** — three ASes, implement
the valley-free rule with communities, then deliberately leak and watch it be prevented —
is the most instructive BGP exercise available, and it costs nothing.

**Lab 20** in this book's [labs/](../../../labs/) directory builds a five-AS topology with
customer, peer and provider relationships, implements valley-free policy, then stages both
a leak and a more-specific hijack and demonstrates filtering and RPKI stopping each.

## For the certification-minded

Objective 2.2 expects BGP as the exterior gateway protocol; objective 4.2 lists routing
attacks. **BGP is examined lightly relative to its importance** — the exam wants the
category and the basics, not the policy engine.

Six things worth over-learning:

1. **BGP is a path-vector EGP running over TCP port 179.**
2. **An AS is a routing-policy domain**; numbers are 16- and 32-bit; **private
   64512–65534**.
3. **The AS_PATH provides loop detection** — a router seeing its own AS discards the
   route.
4. **BGP selects by policy, not shortest path.**
5. **eBGP AD 20, iBGP AD 200.**
6. **You run BGP to multihome.** With one provider, use a default route.

And the four things worth far more than the objective, if you will work with this:

**Never configure a BGP neighbour without an outbound filter.** Every incident in §32.4
involved a missing one.

**A leak and a hijack are different faults** with different causes and different
defences.

**Get ROAs published for your prefixes.** It takes an hour, it is free, and it is now
effective because enough networks validate.

**Monitor your own prefixes.** You cannot stop a hijack; you can notice it immediately.
