# Chapter 55 — Important Concepts

Most unplanned outages are caused by planned changes *(intro)* — Not hardware failure, not
attacks, not capacity. Changes made deliberately, by competent people, following a process,
that turned out to have a consequence nobody anticipated. This is a structural fact about
complex systems, and accepting it makes the operational question tractable: make fewer
changes at once, know precisely what changed, be able to reverse it quickly, and test the
reversal before you need it.

Configuration is a liability that accumulates, not an asset *(§55.1)* — The value is in
the function it provides, not in the configuration itself. Two devices delivering identical
service, one with 200 lines and one with 2,000, are not equally good — the second carries
1,800 lines of unpaid debt that must be understood, preserved through every migration, reviewed
for security relevance, and cannot safely be removed.

**Drift accumulates monotonically** *(§55.1)* — Nothing removes configuration by default. A
new requirement produces a new rule; a problem produces a workaround; a migration leaves both
old and new "until we are sure", and nobody returns. Removals over ten years: approximately
none.

**Drift is invisible** *(§55.1)* — A configuration that is wrong in a way that breaks
something is discovered immediately; a configuration that is merely unnecessary is never
discovered at all. Drift is not detected by operation, only by deliberate inspection —
which is why it requires a process rather than attention.

Drift compounds, and the incentives point at addition *(§55.1)* — Restructuring four
interlocking exception rules requires understanding all four, and the risk of getting it wrong
falls entirely on the person who tries. This is the mechanism by which a network becomes
something nobody dares to change.

