# Chapter 70 — Important Concepts

The consistency argument is the real one; the time argument is weak *(§70.1)* — Time can be
answered with a script or more people. **Consistency cannot**: at a 1% per-device error rate,
the probability that a change applied by hand to 200 devices is correct everywhere is 13%.

And the errors are invisible *(§70.1)* — The 198 devices configured correctly work; the two
that were not work too, until the circumstance that exercises the difference arrives.
Chapter 55 §55.1's drift, created deliberately, one typo at a time.

A configuration file records what a device is configured to do, not what anyone wanted
*(§70.1)* — `ip route 10.9.0.0 …` is a fact; whether it should be there is not in the file.

The CLI was designed for a human *(§70.1)* — Prose errors, output formats that change
between releases, partial application, and a prompt instead of a state query. Screen-scraping
works and is fragile: a vendor changes a column heading and the automation breaks silently, and
the failure mode is that the script believes it succeeded.

Automation multiplies the consequences of a decision in both directions *(§70.1)* — The
organisations with the worst automation incidents did not automate badly; they automated a bad
decision efficiently.

Repeatability is the property that changes what is possible *(§70.1)* — When rebuilding a
device is a two-minute operation, a corrupted device is replaced rather than repaired, and
Chapter 63's diagnostic effort on a single failed device becomes optional.

The step from "versioned" to "generated" is organisational *(§70.1)* — It requires the team
to stop making changes on devices, which is a habit rather than a capability. The
organisations that succeed start with the read-only half; the ones that fail start by automating
a routing change.

An automation system with administrative access to every device is the highest-value target in
the estate *(§70.1, §70.2)* — and it is frequently protected less carefully than the devices
it manages.

YANG makes the interface a contract rather than a convention *(§70.2)* — **Typed,
constrained, hierarchical**, with defaults stated so "not configured" has a defined meaning —
and three protocols share one model, so the understanding transfers.

`config false` is the distinction the CLI never made *(§70.2)* — `show interface` mixes
configuration and state in one text blob, and separating them is what lets a tool ask "what did
I configure?" and "what is actually happening?" as different questions.

The standard models cover a fraction, the vendor models do not transfer, and OpenConfig sits
between *(§70.2)* — Most real automation uses a mixture, and the mixture is a maintenance
burden.

A candidate datastore means a change is assembled completely and committed atomically
*(§70.2)* — Which the CLI cannot do, because each line takes effect as it is typed (Chapter
55 §55.2's point-of-no-return). And `commit confirmed` is a protocol primitive — the highest
value habit in this book, available in the interface.

NETCONF's `lock` removes a real failure mode *(§70.2)* — Two engineers configuring one
device simultaneously, which was possible for forty years and is an ordinary database property.

RESTCONF trades transactions for accessibility, and the transaction is the significant half
*(§70.2)* — A RESTCONF change takes effect immediately, per resource, reintroducing the
partial-application problem NETCONF solved. Excellent for reading; wrong for a multi-part change.

gNMI: one interface, one model, one credential, for configuration and telemetry *(§70.2)* —
A genuine simplification over SNMP for monitoring, NETCONF for configuration and a CLI for
everything else — and its limitation is model coverage, which varies between vendors despite
OpenConfig.

Idempotence is what makes automation safe to re-run *(§70.3)* — Applying the same desired
state repeatedly produces the same result, which is why a declarative tool can run on a
schedule to correct drift and an imperative script cannot. Burgess's convergent configuration,
from 1993 (Chapter 55's reading).

`replaced` and `overridden` eliminate drift, and are the ones people avoid *(§70.3)* —
Because removing configuration nobody understands is frightening. Which means most Ansible
network automation uses `merged` and adds without ever removing — reproducing exactly the
accumulation problem it was meant to solve.

Ansible knows what it did; it does not know what exists *(§70.3)* — Fine for "ensure this is
configured", inadequate for "this should no longer exist."

`terraform plan` converts "what will this change do?" from a judgement into a computation
*(§70.3)* — Chapter 55 §55.2's change record, generated automatically and accurately, and
a plan showing an unexpected destruction has caught more errors than any review process.

Terraform's state file is its distinguishing feature and its principal liability *(§70.3)* —
It contains secrets, must be shared with locking, can drift from reality, and losing it means
Terraform no longer knows what it owns.

Terraform manages resources with a lifecycle; Ansible configures things that already exist
*(§70.3)* — A network estate has both, and most mature practice uses both, which is untidy
and correct.

Automation without a source of truth is a faster way to apply whatever someone typed into a
variable file *(§70.3)* — The data lives once, and the switch configuration, the DHCP
scope, the DNS record, the monitoring configuration and the firewall object are all generated
from it. Model the network in NetBox before writing any templates, because the modelling
exercise reveals what you do not know.

Logic belongs in the data or in the code, not in the template *(§70.3)* — The commonest
template failure is not a bug; it is a template nobody understands because the logic that should
have been in the data model migrated into it.

A pipeline produces all six of Chapter 55 §55.2's change record elements as a by-product
*(§70.4)* — The diff, the commit message, the plan, the tests, the revert and the review —
which stops the discipline depending on the engineer's diligence at 02:00.

A unit test that asserts BPDU guard on access ports and not on uplinks runs in a second and
catches a template error before it reaches two hundred switches *(§70.4)* — Chapter 62
§62.4's hardening checklist, enforced by CI, which is the only way it stays enforced.

A virtual topology catches syntax, logic and gross topology errors and not platform behaviour
or scale limits *(§70.4)* — "We tested it in the lab" is a verification claim, and the
lab's coverage should be known rather than assumed.

Batfish makes "can the guest network reach the finance servers?" a test that runs on every
commit *(§70.4)* — The capability that most distinguishes network CI from a script that
applies configuration, and it is free and rarely used.

Automation trades fewer errors for larger blast radii *(§70.4)* — And the worst incidents
in the industry are all the same shape: a correct automation system, applying an incorrect
change, quickly, everywhere. No tooling prevents it and staged deployment bounds it.

AIOps is statistics on operational data, with better tooling and a better name *(§70.4)* —
Correlation, clustering, forecasting and anomaly detection are genuinely useful and were
possible before. Automated root cause analysis correlates and does not diagnose; predictive
failure works for components with wear characteristics and not for configuration; and autonomous
operation does not exist.

Three structural reasons the harder claims fail *(§70.4)* — The training data does not
exist (a well-run network produces very few labelled failures — the outcome you wanted and the
data you lack); every network is different so a model transfers poorly; and the incidents
that matter are novel, so a system that learned from the last hundred is prepared only for one
that resembles them.

"Anomaly detected, confidence 0.87" is not actionable *(§70.4)* — "Interface Gi1/0/14's
error rate is 40 standard deviations above its 30-day baseline, beginning at 14:07" is. The
ability to show why is what separates useful products from unusable ones.
