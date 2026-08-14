# 50.5 Submarine Cables

The Internet is a set of glass fibres lying on the ocean floor, and it is worth spending a
section on them because almost nobody who works on networks has any picture of what they
physically are.

## The scale

| | |
|---|---|
| Systems in service | **roughly 500** |
| Total length | **about 1.4 million km** — **thirty-five times round the Earth** |
| Share of intercontinental traffic | **above 99%** |
| Satellite's share | **a fraction of a percent** |
| Typical design capacity, modern system | **100–400 Tb/s** |
| Design life | **25 years** |
| Cost of a transoceanic system | **$200–400 million** |

> The public imagines the Internet is in space. It is at the bottom of the sea, and the
> maps are published — TeleGeography's is the well-known one — because there is nothing
> secret about where they land.

## What the cable is

In deep water, a submarine cable is about the diameter of a garden hose.

```
   ┌─────────────────────────────────────┐
   │ polyethylene sheath                 │
   │ ┌─────────────────────────────────┐ │
   │ │ copper conductor (the power feed)│ │
   │ │ ┌─────────────────────────────┐ │ │
   │ │ │ steel strength wires        │ │ │
   │ │ │ ┌─────────────────────────┐ │ │ │
   │ │ │ │ steel tube              │ │ │ │
   │ │ │ │  ┌──────────────────┐   │ │ │ │
   │ │ │ │  │ 4–24 fibre pairs │   │ │ │ │
   │ │ │ │  │ in gel           │   │ │ │ │
   │ │ │ │  └──────────────────┘   │ │ │ │
   │ │ │ └─────────────────────────┘ │ │ │
   │ │ └─────────────────────────────┘ │ │
   │ └─────────────────────────────────┘ │
   └─────────────────────────────────────┘
       ~17–21 mm in deep water
```

In shallow water and near shore it is armoured — one or two layers of heavy steel wire —
and can reach 50 mm and several kilograms per metre. Deep water needs no armour, because
nothing down there threatens it. Shallow water needs a great deal, because everything does.

The whole assembly carries perhaps 4 to 24 fibre pairs, and it is the copper conductor that
makes the system possible.

## Powering the amplifiers

A transatlantic cable has EDFAs (§50.3) every 60–100 km — perhaps 80 of them on a 6,600 km
route — and they need power.

The power comes from the shore, along the cable itself, at up to **15 kV DC** — a constant
current in series through every repeater, with the sea itself as the return path.

> A single conductor, one current, eighty amplifiers in series, across an ocean, running
> unattended for twenty-five years. The failure of any repeater's power path takes the whole
> system down, which is why the components in a submerged repeater are qualified to a
> standard that has more in common with spaceflight than with telecommunications.

Repeaters are the reason cables are so expensive, and the reason **unrepeatered systems** —
up to about 400 km with no amplifiers at all — are used wherever the geography permits.

## Laying and repairing

A cable ship pays out cable at 6–8 knots, with a plough burying it to a metre or two in
shallow water and simply laying it on the seabed in deep water.

Route selection is a survey exercise: avoid fishing grounds, anchorages, seismic zones,
seamounts and existing cables, and cross other cables at as close to a right angle as
possible.

**Faults happen constantly.**

| Cause | Share |
|---|---|
| **Fishing gear** | **the largest single cause** |
| **Ship anchors** | **second, and both are shallow-water events** |
| Abrasion and geological events | some |
| Component failure | **rare** |
| Deliberate damage | rare, and increasingly discussed |

Roughly 150–200 faults occur worldwide each year, and most of them are in shallow water
because that is where the human activity is.

**Repair is a physical operation:**

1. Locate the fault electrically from shore, by time-domain reflectometry, to within a
   kilometre or so
2. Dispatch a cable ship, which may be days away
3. Grapple for the cable and cut it, raising one end to the surface
4. Splice in new cable, then recover and splice the other end
5. Lower the repaired section back, with a deliberate slack loop

Two to four weeks is typical, and the fleet of cable repair ships worldwide numbers a few
dozen. Availability of a ship, not the repair itself, is usually the long pole.

> **There is no faster way to do this**, and there is unlikely to be one. The redundancy is
> in having several cables, not in repairing one quickly.

## The chokepoints

The physical geography of the Internet is not uniform, and a few places matter enormously.

