# Resources and Cleanup

Some things must be released. An open file holds an operating-system handle, a
socket holds a connection, a database connection holds a server-side session, and
a lock — Chapter 31 — holds up every thread waiting for it.

Java's garbage collector does not help. It reclaims **memory**, eventually, and
knows nothing about handles. An object with an open file inside it may sit
uncollected for minutes, and the file stays open the whole time.

So releasing is your job, and the difficulty is that failure can happen between
acquiring and releasing.

## The wrong way

```java
BufferedReader r = new BufferedReader(new FileReader(path));
String line = r.readLine();
process(line);
r.close();
```

If `readLine` throws, or `process` throws, `close` never runs. The handle leaks.

A program that leaks handles is fine in testing and fails in production after a
few hours, with `Too many open files` — an error that names the symptom and not
the cause, in a part of the program that did nothing wrong.

## The old right way

```java
BufferedReader r = null;
try {
    r = new BufferedReader(new FileReader(path));
    process(r.readLine());
} finally {
    if (r != null) {
        try { r.close(); }
        catch (IOException e) { /* now what? */ }
    }
}
```

This is correct, and look at it. A null initialization outside the block, a null
check, and a nested try because `close` itself can throw — and the inner catch has
no good answer, because a failure while closing after a failure while reading
leaves you with two exceptions and one stack to travel up.

Worse, the naive version of that inner block *replaces* the original exception
with the close failure, so the interesting error is lost and you are told the file
would not close.

Every Java program written before 2011 contains this, and a meaningful fraction
got it wrong.

## try-with-resources

```java
try (BufferedReader r = new BufferedReader(new FileReader(path))) {
    process(r.readLine());
}
```

`close` is called automatically when the block exits, however it exits. Verified
in Section 28.1.2, where the resource was closed before the handler ran:

```
open bob
close bob
caught: short by 989 cents
```

The `close` happened on the way out of the block, before control reached the
`catch`.

Two details that the manual version got wrong and this gets right.

**The original exception wins.** If the body throws and `close` also throws, the
body's exception propagates and the close failure is attached as a **suppressed**
exception, retrievable with `getSuppressed()` and printed by the default handler.
Nothing is lost and the important one is on top.

**Multiple resources close in reverse order**, and each is closed even if an
earlier one's close failed:

```java
try (var in = Files.newInputStream(src);
     var out = Files.newOutputStream(dst)) {
    in.transferTo(out);
}
```

`out` closes first, then `in`. Reverse of acquisition, which is the correct order
whenever the later resource might depend on the earlier.

## AutoCloseable

Any class can participate:

```java
static class Account implements AutoCloseable {
    public void close() { System.out.println("close " + name); }
}
```

That is all it takes — one interface, one method. The demonstration in Section
28.1.2 used exactly this.

If you write a class that holds something needing release, implement
`AutoCloseable` and let callers use the block form. It is a small courtesy and it
removes an entire category of mistake from every use site.

Two conventions worth following:

**`close` should be idempotent.** Calling it twice should not fail. Callers
sometimes close explicitly and then leave the block.

**`close` should not throw if it can avoid it.** A close that throws puts the
caller in the awkward position described above, and there is usually nothing they
can do.

## What about finalizers

Java has two mechanisms that look like automatic cleanup, and both are traps
worth knowing about so you can recognize them in old code.

`finalize()` is deprecated for removal. It runs at an unpredictable time, or
never, it can resurrect objects, it delays collection, and its failures are
swallowed. It was a mistake and the JDK says so.

`Cleaner`, its replacement, is better and is still a *safety net* rather than a
mechanism. It runs after an object becomes unreachable, which may be long after
the resource should have been freed.

The rule the JDK's own documentation gives: **use try-with-resources; use a
`Cleaner` only as a backstop for callers who forget.** Nothing that must happen
promptly should depend on collection.

## The general principle

There is a shape here that reaches past resources.

**Acquisition and release should be visible in the same place**, so that reading
one you can see the other. `try (...) { }` does that syntactically. A field
holding a resource acquired in one method and released in another does not, and
such a class is where leaks live.

The stronger version, and Chapter 31 will need it: **prefer a scope to a
lifetime.** If a resource can be acquired, used, and released inside one block,
do that, even if it means opening the file twice. A resource whose lifetime spans
methods is a resource whose release depends on control flow you have to reason
about.

Next: what to do when something impossible happens.
