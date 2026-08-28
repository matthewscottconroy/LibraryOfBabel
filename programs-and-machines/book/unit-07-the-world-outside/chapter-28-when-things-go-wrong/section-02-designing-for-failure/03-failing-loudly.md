# Failing Loudly

```java
try { ... } catch (Exception e) { }
```

That is the worst line in this chapter and it is in almost every large codebase.
It compiles, it silences a problem, and it is the reason somebody will spend a day
next month looking for a wrong number whose cause was destroyed at the moment it
was available.

A program that has detected an impossible situation has two options: stop, or
continue in a state it does not understand.

This lesson argues for stopping, and then qualifies it.

## The case

```java
try { ... } catch (Exception e) { }
```

The empty catch. It is the worst line in this chapter, and it appears in almost
every large codebase.

What it produces: the failure happened, nothing recorded it, and the program
carried on with a variable unassigned or a file unwritten. The symptom appears
minutes or days later, somewhere else, as a wrong number or a missing record. The
stack trace that would have named the cause was constructed and discarded.

Debugging that is genuinely hard, because the evidence was deliberately destroyed
at the moment it existed.

Compare with the alternative: the program stops, prints a trace naming the file
and line, and someone fixes it in ten minutes.

**Failing immediately is cheaper than failing eventually**, and the gap grows with
the distance between the cause and the symptom.

## Fail fast

The general form of the principle: **check assumptions at the earliest point where
they can be checked, and stop if they do not hold.**

```java
Account(String owner, long cents) {
    if (owner == null)  throw new IllegalArgumentException("owner is required");
    if (cents < 0)      throw new IllegalArgumentException("negative balance: " + cents);
    ...
}
```

Validating in the constructor means an invalid `Account` never exists. Every
method afterwards can rely on it, and the failure — if it comes — arrives with the
caller who supplied the bad value still on the stack, which is exactly the
information needed.

This is Chapter 19's invariant with an enforcement point, and Chapter 22's compact
constructor is the same move for records.

The contrast is validating late, or not at all. A `null` owner accepted in the
constructor produces a `NullPointerException` in some unrelated method later, and
the trace names a method that did nothing wrong.

## What loud means

Stopping is not always available — a server should not exit because one request
was malformed. So "loudly" is the more general requirement, and it has three
parts.

**Something durable records it.** A log entry with the stack trace and enough
context to identify the case: which request, which record, which file.

**Someone can find out.** A log nobody reads is only marginally better than no
log. Real systems alert on error rates.

**The failure is not converted into a plausible wrong answer.** This is the
important one. Returning 0 for a balance that could not be read is worse than
failing, because 0 is a number that will be believed, printed, and possibly
transferred.

## The cost of throwing

An argument sometimes made against exceptions is that they are slow. Measured
here, one million throw-and-catch cycles:

```
throw with trace      355 ms
return a sentinel       0 ms
throw without trace     0 ms
```

Two conclusions.

**Throwing is expensive** — about 355 nanoseconds each, roughly a thousand times a
returned sentinel.

**Almost all of it is the stack trace.** The third row throws a pre-allocated
exception whose `fillInStackTrace` is overridden to do nothing, and it costs
essentially the same as returning. So the expense is walking the live frames and
recording them, not the throw or the catch.

What follows:

**Do not use exceptions for control flow.** A million exceptions in a loop is a
third of a second of pure overhead. Chapter 18's rule applies, but here the
magnitude is large enough to matter without measuring.

**Do not avoid exceptions for exceptional cases.** If a failure happens once per
request, 355 nanoseconds is nothing next to the microseconds of everything else,
and the diagnostic value of the trace is worth far more.

**The stack-trace-suppression trick exists** and is occasionally right — some
high-throughput libraries use a pre-allocated exception for a hot control-flow
path — and it should make you uneasy, because an exception without a trace tells
you nothing about where it came from.

## Assertions

Java has a construct for checks that should never fail:

```java
assert cents >= 0 : "balance went negative: " + cents;
```

It throws `AssertionError` if the condition is false, and it is **disabled by
default** — the JVM ignores assertions unless run with `-ea`.

That default is the whole story of how to use them.

**Assertions are for internal invariants**, checked during development and
testing, where being wrong means a bug in your own code.

**They are not for validating input.** A check that must run in production must be
an `if` and a `throw`, because an assertion in production is a check that is not
happening.

The distinction: validate what comes from outside with real code; assert what you
believe about your own logic.

There is one honest criticism, which is that a check disabled in production is a
check whose failure you learn about from a customer. Some teams therefore enable
assertions everywhere and treat them as ordinary checks; others avoid them
entirely in favour of explicit `if`s. Both positions are defensible; the
indefensible one is writing assertions and assuming they run.

## The habit worth forming

The through-line of this chapter, in three sentences.

**Make invalid states unrepresentable where you can** — with types, immutability,
and constructors that validate. This is Chapters 19, 20 and 22, and it means many
failures cannot occur.

**Detect the rest as early as possible**, at the boundary where bad data enters,
with the supplier still on the stack.

**When something you believed turns out to be false, say so loudly** — and never
convert it into a value that looks like an answer.

Chapter 29 takes this to the world of files, where the failures are not
hypothetical: the file will be missing, the encoding will be wrong, and the disk
will fill.
