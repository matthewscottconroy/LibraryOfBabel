# 57.2 Confidentiality, Integrity, Availability

**The triad is usually presented as a list to memorise. It is better understood as a
derivation**, and the derivation is short.

## The three verbs

**Return to Shannon's diagram** (Chapter 1 §1.2) **and place an adversary on the channel.**

```
   ┌────────┐   ┌───────────┐              ┌──────────┐   ┌─────────────┐
   │ Source │──▶│Transmitter│──── channel ─│ Receiver │──▶│ Destination │
   └────────┘   └───────────┘       │      └──────────┘   └─────────────┘
                                    │
                            ┌───────┴───────┐
                            │   ADVERSARY   │
                            └───────────────┘
                       what, exactly, can they do?
```

**Three things. That is the complete list.**

| Verb | Property attacked | Defence |
|---|---|---|
| **Listen** | **confidentiality** | **encryption** |
| **Alter** | **integrity** | **cryptographic authentication** |
| **Prevent** | **availability** | **capacity and filtering** |

**And note that the list is exhaustive.** **There is no fourth thing you can do to a channel** —
you may observe what crosses it, change what crosses it, or stop things crossing it.

> **That completeness is why the triad has survived as a framework, and it is a much better
> reason to believe it than the fact that it appears on a certification syllabus.**

## Listening, and why only encryption works

**Copying what crosses the channel without altering it. The communication proceeds normally and
the parties have no way to detect it.**

**The undetectability is the important property.** **A tap on a fibre, a passive receiver in
range of an access point, a mirror port, a compromised router** — **none of them changes what the
endpoints observe**, so **no protocol mechanism can reveal them.**

> **You cannot prevent someone from receiving radio waves or splicing a fibre.** **You can only
> ensure that what they receive is useless**, and that is what encryption is for.

**What encryption does not protect:**

| | |
|---|---|
| **That the communication occurred** | **traffic analysis** — Chapter 54 §54.4's flow records, from the adversary's side |
| **Who talked to whom** | addresses are in the clear |
| **How much, and when** | **volume and timing leak a surprising amount** |
| **The endpoints themselves** | **encryption in transit protects transit** |

**Traffic analysis deserves more respect than it gets.** **Knowing that a device contacted a
particular medical, legal or recruitment service, at a particular time, with a particular volume
of data, is frequently sufficient** — **and TLS provides none of it.** **This is the same
argument as §54.4's privacy discussion, from the other direction.**

## Altering, and why a checksum is not enough

**Modifying data in transit, injecting data that was never sent, or replaying data that was sent
earlier.**

**The critical distinction, and it is the one people get wrong:**

> **A checksum detects accident. It does not detect intent.**

**Chapter 15 §15.4's CRC is excellent at catching a bit flipped by noise** — it is designed for
exactly that. **It is useless against an adversary, because anyone who alters the data can
recompute the checksum**, and the recomputation is a published algorithm requiring no secret.

**What is required is a keyed mechanism:**

| Mechanism | Requires | Gives |
|---|---|---|
| **Checksum / CRC** | **nothing** | **accident detection** |
| **Hash** | nothing | **integrity against accident; not against an adversary who can also change the hash** |
| **MAC (keyed hash)** | **a shared secret** | **integrity and origin, between parties who share the key** |
| **Signature** | **a private key** | **integrity, origin, and non-repudiation** |

**Chapter 58 §58.3 covers the mechanisms. The point here is the requirement:** **integrity
against an adversary requires a secret**, and **no amount of clever unkeyed checksumming
substitutes for one.**

**And replay is the case people forget.** **A message that is genuine, correctly authenticated,
and sent again by an adversary** — **"transfer £5,000", replayed forty times.** **Nothing about
the message is forged.** **The defences are sequence numbers, timestamps and nonces**, and they
must be designed in; **authentication alone does not provide them.**

## Preventing, and why it is the hardest

**Stop the communication happening at all: flood the channel, cut the cable, exhaust the
receiver's resources.**

> **Availability is the hardest of the three, because a sufficiently large flood is
> indistinguishable from legitimate popularity.**

**Cryptography does not help here**, and this is worth stating plainly because it surprises
people: **encryption and authentication protect the first two properties and do nothing for the
third.** **A perfectly encrypted, perfectly authenticated service is trivially taken offline by
a large enough flood.**

**The defences are of a different kind:**

| Defence | Against |
|---|---|
| **Capacity** | **volumetric floods** — and it is an arms race you may not win alone |
| **Filtering upstream** | **the traffic must be dropped before your circuit**, not at your firewall |
| **Scrubbing services** | **volumetric attacks at scale** |
| **Rate limiting and resource management** | **resource-exhaustion attacks** (SYN floods, Chapter 37 §37.2) |
| **Anycast** | **spreading an attack across many locations** (Chapter 52 §52.4) |
| **Redundancy and diversity** | Chapter 56 |

