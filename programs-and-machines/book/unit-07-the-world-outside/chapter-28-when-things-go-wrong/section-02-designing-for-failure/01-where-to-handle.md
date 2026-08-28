# Where to Handle

Catching an exception feels responsible. Letting one pass through your method feels
careless.

It is the other way round, and this lesson is about why. A `catch` block that
cannot improve the situation has not handled anything — it has only removed
information that somebody further up was going to need.

The mechanism is easy. The question that decides whether a program is pleasant to
operate is *where* the `catch` goes, and the usual answer is: further up than you
first want to put it.

## The test

**Catch an exception where you can do something meaningful about it.**

Meaningful means one of a short list:

- retry, possibly after a wait
- fall back to an alternative — a cached value, a default, a second server
- report it to a user in terms they can act on
- record it and continue with the remaining work
- add context and rethrow

If none of those applies at this point in the program, do not catch. Let it pass.

That rule is uncomfortable at first, because catching feels responsible and
letting an exception through feels careless. It is the other way round: a `catch`
that does not improve the situation has only hidden information.

## The three wrong places

**Too low.** A parsing method that catches its own failure and returns a default
has destroyed the caller's ability to distinguish "the field was 0" from "the
field was garbage". The parser does not know which the caller wants; only the
caller does.

**Too broad.**

```java
try {
    loadConfig();
    connectToDatabase();
    startServer();
} catch (Exception e) {
    System.out.println("startup failed");
}
```

Three different failures with three different responses, collapsed into one
message that says nothing. And `catch (Exception e)` also caught the
`NullPointerException` from the bug in `loadConfig`, reported it as "startup
failed", and lost the stack trace.

**Everywhere.** A `try/catch` around every call produces code where the error
handling outweighs the logic and no reader can find the main path. It also
guarantees the too-low problem at every level.

## The right shape

Most programs want error handling concentrated at a small number of **boundaries**:

**The top of a request.** A web server catches around each request, logs, and
returns a 500. One handler covers every failure inside, and the request that
failed does not take the server down.

**The top of a task.** A batch job catches around each item, records the failure,
and continues with the rest. One bad record does not lose the run.

**The user's action.** A desktop application catches around each command and shows
a message. Chapter 30's event loop is exactly this.

**The top of `main`.** The last resort, so that an unexpected failure produces a
useful log rather than a bare trace.

Between those boundaries, most code should not mention exceptions at all. That is
the propagation argument from Section 28.1.1 paying off: the methods with nothing
to say say nothing.

## Adding context on the way through

The exception to the rule, and it is worth taking:

```java
try {
    return parseRecord(line);
} catch (RuntimeException e) {
    throw new ParseException("line " + lineNumber + " of " + fileName, e);
}
```

This handler does not fix anything. It adds information that only exists here —
the line number and the file name — and passes the failure on with the cause
attached.

That is legitimate and undervalued. The difference between an operator reading
`NumberFormatException: For input string: "N/A"` and reading `line 4,127 of
customers.csv, caused by NumberFormatException: For input string: "N/A"` is the
difference between an hour and a minute.

The rule: **catch to add context, or catch to act. Never catch to be seen
catching.**

## Recovery is rarer than it looks

Most exceptions cannot be recovered from in any local sense, and it helps to be
realistic about which can.

**Genuinely recoverable**: a transient network failure, worth retrying with a
backoff. A missing optional config file, worth defaulting. A malformed record in
a batch, worth skipping and recording.

**Not recoverable**: a missing required config file. A `NullPointerException`. An
unparseable database schema. Anything indicating the program's assumptions are
wrong.

For the second group, the right response is to stop, which is Section 28.2.3's
argument. Attempting to continue means executing code whose preconditions you know
to be false, and what happens next is unpredictable and usually worse than
stopping.

## Retries, since everyone writes one

```java
for (int attempt = 1; attempt <= 3; attempt++) {
    try { return fetch(url); }
    catch (IOException e) {
        if (attempt == 3) throw new UncheckedIOException("gave up after 3", e);
        sleep(100 * (1L << attempt));      // 200ms, 400ms
    }
}
```

Three things this gets right and that naive retry loops get wrong.

**A bound.** Retrying forever converts a failure into a hang, which is harder to
diagnose.

**A backoff.** Immediate retries hammer a service that is already struggling, and
if many clients do it the effect is an outage that would otherwise have been a
blip.

**The last failure is rethrown with a cause.** Giving up silently after three
attempts is the swallowing problem with extra steps.

And one thing to check before writing any retry: **is the operation safe to repeat?**
Retrying a read is fine. Retrying a payment may charge twice. If the operation is
not idempotent, a retry needs a request identifier the server can recognize, and
that is a design decision rather than a loop.

Next: making sure things get closed.
