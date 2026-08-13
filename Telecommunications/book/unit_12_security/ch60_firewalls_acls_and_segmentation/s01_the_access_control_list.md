# 60.1 The Access Control List

**A firewall is a list of rules, evaluated in order, each matching some property of a packet and
specifying an action.** **That is the entire concept**, and it has been since Digital Equipment
Corporation built the first packet filters in 1988.

**Starting there is worth doing, because "firewall" has accumulated enough marketing that people
forget how simple the underlying object is** — **and the simplicity is what lets you reason about
it.**

## The object

```
   ┌──────────────────────────────────────────────────────────────┐
   │ seq  action  proto  source        destination      port      │
   ├──────────────────────────────────────────────────────────────┤
   │ 10   permit  tcp    any           host 203.0.113.10  eq 443  │
   │ 20   permit  tcp    any           host 203.0.113.10  eq 80   │
   │ 30   deny    ip     any           any                        │  ← explicit
   └──────────────────────────────────────────────────────────────┘
                                                                   ← implicit deny
                                                                     is here anyway
```

**Match on**: **source and destination address**, **protocol**, **ports**, and — depending on
the platform — **interface, direction, DSCP, TCP flags, fragment status, time of day, and the
user or group identity** (Chapter 59).

**Act by**: **permit**, **deny**, and frequently **log**, **rate-limit**, **re-mark** or **redirect**.

## The two things that cause most errors

### Order matters, and evaluation stops at the first match

> **A permit rule below a deny rule that covers the same traffic is dead.**

**Which produces the classic symptom: a rule that is present, correct, and has no effect.**

```
   10  deny   ip  10.20.0.0/16  any            ← added last year, for a reason
   ...
   80  permit tcp 10.20.5.14   host 10.9.0.5 eq 443   ← added today. Never matches.
```

> **The diagnosis is to look upward in the list, not at the rule itself.** **Nearly everyone's
> first instinct is to re-read the rule they just wrote**, and the rule is fine.

**Hit counters make it immediate** (Chapter 55 §55.1): **a rule with zero hits after a week is
either unnecessary or shadowed**, and **distinguishing the two is a one-line search for a
broader rule above it.**

