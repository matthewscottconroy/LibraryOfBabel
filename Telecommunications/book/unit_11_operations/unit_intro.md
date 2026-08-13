# Unit XI — Operating a Network

There is a moment, familiar to everyone who has built something, when the last cable
is seated and the last configuration is committed and traffic begins to flow, and it
feels like the work is finished.

It is not finished. It has just started, and everything up to this point was the
cheap part.

A network runs for ten or fifteen years. Over that period it will be modified several
hundred times by several dozen people, most of whom will not have been present when it
was designed. Equipment will reach end of support. Firmware vulnerabilities will be
published. Someone will make a change at 22:00 that seems minor. A cable will be
disconnected by a cleaner. The person who understood the addressing scheme will leave.

**The total cost of a network is dominated by its operation, and the operational cost
is dominated by how well it was documented and how disciplined its changes are.**
That is not an inspiring statement and it is true, and this unit is deliberately
expanded beyond what an introductory syllabus usually contains, because operations is
19% of the Network+ blueprint and considerably more than 19% of the actual job.

## The four questions

**"What do we have, and how is it connected?"** (Chapter 53.) Documentation, diagrams
at three levels of abstraction, inventory, cable maps, and address management. The
unglamorous answer to most incidents is that somebody could not find out what was
supposed to be true.

**"Is it working, and how would we know?"** (Chapter 54.) Monitoring, baselines,
SNMP, syslog, flow records, and streaming telemetry. Also — and this receives more
attention here than is usual — **alert design**, because a monitoring system that
generates alerts nobody reads is worse than none at all: it consumes effort and
provides false assurance.

**"How do we change it without breaking it?"** (Chapter 55.) Configuration
management, change control, firmware lifecycle, backups, and drift detection. The
overwhelming majority of unplanned outages are caused by planned changes, and this is
so consistently true across every organisation that measures it that it should be
treated as a law rather than an observation.

**"What happens when it breaks anyway?"** (Chapter 56.) Availability arithmetic,
redundancy and its failure modes, first-hop protection, the physical plant, and
disaster recovery with RPO and RTO defined properly.

## The recurring theme

One idea threads through all four chapters, and it is the most useful thing in the
unit: **the value of an operational practice is measured by how much it reduces the
cost of the next incident.**

Documentation is not virtue. It is a cache: work done once, in calm conditions, so
that it does not have to be done repeatedly under pressure. A network diagram that is
current saves twenty minutes at 03:00; a diagram that is two years out of date costs
forty, because someone will trust it before discovering it is wrong.

Monitoring is not surveillance. It is the difference between learning of an outage
from your customers and learning of it from a graph, and the difference between those
two is measured in reputation.

Change control is not bureaucracy. It is the mechanism by which a change that goes
wrong can be identified as the cause within minutes rather than hours, because
somebody wrote down what was changed and when.

Every practice in this unit should be able to answer the question *what incident does
this make cheaper?* Practices that cannot answer it are ceremony, and there is plenty
of ceremony in this field.

## A note on what this unit does not do

It does not tell you which monitoring product to buy. The market changes every few
years, the products are largely interchangeable in their fundamentals, and the
decision is usually made on price and existing relationships rather than on capability.

What it does instead is establish what any such system must do — what a baseline is
and why you need one before you have a problem, what SNMP actually retrieves and what
its versions do and do not protect, why flow data answers questions that interface
counters cannot, and how to design a threshold that fires when something is wrong and
stays quiet otherwise.

Those are durable. The product you use in 2031 will be a different product, and it
will still be answering the same questions.
