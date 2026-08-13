# 55.4 Backups and Rollback

> **A backup you have not restored is not a backup. It is a hope with a filename.**

**This section is short because its central point is short, and it is the one people learn
expensively.**

## What a configuration backup must be

**Four properties, and all four are required.**

| Property | Why |
|---|---|
| **Automatic** | **a manual backup is a backup that stops happening** |
| **Versioned** | **"what changed?" is the first question of every incident** |
| **Off the device** | **a device that failed cannot serve its own backup** |
| **Restore-tested** | **otherwise you do not know whether it is a backup** |

**And a fifth that is usually omitted:**

**Complete.** **The running configuration is not the whole device.** **Licences, certificates,
SSH host keys, local user databases, boot variables, the firmware image itself, and — on many
platforms — things that live outside the configuration file entirely.** **A "configuration
restore" onto a replacement chassis that then cannot authenticate anyone, or has a different SSH
host key that every automation tool refuses, is a restore that did not work.**

## Version control is the natural home

**And it is nearly free.**

```
   $ git log --oneline -- devices/sw-hq-02-1.cfg
   a3f21c9  CHG-8841 add supplier access rule
   7b2e4d1  CHG-8802 vlan 240 for new AV system
   1c9a8f3  automated collection 2026-04-12
   ...

   $ git diff HEAD~1 -- devices/sw-hq-02-1.cfg
   -  ip route 0.0.0.0 0.0.0.0 203.0.113.1
   +  ip route 0.0.0.0 0.0.0.0 203.0.113.1 200
   +  ip route 0.0.0.0 0.0.0.0 198.51.100.1
```

> **The diff between last week and today answers "what changed" instantly**, which is the first
> question of every incident, **and it costs nothing to set up.**

**The standard arrangement:** **`oxidized` or `rancid` collects every device's configuration on
a schedule and on change notification, normalises it, and commits it to git.** **A change
produces a commit; a commit produces a diff; a diff produces an answer.**

**Two refinements worth the effort:**

**Commit on change, not only on schedule.** **Most platforms can send a syslog message or an
SNMP trap when the configuration is written** (§54.3). **Trigger a collection on it**, and the
commit timestamp then reflects when the change happened rather than when the poller next ran.

**Link commits to change records.** **Put the change reference in the commit message**, so the
history answers not only "what changed" but "why, and who approved it."

### The thing to be careful about

> **Configurations contain secrets.** **SNMP community strings, RADIUS shared secrets, VPN
> pre-shared keys, local password hashes, wireless PSKs, API keys.**

**A git repository of network configurations is a high-value target**, and **it is frequently
protected far less carefully than the devices themselves.**

**Minimum measures:** **a private repository with access control and audit**; **encryption at
rest**; **no mirroring to a hosted service without deliberate assessment**; **and a scrubbing
step that removes or masks secrets before commit**, where the tooling supports it.

**And note that git remembers.** **A secret committed once is in the history forever unless the
history is rewritten**, and **rotating the secret is nearly always easier than rewriting
history.**

## Restore testing

**The part that is skipped, and the reason "restore-tested" is in the list above.**

**What restore testing must establish, and each has been discovered to fail in practice:**

1. **The backup exists** — for that device, at a recent date
2. **It is complete** — not truncated by a collection that timed out
3. **It can be read** — not corrupted, not encrypted with a lost key
4. **It applies to the target** — the replacement hardware accepts it
5. **The device works afterwards** — including the things that are not in the file
6. **You know how long it takes** — because the rollback plan (§55.2) states a duration

> **Item 6 is why testing matters even when the restore works.** **A rollback plan that says
> "restore the configuration" and turns out to take 45 minutes is a different plan from one
> that takes 4**, and the difference determines whether it is usable in a maintenance window.

**A workable cadence:** **restore one device per quarter, chosen at random, onto a lab or spare
unit.** **Record how long it took and what was missing.** **The missing list is the valuable
output** — it is almost never empty the first time.

## Rollback mechanisms on the device

**Several, in increasing order of how much they save you.**

**Copy the configuration first.** `copy running-config startup-config` is not a backup; **copying
the running configuration to a file with a timestamp before the change is.** **Trivial, and it
is the difference between reverting and reconstructing.**

**Configuration replace.** **On platforms that support it** — `configure replace` on IOS-XE,
`rollback` on Junos, `load override` — **the device computes the difference between the current
and target configurations and applies only what is needed.** **Vastly better than pasting the
old configuration back**, which frequently leaves the additions in place because **pasting adds;
it does not remove.**

