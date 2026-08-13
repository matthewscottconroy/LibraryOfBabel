# Chapter 55 — Exercises

## A. Recall

**A1.** State the observation about outage causes that this chapter opens with, and the four
consequences it implies for practice.

**A2.** Why is a configuration described as a liability rather than an asset? Give three
specific costs.

**A3.** Name the three properties of configuration drift and say what each implies about how to
counter it.

**A4.** What is a golden configuration, and what is the output of a compliance check against
one?

**A5.** Why is "zero hits on this rule" not sufficient grounds for deletion?

**A6.** State the test that distinguishes a useful element of a change process from ceremony.

**A7.** List the six things a change record must contain.

**A8.** What distinguishes a real rollback plan from an aspirational one? Give four differences.

**A9.** What is a point of no return, and why must it be identified in advance?

**A10.** Distinguish standard, normal and emergency changes, and say what each requires.

**A11.** Give the four lifecycle dates and say which one matters most and why.

**A12.** State the version-selection rule for firmware, and explain why it is not "the latest".

**A13.** Why are network equipment vulnerabilities unusually serious? Give two reasons.

**A14.** State the sentence about backups, and list the five properties a configuration backup
must have.

**A15.** Why does pasting a previous configuration back frequently fail to reverse a change?

## B. Apply

**B1.** Examine this ACL excerpt:

```
   10 permit tcp any host 203.0.113.10 eq 443   (18,220,441 matches)
   20 permit tcp any host 203.0.113.10 eq 80    (912,003 matches)
   30 permit tcp 198.51.100.0/24 any eq 3389    (0 matches)
   40 permit udp any host 203.0.113.12 eq 161   (0 matches)
   50 permit tcp any host 203.0.113.14 eq 22    (14 matches)
   60 deny ip any any                            (4,881,220 matches)
```

(a) Identify the entries that warrant review and say why for each.
(b) Rank them by security concern and justify the ranking.
(c) Write the removal process you would follow for the highest-concern entry.
(d) One of these might legitimately have zero hits. Which, and under what circumstances?

**B2.** Write a complete change record for: adding a second default route with a floating static
so that a new backup circuit is used only if the primary fails. Include all six required
elements, with the exact configuration diff, a specific verification and a timed rollback.

**B3.** For each of the following changes, state the blast radius. Be more thorough than the
proposer would be.

(a) Changing the native VLAN on one trunk port
(b) Upgrading firmware on one of two core switches
(c) Adding a firewall rule permitting a new supplier's address
(d) Changing the DHCP lease time on one scope
(e) Renaming a device

**B4.** Classify each of the following as standard, normal or emergency, and justify:

(a) Adding a port description
(b) Adding a VLAN to an access port from an approved list
(c) Changing an OSPF cost
(d) Disabling a compromised user's access at 02:00
(e) Applying a critical firmware patch for an actively exploited vulnerability
(f) Adding a new device to monitoring
(g) Replacing a failed switch with an identical spare

**B5.** An organisation has this inventory:

| Class | Count | EoL |
|---|---|---|
| Firewall pair | 2 | 2027-03 |
| Core switches | 2 | 2028-06 |
| Access switches | 60 | 2029-01 |
| Branch routers | 25 | 2027-11 |
| Wireless controller | 1 | 2027-06 |

Today is 2026-05.

(a) Assuming 12 months from decision to completion for the firewalls and controller, and 18 for
the core and access estate, give the "must decide by" date for each.
(b) Which items belong in this year's budget submission?
(c) State what you would do about anything that is already too late.

**B6.** A vendor's release train shows: 17.9.1 (latest), 17.6.5 (suggested), 17.3.8 (long-term
maintenance, security fixes only).

(a) Which would you deploy for a routine refresh, and why?
(b) A critical vulnerability is fixed only in 17.9.1. What do you do?
(c) Your hardware is not supported beyond 17.3. What are your options?

**B7.** Design the restore test for a network's configuration backups: what you would test, how
often, on what, what you would record, and what would constitute a failure.

## C. Analyse

**C1.** The chapter claims changes cause most unplanned outages. Analyse whether this is a
criticism of change processes or an argument for them, and state what a zero-change network's
availability would actually be.

**C2.** Analyse the incentive structure that causes configuration to be added rather than
corrected. Who bears the cost of restructuring, who bears the cost of not restructuring, and
what would change the balance?

**C3.** "Nobody is ever thanked for the outage that did not happen." Analyse this as a
management problem and propose three measurable proxies that would let preventive work be
argued for.

