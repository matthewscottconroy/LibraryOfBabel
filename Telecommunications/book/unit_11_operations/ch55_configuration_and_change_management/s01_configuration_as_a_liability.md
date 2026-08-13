# 55.1 Configuration as a Liability

**A device's configuration is not an asset. It is a liability that accumulates**, and framing
it that way changes what you do about it.

## Why "liability"

**An asset is something you own that produces value. A liability is something you carry that
imposes ongoing cost.**

**Every line of configuration:**

- **must be understood** by anyone reasoning about the device
- **may interact** with any other line, in ways visible only in specific circumstances
- **must be preserved** through every migration, upgrade and replacement
- **must be reviewed** for security relevance
- **and cannot safely be removed** unless someone establishes what it does

> **The value is in the function the configuration provides, not in the configuration itself.**
> **Two devices delivering identical service, one with 200 lines and one with 2,000, are not
> equally good.** The second is carrying 1,800 lines of unpaid debt.

## The three properties of drift

**Configuration drift has a shape**, and naming its properties determines how to fight it.

### It accumulates monotonically

**Nothing removes configuration by default.**

**A change is almost always an addition.** A new requirement produces a new rule; a problem
produces a workaround; a migration leaves both the old and the new in place "until we are
sure", and nobody returns.

```
   Year 1:  ████                          40 lines
   Year 3:  ████████████                  190 lines
   Year 5:  ████████████████████          420 lines
   Year 10: ██████████████████████████████ 1,100 lines

   Removals over ten years: approximately none.
```

**A ten-year-old firewall policy contains rules referencing servers decommissioned in 2019**,
permitting protocols nobody uses, **in an order that matters** — and nobody dares remove any of
them **because nobody knows what would break.**

### It is invisible

**There is no alarm for "this rule has matched zero packets in three years."**

**Unless you build one.** And this is the crucial asymmetry: **a configuration that is wrong in
a way that breaks something is discovered immediately; a configuration that is merely
unnecessary is never discovered at all.**

> **Drift is not detected by operation. It is only detected by deliberate inspection**, which is
> why it requires a process rather than attention.

### It compounds

**Each layer of accumulated exception makes the next change harder to reason about**, which
makes it more likely to be made **by addition rather than by correction.**

```
   Rule 40: permit 10.1.0.0/16 → 10.2.5.10 tcp/443
   Rule 41: deny   10.1.7.0/24 → 10.2.5.10          ← added later, exception
   Rule 42: permit 10.1.7.14   → 10.2.5.10 tcp/443  ← exception to the exception
   Rule 43: deny   10.1.7.14   → 10.2.5.10 tcp/8443 ← and so on
```

**Nobody in year five will restructure rules 40–43 into the two rules they should be**, because
restructuring requires understanding all four **and the risk of getting it wrong falls entirely
on the person who tries.** **The incentives point at addition**, permanently.

**This is the mechanism by which a network becomes something nobody dares to change** — which
is Chapter 53 §53.4's undocumented "why", seen from the configuration's side.

## What it costs

**Worth being concrete, because "technical debt" is too abstract to fund work against.**

| Cost | How it appears |
|---|---|
| **Slower changes** | every change requires understanding more |
| **More incidents** | interaction with a rule nobody remembered |
| **Longer incidents** | more configuration to read at 03:00 |
| **Failed migrations** | **the new platform will not accept a rule the old one tolerated** |
| **Security exposure** | **a permit rule for a server that was replaced by an untrusted one** |
| **Audit failure** | rules nobody can justify |
| **Vendor lock-in** | **the configuration is now too complex to reproduce elsewhere** |

**The security one deserves expansion, because it is the sharpest.**

