# Throwing and Catching

The mechanism is two keywords and takes ten minutes. What takes longer is noticing
what an exception actually *is*.

It is not an error message. It is a value — constructed at the point where things
went wrong, carrying whatever the handler will need, and then travelling. Where it
travels is Chapter 12's stack, unwound frame by frame until something is prepared
to deal with it.

```java
void withdraw(long amount) {
    if (amount <= 0)     throw new IllegalArgumentException("amount must be positive: " + amount);
    if (amount > cents)  throw new InsufficientFunds(amount - cents);
    cents -= amount;
}
```

`throw` takes an object and stops the method. Control does not return to the
caller normally; it goes to the nearest enclosing `catch` that matches, however
many frames up that is.

## Catching

```java
try {
    a.withdraw(amt);
    System.out.println("withdrew " + amt);
} catch (InsufficientFunds e) {
    System.out.println("refused " + amt + ": " + e.getMessage()
        + " (shortfall " + e.shortfall + ")");
} catch (IllegalArgumentException e) {
    System.out.println("rejected " + amt + ": " + e.getMessage());
}
```

Verified, withdrawing 300, then 5000, then $-5$ from a balance of 1000:

```
withdrew 300
refused 5000: short by 4300 cents (shortfall 4300)
rejected -5: amount must be positive: -5
```

Several things in that output.

**The clauses are tried in order, and the first matching one wins.** So the most
specific must come first — a `catch (Exception e)` before `catch
(InsufficientFunds e)` would not compile, because the second could never run, and
the compiler says so.

**The `try` block stopped at the throw.** The `println` after `withdraw` did not
run for the failing cases. Everything between the throw and the `catch` is
abandoned.

**The exception carried data.** `e.shortfall` is 4300 — a field on a custom
exception type, holding information the handler needed. That is the difference
between an exception and an error message.

## Writing your own

```java
static class InsufficientFunds extends RuntimeException {
    final long shortfall;

    InsufficientFunds(long shortfall) {
        super("short by " + shortfall + " cents");
        this.shortfall = shortfall;
    }
}
```

An ordinary class. It extends `RuntimeException`, it calls `super` with a message,
and it adds a field.

Two pieces of guidance.

**Add the data a handler would want.** A handler that must parse the message
string to learn the shortfall is a handler you have failed. The message is for
humans; fields are for code.

**Use the standard types when they fit.** `IllegalArgumentException`,
`IllegalStateException`, `UnsupportedOperationException`,
`NullPointerException` and `IndexOutOfBoundsException` cover most cases, they are
familiar, and a reader knows what they mean. Invent a type when a caller might
plausibly want to catch *this specific thing* — as `InsufficientFunds` is, and a
generic `IllegalArgumentException` would not be.

## The hierarchy

```
Throwable
├── Error                  — the JVM is in trouble. Do not catch.
│     StackOverflowError, OutOfMemoryError
└── Exception
      ├── RuntimeException — unchecked
      │     IllegalArgumentException, NullPointerException, ...
      └── everything else  — checked
            IOException, SQLException, ...
```

`Error` is the JVM reporting that something outside your program's control has
gone wrong. Chapter 12's `StackOverflowError` is one. Catching them is almost
always a mistake, because there is generally nothing useful to do and the
machinery you would need may itself fail.

The split inside `Exception` is the next lesson.

## finally

```java
static int tricky() {
    try { return 1; }
    finally { System.out.println("finally ran"); }
}
```

Verified:

```
finally ran
returned 1
```

The `finally` block runs on the way out, whichever way out is taken — normal
return, exception, or `break`. It ran here even though the `try` returned.

This is for cleanup that must happen regardless, which is nearly always a resource
being released.

One warning: a `return` inside `finally` discards whatever the `try` was doing,
including an in-flight exception, which silently loses failures. Never return from
`finally`; most linters flag it, and Java warns.

## try-with-resources

The better form, when the thing to release implements `AutoCloseable`:

```java
try (Account b = new Account("bob", 10)) {
    b.withdraw(999);
} catch (InsufficientFunds e) {
    System.out.println("caught: " + e.getMessage());
}
```

Verified:

```
open bob
close bob
caught: short by 989 cents
```

`close` ran before the handler, automatically, because the exception left the
`try` block. No `finally`, no null check, no possibility of forgetting. Section
28.2.2 covers why this matters more than it looks.

## The stack trace

An exception records where it was created, and how it got there.

Verified, throwing three frames deep:

```
message: something broke down here
  at Fail.level2(Fail.java:33)
  at Fail.level1(Fail.java:32)
  at Fail.demoStack(Fail.java:31)
  at Fail.main(Fail.java:66)
```

Read it **top down**: the throw was in `level2`, at line 33. Read it **downward**
for the story: `main` called `demoStack` called `level1` called `level2`.

That is Chapter 12's call stack, captured. It exists because the exception's
constructor walked the live frames and recorded them, which is why the trace shows
where the exception was *created* rather than where it was thrown, and why the two
differ if you reuse an exception object.

Beginners tend to look at the last line. The most useful line is usually the
topmost one in *your* code — the frames above it are library internals, and the
frames below are how you arrived.

## Chaining

When you catch a low-level failure and rethrow at a higher level, keep the
original:

```java
try { Integer.parseInt(raw); }
catch (NumberFormatException e) {
    throw new IllegalStateException("config is bad", e);
}
```

Verified:

```
config is bad  <- caused by java.lang.NumberFormatException: For input string: "xyz"
```

The second argument becomes the **cause**, retrievable with `getCause()`, and
printed by the default handler as `Caused by:`.

This matters because a rethrown exception without a cause has destroyed the
diagnostic information. "Config is bad" tells an operator nothing; "config is bad,
caused by `NumberFormatException` on `xyz`" tells them where to look.

Always pass the cause. It is one argument, and its absence is the reason for a
great many unproductive afternoons.

Next: the distinction Java made and nobody else copied.
