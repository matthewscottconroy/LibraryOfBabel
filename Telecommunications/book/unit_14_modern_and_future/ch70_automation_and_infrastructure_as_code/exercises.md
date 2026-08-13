# Chapter 70 — Exercises

## A. Recall

**A1.** Give the time argument for automation and say why it is the weak one.

**A2.** State the consistency argument, and give the probability of zero errors when applying a
change by hand to 200 devices at a 1% per-device error rate.

**A3.** Give three problems the CLI cannot solve regardless of scripting.

**A4.** Why is screen-scraping a CLI fragile? Give the specific failure mode.

**A5.** Name the four things automation changes and say which one changes what is possible.

**A6.** Name the four things automation does not change, and state what it does to the error
distribution.

**A7.** Give the six stages of the automation maturity progression, and say which step is
difficult and why.

**A8.** Distinguish a data model, a protocol and an encoding, and place NETCONF, RESTCONF and
gNMI in the table.

**A9.** Name five things a YANG model provides that a CLI does not.

**A10.** What is `config false`, and why does the distinction matter?

**A11.** What do NETCONF's datastores enable that the CLI cannot?

**A12.** What does RESTCONF trade for accessibility?

**A13.** What does gNMI's `Subscribe` provide, and why does one interface for configuration and
telemetry matter?

**A14.** Distinguish imperative from declarative, and define idempotence.

**A15.** Give the five Ansible `state` values and say which two eliminate drift and why they are
avoided.

**A16.** Name Terraform's three distinguishing properties and say which is the most valuable
feature.

**A17.** Why is a source of truth not a tool, and what property makes it work?

**A18.** Give the four levels of the network testing pyramid with their cost and count.

**A19.** State the trade automation makes in the failure modes.

**A20.** Give three things AIOps does that work and three that do not, with the structural reason
for each failure.

## B. Apply

**B1.** Compute, for a manual change applied to 30, 120 and 400 devices at per-device error rates
of 0.5%, 1% and 2%:

(a) The expected number of errors in each of the nine cases.
(b) The probability of zero errors in each.
(c) State the device count at which you would consider manual change indefensible, and justify
it.

**B2.** For each requirement, choose NETCONF, RESTCONF, gNMI or the CLI, and justify:

(a) Applying a multi-part routing change atomically
(b) Reading interface counters every second from 400 devices
(c) A quick script to list every device's software version
(d) Configuring a device whose vendor implements no API
(e) Both configuration and telemetry through one credential
(f) A change that must be reversible if management access is lost

**B3.** Write the YANG-style description (types and constraints, not full syntax) for a VLAN
object: identifier, name, whether it is enabled, its associated subnet, and its operational
state. Mark which fields are `config false` and state the constraint on each.

**B4.** An Ansible playbook uses `state: merged` for all tasks. Over three years the estate has
accumulated configuration.

(a) Explain why.
(b) State what `replaced` and `overridden` would do differently.
(c) Design the safe transition to `replaced`, including what you would do first.
(d) State what would go wrong if you simply changed the parameter.

**B5.** Read this Terraform plan and state what you would do:

```
   ~ aws_security_group.web will be updated in-place
       + ingress { from_port = 22, cidr_blocks = ["0.0.0.0/0"] }
   - aws_subnet.data["eu-west-1c"] will be destroyed
   + aws_subnet.data["eu-west-1d"] will be created
   Plan: 1 to add, 1 to change, 1 to destroy.
```

**B6.** Design the CI pipeline for a network repository: the stages, what runs at each, what
fails the build, where the approval gate sits, and what the post-deployment verification checks.
Present it as a list with a one-line justification per stage.

**B7.** Write three unit tests for a template that generates an access switch configuration.
Each must assert something from Chapter 62 §62.4's hardening checklist, and each must be
specific enough to fail if the template regresses.

**B8.** Assess each AIOps claim, stating whether it is real, bounded or unsupported, and the
question you would ask the vendor:

(a) "Reduces alert volume by 90%"
(b) "Identifies the root cause automatically"
(c) "Predicts hardware failures before they occur"
(d) "Self-heals common faults"
(e) "Detects anomalies your thresholds miss"
(f) "Answers questions about your configuration in natural language"

## C. Analyse

**C1.** The chapter says the consistency argument is the real one and the time argument is weak.
Analyse why the time argument is nonetheless the one used in business cases, and what a better
argument would look like.

**C2.** Analyse the claim that "the organisations with the worst automation incidents automated a
bad decision efficiently". Find a published example, and state what process would have caught it.

