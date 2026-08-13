# Chapter 63 — Exercises

## A. Recall

**A1.** Name the six cognitive failure modes in §63.1 and give the countermeasure for each.

**A2.** Why is "there is something in the firewall logs" not information?

**A3.** List six things a reboot destroys.

**A4.** Distinguish "the VPN is broken" from an observation, and say why the difference matters.

**A5.** Give the working definition of evidence, and the three properties that distinguish it
from an impression.

**A6.** Give the six opening questions in order, and say which eliminates most of the search
space.

**A7.** State the seven steps in order.

**A8.** Which step contains "question the obvious", and which contains "duplicate the problem"?

**A9.** Why is a fault you cannot reproduce a problem for step 6?

**A10.** State the four things a plan of action must contain.

**A11.** Give four circumstances in which escalation is the correct action.

**A12.** What does "verify full system functionality" mean, in its two parts?

**A13.** Name the four troubleshooting approaches and say when each is correct.

**A14.** Why does "other applications work" make bottom-up the wrong approach?

**A15.** Give the single highest-yield test in networking and say what it eliminates.

**A16.** State the method for an intermittent fault, and the commonest failure with them.

**A17.** Why does a record need to state the mechanism rather than the action?

**A18.** Which metric makes the value of documentation and method visible?

## B. Apply

**B1.** For each report, state the observation the reporter has actually made, the hypothesis
they have embedded, and your first question:

(a) "The internet is down"
(b) "The VPN is slow"
(c) "The server has crashed"
(d) "DNS is broken again"
(e) "Nobody can print"
(f) "The Wi-Fi doesn't reach my desk"

**B2.** For each scenario, choose an approach (bottom-up, top-down, divide-and-conquer,
follow-the-path) and justify in one sentence:

(a) One laptop has no network connectivity; everything else is fine
(b) One web application is unreachable; email, file shares and other sites work
(c) An entire branch site is unreachable
(d) Intermittent packet loss between two data centres
(e) A newly installed access point will not join the controller
(f) Everything is slow, everywhere, since 14:00

**B3.** A user cannot reach `app.example.com`. Design a bisection sequence of at most five
tests that will localise the fault. For each test, state what a pass eliminates and what a
failure eliminates.

**B4.** Rewrite each of these as an evidence statement:

(a) "The link seems flaky"
(b) "The firewall might be involved"
(c) "It's slower than usual"
(d) "The server is under load"
(e) "There are a lot of errors"

**B5.** You have three competing theories for a fault. Construct the table of "what I would see
if true / if false" for each, and identify which pair your planned observation cannot
distinguish.

Theories: (i) a routing change, (ii) a firewall rule, (iii) the destination server.

**B6.** Write the seven-line incident record for this scenario: a site lost connectivity for 90
minutes; the cause was a circuit failure at the carrier; the backup LTE link did not activate
because the SIM had been deactivated for non-use; service was restored when the carrier repaired
the circuit.

(a) Write the record.
(b) List the four outputs that should follow, with an owner type for each.
(c) State the metric this incident should worsen and by how much.

**B7.** An intermittent fault occurs roughly every three days and lasts under a minute. Design
the instrumentation: what you would capture, where, with what retention, what trigger, and how
you would correlate. State how long you would run it before drawing a conclusion.

**B8.** For each record, state what is wrong and rewrite it:

(a) "Rebooted the switch. Fixed."
(b) "CHG-4471 remediation complete."
(c) "DNS issue resolved."
(d) "Cleared ARP cache, restarted service, reseated cable. Working now."

## C. Analyse

**C1.** Analyse why confirmation bias is particularly dangerous in network troubleshooting
specifically. What is it about network devices' output that makes it easy to find supporting
evidence for any hypothesis?

**C2.** Under pressure the incentive is to restore service, which conflicts with preserving
evidence. Analyse this trade honestly and propose a rule that resolves it, stating what it costs.

**C3.** The chapter argues that the cost of guessing is not the wasted hour but the configuration
and documentation left behind. Analyse this claim and estimate the compounding cost over a
decade for a network where it happens monthly.

**C4.** Analyse the seven-step method's applicability. Construct the strongest case that it is
bureaucratic overhead, then the strongest rebuttal, and state where the boundary actually lies.

