# 53.3 Address Management

Chapter 27 designed the address plan. This section is about keeping it true, which is a
different and harder problem.

## Plan versus record

| | **The plan** | **IPAM** |
|---|---|---|
| Is | **a design document** | **a live record** |
| Says | **how space is allocated** | **what is actually allocated, to what, by whom, when** |
| Changes | rarely, deliberately | **constantly** |
| Fails by | being wrong | **being stale** |

The plan says "site 3 gets 10.3.0.0/16, with /24 per VLAN, servers in the first ten."
IPAM says "10.3.20.14 is the Manchester file server, assigned by K. Osei on 12 March, ticket
CHG-4471."

> A plan without a record is a network where nobody knows what is in use, and the observable
> consequence is that people choose addresses by pinging to see whether anything answers.
> Which is how you assign an address to a machine that is merely switched off.

## What IPAM must record

| | |
|---|---|
| **Subnets**, with prefix, VLAN, site, gateway and purpose | |
| **Every allocation** within them | address, hostname, MAC where known |
| **Who allocated it, when, and why** | **the ticket reference** |
| **Status** | in use / reserved / free / **quarantined** |
| **DHCP ranges**, explicitly | **so nobody statically assigns inside them** |
| **Reservations** — static DHCP mappings | |
| **The corresponding DNS records** | |

**"Quarantined" is worth having as a status.** An address freed today should not be reissued
tomorrow — stale ARP entries, DNS caches, firewall rules and monitoring configurations all
still refer to it. Hold it for a defined period, then release it.

## The failure this prevents

**Specific, common, and unpleasant to diagnose.**

```
   1. DHCP pool configured: 10.20.0.100 – 10.20.0.200
   2. Someone assigns 10.20.0.150 statically to a printer.
      It works. It works for months.
   3. The pool fills. DHCP reaches .150.
   4. A laptop is offered .150. It performs a gratuitous ARP,
      or it does not, depending on the client.
   5. Two devices now hold one address.
```

And the symptom is the reason this is worth a diagram:

> Two unrelated machines have intermittent connectivity, at unpredictable times, affecting
> different protocols differently, depending on whose ARP entry each other device happens to
> hold. The printer works from some machines and not others. The laptop works until it does
> not. **Nothing in either machine's configuration is wrong.**

Duplicate address detection catches some of it — DHCP servers may ping before offering, and
clients may ARP before accepting — and both behaviours are optional and frequently disabled
for speed.

**The fix is structural, not diagnostic:** the DHCP range is recorded in IPAM, and the static
assignment is refused because IPAM knows the address is inside it.

## Integration is the point

At small scale a spreadsheet is genuinely adequate, and pretending otherwise is affectation.

A single site with 200 devices and three subnets does not need dedicated tooling. A
maintained spreadsheet, with the fields above, is better than an unmaintained IPAM system
and costs nothing.

**What changes the calculation is integration.**

```
   Without integration:                With integration:

   IPAM  ─── discipline ───▶ DNS      IPAM ──▶ DNS      created together,
     │                                  │  ──▶ DHCP     or not at all
     └───── discipline ───▶ DHCP        └── ──▶ monitoring
     
   Three records that agree            One action, three consistent
   because someone remembered          records, by construction
```

> Integrated IPAM removes an entire class of error, because the three records are guaranteed
> consistent by construction rather than by discipline.

And "by discipline" fails predictably — not because people are careless, but because the
step that is skipped under pressure is always the record-keeping one.

**The DDI acronym** — **DNS, DHCP, IPAM** — names the integrated product category, and the
common implementations are **Infoblox and BlueCat** commercially, **NetBox with plugins**,
**phpIPAM**, **NIPAP**, and **Microsoft's IPAM feature** where the environment is already
Windows-centric.

## IPv6 changes the problem

**And it changes it in both directions.**

**Easier:**

- **Scarcity is gone.** A /64 per subnet, always (Chapter 28 §28.3), so there is no
  allocation arithmetic and no reclaiming.
- Address plans can be structured semantically — encode site, floor and VLAN in the prefix
  digits, which makes an address self-describing.

**Harder:**

- **Addresses are long and not memorable.** DNS stops being a convenience and becomes
  mandatory.
- A host has several addresses at once — link-local, one or more global, temporary privacy
  addresses (RFC 8981), possibly a ULA. "Which address is this host" has no single answer.
- SLAAC assigns without telling anyone. A host can appear on the network with a valid
  global address that nothing recorded — which is the opposite of DHCP's model.

> The IPv4 question is "which addresses are free?" The IPv6 question is "which addresses are
> in use, and by what?" They require different tooling, and IPAM products that were built for
> the first answer the second badly.

**The practical answers:** DHCPv6 where you need a record (with the caveat that Android does
not support it, Chapter 40 §40.4); neighbour discovery cache harvesting from routers, which
is the IPv6 equivalent of ARP table collection; and accepting that privacy addresses will
appear and disappear and should not be treated as allocations.

## Reclaiming, which nobody does

**The unglamorous half of address management.**

Addresses are allocated constantly and released almost never, because releasing requires
knowing that nothing uses them, and nobody is confident enough to be the person who broke
something.

**A workable process:**

1. **Identify candidates** — no ARP or ND entry, no DNS query, no traffic, for a defined period
2. **Quarantine** — mark them reserved, and null-route or block them
3. **Wait** — a month is a reasonable default, longer if anything is quarterly
4. **Release** if nobody complained
5. **Record the release**, so the history exists

The blocking step is the one that makes it safe. Something that breaks during quarantine
is trivially restored; something that breaks after release is a mystery.

## What breaks here

Two machines with intermittent problems and correct configurations. **Duplicate address.**
Check the switch's MAC table for the address's ARP entry changing between two MACs.

**An address assigned by pinging first.** The device was switched off. This is not a
diagnostic technique; it is a way of creating the fault above.

**DNS and DHCP disagreeing.** Unintegrated records maintained by discipline. The
disagreement is the expected outcome, not an anomaly.

A "free" address that breaks something when reused. **No quarantine period.** Stale
references in firewall rules, monitoring, and someone's `/etc/hosts`.

Subnets exhausted while the plan says there is room. **Allocations recorded nowhere.**
Reconcile against ARP tables and DHCP leases before ordering more space.

An IPv6 host on the network with no record of it. **SLAAC.** Expected behaviour; harvest
neighbour caches rather than trying to prevent it.

An IPAM system that is beautifully complete and eight months out of date. It was not
integrated and not tied to change. The tooling was never the problem.

> **Network+ note.** Objective 3.1 and 1.7. Over-learn: **IPAM tracks address allocation**;
> **static addresses must be outside DHCP scopes**; **DHCP reservations assign a fixed address
> by MAC**; and **duplicate addresses cause intermittent connectivity.** The
> static-inside-the-pool failure is examined regularly and is worth recognising by its symptom
> rather than its description.
