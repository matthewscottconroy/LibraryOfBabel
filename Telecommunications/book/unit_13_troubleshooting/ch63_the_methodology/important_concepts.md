# Chapter 63 — Important Concepts

**Have a method, and use it precisely when you feel least inclined to** *(intro)* — **Instincts
under pressure are unreliable in specific, predictable ways**, which is why aviation insists on
procedure and why the same discipline transfers.

**There is always something in the firewall logs** *(§63.1)* — **So "I found something
suspicious" is not information.** **The evidence did not point at the firewall; you pointed at
the firewall and then found evidence.** **The countermeasure is to state, before looking, what
you would expect to see if the hypothesis were true and if it were false** — **if the answers
match, the observation cannot distinguish them.**

**Three changes and a resolution means you do not know what was wrong** *(§63.1)* — **You have
converted a diagnosable fault into one that will return.** Restoring service is legitimately the
priority; **the compromise is to record what you did in what order**, so the sequence is
reconstructible.

**A reboot may fix the symptom and guarantees you meet the problem again with no more
information** *(§63.1)* — It destroys interface counters, ARP and MAC tables, connection state,
adjacency state, volatile logs and the condition itself. **Thirty seconds of capture first**,
and on most platforms that is one command.

**"The VPN is broken" is a hypothesis presented as an observation** *(§63.1)* — **The observation
is "I cannot open the file share."** Starting from the first can waste an hour during which the
actual fault — a permissions change — remains undiagnosed.

**Frequently there are two faults, and the second is what makes the first hard to diagnose**
*(§63.1)* — **Complex systems run in degraded mode with latent faults present**, so an incident
is often one new fault interacting with one that was already there. **The signal is a fix that
improves things without resolving them** — take that seriously rather than concluding the fix
was wrong.

**Evidence is an observation that would have been different if your hypothesis were false**
*(§63.1)* — **A measurement, with a comparison, that discriminates.** "There are errors in the
log" is not; "error rate rose from 2/hour to 400/hour at 14:07" is. **A number with nothing to
compare it to is not evidence.**

**"Who and where is affected?" eliminates more than an hour of packet capture** *(§63.1)* —
**One user means the client or their port; one site means that site's link; everyone everywhere
means something central.** Three questions to the service desk.

**Constant and intermittent faults need entirely different methods** *(§63.1, §63.3)* — Constant:
bisect the path. **Intermittent: instrument and wait** — and the danger is that **a fault
recurring every few days appears to be fixed by whatever was done most recently.**

**The most valuable question is "when did it last work?"** *(§63.2)* — **Followed by "what
changed?"** Chapter 55 established that most unplanned outages are caused by planned changes,
**so the change log is the highest-yield place to look before anything else.**

**A fault you cannot reproduce is a fault you cannot confirm you have fixed** *(§63.2)* — **You
will make a change, the symptom will be absent, and you will not know whether you fixed it or
whether it was not happening.**

**"Question the obvious" means check the boring causes rather than dismiss them** *(§63.2)* —
A cable, a duplex mismatch, a full disk, an expired certificate, an exhausted lease. **The
boring causes are the common ones, and skipping them because they are beneath the symptom's
apparent complexity loses an hour regularly.**

**Form more than one theory** *(§63.2)* — **A single theory invites confirmation bias.** Two or
three force you to look for observations that distinguish them, **which is the useful kind of
looking.** **And the most interesting hypothesis is rarely the most probable one.**

**Confirming a theory is not establishing the mechanism** *(§63.2)* — **"The firewall is blocking
it" is confirmed; "rule 40, added at 09:15, shadows rule 80" is a mechanism**, and only the
second tells you what to do.

**Escalating early is cheap and escalating late is expensive** *(§63.2)* — **The reluctance is
social rather than technical.** **An engineer who escalates after twenty minutes with good notes
is more useful than one who escalates after three hours with none.**

