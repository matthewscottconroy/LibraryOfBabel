# 27.4 IPAM and Documenting a Plan

An address plan that exists only in the running configuration is not a plan. It is an
archaeological record, and reading it is archaeology.

This section is about the documentation discipline that makes an address plan usable, and
it is the least glamorous and most consistently valuable material in the unit.

## Why documentation is the plan

Consider what happens without it.

An engineer needs a subnet for a new application. **They must determine what is free.**
Their options:

- Ask the person who has been there longest, and hope
- Ping-scan a range and see what answers — which finds live hosts, **not allocations**,
  and a range that is allocated but currently idle looks free
- Read every router configuration and reconstruct the plan
- **Guess**

**Every one of these fails eventually**, and the failure mode is an overlapping
allocation, which produces intermittent connectivity that depends on which route was
learned first. It is among the hardest faults to diagnose because nothing is broken —
two things are both correct and incompatible.

> **The purpose of address documentation is not tidiness. It is to make "what is free?"
> answerable.**

Chapter 26 §26.4 made the same point about reserved ranges: undocumented free space is
space nobody dares use.

## What a record must contain

Per allocation, at minimum:

| Field | Why |
|---|---|
| **Prefix** | the allocation itself |
| **Purpose** | what it is for, in words |
| **Site / location** | where |
| **VLAN ID** | the correspondence of Chapter 20 §20.4 |
| **Gateway address** | the convention, stated explicitly |
| **DHCP range** | which part is dynamic |
| **Static range** | which part is not |
| **Reserved** | explicitly, so it is not mistaken for free |
| **Owner / contact** | who to ask before changing it |
| **Date allocated** | for aging out abandoned allocations |
| **Status** | in use / reserved / deprecated / free |

**The status field earns its place.** A deprecated allocation must not be reused
immediately — old configurations, old firewall rules and old documentation still
reference it, and reissuing it produces the most confusing possible fault. **Leave
deprecated ranges fallow for six months.**

## A worked record

```
Prefix       : 10.1.4.0/22
Purpose      : HQ user workstations
Site         : Headquarters, floors 1-4
VLAN         : 140
Gateway      : 10.1.4.1  (HSRP virtual; .2 and .3 are the physical routers)
DHCP range   : 10.1.4.50 - 10.1.7.200
Static range : 10.1.4.2 - 10.1.4.49  (infrastructure, printers)
Reserved     : 10.1.7.201 - 10.1.7.254  (future static)
Owner        : Network Operations
Allocated    : 2023-03-14
Status       : in use
Notes        : Sized for 1,000 users; currently ~640. Expansion to /21
               would require renumbering 10.1.8.0/22 (voice).
```

**The Notes field is where the next engineer's time is saved.** It records what the
allocation cannot do and why, which is exactly what is not recoverable from a router
configuration.

## Conventions worth adopting

Consistency matters more than which convention you choose. Pick one, write it down, and
apply it everywhere.

### Gateway placement

**`.1` is conventional** and near-universal. Some organisations use the **last** usable
address (`.254` in a /24) on the reasoning that it leaves the low addresses contiguous
for servers.

Either is fine. **What is not fine is varying it**, because every troubleshooting session
then begins with determining the gateway.

### Address layout within a subnet

A common and sensible arrangement for a /24:

| Range | Use |
|---|---|
| `.1` | gateway |
| `.2` – `.9` | network infrastructure (switches, APs, routers) |
| `.10` – `.49` | servers and static devices |
| `.50` – `.200` | **DHCP pool** |
| `.201` – `.250` | reserved for future static |
| `.251` – `.254` | management / spare |

**The reserved block matters.** Without one, the DHCP pool eventually reaches the top of
the subnet and any new static device must be squeezed into the pool's range, which
requires a DHCP exclusion and produces a conflict the day someone forgets.

### Point-to-point links

Allocate from **one dedicated block**, so they aggregate:

```
   10.255.0.0/16  — all WAN and inter-router links
     10.255.0.0/30    HQ - Branch 1
     10.255.0.4/30    HQ - Branch 2
     10.255.0.8/30    Branch 1 - Branch 2
```

**Use /31 where supported** (RFC 3021). It halves the consumption and the objection —
"some equipment does not support it" — is now rare.

### Loopback addresses

**Every router should have a loopback interface with a /32 address**, from a dedicated
block:

```
   10.254.0.0/24  — router loopbacks, /32 each
     10.254.0.1/32   core-1
     10.254.0.2/32   core-2
     10.254.0.11/32  branch1-rtr
```

**Why this matters more than it looks:** a loopback is **always up**, independent of any
physical interface. So it is the correct source address for management traffic, for
logging, for SNMP, for routing protocol identifiers, and for BGP peering — because it
does not go down when one link does. Chapter 32 §32.2 develops this for BGP, and it is
one of the more valuable conventions in this section.

