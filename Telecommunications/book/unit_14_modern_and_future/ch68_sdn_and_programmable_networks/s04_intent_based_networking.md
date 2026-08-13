# 68.4 Intent-Based Networking

**The current framing of the same argument**, and it deserves both a fair hearing and a
sceptical one — **because the idea is sound and the marketing has outrun it by a distance.**

## The claim

> **Describe what you want. The system determines how, implements it, verifies that it was
> implemented, and continuously confirms that it remains true.**

**Four verbs, and each is a distinct capability:**

| | |
|---|---|
| **Translate** | **intent → configuration** |
| **Activate** | **deploy it, consistently, across every device** |
| **Assure** | **verify that the network's actual state matches the intent** |
| **Remediate** | **correct drift, or report it** |

**And the distinction from automation is the third and fourth:**

> **Automation deploys what you told it to deploy.** **Intent-based networking checks
> afterwards that the network is doing what you asked, continuously, and tells you when it is
> not.** **The verification is the claim.**

## What is genuinely new

**Three things, and they are real.**

### Continuous verification

**Chapter 55 §55.1's configuration drift, addressed by construction rather than by audit.**

**A system that holds the intent can compare it against the network's actual state — not the
configuration, the *state*: the routing tables, the forwarding entries, the reachability — and
report divergence.**