**The "upstream" point is the one that determines the design:**

> **A 40 Gb/s attack against a 1 Gb/s circuit cannot be filtered by anything you own.** **The
> circuit is full before the traffic reaches your equipment**, and **the only useful control is
> at a point upstream with more capacity than the attack** — your provider, or a scrubbing
> service.

**And availability has causes that are not adversarial at all**, which is why Chapter 56 sits
where it does: **a cut cable, a failed power supply and a configuration error produce the same
outcome as an attack**, and the same controls address several of them.

## Ends and means

**The distinction that prevents muddled security architecture.**

> **Everything else you will hear — authentication, authorisation, non-repudiation,
> accountability — is a mechanism serving these three ends, not a fourth end.**

| Mechanism | Serves |
|---|---|
| **Authentication** | **integrity** (you cannot trust data whose origin you cannot establish) **and confidentiality** (you must know who you are encrypting to) |
| **Authorisation** | **confidentiality and integrity** (who may read, who may change) |
| **Non-repudiation** | **integrity**, extended over time and to third parties |
| **Accountability / logging** | **all three**, indirectly — by enabling detection and deterrence |
| **Segmentation** | **confidentiality and integrity**, by limiting reach |

**Why the distinction matters practically:** **"we need authentication" is not a security
requirement.** **"We need to know that this configuration change came from an authorised
engineer, and to prove it afterwards" is** — and **it identifies integrity and non-repudiation as
the ends, from which the mechanism follows.**

**Requirements stated as mechanisms produce solutions looking for problems.** **Requirements
stated as ends can be met by whichever mechanism is proportionate** (§57.3).

## Where the three conflict

**They are not independent, and the conflicts are real design decisions rather than
philosophical curiosities.**

| Conflict | Example |
|---|---|
| **Confidentiality vs availability** | **encrypted traffic cannot be inspected**, so security tooling loses visibility (Chapter 60 §60.3) |
| **Confidentiality vs availability** | **lose the key and the data is gone** — encryption is a reliable denial-of-service against yourself |
| **Integrity vs availability** | **fail-closed**: a device that cannot verify integrity stops forwarding |
| **Availability vs confidentiality** | **a redundant copy is another copy to protect** |
| **Availability vs integrity** | **restoring quickly from a backup that may contain the attacker's persistence** |

**The fail-open/fail-closed choice is the sharpest of these**, and it must be decided
deliberately:

> **When a security control fails, does traffic pass or stop?** **A firewall that fails open is
> available and insecure; one that fails closed is secure and unavailable.** **Neither is
> universally right**, and **a control whose failure mode nobody chose has one anyway.**

**Which one is correct depends on what the system does.** **An industrial safety network fails
open; a payment system fails closed**, and stating the choice on the record is the engineering
act.

## Ranking them for a given system

**The triad is not a set of equal priorities, and treating it as one produces bad designs.**

| System | Priority order |
|---|---|
| **Public website** | **A**, then I, then C |
| **Medical records** | **C**, then I, then A |
| **Industrial control** | **A and I**, far ahead of C |
| **Financial ledger** | **I**, then C, then A |
| **Emergency services dispatch** | **A**, overwhelmingly |
| **Network management plane** | **I**, then C, then A |

> **Industrial control is the instructive case.** **A process control network's confidentiality
> is frequently near-irrelevant and its availability is safety-critical** — **which inverts every
> instinct trained on IT systems**, and is why IT security practice transplanted into an
> operational technology environment causes damage.

**State the ranking per system, explicitly.** **It determines the fail-open/fail-closed choice,
the patching cadence, the acceptable downtime for security work, and what you do when the
three conflict.**

## What breaks here

**A CRC relied on for integrity against tampering.** **It detects accident only.** A keyed MAC
is required.

**An authenticated protocol vulnerable to replay.** **Authentication is not freshness.**
Sequence numbers, timestamps or nonces.

**A firewall filtering a volumetric attack.** **The circuit is already full.** The filtering must
be upstream.

**Encryption deployed and traffic analysis still revealing everything.** **Expected.** TLS
protects content, not metadata.

**A security control whose failure mode nobody chose.** **It has one anyway.** Decide it, test
it, document it.

**IT security practice applied to an industrial network, causing an outage.** **The triad's
ranking is inverted there**, and the transplant was the error.

**Data unavailable because the key was lost.** **Encryption is a denial-of-service against
yourself if key management is not designed.** Chapter 58 §58.4.

> **Network+ note.** Objective 4.1 covers the CIA triad directly. Over-learn: **confidentiality
> is protection from disclosure, integrity from unauthorised modification, availability is
> access when needed**; **encryption provides confidentiality, hashing supports integrity, and
> redundancy supports availability**; and **authentication, authorisation and accounting are
> mechanisms.** The ends-versus-mechanisms distinction is examined implicitly and is worth
> holding firmly.
