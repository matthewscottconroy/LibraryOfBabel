# 56.3 Power, Cooling and the Physical Plant

What network engineers usually treat as somebody else's problem until the day it is not.

## Power

Every availability figure in §56.1 assumes the equipment has electricity, and that
assumption fails more often than the equipment does.

### The layers

```
   Grid ──▶ [ Transfer switch ] ──▶ [ UPS ] ──▶ [ PDU ] ──▶ equipment
              ▲                       ▲
         [ Generator ]         batteries: minutes
         starts in 10–30 s     bridge the gap
```

| Layer | Protects against | Duration |
|---|---|---|
| **UPS** | **transients, sags, brief cuts, and the gap while the generator starts** | **minutes** |
| **Generator** | **a sustained outage** | **hours to days, given fuel** |
| **Dual feeds** | **one supply path failing** | **indefinite, if genuinely separate** |
| **Dual PSUs** | one power supply failing | |

The UPS's job is not to run the network through an outage. Its job is to run it for the
ten to thirty seconds the generator needs to start and stabilise — and, where there is no
generator, to run it long enough for a controlled shutdown.

**Which changes the sizing question:** "how long will the UPS last?" is the wrong question if
there is a generator, and the only question if there is not.

### The arithmetic

UPS capacity is quoted in VA and the load is in watts, and they are not the same.

$$\text{watts} = \mathrm{VA} \times \text{power factor}$$

Modern equipment has a power factor near 0.95–1.0; a UPS is usually rated at 0.8–0.9.
So a "3,000 VA" UPS supplies about 2,700 W, and a 2,000 W load is at 74% of it.

**Runtime falls sharply with load, and non-linearly.**

| Load as % of capacity | Typical runtime |
|---|---|
| 25% | **~30 min** |
| 50% | **~12 min** |
| **75%** | **~6 min** |
| 100% | **~3 min** |

> A UPS loaded to 75% will not run your network for long enough to think. **Size for 40–60%
> of capacity**, which leaves headroom for growth and gives usable runtime.

**And batteries degrade.** Sealed lead-acid batteries last three to five years and lose
capacity throughout, faster in a warm room. A four-year-old UPS at nominal load may deliver
half its rated runtime, and nothing tells you this except a load test.

> A UPS that passes its self-test has passed a test of its electronics, not of its
> batteries. Self-tests draw a small load for a few seconds. Only a real load test, at
> real load, reveals the actual runtime, and it should be done annually.

### The things that catch people

**"Dual feeds" that are not.** Two power strips in a rack, fed from two circuits, fed from the
same distribution board — which fails as one. Trace the feeds to the board, and then to
the supply.

**Devices with one power supply.** A single-PSU switch in a dual-fed rack is a single point of
failure regardless of everything around it. And it is common — access switches frequently
have one, and the redundancy of the rack does not extend to them. Know which devices these
are, and if they matter, replace or duplicate them.

**Both PSUs on the same strip.** Trivially avoidable, routinely found.

**Overloaded circuits.** A 16 A circuit at 230 V supplies about 3.7 kW, and a rack of
modern equipment plus PoE can exceed it. The rack elevation should carry power draw
(Chapter 53 §53.2), and the sum should be checked before the device is ordered.

**PoE budget.** A switch with a 370 W PoE budget cannot run 48 access points at 25 W each.
Add the PoE load to the rack's power budget too — it is drawn from the switch, which draws
it from the circuit.

The generator that has never run under load. Monthly no-load starts prove the starter
battery works. They prove nothing about the generator's ability to carry the load, and
an annual full load test is the only thing that does.

**Fuel.** Generators run for as long as there is fuel and a contract to deliver more. A
72-hour tank is 72 hours only if the refuelling contract is real and the roads are passable —
which, in the events that cause multi-day power outages, is the assumption most likely to fail.

## Cooling

Heat is the primary cause of electronic ageing, and Chapter 4 §4.3 noted that thermal
noise rises with temperature — a hot cabinet performs measurably worse before it fails.

### How fast it goes wrong

Faster than people expect, and this is the number worth having.

A 5 kW load in a 30 m³ room with no cooling:

$$\frac{dT}{dt} = \frac{P}{m \, c_p} = \frac{5000}{36 \times 1005} \approx 0.14 \text{ K/s} \approx 8 \text{ K per minute}$$

> **Eight degrees a minute.** A room at 22 °C reaches 60 °C in under five minutes.

The calculation ignores the thermal mass of the equipment and the building, which slows it —
in practice a small comms room typically rises 1–3 °C per minute. The order of magnitude is
the point:

