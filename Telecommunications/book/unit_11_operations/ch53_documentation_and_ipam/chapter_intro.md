# Chapter 53 — Documentation and IPAM

It is 03:14. A significant part of the network is down. You have been awake for four
minutes and you are looking at a rack you have never seen, in a building you have
visited twice, containing forty-eight patch cables of which none is labelled.

Everything in this chapter exists for that moment.

Documentation is unglamorous, it is skipped under deadline pressure, and it is the
single highest-leverage operational investment available — because its entire value is
realised at exactly the moments when thinking is hardest and mistakes are most
expensive.

The framing that makes it worth doing: **documentation is a cache.** It is work
performed once, calmly, with the equipment in front of you and the reasoning fresh, so
that it does not have to be performed repeatedly under pressure by people who lack
that context. Like any cache it has a hit rate and a staleness problem, and both are
manageable.

## The three diagrams

The most common documentation mistake is drawing one diagram containing everything.
It becomes unreadable, it is impossible to keep current, and it answers no question
well. The standard practice is three, each answering a different question.

**The physical diagram (L1)** answers *what is plugged into what?* Devices, their
locations, ports, cable types and identifiers, patch panel positions, and the length
and route of anything that leaves a room. This is the one you want at 03:14. It has no
IP addresses on it and no VLANs; it is about copper, glass and rack units.

**The logical diagram (L2)** answers *what talks to what, and in which segment?*
VLANs, trunk links, spanning tree topology with the root bridge and blocked ports
identified, link aggregation groups. This one may bear little resemblance to the
physical layout, which is exactly Chapter 11 §11.3's point.

**The routed diagram (L3)** answers *how does traffic get between segments?*
Subnets and their prefixes, gateway addresses, routing protocols and areas, WAN
circuits with their providers and reference numbers, firewalls and their zones.

Each fits on a page. Each is comprehensible without the others. Together they cover
what any incident will ask, and separating them is what makes each maintainable.

§53.1 also covers the standard symbol conventions, and — more usefully — what to leave
*off*. A diagram showing every workstation is not a diagram; it is a database
rendered badly. Diagrams show structure; databases hold instances.

## Labelling and inventory

§53.2 covers the parts nobody enjoys.

**Cable labelling** at both ends, with a scheme that encodes location rather than
purpose — purpose changes, location does not. A label reading `A-3-14 → B-1-07` is
still correct in five years; a label reading `Accounts printer` will be wrong within
eighteen months and will actively mislead.

**Port and patch records**, mapping wall outlet to patch panel position to switch
port. This is the record that turns "which port is this device on" from a
twenty-minute exercise into a lookup, and it is what makes port-based security and
tracing practical.

**Rack elevations**, showing what occupies which rack unit, with power draw and
weight. Boring, until you need to know whether a new device fits and whether the
circuit can carry it.

**Asset inventory**: model, serial, purchase date, support contract expiry, current
firmware, and — critically — **end-of-support date**, which Chapter 55 turns into a
planning input. An inventory that does not carry EOL dates cannot answer the only
strategic question anyone asks of it.

## Address management

§53.3 covers IPAM, which is Chapter 27's address plan made operational.

The plan is a design document. IPAM is the live record: which subnets exist, which
addresses within them are allocated, to what, by whom, and when. At small scale a
spreadsheet is genuinely adequate and pretending otherwise is affectation. At larger
scale, dedicated tooling integrated with DNS and DHCP removes an entire class of
error, because the three records are then guaranteed consistent by construction rather
than by discipline.

The failure this prevents is specific and common: a static address assigned from
inside a DHCP pool, producing an intermittent conflict that appears weeks later when
the pool reaches that address, and that presents as a mysterious connectivity problem
affecting two unrelated machines at unpredictable times.

## Runbooks

§53.4 covers the documentation that is most often absent and most valuable during an
incident: **what to do**, written down, for the situations you can anticipate.

A good runbook is specific and executable. Not "investigate the VPN issue" but: here
is the command that shows tunnel state, here is what healthy output looks like, here
are the three most common causes in order of frequency, here is the command to restart
the service and the expected disruption, here is who to escalate to and their number.

The test of a runbook is whether someone competent but unfamiliar can execute it at
03:00 without waking anyone. Most fail that test because they were written by the
person who already knew, who therefore omitted everything that was obvious to them.

Two habits worth adopting from the start of a career: write the runbook **during** the
first incident, while the knowledge is being acquired and the gaps are visible; and
after every significant incident, ask what documentation would have made it shorter,
and then create it. That single practice compounds over a career more than any
technical skill in this book.

## What breaks here

- **Stale documentation**, which is worse than none, because it is trusted. The fix is
  a review trigger tied to the change process (Chapter 55), not to a calendar.
- **The undocumented change** made during an incident and never recorded, which becomes
  the mystery in the next incident.
- **Purpose-based labels** that go wrong silently when purposes change.
- **The single point of knowledge** — one person who understands the addressing, the
  firewall policy, or why that odd static route exists. This is an availability risk
  in the same category as a single power feed, and it should be treated as one.

## By the end you will be able to

- Produce the three diagrams for a described network and state what belongs on each.
- Design a labelling scheme that survives changes of purpose.
- Specify what an asset inventory must record and justify each field operationally.
- Explain what IPAM prevents that a spreadsheet plan does not.
- Write a runbook that passes the 03:00 test.