**C3.** Analyse why the step from "configuration versioned" to "configuration generated" is
organisational rather than technical. What exactly must change, and who must agree?

**C4.** RESTCONF trades transactions for accessibility. Analyse when that trade is correct, and
construct a change for which it would be dangerous.

**C5.** Most Ansible network automation uses `merged`, which reproduces the accumulation problem
it was meant to solve. Analyse why practitioners avoid `replaced`, and design an approach that
makes it safe enough to adopt.

**C6.** Analyse Terraform's state file as both its distinguishing feature and its principal
liability. What exactly goes wrong when it is lost, shared badly, or diverges from reality?

**C7.** Analyse the automation failure-mode trade: fewer errors with larger blast radii. Compute
or estimate the expected annual cost of each regime for a 200-device estate, stating your
assumptions.

**C8.** The chapter argues AIOps is statistics with a better name. Assess this fairly:
what, if anything, do current machine learning methods do that classical statistics could not,
and where is the boundary?

**C9.** Analyse why an AIOps model trained on one network transfers poorly to another. Is this a
fundamental limitation or a data problem, and what would change it?

## D. Design

**D1.** Design the automation adoption plan for a 250-device estate currently at stage 1
(scripted). Twelve months, with the sequence, what is automated at each stage, what capability
must be built, and how you would demonstrate value at each step. Include what you would not
automate and why.

**D2.** Design the source of truth for a network of 40 sites: what is modelled, the relationships
between objects, what is generated from it, and how you would populate it from an estate that has
no such record. State what the modelling exercise would reveal.

**D3.** Design the test suite for a network automation repository: the levels, at least three
tests at each, what infrastructure each requires, and the runtime budget for the whole pipeline.

**D4.** Design the security architecture for an automation platform: where it runs, what
credentials it holds, how they are stored and rotated, what network it sits on, what it is
authorised to do, and how its actions are logged and reviewed.

**D5.** Write the two-page assessment you would give a manager who has been told an AIOps product
will let the team operate the network with two fewer people. Be fair to what the product does,
specific about what it does not, and clear about what would actually reduce operational effort.

## E. Troubleshoot

**E1.** An automation run reports success on all 200 devices and 3 are misconfigured. Give three
possible causes.

**E2.** A playbook that worked for a year fails on 40 devices after a firmware upgrade. Diagnose.

**E3.** A `terraform apply` destroys a production subnet. Analyse what should have prevented it at
three separate points.

**E4.** A configuration is applied by automation and reverted by hand within an hour, repeatedly.
Analyse the organisational problem and the technical one.

**E5.** A NETCONF `edit-config` succeeds and the change does not take effect. Give the likely
cause.

**E6.** An engineer loses management access to a device during an automated change. State what
should have prevented it and why it is a protocol primitive.

**E7.** A CI pipeline passes and the change breaks production. List five things the pipeline could
have tested and did not.

**E8.** An AIOps system correlates two unrelated incidents and directs the team at the wrong
subsystem. Analyse the cost and the design response.

**E9.** A template renders correctly for 199 devices and produces invalid configuration for one.
Diagnose, and state which test level would have caught it.

## F. Extend

**F1.** Set up a NETCONF or RESTCONF session against a virtual router (FRR, Nokia SR Linux,
Arista cEOS, or a vendor's virtual image). Retrieve the interface configuration, modify it, and
observe the candidate/commit behaviour. Then demonstrate `commit confirmed` by deliberately
locking yourself out.

**F2.** Model a small network in NetBox or Nautobot, write a Jinja2 template for one device role,
and generate the configuration from the data. Then change one value in the source of truth and
regenerate.

**F3.** Write an Ansible playbook using `gathered` state to collect a fact from every device you
have access to, and produce a compliance report. Then extend it to `merged` for one low-risk
setting.

**F4.** Build a CI pipeline (GitHub Actions, GitLab CI, or a local runner) that lints YAML,
renders a template, validates the output against three assertions, and fails on any error.
Deliberately break each check and confirm the failure.

**F5.** Build a virtual topology in containerlab, configure it entirely from automation, and write
three integration tests: an adjacency check, a reachability check and a negative reachability
check. Run them in CI.

**F6.** Run Batfish against your rendered configurations and assert a reachability property in
CI. Then introduce a change that violates it and confirm the pipeline fails.

**F7.** Compute the error arithmetic for your own environment: estimate your per-device error
rate from incident records if you can, and calculate the expected errors for the largest change
your team has made manually.

**F8.** Evaluate an AIOps or anomaly-detection product against §70.4's five questions, using its
documentation rather than its marketing. Report which questions it answers clearly and which it
does not.
