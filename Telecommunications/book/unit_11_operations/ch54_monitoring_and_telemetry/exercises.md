# Chapter 54 — Exercises

## A. Recall

**A1.** Why can "is the network slow?" not be answered without a baseline?

**A2.** State the inconvenient corollary about when a baseline must be collected, and why it
makes baselining hard to fund.

**A3.** List six quantities a baseline should contain, and say what each is for.

**A4.** Explain why a five-minute average can hide a serious problem, and name the counter that
reveals it.

**A5.** Why do percentiles beat averages for latency? Give the specific percentile most
complaints come from.

**A6.** Name SNMP's five operations and say which direction each travels.

**A7.** Distinguish a TRAP from an INFORM, and say why the difference matters during an
incident.

**A8.** State the security properties of SNMP v1, v2c and v3, and name the two default community
strings.

**A9.** How long does a 32-bit octet counter take to wrap at 1 Gb/s and at 10 Gb/s? What is the
fix?

**A10.** Give the eight syslog severities in order, state which direction is more severe, and
say what "level 6" means operationally.

**A11.** Give three independent arguments for centralising logs.

**A12.** Why is NTP a prerequisite for log correlation? State the failure precisely.

**A13.** What does a flow record contain that an interface counter does not?

**A14.** Distinguish NetFlow/IPFIX from sFlow architecturally, and say what each is good at.

**A15.** State the four alerting rules, and give the test for whether something should be an
alert at all.

## B. Apply

**B1.** A 1 Gb/s link shows 40% utilisation on a five-minute average.

(a) Give two traffic patterns consistent with that figure.
(b) Compute the instantaneous rate if traffic runs at line rate for 24 seconds of each minute
and is idle otherwise.
(c) Which counter would distinguish the two patterns, and why?
(d) What polling interval would be needed to see the second pattern directly?

**B2.** A set of latency samples has mean 22 ms, p50 18 ms, p95 61 ms, p99 340 ms, max 2,100 ms.

(a) Which figure best predicts complaints, and why?
(b) The mean rises to 24 ms and p99 rises to 900 ms. Which change matters, and what might cause
it?
(c) A colleague computes the day's p95 by averaging the 24 hourly p95 values. Explain why this
is wrong.

**B3.** Compute the wrap time of a 32-bit octet counter at 10 Mb/s, 100 Mb/s, 1 Gb/s, 10 Gb/s
and 100 Gb/s. Repeat for a 64-bit counter at 100 Gb/s. State the polling interval that would be
safe for each 32-bit case and comment on whether it is practical.

**B4.** An estate has 400 devices with an average of 30 monitored interfaces each, and 8 objects
polled per interface.

(a) How many objects are polled in total?
(b) Compute the requests per second at 5-minute, 1-minute and 10-second intervals, assuming one
object per request.
(c) Recompute assuming GETBULK retrieves 20 objects per request.
(d) Comment on which intervals are feasible.

**B5.** Decode the syslog priority values 134, 30, 187 and 0 into facility and severity.

**B6.** Two devices log the following. Their clocks are not synchronised.

```
   fw-01     14:22:08  interface outside down
   rtr-core  14:26:11  BGP neighbour 203.0.113.1 Down
```

(a) State what you can and cannot conclude.
(b) NTP later shows fw-01 is 245 seconds slow. Restate the sequence.
(c) What would you have configured to avoid this?

**B7.** A collector receives sampled sFlow at 1:2000 from a 40 Gb/s link.

(a) A flow transfers 8 GB in 1,500-byte packets. How many samples would you expect?
(b) A reconnaissance scan sends one packet to each of 300 hosts. How many samples?
(c) State what each result implies about the suitability of this sampling rate for capacity
analysis and for security analysis.

**B8.** A network generates 3,500 flow records per second.

(a) How many records per day?
(b) At 48 bytes per record, how much storage per day before indexing?
(c) Design a retention policy with three tiers and justify each duration.

**B9.** Classify each of the following as page, ticket, dashboard or log, with a one-line
justification:

(a) A redundant power supply has failed on a core switch
(b) CPU on an access switch is 82%
(c) The primary internet circuit is down and the backup has taken over
(d) A TLS certificate expires in 21 days
(e) An access point has rebooted
(f) Users' authentication requests are failing
(g) Log storage will be full in 5 days
(h) A link has been at 96% for 12 minutes

## C. Analyse

**C1.** The chapter argues that deviation from expected shape is the signal and absolute
thresholds are a crude proxy. Analyse when absolute thresholds are nonetheless the right choice,
and what a shape-based system needs that a threshold-based one does not.

**C2.** Analyse the microburst problem: why does averaging destroy the information, why is the
discard counter a better detector than the utilisation graph, and what would you have to change
to see bursts directly?

**C3.** SNMPv3 has existed since 1998 and v2c remains widespread. Analyse why, being fair to the
people who have not migrated, and state what would actually change the outcome.

