# 55.2 Change Control

Most unplanned outages are caused by planned changes, made deliberately, by competent
people, following a process, that turned out to have a consequence nobody anticipated.

Once you accept that, the operational question becomes tractable: make fewer changes at
once, know precisely what changed, be able to reverse it quickly, and test the reversal before
you need it.

That is the whole of change management. Everything else is mechanism.

## The test for every element of the process

Change control has a deserved reputation as bureaucracy, because badly implemented, it
is.

> Does this element reduce the probability of an unnoticed failure, or the time to recover
> from one?

Elements that do neither are ceremony, and ceremony is not neutral: it consumes the
goodwill that the useful parts of the process depend on.

| Element | Test |
|---|---|
| **Recording the exact diff** | **passes** — it is the first question of every incident |
| **A tested rollback** | **passes** — directly reduces recovery time |
| **Stating the verification in advance** | **passes** — reduces unnoticed failure |
| A second engineer reviewing | **passes** — catches a real class of error |
| **Three signatures from people who will not read it** | **fails** |
| A form field for "business justification" nobody reads | **fails** |
| **A four-week lead time for a low-risk change** | **fails, and worse** — it drives work outside the process |

The last row is the one that destroys processes. A process that treats a port description
edit like a core routing change will be circumvented, and a circumvented process protects
nothing while still costing everything.

## What a change record must contain

**Six things, and each earns its place.**

### What is changing, precisely

The exact configuration diff, not a description.

```
   -  ip route 0.0.0.0 0.0.0.0 203.0.113.1
   +  ip route 0.0.0.0 0.0.0.0 203.0.113.1 200
   +  ip route 0.0.0.0 0.0.0.0 198.51.100.1
```

> "Change the default route to prefer the new circuit" is a description. The three lines
> above are a change. The description can be wrong in ways the diff cannot.

### Why, in terms of the outcome sought

Not "as requested by the applications team" but "so that outbound traffic uses the new 1 Gb/s
circuit and fails back to the existing one."

Because the reviewer's job is to check whether the change achieves the outcome, and they
cannot do that without knowing what it is.

### The blast radius

What could be affected if this goes wrong — and it is usually larger than the person
proposing it believes.

> The proposer thinks about what they are changing. The blast radius is about what shares a
> failure domain with it.

**Prompts that produce a better answer:**

- **What else is on this device?**
- What routes through it?
- What depends on the service it provides — DHCP, DNS, authentication, time?
- What happens if it reboots unexpectedly during the change?
- Who is on the other side of every link?

A change to a switch's VLAN configuration has a blast radius of everything in that VLAN
anywhere, not of the switch.

### The verification, stated before the change

As a specific observable, not "check it works."

| Poor | Good |
|---|---|
| "Verify connectivity" | **`ping 10.2.5.10` from the branch returns < 20 ms** |
| "Check routing" | **`show ip route 0.0.0.0` shows next hop 198.51.100.1** |
| "Confirm no impact" | **the three graphs named below match the same hour last Tuesday** |
| "Test the application" | **a named person loads a named page and confirms** |

**Stating it in advance does three things.** It forces the proposer to think about what
success looks like, which frequently reveals that they had not. It prevents the retroactive
redefinition of success that follows an ambiguous outcome. And it lets someone else perform
the verification.

> The commonest verification failure is checking that the change was applied rather than that
> it worked. `show running-config` proves the configuration is there. It proves nothing
> about traffic.

### The rollback

The exact steps to reverse it, and how long they take.

> A rollback plan reading "restore from backup" is not a plan until someone has timed it.

What separates a real plan from an aspirational one:

| | Aspirational | Real |
|---|---|---|
| Steps | "revert the change" | **the exact commands, in order** |
| Duration | unstated | **"4 minutes, of which 90 s is a reload"** |
| **Tested?** | **no** | **yes, in a lab or a previous window** |
| Prerequisites | assumed | **"requires console access if the management VLAN is affected"** |
| **Point of no return** | **not identified** | **"after step 6 the old configuration cannot be restored without a maintenance window"** |

The point of no return is the item most often missing and most important. Some changes
become irreversible partway through — a firmware upgrade that migrates a configuration
format, a database schema change, a licence activation — and the person executing needs to
know exactly where that line is before they cross it at 02:40.

