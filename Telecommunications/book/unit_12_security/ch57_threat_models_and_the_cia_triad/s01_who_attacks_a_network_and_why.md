# 57.1 Who Attacks a Network, and Why

The appropriate defence depends heavily on who you are defending against, and treating all
adversaries as equivalent leads to spending in the wrong places — usually a great deal on the
adversary you will never meet and very little on the one attacking you now.

## The five categories

Ordered by how likely you are to meet them, which is the opposite of how much attention they
receive.

### Opportunists

Scanning the entire Internet for known vulnerabilities. They do not know or care who you
are.

| | |
|---|---|
| **Motivation** | **whatever the access is worth** — cryptomining, a botnet node, resale |
| **Method** | **automated scanning against a list of known defects and defaults** |
| **Target selection** | **none. Your address was in the range.** |
| **Sophistication** | **low, and it does not need to be** |
| **Volume** | **overwhelming — an exposed address is scanned within minutes** |

> An IPv4 address newly exposed to the Internet receives its first scan within minutes and
> its first exploit attempt within hours. This is not an exaggeration and it is easily
> demonstrated — a honeypot on a fresh address will log thousands of attempts a day
> indefinitely.

**What defeats them, in order of effect:**

1. Do not expose management interfaces. SSH, RDP, SNMP, web administration and database
   ports on public addresses account for a very large share of successful opportunistic
   compromises.
2. Do not use default credentials. Chapter 54 §54.2's `public` and `private`, and every
   device with `admin/admin`.
3. **Patch known vulnerabilities** (Chapter 55 §55.3), especially the ones on CISA's
   known-exploited list, which is a far smaller and more actionable set than "everything with
   a high score".
4. Do not expose what does not need exposing. The strongest control is absence.

> **Most organisations' actual threat is this one**, and most organisations under-invest in
> defeating it while over-investing elsewhere. A network with no exposed management, no
> default credentials and current patches has defeated the overwhelming majority of attacks it
> will ever experience, at almost no cost.

### Criminal enterprises

Principally ransomware, and it is a business with a business model — which is the useful
way to think about it, because a business model has steps, and steps can be made expensive.

```
   1. Initial access      ── phishing, a stolen credential, an exposed service, a supplier
   2. Establish foothold  ── persistence, command and control
   3. Escalate privilege  ── local admin, then domain admin
   4. MOVE LATERALLY      ── ◀── the step that turns a foothold into a catastrophe
   5. Locate the backups  ── and destroy or encrypt them
   6. Exfiltrate data     ── for the second extortion lever
   7. Encrypt             ── everything, simultaneously, usually at 03:00 on a Sunday
   8. Demand payment
```

**Two observations that determine the network's contribution.**

**Step 4 is the network's step.** Steps 1 to 3 are largely endpoint and identity problems.
Step 4 is a network problem, and it is the step at which the outcome is decided: an
attacker confined to one segment has compromised one machine; an attacker who can reach
everything has compromised the organisation.

> Segmentation is the specific countermeasure to ransomware's business model (Chapter 60
> §60.4), because it makes step 4 slow, noisy and incomplete — and every hour it costs is an
> hour in which detection may occur.

Step 5 is why "we have backups" is not an answer. Modern ransomware operators specifically
hunt backup systems, and a backup reachable with the credentials that were compromised is
not a backup. Chapter 56 §56.4's DR planning must assume the adversary had administrative
access — which means offline or immutable copies, with separate credentials.

**And step 6 changed the calculation.** Exfiltration before encryption means paying to recover
data and paying again to prevent publication, and it removes the "we restored from backup
and did not pay" defence that worked until about 2019.

### Insiders

Malicious or — far more often — negligent.

| | Malicious | **Negligent** |
|---|---|---|
| Frequency | **rare** | **common** |
| Examples | data theft on departure, sabotage | **misconfiguration, a click, a lost laptop, a shared credential** |
| Defence | **least privilege, logging, separation of duties** | **least privilege, and making the safe path the easy one** |

> The negligent insider is not an adversary and is the largest source of incidents, and
> **defending against them is mostly design**: make the dangerous thing hard to do by
> accident. Chapter 55 §55.2's commit timers, Chapter 59's least privilege, and confirmation
> prompts that show what is about to change.

The malicious insider's distinguishing property is legitimate access, which defeats
perimeter controls entirely. Logging and least privilege are the answers, and both are
unglamorous.

