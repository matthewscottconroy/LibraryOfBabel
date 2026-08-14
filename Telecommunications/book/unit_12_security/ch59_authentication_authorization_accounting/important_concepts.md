# Chapter 59 — Important Concepts

Authentication, authorization and accounting are independent *(§59.1)* — A system can
authenticate perfectly and authorise terribly, which is the situation in a great many
organisations. The 2013 Target breach began with a refrigeration contractor's credentials,
correctly authenticated, that reached the payment network. Authentication worked exactly as
designed.

A compromised password is changed in seconds; a compromised biometric is compromised for
life *(§59.1)* — Which argues for treating biometrics as a convenience layer over a
revocable credential, which is what a phone's fingerprint reader does: it unlocks a key in
secure hardware and is not transmitted anywhere. A biometric template sent across a network
as an authentication token is a serious design error.

A password plus a security question is one factor twice *(§59.1)* — Both are things you
know, both are disclosed by the same breach, and the answers are frequently easier to find
than the password.

FIDO2 resists phishing structurally, because the user is not the one deciding *(§59.1)* —
The key signs a challenge including the origin, so a user on `exarnple.com` cannot produce a
signature valid for `example.com`. Every other factor can be relayed by a real-time proxy,
which is now a commodity technique.

"SMS is worthless" is advice that causes organisations to deploy nothing *(§59.1)* — A very
large fraction of compromises are credential stuffing against accounts with no second factor at
all, and SMS defeats all of them. The correct message is "SMS if that is what you can
deploy, and plan to move."

Forced expiry produces worse passwords *(§59.1)* — `Summer2026!` becomes `Autumn2026!`, and
it does nothing against the actual threat, since a phished password is used within minutes.
Composition rules produce `Password1!` at enormous scale. Expire on evidence, not on a
schedule — length, a breach-list check, and rate limiting achieve far more.

SSH keys never expire, and nobody has an inventory *(§59.1)* — An authorised key grants
access indefinitely to whoever holds the private half, and they accumulate for a decade.
Certificate-based SSH or a brokered bastion converts an unbounded standing grant into a
bounded one.

Federation makes the identity provider the most critical system you operate *(§59.1)* —
If it is unavailable nobody can log in to anything; if it is compromised, everything is.
Which means its availability target exceeds anything it serves, it needs a break-glass
path that does not depend on it, and its own administrative access must be protected
differently, because it cannot protect itself.

The 802.1X authenticator decides nothing *(§59.2)* — The switch does not know what a
certificate is. It relays EAP and applies what the server returns, so policy lives in one
place and switches need no configuration when it changes.

Before authentication the port forwards only EAP frames *(§59.2)* — Link light and nothing
else. No DHCP, no ARP, no traffic. EAPOL works before addressing does.

PEAP protects the password only if the client validates the server certificate — and it
usually does not *(§59.2)* — The single most common 802.1X misconfiguration, and its
consequence is an attacker with a laptop in the car park collecting credentials. The correct
client configuration names the CA, names the expected server, and refuses to prompt — because
a prompt saying "not trusted, continue?" will be accepted.

EAP-TLS removes the problem entirely *(§59.2)* — No password to steal, mutual
authentication inherent, revocable. Its cost is a PKI and device enrolment, which is why it is
not universal.

RADIUS encrypts only the password field, with MD5 *(§59.2)* — Usernames, attributes and
accounting records cross in the clear, and the shared secret is per-client, usually identical
across the estate, and in every configuration backup. RadSec fixes the transport and
deployment is patchy.

The attributes are what make 802.1X useful, not the yes/no *(§59.2)* — VLAN, ACL, session
timeout and vendor policy are decided per authentication, from the directory. A contractor's
laptop and a finance workstation plug into identical ports and land in different networks with
different rules — dynamic segmentation, and the strongest reason to deploy it.

MAB is not authentication; it is identification of a claim anyone can make *(§59.2)* — Its
value is that it places the device in a restricted VLAN with an ACL permitting only what that
device class needs, so spoofing a printer's MAC gets you a printer's access. MAB plus tight
authorisation is defensible; MAB into the general network is theatre.

