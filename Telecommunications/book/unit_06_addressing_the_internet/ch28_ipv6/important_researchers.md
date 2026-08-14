# Chapter 28 — The People

**Steve Deering.** The principal architect of IPv6, with Robert Hinden, and the
author of **RFC 2460** (1998) and its successor **RFC 8200** (2017).

His work spans this book: IP multicast (Chapter 27), path MTU discovery (Chapter 24), and
IPv6. The consistent thread is **removing things**. IPv6's header is *simpler* than
IPv4's despite carrying addresses four times the size:

| Removed from IPv4 | Why |
|---|---|
| **Header checksum** | Layer 2 checks frames, Layer 4 checks end to end — the middle check is redundant (Chapter 24 §24.2) |
| **Router fragmentation** | The source discovers the path MTU; routers never fragment (Chapter 24 §24.3) |
| **Options in the header** | Replaced by extension headers, so the base header is fixed-length and fast |
| **Broadcast** | Every use replaced by multicast |
| Variable header length | Fixed 40 bytes; no IHL field needed |

Each removal is an application of the end-to-end argument (Chapter 23 §23.4), and
together they make IPv6 forwarding cheaper per packet than IPv4 despite the larger
addresses. This is not widely appreciated and it is one of the better arguments for the
protocol.

Deering has been notably candid about what was underestimated. His public position — that
the transition difficulty was badly misjudged, and backward incompatibility was the
central cost — is unusually honest for a protocol's architect, and §28.1's account
follows him.

**Robert Hinden.** Deering's co-author on the IPv6 specification and the
addressing architecture (RFC 4291), and co-chair of the IPng working group. He was also
involved in the earlier gateway and routing work at BBN, so the design reflects operating
experience rather than only architecture.

The **/64 boundary** and the address-type structure of §28.2 are largely his, and the
decision to make every subnet a /64 — extravagant by IPv4 standards and load-bearing for
SLAAC — is the clearest expression of §28.1's *never make conservation a design
constraint again.*

**The IPng selection, 1992–94.** Worth knowing because the result was not obvious.
Several proposals competed:

| Proposal | Character |
|---|---|
| **SIPP** (Deering, Hinden, Francis) | Became IPv6 |
| **TUBA** | Use OSI's CLNP addressing — **backward compatible in a sense**, and would have meant adopting an OSI protocol |
| **CATNIP** | A convergence of IP, CLNP and IPX |
| **PIP** | Paul Francis's, later merged into SIPP |

**TUBA is the road not taken.** Its advocates argued that CLNP's variable-length
addresses solved the problem with a deployed protocol, and its opponents argued that
adopting an OSI protocol after winning that argument (Chapter 22) was unthinkable.

The choice of a clean redesign over a compatible-but-compromised one is the decision
§28.1 examines, and it is genuinely arguable in both directions.

Thomas Narten, Erik Nordmark, William Simpson and Hesham Soliman. NDP (RFC 4861),
covered in Chapter 18's notes. Neighbour Unreachability Detection is the contribution
with the largest operational consequence — it is why an IPv6 host recovers from a dead
first hop without VRRP.

**Susan Thomson, Thomas Narten and Tatuya Jinmei.** **RFC 4862**, SLAAC. The mechanism of
§28.3, and the most distinctive thing IPv6 does.

Its ambition is worth stating: a network with no servers, no configuration and no
administrator should give every host a working global address and a default route. It
achieves this, and the achievement is diminished only by the RDNSS gap — for a decade,
SLAAC gave you everything except the ability to resolve a name, which meant nothing
worked.

The gap is instructive: a mechanism that is 95% complete can be 0% useful, and the
missing 5% took until RFC 8106 in 2017.

**Thomas Narten and Richard Draves.** **RFC 3041**, later **RFC 4941** — privacy
addresses. The response to the EUI-64 tracking problem of §28.3.

What is notable is the **timeline**: EUI-64 was specified in 1998, the privacy concern
was raised almost immediately, RFC 3041 appeared in **2001**, and privacy addresses
became the default across operating systems over the following decade. A privacy
problem identified early, addressed by a standard within three years, and deployed by
default — which is a far better record than most of this book's security stories, and
worth noting as a counterexample.

**Fernando Gont.** RFC 7217 (stable-privacy addresses), RFC 8064, and a great deal of
IPv6 security analysis. His work on IPv6 in operational security — the attack surface
of NDP, the rogue RA problem, extension header abuse — is the most systematic that
exists, and §28.4's security-gap argument follows his framing.

His repeated point is that IPv6 is not more or less secure than IPv4; it is
differently secure, and organisations that have not analysed the difference are exposed
by default — because IPv6 is on whether or not they deployed it.

**Dan Wing and Andrew Yourtchenko.** **RFC 6555**, Happy Eyeballs, later **RFC 8305**.

The contribution is small in specification terms and enormous in effect. Before it, a
site with a broken AAAA record was unreachable for IPv6-capable users, so publishing a
AAAA record was a risk — which meant content providers would not, which meant there was
nothing to reach over IPv6, which meant nobody deployed it.

Happy Eyeballs broke that deadlock by making broken IPv6 cost 250 ms instead of
everything. World IPv6 Launch in June 2012 — when major content providers enabled IPv6
permanently — was feasible because of it.

> Sometimes the unblocking contribution is not the protocol but the thing that makes
> deploying the protocol safe to try.

Marcelo Bagnulo, Philip Matthews, Iljitsch van Beijnum and Andrew Sullivan. NAT64
(RFC 6146) and DNS64 (RFC 6147) — the mechanisms that let an IPv6-only network reach the
IPv4 Internet, and therefore the mechanisms that made IPv6-only deployment possible at
all.

**Masataka Mawatari, Masanobu Kawashima and Cameron Byrne.** **RFC 6877**, 464XLAT.
Byrne was at T-Mobile US and drove the deployment, which is why the carrier has run an
IPv6-only mobile core since 2014.

464XLAT is the most successful IPv6 transition mechanism by user count, running on
hundreds of millions of handsets, and almost nobody outside the field has heard of it.

**Geoff Huston.** Again. His IPv6 adoption measurements are the source of every
figure in §28.4, and his long-running commentary on the transition — sceptical,
data-driven, and unsentimental about the protocol's advocates — is the best available
antidote to both IPv6 boosterism and IPv6 dismissal.

**His most useful observation for a student:** IPv6 adoption is not one number. It is a
different number for content, for access networks, for enterprises and for internal
infrastructure, and the four have diverged enormously — which is exactly what §28.4's
incentive argument predicts.