### Encode the structure

Chapter 26 §26.4's point, restated as a convention:

```
   10 . <site> . <function+subnet> . <host>
```

so that `10.3.21.45` is readable as *site 3, function-block 21, host 45* **without
consulting anything.** A log line becomes self-describing, which is worth a great deal
during an incident at 3 a.m.

## Tools

### The spreadsheet

**Where most organisations are**, and it is genuinely acceptable for a small network.

**Its failure modes are well known:** it goes out of date, several copies exist, nobody
knows which is authoritative, and it does not prevent an overlapping allocation because
nothing checks.

**If you use a spreadsheet, put it in version control.** That single change gives you
history, attribution, and a single authoritative copy — most of what a real tool provides,
at no cost.

### Real IPAM tools

| Tool | Character |
|---|---|
| **NetBox** | Open source, the current de-facto standard. IPAM plus full DCIM, strong API. |
| phpIPAM | Open source, lighter, IPAM-focused |
| Infoblox | Commercial, integrated DDI (DNS + DHCP + IPAM), large enterprise |
| BlueCat | Commercial DDI |
| SolarWinds IPAM | Commercial, common in mid-sized organisations |

**What a real tool provides over a spreadsheet:**

- **Overlap detection** — it refuses to create a conflicting allocation
- **Automatic next-free-subnet** of a given size
- **An API**, so provisioning and automation can allocate correctly (Chapter 70)
- **Audit history** — who changed what, when
- **Integration with DNS and DHCP** so records are created together
- **Discovery** — scanning to find what is actually in use versus what is recorded

**NetBox is the one to know.** It has become the standard source of truth in network
automation, and Chapter 70 §70.3 uses it as the data source for generated configuration.

### DDI

**DNS, DHCP and IPAM together.** The argument is that the three are the same data: an
allocation implies a DHCP scope and a set of DNS records, and maintaining them separately
guarantees they will disagree.

**They do disagree, constantly**, in organisations that maintain them separately — and
stale DNS records pointing at reassigned addresses are a recurring source of both outages
and security findings.

## Renumbering

**The thing everyone avoids**, and it is worth knowing what makes it expensive so you can
avoid needing it.

**Why it is hard:**

| Depends on the address | Where |
|---|---|
| Static host configurations | every server, printer, camera, controller |
| DHCP reservations | the DHCP server |
| DNS records | forward and reverse zones |
| Firewall rules | every rule referencing the range |
| ACLs | routers, switches |
| Application configuration | connection strings, allow-lists, licence servers |
| Monitoring | every check, every dashboard |
| Documentation | all of it |
| Partner allow-lists | **other organisations' firewalls** |

**The last row is the killer.** You do not control it, and every partner must be
contacted, scheduled and verified.

**When it becomes necessary:**

- A merger with overlapping ranges (§27.1's collision problem)
- Outgrowing a block with no room to expand
- Migrating from a provider-assigned public block
- Fixing an unaggregable plan (Chapter 26 §26.4)

**The mitigation strategy is to make it unnecessary**: allocate generously, plan
hierarchically, leave reserved space, and choose unlikely private ranges. **All of that
is free at design time and expensive to retrofit.**

**When it is unavoidable**, the standard approach is **dual-addressing**: add the new
range as a secondary on each interface, migrate hosts gradually, verify, then remove the
old range. Slow, safe, and the only approach that permits rollback at every stage.

## The audit

A discipline worth adopting: **periodically compare what is documented with what exists.**

```bash
# What subnets do the routers actually have?
show ip route connected
ip -br addr

# What is actually responding?
nmap -sn 10.1.0.0/16

# What does DHCP think it has leased?
# What does DNS have records for?
```

**Three lists — documented, configured, and live — and they will not agree.** Each
disagreement is either an undocumented allocation, an abandoned one, or a mistake, and
all three are worth finding before they cause an incident.

## What breaks here

**No documentation.** Overlapping allocations, and nobody can answer "what is free?".

**Documentation that nobody updates.** Worse than none, because it is trusted.

**Reusing a deprecated range immediately.** Old configurations still reference it.

**No reserved space in a subnet.** The DHCP pool grows into the static range.

**Inconsistent gateway convention.** Every troubleshooting session starts with a
question that should not need asking.

**No loopbacks on routers.** Management and routing sessions die when a physical
interface does.

**Discovering a plan cannot aggregate after deploying it.** Chapter 26 §26.4, and the fix
is renumbering.

> **Network+ note.** Objective 3.1 expects documentation, IPAM and asset management as
> operational practice. It is examined lightly and it matters enormously in practice.
> Over-learn: **IPAM prevents overlapping allocations and answers "what is free?"**;
> **DDI integrates DNS, DHCP and IPAM because they are the same data**; and
> **documentation must record reserved ranges explicitly**, or they become unusable.