> A cooling failure becomes an outage in minutes, not hours, and **the monitoring must be
> fast enough to matter.** A temperature alert on a five-minute poll is a temperature alert
> that arrives after the shutdown.

### The arrangement

| | |
|---|---|
| **Target temperature** | **18–27 °C at the equipment inlet** (ASHRAE's recommended range) |
| **Humidity** | **too low: static discharge. Too high: condensation and corrosion.** |
| **Hot aisle / cold aisle** | **all equipment faces the same way in each aisle**, so cold air is drawn from one and exhaust discharged into the other |
| **Blanking panels** | **empty rack units let hot exhaust recirculate to the front.** **Cheap, and routinely absent.** |
| **Airflow direction** | Chapter 53 §53.2 — **a switch with the wrong airflow ingests its neighbour's exhaust** |

Blanking panels are the highest-value item on that list per pound spent. An unblanked rack
recirculates hot air to the intakes above the gap, and the equipment there runs several
degrees hotter than the room, which is invisible from a room temperature sensor.

**Redundancy in cooling** follows the same rules as everywhere else: N+1 units, on separate
circuits, and — the part that is missed — with the failure of one not exceeding the capacity of
the rest at peak load on the hottest day of the year.

## Environmental monitoring

**Cheap, and routinely absent.**

| Sensor | Detects | Cost |
|---|---|---|
| **Temperature** — several per room, at inlets | **cooling failure** | **trivial** |
| **Humidity** | condensation, static risk | trivial |
| **Water detection** — under floor, near pipes | **a leak, before it reaches equipment** | **trivial** |
| **Door contact** | **unauthorised access**, or a door propped open | trivial |
| **Smoke** | fire | required anyway |
| **Power draw per circuit** | **an overload before it trips** | via a metered PDU |
| **UPS state** | **on battery, battery low, battery replacement due** | via SNMP |

> The cost of a sensor is trivial against the cost of discovering a cooling failure the
> following morning, and the "following morning" scenario is the normal one in an unmonitored
> room.

**Two specifics worth insisting on:**

Alert on the UPS going onto battery, immediately. It is the earliest possible warning of a
power problem, and it is frequently the first sign of an issue in the building that nobody
else has noticed yet.

Alert on temperature rate of change, not only on absolute temperature. A room rising 5 °C
in ten minutes is a cooling failure, and it will trigger a rate alert long before it reaches
any absolute threshold.

## The things that are not electrical

A short list of physical failures that take out networks and are not in most designs.

**Water.** **From above** — a burst pipe, a blocked drain, a roof. Comms rooms are frequently
placed beneath toilets or plant rooms, and this is discovered once.

**Fire suppression.** A gas discharge is extremely loud, and the acoustic energy has
caused hard drive failures in documented incidents. It is also a mandatory evacuation, so
nobody is in the room afterwards.

**Physical access.** Anyone in the comms room can unplug anything. A locked room with a
logged door contact is a control, and an unlocked cupboard in a corridor is common.

**Cable management.** A rack where cables must be disturbed to reach a device is a rack where
an unrelated service goes down during maintenance.

**Vibration and seismic.** Relevant in some regions and in some buildings — near lift
machinery, near railways.

**Rodents.** Genuinely — they eat cable insulation, and it is a recognised cause of outages
in older buildings.

## What breaks here

**Equipment down and the UPS "worked".** It ran for four minutes and the generator did not
start. Load-test the generator annually.

A UPS that passed its self-test and lasted 90 seconds. **Battery degradation.** Only a real
load test reveals it.

Both power supplies down on a "dual-fed" device. Same board, or both on the same strip.
Trace the feeds physically.

A single-PSU switch taking out an access layer. Redundancy in the rack does not extend to
devices with one supply.

A circuit tripping when a new device is added. The rack's power draw was never summed.
Chapter 53 §53.2's elevation.

Equipment hot in a room at 21 °C. **Recirculation** — missing blanking panels, or wrong
airflow direction. Measure at the inlet, not in the room.

**A cooling failure discovered the next morning.** **No environmental monitoring.** The sensor
costs less than one hour of the resulting outage.

**A leak reaching equipment.** **No water detection**, and the room is under something wet.

A room that cannot be entered during an incident because the access is controlled by a system
that is down. **Genuinely happens**, and the fix is a physical key held somewhere sensible.

> **Network+ note.** Objective 3.3 covers power and environmental factors. Over-learn: a UPS
> provides short-term power and a generator long-term; **PDUs distribute power in a rack**;
> **hot aisle/cold aisle improves cooling efficiency**; **HVAC failure causes rapid
> overheating**; and **environmental sensors monitor temperature, humidity and water.** The
> UPS-versus-generator distinction is examined regularly.
