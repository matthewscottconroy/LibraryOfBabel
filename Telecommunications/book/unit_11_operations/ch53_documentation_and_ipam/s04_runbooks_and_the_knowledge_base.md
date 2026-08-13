# 53.4 Runbooks and the Knowledge Base

**The documentation most often absent and most valuable during an incident: what to do,
written down, for the situations you can anticipate.**

## The test

> **Can someone competent but unfamiliar execute this at 03:00, without waking anyone?**

**Most runbooks fail that test**, and they fail for one reason: **they were written by the
person who already knew**, who therefore omitted everything obvious to them.

**What "obvious to them" looks like in practice:**

| The runbook says | The reader needs |
|---|---|
| "Check the tunnel state" | **the exact command, on which device** |
| "If it looks wrong" | **what right looks like** |
| "Restart the service" | **the command, and how long the disruption lasts** |
| "Escalate if necessary" | **to whom, at what number, and what "necessary" means** |
| "Log in to the firewall" | **which firewall, and where the credential is** |

**Every one of those gaps costs minutes at 03:00**, and minutes at 03:00 are the expensive
ones.

## The shape of a good runbook

**Specific and executable. Not a description — an instruction.**

```
   ════════════════════════════════════════════════════════
   RUNBOOK: Branch VPN tunnel down
   Last reviewed: 2026-04-12   Owner: network team
   Expected duration: 10–30 min   Escalation after: 30 min
   ════════════════════════════════════════════════════════

   SYMPTOM
     Branch reports loss of access to head office systems.
     Monitoring alert "TUNNEL-DOWN <site>".

   IMPACT
     That branch only. Internet access at the branch continues.
     Card payment is unaffected (separate path).

   1. CONFIRM
      ssh netops@edge-<site>.example.net
      # show crypto session detail
      Healthy: "Session status: UP-ACTIVE", both SAs present.
      If "DOWN-NEGOTIATING" → phase 1 is failing, go to 3.
      If no output at all → the device is unreachable, go to 5.

   2. CHECK THE UNDERLAY FIRST
      # ping <peer public address> source <wan interface>
      If this fails, the problem is the circuit, not the tunnel.
      → Runbook: "Branch circuit down"

   3. THREE COMMON CAUSES, IN ORDER OF FREQUENCY
      a) Peer address changed (dynamic WAN address at the branch)
         # show ip interface brief | include Dialer|Wan
         Compare with the configured peer at head office.
      b) Pre-shared key or certificate expired
         # show crypto pki certificates | include expire
      c) Head-office firewall change blocked IKE
         Check change records for the last 24 h.

   4. RESTART (disruptive: ~30 s for that branch only)
      # clear crypto session remote <peer>
      Wait 60 s. Re-run step 1.

   5. IF THE DEVICE IS UNREACHABLE
      Branch has an LTE backup: <details, and how to verify>
      Site contact: <name, number, and their working hours>

   6. ESCALATE
      After 30 minutes, or immediately if more than one branch:
      <name>, <number>. Out of hours: <different number>.

   AFTERWARDS
     Record the cause on the ticket.
     If the cause is not in step 3, add it.
   ════════════════════════════════════════════════════════
```

**Six properties of that document are doing the work:**

**The impact statement comes early.** **Knowing that card payment is unaffected changes what
the responder does in the first minute**, and it is the question a manager will ask.

**It says what healthy output looks like.** **A responder who has never seen the healthy output
cannot recognise the unhealthy one.**

**It checks the layer below first.** **Chapter 63's methodology, embedded in the procedure**
rather than left to the responder's judgement at 03:00.

**Causes are ordered by frequency**, not by the author's sense of elegance. **Most incidents are
the common cause.**

**Disruptive actions state their disruption.** **"~30 s for that branch only" is what lets
someone act without asking permission.**

**It escalates on a clock, not on a feeling.** **"After 30 minutes" removes the judgement call
that people get wrong under stress** — and the "immediately if more than one branch" clause
correctly identifies the case that is not what the runbook is for.

## What deserves a runbook

**Not everything. The ones worth writing:**

