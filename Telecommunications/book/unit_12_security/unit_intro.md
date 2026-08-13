# Unit XII — Securing a Network

Every chapter up to this point has assumed that everyone on the network is behaving
in good faith.

ARP (Chapter 18) accepts any reply from anyone, and caches it. DHCP (Chapter 40)
accepts an address from whichever server answers first. Spanning tree (Chapter 19)
believes any switch claiming a low bridge ID. BGP (Chapter 32) believes an autonomous
system that says it can reach a prefix. DNS (Chapter 39) originally accepted any
response that looked plausible. TCP (Chapter 37) accepts any segment with a sequence
number in the window.

None of those assumptions is safe, and this unit rebuilds each of them.

It is worth being clear about why the assumption was made, because "they should have
thought about security" is both true and useless as history. The protocols in this
book were designed between 1973 and 1998, for a network connecting research
institutions and government facilities whose participants were known to one another,
frequently by name, and where the number of people with the equipment and expertise to
attack anything was in the low thousands. Adding cryptography would have been
computationally prohibitive on the hardware of the era — a 1985 workstation could not
have encrypted a 10 Mb/s link if it did nothing else — and it would have solved a
problem that did not yet exist.

They were not careless. They were solving the problem in front of them, and the
environment changed underneath the result. That is worth remembering, because you will
build systems whose environment also changes.

## Deriving security rather than asserting it

The unit's method is the book's method. Rather than presenting the CIA triad as a
definition to be memorised, Chapter 57 asks a question:

> **What can an adversary actually do to a communication system?**

Looking at Shannon's diagram from Chapter 1 §1.2, there are exactly three
interventions available. An adversary can **listen** to the channel. An adversary can
**alter** what is on it. An adversary can **prevent** it from carrying anything.

From those three verbs, the three properties fall out:

- Defeating listening is **confidentiality**.
- Defeating alteration is **integrity**.
- Defeating prevention is **availability**.

That is the CIA triad, derived rather than recited, and derived in a way that also
tells you it is *complete* — there is no fourth verb, because there is nothing else
you can do to a channel. (Authentication, non-repudiation and the rest are mechanisms
serving these ends, not additional ends.)

## What the unit contains

**Chapter 57 — Threat Models and the CIA Triad.** The derivation above; who attacks
networks and what they want; risk and proportionate defence; and an enumeration of the
attack surface layer by layer, which doubles as a review of Units II through X.

**Chapter 58 — Cryptography for Network Engineers.** Not a cryptography course — you
will implement none of it — but the working knowledge required to deploy it correctly:
symmetric and asymmetric ciphers, the key distribution problem and Diffie–Hellman's
solution, hashes and MACs and signatures, and certificates, PKI and the TLS 1.3
handshake in full.

**Chapter 59 — Authentication, Authorization, Accounting.** Identity, factors,
802.1X and RADIUS, authorization models and least privilege, and zero trust — treated
as an architecture with a real argument behind it rather than as a marketing term.

**Chapter 60 — Firewalls, ACLs, and Segmentation.** The ACL as a match-action list
and the implicit deny; stateful inspection and what statefulness costs; NGFW, IDS/IPS
and TLS inspection's genuine tradeoff; and segmentation from DMZs to
microsegmentation.

**Chapter 61 — VPNs and Secure Remote Access.** The tunnel idea; IPsec in detail;
TLS VPNs and WireGuard; split tunnelling and the post-2020 remote access reality.

**Chapter 62 — Attacks on the Stack.** A systematic walk back down through the layers,
attacking each mechanism we built, with the mitigation for each. This chapter is the
unit's payoff and it is where the earlier units are stress-tested.

## Three principles to carry

**Defence in depth.** Every control fails eventually. The question is not whether the
firewall will be bypassed but what happens next, and the answer should not be
"everything." Layered controls mean a single failure is a setback rather than a
breach.

**Least privilege.** Every entity gets the minimum access required for its function.
This is easy to state, tedious to implement, and the single most effective limiter of
blast radius. Most catastrophic breaches involve an account or a device with far more
access than its job required.

**Security is a property of a system, not a product.** There is no appliance that
makes a network secure. There are appliances that address specific threats within a
design that considered them, and there are appliances deployed to satisfy an auditor
that address nothing. The difference is whether someone did the analysis of
Chapter 57 first.

## And a caution about proportion

This unit will describe a large number of attacks. It is easy to come away believing
that everything is broken and nothing can be trusted, which is both paralysing and
inaccurate.

The correct disposition is proportionate. Most networks are not targeted by
sophisticated adversaries; most compromises are opportunistic, exploit known
vulnerabilities in unpatched systems, and would have been prevented by basic hygiene —
patching, segmentation, least privilege, multi-factor authentication, and monitoring
that someone reads. **Do the ordinary things well before doing the exotic things at
all.** An organisation with unpatched edge devices and a flat network does not need a
better firewall; it needs to do the work it already knows about.
