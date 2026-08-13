# Chapter 59 — The People

**Roger Needham (1935–2003) and Michael Schroeder.** **"Using Encryption for Authentication in
Large Networks of Computers" (1978)** — **the foundational protocol, and the foundational
lesson.**

**The Needham–Schroeder protocol establishes a session key between two parties via a trusted
third party**, and **it is the direct ancestor of Kerberos.**

**Its lesson is more valuable than the protocol.**

> **In 1981 Denning and Sacco found a flaw**: **an attacker who obtains an old session key can
> replay a recorded message and impersonate a party indefinitely**, because **the protocol
> contained no freshness guarantee.**

**And in 1995 — seventeen years after publication — Gavin Lowe found a second flaw** in the
public-key variant, **using an automated model checker.** **The protocol had been published,
reviewed, taught and implemented for seventeen years with a flaw that a machine found in
minutes once someone thought to look.**

> **Authentication protocols are extraordinarily hard to get right, and human review does not
> reliably find the errors.** **This is the origin of formal verification in security
> protocols**, and it is why modern protocols — TLS 1.3 among them — are verified mechanically
> before publication rather than after.

**Needham's other contributions are substantial:** **password hashing with salt** (with Mike
Guy, at Cambridge in the 1960s — **the technique predates its rediscovery by decades**),
**the Needham–Schroeder–Lowe corrections, and much of the vocabulary of the field.**

**Roger Needham also coined the observation that** *"programming Satan's computer"* **describes
security protocol design** — **you must assume the machine actively rearranges your messages to
defeat you** — which he developed with **Ross Anderson** in a paper of that title.

**The Kerberos team — Steve Miller, Clifford Neuman, Jeffrey Schiller and Jerome Saltzer.**
**MIT's Project Athena, from 1983.**

**Kerberos took Needham–Schroeder into production**, and its design goal was specific: **a
campus of thousands of untrusted workstations where a user's password must never cross the
network and must never be stored on the machine they are using.**

| The mechanisms | |
|---|---|
| **Tickets** | **time-limited credentials, so a stolen ticket expires** |
| **A ticket-granting ticket** | **obtained once; used to get service tickets** — single sign-on, actually implemented |
| **Timestamps** | **the freshness that Needham–Schroeder lacked** |
| **Mutual authentication** | the service proves itself too |

> **The timestamp requirement is why Kerberos breaks when clocks drift**, and it is why
> **Chapter 41 §41.3 insisted that NTP is a security dependency.** **A five-minute clock skew
> and Kerberos stops working entirely**, which every Windows administrator has met.

**And Kerberos is the authentication in every Windows domain**, which makes it — quietly — among
the most widely deployed security protocols in existence.

**The RADIUS authors — Carl Rigney and colleagues at Livingston Enterprises, 1991.**

**RADIUS was built to solve a specific commercial problem: a bank of dial-up modems needed to
check a user's credentials against a central database.** **That is the entire original
requirement**, and **the protocol's shape follows from it** — UDP because it was simple,
minimal cryptography because the link was a serial line inside a machine room, and a shared
secret because there were three devices.

> **Its longevity is remarkable and largely accidental.** **A protocol designed for a modem pool
> in 1991 now authenticates most of the world's enterprise wireless and wired access** — because
> **802.1X needed a back end and RADIUS was there, universally implemented, and adequate.**

**And its weaknesses are all consequences of the original scope** (§59.2). **The people who
built it were not careless; they were solving a smaller problem correctly.**

**John Kindervag**, again — **Chapter 51's reading covers him, and the substance belongs here.**

**Kindervag's 2010 Forrester work named zero trust**, and **the argument's substance was not
new** — **the Jericho Forum's de-perimeterisation papers (2004) and Saltzer and Schroeder's
complete mediation (1975) contain it** — **but the name was fundable.**

> **"Never trust, always verify" is a slogan, and beneath it is a specific claim: that network
> location is not evidence of anything**, and that treating it as evidence had been wrong for
> years before anyone said so.

**The BeyondCorp team at Google, 2014 onwards** — **the implementation, documented.**

**What makes the BeyondCorp papers unusually valuable is that they describe the migration, not
the destination:**

- **It took years**
- **A device inventory had to be built first**, and it did not exist
- **Legacy applications required a proxy tier**
- **The unmanaged-device policy had to be decided before anything else could proceed**
- **Every step had to preserve the ability to work**

> **Most zero trust material describes an architecture. BeyondCorp describes a project**, with
> its costs, and that is why it remains the most useful thing published on the subject.

**Cliff Neuman and Tatu Ylönen**, for the two things that replaced passwords on the wire.

**Neuman's Kerberos work is above.** **Ylönen wrote SSH in 1995**, in response to a
password-sniffing attack on the Helsinki University of Technology network — **and the
circumstances are worth stating: he wrote it because someone was capturing plaintext telnet
passwords on his own network, and he released it in July of that year.**

**Adoption was extraordinarily fast.** **Within a year it had tens of thousands of users**,
because **the problem it solved was one every administrator had and nobody had a fix for.**

> **SSH's history is the counter-example to this book's usual pattern.** **It was not a
> standards committee, not a large vendor, and not a research programme.** **It was one person
> solving an urgent problem well, and the resulting protocol became universal before it was
> standardised** — **the IETF's SSH working group formed after the deployment, not before.**

## What this chapter's history establishes

**Two patterns.**

**Authentication protocols are exceptionally hard, and review does not find the errors.**
**Needham–Schroeder stood for seventeen years with a flaw a model checker found in minutes.**
**This is why the field moved to formal verification**, and why **"we designed our own
authentication protocol" should worry you as much as "we implemented our own cryptography."**

**And the protocols in production were built for smaller problems.** **RADIUS for a modem pool.
Kerberos for a campus. LDAP for a directory.** **Each is now doing something far beyond its
original scope, adequately, because it was there and it worked** — which is Chapter 54's
observation about SNMP, syslog and NetFlow, in a different unit.

> **The corollary is practical: when a protocol behaves oddly, its original purpose usually
> explains why.** **RADIUS's cryptography, Kerberos's clock sensitivity and LDAP's schema
> awkwardness are all legible once you know what problem each was actually built for.**
