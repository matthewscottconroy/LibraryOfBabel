# 59.1 Identity and Credentials

Three words used interchangeably in casual speech that mean entirely different things, and
the confusion between the first two causes real design errors.

| | Answers | |
|---|---|---|
| **Authentication** | **who are you?** | establishes identity |
| **Authorization** | **what are you allowed to do?** | grants access to an identified party |
| **Accounting** | **what did you do?** | records activity |

> **They are independent.** **A system can authenticate perfectly and authorise terribly** —
> which is precisely the situation in a great many organisations, where everyone is correctly
> identified and then given far more access than their role requires.

The 2013 Target breach is the canonical illustration. It began with credentials belonging
to a refrigeration contractor, correctly authenticated, that turned out to reach the payment
network. Authentication worked exactly as designed. Authorization was the failure, and
§59.3 is about it.

## The three factors

| Factor | Examples | The problem with it |
|---|---|---|
| **Something you know** | password, PIN, passphrase | **guessing, reuse, phishing, and disclosure at scale via other systems' breaches** |
| **Something you have** | token, phone, smartcard, certificate | **theft — and the recovery process** |
| **Something you are** | fingerprint, face, iris | **you cannot revoke a fingerprint** |

The biometric point deserves its own statement, because it is under-appreciated:

> A compromised password is changed in seconds. A compromised biometric template is
> compromised for life.

Which is a genuine argument for treating biometrics as a convenience layer over a revocable
credential rather than as a credential in themselves — which is exactly what a phone's
fingerprint reader is doing. The fingerprint unlocks a key held in secure hardware; it is
not transmitted anywhere and is not the credential. A system that sends a biometric template
across a network as an authentication token has made a serious design error.

And there are two additional factors that are used and rarely named:

**Somewhere you are** — location, network, device posture. **Not sufficient alone** (addresses
are spoofable, VPNs move you) and useful as a signal in the risk-based schemes below.

**Something you do** — typing rhythm, gait, behavioural patterns. Continuous rather than
point-in-time, which is its interesting property, and it is largely a fraud-detection tool
rather than an authentication one.

## Multi-factor, and the common mistake

> **Multi-factor requires two or more *different kinds*.**

A password plus a security question is one factor twice. Both are things you know, both
are disclosed by the same breach, and the answers to security questions are frequently
easier to find than the password — a mother's maiden name and a first school are public
information for most people.

**And the second factors are not equivalent.** **Grading them honestly:**

| Factor | Resists | Defeated by |
|---|---|---|
| **SMS code** | **casual credential stuffing** | **SIM swapping; SS7 interception; real-time phishing** |
| **Email code** | the same | **and it is only as strong as the email account** |
| **TOTP app** (authenticator) | **most credential attacks** | **real-time phishing — the user types the code into the fake site** |
| **Push notification** | | **push fatigue — repeated prompts until the user approves one** |
| **Number matching push** | **push fatigue** | real-time phishing, with effort |
| **FIDO2 / WebAuthn hardware key** | **phishing, structurally** | **theft plus the device PIN; and account recovery** |
| **Passkeys** (FIDO2 in software) | **phishing, structurally** | the platform account they sync with |

The FIDO2 property is worth understanding precisely, because it is not merely "stronger":

> The key signs a challenge that includes the origin — the actual domain the browser is
> connected to. A user on `exarnple.com` cannot produce a signature valid for
> `example.com`, because **the key will not produce one.** The user cannot be tricked,
> because the user is not the one making the decision.

This is a structural defence rather than a behavioural one, and it is the only widely
deployed second factor that has it. Every other factor can be relayed by an attacker running a
real-time proxy between the user and the real site — which is now a commodity technique with
off-the-shelf tooling.

**SMS deserves an honest word.** It is much better than nothing. A very large fraction of
account compromises are credential stuffing against accounts with no second factor at all, and
SMS defeats all of them. The correct message is "SMS if that is what you can deploy, and plan
to move", not "SMS is worthless", which causes organisations to deploy nothing.

## Passwords, and what actually helps

The guidance changed substantially and much deployed policy has not caught up.

| Old guidance | **Current (NIST SP 800-63B and equivalents)** |
|---|---|
| **Expire every 90 days** | **do not expire without cause** |
| **Require complexity classes** | **do not impose composition rules** |
| Minimum 8 | **minimum 8, and permit at least 64** |
| Prohibit paste | **permit paste — password managers need it** |
| **Hints and security questions** | **do not use them** |
| — | **check against known-breached password lists** |
| — | **rate-limit and lock out on repeated failure** |

The rationale for the two reversals is worth stating, because they are counter-intuitive and
still argued about.

**Forced expiry produces worse passwords.** Users respond predictably: `Summer2026!` becomes
`Autumn2026!`. The measurable effect is that expiry increases the use of predictable
transformations, and it does nothing against the actual threats — a phished password is
used within minutes, not within 90 days.

