# 70.4 CI/CD, Testing and the Honest State of AIOps

**The pipeline that turns §70.3's tools into a process**, and **a section on machine learning
that is deliberately sceptical.**

## The pipeline

```
   Engineer          Repository         CI pipeline              Network
      │                  │                   │                      │
      │──── branch ─────▶│                   │                      │
      │──── commit ─────▶│                   │                      │
      │──── PR ─────────▶│─── trigger ──────▶│                      │
      │                  │                   │─ lint                │
      │                  │                   │─ render templates    │
      │                  │                   │─ validate syntax     │
      │                  │                   │─ unit tests          │
      │                  │                   │─ deploy to lab ─────▶│ (virtual)
      │                  │                   │─ integration tests   │
      │                  │                   │─ policy checks       │
      │◀─── review ──────│◀──── results ─────│                      │
      │                  │                   │                      │
      │──── merge ──────▶│─── trigger ──────▶│─ plan / --check      │
      │                  │                   │─ [approval gate]     │
      │                  │                   │─ deploy, staged ────▶│ production
      │                  │                   │─ verify             │
      │                  │                   │─ rollback on failure │
```

**And each stage maps onto Chapter 55 §55.2's change record**, automated:

| Change record element | Pipeline stage |
|---|---|
| **What is changing, exactly** | **the diff** |
| **Why** | **the commit message and the PR** |
| **Blast radius** | **the plan, and the target inventory** |
| **Verification** | **the tests, defined before** |
| **Rollback** | **revert the commit and re-apply** |
| **Approval** | **the PR review and the gate** |

> **Chapter 55 §55.2 said a change record should contain six things and that they take two
> minutes to state.** **A pipeline produces all six as a by-product of the workflow**, which is
> the strongest argument for it — **the discipline stops depending on the engineer's diligence
> at 02:00.**

## The testing pyramid, applied to networks

**Four levels, and the cost rises and the count falls at each.**

| Level | Tests | Cost | Count |
|---|---|---|---|
| **Lint / syntax** | **is the YAML valid? does the template render?** | **seconds** | **every commit** |
| **Unit** | **does this template produce the expected configuration for this input?** | **seconds** | **hundreds** |
| **Integration** | **does the configuration work in a virtual topology?** | **minutes** | **dozens** |
| **Policy / verification** | **is the resulting reachability correct?** | **minutes** | **tens** |

**And a fifth that is not a test:**

**Post-deployment verification.** **Chapter 55 §55.2's step 6, automated** — **check the state
after applying and roll back if it is wrong.**

### Unit testing a template

**The one people skip and the one with the best return.**

```
   Given:   device role = access_switch, vlans = [20, 240], uplink = Te1/1/1
   Expect:  the rendered configuration contains 'spanning-tree bpduguard enable'
            on every access port, and does not contain it on Te1/1/1
```

> **A test that asserts BPDU guard is present on access ports and absent on uplinks catches a
> template error before it reaches two hundred switches**, and it runs in a second. **Chapter
> 62 §62.4's hardening checklist, enforced by CI**, which is the only way it stays enforced.

### Integration testing

**A virtual topology, built from the same source of truth, configured by the same automation.**

**containerlab, GNS3, EVE-NG, or the vendors' virtual images** (Chapter 67's reading) —
**and the tests are ordinary:**

| Test | |
|---|---|
| **Does OSPF form the expected adjacencies?** | |
| **Is every prefix in every routing table?** | |
| **Can host A reach host B?** | **and can it not reach host C?** |
| **Does the failover work?** | **shut an interface and re-test** — Chapter 56 §56.2, in CI |

**And the honest limitation:** **a virtual topology is not the production network.** **The
hardware differs, the scale differs, the traffic differs, and the accumulated configuration
differs** (Chapter 55 §55.1). **It catches syntax, logic and gross topology errors**, and **it
does not catch a platform-specific behaviour or a scale limit.**

> **Which is worth stating because "we tested it in the lab" is Chapter 63 §63.2's verification
> claim**, and **the lab's coverage should be known rather than assumed.**

### Policy verification

**Chapter 68 §68.4's argument, in a pipeline.**

