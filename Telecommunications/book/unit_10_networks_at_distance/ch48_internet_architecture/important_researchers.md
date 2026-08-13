# Chapter 48 — The People

**Jon Postel (1943–1998).** **IANA, for nearly thirty years, was substantially one person.**

Postel edited the RFC series from 1969, maintained the assigned-numbers registries, ran the
`.us` domain, and administered the root zone — **and he did all of it as an employee of a
university research institute, on the strength of nobody objecting.**

> **The Internet's central registry functions were performed for three decades by a man with a
> beard and a text file**, and the arrangement worked because everyone involved trusted him and
> because the alternative had not been invented.

**His formulation of the robustness principle** — *"Be conservative in what you send, be
liberal in what you accept"* (RFC 761, 1980) — **shaped a generation of implementations**, and
it is now genuinely contested. The argument against it is that liberal acceptance **entrenches
buggy senders**, producing protocols whose real specification is "whatever the dominant
implementation does" — and that security-critical parsers should reject anything nonconforming
outright. **Both positions have merit and the modern consensus has moved substantially towards
strictness.**

**The 1998 root incident is the story worth knowing.** In January, Postel emailed the operators
of eight of the thirteen root servers and **asked them to point at his machine at ISI
rather than at the government-designated root.** They did. **For a period the Internet had two
roots, one of them Postel's**, and the government's response was immediate and severe.

**He called it a test.** Others called it a demonstration of where authority actually lay.
**Within months the process that produced ICANN was underway**, and Postel died in October of
that year, aged 55, before it concluded.

> **The incident is instructive because it shows the system's real structure.** Authority over
> the root rested on **the willingness of a dozen operators to accept an email from someone
> they trusted** — and formalising that was the whole point of ICANN.

**Vint Cerf's obituary RFC — RFC 2468, *I Remember IANA*** — is worth reading and takes five
minutes.

**David D. Clark (b. 1944).** MIT, and **the IETF's chief protocol architect from 1981 to
1989.**

**The "rough consensus and running code" line is from a 1992 presentation** on the future of
Internet architecture, and it was **a deliberate rejection of the ISO process** rather than an
abstract principle. Clark also wrote **"The Design Philosophy of the DARPA Internet Protocols"
(1988)**, which sets out the priority ordering the designers actually used — survivability
first, multiple service types second, variety of networks third, **and accountability last** —
**and explains a great deal about why the Internet's security and billing are the way they
are.**

> **Clark's later work is more interesting than the slogan.** He has argued for two decades
> that **the Internet's architecture cannot express the requirements now placed on it** —
> trust, accountability, economics — because those were explicitly deprioritised in 1978, **and
> that no amount of protocol work at the edges will supply what the architecture omits.**

**Steve Crocker (b. 1944).** **Wrote RFC 1 in April 1969**, and invented the RFC series in the
process.

**The name was chosen out of diffidence.** Crocker was a graduate student, the group had no
authority to issue standards, and **"Request for Comments" was intended to signal that these
were notes rather than pronouncements.** He has said he agonised over the tone, wrote it in a
bathroom at night so as not to wake his hosts, and **worried that it presumed too much.**

**The name stuck for over nine thousand documents** and the diffidence became a culture:
**anyone may write one, the barrier is technical merit, and the document does not become
authoritative by being published.**

**Elise Gerich, Kim Davies and the institutionalisation.** After Postel, the
IANA functions became a **job description rather than a person** — which is less romantic and
substantially more robust. **Gerich led IANA through the stewardship transition**, and **Davies has run the root zone
management function since** — under PTI, with a documented process, an audit trail, and a
succession plan.

> **The interesting engineering here is organisational.** The problem was to take a function
> performed on trust by one individual and make it survive that individual, resist capture by
> any government or company, and remain accountable to a community with no formal membership.
> **It took eighteen years and it worked**, which is worth more attention than it receives.

**Bill Norton.** Not a researcher — **an analyst, and the person who made peering economics
legible.**

**Norton's *Internet Peering Playbook* and the earlier "Peering White Papers"** documented what
had been folklore: how peering is negotiated, what the ratio arguments actually claim, how the
cost comparison is constructed, and **how the negotiation tactics work.** The material is
frank in a way industry documents rarely are.

**He also coined "peering coordinator" as a job description** and documented the informal
culture — **the conference-corridor agreements, the dinner negotiations, the reputation
economy** — that governs an infrastructure carrying a substantial fraction of global commerce.

> **Peering runs on relationships between a few hundred people who mostly know each other**,
> and Norton is the reason that fact is documented rather than merely true.

## A note on the shape of authority

**This chapter's institutions have a common property: none of them can compel anyone.**

**IANA cannot force a network to use the addresses it was allocated.** The RIRs cannot stop a
network announcing space it does not hold. **The IETF cannot make anyone implement an RFC.**
ICANN cannot prevent an alternative root.

**What they have instead is the coordination advantage.** A network that announces someone
else's space is filtered by its peers; an implementation that ignores an RFC does not
interoperate; **an alternative root reaches nobody.** **Compliance is voluntary and
non-compliance is useless**, which turns out to be a stronger arrangement than enforcement in
a system nobody owns.

> **The Internet is governed the way a language is governed** — by convention, by usefulness,
> and by the fact that deviation costs the deviator. **It is not obvious that this should
> work at the scale of global infrastructure, and it has for fifty years.**
