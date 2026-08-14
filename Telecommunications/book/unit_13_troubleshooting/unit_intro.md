# Unit XIII — Diagnosis

You have been troubleshooting since Chapter 1.

Every chapter of this book has closed with failure modes. Whitehouse mistook a
signal-integrity problem for a power problem. A wrong subnet mask produced selective
connectivity. A missing DHCP relay produced APIPA addresses on every subnet but one. A
blocked ICMP message produced connections that established and then hung. A shadowed
ACL rule had no effect. A stateful firewall dropped return traffic on an asymmetric
path.

This unit does not introduce troubleshooting. It **formalises** a method you have
already been using, and it does so at the end rather than the beginning for the same
reason that Unit V put layering after Unit IV: a method presented before you have
problems to apply it to is a procedure to memorise, and a method presented after is a
name for something you were already doing imperfectly.

## Why this is the largest domain

CompTIA's N10-009 blueprint assigns 24% to troubleshooting — the largest single
domain, larger than networking concepts, larger than implementation, larger than
security. Employers, asked what they want from a junior network hire, describe
diagnosis before they describe configuration.

The reason is straightforward. Building a network is a project, done once, with time
to think and colleagues to consult. Operating one is continuous, and the operational
work is overwhelmingly *finding out why something that used to work has stopped*. An
engineer who can configure OSPF but cannot determine why a user's application is slow
is useful for two weeks a year.

## The central discipline

If the unit has one idea, it is this:

> **Evidence before hypothesis.**

The natural human response to a problem is to think of a likely cause and act on it.
It is fast, it is frequently right, and when it is wrong it is expensive in a
particular way: acting on an untested hypothesis *changes the system*, which destroys
the evidence that would have identified the real cause, and adds a new variable to a
situation that already had too many.

The engineer who reboots the switch, replaces the cable, and restarts the service —
and then finds the problem has gone — has learned nothing, cannot prevent recurrence,
and has possibly introduced a second fault that will surface next week. The engineer
who reads the interface counters first spends four more minutes and knows.

Chapter 63 formalises this as a seven-step method. Chapter 64 gives the tools that
produce the evidence. Chapter 65 is a catalogue of failure modes organised by layer
and by symptom. Chapter 66 covers performance problems, which are the hardest class
because nothing is broken.

## The layered method, which is what OSI is for

Chapter 22 §22.4 claimed that the OSI model's lasting value is diagnostic, and this is
where the claim is cashed.

The model's use is **bisection**. Seven layers, and every piece of evidence eliminates
some of them:

- Link light on → Layer 1 is probably fine.
- `ping` to the gateway succeeds → Layers 1, 2 and 3 work locally.
- `ping` to a public address succeeds → routing and NAT work.
- `ping` by name fails while `ping` by address succeeds → everything below Layer 7
  works, and the fault is name resolution. One test, six layers eliminated.
- TCP connects but the application fails → Layers 1 through 4 are fine.

Each observation halves the space. Six or seven well-chosen tests will locate almost
any fault, which is a dramatically better strategy than trying likely causes in order
of familiarity.

The corollary, which §63.3 develops: choose the test that eliminates the most, not
the test that confirms your suspicion. A test whose result you can predict tells you
nothing. The valuable test is the one you genuinely do not know the answer to.

## What this unit assumes

That you have read the rest of the book. Chapter 65 is a catalogue of failure modes
and it is only useful to someone who knows the mechanisms that are failing — "wrong
native VLAN" means nothing without Chapter 20, "PMTUD black hole" means nothing
without Chapters 24 and 34, and "bufferbloat" means nothing without Chapters 3 and 38.

That is the argument for putting this unit here. A troubleshooting course taught first
produces someone who can follow a flowchart. A troubleshooting unit taught after
twelve units of mechanism produces someone who can diagnose a fault the flowchart does
not cover, which is most of them.

## And the habit worth building now

Whatever you find, **write it down**. The symptom, the evidence, the cause, the fix,
and — most valuable and most often omitted — the thing that made it hard to find.

Networks fail in patterns. The fault you spend three hours on today will recur in
eighteen months, in a different building, reported differently, and the person facing
it may be you having forgotten. A searchable record of past incidents is the highest-
return documentation any team maintains, and Chapter 63 §63.4 makes the case
properly.