> **A firewall rule permitting traffic to 10.2.5.10 was correct when 10.2.5.10 was a hardened
> application server. It is a hole when 10.2.5.10 has been reissued to a printer** (Chapter 53
> §53.3's quarantine argument) **or to a test system.** **The rule did not change; the world
> did.**

## The countermeasures

**Four, and they are complementary rather than alternatives.**

### Golden configurations

**A defined standard configuration per device role**, from which every device's actual
configuration should differ only in identity.

```
   Golden template (role: branch access switch)
     ├── AAA, syslog, SNMPv3, NTP servers
     ├── management VLAN and ACL
     ├── standard port profiles: access, voice, AP, uplink
     ├── spanning tree settings and guards
     └── banner, timezone, logging levels

   Device sw-mcr-03 = golden + { hostname, addresses, VLAN membership }
```

**And then compliance checking:** **compare each device's running configuration against the
template and report the differences.**

> **The output of a compliance check is a list of things that are either drift or undocumented
> local requirements**, and **triaging that list is the work.** Every difference is either
> removed or promoted into the template.

**Tools:** `rancid`/`oxidized` for collection, **Batfish** for configuration analysis, **NAPALM
and Nornir** for scripted comparison, **and any vendor's own compliance product.** **A
twenty-line script comparing configurations to a template is worth more than most of them**,
because it will actually be run.

### Hit counters

**The direct answer to invisibility.**

**Every ACL entry, firewall rule and route-map clause on a modern platform has a match counter.**

```
   $ show access-list OUTSIDE-IN
     10 permit tcp any host 203.0.113.10 eq 443 (14,882,913 matches)
     20 permit tcp any host 203.0.113.11 eq 25  (2,441 matches)
     30 permit tcp 198.51.100.0/24 any eq 3389  (0 matches)      ← three years
     40 permit udp any host 203.0.113.12 eq 161 (0 matches)      ← and this is SNMP
```

**Rules 30 and 40 are the finding.** **Rule 40 in particular is an open SNMP hole to the
Internet that nothing uses** (§54.2's disclosure argument).

**The discipline:** **record hit counts periodically, and treat "zero matches over a defined
period" as a review trigger** — not as an automatic deletion, because **some rules exist for
rare events** (disaster recovery paths, annual processes) **and a zero count is expected.**

> **Zero hits is not proof a rule is unnecessary. It is proof that nobody has asked the
> question**, and asking it is the point.

### Periodic review with a defined removal process

**Review without a removal process produces a list nobody acts on.**

**A process that works:**

1. **Identify candidates** — zero hits, references to decommissioned assets, expired exceptions
2. **Find an owner** — and **if there is no owner, that is itself the finding**
3. **Announce intent to remove**, with a date
4. **Disable rather than delete** — comment it out, or add a deny above it with logging
5. **Wait** — one full business cycle, which for many organisations means a quarter
6. **Delete**, and **record the removal** in version control (§55.4)

**Step 4 is what makes it safe.** **Something that breaks while a rule is disabled is restored
in seconds; something that breaks after deletion is an archaeology exercise.**

### Expiry dates on exceptions

**The preventive measure, and the one most worth adopting.**

> **Every exception gets an expiry date at the moment it is created**, recorded in the change
> record and in a comment on the configuration.

```
   ! CHG-8841 — temporary access for supplier migration
   ! REVIEW: 2026-09-30 — owner: j.okafor
   permit tcp host 198.51.100.44 host 10.2.5.10 eq 22
```

**Because the moment of creation is the only moment when anyone knows why**, and it costs
nothing then. **A year later, establishing why costs hours and frequently fails.**

**And "temporary" is the word to be suspicious of.** **A temporary rule with no expiry date is
a permanent rule with an inaccurate label**, and every long-lived network is full of them.

## The honest part

**These countermeasures require sustained management support**, and **their benefit is invisible
while their cost is not.**

> **Nobody is ever thanked for the outage that did not happen.** **The team that spends a week
> removing 300 obsolete firewall rules has, from the outside, produced nothing** — the network
> does exactly what it did before.

**Which means the argument has to be made in terms that are visible:**

- **Time to make a change** — measurable, and it improves
- **Audit findings** — measurable, and auditors are listened to
- **Incidents attributable to configuration interaction** — measurable if you record cause
- **Migration cost** — **the most persuasive**, because a platform refresh quoted against a
  1,100-line configuration and against a 300-line one are visibly different projects

**And the pragmatic route is to attach cleanup to work that is already funded.** **A migration,
a refresh or a compliance programme is an opportunity to remove rather than to copy**, and
**"we will rebuild from the standard rather than migrate the existing configuration" is the
single most valuable decision available** at those moments.

## What breaks here

**A change with an unexpected effect on unrelated traffic.** **Interaction with an accumulated
rule.** The diff (§55.4) tells you what you changed; **the interaction requires reading the
whole policy.**

**A rule nobody can justify, found in an audit.** **Expected.** The finding is not the rule; it
is that there was no process to have found it first.

**A migration that stalls because the new platform rejects the old configuration.** **Frequently
a blessing.** It is forcing the review that should have happened.

**A permit rule to an address that has been reissued.** **Chapter 53 §53.3's quarantine, and
§55.1's review, both failed.** This is a genuine security exposure with a mundane cause.

**A "temporary" exception three years old.** **No expiry date at creation.** Add them from now
on; retrofitting them is a review exercise.

**Compliance checking producing 4,000 differences.** **The template does not reflect reality.**
Start by promoting legitimate local variation into the template; **a check nobody can act on
will be switched off.**

**Configuration removed and something broke a month later.** **No disable-before-delete, or too
short a soak.** One business cycle, minimum.

> **Network+ note.** Objective 3.2 covers configuration management. Over-learn: **configuration
> drift is unplanned divergence from a baseline**; **a baseline or golden configuration defines
> the standard**; **version control tracks changes**; and **regular audits detect drift.** The
> drift concept is examined; the accumulation argument is what you will use.
