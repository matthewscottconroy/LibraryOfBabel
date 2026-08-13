# 70.1 Why the CLI Does Not Scale

**The command line is an excellent interface for one device and a poor one for two hundred**, and
**the arithmetic explains both halves.**

## The time argument, which is the weak one

| Devices | At 5 minutes each | |
|---|---|---|
| 10 | **50 minutes** | annoying |
| 50 | **4.2 hours** | a day's work |
| **200** | **16.7 hours** | **two days, and it cannot be done in a window** |
| 1,000 | **83 hours** | not possible |

**And it is the weak argument because it can be answered with a script**, or with more people,
or with a longer window. **The strong argument is the next one.**

## The consistency argument, which is the real one

> **Every manual change has a probability of being wrong, and the probability that all of them
> are right falls exponentially with the device count.**

**At a 1% per-device error rate** — **which is optimistic for a tired engineer at 02:00** —

| Devices | Expected errors | **Probability of zero errors** |
|---|---|---|
| 10 | 0.1 | **90%** |
| 50 | 0.5 | 61% |
| **200** | **2.0** | **13%** |
| 1,000 | 10 | **essentially zero** |

**Read the right-hand column again.**

> **A change applied by hand to two hundred devices is almost certainly wrong somewhere.** **Not
> because the engineer is careless — at 1% they are doing well — but because 0.99 to the power
> of 200 is 0.13.**

**And the errors are invisible.** **The 198 devices that were configured correctly work; the two
that were not work too, until the circumstance that exercises the difference arrives** —
Chapter 55 §55.1's drift, created deliberately, one typo at a time.

**Which is the actual argument for automation**, and it is a statement about probability rather
than about effort.

## The three problems the CLI cannot solve

**Beyond scale, and each is structural.**

### There is no record of intent

**A configuration file records what a device is configured to do.** **It does not record what
anyone wanted, why, or whether it is correct.**

