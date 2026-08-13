# Chapter 62 — The People

**Robert Tappan Morris (b. 1965).** **The Morris worm, 2 November 1988 — and the incident that
created the field.**

**A graduate student's program**, intended — by his own account — **to measure the size of the
Internet.** **A reinfection bug made it spread far faster than intended**, and it **disabled roughly a
tenth of the 60,000 hosts then connected** — by reinfecting them until they could do nothing
else.

**Its mechanisms are a catalogue of this book's material:**

| | |
|---|---|
| **A buffer overflow in `fingerd`** | the first widely known instance |
| **A debug backdoor in `sendmail`** | left in production builds |
| **Weak passwords**, from a 432-word dictionary | Chapter 59 §59.1 |
| **Trust relationships** — `rhosts` | Chapter 59 §59.3's lateral movement, in 1988 |

> **Every category in Chapter 57 §57.4's table is in the worm**, and **it was the first
> demonstration that a network's connectivity is also its attack surface.**

**The consequences were institutional.** **CERT/CC was founded within weeks**, at Carnegie
Mellon, **and it is the ancestor of every national CERT.** **Morris was the first person
convicted under the US Computer Fraud and Abuse Act**, received community service and a fine,
**and is now a professor at MIT** — where his work on distributed systems is substantial and
entirely unrelated.

**Steven Bellovin**, for the third time in this unit — **and here for the observation that
matters most.**

**His 1989 paper enumerated the attacks in §62.1 and §62.2 before most of them had names.**
**And his later writing makes the point this chapter closes on:**

> **The problems were known. They were documented. They were not fixed, because fixing them
> required action by parties who bore the cost and not the benefit.**

**Which is Chapter 57 §57.4's BCP 38 argument, and it is Bellovin's rather than this book's.**

**Paul Vixie (b. 1963) and Dan Kaminsky (1979–2021).** **DNS, from both sides.**

**Vixie wrote and maintained BIND for many years**, founded the **Internet Software Consortium**,
and **created the first anti-spam DNS blocklist** — **which makes him responsible for a
substantial fraction of the DNS infrastructure that Chapter 39 describes.**

**Kaminsky's 2008 discovery is the one worth studying**, and **for the disclosure as much as the
technique.**

**The flaw was in every major DNS implementation simultaneously.** **Kaminsky did not publish.**
**He assembled the vendors — Microsoft, Cisco, ISC, Sun and others — at a secret meeting at
Microsoft's campus, coordinated a simultaneous patch release across the industry on 8 July
2008, and gave them thirty days before disclosing.**

> **The coordination worked, and it worked because he had the discipline not to publish.**
> **It is the model for coordinated vulnerability disclosure**, and **the details leaked
> anyway** — a security researcher reconstructed the flaw from the patch within thirteen days,
> **which is itself the lesson that a patch discloses the vulnerability.**

**Kaminsky spent the rest of his career on DNSSEC and on the argument that the Internet's
foundational protocols needed authentication.** **He died in 2021, aged 42**, and the field
lost one of its very few people who was simultaneously an excellent researcher and an
excellent communicator.

**Moxie Marlinspike.** **SSL stripping, sslsniff, and the argument about where security must
live.**

**Marlinspike's 2009 Black Hat presentation demonstrating SSL stripping** (§62.2) **changed how
the web was deployed** — **HSTS, HTTPS-first browsing and eventually HTTPS-by-default are all
downstream of it.**

**His larger argument is about usability and authority:**

> **Security that depends on a user noticing something — a padlock, a certificate warning, a
> URL — will fail**, because **users are not attending to it and cannot be made to.** **The
> mechanism must not require them to.**

**Which is Chapter 59 §59.1's FIDO2 argument, made a decade earlier about a different
mechanism**, and it is the same principle as Saltzer and Schroeder's psychological
acceptability.

**Marlinspike went on to found Signal** — **where the same argument produced a messaging system
whose security requires nothing of the user at all** — and has been consistently critical of
security models that assume attentive users.

**Robert Graham, Dug Song, and the tool authors.**

**`dsniff` (Song, 1999)** — **`arpspoof`, `macof`, `dnsspoof`, `sshmitm`** — **made the attacks
in §62.1 and §62.2 available to anyone.**

**The disclosure argument applies here and is worth engaging with honestly:**

> **Song's tools demonstrated that switched networks were not the security boundary people
> believed.** **Before them, "we use switches, so sniffing is not possible" was a widely held
> and false belief.** **After them, it was untenable** — **and the mitigations in §62.1 were
> developed and deployed in response.**

**The counter-argument — that the tools enabled attacks that would not otherwise have occurred
— is real and is weaker**, because **the techniques were already known to those who cared to
find them**, and **the tools' effect was to inform defenders who did not.**

**Dan Geer**, for the argument this chapter's closing section makes.

**Geer's writing on security economics and on monoculture** — **including the 2003 paper
"CyberInsecurity: The Cost of Monopoly", which cost him his job** — **makes the case that
diversity is a security property.**

> **A population in which every host runs the same software has one vulnerability away from
> total compromise.** **§62.4's independence requirement is the same argument at a smaller
> scale**, and **the objection to it — that diversity costs money and expertise — is the same
> objection.**

**Geer's later work on the economics of vulnerability disclosure and on the vulnerability
market is the honest treatment of a subject that is usually discussed dishonestly.**

## What this chapter's history establishes

**Three things, and they are uncomfortable.**

**The attacks were documented before the systems were deployed.** **Bellovin 1989. Morris 1988.
Saltzer and Schroeder 1975.** **The gap is not knowledge.**

**Coordinated disclosure works and is fragile.** **Kaminsky's coordination succeeded and the
details leaked in thirteen days.** **A patch is a disclosure**, and the defender's window is
therefore short and getting shorter — Chapter 55 §55.3's emergency track exists for this
reason.

**And the fixes fail on economics rather than on engineering.** **BCP 38, RPKI, DNSSEC,
reflector closure** — **each is technically settled and each requires action from a party that
does not benefit.** **Which means the remaining work is not protocol design.**

> **This unit began with Shannon's diagram and an adversary placed on the channel, and it ends
> with an observation about incentives.** **That is not a failure of the technical material; it
> is where the technical material honestly leads.**