**Batfish (Chapter 55's reading) takes the rendered configurations and answers reachability
questions without any device**, which means **"can the guest network reach the finance servers?"
is a test that runs on every commit.**

> **This is the capability that most distinguishes network CI from a script that applies
> configuration**, and it is available, free, and rarely used.

## Deployment strategy

**Chapter 55 §55.2's staging, made routine.**

| Strategy | |
|---|---|
| **Canary** | **one device, then a site, then the estate** — with a wait between |
| **Rolling** | **in batches, with verification between** |
| **Blue-green** | **rarely applicable to network devices; standard for cloud networks** |
| **Automatic rollback** | **on verification failure** — and it must be tested |

**Two rules that matter more than the strategy:**

**Wait long enough between stages.** **A change that breaks something under load breaks it at
09:00**, not at 02:00 (Chapter 55 §55.2).

**And define what "verified" means before deploying.** **A pipeline that deploys and does not
check has automated the risky half and left the safe half manual.**

## What automation does to the failure modes

**The honest accounting, and it is not all favourable.**

| | **Manual** | **Automated** |
|---|---|---|
| **Random errors** | **frequent** (§70.1) | **rare** |
| **Systematic errors** | rare | **possible, and applied everywhere** |
| **Time to make a change** | hours | **minutes** |
| **Time to make a mistake** | hours | **minutes** |
| **Blast radius** | **bounded by how many you got to** | **the whole estate** |
| **Detectability** | **the two wrong devices are invisible** | **the whole estate is wrong, visibly** |

> **The trade is fewer errors with larger blast radii**, and **the mitigation is the pipeline —
> testing, staging and automatic rollback** — **which is why the process matters more than the
> tool.**

**And the worst automation incidents in the industry are all the same shape:** **a correct
automation system, applying an incorrect change, quickly, everywhere** — **which no amount of
tooling prevents and which staged deployment bounds.**

## AIOps, honestly

**A section that is deliberately sceptical, because the claims are large and the evidence is
mixed.**

### What works now

| | Why it works |
|---|---|
| **Anomaly detection on metrics** | **statistics on time series** (Chapter 54 §54.1) — **and this is not new; "AI" is the current label for it** |
| **Alert correlation and deduplication** | **grouping related alerts is a tractable problem**, and it genuinely reduces noise (Chapter 54 §54.4) |
| **Log clustering** | **finding the twelve distinct messages in a million lines** — **real, and useful** |
| **Capacity forecasting** | **trend extrapolation** (Chapter 54 §54.1) |
| **Natural-language interfaces to documentation and configuration** | **the current genuine advance** |

**The last row deserves its own note**, because it is the one that has changed recently:

> **A model that can answer "which ACL entry permits traffic from 10.20.5.0/24 to the finance
> subnet?" against a corpus of configurations is doing something useful** — **not because it
> reasons about networks, but because it searches and summarises text better than `grep`.**
> **Which is a real productivity improvement and is not autonomous operation.**

### What does not work

| Claim | Status |
|---|---|
| **Automated root cause analysis** | **it correlates; it does not diagnose** — and Chapter 55's Cook argues there is no single root cause |
| **Self-healing networks** | **bounded remediation of anticipated faults**, which is a runbook with a trigger (Chapter 68 §68.4) |
| **Predictive failure** | **works for components with wear characteristics** — optics, fans, disks. **Not for configuration or software faults** |
| **Autonomous operation** | **no** |

**And the structural reasons, which are worth understanding rather than merely noting:**

**The training data does not exist.** **A model that predicts failures needs labelled examples of
failures**, and **a well-run network produces very few** — which is the outcome you wanted and
the data you lack.

**Every network is different.** **A model trained on one estate transfers poorly to another**,
because the topology, the vendors, the applications and the accumulated configuration are all
specific.

**Correlation is not causation, and the incidents that matter are novel.** **A system that
learned from the last hundred incidents is well prepared for the hundred-and-first only if it
resembles them** — **and the ones that cause real damage do not.**

**And the cost of a wrong action is asymmetric.** **A remediation system that is right 95% of the
time and takes action is causing an outage every twentieth incident** (Chapter 68 §68.4), **which
is why every serious implementation reports rather than acts.**

### The honest position

> **AIOps is doing statistics on operational data, with better tooling and a better name.**
> **The statistics are genuinely useful — correlation, clustering, forecasting, anomaly
> detection — and they are the parts that were possible before and were not done because nobody
> built the tooling.**

**Which is not dismissive.** **A system that turns four hundred alerts into six correlated
incidents has solved Chapter 54 §54.4's problem**, and that is worth buying.

**What to ask a vendor**, and the questions are the same as Chapter 68 §68.4's:

1. **What does it detect that a threshold does not?**
2. **What does it do automatically, and what is the list?**
3. **What is its false positive rate on our data, measured?**
4. **What happens when it is wrong?**
5. **Can we see why it concluded what it concluded?**

**The fifth is the one that separates useful products from unusable ones.** **An alert that says
"anomaly detected, confidence 0.87" is not actionable; one that says "interface Gi1/0/14's error
rate is 40 standard deviations above its 30-day baseline, beginning at 14:07" is.**

## What breaks here

**A pipeline that deploys and does not verify.** **The risky half automated and the safe half
manual.**

**"We tested it in the lab" and it broke in production.** **The lab's coverage was assumed.**
Know what it does and does not test.

**A template error applied to two hundred switches.** **No unit test.** It would have run in a
second.

**An automation system with no staged deployment.** **The blast radius is the estate.**

**A correct automation system applying an incorrect change everywhere.** **The characteristic
automation incident**, and staging is the only bound.

**An AIOps product that reports anomalies with no explanation.** **Not actionable.** Ask for the
evidence, not the confidence.

**A self-healing system that healed the wrong thing.** **Report by default** (Chapter 68 §68.4).

**A model trained on someone else's network.** **Every network is different**, and the transfer
is poor.

> **Network+ note.** Objective 3.2 and 1.8. Over-learn: **CI/CD applies software practices to
> infrastructure**; **testing before deployment reduces errors**; **version control provides
> history and rollback**; and **automation and orchestration differ — automation performs a task,
> orchestration coordinates many.** The automation/orchestration distinction is examined and
> AIOps is a vendor term rather than an examined concept.