> **This is the single most common rollback failure: pasting the previous configuration back
> and finding that the change is still there**, because the previous configuration does not
> contain the negation of what was added.

**Commit confirmed / rollback timer.** **§55.2's recommendation**, and it deserves repeating
because it is the highest-value habit in this chapter:

```
   Junos:     commit confirmed 5
   IOS-XE:    configure terminal revert timer 5
   others:    equivalent
```

**The device reverts automatically unless you confirm within the window.** **Which means a
change that removes your own management access costs five minutes rather than a site visit.**

**Configuration checkpoints.** **Junos keeps the last 50 committed configurations on the device
by default**; several platforms have equivalents. **`show configuration | compare rollback 3` is
an instant answer to "what did we change on Tuesday?"**

## Beyond the device: what else needs backing up

**Networks fail in places the device configuration does not cover.**

| | |
|---|---|
| **Controller and management platforms** | **wireless controllers, SD-WAN orchestrators, NAC, firewall managers** — and these hold the policy for hundreds of devices |
| **The monitoring system's own configuration** | Chapter 54 |
| **IPAM, DNS zones and DHCP scopes** | Chapter 53 §53.3 |
| **Certificate private keys and the CA** | **Chapter 58 — and losing a private CA is catastrophic and irreversible** |
| **RADIUS/TACACS policy and its database** | Chapter 59 |
| **The documentation itself** | Chapter 53, **and it must be restorable without the network** |
| **The automation repository** | Chapter 70 |

> **The controller is the sharpest of these.** **A wireless controller or firewall manager holds
> the configuration of the entire estate**, and **restoring it is the difference between a
> two-hour recovery and a two-week rebuild.** **Back it up, and restore-test it**, with the same
> seriousness as the devices it manages.

## The destination: configuration as source of truth

**Everything in this chapter points somewhere**, and it is worth naming the direction.

**Today, in most organisations:**

```
   Device  ──── is the truth ────▶  git repo (a record of it)
```

**The repository describes the device. If they disagree, the device is right and the repository
is stale.**

**The destination (Chapter 70):**

```
   git repo ──── is the truth ────▶  Device (generated from it)
```

**The repository defines the device. If they disagree, the device is wrong and is corrected.**

**And every property this chapter wants falls out of that inversion:**

| Want | How it follows |
|---|---|
| **Know what changed** | **it is a commit** |
| **Review before change** | **it is a pull request** |
| **Rollback** | **revert the commit and re-apply** |
| **No drift** | **drift is a diff, and it is corrected automatically** |
| **Golden configuration** | **it is a template, and every device is generated from it** |
| **Audit trail** | **it is the history, with authors and approvals** |

> **Configuration management, change control, drift detection, golden configurations and
> rollback are five separate disciplines in the model where the device is the truth. In the
> model where the repository is the truth, they are one mechanism.**

**That is Chapter 70's argument, and §55.4's practices are the honest route to it.** **An
organisation that has automated, versioned, restore-tested backups in git has already built
most of the infrastructure**; what remains is reversing the direction of authority, **which is
an organisational change more than a technical one.**

## What breaks here

**A backup that will not restore.** **Never tested.** The commonest and most expensive finding
in this chapter.

**A restore that leaves the device unable to authenticate.** **Licences, keys and the local user
database were not in the configuration file.** Completeness, not just currency.

**Pasting the old configuration back and the change is still there.** **Pasting adds; it does
not remove.** Use `configure replace` or its equivalent.

**Management access lost during a remote change.** **No commit timer.** Preventable, entirely.

**Secrets found in a public repository.** **The configurations were mirrored without
assessment.** **Rotate the secrets** — rewriting git history is harder and less reliable.

**A collection that has been silently failing for two months.** **Nobody monitors the backup
system.** Alert on the absence of a recent commit per device (§54.3's absence alerting).

**A wireless controller lost with no backup, and 300 access points to reconfigure.** **The
controller was not treated as a device.** It holds more configuration than any of them.

**A private CA lost.** **Irreversible.** Every certificate it issued must be replaced, and
Chapter 58 §58.3 explains why there is no shortcut.

> **Network+ note.** Objective 3.2 and 3.3. Over-learn: **configuration backups should be taken
> before changes and stored off-device**; **version control tracks configuration history**;
> **a rollback plan restores the previous known-good state**; and **backups must be tested.**
> The "untested backup is not a backup" point is examined in principle and learned in practice.
