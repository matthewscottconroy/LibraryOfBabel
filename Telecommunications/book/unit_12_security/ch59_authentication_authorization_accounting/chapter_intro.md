# Chapter 59 — Authentication, Authorization, Accounting

Three words that are used interchangeably in casual speech and mean entirely different
things, and the confusion between the first two causes real design errors.

**Authentication** answers *who are you?* It establishes identity.

**Authorization** answers *what are you allowed to do?* It grants or denies specific
access to an already-identified party.

**Accounting** answers *what did you do?* It records activity for billing, capacity
planning, audit and incident investigation.

They are independent. A system can authenticate perfectly and authorise terribly —
which is precisely the situation in a great many organisations, where everyone is
correctly identified and then given far more access than their role requires. The
2013 Target breach began with credentials belonging to a refrigeration contractor,
correctly authenticated, that turned out to reach the payment network. Authentication
worked exactly as designed. Authorization was the failure.

## Factors, and why the count matters

§59.1 covers the standard taxonomy:

**Something you know** — password, PIN, passphrase. Cheap, and subject to guessing,
reuse, phishing and disclosure at scale via breaches of other systems.

**Something you have** — a hardware token, a phone, a smartcard, a certificate.
Requires physical possession, and is why theft is the attack rather than guessing.

**Something you are** — fingerprint, face, iris. Convenient, and carrying a permanent
liability: you cannot revoke a fingerprint. A compromised password is changed in
seconds; a compromised biometric template is compromised for life. This is a genuine
argument for treating biometrics as a convenience layer over a revocable credential
rather than as a credential in themselves.

**Multi-factor** requires two or more *different* kinds. A password plus a security
question is one factor twice — both are things you know, both are disclosed by the
same breach.

§59.1 also grades the second factors honestly, because they are not equivalent. **SMS
codes** are better than nothing and are defeated by SIM swapping and by SS7
interception (Chapter 12 §12.3's security assumption, still being exploited).
**Authenticator apps** are substantially better. **Hardware security keys** using
FIDO2/WebAuthn are the only widely deployed factor that resists phishing structurally
— the key checks the origin, so a user cannot be tricked into authenticating to a
lookalike site, because the key simply will not respond.

## 802.1X: authentication before the network

The mechanism that makes port-based network access control possible, and the reason
a device cannot simply be plugged into a wall socket to gain access.

Three roles: the **supplicant** (the device), the **authenticator** (the switch or
access point), and the **authentication server** (typically RADIUS, typically backed
by a directory). Until authentication succeeds, the switch port forwards nothing but
EAP frames — the device has link, and no network.

**EAP** is the extensible framework carrying the actual authentication, and the method
matters: EAP-TLS uses certificates on both sides and is the strongest; PEAP and
EAP-TTLS tunnel a password-based method inside TLS; EAP-MD5 is broken and should never
appear. §59.2 covers the deployment realities, including the two that consume most of
the effort: **certificate distribution** to every device, and what to do about
devices that cannot do 802.1X — printers, cameras, building controllers — for which
MAC authentication bypass is the usual answer, with the honest acknowledgement that
MAC addresses are trivially spoofed and MAB is therefore a inventory control rather
than a security control.

**RADIUS** (RFC 2865) is the protocol between authenticator and server, and it carries
its own history: it encrypts only the password field, uses MD5, and runs over UDP. It
is still ubiquitous. **TACACS+** encrypts the whole payload and separates
authentication from authorisation, which is why it is preferred for administrative
access to devices, where per-command authorisation is valuable.

## Authorization models

§59.3 covers the models and, more usefully, the discipline.

**Role-based access control** assigns permissions to roles and roles to users. The
dominant model, because it scales: adding a person means assigning a role, not
enumerating permissions.

**Attribute-based access control** decides from attributes of user, resource,
environment and action — permitting policies like "finance staff, on managed devices,
during business hours, from the corporate network." More expressive, more complex, and
the direction modern systems are moving.

**Least privilege** is the principle underneath both, and it is easy to state and
tedious to implement: every entity gets the minimum access required for its function,
and no more. The reason it matters is **blast radius**. An attacker who compromises a
credential gets what that credential could do. If the answer is "read three specific
file shares," the incident is contained. If the answer is "anything, anywhere," it is
a catastrophe. Most catastrophic breaches are the second case, and the difference was
decided years earlier by someone granting broad access because it was easier.

**Privilege creep** is the practical enemy: people accumulate access as they change
roles and rarely lose the old permissions. The countermeasure is periodic access
review, which is unglamorous and works.

## Zero trust

§59.4 treats it as an architecture with a real argument, because it is one — despite
having been thoroughly appropriated by marketing.

The traditional model is a perimeter: a trusted inside, an untrusted outside, and
controls at the boundary. It fails for reasons that are now structural rather than
theoretical. The applications moved to the cloud, so they are not inside. The users
moved out of the office, so they are not inside either. And the model's core
assumption — that inside is safe — means that an attacker who gets in faces no further
obstacles, which is exactly the lateral movement that turns a foothold into a breach.

Zero trust's premise: there is no inside. Every request is authenticated and
authorised on its own merits, regardless of origin. Network location confers no trust.
The practical components are strong identity, device posture assessment,
per-application access rather than network access, continuous verification rather than
one-time authentication, and microsegmentation (Chapter 60 §60.4).

§59.4 also states the honest difficulties: it is a multi-year programme rather than a
purchase; legacy applications frequently cannot participate; and the identity provider
becomes a single point of failure of the most consequential kind, since compromising it
compromises everything. Those are not reasons to reject it. They are reasons to plan
it properly rather than buy it.

## By the end you will be able to

- Distinguish authentication, authorization and accounting, and identify which failed
  in a described incident.
- Evaluate a multi-factor scheme and rank second factors by phishing resistance.
- Explain 802.1X's three roles and trace an authentication.
- Explain why MAC authentication bypass is not a security control.
- Compare RADIUS and TACACS+ and choose appropriately.
- Apply least privilege to a described scenario and estimate the blast radius
  reduction.
- State zero trust's premise, its components, and its three main difficulties.