> A great many rollback plans are discovered to be fiction at the moment they are needed,
> most commonly because **the backup was never verified restorable** (§55.4) or because the
> reversal requires a reboot nobody accounted for.

### The window, and who is available

When, how long, and — the part that is skipped — who else is contactable.

"The vendor's support contract covers 24×7" is not the same as "someone at the vendor will
answer within 20 minutes at 03:00 on a Sunday", and the difference is discovered during the
incident. Test the escalation path before you rely on it.

## The three categories

A process becomes bearable when it distinguishes risk.

| | **Standard** | **Normal** | **Emergency** |
|---|---|---|---|
| Approval | **pre-approved** | **reviewed and approved** | **bypasses approval** |
| Examples | port description, VLAN on an access port, adding a monitored device | routing change, firewall rule, firmware | **restoring service** |
| Lead time | **none** | days | **none** |
| Record | **logged after** | **before** | **after, without exception** |
| Requires | **a documented, rehearsed procedure** | review | **retrospective documentation** |

Standard changes are what make the process survivable. A defined list of low-risk,
well-rehearsed operations that do not require individual approval each time — reviewed
periodically, and added to whenever a normal change has been performed enough times safely.

> The size of the standard change list is a good indicator of a healthy process. A short
> list means everything is treated as risky, **which means the process is being circumvented.**

Emergency changes bypass approval and must be documented retrospectively without exception.

**The exception clause is where processes fail.** "We were busy restoring service and never
wrote it up" produces the undocumented change that becomes the mystery in the next incident
(Chapter 53). Make the retrospective record a condition of closing the incident, not a
separate task that competes with sleep.

## Reducing the risk of the change itself

Process is not the only lever. Five practices reduce risk directly.

**Change one thing.** When three changes go in together and something breaks, you have three
suspects and no way to bisect. The temptation to batch is strongest during a scarce
maintenance window, which is exactly when the cost of a compound failure is highest.

**Have a commit timer.** Many platforms support a timed rollback — `commit confirmed` on
Junos, `configure terminal revert timer` on IOS-XE, `commit confirmed` on many others. The
device reverts automatically unless you confirm within N minutes, which converts "I have lost
management access" from an outage into a two-minute wait. Use it for every remote change to a
device you cannot easily reach.

**Stage it.** One site, then a few, then the rest — and wait long enough between stages
for a slow-appearing problem to appear. A change that breaks something only under load breaks
it at 09:00, not at 02:00.

**Verify from the user's position.** Chapter 63's first question. A change verified from
the network operations centre and not from a branch is half verified.

Do not change at the start of a period nobody is watching. A change made at 17:00 on a
Friday is a change whose consequences appear when nobody is available, and the practice of
Friday freezes exists for that reason and is defensible.

## The post-change review

**Cheap and skipped.**

Twenty-four hours after a significant change, check the graphs against the same period on
previous days (§54.1). This catches the class of change that was technically successful and
operationally wrong — a routing change that works and shifted traffic onto a more expensive
path, a firewall rule that permits what was intended and also something else.

> "It worked" is a statement about the moment of the change. "It is still working, and
> nothing else got worse" requires a day.

## What breaks here

A change that worked and broke something unrelated. Blast radius was understood too
narrowly. Add the failure it revealed to your prompt list.

Three changes in one window and one of them broke something. Bisection is now expensive.
One at a time.

**A rollback that could not be performed.** **Untested.** Time it and rehearse it, or accept
that you do not have one and say so on the record.

Management access lost partway through a remote change. **No commit timer.** This is
entirely preventable and it is the commonest self-inflicted remote outage.

A verification that passed and users still affected. Verified applied, not verified
working, or verified from the wrong place.

**An undocumented emergency change surfacing months later.** The retrospective record was
optional. Make it a condition of closing the incident.

**A process everyone circumvents.** Look at what it costs to follow. If a two-minute change
takes an hour of process, the process is producing the risk it was meant to prevent —
because the change will happen anyway, unrecorded.

> **Network+ note.** Objective 3.2 covers change management. Over-learn: a change request
> documents the change, its purpose, its risk and its rollback plan; **changes should be
> approved and scheduled**; **a rollback plan is required**; and **configuration backups should
> precede changes.** The rollback-plan requirement is examined; testing it is the part that
> matters and is not.
