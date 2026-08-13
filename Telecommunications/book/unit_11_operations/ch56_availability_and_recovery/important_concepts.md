# Chapter 56 — Important Concepts

**Five nines is 5.26 minutes a year, including all planned maintenance** *(§56.1)* — **That
clause defeats most claims.** An organisation that reboots its firewalls quarterly has spent its
annual budget several times over before anything failed. **Always ask whether the target
excludes planned windows** — "99.999% excluding scheduled maintenance" is achievable and is not
what the listener heard.

**Each nine costs roughly an order of magnitude more, because each removes a different class of
cause** *(§56.1)* — **The step from 99.9% to 99.99% is where automation replaces people**,
because 43 minutes a month permits waking a human and 4 minutes does not. **The step to 99.999%
is where the change process becomes the limiting factor**, since 5 minutes a year does not
permit a failed change — **which is why the highest-availability systems change slowly, and why
that is a cost rather than a virtue.**

**The question is not how many nines but what an hour costs** *(§56.1)* — For a site turning
over £2,000/hour, **99% → 99.9% saves £157,000 for £3,600 and is obviously right; 99.9% →
99.99% saves £15,768 for £40,000 and is obviously wrong** — **at that site.** At £40,000/hour
the same step saves £315,000. **The deciding number is a business figure, and an engineer who
designs without it is guessing.** **Frequently the right answer is three nines with a fast,
rehearsed recovery.**

**Halving MTTR beats doubling MTBF, and costs far less** *(§56.1)* — $A =
\mathrm{MTBF}/(\mathrm{MTBF}+\mathrm{MTTR})$. **Taking repair time from four hours to thirty
minutes moves a device from 99.95% to 99.99% — a full nine** — **and the mechanisms are
documentation, spares, monitoring and rehearsal rather than better hardware.** **Which reframes
Chapters 53 to 55: they are all MTTR reductions, and MTTR is where the leverage is.**

**Four questions before an availability figure means anything** *(§56.1)* — **Available to
whom** (measure from the user, not the data centre); **available for what** (a health check that
passes while the service errors is not a measurement); **how does partial count** (user-minutes
lost, which correctly ranks "everyone for 5 minutes" against "10% for an hour"); **and is
planned work included.**

**An error budget converts an argument into arithmetic** *(§56.1)* — **If the target is 99.9%,
0.1% of failure is budgeted and spending it is permitted.** Budget remaining → ship. Budget
exhausted → freeze. **It replaces "no changes" versus "we must ship", where both positions are
reasonable and neither is falsifiable.** **And an unspent budget is information**: a team
achieving 99.99% against a 99.9% target is over-investing.

**Four components at high availability produce a system below the weakest** *(§56.1)* — Series
multiplies. **A 99.9% circuit dominates three 99.99% devices, and improving the devices achieves
nothing.** **Find the weakest term before spending anything.**

**Components that share a fate are not independent** *(§56.2)* — **The most important idea in
the chapter**, and the parallel availability formula assumes exactly what is usually false.
Same circuit, same rack, same duct, same building entry, **same firmware defect**, same
configuration error, same substation, **same carrier**, same control plane, **same engineer.**

**"Two providers" frequently means one physical circuit** *(§56.2)* — **Wholesale is normal**,
and a customer buying from two retailers may be buying the same tail twice. **Ask for the
physical path, not the provider's name.**

**A shared control plane is the cloud-era shared power feed** *(§56.2)* — **Two availability
zones that fail independently at the compute layer may share an authentication or metadata
service**, and the outage takes both out while every component in each remains healthy.

**Document what you decided to accept** *(§56.2)* — "The two fibres share a duct for 400 m; a
second entry costs £180,000; accepted, reviewed annually" is a good record. **The failure is not
accepting a risk; it is accepting one without knowing.**

**A backup component that has never been exercised is of unknown status** *(§56.2)* — The
standby with drifted configuration, the generator never load-tested, the LTE SIM deactivated for
non-use, **the standby that has been down for months.** **Nothing monitors the component that is
not carrying traffic**, because it looks identical to a healthy idle one.

**Fail the primary; do not merely fail over** *(§56.2)* — **A graceful administrator-initiated
switchover exercises a different path from a power cut.** Run in production, stay on the standby
for a working day, **fail back deliberately and time it** (failback is harder and almost never
rehearsed), and **exclude the person who built it.**

**The virtual MAC moves as well as the virtual IP** *(§56.2)* — Which is why **from the host's
position, nothing happened**: no ARP cache change, no gratuitous ARP to act on.

**HSRP's ten-second defaults drop voice calls** *(§56.2)* — VRRP defaults to 1 s hello / 3 s
hold; HSRP to 3 s / 10 s. **TCP survives ten seconds; a call does not**, and this is a common
and quiet cause of "the failover works and calls drop." **Tuning down costs sensitivity to
transient loss — a flapping FHRP is worse than a slow one — so BFD is the better answer where
supported.**

