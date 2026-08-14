# 53.2 Inventory, Racks and Labels

The parts nobody enjoys, and the parts that pay back most. Each of them converts a
twenty-minute investigation into a ten-second lookup, and they do it at the moments when
twenty minutes is expensive.

## Labelling: encode location, not purpose

The single most important rule in this section, and it is counter-intuitive.

> **Label by location. Never by purpose.**

**Because purpose changes and location does not.**

| Label | In five years |
|---|---|
| `Accounts printer` | **wrong, and actively misleading** |
| `Server room switch 2` | **wrong after the room is renumbered** |
| `Sales VLAN uplink` | **wrong after the VLAN is repurposed** |
| **`A-3-14 → B-1-07`** | **still correct** |

A purpose-based label does not merely become useless. It becomes a trap, because the next
person reads it, believes it, and acts on it. An unlabelled cable is honestly unknown; a
wrongly labelled one is confidently wrong.

### A scheme that works

**Structured, hierarchical, and readable in both directions:**

```
   BUILDING - FLOOR - ROOM - RACK - UNIT - PORT

   HQ-02-CR1-R03-U14-P12
   │   │   │    │   │   └── port 12
   │   │   │    │   └────── rack unit 14
   │   │   │    └────────── rack 3
   │   │   └─────────────── comms room 1
   │   └─────────────────── floor 2
   └─────────────────────── building HQ
```

**And a cable label carries both ends:**

```
   HQ-02-CR1-R03-U14-P12 ──▶ HQ-04-CR2-R01-U08-P03
```

Label both ends of every cable, with the same information on each. A label at one end
only is half a label, and the end you can reach is never the one you need.

**Three practical notes:**

**Use printed labels, not handwriting.** A label maker costs less than an hour of engineer
time, and handwriting on a cable in a dark rack is not legible.

Wrap-around or flag labels on cables, because adhesive labels on round cable fall off.

Label the patch panel ports themselves, permanently, at installation. The panel outlives
every cable plugged into it.

## Port and patch records

The record that answers "which switch port is this device on?"

Without it, that question is a physical exercise: find the wall outlet number, find the
patch panel, trace the patch lead, read the switch port. Twenty minutes, and a trip to the
comms room.

**With it, it is a lookup.**

| Outlet | Patch panel | Panel port | Switch | Switch port | VLAN | Notes |
|---|---|---|---|---|---|---|
| `2-114-A` | `HQ-02-CR1-R03-U20` | 14 | `sw-hq-02-1` | `Gi1/0/14` | 20 | |
| `2-114-B` | `HQ-02-CR1-R03-U20` | 15 | `sw-hq-02-1` | `Gi1/0/15` | 20 | **wall AP** |

And it is what makes several other things practical:

- **Port-based security** (Chapter 60) — you cannot restrict by port if you do not know which
  port
- Tracing a device from a MAC address — the switch tells you the port; this table tells
  you the room
- **Decommissioning** — finding and reclaiming the ports of removed equipment, which otherwise
  never happens

> The MAC-address-table-to-physical-outlet path is the single most common operational
> lookup in an access network, and this record is what makes it possible without walking.

## Rack elevations

What occupies which rack unit, and three numbers that matter more than they sound.

| | Why |
|---|---|
| **Position (U)** | **does the new device fit, and where** |
| **Depth** | **a 900 mm switch does not fit a 600 mm rack**, and this is discovered on delivery |
| **Power draw** | **does the circuit have headroom** (Chapter 56 §56.3) |
| **Weight** | **floor loading, and whether one person can lift it** |
| **Airflow direction** | **front-to-back or back-to-front** — mixing them makes a hot aisle useless |

Airflow direction is the one that surprises people. Most network switches were
historically side-to-side or back-to-front, which is the opposite of every server in the same
rack. Installing one without checking produces a device ingesting its neighbours' exhaust,
and it is a slow, intermittent, temperature-dependent fault.

