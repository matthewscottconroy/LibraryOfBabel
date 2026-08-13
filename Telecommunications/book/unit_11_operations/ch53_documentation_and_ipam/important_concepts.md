# Chapter 53 — Important Concepts

**Documentation is a cache** *(intro)* — **Work performed once, calmly, with the equipment in
front of you**, so it need not be performed repeatedly under pressure by people without that
context. **Like any cache it has a hit rate and a staleness problem, and both are manageable.**
Its entire value is realised at the moments when thinking is hardest.

**Three diagrams, not one** *(§53.1)* — **Physical answers "what is plugged into what";
logical answers "what talks to what"; routed answers "how do segments reach each other".**
Chapter 11 §11.3's independence of physical and logical topology, turned into practice.

**Different change rates are the practical argument for separating them** *(§53.1)* — A
combined diagram must be updated when **any** of the three changes, **which is why combined
diagrams are always stale.**

**The physical diagram is about copper, glass and rack units** *(§53.1)* — Ports by their real
identifiers, patch panel positions at both ends, **and the duct or riser anything uses** —
Chapter 50 §50.3's shared risk at building scale. **If you find yourself writing a subnet on
it, you are drawing the wrong diagram.** The test: **could someone who has never been in the
building find the far end of a cable using only this?**

**A diagram showing every link as active is a lie about a spanning-tree network** *(§53.1)* —
**Mark the root bridge and the blocked ports**, because the actual traffic path is the one with
them removed — **and because a link that has been blocking for three years is a link nobody has
verified works.**

**Put the circuit reference on the routed diagram** *(§53.1)* — **It is the string the carrier
will ask for at 03:20 and nothing else will do**, and it is almost never there.

**Diagrams show structure; databases hold instances** *(§53.1)* — **If a thing is one of many
identical things, it belongs in a table.** Forty identical access switches are one box and a
reference. **A diagram that requires zooming is a diagram nobody reads.**

**An undated diagram is an assertion with no evidence** *(§53.1)* — Title, date, author,
version and key, on every one. **Without them a reader cannot tell whether it describes the
network or the network of four years ago**, and will either trust it wrongly or ignore it.

**Calendar review fails; change-triggered review works** *(§53.1, §53.4)* — **A change is not
complete until the documentation reflects it**, enforced as a required field on the change
record. **Slipping a quarterly review is invisible until an incident.**

**Automated discovery and hand-drawn diagrams do different jobs** *(§53.1)* — **Discovery
produces a current, ugly, complete picture; a drawing produces an intelligible, curated,
out-of-date one.** Keep both, **use the diff to detect divergence, and treat divergence as a
defect.**

**Label by location, never by purpose** *(§53.2)* — **Purpose changes; location does not.**
`A-3-14 → B-1-07` is still correct in five years; `Accounts printer` is wrong within eighteen
months. **And a wrong label is worse than none: an unlabelled cable is honestly unknown, a
wrongly labelled one is confidently wrong.**

**Label both ends, with the same information** *(§53.2)* — **The end you can reach is never the
one you need.** Printed, wrap-around, and **the panel ports labelled permanently at
installation** — the panel outlives every cable plugged into it.

**Port and patch records turn a twenty-minute walk into a lookup** *(§53.2)* — Outlet → panel →
panel port → switch → switch port. **The MAC-table-to-physical-outlet path is the commonest
operational lookup in an access network**, and this record is what makes it possible without
walking. It is also what makes port-based security and decommissioning practical.

**Airflow direction belongs on the rack elevation** *(§53.2)* — **Network switches are
frequently side-to-side or back-to-front, which is the opposite of every server beside them.**
The result is a device ingesting its neighbours' exhaust — **a slow, intermittent,
temperature-dependent fault.** And **a rack with no spare units cannot accept a replacement
during an incident.**

**An inventory without end-of-support dates answers no strategic question** *(§53.2)* — "What
must we replace next year and what will it cost?" is the only question anyone asks of it.
**And EOL is a security input**: a device past support receives no patches, **so a vulnerability
disclosed after that date is permanent.**

**Two inventory fields that standard templates omit** *(§53.2)* — **"Why does this exist?"** —
one sentence, written at installation, prevents the device nobody dares decommission. **"What
breaks if this is turned off?"** — the blast radius, recorded when it is cheap to determine.