**And most platforms have a tool for this** — `show access-list` counters, a firewall manager's
shadow analysis, or Batfish (Chapter 55's reading). **Use one; shadowing is not reliably found
by reading.**

### There is an implicit deny at the end

**Whether or not you write it.** **Anything not explicitly permitted is dropped.**

**This is the correct default** — **a security control should fail closed** (Chapter 57 §57.2) —
**and it means that adding a service requires adding a rule, which is the point.**

**Two practical consequences:**

**Write the deny explicitly anyway**, with logging. **The implicit deny does not log**, so
**denied traffic is invisible** — and denied traffic is exactly what you want to see
(Chapter 59 §59.3). **An explicit `deny ip any any log` at the end makes the invisible visible**,
at the cost of log volume that must be rate-limited (Chapter 54 §54.3).

**And the implicit deny is why an empty ACL blocks everything.** **Applying an ACL you have not
finished writing takes the interface down**, which is a memorable way to learn it.

## Directionality

**The related error, committed constantly.**

> **Firewall rules are directional, and traffic is bidirectional.**

**A stateless filter permitting outbound HTTP must also permit the return traffic:**

```
   ! outbound
   permit tcp 10.0.0.0/8 any eq 80

   ! and the return — which is the problem
   permit tcp any eq 80 10.0.0.0/8 gt 1023
```

**That second rule permits any packet from any source port 80 to any internal address on any
high port.** **An attacker need only source their traffic from port 80 to walk through it**, and
**scanning your entire internal network from source port 80 is one command.**

**The partial mitigation is the `established` keyword** — **match only packets with ACK or RST
set**, so **a bare SYN from outside is dropped.**

```
   permit tcp any eq 80 10.0.0.0/8 gt 1023 established
```

**Better, and not sufficient.** **The flag is in the packet and the attacker sets it.** **A
packet with ACK set and no corresponding connection passes**, which is enough for scanning and
for some attacks.

> **That weakness is precisely what stateful inspection was invented to fix** (§60.2), **and
> understanding the stateless case is why the stateful one is worth its cost.**

## Where ACLs are applied

**Placement changes both effect and efficiency.**

| | **Inbound on the ingress interface** | **Outbound on the egress interface** |
|---|---|---|
| Filtered | **before routing** | **after routing** |
| Efficiency | **better — dropped early** | worse |
| Applies to | traffic arriving on that interface | traffic leaving, from any interface |
| Router's own traffic | **filtered** | **frequently not** |

**The classical guidance — "standard ACLs close to the destination, extended ACLs close to the
source" — follows from this:** **a standard ACL matches only the source address**, so **applying
it near the source would block that source's traffic to everywhere**, and **an extended ACL
matches the destination too, so it can be specific and should drop early.**

**And "close to the source" has a security rationale beyond efficiency:** **traffic that is
dropped at the access layer never traverses the network at all**, which means **it cannot be
seen, cannot be misrouted, and cannot exploit anything in between.**

## Writing rules that survive

**Six practices, each of which prevents a class of problem the chapter has already met.**

**Use named ACLs and sequence numbers.** **Numbered ACLs on older platforms cannot be edited —
adding a rule appends it, and reordering means deleting and retyping the whole thing.** **Named
ACLs with sequence numbers permit insertion**, which is the difference between a five-second
change and a risky one.

**Leave gaps in the numbering.** **Sequence 10, 20, 30 — not 1, 2, 3** — so a rule can be
inserted between two existing ones without renumbering.

**Comment every rule with a change reference.** Chapter 55 §55.1's expiry-date argument:

```
   ! CHG-8841  supplier migration  REVIEW 2026-09-30  owner j.okafor
   permit tcp host 198.51.100.44 host 10.2.5.10 eq 22
```

**Object groups, not repeated addresses.** **A group named `WEB-SERVERS` referenced by twelve
rules is changed in one place**; twelve rules with literal addresses are changed in twelve, and
one will be missed.

**Specific before general.** **Not merely for correctness but for readability** — a policy whose
exceptions are scattered among its general rules cannot be reasoned about.

**And a deny for the things that should never appear**, at the top:

```
   deny ip 10.0.0.0/8       any   ! RFC 1918 arriving from outside
   deny ip 127.0.0.0/8      any   ! loopback
   deny ip 169.254.0.0/16   any   ! link-local
   deny ip <your own prefixes> any ! spoofed as you — and this one catches real attacks
```

**The last line is anti-spoofing** (Chapter 57 §57.4's BCP 38, from the receiving side), **and
it is cheap and effective.**

## IPv6 and the ICMP mistake

**A specific trap that produces mysterious failures.**

> **Blocking all ICMP is bad practice in IPv4** (Chapter 34) **and breaks IPv6 entirely.**

**IPv6 depends on ICMPv6 for functions IPv4 does not:**

| Type | Required for |
|---|---|
| **Neighbour Solicitation / Advertisement** | **address resolution — IPv6's ARP** (Chapter 28 §28.3) |
| **Router Solicitation / Advertisement** | **SLAAC — hosts get no address without it** |
| **Packet Too Big** | **PMTUD — and IPv6 routers do not fragment** (Chapter 24 §24.3) |
| Time Exceeded, Parameter Problem | diagnostics |

**Blocking ICMPv6 does not harden an IPv6 network. It stops it working**, and **RFC 4890
specifies which types must be permitted.**

**And in IPv4, blocking Type 3 Code 4 — Fragmentation Needed — creates a PMTUD black hole**
(Chapter 34 §34.4): **small packets work, large ones vanish, and the cause is a firewall rule
written years earlier by someone being careful.**

## What breaks here

**A rule that is present, correct and has no effect.** **Shadowed.** Look upward.

**An ACL applied and the interface goes dark.** **The implicit deny, and the ACL is empty or
incomplete.**

**Denied traffic invisible.** **The implicit deny does not log.** Add an explicit denying rule
with rate-limited logging.

**A stateless return rule permitting a scan from source port 80.** **The classic**, and the
reason for §60.2.

**`established` relied on as a control.** **The flag is in the packet.** Stateful inspection is
the actual answer.

**IPv6 not working after a firewall was configured.** **ICMPv6 blocked.** RFC 4890.

**Large transfers failing while pings and small requests succeed.** **PMTUD black hole** — ICMP
Type 3 Code 4 filtered.

**A change that required editing twelve rules and eleven were changed.** **No object groups.**

**A policy nobody can reorder because the platform appends.** **Numbered ACLs.** Convert to
named with sequence numbers.

**A rule from 2018 that nobody can justify.** Chapter 55 §55.1 — **comments with owners and
review dates, from the moment of creation.**

> **Network+ note.** Objective 4.3 covers ACLs. Over-learn: **ACLs are evaluated top to bottom
> and stop at the first match**; **there is an implicit deny at the end**; **standard ACLs match
> source only, extended match source, destination, protocol and port**; **place standard ACLs
> near the destination and extended near the source**; and **ACLs are applied per interface and
> per direction.** The implicit deny and the first-match rule are examined constantly.
