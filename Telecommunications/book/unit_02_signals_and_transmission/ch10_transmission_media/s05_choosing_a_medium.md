# 10.5 Choosing a Medium

The section this chapter exists for. In practice nobody asks you the bandwidth of
Cat6a; they ask you how to connect the warehouse.

## The procedure

Six questions, in order. Each one eliminates options, and answering them in this
order avoids the common failure of choosing a medium and then justifying it.

### 1. How far?

The single most eliminating question, because distance limits are hard.

| Distance | What survives |
|---|---|
| < 7 m | Everything, including direct-attach copper |
| < 30 m | Everything, including Cat8 |
| < 55 m | Cat6 at 10 Gb/s, everything below |
| < 100 m | All twisted pair at its rated speed |
| 100 m – 400 m | Multimode fibre (grade-dependent), wireless |
| 400 m – 2 km | OM4/OM5 at low rates, single-mode, wireless |
| 2 km – 40 km | **Single-mode fibre**, licensed microwave |
| 40 km – 80 km+ | Single-mode with appropriate optics; amplified beyond |

**Over 100 m rules out twisted pair.** Not "makes it marginal" — rules it out.
There is no supported way to run copper Ethernet past 100 m, and the extenders that
claim otherwise are either media converters in disguise or non-standard products
that will cause trouble.

This alone answers a large fraction of real questions.

### 2. What rate, now and in the life of the installation?

Note the second clause. Horizontal cabling lasts fifteen years or more; the
equipment at its ends is replaced three or four times in that period.

Design for the rate you can foresee, and note that the foresight is usually poor —
which argues for headroom rather than for precision. Chapter 5 §5.3's observation
applies: specify **bandwidth**, not a rate, and let the encoding of the day extract
what it can.

### 3. What is the electrical environment?

| Environment | Implication |
|---|---|
| Ordinary office | U/UTP is fine |
| Alongside power runs, lift motors, HVAC | Consider shielded, or separate the routes |
| Industrial: drives, welding, contactors | Shielded copper or **fibre** |
| Between buildings | **Fibre, always** — see below |
| Outdoors | Fibre, or outdoor-rated shielded copper with surge protection |

**Between buildings, use fibre, and the reason is not bandwidth.** Two buildings
have separate earthing systems at potentially different potentials, and a copper
conductor between them carries the difference as current. A lightning strike on or
near one building induces a surge that copper conducts directly into the other
building's equipment — destroying switches at both ends and occasionally starting
fires.

Fibre is a **dielectric**. It carries no current, conducts no surge, and bonds no
earths. This is a safety argument before it is a networking one, and it overrides
cost.

If copper between buildings is unavoidable, it requires surge protection at both
ends and a proper bonding design, and you should expect to replace equipment
occasionally anyway.

### 4. Does the far end need power?

This question decides more installations than it is given credit for, and it did not
exist before 2003.