**C4.** Aggressive SNMP polling has caused outages. Analyse this as an instance of a general
pattern — measurement affecting the measured system — and identify two other examples in this
book.

**C5.** Analyse the claim that centralised logging is "cheap". What are the real costs, including
the ones that do not appear on an invoice?

**C6.** Sampling is described as "accurate for elephants and blind to mice". Analyse the
consequences for each of: capacity planning, chargeback, security monitoring and troubleshooting
a specific user's complaint.

**C7.** Flow data survives encryption. Analyse what this means for both network operations and
for privacy, and argue for a defensible position on retention.

**C8.** Streaming telemetry has existed for a decade and adoption is uneven. Analyse the
barriers honestly — technical, commercial and organisational — and predict which will resolve
first.

**C9.** "An alert nobody acts on is worse than no alert." Analyse the mechanism by which this is
true, and design a process that would detect the condition before an incident does.

## D. Design

**D1.** Design the monitoring for a 30-site organisation: what is measured, at what interval,
by what mechanism, where it is stored, for how long, and what is alerted on. Present it as a
table plus one page of justification, and state explicitly what you chose not to monitor.

**D2.** Design the logging architecture for the same organisation: collectors, transport,
retention tiers, access control, time synchronisation, and what happens when the central
collector is unreachable. Address the privacy obligations explicitly.

**D3.** Design a flow collection deployment for a network with a 100 Gb/s core, 10 Gb/s
distribution and 1 Gb/s access. Specify where flow data is collected, at what sampling rate,
why, and what questions each collection point is intended to answer.

**D4.** Design an alert set for a branch network: a maximum of eight alerts that page a human.
For each, state the condition, the duration, the expected action and why it justifies waking
someone. Then list five things you deliberately did not alert on and why.

**D5.** An organisation's monitoring system generates 400 alerts a day and everyone ignores it.
Design the remediation programme: what you measure first, what you change, in what order, and
how you would demonstrate improvement to a sceptical manager.

## E. Troubleshoot

**E1.** A 10 Gb/s link's graph shows periodic spikes exceeding 40 Gb/s. Diagnose and give the
fix.

**E2.** Users report poor performance. The link graph shows a steady 45%. Give the two things
you would check and what each would tell you.

**E3.** A device stops responding to SNMP under load but continues forwarding traffic normally.
Diagnose, and give two mitigations.

**E4.** After a reboot, a switch's traffic graphs attach to the wrong interfaces and the history
is lost. Explain and give the fix.

**E5.** A monitoring system reported no problems during a two-hour outage. List five distinct
reasons this could happen, and say which you would check first.

**E6.** Logs from a firewall are missing for the exact period of an incident. The device is
healthy. Give two causes and the structural fix.

**E7.** A search of centralised logs for "error" during an incident window returns nothing
relevant. Describe what you would do instead and why.

**E8.** Enabling ACL logging on a busy rule causes a router's CPU to reach 100% and traffic to
be dropped. Explain the mechanism and state the correct configuration.

**E9.** Flow records show 6 Gb/s on a link whose counters show 9 Gb/s. Give three possible
causes.

**E10.** A monitoring system has been silently dead for three weeks and nobody noticed. Explain
why this failure mode is particularly dangerous and give the specific mechanism that detects it.

## F. Extend

**F1.** Collect a week of interface statistics from a device you administer at both five-minute
and ten-second intervals. Plot both. Identify at least one event visible in one and not the
other, and explain it.

**F2.** Set up a syslog collector (rsyslog, syslog-ng, or a container running one) and point at
least three devices at it. Deliberately misconfigure NTP on one, generate correlated events, and
observe what the correlation looks like when the clocks disagree.

**F3.** Configure SNMPv3 with authPriv on one device and capture the traffic. Then configure
v2c and capture that. Compare what an observer learns from each. Write a paragraph on what you
would be disclosing.

**F4.** Enable flow export (NetFlow, IPFIX or sFlow) on a router or on a Linux host
(`softflowd`, `nfcapd`, or `pmacct`) and collect a day's data. Produce the top ten talkers and
the top ten conversations. Comment on anything you did not expect.

**F5.** If you have access to equipment supporting gNMI, subscribe to interface counters at
1-second intervals and compare the data volume and the visible detail against 5-minute SNMP
polling of the same counters. If you do not, read the OpenConfig interface model and write down
which fields have no SNMP equivalent.

**F6.** Audit an existing alert configuration: for every alert, record how many times it fired
in the last 90 days and how many times anyone took action. Present the result and propose
deletions. If you have no such system, do the exercise on a public example configuration.

**F7.** Investigate a published outage in which alert fatigue or missed monitoring was a
contributing factor. Summarise the mechanism and identify which of §54.4's four rules would have
changed the outcome.