| Category | Examples |
|---|---|
| **Frequent** | tunnel down, circuit down, AP not joining, port errors |
| **High-impact** | core switch failure, DNS failure, firewall failover |
| **Rarely done and easy to get wrong** | **firmware upgrade, certificate renewal, failover test** |
| **Time-critical** | **DR invocation** (Chapter 56 §56.4), security incident containment |
| **Done by people who are not experts** | anything the service desk handles first |

**The third category is the underrated one.** **A procedure performed once a year is a
procedure nobody remembers**, and the runbook is the only thing standing between the operator
and improvisation. **Certificate renewal is the canonical example** — annual, critical, easy to
do wrong, and the person doing it has done it once before.

## When to write it

**Two habits, and they compound over a career more than any technical skill in this book.**

**Write the runbook during the first incident.**

> **While you are working out what is wrong, you are generating exactly the content the runbook
> needs** — the commands, the healthy output, the false leads, the order you tried things in.
> **Afterwards, that knowledge has been compressed into "oh, it's usually the peer address",
> and the detail is gone.**

**Keep a scratch file open during the incident and paste commands and output into it.** **The
tidying takes twenty minutes afterwards; the reconstruction takes two hours a week later and
produces something worse.**

**After every significant incident, ask what documentation would have made it shorter.** Then
create it.

**That question is better than "what was the root cause?"** for this purpose, **because it
produces an action rather than an explanation**, and because it is answerable by everyone
involved rather than only by the person who found the fault.

## The knowledge base

**Runbooks answer "what do I do?" A knowledge base answers "why is it like this?"**

**The category of information that is nowhere else:**

- **Why that odd static route exists**, and what breaks if it is removed
- **Why that VLAN is numbered inconsistently with the standard** — usually a merger
- **Which vendor bug this workaround is for**, with the case reference
- **The decision that was made and rejected**, and the reason
- **What was tried during the last incident and did not work** — which saves the next person
  from trying it again

> **The undocumented "why" is how a network becomes something nobody dares to change.** Every
> unexplained configuration is a thing that must be preserved because removing it might break
> something, **and after ten years the configuration is mostly things nobody understands.**

**Chapter 55 §55.1 treats this as accumulated liability.** **The knowledge base is the
mechanism that prevents it accruing.**

**The practical rule:** **any configuration that would make a competent engineer say "why is
that there?" needs a comment in the configuration and an entry in the knowledge base** — the
comment for the person reading the device, the entry for the person searching.

## The single point of knowledge

**The risk this whole chapter exists to reduce.**

> **One person who understands the addressing, the firewall policy, or why that static route
> exists is an availability risk in the same category as a single power feed**, and it should
> be treated as one.

**And it is usually invisible**, because the person is present and answers the question.

**Tests that reveal it:**

- **Can the network be operated for two weeks with that person unreachable?**
- **If they document something, does anyone else read it?**
- **When they are asked "where is that written down?", what happens?**

**The remedies are ordinary:** **rotate who handles incidents**, **require the runbook before
the change is closed**, **pair on unfamiliar work**, and **run the disaster recovery test with
that person deliberately excluded** (Chapter 56 §56.4).

## What breaks here

**A runbook that assumes knowledge the reader lacks.** **The commonest failure.** **Have someone
unfamiliar execute it in a non-incident**, and watch where they stop.

**A runbook that is out of date.** **The command changed, the hostname changed, the escalation
contact left.** **Date and owner on every one**, and review triggered by change (Chapter 55
§55.2).

**A knowledge base nobody searches.** **Usually because search is bad**, or because it lives
somewhere people do not go. **The tool matters less than whether it is the first place people
look.**

**Documentation written after the incident, from memory.** **Worse and slower than writing it
during.** The detail is already gone.

**An escalation path with a name and no number.** **At 03:00 the corporate directory may
require the VPN that is broken.** **Contact details belong in the runbook, in text.**

**Everything documented and the documentation unreachable during the outage.** **A wiki inside
the network that is down**, or requiring single sign-on that depends on a failed component.
**Keep an offline or independently hosted copy of the critical runbooks**, and test that
assumption.

> **Network+ note.** Objective 3.1 covers standard operating procedures and documentation.
> Over-learn: **runbooks and SOPs document routine and incident procedures**; **a knowledge
> base records solutions to known problems**; **change and incident records feed both**; and
> **documentation must be kept current to be useful.** The 03:00 test is not examinable and is
> the whole point.