> **"Is VLAN 240 reachable from every access switch?" is a question no configuration audit can
> answer**, because **the configuration may be correct and the network may not be**
> (Chapter 65 §65.2's spanning tree, a failed uplink, an unrelated ACL).

### Formal verification of reachability

**The capability that is most under-appreciated.**

**Given a model of the network — configurations, topology, forwarding tables — it is possible to
answer, exhaustively:**

| Question | |
|---|---|
| **Can A reach B?** | **for every A and B, without sending a packet** |
| **Is there any path from the guest network to the finance servers?** | **including paths nobody designed** |
| **Would this change alter what is reachable?** | **before applying it** |
| **Are these two firewalls' policies equivalent?** | **after a migration** |

**Batfish, Forward Networks, Veriflow and the academic work behind them do this**, and **it is a
genuine advance over testing**, because **testing samples and verification is exhaustive.**

> **"We tested it and it works" and "we proved that no packet can reach the finance network from
> the guest VLAN" are different claims**, and the second was not available before.

### A single source of truth that generates rather than describes

**Chapter 55 §55.4's inversion, and Chapter 70's argument.**

## What is oversold

**Four things, stated plainly.**

### "Describe intent in natural language"

> **The demonstrations show "make the finance network secure" producing a configuration.**
> **What is actually specified is a structured model** — **a YAML document, a policy language, a
> graph of relationships** — **which is a formal specification with better ergonomics, and that
> is a genuine improvement without being what the demonstration implied.**

**And the reason is not a limitation of the technology.** **"Make it secure" has no
determinate meaning** (Chapter 57 §57.3) — **secure against whom, at what cost, with what
availability trade** — **and a system that produced a configuration from it would be guessing.**

### "The system determines how"

**Within a template's parameters, yes.**

> **A system that translates "these two sites must communicate" into a VPN configuration is
> selecting from designs its authors anticipated.** **It is not doing network design**, and
> **presenting it as though it were sets an expectation that the first unanticipated requirement
> will disappoint.**

### "Self-healing"

**Remediation is real and bounded.**

**A system can reapply a configuration that drifted, disable a port that is flapping, or shift
traffic away from a degraded path.** **It cannot diagnose a novel fault, and it should not act on
one.**

> **And a remediation system acting on a fault it has misdiagnosed is a system causing an
> outage**, which is why every serious implementation defaults to reporting rather than acting,
> **and the automatic actions are a short, deliberately chosen list** (Chapter 55 §55.2's
> standard changes, in a different guise).

### And the scope

**Current products are vendor-specific and domain-specific** — **a campus product, a data centre
product, a WAN product, each from one vendor** — **and "intent-based" across a heterogeneous
estate is not something you can buy.**

**Which is not a criticism of the idea.** **It is a statement about where the products are**,
and the gap between the two is where disappointment happens.

## The honest assessment

| Claim | Status |
|---|---|
| **Verification of reachability before and after change** | **real, valuable, and under-used** |
| **Continuous state assurance** | **real** |
| **Configuration generated from a model** | **real** (Chapter 70) |
| **Reduced configuration drift** | **real** |
| **Natural-language intent** | **marketing** |
| **The system designs the network** | **no** |
| **Self-healing** | **bounded, and correctly so** |
| **Vendor-neutral, whole-estate** | **not yet** |

> **The verification half is the part worth buying and the part least emphasised in the
> marketing**, which is an odd but consistent pattern: **the demonstrable capability is less
> exciting than the aspirational one**, so the aspirational one is sold.

## What to actually do

**Four things available now, none of which requires a product.**

**Put the configuration in version control and generate it** (Chapter 55 §55.4, Chapter 70).
**This is the source-of-truth half, and it costs an afternoon to begin.**

**Run a verification tool against your configurations.** **Batfish is free.** **Ask it whether
the guest network can reach anything it should not**, and the answer is frequently informative.

**Check state, not configuration** (Chapter 54). **A monitoring system that verifies "the
expected VLANs are present on every trunk" and "every leaf has a route to every prefix" is
performing assurance**, and it can be built from `show` commands and a script.

**And define the intent explicitly, in a document, even without a system that consumes it.**
**"Every branch must reach the data centre over two independent paths" and "the guest network
must reach only the Internet" are testable statements**, and **writing them down is the
prerequisite for any system that would enforce them.**

> **Most of the value attributed to intent-based networking is available to an organisation that
> writes down what it intends and checks periodically that it is true.** **The products
> industrialise that; they do not create it.**

## Where the whole chapter lands

**Fifteen years after SDN was going to change everything:**

| | |
|---|---|
| **The control/data separation** | **won, in forms nobody predicted** (§68.1) |
| **The central controller** | **lost, except at hyperscale** (§68.2) |
| **Commodity hardware and disaggregation** | **won** |
| **The programmable pipeline** | **right, and commercially difficult** (§68.3) |
| **Network programmability** | **won completely — as APIs and automation** (Chapters 69, 70) |
| **The network with an API** | **arrived, as the cloud** |

> **The idea was correct and the implementation route was wrong.** **Which is a more common
> outcome in engineering than either triumph or failure, and it is worth being able to recognise
> while it is happening** — **because the current claims about intent, about AI-driven
> operations (Chapter 71) and about autonomous networks have the same shape, and the same
> question applies: which part of this is the architecture, and which part is the product?**

## What breaks here

**A demonstration in natural language and a deployment in YAML.** **Expected.** The formal model
is the real interface.

**A system that "determines how" and cannot express a requirement its authors did not
anticipate.** **A template, with better ergonomics.**

**Automatic remediation acting on a misdiagnosed fault.** **A system causing an outage.** Report
by default; automate a short list deliberately.

**An intent system that covers the campus and not the WAN, the firewalls or the cloud.**
**Current products are domain-specific.** Plan for the boundary.

**Assurance that checks configuration rather than state.** **The configuration may be correct and
the network wrong.** That gap is the whole point.

**An organisation buying an intent product without having written down its intent.** **The
prerequisite is the hard part**, and the product will not supply it.

> **Network+ note.** Objective 1.8 mentions SDN and network programmability. Over-learn:
> **infrastructure as code, automation and orchestration reduce configuration errors**; **APIs
> allow programmatic network management**; and **SDN centralises control and enables
> programmability.** Intent-based networking is a vendor term rather than an examined concept,
> and **the verification argument is the part worth carrying into practice.**