> **`ip route 10.9.0.0 255.255.0.0 10.20.0.9` is a fact. Whether it should be there is not in
> the file** (Chapter 55 §55.1's accumulation, from the other direction).

### There is no way to verify

**"Is every access switch configured according to the standard?" cannot be answered by looking at
devices one at a time**, and **an audit that samples is an audit that misses.**

### And the same change is expressed differently on each platform

**Which means an estate with three vendors has three procedures for every change**, and **the
procedures drift apart.**

## The CLI's actual problem: it was designed for a human

**The deeper observation, and it explains why scripting the CLI is a poor substitute for an
API.**

| The CLI assumes | An automation tool needs |
|---|---|
| **A human reads the output** | **structured, parseable data** |
| **Errors are reported in prose** | **a status code and a reason** |
| **The prompt indicates state** | **an explicit state query** |
| **Output format may change between releases** | **a stable contract** |
| **Partial application is acceptable** | **atomicity — all or nothing** |
| **`?` and tab completion guide** | **a schema** |

> **Screen-scraping a CLI works and is fragile.** **A vendor changes a column heading in a
> minor release and the automation breaks**, silently, on some devices — **and the failure mode
> is that the script believes it succeeded.**

**Which is why §70.2's APIs matter and why "we automated it with Expect scripts" is a
transitional state rather than a destination.**

**And the honest qualification: screen-scraping is what most organisations actually do**, because
the devices are old, the API is not implemented, or the vendor's API covers a fraction of the
CLI's functionality. **Netmiko and its equivalents exist because the transition is incomplete**,
and using them is legitimate — **provided the fragility is understood and the output is
validated rather than assumed.**

## What automation actually changes

**Four things, and only the first is the one people expect.**

**Speed.** **Two hundred devices in four minutes rather than two days.**

**Consistency.** **The same input produces the same output, on every device, every time** —
which is the probability argument, inverted.

**Reviewability.** **A change is a diff, in a pull request, that a second person reads before it
is applied** (Chapter 55 §55.2's peer review, made routine rather than exceptional).

**And repeatability.** **A change that can be applied can be applied again** — **which makes
rebuilding a device from scratch a two-minute operation rather than a project**, and it is what
makes Chapter 56's recovery arithmetic tractable.

> **The fourth is the one that changes what is possible.** **When rebuilding is cheap, a
> corrupted device is replaced rather than repaired**, and **the whole of Chapter 63's diagnostic
> effort on a single failed device becomes optional.**

## What automation does not change

**Four things, stated because the expectation is otherwise.**

**It does not remove the need to understand the network.** **An automated wrong configuration is
applied to two hundred devices in four minutes.**

> **Automation multiplies the consequences of a decision in both directions**, and **the
> organisations that have had the worst automation incidents are not the ones that automated
> badly — they are the ones that automated a bad decision efficiently.**

**It does not reduce headcount.** **The evidence is consistent**: **automation changes what
people do rather than how many are needed**, and an organisation that automates in order to
reduce staff typically finds that the automation itself requires operating.

**It does not eliminate errors.** **It changes their distribution** — **fewer random errors,
occasional systematic ones** — **and a systematic error is worse.**

**And it adds a system to operate.** **The automation platform, its credentials, its access, its
versions and its failure modes are now part of the network** (Chapter 55 §55.3, Chapter 60
§60.4) — **and an automation system with administrative access to every device is the highest-
value target in the estate.**

## The maturity progression

**Where organisations actually are, and the honest observation is that most are at step two.**

| | Stage | Characteristic |
|---|---|---|
| **0** | **Manual** | the CLI, per device |
| **1** | **Scripted** | **Expect, Netmiko, or a shell loop** — the same change, faster |
| **2** | **Configuration collected and versioned** | **Chapter 55 §55.4 — the device is still the truth** |
| **3** | **Configuration generated from a source of truth** | **the repository is the truth** |
| **4** | **Change is a pull request, tested, applied by a pipeline** | §70.4 |
| **5** | **State continuously reconciled against intent** | Chapter 68 §68.4 |

> **The step from 2 to 3 is the difficult one**, and **it is an organisational change more than
> a technical one** (Chapter 55 §55.4) — **because it requires the team to stop making changes
> on devices**, which is a habit rather than a capability.

**And the incremental route is the one that works:**

1. **Collect and version everything** (Chapter 55 §55.4) — **an afternoon, and it pays
   immediately**
2. **Automate the read-only things first** — inventory, compliance checks, gathering state.
   **No risk, immediate value, and it builds confidence**
3. **Automate one low-risk change type** — a banner, an NTP server, a syslog target
4. **Add a source of truth for the data**, and generate that one thing from it
5. **Widen**, one change type at a time

> **The organisations that succeed start with the read-only half.** **The ones that fail start by
> automating a routing change.**

## What breaks here

**A change applied by hand to two hundred devices, and two are wrong.** **Expected, at any
plausible error rate.** The arithmetic, not the engineer.

**An automation script that broke after a firmware upgrade.** **Screen-scraping, and an output
format changed.** Validate the parse rather than assuming it.

**A script that reported success and did nothing.** **The CLI returned prose the script did not
recognise as an error.**

**An automated change applied to every device in four minutes, and it was wrong.** **The
consequences are multiplied in both directions.**

**An automation platform with administrative credentials for every device, on the user network.**
**The highest-value target in the estate**, and Chapter 60 §60.4's management plane argument
applies with force.

**An automation project that began with the riskiest change.** **Start with the read-only
half.**

**A team at stage 2 for four years.** **The step to 3 is organisational** — it requires agreeing
to stop configuring devices.

> **Network+ note.** Objective 3.2 and 1.8 touch automation. Over-learn: **automation reduces
> configuration errors and improves consistency**; **scripts and APIs replace manual
> configuration**; **infrastructure as code manages configuration in version control**; and
> **automation requires testing and rollback like any change.** The consistency argument is the
> examinable one and the probability arithmetic is what makes it persuasive.