**Leave gaps deliberately** — a full rack with no spare units is a rack that cannot accept a
replacement device during an incident without removing something first.

## Asset inventory

What you own, and — critically — how long it will keep working.

| Field | Why it is there |
|---|---|
| **Hostname and role** | identification |
| **Make, model, serial** | **support calls require the serial** |
| Physical location | **references the rack elevation** |
| Purchase date | depreciation, and age |
| **Support contract expiry** | **whether you can get a replacement at 04:00** |
| **Current firmware version** | Chapter 55 §55.3's patching input |
| **End-of-sale date** | you can no longer buy more |
| **End-of-support / EOL date** | **no more security patches; no more replacements** |
| Management address | |
| **Owner** | **who decides about it** |

> An inventory without end-of-support dates cannot answer the only strategic question anyone
> asks of it, which is "what must we replace next year and what will it cost?"

And the EOL date is a security input as much as a budgeting one. A device past end of
support receives no patches, so a vulnerability disclosed after that date is permanent.
Chapter 62 treats this as a risk; Chapter 55 §55.3 treats it as a plan.

Two fields worth adding that standard templates omit:

"Why does this exist?" — a sentence. The device nobody can decommission because nobody
knows what it does is a real and common cost, and one sentence written at installation
prevents it.

"What breaks if this is turned off?" — the blast radius, recorded when it is cheap to
determine rather than during an outage.

## Circuit and service inventory

The records nobody keeps and everybody needs at 03:20.

| Field | |
|---|---|
| **Provider** | |
| **Circuit reference** | **the string they will ask for and nothing else will do** |
| **Service type and rate** | |
| **Support telephone number** | **and the out-of-hours one, which is different** |
| **Contract and account number** | |
| **Contracted restoration time** | **so you know whether to wait or to invoke DR** |
| **Renewal date** | so a renewal is a decision rather than an event |
| **Physical path**, as far as it is known | Chapter 50 §50.5's diversity question |

The out-of-hours number being different from the daytime number is not a detail. It is
the difference between reporting a fault at 03:20 and reporting it at 09:00.

## Making it survive

Every record in this section decays, and the mechanisms that keep them current are the same
three:

**Tie updates to the change process.** Chapter 55 §55.2. A change is not closed until the
records reflect it.

**Automate discovery and compare.** LLDP, ARP tables, MAC tables, DHCP leases and switch port
descriptions can be collected continuously and diffed against the source of truth. The diff
is the useful output — it tells you where reality and records have parted company.

**Make the record the thing that provisions.** The strongest version: the inventory is not a
description of the network, it is the source from which the network is configured
(Chapter 70). Then it cannot be stale, because a stale record produces a wrong device.

> A source of truth that only describes will drift. A source of truth that also configures
> cannot.

## What breaks here

A cable labelled with a purpose that changed. **Confidently wrong.** Relabel by location.

**Labels at one end only.** The end you can reach is never the one you need.

A device that does not fit the rack. **Depth, not height.** The elevation should record
both.

A switch overheating in a properly cooled room. **Airflow direction mismatch.** It is
ingesting the exhaust of the device below it.

A circuit fault reported and the provider asking for a reference nobody has. Twenty
minutes lost at the worst moment. The circuit inventory exists for this.

A device that cannot be decommissioned because nobody knows what it does. The "why does
this exist" field, written three years ago, would have cost one sentence.

**A vulnerability that cannot be patched.** The device passed end of support and nobody was
tracking it. This is a planning failure, not a security one.

Records that were accurate at audit and wrong three months later. **Calendar-driven
maintenance.** Tie it to change, or automate the diff.

> **Network+ note.** Objective 3.1 covers documentation and asset management. Over-learn:
> inventory records include make, model, serial, location and support status; labelling
> should follow a standard (ANSI/TIA-606 is the reference); **rack diagrams record equipment
> placement**; and **IDF/MDF documentation maps the cabling infrastructure.** The
> label-by-location principle is not examined and is the thing you will actually use.
