# Chapter 60 — Firewalls, ACLs, and Segmentation

A firewall is a list of rules, evaluated in order, each matching some property of a
packet and specifying an action. That is the entire concept, and it has been the
entire concept since Digital Equipment Corporation built the first packet filters in
1988. Everything since has been elaboration on what can be matched and how much state
is kept.

Starting there is worth doing, because "firewall" has accumulated so much marketing
that people forget how simple the underlying object is — and the simplicity is what
lets you reason about it.

## The ACL, and the two things beginners get wrong

```
  permit tcp any host 203.0.113.10 eq 443
  permit tcp any host 203.0.113.10 eq 80
  deny   ip  any any
```

Three rules. Two things about them cause most of the errors in this chapter.

**Order matters, and evaluation stops at the first match.** A permit rule below a deny
rule that covers the same traffic is dead. This produces the classic symptom of a rule
that is present, correct, and has no effect — and the diagnosis is to look *upward* in
the list, not at the rule itself. Hit counters, where available, make this immediate:
a rule with zero hits after a week is either unnecessary or shadowed.

**There is an implicit deny at the end**, whether or not you write it. Anything not
explicitly permitted is dropped. This is the correct default — a security control
should fail closed — and it means that adding a service requires adding a rule, which
is the point.

The related error, and it is worth stating because it is committed constantly:
**firewall rules are directional, and traffic is bidirectional.** A stateless filter
permitting outbound HTTP must also permit the return traffic, which means permitting
inbound packets from port 80 to your ephemeral range — a rule broad enough that an
attacker need only source their traffic from port 80 to walk through it. That
weakness is precisely what stateful inspection was invented to fix.

## Stateful inspection

A stateful firewall remembers connections. When an internal host initiates a
connection, the firewall records the five-tuple (Chapter 35 §35.2) in a **connection
table**; return traffic matching an existing entry is permitted automatically, and
anything else is not.

This is strictly better and it costs something worth understanding.

**It costs memory**, one entry per connection, which is a finite and exhaustible
resource — and exhausting it is a denial of service in itself.

**It costs statelessness**, which Chapter 24 §24.1 identified as what let routers
scale and survive. A firewall that loses its state table — a reboot, a failover to an
unsynchronised peer — drops every established connection simultaneously. This is why
firewall high-availability pairs synchronise state, and why testing that
synchronisation is a genuine operational requirement rather than a formality.

**It requires symmetric routing.** The firewall must see both directions of a
conversation to maintain state. Asymmetric paths (Chapter 29 §29.1) break stateful
inspection, and this is one of the most confusing faults in networking: traffic works
in one direction, is dropped in the other, and everything appears correctly
configured.

§60.2 also covers the connection table's timeouts, which cause the classic "idle SSH
session dies after an hour" complaint — the firewall aged out an idle entry, and the
fix is keepalives rather than a longer timeout.

## What "next generation" means

§60.3 disentangles the terminology, since the market has made it opaque.

A **next-generation firewall** adds to stateful inspection: application identification
independent of port (recognising that traffic on port 443 is a file-sharing
application rather than a website), user identity from a directory rather than IP
address, and integrated intrusion prevention and malware inspection.

The capability that makes the rest possible is **TLS inspection**, and it deserves a
clear-eyed treatment because it is a genuine tradeoff rather than a feature.

Since most traffic is encrypted, a firewall that cannot decrypt sees only metadata. To
inspect content it must terminate the TLS session, inspect, and re-encrypt to the
destination — which is a deliberate on-path interception, requiring every client to
trust a CA controlled by the firewall.

What that buys: visibility into content, malware detection, data loss prevention.

What it costs: **every inspected session's security now depends on the firewall's TLS
implementation**, which has historically been worse than the browsers' — several
inspection products have been found downgrading cipher suites, failing to validate
upstream certificates, and reintroducing vulnerabilities the endpoints had fixed. It
also breaks certificate pinning, it creates a single point at which all of an
organisation's plaintext exists, and it has privacy and legal implications that vary
by jurisdiction and that engineers should not decide alone.

The defensible position is selective inspection with documented exclusions — banking,
health, legal — and a clear organisational decision rather than a default.

**IDS versus IPS**, also disentangled: an intrusion *detection* system observes a copy
of traffic and alerts; an intrusion *prevention* system sits in the path and blocks. A
false positive in an IDS generates an alert; a false positive in an IPS generates an
outage. That difference determines where each belongs and how aggressively each should
be tuned.

## Segmentation, which is the chapter's real subject

§60.4 makes the argument that segmentation is more valuable than any appliance, and it
follows directly from Chapter 57's threat model.

Ransomware's business model requires **lateral movement**: gain a foothold on one
machine, then spread to find the data worth encrypting and the backups worth
destroying. A flat network makes that trivial — every host can reach every other host,
so one compromised laptop reaches the file servers, the domain controllers and the
backup system.

Segmentation makes each step expensive. If the workstation VLAN cannot reach the
server VLAN except through specific permitted services, an attacker on a workstation
must find a way through rather than simply connecting. **Segmentation does not prevent
the initial compromise. It changes the compromise from a catastrophe into an
incident**, and that is the most valuable thing the network layer contributes to
security.

The chapter covers the progression: VLANs (Chapter 20) as the coarse mechanism; DMZs
for services that must be reachable from outside; **microsegmentation**, in which
policy is applied per workload rather than per subnet, enforced by hypervisor or host
firewalls, so that two servers on the same VLAN can be prevented from communicating.

And the practical warning: microsegmentation projects fail when the organisation does
not know what talks to what. The prerequisite is discovery — flow data (Chapter 54
§54.4) collected and analysed — and the sensible order is to monitor first, then
alert on violations, then enforce. Enforcing first produces an outage and a project
that gets cancelled.

## By the end you will be able to

- Write and order an ACL correctly, and diagnose a shadowed rule.
- Explain the implicit deny and why fail-closed is correct.
- Explain what stateful inspection buys and its three costs.
- Diagnose the asymmetric-routing and idle-timeout failures.
- Argue both sides of TLS inspection and propose a defensible policy.
- Distinguish IDS from IPS and place each appropriately.
- Design a segmentation scheme for a described organisation and explain the blast
  radius reduction, with a staged deployment plan.