### Targeted adversaries

Including state actors, who have time, money and specific objectives.

| | |
|---|---|
| **Motivation** | **espionage, disruption, positioning for later** |
| **Method** | **whatever works** — including supply chain, zero-days, and physical access |
| **Persistence** | **months to years, deliberately quiet** |
| **Detection** | **frequently by a third party** — a partner, a vendor, or a government |

> If you are genuinely a target, this book is a starting point and not a sufficient one.

The honest position for most organisations: you are not a primary target and you may be a
route to one. Supply chain compromise means a small supplier's network is attacked to reach
a large customer's, and "we are too small to be interesting" is precisely the reasoning that
makes a small organisation a useful route.

And note the pre-positioning pattern, which is now openly discussed by governments:
access obtained in critical infrastructure and left dormant, for use in a future
contingency. Its distinguishing feature is that nothing happens, which makes it
extraordinarily hard to detect — and the detection method is anomalous behaviour in the
absence of any incident (Chapter 54 §54.1's baselines).

### Hacktivists and vandals

Mostly denial of service, mostly defeated by capacity and scrubbing rather than by
cleverness.

**Their distinguishing property is publicity.** The objective is attention, so the attack
is announced, timed for effect, and directed at whatever is most visible — typically a public
website rather than the systems that matter.

Which makes them, uncomfortably, the easiest to plan for: you frequently know when, and
the target is the thing you can most easily put behind a scrubbing service (Chapter 62).

## Mapping actors to controls

**The table that should drive spending.**

| Actor | Likelihood | Impact | **The control that matters** |
|---|---|---|---|
| **Opportunist** | **certain** | low–high | **exposure reduction, patching, no defaults** |
| **Ransomware** | **high** | **catastrophic** | **segmentation, offline backups, MFA** |
| **Negligent insider** | **certain** | low–high | **least privilege, safe defaults, change control** |
| **Malicious insider** | low | high | **least privilege, logging, separation of duties** |
| **Targeted** | **low for most** | **catastrophic** | **detection, and accepting you may not win** |
| **Hacktivist** | low–moderate | moderate | **DDoS capacity or a scrubbing contract** |

> **Two of the six are certain and cheap to defend against.** Doing those two things well
> before anything else is the correct order, and it is not the order in which security
> products are sold.

## What actually happens, in practice

A useful corrective, because the categories above can make everything sound deliberate.

The published incident data — from breach reports, insurers and incident responders — is
consistent year on year:

- The great majority of incidents begin with a credential or a person, not a protocol flaw
- Phishing and stolen credentials dominate initial access
- Exploited vulnerabilities are a substantial minority, and are overwhelmingly known ones with
  patches available
- Zero-days are rare and receive attention out of all proportion to their frequency
- The median time from compromise to detection is measured in weeks or months, and a
  significant share of breaches are reported to the victim by an outsider

> **The last point should shape monitoring design** (Chapter 54): you are not likely to catch
> the intrusion. You are trying to catch what the intruder does afterwards, which is why
> baselines, lateral-movement detection and egress monitoring matter more than perimeter
> alerting.

## What breaks here

Investment concentrated on advanced threats while defaults remain unchanged. **The common
failure.** Fix the certain, cheap things first.

"We are too small to be a target." Opportunists do not select targets, and supply chain
attacks specifically prefer small suppliers.

**A backup destroyed during a ransomware incident.** It was reachable with the compromised
credentials. Offline or immutable, with separate authentication.

A flat network in which a compromised laptop reaches the finance server. **Step 4.**
Segmentation.

**An incident notified by a third party.** Normal, and it is information about your
detection. Chapter 54 §54.1's baselines are the remedy.

**A DDoS defence purchased and never tested.** Chapter 56 §56.2's argument, applied to security
controls.

**A "sophisticated attack" in a public statement.** Frequently an unpatched service or a
reused password. The word is doing public-relations work, and treating it as technical
information will mislead you.

> **Network+ note.** Objective 4.1 covers threats and actors. Over-learn: threat actors
> include script kiddies, hacktivists, organised crime, insiders and nation states; social
> engineering and phishing are the commonest initial vectors; **insider threats may be
> malicious or unintentional**; and an attack surface is the set of exposed entry points.
> The actor-to-motivation mapping is examined; **the proportionality argument is what you will
> use.**
