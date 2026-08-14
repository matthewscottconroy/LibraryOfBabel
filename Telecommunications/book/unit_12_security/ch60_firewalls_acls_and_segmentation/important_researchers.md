# Chapter 60 — The People

**Bill Cheswick and Steven Bellovin.** *Firewalls and Internet Security:
Repelling the Wily Hacker* (1994) — the book that defined the field, and the honesty that
made it good.

Both were at AT&T Bell Labs, and the book is unusual in that it is largely about failure:
what the authors' own network actually experienced, what worked, what did not, and what they got
wrong.

Cheswick's "An Evening with Berferd" (1992) is the piece worth reading first. It describes
a real intruder, over months, whom Cheswick deliberately allowed to continue in a controlled
environment — the first documented honeypot — and it is funny, detailed and slightly
uncomfortable in a way security writing rarely is.

Bellovin's 1989 paper "Security Problems in the TCP/IP Protocol Suite" is the more
consequential document.

> Written when the Internet had a few hundred thousand hosts, it enumerated sequence number
> prediction, source routing abuse, ICMP attacks, routing protocol attacks, DNS spoofing and
> RIP forgery. Almost every attack in Chapter 62 is in that paper, and almost none of
> them was fixed for a decade or more.

Bellovin has spent the intervening thirty-five years pointing out, patiently, that the
problems were known — and his later writing on why known problems remain unfixed is
Chapter 57 §57.4's economics argument, from someone who watched it happen.

**Marcus Ranum.** The first commercial firewall product, the DEC SEAL, 1990 — and
the proxy architecture.

Ranum's design was a proxy firewall, not a packet filter: every connection terminated at
the firewall and was re-originated, so the internal network never received a packet
constructed by an outsider.

> **This is §60.3's proxy argument, made first**, and **the packet-filtering approach won on
> performance and flexibility** — which Ranum has argued, at length and for decades, was the
> wrong trade.

**His larger contribution is polemical and useful.** "The Six Dumbest Ideas in Computer
Security" (2005) is four pages and its first two are worth internalising:

| | |
|---|---|
| **"Default permit"** | **enumerate what is allowed, not what is forbidden** — §60.1's implicit deny |
| **"Enumerating badness"** | **the list of bad things grows without bound; the list of good things does not** |

**And the argument has aged well.** Signature-based detection — antivirus, IPS signatures,
threat feeds — is enumerating badness, and its diminishing returns are exactly what he
predicted.

Ranum is contrarian in a way that is sometimes wrong and is reliably worth reading, because
he argues against consensus positions that have not been examined recently.

**Gene Spafford (b. 1956).** Purdue, COAST/CERIAS, and the analysis of the 1988 Morris worm.

Spafford's technical analysis of the worm — published within weeks — is the founding
document of incident analysis as a discipline: what it did, how it spread, what it exploited,
and what should change.

**His better-known contribution is a sentence:**

> "The only truly secure system is one that is powered off, cast in a block of concrete and
> sealed in a lead-lined room with armed guards — and even then I have my doubts."

Which is usually quoted as a joke and is an argument about proportionality (Chapter 57
§57.3): security is a trade against usefulness, and a system secured beyond its purpose is
not a success.

Spafford also co-wrote *Practical UNIX and Internet Security* with Simson Garfinkel, which
taught a generation how to secure systems and is unusual in the same way as Cheswick and
Bellovin's book: specific, practical, and honest about what does not work.

**Dorothy Denning**, again — Chapter 57's reading covers the intrusion detection model, and
§60.3's IDS discussion is her 1987 paper's false-positive constraint, still binding.

**Martin Roesch.** Snort, 1998 — and the argument that detection should be open.

Roesch wrote Snort as a packet sniffer that grew a rules language, and **released it free.**
Its consequence was that intrusion detection rules became a public, shared, inspectable
resource rather than a vendor's secret.

> **Which changed the field's economics.** A community writing and sharing signatures produces
> coverage no single vendor matches, and rules that anyone can read can be assessed for
> false positives before deployment — which is §60.3's tuning argument made possible.

Snort's descendants — Suricata, and Zeek from Vern Paxson's separate lineage — remain the
open detection stack, and **F6 uses one.**

**Vern Paxson.** Bro, later Zeek, from the mid-1990s — and a different philosophy.

Where Snort matches signatures, Bro parses protocols and produces structured logs. The
idea was that detection should be programmable and that the analyst should have the transaction
record, not merely the alert.

> **Which is the argument that won.** Modern detection is largely about producing rich
> structured telemetry and analysing it (Chapter 54 §54.4), **rather than about matching
> patterns in flight** — and Zeek's model prefigured it by two decades.

Paxson's measurement work is equally significant — "End-to-End Internet Packet Dynamics"
and the wider body of Internet measurement research — and it is the foundation of most of
what is actually known about how the Internet behaves rather than how it is specified.

## What this chapter's history establishes

Two arguments were made early, lost, and are being rediscovered.

Ranum's proxy architecture lost to packet filtering on performance, and it is returning as
the cloud proxy, the API gateway and the service mesh sidecar — all of which terminate
connections and re-originate them.

And "enumerate goodness, not badness" lost to signature-based detection, and it is
returning as allow-listing, as zero trust's per-application grants (Chapter 59 §59.4), and
as microsegmentation's label-based policy (§60.4). The default-deny principle keeps winning
in new contexts after losing in old ones, which suggests it was right and merely
impractical — and that what changed is the tooling, not the argument.

> The recurring lesson of this unit: the security arguments were made correctly and early, and
> lost to cost, performance and convenience. **They return when those constraints move**, and
> the engineer's useful question is not "is this the right control?" but "has the cost of the
> right control fallen far enough yet?"
