# Chapter 57 — Threat Models and the CIA Triad

Before defending anything it is worth knowing what you are defending, from whom, and
at what cost — and the discipline of asking those three questions in that order is
called **threat modelling**. It is the difference between security engineering and
buying appliances.

## The derivation

Return to Shannon's diagram from Chapter 1 §1.2: source, transmitter, channel, noise,
receiver, destination. Now place an adversary somewhere on it and ask what they can
actually do.

**They can listen.** Copy what crosses the channel without altering it. The
communication proceeds normally; the parties have no way to detect it. Defeating this
is **confidentiality**, and the only general defence is encryption — because you
cannot prevent someone from receiving radio waves or splicing a fibre, you can only
ensure that what they receive is useless.

**They can alter.** Modify data in transit, inject data that was never sent, or
replay data that was sent earlier. Defeating this is **integrity**, and the defence is
cryptographic authentication — a MAC or a signature — because a checksum (Chapter 15
§15.4) detects accident and not intent, since anyone who alters the data can recompute
the checksum.

**They can prevent.** Stop the communication happening at all: flood the channel, cut
the cable, exhaust the receiver's resources. Defeating this is **availability**, and it
is the hardest of the three, because a sufficiently large flood is indistinguishable
from legitimate popularity and the defence is largely capacity and filtering rather
than cryptography.

Three verbs, three properties. And note that the list is **exhaustive** — there is no
fourth thing you can do to a channel. That completeness is why the triad has survived
as a framework, and it is a much better reason to believe it than the fact that it
appears on a certification syllabus.

Everything else you will hear — authentication, authorisation, non-repudiation,
accountability — is a *mechanism serving* these three ends, not a fourth end.
Authentication serves integrity (you cannot trust data whose origin you cannot
establish) and confidentiality (you must know who you are encrypting to). Keeping the
distinction between ends and means clear prevents a great deal of muddled security
architecture.

## Who is attacking, and what for

§57.1 covers the actors, because the appropriate defence depends heavily on who you
are defending against and treating all adversaries as equivalent leads to spending in
the wrong places.

**Opportunists** scanning the entire Internet for known vulnerabilities. They do not
know or care who you are. They are the overwhelming majority of attempts, and they are
defeated by patching, by not exposing management interfaces, and by not using default
credentials. Most organisations' actual threat is this one, and most organisations
under-invest in defeating it while over-investing elsewhere.

**Criminal enterprises**, principally ransomware, which is a business with a business
model: gain access, escalate, move laterally, locate and destroy backups, encrypt, and
demand payment. The network-level countermeasure is segmentation, because lateral
movement is the step where a foothold becomes a catastrophe, and it is the step
segmentation makes expensive.

**Insiders**, malicious or — far more often — negligent. Least privilege and logging.

**Targeted adversaries**, including state actors, who have time, money and specific
objectives. If you are genuinely a target, this book is a starting point and not a
sufficient one.

**Hacktivists and vandals**, mostly conducting denial of service, defeated by capacity
and scrubbing rather than by cleverness.

## Proportion

§57.3 covers risk, and its purpose is to prevent both of the standard failures:
spending nothing because it seems abstract, and spending everything because the
attacks sound frightening.

The usual formulation is risk = likelihood × impact, and its value is not arithmetic
precision — the numbers are estimates and everyone knows it — but the discipline of
being *explicit*. Writing down "we assess this as unlikely and catastrophic" forces a
conversation that "we should really do something about that" does not.

The four responses to a risk: **mitigate** (reduce it), **transfer** (insure it),
**accept** (document that you chose to live with it), and **avoid** (stop doing the
risky thing). All four are legitimate. Acceptance in particular is a valid engineering
decision when it is *explicit and documented* — the failure is not accepting a risk,
it is accepting it silently and later claiming nobody knew.

The proportionality test that §57.3 offers: the cost of the control should not
exceed the expected loss it prevents. A £50,000 control for a risk with an expected
annual loss of £2,000 is not prudence but innumeracy, and defending that position to a
finance director is a skill worth developing, because it is how security budgets are
actually won.

## The attack surface, layer by layer

§57.4 walks the stack we built and enumerates what is exposed at each level. It
doubles as a review of Units II through X, and it is the map for Chapter 62.

| Layer | Exposure | Chapter |
|---|---|---|
| Physical | Cable tapping, unlocked rooms, unattended ports, RF eavesdropping | 10, 42 |
| Data link | MAC flooding, ARP spoofing, VLAN hopping, rogue DHCP, STP attacks | 17–20 |
| Network | IP spoofing, route injection, BGP hijack, ICMP abuse, fragmentation | 24–34 |
| Transport | SYN flood, session hijack, port scanning | 35–38 |
| Application | Everything — injection, credential attacks, protocol abuse, DNS poisoning | 39–41 |
| Human | Phishing, pretexting, physical social engineering | — |

Two observations from that table.

The last row is not a joke and is not an afterthought. The overwhelming majority of
successful compromises begin with a human being persuaded to do something, not with
a protocol weakness. Every technical control in this unit protects a system whose most
reliable entry point is an email.

And the physical row is the one most often ignored by network engineers, on the
assumption that it is facilities' problem. An unlocked comms room with a spare switch
port is a complete bypass of every control in Chapters 58 through 62.

## By the end you will be able to

- Derive the CIA triad from the three things an adversary can do, and explain why the
  list is complete.
- Distinguish security *ends* from security *mechanisms*.
- Characterise the threat actors relevant to a described organisation.
- Perform a proportionate risk assessment and choose among the four responses.
- Enumerate the attack surface of a described network by layer.
- Explain why segmentation is the specific countermeasure to ransomware's business
  model.