**Copper can deliver up to 90 W** (Chapter 10 §10.1's PoE table). Fibre cannot
deliver any.

An access point, camera, IP telephone, door controller or sensor at the end of a
copper run needs no local outlet, no electrician, and no small power supply that
will fail in five years. The same device on fibre needs all three, plus a media
converter that also needs power and also fails.

For 22 cameras and 12 access points, this consideration alone frequently decides
against fibre for runs that are within copper's distance limit.

### 5. What is the cost, honestly?

Count all of it:

| Cost | Copper | Fibre | Wireless |
|---|---|---|---|
| Material per metre | Low | Low | — |
| Installation labour | Similar | Similar | **Low** |
| Trenching or containment | Same | Same | **None** |
| Termination | Cheap, fast | Requires skill and equipment | — |
| Transceivers | Included | **Extra, per port** | Included |
| Power at the far end | **Free (PoE)** | Needs provision | Needs provision |
| Ongoing | Nil | Nil | Spectrum risk |

The costs that surprise people: **fibre transceivers**, which are per-port and
recur at every equipment refresh; and **trenching**, which dominates any outdoor
run and is why wireless wins across a car park and loses across a campus where
ducts already exist.

### 6. What are the non-technical constraints?

- **Wayleave and permissions.** Can you dig? Across whose land?
- **Listed building or fabric restrictions.** Can you drill?
- **Time.** Fibre across a business park is months; a radio link is days.
- **Skills.** Will the team who maintains it be able to terminate fibre at 2 a.m.?
- **Regulation.** Licensed spectrum needs an application.

Chapter 72 §72.1's point applies: **design for the team you have.** A technically
optimal solution the organisation cannot maintain is not optimal.

## Worked decisions

**A. Office desk outlets, 40 m, 1 Gb/s now.**
→ **Cat6a U/UTP.** Distance trivial; PoE likely wanted for phones and future
devices; the material premium over Cat5e is small against fifteen years and
identical labour. Not fibre: no distance need, transceivers per port, no PoE.

**B. Between two buildings, 240 m, across a yard with three-phase machinery.**
→ **Single-mode fibre.** Over 100 m rules out copper outright, and even at 90 m the
inter-building earthing and lightning argument would decide it. Single-mode rather
than multimode because the transceiver premium is small for one or two links and the
plant then supports any future rate. Note the consequence: **anything at the far end
needs local power**, which must be in the design.

**C. Warehouse handheld scanners.**
→ **Wireless, necessarily.** The endpoints move. The design work is entirely
Chapter 45's — coverage, roaming, and the metal racking — and the wired part is
getting cabled backhaul to the access points, which is decision A again.

**D. Top-of-rack to server, 2 m, 25 Gb/s, forty-eight of them.**
→ **Direct-attach copper twinax.** Cheaper than optics, lower power, no transceiver
to fail, and 2 m is comfortably within reach. Fibre would work and costs more for no
benefit at this distance.

**E. Across a public road to a building 180 m away, needed in three weeks.**
→ **Licensed or unlicensed point-to-point wireless.** Fibre requires wayleave across
a public highway, which is a permit process measured in months and a cost measured
in tens of thousands. The radio link costs a few thousand and is up in days. Revisit
in two years if the traffic justifies fibre and the permits are obtainable — and
note in the design document that this is a deliberate deferral, per Chapter 72
§72.4.

**F. Campus backbone, six buildings, ducts already exist, 15-year horizon.**
→ **Single-mode fibre, and install more strands than you need.** The trenching is
done; the marginal cost of additional fibres in the same duct is almost nothing;
and the one thing you will not want to do in eight years is dig again. Chapter 9
§9.4's economics — the path costs, the capacity is cheap — applied at campus scale.

## The two rules worth carrying

**Install the best medium you can terminate properly**, because the labour is the
cost and it is spent either way.

**And state the condition under which the answer changes.** "We chose Cat6a rather
than fibre because all runs are under 90 m and 34 devices require PoE; if the
warehouse extension proceeds and any run exceeds 100 m, that run becomes fibre with
local power provisioned." That sentence is what makes a design document useful to
whoever inherits it, and it is what Chapter 72 §72.4 marks.

## What breaks here

**Choosing a medium before answering the six questions**, and then justifying it.
Visible in a design document as a justification that mentions only the chosen
option.

**Forgetting the power question**, and discovering at installation that 22 cameras
need 22 electrical outlets.

**Copper between buildings.** Works, and then a storm destroys equipment at both
ends.

**Multimode installed for a 300 m run at 1 Gb/s**, which fails when the rate rises
to 10 Gb/s and the run exceeds the grade's reach.

**A wireless link chosen for cost and specified without a rain margin.**

> **Network+ note.** Objective 1.5's media selection is exactly this section, and
> the exam's scenario questions supply a distance, an environment and a rate and ask
> for a medium. Working the six questions in order produces the answer reliably, and
> the distance question alone resolves most of them.
