# Resources and Cleanup

Java has a garbage collector, which is why you have gone this far without ever
freeing anything. It has been quietly cleaning up behind you since your first
program.

It is about to stop being enough, and it is worth being precise about why.

The collector reclaims **memory**, on its own schedule, when it feels like it. That
is the whole of its job description. It knows nothing whatsoever about the
operating-system handle sitting inside the object it is about to reclaim, and it
has no opinion about when that handle ought to be released.

So some things are your responsibility. An open file holds an OS handle. A socket
holds a connection. A database connection holds a session on a server somewhere
else. And a lock — which Chapter 31 will make vivid — holds up every single thread
waiting to acquire it.

An object with an open file inside it can sit uncollected for minutes, and the file
stays open for every one of them.

Here is what that looks like when it goes wrong, because the failure mode is
distinctive. The program passes every test. It runs fine for hours. Then production
falls over with `Too many open files` — an error that names the symptom rather than
the cause, raised in a part of the program that did nothing wrong, at a moment
unrelated to the mistake.

Releasing things is your job. And the difficulty, as ever, is that failure can
strike between acquiring and releasing.

## The wrong way, which looks fine

```java
BufferedReader r = new BufferedReader(new FileReader(path));
String line = r.readLine();
process(line);
r.close();
```

Find the leak before reading on. The `close` is right there at the end.

It is right there at the end, and it runs only if nothing above it throws. If
`readLine` fails, or `process` fails, control leaves by another door and `close`
never executes. The handle is gone.

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

That is correct. Look at what correct costs: a null initialization outside the
block, a null check inside the handler, and a nested `try` because `close` can
itself throw.

And look at that inner comment, because it is not a joke. There genuinely is no
good answer. A failure while closing, *after* a failure while reading, leaves you
holding two exceptions and one stack to send them up.

The naive version of that inner block does the worst possible thing: it lets the
close failure **replace** the original exception. So the interesting error — the
one that explains what actually went wrong — vanishes, and you are informed that
the file would not close.

Every Java program written before 2011 contains some version of this, and a
meaningful fraction of them got it wrong.

## try-with-resources

```java
try (BufferedReader r = new BufferedReader(new FileReader(path))) {
    process(r.readLine());
}
```

`close` is called when the block exits, no matter how it exits. It was verified in
Section 28.1.2, where the resource closed before the handler ran:

```
open bob
close bob
caught: short by 989 cents
```

Read that ordering carefully. The `close` happened on the way out of the block,
*before* control ever reached the `catch`.

Two things the manual version got wrong, which this gets right without being asked.

**The original exception wins.** If the body throws and `close` also throws, the
body's exception is the one that propagates, and the close failure is attached to
it as a **suppressed** exception — retrievable with `getSuppressed()`, and printed
by the default handler. Nothing is lost, and the one you care about is on top.

**Multiple resources close in reverse order**, and each one closes even if an
earlier close already failed:

```java
try (var in = Files.newInputStream(src);
     var out = Files.newOutputStream(dst)) {
    in.transferTo(out);
}
```

`out` closes first, then `in` — the reverse of the order they were acquired in,
which is the only correct order whenever the later resource might depend on the
earlier one.

## Letting your own classes join in

Any class can participate, and the barrier is lower than you would guess:

```java
static class Account implements AutoCloseable {
    public void close() { System.out.println("close " + name); }
}
```

One interface, one method. That is the entire cost of admission, and the
demonstration in Section 28.1.2 used exactly this.

So if you write a class holding something that needs releasing, implement
`AutoCloseable` and let your callers use the block form. It is a small courtesy
that removes a whole category of mistake from every place your class is used —
which is a very good exchange rate.

Two conventions worth honoring:

**`close` should be idempotent.** Calling it twice must not fail, because callers
sometimes close explicitly and then leave the block anyway.

**`close` should avoid throwing if it possibly can.** A close that throws puts your
caller in the awkward position described above, and there is usually nothing useful
they can do about it.

## The two things that look like automatic cleanup and are not

You should know both of these well enough to recognize them in somebody's code from
ten years ago.

`finalize()` is deprecated for removal, and deserves it. It runs at an
unpredictable time, or never. It can resurrect objects. It delays collection. Its
failures are swallowed silently. It was a mistake, and the JDK now says so in
writing.

`Cleaner`, its replacement, is genuinely better and is still a *safety net* rather
than a mechanism. It runs after an object becomes unreachable, which may be a very
long time after the resource ought to have been freed.

The rule the JDK's own documentation gives: **use try-with-resources, and use a
`Cleaner` only as a backstop for callers who forget.** Nothing that has to happen
promptly should ever be made to depend on garbage collection.

## The principle underneath

There is a shape here that reaches well past files and sockets.

**Acquisition and release belong in the same place**, close enough together that
reading one puts the other in front of you. `try (...) { }` enforces that
syntactically — you cannot write the acquisition without the scope that releases
it.

A field holding a resource acquired in one method and released in another does the
opposite, and classes shaped like that are where leaks live.

And here is the stronger version, which Chapter 31 will need badly: **prefer a
scope to a lifetime.** If a resource can be acquired, used, and released inside one
block, do that — even at the cost of opening the file twice. A resource whose
lifetime spans several methods is a resource whose release depends on control flow,
and control flow is a thing you have to reason about rather than see.

Next: what to do when something impossible happens.
