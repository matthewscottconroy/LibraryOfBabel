# Further Reading

**Brian Goetz and others, *Java Concurrency in Practice* (2006).** Still the book
on this subject, twenty years on, and the co-authors include Doug Lea. Chapters 2
and 3 on thread safety and shared objects are the careful version of Section
31.1.2. The material on the Java Memory Model is the best explanation available of
what `volatile` and `final` actually guarantee. Predates virtual threads, which
changes its advice on thread pools and nothing else.

**Leslie Lamport, "Time, Clocks, and the Ordering of Events in a Distributed
System" (1978).** Eight pages, and it will change how you think about time. The
argument that there is no global "now" and that causality is the only real
ordering is not difficult, and it explains why distributed systems are hard in a
way no list of failure modes does.

**Rob Pike, "Concurrency is not Parallelism" (2012).** A twenty-minute talk, and
the clearest available statement of Section 31.1.1's distinction. Watch it before
you next argue with someone about the two words.

**Martin Kleppmann, *Designing Data-Intensive Applications*.** Recommended again
from Chapter 29, and chapters 8 and 9 are this chapter's second half at full
length: unreliable networks, unreliable clocks, and what consistency actually
means. The treatment of CAP is the corrective to every overstatement of it you
will meet.

**Doug Lea, "The java.util.concurrent Synchronizer Framework" (2004).** How the
locks and queues in the standard library are actually built, by the person who
built them. Read it when you are tempted to write your own synchronization, and
you will not be.

**Beej's Guide to Network Programming.** Free online, written for C and sockets,
and the best introduction to what is happening below `java.net`. The sections on
what `accept` and `select` do, and on byte ordering over the wire, will make the
Java API stop feeling arbitrary.

**JEP 444 (Virtual Threads) and Ron Pressler's talks on Project Loom.** The
argument for why Java chose virtual threads over async/await, made by the people
who made it. Worth reading for the case that a language feature was avoided
deliberately rather than by neglect.

**Kyle Kingsbury's Jepsen reports.** Empirical testing of distributed databases
against their own consistency claims, and a great many of them fail. Entertaining,
rigorous, and the most persuasive available argument that this material is
genuinely hard for people who do it professionally.
