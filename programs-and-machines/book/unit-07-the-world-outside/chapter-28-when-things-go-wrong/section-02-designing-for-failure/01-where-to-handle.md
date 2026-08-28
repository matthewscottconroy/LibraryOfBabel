# Where to Handle

Catching an exception feels responsible. Letting one travel straight through your
method feels like getting away with something.

It is the other way round, and this lesson is about why.

Here is the claim, and it is worth stating baldly before we soften it: **a `catch`
block that cannot improve the situation has not handled anything.** It has removed
information that somebody further up the call stack was going to need, and it has
done so while looking diligent.

The mechanism was the easy part. This is the part that determines whether anybody
enjoys operating your program at three in the morning.

## One test, and it fits on a line

**Catch an exception where you can do something meaningful about it.**

"Meaningful" is not a matter of taste. It means one of these:

- retry, possibly after waiting
- fall back to something else — a cached value, a default, a second server
- tell a user something they can act on
- record it and carry on with the remaining work
- add context and rethrow

Look at where you are in the program and ask which of those five you are in a
position to do. If the answer is none of them, do not catch. Let it go past.

## Three wrong places, and you have probably written all three

**Too low.** A parsing method that catches its own failure and returns a default
has destroyed something. The caller can no longer distinguish "the field contained
0" from "the field contained garbage" — and those call for different responses.
The parser cannot know which one the caller wanted, because only the caller knows.

**Too broad.** Read this one and count the distinct failures it is pretending are
the same failure:

```java
try {
    loadConfig();
    connectToDatabase();
    startServer();
} catch (Exception e) {
    System.out.println("startup failed");
}
```

Three operations, three completely different things that could go wrong, three
different sensible responses — flattened into one message that tells the operator
nothing at all.

And it is worse than it looks. That `catch (Exception e)` also caught the
`NullPointerException` from the bug in `loadConfig`, announced it as "startup
failed", and threw away the stack trace that would have located it.

**Everywhere.** A `try/catch` wrapped around every call gives you code in which the
error handling outweighs the logic and no reader can find the main path through it.
It also guarantees the too-low problem, at every single level.

## What the right shape looks like

Most programs want their error handling gathered at a small number of
**boundaries** — places where a unit of work begins and ends.

**The top of a request.** A web server catches around each incoming request, logs
it, returns a 500. One handler covers every failure that could happen inside, and
the request that failed does not take the whole server with it.

**The top of a task.** A batch job catches around each item, records the failure,
and moves to the next one. One bad record does not cost you the run.

**The user's action.** A desktop application catches around each command and puts a
message on the screen. The event loop of Chapter 30 is precisely this.

**The top of `main`.** Last resort, so that something unexpected produces a useful
log entry rather than a bare stack trace on somebody's terminal.

In between those boundaries, most of your code should not mention exceptions at
all. Which is the propagation argument from Section 28.1.1 finally paying for
itself: the methods with nothing useful to say get to say nothing.

## The one exception to the rule

Take this one, because it is legitimate and badly undervalued:

```java
try {
    return parseRecord(line);
} catch (RuntimeException e) {
    throw new ParseException("line " + lineNumber + " of " + fileName, e);
}
```

That handler fixes nothing. It cannot. What it does is attach information that
exists *only here* — the line number, the file name — and send the failure onward
with the original cause still attached.

Consider the difference from the other end. An operator reading

`NumberFormatException: For input string: "N/A"`

against an operator reading

`line 4,127 of customers.csv, caused by NumberFormatException: For input string: "N/A"`

is the difference between an hour of work and a minute of it. Same bug. Same
exception. One of them tells you where to look.

So: **catch to add context, or catch to act. Never catch to be seen catching.**

## Recovery is rarer than you would like

It is worth being honest about how few exceptions can actually be recovered from
where they happen.

**Genuinely recoverable**: a transient network failure, worth a retry with a
backoff. A missing *optional* config file, worth a default. A malformed record in
the middle of a batch, worth skipping and recording.

**Not recoverable**: a missing *required* config file. A `NullPointerException`. A
database schema that will not parse. Anything at all that means your program's
assumptions about the world are wrong.

For that second group the correct response is to stop, which is what the next
lesson argues in full. Carrying on means executing code whose preconditions you
already know to be false, and what happens after that is unpredictable and
generally worse than the stopping would have been.

## Retries, since you are going to write one

```java
for (int attempt = 1; attempt <= 3; attempt++) {
    try { return fetch(url); }
    catch (IOException e) {
        if (attempt == 3) throw new UncheckedIOException("gave up after 3", e);
        sleep(100 * (1L << attempt));      // 200ms, 400ms
    }
}
```

Three things in there are right, and naive retry loops get all three wrong.

**There is a bound.** Retrying forever turns a failure into a hang, and a hang is
considerably harder to diagnose than a crash — nothing is reported, nothing is
logged, and the program sits there looking busy.

**There is a backoff.** Retrying instantly hammers a service that is already in
trouble. When many clients do it at once, the thing that would have been a
five-second blip becomes an outage, and the clients caused it.

**The final failure is rethrown, with its cause.** Giving up quietly after three
attempts is the swallowing problem again, wearing a loop.

And one question to settle before you write any retry at all: **is this operation
safe to repeat?** Retrying a read is fine. Retrying a payment may charge somebody
twice. If the operation is not idempotent then a retry needs a request identifier
the server can recognize as a duplicate — which is a design decision, not a loop.

Next: making sure things get closed.
