# 53.1 The Three Diagrams

The commonest documentation mistake is drawing one diagram containing everything. It
becomes unreadable, it is impossible to keep current, and it answers no question well.

The standard practice is three, each answering one question.

## Why three

A network has three independent structures, and they do not resemble each other.

```
   PHYSICAL                LOGICAL                 ROUTED
   what is plugged         what talks to what,     how traffic gets
   into what               in which segment        between segments

   Rack 3 U14 port 12 ───▶ VLAN 20 ──────────────▶ 10.20.0.0/24
   Panel A-3-14                                     gw 10.20.0.1
   Rack 7 U2 port 4        (trunk, STP blocking     OSPF area 0
                            on one uplink)
```

Chapter 11 §11.3 made the point in the abstract: physical topology and logical topology are
independent. The three diagrams are that argument turned into practice — and a single
diagram attempting all three necessarily obscures all three.

| | Answers | Audience | Changes when |
|---|---|---|---|
| **Physical (L1)** | **what is plugged into what** | **the person at the rack at 03:14** | **cables move** |
| **Logical (L2)** | **what talks to what** | design, troubleshooting | **VLANs or trunks change** |
| **Routed (L3)** | **how segments reach each other** | design, routing, security | **subnets or protocols change** |

And the different change rates are the practical argument. A combined diagram must be
updated whenever any of the three changes, which is why combined diagrams are always stale.

## The physical diagram

**Answers: what is plugged into what?**

This is the one you want at 03:14, and it is the one most often missing.

**What belongs on it:**

- Devices, with their names and physical locations — building, room, rack, rack unit
- **Ports**, by their actual identifiers — `Gi1/0/24`, not "the third one along"
- **Cable types** — Cat6a, OM4, single-mode — and **cable identifiers**
- **Patch panel positions** at both ends
- The route and length of anything leaving a room, especially which duct or riser it
  uses (Chapter 50 §50.3's shared risk, at building scale)
- Media converters, extenders and anything else in the path that people forget exists

**What does not belong on it:**

- **IP addresses.** They are on the L3 diagram.
- **VLANs.** They are on the L2 diagram.
- **Individual workstations.** A diagram showing every desk is a database rendered badly.

> **The physical diagram is about copper, glass and rack units.** If you find yourself writing
> a subnet on it, you are drawing the wrong diagram.

**The test:** could someone who has never been in the building find the far end of a cable
using only this document? If not, it is incomplete.

## The logical diagram

Answers: what talks to what, and in which segment?

**What belongs on it:**

- **VLANs**, with numbers, names and purpose
- **Trunk links**, and which VLANs each carries — the allowed list matters
- **Spanning tree topology**: the root bridge, identified explicitly, and which ports are
  blocking (Chapter 19 §19.2)
- **Link aggregation groups** and their members (Chapter 19 §19.4)
- Where Layer 2 ends — the boundary between switching and routing

The spanning tree information is the part most often omitted and most often needed.

> A diagram showing every link as active is a lie about a spanning-tree network. **Mark the
> blocked ports**, because the network's actual traffic path is the one with them removed —
> and because a link that has been blocking for three years is a link nobody has verified
> works.

This diagram may bear little resemblance to the physical one, and that is expected. Two
devices in adjacent racks may be in different VLANs and never exchange a frame; two devices in
different buildings may be in the same broadcast domain.

## The routed diagram

**Answers: how does traffic get between segments?**

**What belongs on it:**

- Subnets and their prefixes, with the gateway address
- **Routing protocols**, **areas** and which devices are in which (Chapter 31)
- **WAN circuits**, with provider, circuit reference and contracted rate — the reference is
  what you quote when you telephone them at 03:20
- Firewalls and their zones (Chapter 60)
- **NAT boundaries** (Chapter 33) and **where addresses change**
- **Default route origination** — where does "everything else" go, and what happens if that
  device fails

The circuit reference is a small thing that saves twenty minutes, and it is almost never on
the diagram. **Put it there.**

## What to leave off, in general

**The most useful editorial rule:**

> **Diagrams show structure. Databases hold instances.**

If a thing is one of many identical things, it belongs in a table, not on a drawing. Forty
access switches configured identically are one box labelled "access layer, 40 × model X, see
inventory", not forty boxes.

And a diagram that requires zooming is a diagram nobody reads. Each of the three should
fit on a page at a size a person can read, or it should be split by site.

## Conventions worth following

Not because they are mandated, but because a reader who knows them reads faster.

| Convention | Meaning |
|---|---|
| **Solid line** | **physical link** |
| **Dashed line** | **logical link, tunnel or virtual connection** |
| **Line weight** | **relative capacity** — thicker is faster |
| **Cloud** | **something outside your control** |
| **Consistent left-to-right or top-to-bottom flow** | **outside → inside**, or **core → edge** |
| **Colour by function**, not decoration | VLANs, zones, or ownership — **and stated in a key** |

**Always include, on every diagram:** a title, the date, the author, a version, and a key.

> **An undated diagram is an assertion with no evidence.** The reader cannot tell whether it
> describes the network or the network of four years ago, and will therefore either trust it
> wrongly or ignore it.

## Keeping them current

The hard part, and the reason most documentation fails.

**Calendar-based review does not work.** "Review the diagrams every quarter" is an item that
slips, and slipping is invisible until an incident.

Change-triggered review does work, and Chapter 55 §55.2 supplies the mechanism:

> **A change is not complete until the documentation reflects it.** Make it a required field on
> the change record, and make the change's closure depend on it.

**And automate what can be automated.** LLDP and CDP neighbour tables can generate a physical
topology automatically; NetBox, Nautobot and similar tools can render diagrams from a source
of truth; `show spanning-tree` output tells you the real root bridge, which is frequently
not the one on the diagram.

**The realistic position:** automated discovery produces a current, ugly, complete picture;
a hand-drawn diagram produces an intelligible, curated, out-of-date one. **Keep both**, use
discovery to detect when the drawing has diverged, and treat divergence as a defect.

## What breaks here

**Stale documentation trusted during an incident.** **Worse than none**, because it directs
effort at the wrong place. **Date everything.**

**A diagram with everything on it.** Nobody reads it, so nobody maintains it, so it decays
faster. **Split it.**

Spanning tree not shown, and the actual root is not the intended one. **Extremely common**,
and it means the traffic path is not the designed one (Chapter 19 §19.3).

A "redundant" pair of links that share a duct. The physical diagram would have shown it
if it recorded routes rather than only endpoints.

Nobody can find the far end of a cable. The physical diagram exists and lacks patch panel
positions. Endpoints are not enough; the path through the panels is the part you need.

**An undocumented device discovered during an incident.** It appears in no diagram and no
inventory. Discovery tooling finds these; nothing else will.

> **Network+ note.** Objective 3.1 covers documentation directly. Over-learn: physical
> diagrams show cabling and hardware, logical diagrams show data flow and addressing; **rack
> diagrams and floor plans document physical layout**; **wiring diagrams and IDF/MDF
> documentation record patching**; and baseline configurations and network maps are part of
> the standard documentation set. The physical-versus-logical distinction is examined
> regularly.