**C5.** Bisection is logarithmic and linear search is not. Analyse why engineers nonetheless
default to following the path, and whether that default is defensible.

**C6.** Analyse the layer model's limits as a diagnostic tool. Give three faults it cannot
locate, and propose what should supplement it.

**C7.** The chapter claims that "what documentation would have made this shorter?" is a better
question than "what was the root cause?". Analyse both, and say what each produces and misses.

**C8.** Analyse time-to-diagnose as a metric: what would improve it, what could game it, and why
it is more useful than time-to-resolve for justifying investment in this unit's practices.

**C9.** An organisation runs blameless post-incident reviews and its engineers still do not
volunteer information. Analyse the possible causes, and say what would have to be true for the
process to work.

## D. Design

**D1.** Write the troubleshooting standard for a network team of six: the method, when it
applies in full and when it does not, the roles during an incident, what must be captured before
any restorative action, the escalation criteria, and the documentation requirement. Two pages
maximum, and every element must pass the test of reducing either time-to-diagnose or
probability-of-recurrence.

**D2.** Design the incident record template your organisation would use, and justify each field
against a specific failure mode from §63.1 or §63.4. Then fill it in for a real or realistic
incident.

**D3.** Design the intermittent-fault investigation kit: the tooling, the capture strategy, the
triggers, the retention, and the runbook for deploying it. Specify what a technician who has not
done it before would need in order to set it up in twenty minutes.

**D4.** Design the post-incident review process for an organisation of 400 people: who attends,
what is asked, how actions are tracked, what is published and to whom, and how you would prevent
it becoming either a blame exercise or a formality.

**D5.** Design the metrics dashboard for troubleshooting effectiveness: what is measured, how it
is captured without adding burden, what a good and a bad value looks like, and what decision
each metric supports.

## E. Troubleshoot

**E1.** An engineer spends two hours on a firewall and the fault turns out to be a DNS record.
Identify every failure mode from §63.1 that could have contributed, and say what single question
would have prevented it.

**E2.** A fault is fixed by restarting a service. It recurs eleven days later. Analyse what was
lost and what should have been done at the first occurrence.

**E3.** A team makes three changes during an incident and service is restored. Describe what
they now do not know, and design the minimum record that would have preserved it.

**E4.** A fault is intermittent, has been "fixed" three times, and recurs. Explain the mechanism
by which each fix appeared to work, and describe the approach that would settle it.

**E5.** An investigation concludes that authentication is failing, and the cause turns out to be
NTP. Explain the connection and state which cross-layer check would have found it.

**E6.** A fault affects traffic in one direction only and was diagnosed as a general failure.
Give three faults with this property and the test that reveals directionality.

**E7.** A knowledge base contains an entry for exactly this symptom and nobody found it. Give
three reasons and the fix for each.

**E8.** During an incident, the engineer diagnosing is also answering status requests. Analyse
the cost and give the organisational fix.

**E9.** A fix resolves 80% of the symptom. The engineer concludes the fix was wrong and reverts
it. Assess this decision.

## F. Extend

**F1.** Take three incidents from your organisation's records (or three published outage
post-mortems) and score each against the seven steps: which were performed, which were skipped,
and what the skipping cost. Present the result as a table.

**F2.** Instrument an intermittent fault in a lab: create one deliberately (a script that
disables an interface for two seconds at random intervals), then set up ring-buffer capture,
counter polling and a trigger, and demonstrate that you can identify the event afterwards from
the data alone.

**F3.** Audit a knowledge base you have access to: sample twenty entries and score each on
whether it records the mechanism, what was ruled out, the evidence, and whether it is findable
by symptom. Report the proportions.

**F4.** Run a blameless post-incident review for a real or simulated incident, using §63.4's
questions. Record which questions produced information that the usual questions would not have.

**F5.** Measure time-to-diagnose on the next five incidents you are involved in, separately from
time-to-detect and time-to-resolve. Report the distribution and identify the largest single
contributor to the diagnosis time.

**F6.** Read a published post-incident report from a major provider and map it onto §63.1's
failure modes: which, if any, are visible in the narrative? Note that good reports frequently
document their own cognitive errors, and identify whether this one does.

**F7.** Take a fault you have solved recently and write the record twice: once as you would have
under time pressure, and once to §63.4's standard. Have a colleague attempt to use each to
diagnose a similar fault, and report the difference.
