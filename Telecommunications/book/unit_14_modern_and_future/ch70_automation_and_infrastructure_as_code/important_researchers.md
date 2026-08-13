# Chapter 70 — The People

**Mark Burgess (b. 1966)**, again — **Chapter 55's entry, and this chapter is the argument's
destination.**

**CFEngine (1993) and promise theory.** **Burgess's two contributions bear directly:**

**Convergent configuration.** **The system is described as a desired state and repeatedly
converges on it** — **which is §70.3's declarative model, and it predates every tool in this
chapter by a decade or more.**

**And promise theory**, which is the subtler one:

> **An agent can only make promises about its own behaviour. It cannot impose obligations on
> others.** **Which sounds philosophical and is a precise model of why centralised configuration
> systems are harder than they look** — **a controller cannot make a device do anything; it can
> only ask, and the device promises.**

**The practical consequence is the one automation practitioners meet:** **an automation run's
report of success is a report that the request was accepted**, and **verification is a separate
act** (§70.4's post-deployment check). **Burgess argued this in 1998 and the industry has
rediscovered it repeatedly.**

**Luke Kanies, Adam Jacob, Michael DeHaan and Mitchell Hashimoto** — **the tool authors, in
sequence, and each solved the previous one's problem.**

| | Tool | The problem it addressed |
|---|---|---|
| **Kanies** | **Puppet (2005)** | **CFEngine's language; a declarative model with a better DSL** |
| **Jacob** | **Chef (2009)** | **Puppet's DSL; use a real programming language** |
| **DeHaan** | **Ansible (2012)** | **both tools' agents; do it over SSH with YAML** |
| **Hashimoto** | **Terraform (2014)** | **none of them managed resource lifecycle** |

**DeHaan's decision is the one that mattered for networks:**

> **Ansible is agentless because DeHaan judged that the agent was the adoption barrier.** **And
> a network device cannot run an agent at all** — **which made Ansible the only one of the four
> that could configure a switch**, and it is why it dominates network automation despite not
> having been designed for it.

**And DeHaan has written that Ansible's YAML was chosen for approachability rather than for
expressiveness** — **explicitly trading power for the ability of a non-programmer to read a
playbook** — **which is Chapter 57's psychological acceptability, applied to a tool.**

**Hashimoto's contribution is the state file**, and it is the one the others lacked: **a record
of what was created, which is what makes "this resource should no longer exist" expressible.**

**Jeremy Schulman, Kirk Byers, David Barroso and the network automation community.**

**This is a group that did not come from research and whose contribution is largely libraries.**

| | |
|---|---|
| **Netmiko** (Byers) | **SSH to network devices, per platform, from Python** — **the pragmatic bridge** (§70.1) |
| **NAPALM** (Barroso and others) | **a unified API across vendors** — get facts, compare configurations, commit and roll back |
| **Nornir** | **an automation framework in Python rather than YAML** |
| **`ncclient`, `pyats`, `scrapli`** | the plumbing |

> **Their significance is that they made network automation possible before the vendors' APIs
> were adequate**, and they are still what most real automation runs on. **Byers's Netmiko in
> particular is the most-used network automation library in existence and was written by one
> person to solve his own problem.**

**And Schulman's argument — from Juniper, and then independently — that network engineers should
learn to program rather than waiting for a product** is the one that shaped the community's
culture. **The result is a field where the practitioners write the tools**, which is unusual and
healthy.

**Jason Edelman, Scott Lowe and Matt Oswalt.** ***Network Programmability and Automation***
(2018, second edition 2021), **and the teaching work around it.**

**The book's contribution was to establish the curriculum:** **Python, data formats, templating,
APIs, source of truth, testing** — **in that order** — **and it is why a network automation job
description looks the way it does.**

**Oswalt's writing on testing network automation is the under-appreciated part**, and §70.4's
pyramid is substantially his framing.

**Jez Humble, Dave Farley, Nicole Forsgren and Gene Kim** — **Chapter 55's entry, and the CI/CD
material is theirs.**

***Continuous Delivery* (2010)** established the pipeline, and ***Accelerate* (2018)** supplied
the evidence (Chapter 55's reading).

> **The finding that bears on §70.4: organisations that deploy more frequently have fewer
> failures and recover faster** — **and the mechanism is that small, frequent, tested changes
> have small blast radii and are easy to reason about.** **Which is the direct argument against
> the large, infrequent, carefully-reviewed network change window.**

**And the finding that external approval boards do not improve stability** (Chapter 55's entry)
**applies to network change advisory boards specifically**, and it is uncomfortable.

**Charity Majors, Liz Fong-Jones and the observability community.**

**Their argument bears on §70.4's AIOps discussion:**

> **The distinction between monitoring — checking known conditions — and observability — being
> able to ask questions you did not anticipate.** **A system that can only answer the questions
> its dashboards were built for cannot diagnose a novel fault**, and **most "AI-driven
> operations" products are automating the first while claiming the second.**

**And Majors's "test in production" argument** (Chapter 56's entry) **is the honest counterweight
to §70.4's testing pyramid**: **the lab does not have your traffic, your scale or your
accumulated configuration**, and a pipeline that believes otherwise is Chapter 63 §63.2's
verification claim.

## What this chapter's history shows

**Three observations.**

**The ideas came from systems administration, not from networking.** **CFEngine, Puppet, Chef,
Ansible and Terraform were all built for servers**, and **networking adopted them ten to fifteen
years later** — **which is the same lag Chapter 53 observed for documentation and Chapter 55 for
change management.**

**The practitioners built the tools.** **Netmiko, NAPALM, Nornir and the community's libraries
were written by working engineers because the vendors' offerings were inadequate** — **and they
remain the foundation.** **A field in which the users write the tools is a field with unusually
good tools and unusually poor documentation.**

**And the evidence contradicts the instinct.** **Accelerate's finding — more frequent change,
fewer failures — is the opposite of what network operations culture assumes**, and **the
resistance to it is Chapter 55's normalisation of deviance, running in the safe direction.**

> **The chapter's uncomfortable summary: the practices that make network change safe were
> established elsewhere, measured elsewhere, and are adopted here reluctantly** — **and the
> reason is not technical.**
