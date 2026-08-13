# Chapter 49 — The People

**Joseph W. Lechleider (1933–2015).** Bellcore — **and the person who made DSL possible by
proposing that it should be asymmetric.**

**The problem in the late 1980s was crosstalk.** Sending data in both directions at high
frequency on adjacent pairs in the same bundle **produced near-end crosstalk severe enough to
make symmetric high-rate DSL unworkable** over useful distances. The industry's conclusion was
that telephone copper could not carry broadband.

**Lechleider's insight was to give up on symmetry.** **Make the downstream fast and the
upstream slow, and place them in different frequency bands** — which reduces the crosstalk
problem dramatically and, **as it happens, matches what the emerging applications actually
needed.**

> **He was arguing for asymmetry in 1988, before the web existed.** The justification at the
> time was video on demand; **the application that arrived was the web, and it was even more
> asymmetric than video.** A design decision made for one predicted use turned out correct for
> a different actual one.

**ADSL reached hundreds of millions of subscribers**, and it did so on telephone plant that had
been declared useless for the purpose. **Lechleider's contribution was an argument rather than
a device**, which is a harder thing to be credited for and was, eventually.

**John Cioffi (b. 1956).** Stanford — **"the father of DSL", and the person who put DMT into
it.**

**Cioffi's contribution was to insist on multicarrier modulation** against a standards
committee that preferred single-carrier CAP. **The argument was that a copper loop's channel
quality varies enormously across frequency** — attenuation rising with $\sqrt{f}$, bridged taps
producing nulls, crosstalk concentrated in bands — **so a single wideband carrier must be
engineered for the worst part of the spectrum, while many narrow carriers can each be loaded
according to what they individually carry.**

**He won the standards fight in 1993**, largely by demonstrating it, and **DMT is in every DSL
variant since.**

> **The same argument, made again in three other places in this book:** OFDM in Wi-Fi
> (Chapter 44), OFDMA in LTE (Chapter 46), OFDM in DOCSIS 3.1 (§49.2). **Cioffi's committee
> fight in 1993 was the first production deployment of an idea now underneath most of the
> world's data.**

**Cioffi went on to found Amati** (acquired by Texas Instruments in 1997), **to work on
vectoring and G.fast**, and to argue — correctly and unfashionably — **that the copper had far
more left in it than anyone believed.** He holds the Marconi Prize and the IEEE Alexander
Graham Bell Medal.

**Claude Shannon (1916–2001), again.** Not a contributor to this chapter's technology, and
**the person whose result governs it.**

**§49.1's central fact — that the modem industry converged on 33.6 kb/s and stopped — is a
Shannon result**, and it is the clearest practical illustration in this book of a capacity
bound being reached rather than approached. **Chapter 4 gives the theory; the V.34 modem is
the experiment.**

> **Very few engineering fields have a number that says "no further".** Communications does,
> **and the modem industry spent a decade and a half arriving at it from below.**

**Rouzbeh Yassini.** **The cable modem's commercialisation, and the origin of
DOCSIS.**

**Yassini founded LANcity in 1990** and built cable modems when the cable industry did not
believe there was a market. **The significant contribution was not the modem but the
standard.** Early cable modems were proprietary and incompatible; **an operator buying from one
vendor was locked to it permanently**, and no volume market could form.

**Yassini argued for a common specification** and worked with CableLabs to produce **DOCSIS
1.0 in 1997.** **Interoperability collapsed the price of a modem from hundreds of dollars to
tens**, and the subscriber base followed.

> **The pattern is Chapter 20's Ethernet story exactly.** A standard that permits several
> vendors is worth more to every vendor than a proprietary system that locks in customers —
> **and this is not obvious to the vendor holding the lock-in.**

**Arthur C. Clarke (1917–2008).** **The geostationary orbit, proposed in a four-page article in
*Wireless World* in October 1945**, eighteen years before Syncom 2 reached it.

**"Extra-Terrestrial Relays" sets out the whole idea**: three satellites at 35,786 km, each
stationary above a point on the equator, **together covering the populated Earth**, relaying
radio between continents.

**Clarke did not patent it.** He later wrote — with more humour than regret — a piece titled
*"A Short Pre-History of Comsats, Or: How I Lost a Billion Dollars in My Spare Time."*

> **The geostationary belt is formally called the Clarke Orbit**, and his article is another
> entry in this book's collection of **complete designs published decades before the components
> existed** — Ring's cells (Chapter 46), Stockman's backscatter (Chapter 47), Perlman's routed
> Layer 2 (Chapter 19).

**Harold Rosen (1926–2017)** built it. **Hughes Aircraft, and Syncom 2 in 1963** — the first
geosynchronous communications satellite, achieved against internal opposition and a widespread
belief that the round-trip delay would make voice unusable. **It was usable, and it was
strange, and the world adapted.**

**Donald Kessler (b. 1940).** NASA — **and the reason §49.4's orbital congestion discussion is
not alarmism.**

**The 1978 paper with Burton Cour-Palais** described a mechanism: **above a certain density of
objects in an orbital shell, collisions produce debris faster than debris decays**, and the
population grows without further launches. **An orbital band can become unusable, and it stays
unusable for a very long time.**

> **The Kessler syndrome is now a routine consideration in constellation licensing**, and
> operators perform collision-avoidance manoeuvres continuously. **A prediction made in 1978
> about a regime that did not yet exist is now an operational cost line.**

## A note on the shape of this chapter

**Three of this chapter's technologies exist because someone refused to accept a stated
limit.**

**Lechleider was told copper could not carry broadband.** **Cioffi was told single-carrier was
adequate.** **Yassini was told there was no market for a cable modem.** In each case the
consensus was reasonable, evidence-based and wrong.

**And one of them is the opposite case.** **Shannon's bound was stated in 1948 and it held**,
and the modem industry's fifteen-year approach to 33.6 kb/s is what a real limit looks like
from below.

> **The engineering skill this chapter demands is telling those two situations apart** — a
> limit that is a property of the physics from a limit that is a property of the equipment
> someone attached to it. **The 3.4 kHz telephone channel was the second kind, and recognising
> that was worth hundreds of billions of dollars.**
