# Chapter 57 — The People

**Jerome Saltzer (b. 1939) and Michael Schroeder.** "The Protection of Information
in Computer Systems" (1975) — and the eight design principles that everything since has been
a restatement of.

They were written for operating systems and they transfer entirely.

| Principle | In this book |
|---|---|
| **Economy of mechanism** | **keep it simple enough to verify** — Chapter 61's WireGuard argument |
| **Fail-safe defaults** | **deny by default** — Chapter 60 §60.1's implicit deny |
| **Complete mediation** | **check every access, every time** — Chapter 59 §59.4's zero trust |
| **Open design** | **security must not depend on the design being secret** |
| **Separation of privilege** | **two conditions, not one** — multi-factor authentication |
| **Least privilege** | **Chapter 59 §59.3**, and §57.1's insider control |
| **Least common mechanism** | **shared components are shared risk** — Chapter 56 §56.2 |
| **Psychological acceptability** | **§57.4's "make the safe path the easy path"** |

> The last one is the one that took the industry forty years to take seriously. Saltzer and
> Schroeder wrote in 1975 that a mechanism people find burdensome will be circumvented, and
> the entire modern discussion of usable security is that sentence being rediscovered.

**Read the paper.** It is the single highest-value item in this unit's reading, it predates
almost everything it describes, and sections 1 and 2 take twenty minutes.

**Auguste Kerckhoffs (1835–1903).** Dutch linguist, and *La Cryptographie Militaire* (1883).

Six principles for military ciphers, of which the second is the one that survived:

> The system must not require secrecy, and it must be able to fall into the enemy's hands
> without inconvenience.

Which is the ancestor of "open design" above, and it is contested by non-specialists
constantly. Shannon's later restatement is the version usually quoted: "the enemy knows
the system."

The argument is not that secrecy is worthless. It is that secrecy of the design is
brittle — it cannot be changed when it fails, it cannot be reviewed, it will eventually be
discovered, and the system will be discovered to have been broken for years. Secrecy of the
key is different: a key can be changed.

And the practical version, which every network engineer meets: "our protocol is
proprietary" is not a security property, and it has repeatedly turned out to conceal
elementary defects.

**Willis Ware (1920–2013).** RAND Corporation, and the 1970 report that named the problem.

The "Ware Report" — *Security Controls for Computer Systems*, produced for the Defense Science
Board — is the first systematic treatment of computer security as a discipline. It was
**classified until 1979**, which is itself part of the story.

> **Its central observation was structural:** once a system is shared and connected, security
> cannot be added afterwards — **it must be a property of the design.** The report predicted,
> in 1970, essentially every category of problem in §57.4's table.

Ware also chaired the committee that produced the Fair Information Practice Principles,
which are the ancestor of every modern data protection regime — so the same person is behind
both the security and the privacy traditions.

**James Anderson.** The 1972 *Computer Security Technology Planning Study* — and the reference
monitor.

Anderson's contribution was the specification of what a security mechanism must be, and it
is three properties:

| | |
|---|---|
| **Complete mediation** | **every access is checked** |
| **Tamper-proof** | **the mechanism cannot be modified by what it controls** |
| **Small enough to be verified** | **because assurance requires review** |

> **The three properties are a useful test for any control you deploy.** A firewall that can be
> bypassed by an internal path fails the first; a management interface reachable from the
> network it protects fails the second; and a policy of forty thousand rules fails the third
> (Chapter 55 §55.1).

Anderson also produced, in 1980, the first description of intrusion detection by audit
analysis — the ancestor of everything in Chapter 54 §54.3 and of the modern SIEM.

**Dorothy Denning (b. 1945).** "An Intrusion-Detection Model" (1987) — and the idea that
attacks can be detected statistically.

Denning's model was that intrusions manifest as anomalies: behaviour that deviates from an
established profile of normal. Which is precisely Chapter 54 §54.1's baseline argument,
arriving in security a decade before it arrived in network operations.

And her paper is honest about the difficulty, in a way that the products built on it
frequently are not: the false positive rate is the binding constraint, and a detector with
a 1% false positive rate applied to millions of events produces tens of thousands of alerts —
which is Chapter 54 §54.4's alert fatigue, predicted in 1987.

**Bruce Schneier (b. 1963).** Not for a single result, but for the reframing.

Schneier's *Applied Cryptography* (1994) taught a generation the mechanisms. His later and
more important argument was that he had taught the wrong thing:

> **"Security is a process, not a product."** And: "If you think technology can solve your
> security problems, then you don't understand the problems and you don't understand the
> technology."

The shift is visible across his own books — from cryptographic algorithms, to *Secrets and
Lies* (2000) on systems and people, to *Beyond Fear* (2003) on risk and proportion. §57.3's
entire argument is his.

He also coined "security theatre" — measures that provide the feeling of security without
the substance — which is a useful and uncomfortable test to apply to your own controls.

**Adam Shostack**, and the operationalisation of threat modelling.

STRIDE was devised at Microsoft by Loren Kohnfelder and Praerit Garg in 1999, and
Shostack's work there and his *Threat Modeling: Designing for Security* (2014) turned threat
modelling from a concept into a procedure a team can actually perform.

**STRIDE's six categories** — Spoofing, Tampering, Repudiation, Information disclosure, Denial
of service, Elevation of privilege — map onto §57.2's triad: spoofing and tampering attack
integrity, information disclosure attacks confidentiality, denial of service attacks
availability, and repudiation and elevation are mechanisms.

> **Its value is not the taxonomy but the prompt.** "For each component, ask these six
> questions" is a procedure that produces findings, and a framework that a tired engineer
> can follow on a Thursday afternoon beats a better framework that requires an expert.

## What this history establishes

Almost everything in this chapter was known by 1980.

Saltzer and Schroeder's principles (1975), Anderson's reference monitor (1972), Ware's
structural argument (1970), Kerckhoffs's open design (1883). The problems in §57.4's table
were enumerated before most of the protocols in this book were written.

> **Which raises the obvious question, and the answer is uncomfortable:** the principles were
> known and the protocols were built without them, because security was expensive, the networks
> were small and trusted, and nobody was attacking them yet. Chapter 23 §23.4's account of
> why the Internet has no security layer is the same story, and every retrofit in this unit
> is the cost of that decision, still being paid.