**The out-of-hours support number is different from the daytime one** *(§53.2)* — Not a detail.
**It is the difference between reporting a fault at 03:20 and reporting it at 09:00.**

**A plan is a design document; IPAM is a live record** *(§53.3)* — **A plan without a record is
a network where nobody knows what is in use**, and the observable symptom is that people choose
addresses by pinging to see whether anything answers — **which is how you assign an address to a
machine that is merely switched off.**

**The static-inside-the-pool failure** *(§53.3)* — A static assignment inside a DHCP range works
for months, **until the pool fills and reaches it.** Then **two unrelated machines have
intermittent, protocol-dependent failures at unpredictable times**, depending on whose ARP entry
each other device holds, **and nothing in either configuration is wrong.** **The fix is
structural: IPAM records the range and refuses the assignment.**

**Quarantine released addresses** *(§53.3)* — **An address freed today should not be reissued
tomorrow.** Stale ARP entries, DNS caches, firewall rules and monitoring all still refer to it.
**Block during quarantine** — something that breaks then is trivially restored; something that
breaks after release is a mystery.

**A spreadsheet is genuinely adequate at small scale** *(§53.3)* — **A maintained spreadsheet
beats an unmaintained IPAM system.** What changes the calculation is **integration**: DNS, DHCP
and IPAM created together or not at all. **Integrated DDI removes an error class by
construction rather than by discipline** — and discipline fails predictably, because **the step
skipped under pressure is always the record-keeping one.**

**IPv6 changes the question** *(§53.3)* — **The IPv4 question is "which addresses are free?"
The IPv6 question is "which addresses are in use, and by what?"** Scarcity is gone and prefixes
can be semantic; **but addresses are unmemorable so DNS becomes mandatory, a host has several
addresses at once, and SLAAC assigns without telling anyone.** Harvest neighbour caches; accept
that privacy addresses are not allocations.

**The runbook test: 03:00, competent, unfamiliar, without waking anyone** *(§53.4)* — **Most
fail, because they were written by the person who already knew**, who omitted everything obvious
to them. **"Check the tunnel state" needs to be the exact command, on which device, with the
healthy output shown.**

**Six properties do the work in a good runbook** *(§53.4)* — **Impact stated early**; **healthy
output shown**; **the layer below checked first** (Chapter 63's methodology, embedded rather
than left to judgement at 03:00); **causes ordered by frequency**; **disruptive actions stating
their disruption**; and **escalation on a clock rather than on a feeling.**

**The rarely-done procedure is the underrated runbook** *(§53.4)* — **A procedure performed once
a year is a procedure nobody remembers.** **Certificate renewal is the canonical case**: annual,
critical, easy to do wrong, and the operator has done it once before.

**Write the runbook during the first incident** *(§53.4)* — **While you are working it out you
are generating exactly the content the runbook needs** — commands, healthy output, false leads,
the order you tried things. **Afterwards it has compressed into "oh, it's usually the peer
address" and the detail is gone.** Keep a scratch file open and paste into it.

**"What documentation would have made this shorter?" beats "what was the root cause?"**
*(§53.4)* — **It produces an action rather than an explanation**, and everyone involved can
answer it, not only the person who found the fault.

**The undocumented "why" is how a network becomes unchangeable** *(§53.4)* — Every unexplained
configuration must be preserved in case removing it breaks something, **and after ten years the
configuration is mostly things nobody understands.** **Anything that would make a competent
engineer ask "why is that there?" needs a comment in the device and an entry in the knowledge
base.**

**A single point of knowledge is an availability risk in the category of a single power feed**
*(§53.4)* — **And it is invisible, because the person is present and answers the question.**
Test it: **can the network be operated for two weeks with them unreachable?** Remedies are
ordinary — rotate incident duty, require the runbook, pair on unfamiliar work, **and run the DR
test with that person deliberately excluded.**

**Documentation unreachable during the outage is documentation you do not have** *(§53.4)* — A
wiki inside the failed network, or one requiring single sign-on that depends on a failed
component. **Keep an independently hosted copy of the critical runbooks, and test that
assumption.**
