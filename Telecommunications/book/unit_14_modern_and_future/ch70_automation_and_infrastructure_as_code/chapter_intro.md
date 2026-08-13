# Chapter 70 — Automation and Infrastructure as Code

Two hundred switches need a new VLAN.

You can do this. You have done it. It takes a day, or two, and you are careful, and
you keep a list of which ones you have done, and somewhere around switch 140 you make a
typo that nobody notices for six weeks. Then you do it again next month for a different
VLAN.

The problem is not that the work is boring, though it is. The problem is that this way
of working has four properties that make a network progressively harder to operate, and
they compound.

**No record.** The configuration exists on the device and nowhere else. The reason it
is that way is in somebody's memory, and that person's memory is not backed up.

**No review.** Nobody checked your commands before you typed them into a production
device. In every other part of the software industry this would be considered
extraordinary.

**No testing.** You find out whether the change works by making it, in production, and
watching.

**No reproducibility.** Rebuilding an identical device requires someone to remember, or
to copy a configuration whose provenance is unknown.

Compare how software has been developed since roughly 2005: source in version control,
changes proposed as reviewable diffs, automated tests before merge, deployment by
pipeline, and a complete history of what changed and why. **Infrastructure as code is
the application of that discipline to network configuration**, and it is the natural
destination of the practices in Chapter 55.

## Imperative and declarative

§70.3 covers the distinction, which is the conceptual core of the chapter.

**Imperative** automation describes *steps*: connect, enter configuration mode, create
VLAN 200, name it, exit, save. A script. It is better than typing — it is repeatable
and fast — and it inherits a real weakness: it assumes a starting state. Run it against
a device where VLAN 200 already exists and the behaviour depends on the device.

**Declarative** automation describes *the desired end state*: this device has VLANs
100, 200 and 300 with these names. The tool determines the current state, computes the
difference, and applies only what is needed. Running it twice changes nothing the second
time — the property called **idempotence**, and it is what makes automation safe to run
repeatedly and therefore safe to run continuously.

Declarative configuration also gives you something imperative cannot: **the file is the
truth.** Not a record of what you did to the device, but a statement of what the device
must be. Drift from it (Chapter 55 §55.1) becomes detectable automatically, because
drift is simply a non-empty diff.

Ansible is largely declarative in its network modules; Terraform is thoroughly
declarative and maintains explicit state; and the direction of the industry is firmly
this way.

## The interfaces

§70.2 covers how automation actually reaches the device, and the progression is a story
about interfaces designed for humans versus interfaces designed for programs.

**Screen scraping** — driving the CLI programmatically and parsing the text output. It
works with every device ever made, and it is fragile: output formats change between
firmware versions, and a parser that worked yesterday breaks after an upgrade. A great
deal of production network automation is still this, and there is no shame in it.

**SNMP** (Chapter 54 §54.2) reads well and writes badly; it was never a configuration
protocol.

**NETCONF** (RFC 6241) is a proper configuration protocol: XML over SSH, with
transactions, candidate configurations, commit and rollback. The transactional property
matters enormously — either the whole change applies or none of it does, which
eliminates the half-configured device that a script interrupted mid-run produces.

**RESTCONF** is the same data models over HTTP and JSON, easier to use from ordinary
tooling.

**YANG** is the modelling language underneath both — a schema describing what
configuration and state data a device exposes. Its significance is standardisation:
where every vendor's CLI is different, YANG models (particularly OpenConfig's
vendor-neutral models) offer the possibility of writing automation once. The
possibility is partially realised; vendor extensions remain common.

**gNMI** is the modern streaming interface, used both for configuration and for the
telemetry of Chapter 54 §54.4.

## The pipeline

§70.4 assembles it, and the shape will be familiar to anyone who has worked near
software:

```
  change in Git → review → automated tests (syntax, policy, simulation)
       → merge → pipeline applies to staging → verify → apply to production
       → continuous drift detection
```

The step that is unfamiliar to most network engineers and most valuable is
**automated testing before deployment**. Network configuration can be tested: syntax
validation, policy checks ("no rule may permit any-to-any"), and — most powerfully —
simulation in a virtual lab using containerised or virtualised network operating
systems, where the actual change is applied to a model of the real topology and its
effect on reachability is verified.

That capability did not exist in usable form a decade ago and does now, and it moves
network changes from "we think this will work" to "we tested that this works," which
is the single largest available improvement to the change-related outage statistic from
Chapter 55.

## AIOps, assessed honestly

§70.4 closes with the current claims and a measured assessment, since the topic is
subject to more marketing than most.

**What works now.** Anomaly detection against a learned baseline (Chapter 54 §54.1) —
genuinely better than static thresholds, and commercially available. Event correlation,
collapsing a storm of alarms into one probable root cause — real and valuable, and it
addresses the alert fatigue problem directly. Capacity forecasting from historical
trends — straightforward and useful. Natural-language assistance for configuration
syntax and log interpretation — increasingly good.

**What does not work yet, whatever is claimed.** Autonomous remediation of novel
faults. Reliable root cause analysis for problems the system has not seen. Anything
requiring an understanding of business context. Networks are heterogeneous, sparsely
labelled, and consequential to get wrong, which is close to the worst combination for
current machine learning.

The defensible position: automation of the mechanical, human judgement on the
consequential, and a healthy suspicion of anything promising the second.

## By the end you will be able to

- State four specific deficiencies of CLI-based configuration management.
- Distinguish imperative from declarative and explain idempotence with an example.
- Compare screen scraping, NETCONF, RESTCONF and gNMI, and choose for a given estate.
- Explain what YANG models and why vendor-neutral models matter.
- Design a CI/CD pipeline for network configuration including the test stages.
- Separate AIOps capabilities that exist from those that are aspirational.