**"It works from here" is not verification** *(§63.2)* — **Verify from the reporter's position**,
and **verify that nothing else broke** — the change had a blast radius.

**The method's value scales with the cost of being wrong** *(§63.2)* — **Seven formal steps for a
loose cable is theatre**, and treating the method as mandatory regardless of scale trains people
to ignore it. **Use it fully when the impact is large or the system is unfamiliar** — which is
precisely when you will least want to.

**"Other applications work" is a Layer 1 to 4 test already performed for you** *(§63.3)* — **Do
not repeat it.** It makes top-down the correct approach and bottom-up a waste.

**Bisection is logarithmic and linear search is not** *(§63.3)* — **31 elements: 31 tests or 5.**
**And it applies to more than layers** — bisect the path, the population, the time window, and
the change set.

**"Does it work by IP but not by name?" is the highest-yield single test in networking**
*(§63.3)* — **It eliminates Layers 1 through 4 and identifies the layer in one command**, in two
seconds.

**Test both directions** *(§63.3)* — **Asymmetric routing, a one-way ACL, a duplex mismatch and
a one-way fibre all look like general failures and are directional**, and testing only outbound
misses half of them.

**Check the clock** *(§63.3)* — **Clock skew breaks Kerberos, certificate validation and log
correlation**, and **it presents as an authentication problem rather than as a time problem.**

**The layer model is a search strategy, not a theory of the network** *(§63.3)* — **Middleboxes
span layers, encapsulation makes "which layer" ambiguous, and some faults — a licence expiry, a
capacity limit, a DNS record pointing at a decommissioned host — are not at a layer at all.**
**Abandon it the moment the evidence points somewhere it does not describe.**

**For an intermittent fault, instrument first and form theories last** *(§63.3)* — **Ring-buffer
capture running for days, with a trigger that marks the occurrence.** **The commonest failure is
that when it finally happens, nobody was capturing** — the instrumentation must be running
before, unattended, for as long as it takes.

**The record's value is not to you** *(§63.4)* — **It is to the person who meets this at 03:00
in eighteen months, and that person is frequently you, with no memory of it.**

**Record the mechanism, not the action** *(§63.4)* — **"Restarted the service" tells the next
person what to try; "the service exhausted its 1,024 file-descriptor limit" tells them what to
look for** — and only the second helps when the symptom differs slightly.

**Record what was ruled out** *(§63.4)* — **"We confirmed the circuit, the routing and DNS were
correct" saves forty minutes**, and it is almost never written, because eliminating things does
not feel like progress while you are doing it.

**Records are searched by symptom, not by cause** *(§63.4)* — **The person searching has the
symptom.** **Title by symptom, tag by symptom, and quote error messages verbatim** — because the
first thing anyone does is paste the error into a search box.

**"What documentation would have made this shorter?" beats "what was the root cause?"**
*(§63.4)* — **It produces an action rather than an explanation**, and everyone involved can
answer it.

**A blaming process buys you a worse understanding of your own system, permanently** *(§63.4)* —
**An engineer who expects blame will not tell you what actually happened.** **"Blameless" does
not mean consequence-free; it means the review's purpose is understanding**, and an organisation
that cannot separate those will not learn regardless of its process document.

**Near-misses are free information** *(§63.4)* — **"We nearly failed over to the standby and
found it had not been patched" is a finding with no incident attached**, and it is the cheapest
learning available.

**Time-to-diagnose is the metric that makes this chapter's value visible** *(§63.4)* — **It is
the fraction of MTTR that documentation, runbooks, monitoring and method actually reduce**, and
Chapter 56 §56.1 established that MTTR is where the availability leverage is. **And recurrence
is the metric that surfaces superficial fixes** — a fault fixed three times is a fault whose
cause was never found.

**An incident that produces a write-up and no actions has been documented and not learned
from** *(§63.4)* — **Post-incident actions need owners and dates, tracked like any other work**,
and that tracking is the entire difference between organisations that improve and those that do
not.