The rule did not change; the world did *(§55.1)* — A permit rule to 10.2.5.10 was correct
when that was a hardened server and is a hole when it has been reissued to a printer
(Chapter 53 §53.3's quarantine). The sharpest cost of drift is a security one.

Zero hits is not proof a rule is unnecessary *(§55.1)* — It is proof that nobody has asked
the question, and asking it is the point. Some rules exist for rare events — disaster
recovery paths, annual processes — and a zero count is expected.

**Disable before deleting** *(§55.1)* — Something that breaks while a rule is disabled is
restored in seconds; something that breaks after deletion is an archaeology exercise. Wait one
full business cycle.

Every exception gets an expiry date at the moment it is created *(§55.1)* — Because that
is the only moment when anyone knows why, and it costs nothing then. A year later,
establishing why costs hours and frequently fails. "Temporary" with no expiry date means
permanent with an inaccurate label.

Nobody is ever thanked for the outage that did not happen *(§55.1)* — The team that
removes 300 obsolete rules has, from the outside, produced nothing. So the argument must be
made in visible terms — time to change, audit findings, and above all migration cost, since
a refresh quoted against 1,100 lines and against 300 are visibly different projects. And
attach cleanup to work that is already funded: "we will rebuild from the standard rather than
migrate the existing configuration" is the most valuable decision available at a refresh.

The test for any element of a change process *(§55.2)* — Does it reduce the probability of
an unnoticed failure, or the time to recover from one? Elements that do neither are
ceremony, and ceremony consumes the goodwill the useful parts depend on.

The description can be wrong in ways the diff cannot *(§55.2)* — Record the exact
configuration diff, not a description of it.

The blast radius is about what shares a failure domain, not what you are changing *(§55.2)*
— A change to a switch's VLAN configuration has a blast radius of everything in that VLAN
anywhere, not of the switch. Prompts: what else is on this device, what routes through it,
what depends on the service it provides, what happens if it reboots unexpectedly mid-change.

State the verification before the change, as a specific observable *(§55.2)* — It forces the
proposer to define success, prevents its retroactive redefinition, and lets someone else
perform it. The commonest verification failure is checking that the change was applied rather
than that it worked — `show running-config` proves the configuration is there and nothing
about traffic.

"Restore from backup" is not a plan until someone has timed it *(§55.2)* — A real rollback
has exact commands, a measured duration, a test, stated prerequisites, and an identified point
of no return. The point of no return is most often missing and most important — some
changes become irreversible partway through, and the operator must know where that line is
before crossing it at 02:40.

Standard changes are what make the process survivable *(§55.2)* — A pre-approved list of
low-risk, rehearsed operations, added to whenever a normal change has been performed enough
times safely. The size of that list is a good indicator of a healthy process: a short list
means everything is treated as risky, which means the process is being circumvented.

Emergency changes must be documented retrospectively without exception *(§55.2)* — "We
were busy restoring service and never wrote it up" produces the undocumented change that becomes
the next incident's mystery. Make the record a condition of closing the incident, not a
task that competes with sleep.

**Change one thing** *(§55.2)* — Three changes together means three suspects and no way to
bisect. The temptation to batch is strongest in a scarce maintenance window, which is
exactly when a compound failure costs most.

Use a commit timer for every remote change *(§55.2, §55.4)* — `commit confirmed 5`,
`revert timer 5`. The device reverts unless you confirm, which converts "I have lost
management access" from an outage into a two-minute wait. The highest-value habit in this
chapter, and entirely preventable when omitted.

"It worked" is a statement about the moment of the change *(§55.2)* — "It is still
working, and nothing else got worse" requires a day. Check the graphs 24 hours later against
the same period on previous days (Chapter 54 §54.1) — this catches the change that was
technically successful and operationally wrong.

End of support is the date that matters *(§55.3)* — End of sale means you cannot buy more;
end of support means no patches, no replacements, no help. A device bought in the last
month before end of sale has the same EoL date as one bought four years earlier — check before
buying, not after.

From decision to complete is 6–18 months *(§55.3)* — Business case, procurement, lead time,
design, migration. An organisation that discovers its core switches went out of support last
quarter has a problem with no fast solution, and the failure was in the inventory rather than
in the switches. Report on EoL dates 24 months out, into the annual budget process, since
that is the only mechanism that produces money.

Running out of support is a legitimate decision and an indefensible discovery *(§55.3)* —
Documented, risk-assessed, with compensating controls and a date, it is a choice. Without
those it is a failure of the inventory.

Both columns of the firmware dilemma are real *(§55.3)* — Upgrading introduces defects and
requires an outage; not upgrading leaves known vulnerabilities. Network firmware
vulnerabilities are frequently pre-authentication and remotely exploitable, and a
compromised switch is a position from which to observe or modify all traffic — which is why
"it's only a switch" is the wrong instinct.

Do not run the newest release; run the suggested one *(§55.3)* — The one that has been in
the field long enough for its problems to have been found by someone else. Vendors label it,
it is typically several months behind, and deviating from it should require a specific
reason. Read the open caveats; standardise versions per role — an estate running eleven
versions of one platform is an estate where every problem is unique.

The timing asymmetry makes EoL urgent *(§55.3)* — The end-of-support date is known years
in advance; the vulnerability is not. You cannot plan a response to a vulnerability disclosed
next March in a device you cannot patch. You can plan the replacement now.

Four other lifecycles produce outages *(§55.3)* — **Certificates** (the most predictable
outage in this book); support contracts and licences (a lapsed contract means no replacement
part); circuits and commercial agreements; and **cryptographic algorithms** — SHA-1, TLS 1.0,
1024-bit RSA — the slowest lifecycle and the one nobody tracks.

A backup you have not restored is not a backup. It is a hope with a filename *(§55.4)*.

Automatic, versioned, off-device, restore-tested — and complete *(§55.4)* — The running
configuration is not the whole device. Licences, certificates, SSH host keys, local user
databases, boot variables, the image itself. A restore that leaves a device unable to
authenticate anyone is a restore that did not work.

The diff between last week and today is the first question of every incident *(§55.4)* —
And version control answers it instantly for nothing. Commit on change rather than only on
schedule, and put the change reference in the commit message so history answers "why" as
well as "what".

Configurations contain secrets, and git remembers *(§55.4)* — Community strings, RADIUS
secrets, PSKs, password hashes. A repository of network configurations is a high-value target
protected far less carefully than the devices. A secret committed once is in the history
forever, and rotating it is nearly always easier than rewriting history.

Pasting the old configuration back does not remove what was added *(§55.4)* — The most common rollback failure. Use `configure replace`, `rollback`, `load override` — a
mechanism that computes the difference and applies only what is needed.

Back up the controllers with the same seriousness as the devices *(§55.4)* — A wireless
controller or firewall manager holds the configuration of the entire estate, and restoring it
is the difference between a two-hour recovery and a two-week rebuild. And a lost private CA
is irreversible.

Inverting the direction of authority collapses five disciplines into one *(§55.4)* — When the
device is the truth, configuration management, change control, drift detection, golden
configurations and rollback are five separate practices. When the repository is the truth,
they are one mechanism: a commit, a pull request, a revert, a diff, a template, a history.
An organisation with automated, versioned, restore-tested backups in git has already built
most of the infrastructure — what remains is an organisational change more than a technical
one (Chapter 70).