**Without tracking, the FHRP master keeps the role after its uplink fails** *(§56.2)* — **A
black hole produced by redundancy working exactly as specified.** **Track something that
reflects forwarding capability**, not link state — a link that is up to a switch that has lost
its own uplink is still "up".

**Preemption on, with a delay** *(§56.2)* — Without one, **a router that has just booted takes
the active role and black-holes traffic for thirty seconds while OSPF converges** — a
self-inflicted outage caused by recovering.

**Align the FHRP master with the spanning tree root** *(§56.2)* — When they are on different
devices, **all inter-VLAN traffic crosses the inter-switch link twice**, and the symptom is a
peer link far busier than the design predicts with no other explanation. **With several VLANs,
alternate them per VLAN** to balance and keep each path direct.

**A UPS's job is to bridge to the generator** *(§56.3)* — Ten to thirty seconds — **or, with no
generator, to permit a controlled shutdown.** "How long will it last?" is the wrong question in
the first case and the only question in the second.

**Runtime falls non-linearly with load** *(§56.3)* — **~30 min at 25% capacity, ~6 min at 75%.**
**Size for 40–60%.** And **batteries lose capacity throughout a three-to-five-year life**, faster
in a warm room. **A self-test tests the electronics, not the batteries** — only an annual load
test at real load reveals runtime.

**"Dual feeds" that trace back to one board fail as one** *(§56.3)* — **And a single-PSU switch
in a dual-fed rack is a single point of failure regardless of everything around it.** Know which
devices those are.

**Add PoE to the rack's power budget** *(§56.3)* — It is drawn from the switch, which draws it
from the circuit. **A 16 A 230 V circuit supplies 3.68 kW**, and a rack of equipment plus PoE
can exceed it.

**A cooling failure becomes an outage in minutes** *(§56.3)* — **5 kW in a 30 m³ room is about
8 K per minute in theory and 1–3 K in practice.** **A temperature alert on a five-minute poll
arrives after the shutdown.** **Alert on rate of change**, not only on absolute temperature — a
room rising 5 °C in ten minutes is a cooling failure long before it crosses any threshold.

**Blanking panels are the best value per pound in a rack** *(§56.3)* — **An unblanked rack
recirculates hot exhaust to the intakes above the gap**, and that equipment runs several degrees
hotter than the room — **invisible from a room sensor.** Measure at the inlet.

**Alert on the UPS going onto battery, immediately** *(§56.3)* — **The earliest possible warning
of a power problem**, and frequently the first sign of a building issue nobody else has noticed.

**RPO points backwards, RTO points forwards, and they are independent** *(§56.4)* — **A system
can have an RPO of zero and an RTO of a week** — perfect replication, no hardware to run it on.
**RPO is a replication cost; RTO is a standby-capacity and rehearsal cost.**

**Uniform targets across all systems mean nobody did the analysis** *(§56.4)* — "RPO 1 hour, RTO
4 hours, for everything" **over-protects the wiki and under-protects the finance database**, and
both cost money. **Derive RPO from whether lost work can be reconstructed; derive RTO from the
cost per hour and from how long the business can operate without it** — which are not the same
thing.

**Synchronous replication is a metropolitan-distance technology** *(§56.4)* — **Every write pays
the round trip: about 1 ms at 100 km, 10 ms at 1,000 km**, so **2,000 sequential writes at
1,200 km adds 23 seconds to a transaction.** **A DR site 20 km away is a real design** — far
enough for most events, close enough for synchronous writes — **and the trade against it is
regional events.**

**Stretched Layer 2 for DR is the option that looks easiest and is worst** *(§56.4)* — **It
couples two sites into one broadcast domain**: one spanning tree, one storm, one failure —
**which defeats the purpose of having two sites.** The industry has learned this repeatedly and
it is still proposed. **Prefer separate subnets with a low DNS TTL, or announcing your own
prefix from either site.**

**Size the DR site for the traffic, not for the equipment** *(§56.4)* — **A warm site with a
100 Mb/s circuit cannot serve what the primary served over 10 Gb/s**, and this is discovered
during the test, if there is one.

**A DR plan that has never been executed is a document, not a capability** *(§56.4)* —
Walkthroughs and tabletops are worth doing and **do not establish that recovery works.** **A
test in which someone "just checks" the standby the day before is a test of a system maintained
for the test.** **A test that surfaces twelve problems is a successful test; one that surfaces
none was not a test.**

**A failover test that reveals problems must be treated as a success, or it will be the last
one** *(§56.4)* — **If finding twelve issues produces criticism, the next test will be arranged
so as not to find any.**

**"Who decides this is a disaster" is the item most often missing and most costly** *(§56.4)* —
**An hour spent establishing whether to invoke DR is an hour of the RTO**, and the decision is
frequently owed by someone asleep and unreachable. **Name a role, name a deputy, state the
criteria.**

**Nothing works before the network, DNS and authentication do** *(§56.4)* — **The restoration
order is the network engineer's contribution to the DR plan**, and an organisation that starts
by restoring its finance database into a site with no DNS has spent its RTO learning about
dependencies.
