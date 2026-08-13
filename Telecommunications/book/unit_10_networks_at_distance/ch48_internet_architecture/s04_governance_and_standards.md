# 48.4 Governance and Standards

**Nobody is in charge, and yet the thing has standards.** Understanding how that works changes
how you read a specification and how much weight you give it.

## Rough consensus and running code

**The IETF's founding norm**, stated by **David Clark in 1992** in a slide that has outlived
almost everything else from that decade:

> *"We reject: kings, presidents and voting. We believe in: rough consensus and running
> code."*

**Both halves are load-bearing.**

**Rough consensus** is not unanimity and it is not a majority. **It is the chair's judgement
that the remaining objections have been heard and addressed**, not that everyone agrees. **A
single well-argued technical objection can block a document**; a hundred people saying "I don't
like it" cannot. **The distinction is between objections with technical content and objections
without.**

**Running code** means **an implementation is evidence and a design is a claim.** The IETF
strongly prefers protocols that have been built and tested, and **it will change a
specification to match what implementations actually do.**

> **This is precisely the difference that decided OSI's fate** (Chapter 22 §22.1). OSI was
> designed completely and then implemented partially; TCP/IP was implemented and then
> documented. **One of those approaches produces specifications that work.**

**The consequences of "no voting" are not all comfortable**, and it is worth being honest:

**No membership means no accountability.** Anyone may participate; **influence accrues to
those who can afford to attend three meetings a year and read a great deal of email**, which
skews heavily towards large employers.

**Consensus is slow.** A contested document takes years. **HTTP/2 and QUIC each absorbed
half a decade of working-group effort** before publication.

**And chairs have real power** — determining whether consensus exists is a judgement call, and
the appeals process is rarely used and rarely successful.

**Against which:** the process has produced IP, TCP, DNS, HTTP, TLS and BGP, **and nothing
better has been demonstrated.**

## How a document becomes an RFC

```
   idea ──▶ Internet-Draft ──▶ WG adoption ──▶ WG Last Call
                (6 months,        (the WG        (does the WG
                 expires)          owns it)       agree?)
                                                     │
   RFC ◀── RFC Editor ◀── IESG approval ◀── IETF Last Call
              (publish)      (the area          (the whole
                              directors)         community)
```

**Points worth knowing:**

**An Internet-Draft is not a standard and expires after six months.** Citing
`draft-something-05` as authoritative is a common and visible error. **They are working
documents.**

**An RFC is immutable.** It is never edited after publication. **Corrections happen by
publishing another RFC** that updates or obsoletes it — which is why **checking whether an RFC
has been obsoleted is a necessary habit** and why the RFC Editor's index shows the
relationships.

**The RFC number tells you nothing about status.** RFC 1149 and RFC 9293 are both RFCs; one
specifies TCP and the other specifies carrier pigeons.

## Not every RFC is a standard

**The category that catches people out.**

| Category | Meaning |
|---|---|
| **Internet Standard** (STD) | **fully standardised; the small, tested core** |
| **Proposed Standard** | **the normal state of a working protocol** — most of what you use |
| **Best Current Practice** (BCP) | operational guidance, not a protocol |
| **Informational** | a description, an idea, or a vendor's protocol documented for reference |
| **Experimental** | genuinely uncertain; may be abandoned |
| **Historic** | superseded or abandoned |

**"Proposed Standard" sounds provisional and is not.** **TLS 1.3 (RFC 8446), HTTP/2, and QUIC
are all Proposed Standards**, deployed globally, carrying most of the web. **The formal
progression to Internet Standard is done rarely** because it takes effort and confers little
benefit, so the label under-describes maturity almost universally.

**Informational is where the traps are.** **An Informational RFC may document a single
vendor's proprietary protocol** with no community review of its merits. It is a real RFC, with
a real number, and **it is not an endorsement.**

> **Reading habit: check the status header and the "Obsoleted by" line before citing anything.
> It takes five seconds and prevents a specific and embarrassing class of error.**

**And the April Fools tradition is real.** RFC 1149 (*IP over Avian Carriers*, 1990) has been
**implemented and demonstrated** — the Bergen Linux User Group ran it in 2001, achieving
round-trip times of roughly an hour and losing more than half the packets. RFC 2324 (*HTCPCP*, the Hyper Text Coffee Pot Control
Protocol) defines **HTTP status code 418, "I'm a teapot"**, which several real HTTP libraries
implement in earnest.

## The other bodies

**The IETF does not do everything, and knowing the division saves time.**