| Chokepoint | Why |
|---|---|
| **The Red Sea / Suez** | **almost all Europe–Asia cables** pass through it |
| **The Strait of Malacca / Luzon Strait** | Asian traffic concentration; typhoons and seismic activity |
| **The Strait of Gibraltar** | Mediterranean entry |
| **Cornwall, Brittany, Marseille** | European landing concentration |
| **Alexandria and the Egyptian land crossings** | **the Red Sea cables cross Egypt overland** |

**The Egyptian crossing is the sharpest example.** Cables from Asia to Europe enter the Red
Sea, land in Egypt, cross overland to the Mediterranean, and continue — so a large fraction
of Europe–Asia capacity depends on a corridor a few tens of kilometres wide, in one country.

In 2008, cuts near Alexandria and in the Mediterranean disrupted connectivity across the
Middle East and South Asia for days. In 2024, multiple Red Sea cable cuts did it again.

> The Internet's routing is redundant. Its geography is not, and the two facts are
> frequently confused. BGP will find another path if one exists; it cannot create capacity
> that was never laid.

**Other examples worth knowing:** Tonga in January 2022 lost its single cable to a volcanic
eruption and was substantially disconnected for weeks. Countries served by one cable are one
fishing trawler away from isolation, and there are more of them than there should be.

## Who owns them now

A structural change that happened quietly and matters.

| Era | Owner |
|---|---|
| 1858–1990s | **telegraph and telephone monopolies**, then consortia of carriers |
| 2000s | **carrier consortia** — a dozen operators sharing cost and capacity |
| **2010s–** | **content providers** — Google, Meta, Microsoft, Amazon |

Google alone has sole ownership of several transoceanic systems — Dunant, Curie, Grace
Hopper, Equiano — and is a partner in many more. Meta's 2Africa is, at about 45,000 km,
among the longest systems ever built.

Content providers now account for the majority of new transoceanic capacity, and the
reasoning is Chapter 48 §48.1's flattening carried to its conclusion: if you are the traffic,
owning the pipe is cheaper than renting it.

> The organisations that built the Internet's long-haul infrastructure are no longer
> telecommunications companies. Whether that is a concentration worth worrying about is a
> genuine question, and it is being asked in regulatory proceedings rather than in engineering
> ones.

## The latency arithmetic

Which is what the cable route actually costs you.

Light in fibre travels at $c/1.468 \approx 204{,}000$ km/s — about 4.9 µs per kilometre
(Chapter 6).

| Route | Cable length | **One way** | **Round trip** |
|---|---|---|---|
| London – New York | **~6,600 km** | **32 ms** | **65 ms** |
| London – Singapore | ~15,000 km | 74 ms | **147 ms** |
| **Great circle LHR–JFK** | 5,585 km | 27 ms | **55 ms** |

**Note the last row.** The cable is 18% longer than the great circle, because it avoids
seamounts, fishing grounds and other cables — and that detour is 10 ms of round-trip
latency that no equipment can recover.

Which is why route length is worth money. The Arctic and trans-Siberian projects, and the
proposed shorter Atlantic routes, exist because a few milliseconds is worth a great deal to
financial trading — and it is the same argument as Chapter 49 §49.4's LEO constellations,
which beat fibre on very long routes because vacuum is 47% faster than glass.

> You cannot buy latency below the route length, and the route length is a function of
> geography and where fish are caught.

## What breaks here

A cable cut, and traffic rerouting with higher latency. **Working as designed** —
Chapter 32's BGP found another path. The latency increase is the diversion, and it is
permanent until the repair.

**A cable cut, and no alternative path.** **Insufficient diversity.** For an island or a
single-cable country this is an infrastructure problem, not a routing one.

**"Diverse" circuits failing together.** They shared a landing station, a duct at the beach,
or a segment of the same cable. §50.3's shared risk link groups, at continental scale.
Verify against cable route maps and landing station names, not against circuit references.

**Latency higher than the geography suggests.** The cable route, not the great circle — and
possibly a diversion around a current fault.

**Repeated brief outages on one system.** A marginal repeater or a partially damaged section
awaiting repair. The operator knows; the customer usually does not.

**A regional outage with no local cause.** Check submarine cable status pages before
troubleshooting anything local. It is a five-second check that occasionally saves a day.

> **Network+ note.** Submarine systems are not directly examinable. The transferable content
> is: **fibre is the medium for long-distance transmission**, redundancy requires physical
> diversity and not merely logical diversity, and propagation delay is a function of
> distance and cannot be engineered away (Chapter 3 §3.1). The third is examined
> constantly, in disguise.
