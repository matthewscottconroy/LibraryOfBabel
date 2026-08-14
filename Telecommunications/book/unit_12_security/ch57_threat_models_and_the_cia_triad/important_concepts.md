# Chapter 57 — Important Concepts

Threat modelling is asking what you are defending, from whom, and at what cost, in that
order *(intro)* — It is the difference between security engineering and buying appliances.

An exposed address is scanned within minutes and attacked within hours *(§57.1)* — Not an
exaggeration, and trivially demonstrated. Opportunists do not select targets; your address was
in the range.

Four controls defeat the overwhelming majority of attacks you will ever see *(§57.1)* —
Do not expose management interfaces. Do not use default credentials. Patch what is known to be
exploited. Do not expose what need not be exposed. The strongest control is absence, and
most organisations under-invest here while over-investing elsewhere.

Ransomware is a business with a business model, and step 4 is the network's step *(§57.1)* —
Access, foothold, escalate, **move laterally**, find the backups, exfiltrate, encrypt, demand.
Steps 1–3 are endpoint and identity problems; lateral movement is a network problem, and it is
where the outcome is decided. Segmentation makes it slow, noisy and incomplete, and every
hour it costs is an hour in which detection may occur.

"We have backups" is not an answer *(§57.1)* — Operators specifically hunt backup systems,
and a backup reachable with the compromised credentials is not a backup. Exfiltration before
encryption also removed the "we restored and did not pay" defence.

The negligent insider is not an adversary and is the largest source of incidents *(§57.1)* —
Defending against them is mostly design: make the dangerous thing hard to do by accident.

"We are too small to be interesting" is the reasoning that makes a small organisation a useful
route *(§57.1)* — Supply chain compromise attacks the small supplier to reach the large
customer.

You are not likely to catch the intrusion *(§57.1)* — The median time to detection is weeks
or months and a significant share of breaches are reported by an outsider. You are trying to
catch what the intruder does afterwards — which is why baselines, lateral-movement detection
and egress monitoring matter more than perimeter alerting.

Three verbs, and the list is exhaustive *(§57.2)* — **Listen, alter, prevent.** There is no
fourth thing you can do to a channel. That completeness is why the triad has survived, and it
is a much better reason to believe it than that it appears on a syllabus.

Listening is undetectable, so only encryption works *(§57.2)* — A tap, a passive receiver,
a mirror port — none changes what the endpoints observe, so no protocol mechanism can reveal
them. You cannot stop someone receiving radio waves or splicing a fibre; you can only ensure
what they receive is useless.

Encryption does not hide that the communication occurred *(§57.2)* — Who talked to whom,
how much, and when. Knowing a device contacted a medical, legal or recruitment service at a
particular time with a particular volume is frequently sufficient, and TLS provides none of
it.

A checksum detects accident, not intent *(§57.2)* — Anyone who alters the data can
recompute it, using a published algorithm requiring no secret. Integrity against an
adversary requires a secret, and no amount of clever unkeyed checksumming substitutes.

Authentication is not freshness *(§57.2)* — A genuine, correctly authenticated message
replayed forty times is not forged. Sequence numbers, timestamps and nonces must be designed
in.

Cryptography does nothing for availability *(§57.2)* — A perfectly encrypted, perfectly
authenticated service is trivially taken offline by a large enough flood, and a sufficiently
large flood is indistinguishable from legitimate popularity.

A 40 Gb/s attack against a 1 Gb/s circuit cannot be filtered by anything you own *(§57.2)* —
The circuit is full before the traffic reaches your equipment. The only useful control is
upstream, at a point with more capacity than the attack.

Everything else is a mechanism, not a fourth end *(§57.2)* — Authentication, authorisation,
non-repudiation, logging. "We need authentication" is not a requirement; "we need to know this
change came from an authorised engineer and to prove it afterwards" is — and the mechanism
follows from the end. Requirements stated as mechanisms produce solutions looking for
problems.

When a control fails, does traffic pass or stop? *(§57.2)* — Neither answer is universally
right, and a control whose failure mode nobody chose has one anyway. An industrial safety
network fails open; a payment system fails closed. Stating the choice on the record is the
engineering act.

