# Chapter 55 — Further Reading

## The evidence

Forsgren, N., Humble, J. & Kim, G. — *Accelerate: The Science of Lean Software and DevOps*
(2018).
**Read this one.** The only rigorous empirical treatment of whether change practices actually
work, with a defensible methodology. The four measures and the finding about external
approval are the parts that bear on §55.2, and they are uncomfortable in a productive way.

The annual *State of DevOps* / *DORA* reports (dora.dev).
Free, and the research is updated each year. Read the current one's section on change
management.

Vaughan, D. — *The Challenger Launch Decision* (1996).
Long, and the normalisation-of-deviance argument is worth the effort. If the whole book is
too much, her summary essays and the many talks derived from them convey it.

**Perrow, C. — *Normal Accidents* (1984).**
Where "tightly coupled" and "interactive complexity" come from. The introduction and the
Three Mile Island chapter carry the argument.

Cook, R. — "How Complex Systems Fail."
Recommended in Chapter 53 and it belongs here too. Points 4, 7 and 14 are about change
specifically.

## Practice

Limoncelli, T., Chalup, S. & Hogan, C. — *The Practice of System and Network
Administration*.
The standard operations reference, and the chapters on change management, on maintenance
windows and on debugging are directly this chapter. Practical, opinionated and honest about
what does not work.

Beyer, B. et al. — *Site Reliability Engineering*, chapters on release engineering,
configuration and on "Reliable Product Launches" (free at sre.google).
The launch checklist material is a good model for §55.2's change record, and the canary
release discussion is §55.2's staging argument done rigorously.

Kim, G., Behr, K. & Spafford, G. — *The Phoenix Project*.
A novel, and it is the most effective way to make a manager understand why batching changes
is dangerous. Read it for that purpose rather than for technical content.

Humble, J. & Farley, D. — *Continuous Delivery*.
Written about software and applicable throughout. The chapters on configuration management
and on deployment pipelines are the direct ancestors of Chapter 70's approach to networks.

Burgess, M. — *In Search of Certainty*, and the CFEngine documentation.
Where convergent configuration comes from. Dense, and the idea is worth the effort:
declarative desired state, converged repeatedly, is why drift is not a problem in a properly
automated estate.

Cunningham, W. — the 1992 OOPSLA experience report and his later "Debt Metaphor" video.
Five minutes, and it corrects the common misuse of the term.

## Standards and frameworks, used sparingly

**ITIL 4 change enablement material.**
Read a summary for the vocabulary — standard, normal and emergency changes come from here,
and the categories are genuinely useful. The implementation guidance is where organisations go
wrong.

NIST SP 800-128 — "Guide for Security-Focused Configuration Management."
Relevant if you must justify a configuration management programme to an auditor, and a
reasonable checklist of what one contains.

**CIS Benchmarks** (cisecurity.org) — hardening baselines for network platforms.
Free, specific and directly usable as the security portion of a golden configuration
(§55.1). Start here rather than writing your own.

**Vendor hardening guides** — Cisco, Juniper, Palo Alto and others publish them. The
"management plane" sections are the most valuable and the least read.

## Lifecycle and vulnerabilities

**Vendor end-of-life pages and product bulletin feeds.**
**Subscribe to them.** Cisco's EoL bulletins, Juniper's EOL notices and equivalents are
published years ahead, and F5 uses them.

NIST's National Vulnerability Database (nvd.nist.gov) and CISA's Known Exploited
Vulnerabilities catalogue (cisa.gov/kev).
The KEV catalogue is the more useful of the two for §55.3's policy — it lists what is
actually being exploited, which is a far smaller and more actionable set than everything with
a high CVSS score. Use it as the trigger for the emergency track.

**Vendor PSIRT advisories** — Cisco, Juniper, Fortinet, Palo Alto and others. **Subscribe.**
Network equipment advisories are frequently pre-authentication and remotely exploitable, and
the delay between advisory and exploitation has been measured in days.

Release notes, and specifically the open caveats sections.
**F6 uses these.** Long, tedious, and the only place where "this release breaks X" is
documented.

## Tools

**`oxidized` and `rancid`** — configuration collection into version control. **F1 uses one.**
`oxidized` is the more modern and easier to run; both do the essential job.

**git** — and it is genuinely sufficient. A repository, a scheduled collection and a diff
command is 80% of this chapter's value for an afternoon's work.

**Batfish** (batfish.org) — configuration analysis: reachability, ACL semantics, differential
analysis between two configuration sets. **F3 uses it.** It answers "will this change alter
what is reachable?" before the change is applied, which nothing else does well.

NAPALM, Nornir, Ansible, Netmiko — collection, comparison and application. Chapter 70
develops these; for this chapter, a Nornir script that diffs every device against a template
is the compliance check of §55.1.

`configure replace` / `rollback` / `load override` — learn your platform's version.
It is the difference between a rollback and a reconstruction.

`commit confirmed` / `revert timer` — learn this one first. It will save you a site
visit.

Suzieq, and the various "network state" tools — snapshot the operational state before and
after a change and diff it, which catches the consequences a configuration diff does not show.

## Post-mortems worth reading

Any major provider's account of a change-induced outage.
F7 uses one, and there is no shortage — AWS's 2017 S3 outage (a typo in a runbook
command), Facebook's 2021 BGP withdrawal (a configuration change plus a tooling failure that
locked out remediation), Cloudflare's several, and Google's various.

The Facebook 2021 incident is the single most instructive for this chapter: a change with
an unanticipated blast radius, an audit tool that failed to catch it, and a rollback path that
depended on the thing that had failed. All three of §55.2's elements, failing together.

## Where to look next

**Chapter 56** turns the lifecycle dates and change failure rates of this chapter into
availability arithmetic; **Chapter 60** is where §55.1's accumulated firewall policy actually
lives; **Chapter 63** uses the configuration diff as the first diagnostic step; and **Chapter
70** is the destination §55.4 describes — the repository as source of truth rather than as
record.
