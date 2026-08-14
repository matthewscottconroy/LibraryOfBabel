# Chapter 55 — Configuration and Change Management

Ask any operations team what causes their outages and the honest answer, in
organisation after organisation and industry after industry, is the same:

Most unplanned outages are caused by planned changes.

Not hardware failure. Not attacks. Not capacity. Changes — made deliberately, by
competent people, following a process, that turned out to have a consequence nobody
anticipated.

This is not a criticism of the people making changes. It is a structural fact about
complex systems, and once you accept it the operational question becomes much more
tractable. If changes are the dominant cause, then the highest-leverage improvements
are: make fewer changes at once, know precisely what changed, be able to reverse it
quickly, and test the reversal before you need it.

That is the whole of change management, and everything else in this chapter is
mechanism.

## Configuration as accumulated liability

§55.1 begins with an unusual framing, which is that a device's configuration is not an
asset. It is a **liability that accumulates**.

Every line was added for a reason. Most of those reasons are undocumented. Some no
longer apply. Some contradict others in ways that are only visible in specific
circumstances. A ten-year-old firewall policy contains rules referencing servers that
were decommissioned in 2019, permitting protocols nobody uses, in an order that
matters, and nobody dares remove any of them because nobody knows what would break.

This is **configuration drift**, and its properties are worth naming because they
determine how to fight it:

- It accumulates monotonically. Nothing removes configuration by default.
- It is invisible. There is no alarm for "this rule has matched zero packets in three
  years," unless you build one.
- It compounds. Each layer of accumulated exception makes the next change harder to
  reason about, which makes it more likely to be made by addition rather than by
  correction.

The countermeasures — golden configurations, automated compliance checking, hit
counters on ACL entries, periodic review with a defined removal process — are the
subject of §55.1 and §55.4, and the honest observation is that they require sustained
management support because their benefit is invisible and their cost is not.

## Change control that is worth doing

§55.2 covers the process, and takes seriously the objection that change control is
bureaucracy — because badly implemented, it is.

The distinguishing question for any element of a change process is: does this reduce
the probability of an unnoticed failure, or the time to recover from one? Elements
that do neither are ceremony.

A change record that survives that test contains:

- **What is changing**, precisely, with the exact configuration diff.
- **Why**, in terms of the outcome sought.
- **The blast radius** — what could be affected if this goes wrong, which is usually
  larger than the person proposing it believes.
- **The verification** — how you will know it worked, stated *before* the change, as a
  specific observable rather than "check it works."
- **The rollback** — the exact steps to reverse it, and how long they take. A rollback
  plan reading "restore from backup" is not a plan until someone has timed it.
- **The window**, and who is available.

The last item deserves emphasis: the rollback must be tested, not merely written.
A great many rollback plans are discovered to be fiction at the moment they are
needed, most commonly because the backup was never verified restorable or because the
reversal requires a reboot nobody accounted for.

Two categories that make the process bearable: **standard changes** — pre-approved,
low-risk, well-rehearsed operations that do not require individual approval each time
— and **emergency changes**, which bypass approval but must be documented
retrospectively without exception. A process that treats a port description edit like
a core routing change will be circumvented, and a circumvented process protects
nothing.

## Lifecycle

§55.3 covers the calendar, which most organisations manage badly.

**End of sale** means you cannot buy more. **End of support** means no more security
patches. The second is the one that matters, and it is the field in your asset
inventory (Chapter 53 §53.2) that turns documentation into planning.

The uncomfortable arithmetic: network equipment is typically supported for five to
seven years after end of sale, budget cycles are annual, and procurement plus
installation takes months. An organisation that discovers its core switches went out
of support last quarter has a problem with no fast solution, and the failure was in
the inventory rather than in the switches.

Firmware carries a genuine dilemma that §55.3 addresses without pretending it is
simple. Upgrading risks introducing new defects and requires an outage window. Not
upgrading leaves known vulnerabilities in place, and network equipment vulnerabilities
are frequently pre-authentication and remotely exploitable — the class of defect that
gets an organisation on the news. The defensible position is a defined policy: a
regular maintenance cadence for routine updates, an expedited path for critical
security fixes with a stated maximum delay, and — importantly — not running the
newest release but the one that has been in the field long enough for its problems
to have been found by someone else.

## Backups

§55.4's central point is short and is the one people learn expensively:

> A backup you have not restored is not a backup. It is a hope with a filename.

Configuration backups should be automatic, versioned, stored off the device, and
restore-tested on a schedule. Version control is the natural home — the diff between
last week and today answers "what changed" instantly, which is the first question of
every incident, and it costs nothing to set up.

That capability is the direct link to Chapter 70's infrastructure as code, where the
configuration in version control stops being a record of the device and becomes the
**source of truth** from which the device is generated. §55.4 sets that up as the
natural destination of the practices in this chapter.

## By the end you will be able to

- Explain configuration drift and name three countermeasures.
- Write a change record that would let a colleague execute and verify the change.
- Distinguish a real rollback plan from an aspirational one.
- Justify standard and emergency change categories.
- Use an asset inventory's EOL data to produce a refresh plan.
- Argue a firmware policy position, addressing both the risk of upgrading and the
  risk of not.