| Body | Domain | Access |
|---|---|---|
| **IETF** | Internet protocols — IP, TCP, DNS, HTTP, TLS, BGP | **free, open, no membership** |
| **IEEE** | **802: Ethernet, Wi-Fi, 802.1Q, 802.15.4** | **paid membership; standards cost money** |
| **ITU-T** | international telecoms, optical (G.709, G.652), some legacy | member states and paying sector members |
| **3GPP** | cellular — GSM through 5G and beyond | industry consortium |
| **W3C** | web: HTML, CSS, DOM | membership, with open participation routes |
| **ICANN** | **names, the root zone, gTLD policy** | multistakeholder, contentious |
| **CSA, LoRa Alliance, Bluetooth SIG, WFA** | industry consortia (Chapter 47) | paid membership |

**The access column is the interesting one.**

**IETF standards are free.** Anyone may read RFC 9293 today at no cost. **IEEE standards
cost hundreds of dollars each**, and 802.11's full specification is thousands of pages behind
a paywall — **which is a genuine barrier to students and small implementers**, partially
mitigated by IEEE's "Get 802" programme releasing standards free after six months.

> **A specification's accessibility affects who can implement it**, and the Internet's core
> protocols being free to read is not incidental to their adoption. Chapter 22's OSI
> comparison applies here too: **ISO documents cost money; RFCs never have.**

## ICANN, and the part that is genuinely political

**Names are political in a way addresses are not**, because a name means something and an
address does not.

**ICANN's remit:** the root zone (Chapter 39 §39.1), gTLD policy and creation, accreditation
of registrars, and the **UDRP** dispute process for trademark conflicts.

**Its structure is "multistakeholder"** — governments, businesses, technical bodies, civil
society and users, each with defined roles and none with control. **The Governmental Advisory
Committee advises and cannot direct**, which is the arrangement several governments have
consistently disliked.

**The 2012 gTLD expansion** took the root from 22 generic top-level domains to **over 1,200**.
The outcomes were mixed: `.xyz`, `.app` and `.dev` found real use; **many were bought
defensively by brands that never used them**; and `.zip` and `.mov` — **delegated in 2023 —
collide with common file extensions in a way that creates genuine phishing risk**, which is a
case of name policy producing a security consequence.

**And the alternative to multistakeholder governance has a name: the ITU model**, in which
states negotiate and vote. **The 2012 WCIT conference in Dubai was the flashpoint** — a
proposal to extend ITU authority over Internet governance, which around fifty countries
declined to sign. **The disagreement has not been resolved; it has been deferred**, and it
recurs at every subsequent conference.

> **This is not a technical dispute and it determines technical outcomes.** An engineer who
> ignores it will one day be surprised by a requirement that has no engineering justification.

## Reading a specification, practically

**A short protocol for approaching a document you have not read before:**

1. **Check the status and the obsoletion header.** Five seconds.
2. **Read the abstract and the introduction.** Frequently sufficient.
3. **Find the packet format or state machine.** The normative content is usually in one or two
   sections.
4. **Read the Security Considerations section.** **required in every RFC since the 1990s**, and
   often the most honest part of the document — it is where authors state what the protocol
   does not protect against.
5. **Check the IANA Considerations section** for the registries the document creates.
6. **Ignore the rest until you need it.**

> **RFCs are reference documents, not textbooks.** Nobody reads RFC 9293 end to end. **The
> skill is finding the paragraph that answers your question**, and it improves with practice.

## What breaks here

**Implementing from an expired Internet-Draft.** It may have changed substantially. **Find the
RFC, or the current draft revision.**

**Citing an RFC that has been obsoleted.** **RFC 793 was obsoleted by RFC 9293 in 2022** —
forty-one years of accumulated TCP errata and clarifications consolidated into one document — and citing 793 now marks a document as
unmaintained.

**Assuming "Proposed Standard" means immature.** It does not. **Most of the Internet runs on
Proposed Standards.**

**Treating an Informational RFC as a community standard.** Check who wrote it and why.

**Expecting a vendor's implementation to match the RFC.** **Read the Security Considerations
and then read the vendor's release notes**, because divergence is normal and usually
documented somewhere unhelpful.

**Waiting for a standard before deploying.** **Running code precedes the standard by design.**
QUIC carried a substantial fraction of Google's traffic for years before RFC 9000.

> **Network+ note.** Objective 1.1 and the general "standards organisations" material.
> Over-learn: **IETF produces RFCs for Internet protocols**; **IEEE produces the 802 family**;
> **ITU-T covers international telecommunications**; **ICANN administers names and the root
> zone**; and **IANA assigns numbers.** The mapping of body to domain is the examinable
> content.