Design the failure mode before deploying *(§59.2)* — What happens when RADIUS is
unreachable? Fail-closed, a critical VLAN, or fail-open. And already-authenticated sessions
must survive a server outage, or a switch re-authenticating every 3,600 seconds will drop the
estate in a wave.

Deploy in monitor mode first, for weeks *(§59.2)* — It finds the devices you did not know
about, which is always more than expected, and it converts a disruptive project into a boring
one.

RADIUS for network access, TACACS+ for device administration *(§59.2)* — TACACS+ separates
authorization from authentication, which is what makes per-command authorisation possible: a
junior engineer may run `show` and not `configure`, with every attempt authorised individually
and logged. They are not competitors; a mature network runs both.

RBAC's failure mode is role explosion *(§59.3)* — Each exception becomes a new role, and
after five years there are more roles than people and nobody can say what any grants. Chapter
55 §55.1's accumulation argument, in an access control system — monotonic, invisible,
compounding. ABAC solves it and makes "who can access this?" a question requiring evaluation
rather than a lookup.

The failure modes of least privilege are asymmetric *(§59.3)* — Too little privilege
produces an immediate, visible, attributable complaint. Too much produces nothing at all, until
a breach. The incentive points at over-granting, permanently. And privileges accumulate
with tenure, because joining a role adds permissions and leaving one rarely removes them.

Just-in-time elevation is the single highest-value change available *(§59.3)* — Nobody
holds administrative rights standing; they request them, for a reason, for a bounded period.
A compromised account then holds no privileges by default.

An access review whose default is "keep" achieves nothing *(§59.3)* — Unreviewed grants
must be removed, and most reviews are configured the other way.

The role permitted to change AAA configuration should be the smallest *(§59.3)* — An
engineer who can reconfigure authentication can grant themselves anything and remove the
evidence. Separate it, and alarm on changes to it.

A change log that says "admin" is not a log *(§59.3)* — The shared `enable` password is the
specific failure to eliminate. Individual accounts, always.

Log denials as well as permissions *(§59.3)* — A denial is more interesting than a
permission — it is either a misconfiguration or an attempt. And send the log where the
subject cannot alter it: an administrator who can delete the record of their own actions has
no accountability at all.

Zero trust in one sentence: being on the network confers no privilege *(§59.4)* — Everything
else follows from taking that seriously.

Device posture is what distinguishes zero trust from good authentication *(§59.4)* — A
correct credential on an unmanaged, unpatched personal machine is not the same as the same
credential on a managed, current, encrypted one, and a system that cannot tell the difference
is doing authentication. Which makes endpoint management a network dependency, and an
unmanaged device cannot be assessed at all.

In a broker-mediated design the resource has no inbound listener *(§59.4)* — The
application connects outbound to the broker. Nothing to scan, nothing to exploit
pre-authentication, and no network path that does not pass through the policy decision point —
which turns Chapter 57 §57.1's opportunistic scanning into a non-event.

Continuous verification is Saltzer and Schroeder's complete mediation *(§59.4)* — Check every
access, every time, rather than issuing a capability once. Its cost is that the policy decision
point is in the path of everything, so its availability and latency become everyone's —
Chapter 56 §56.1's series arithmetic applied to a security control.

Zero trust does not remove the need for segmentation; it changes its purpose *(§59.4)* — A
flat network with per-application access control still lets a compromised device scan and attack
everything at the network layer. The network provides what identity cannot: containment when
identity fails — and every identity system will eventually authenticate an attacker.

Zero trust is a direction, not a project with an end *(§59.4)* — No organisation is "zero
trust". The useful question is "which of our access decisions still rest on network
location?", which produces a work list rather than a certification. And legacy applications
do not participate — every real deployment has a residual perimeter around them, and
pretending otherwise produces plans that fail.

It replaces one trusted thing with another, and the second is better *(§59.4)* — Not the
absence of trust; trust placed somewhere defensible — verifiable per request, revocable
instantly, logged completely.
