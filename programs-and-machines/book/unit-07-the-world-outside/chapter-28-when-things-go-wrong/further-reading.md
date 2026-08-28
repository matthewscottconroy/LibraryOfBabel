# Further Reading

**Joshua Bloch, *Effective Java*, third edition.** Items 69 through 77 are the
best short treatment of this chapter's material: use exceptions only for
exceptional conditions, favor unchecked exceptions where the caller cannot
recover, include failure-capture information, and — Item 77 — do not ignore
exceptions. Item 72 on standard exceptions is the reference for which type to
throw.

**John Goodenough, "Exception Handling: Issues and a Proposed Notation" (1975).**
The founding paper, and readable. The discussion of resumption against termination
is worth seeing in its original form, because the argument was live then and is
settled now in a way that hides why it was hard.

**Joe Armstrong, "Making Reliable Distributed Systems in the Presence of Software
Errors" (2003).** His doctoral thesis, and the argument for let-it-crash in full.
It is long, well written, and it will make you uncomfortable about defensive
programming in a productive way.

**Jim Gray, "Why Do Computers Stop and What Can Be Done About It?" (1985).** Short,
empirical, and still accurate. The observation that most production failures are
transient and that restarting beats diagnosing is the practical foundation of
modern operations.

**Anders Hejlsberg, "The Trouble with Checked Exceptions" (2003).** An interview
in which C#'s designer explains why C# has no checked exceptions. Two pages, and
the versioning argument in it is the one Section 28.1.3 compresses into a
paragraph.

**Rob Pike, "Errors are values" (2015).** The Go position, argued by one of its
designers. Useful specifically because it is a considered rejection of this
chapter's mechanism rather than an unfamiliarity with it.

**Michael Nygard, *Release It!*, second edition.** What failure handling looks
like at the scale where things actually break: timeouts, circuit breakers,
bulkheads, and the failure modes that only appear under load. The most practically
useful book on this list if you are heading toward systems that run continuously.