The triad is not a set of equal priorities *(§57.2)* — A process control network's
confidentiality is frequently near-irrelevant and its availability is safety-critical, which
inverts every instinct trained on IT systems — and is why IT security practice transplanted
into an operational technology environment causes damage.

The management plane is the asset most often under-valued *(§57.3)* — Compromise it and
you have compromised everything it manages. So is the configuration repository (Chapter 55
§55.4), and so is the documentation, which is a map for an attacker as well as for you.

The arithmetic's value is that it forces the disagreement to be specific *(§57.3)* —
"You think once in twenty years and I think once in five" is productive; "we should take
ransomware seriously" is not. ALE = SLE × ARO, and rare-and-catastrophic can correctly rank
below frequent-and-moderate.

ALE handles the tail badly *(§57.3)* — A 2% annual chance of ending the organisation has
an ALE that understates it, because the organisation cannot average over twenty years — it only
gets one. Use the arithmetic to rank, then override deliberately and say you are doing so.

Avoidance is the cheapest control and is asked about last *(§57.3)* — The service nobody
uses, the data nobody needs, the port nobody opened deliberately. "Do we need this at all?"
should be the first question.

Insurance transfers money, not outages *(§57.3)* — Not the regulatory obligation, the
reputational damage or the work of recovery. And insurers now require MFA, segmentation and
offline backups as a condition of cover, which has driven adoption more than any technical
argument.

Acceptance is valid when explicit; the failure is accepting silently *(§57.3)* — A
documented acceptance has four parts: the risk, the reason, the owner and the review date.
Without the last two it is not an acceptance; it is a note.

Costing a control against one risk understates it *(§57.3)* — The commonest error in these
calculations. Segmentation also reduces insider impact, limits a misconfiguration's blast
radius, and satisfies compliance.

State risks as scenarios, not as missing controls *(§57.3)* — "Lack of segmentation" is a
missing control; "an attacker who compromises a laptop reaches the finance server" is a risk.
Registers full of missing controls become shopping lists, and the scenario form permits the
question "is there another way to prevent this outcome?", which is where the cheaper answer
usually is.

Every Layer 2 protocol in this book authenticates nothing *(§57.4)* — ARP, DHCP, STP and
the discovery protocols all believe whatever they are told, and every control is a bolt-on
added later. Which is why Layer 2 attacks are effective and under-defended: once an attacker
has segment access, the protocols offer no resistance of their own.

Physical access to a device is administrative access to it *(§57.4)* — Every vendor
publishes a password recovery procedure requiring only physical access and a reboot. An
unlocked comms room with a spare port is a complete bypass of every control in Chapters 58
through 62, and network engineers routinely treat it as facilities' problem.

BCP 38 is a solved problem that is not solved *(§57.4)* — Source address filtering
eliminates spoofing and therefore reflection attacks. Specified in 2000, still not universally
deployed, because the network that deploys it protects everyone except itself. The
Internet's remaining security problems are overwhelmingly the ones where the cost falls on a
different party from the benefit — RPKI and Chapter 48's governance have the same shape.

Any protocol that does work before authenticating has the SYN flood's shape *(§57.4)* —
The server allocates memory in response to an unauthenticated packet. SYN cookies' answer
— encode the state in the sequence number and hold none — is the general remedy.

DNS is the single most attractive target in the stack *(§57.4)* — Controlling name
resolution controls where traffic goes, without touching routing.

The human row decides most incidents *(§57.4)* — The overwhelming majority of successful
compromises begin with a person being persuaded, and every technical control in this unit
protects a system whose most reliable entry point is an email. Assume the human control
fails and design for containment. And make the safe path the easy path — a password policy
that forces reuse, or a VPN slow enough that people work around it, produces the behaviour it
was meant to prevent.

The service desk MFA reset is the current weak point *(§57.4)* — An attacker who persuades
a helpdesk to reset a factor has defeated the factor, and the fix is a process rather than a
technology.

Enumerate the surface, deploy the control, then instrument the control *(§57.4)* — The
third step is the one that is skipped, and it is why compromises are detected by third
parties. A control with no detection is a control you are trusting, and most of the attack
surface has a detection signature: MAC table churn, ARP anomalies, unexpected DHCP offers, BGP
announcement changes, SYN backlog depth, DNS query volume.