**Composition rules produce predictable passwords.** "One uppercase, one digit, one symbol"
yields `Password1!` at enormous scale, because humans satisfy the rule in the same way.
Length and a breach-list check achieve far more.

> **Expire on evidence, not on a schedule.** A password known to be breached, or an account
> showing signs of compromise, should be reset immediately. A password nobody has any reason
> to doubt should be left alone, because changing it makes it worse.

## Credentials that are not passwords

A network engineer deals with more machine credentials than human ones, and they have
different failure modes.

| Credential | Where | Failure mode |
|---|---|---|
| **Certificates** | 802.1X, VPN, mutual TLS | **expiry** (Chapter 58 §58.4), and **private key protection** |
| **SSH keys** | administration, automation | **they never expire, and nobody knows how many exist** |
| **API tokens** | automation, cloud | **long-lived, over-privileged, and in source code** |
| **Shared secrets** | RADIUS, TACACS+, SNMPv3 | **shared, so rotation is coordinated and therefore never happens** |
| **Service accounts** | applications | **passwords that cannot be changed without an outage** |

SSH keys deserve emphasis because the problem is invisible:

> An authorised SSH key grants access indefinitely, to whoever holds the private half.
> There is no expiry, no central record, and in most organisations no inventory — keys
> accumulate in `authorized_keys` files for a decade and nobody knows whose they are.

**The answers are known and under-used:** **certificate-based SSH authentication** (short-lived
certificates issued by a CA, so access expires by construction), or a bastion with brokered
access. Both convert an unbounded standing grant into a bounded one.

And secrets in source control is the modern version of the same problem — Chapter 55 §55.4's
warning. Scanning repositories for credentials finds them reliably, which is why attackers
do it.

## Identity providers and federation

The consolidation that has happened and its consequences.

Instead of every application holding credentials, one identity provider authenticates and
asserts identity to the others.

| Protocol | Used for |
|---|---|
| **SAML 2.0** | **enterprise web single sign-on**; XML; entrenched |
| **OAuth 2.0** | **authorisation delegation** — "let this app read my calendar" |
| **OpenID Connect** | **authentication on top of OAuth 2.0**; the modern default |
| **Kerberos** | **Windows domains and internal services**; tickets, not passwords, on the wire |
| **LDAP** | **the directory itself** (Chapter 41 §41.4) — frequently the store behind the others |

> OAuth 2.0 is an authorisation protocol and is routinely used for authentication, which it
> was not designed for. OpenID Connect exists precisely to add the authentication layer
> properly, and "log in with X" implemented directly on OAuth without OIDC is a recurring
> class of vulnerability.

**What federation buys:** one credential, one place to disable it, one place to enforce MFA,
one place to log. When someone leaves, one account is disabled and their access to
everything ends — which is a genuine and large improvement over the alternative.

**And what it costs:**

> **The identity provider becomes the single most critical system you operate.** If it is
> unavailable, nobody can log in to anything. If it is compromised, everything is
> compromised.

Which has three consequences that belong in Chapter 56's terms: its availability target is
higher than anything it serves; it needs a break-glass path that does not depend on it;
and its own administrative access must be protected differently from everything else, because
it cannot protect itself.

## What breaks here

**Correct authentication and excessive access.** **The Target pattern.** §59.3.

**"Multi-factor" that is two things you know.** **Not multi-factor.** A password and a security
question is one factor twice.

**MFA defeated by a real-time phishing proxy.** Expected for every factor except FIDO2. The
user typed a valid code into the attacker's page and the attacker used it immediately.

**MFA defeated by a helpdesk reset.** Chapter 57 §57.4 — a process gap, and currently the most
productive route into well-defended organisations.

**Push fatigue.** The user approved the fortieth prompt at 02:00. Number matching helps.

Passwords expiring every 90 days and getting worse. **Predictable transformations.** Expire
on evidence.

An SSH key granting access to someone who left in 2021. Keys do not expire and nobody has
an inventory. Certificate-based SSH or a brokered bastion.

A service account whose password cannot be changed. The application hard-codes it, or
nobody knows what depends on it. This is a Chapter 53 §53.2 documentation failure with a
security consequence.

The identity provider is down and nobody can work. Expected, and it is why it needs a
break-glass path — a small number of local accounts, with credentials in a safe, and their use
alarmed.

A biometric template transmitted as an authentication token. A serious design error. It
cannot be revoked.

> **Network+ note.** Objective 4.1 covers authentication. Over-learn: the three factors are
> something you know, have and are; **MFA requires factors of different types**; **SSO allows
> one credential across systems**; LDAP is a directory service and Kerberos provides ticket-
> based authentication; and **certificates can authenticate devices and users.** The
> multi-factor definition is examined and the "two of the same type is not MFA" point is a
> favourite.