**C4.** Analyse the trade-off in firmware upgrades rigorously. Under what conditions is
"upgrade immediately" correct? Under what conditions is "never upgrade" defensible? Where does
the crossover sit, and what does it depend on?

**C5.** A process that treats a trivial change like a major one will be circumvented. Analyse
the dynamics of process circumvention: what triggers it, why it spreads, and why a circumvented
process is worse than no process.

**C6.** Analyse the "point of no return" concept. Give three examples of network changes that
have one, and say what the operator should do differently on each side of it.

**C7.** Analyse the claim that inverting the direction of authority — from device-as-truth to
repository-as-truth — collapses five disciplines into one. Is this accurate? What does it cost,
and what does it not solve?

**C8.** Configuration repositories contain secrets and are frequently protected less carefully
than the devices. Analyse this as a threat, including who the adversary is, what they gain, and
what the correct controls are.

## D. Design

**D1.** Design a golden configuration for a branch access switch: what it contains, what is
allowed to vary per device, how compliance is checked, how often, and what happens to a
detected difference. Include the triage process for legitimate local variation.

**D2.** Design a change management process for a team of eight engineers supporting 40 sites.
Specify the categories, the approval path for each, the required record contents, the standard
change list (at least eight entries), and the review cadence. Justify every element against the
test in §55.2 and state explicitly what you left out.

**D3.** Produce a three-year refresh plan from the inventory in B5, with dates, groupings and a
one-line risk statement per item. Present it as you would to a finance director, in one page.

**D4.** Design a firmware management policy: tracks, triggers, maximum delays, version selection,
testing, and rollback. Address explicitly what happens when a critical vulnerability affects a
device that cannot be upgraded.

**D5.** Design the configuration backup and version control arrangement for a 200-device estate:
collection mechanism, trigger, storage, secret handling, access control, monitoring of the
backup system itself, and the restore test regime. State what else besides device configurations
is backed up and why.

## E. Troubleshoot

**E1.** A change was applied successfully, verified, and an unrelated service broke four hours
later. Describe your investigation, and identify what in the change record would have made it
shorter.

**E2.** During a remote change, management access to the device is lost. Describe what should
have prevented it, what you do now, and what you would change afterwards.

**E3.** A rollback is attempted by pasting the previous configuration. The device still exhibits
the changed behaviour. Explain and give the correct mechanism.

**E4.** A configuration restore onto a replacement chassis succeeds, but no user can
authenticate and the automation tooling refuses to connect. Give two causes and the general
lesson.

**E5.** Compliance checking reports 3,100 differences across the estate. Describe how you would
approach this so that the check is still running in six months.

**E6.** A firewall rule permitting access to an internal address is found to be permitting
access to a printer. Explain how this happened and name the two processes that failed.

**E7.** Configuration backups have not been collected for one device for eleven weeks and nobody
noticed. Give the mechanism that would have detected it and say why the usual alerting approach
misses this class of problem.

**E8.** An emergency change restored service at 02:40 and was never documented. Six weeks later
an incident is traced to it. Describe the process change that prevents recurrence, and why
"remember to write it up" is not one.

**E9.** A firmware upgrade proceeds normally and the device fails to boot the new image.
Describe the recovery, and the two preparation steps that would have made it a ten-minute
problem rather than a four-hour one.

## F. Extend

**F1.** Set up `oxidized` or `rancid` against at least two devices (virtual routers are fine),
commit to git, make a change, and examine the diff. Then configure the device to notify on
configuration change and trigger a collection from it.

**F2.** Take a real or realistic firewall or ACL configuration and audit it: for every rule,
determine whether it can be justified, who owns it, and whether it has matched traffic. Report
the proportion you could not justify and how long the exercise took per rule.

**F3.** Use Batfish (or a similar configuration analysis tool) against a set of configurations
and run its built-in checks. Report what it found that a human review would have missed.

**F4.** Perform a full restore test: take a device's backup, restore it onto a different unit or
a virtual instance, and record everything that did not come across. Time the whole operation.

**F5.** Find the end-of-life documentation for a network platform you use or can access.
Determine its EoS, end-of-software-maintenance and EoL dates, and calculate how long an
organisation buying it today would have before it must be replaced.

**F6.** Read the release notes for two consecutive releases of a network operating system,
including the open caveats. Write a page on what you learned about the risk of upgrading, and
identify at least one caveat that would affect a network you know.

**F7.** Investigate a published outage caused by a network configuration change — there are many,
and several major providers have published detailed accounts. Map the incident against §55.2's
six change record elements and state which were absent or inadequate.
