# Checked and Unchecked

Here is a feature that exists in Java and in no mainstream language designed since.

That is unusual enough to be worth understanding rather than obeying. The idea
behind it was good, the arguments for it are still sound as far as they go, and it
lost — and the way it lost is more instructive than either the feature or its
replacement.

Java divides its exceptions into two kinds.

**Checked** exceptions — anything extending `Exception` but not `RuntimeException`
— must be declared or handled. The compiler enforces it:

```java
static String read(Path p) throws IOException {     // declared
    return Files.readString(p);
}

try { Files.readString(Path.of("/no/such/file")); }
catch (IOException e) { ... }                        // or handled
```

Verified:

```
IOException: /no/such/file
```

**Unchecked** exceptions — `RuntimeException` and its subclasses, plus `Error` —
need neither. `IllegalArgumentException` can be thrown from anywhere and no
signature mentions it.

## The idea

Before the criticism, the case for it — put as well as I can put it, because it is
a better argument than its reputation suggests.

An `IOException` is not a bug. Files genuinely go missing, disks genuinely fill,
networks genuinely fail, and a caller who has not thought about it has an
incomplete program. Putting the failure in the signature makes it part of the
contract — the compiler will not let a caller forget that reading a file might not
work.

A `NullPointerException`, by contrast, *is* a bug. Requiring every method to
declare that it might dereference null would be absurd, since every method might.

So the intended rule: **checked for recoverable conditions the caller should plan
for; unchecked for programming errors.**

## Why it did not work

Three problems, and the third is fatal.

**Callers cannot always do anything useful.** A method four levels deep from the
user interface catches an `IOException` and has no idea what the appropriate
response is. The correct handler is far away, so the exception has to be declared
at every level in between — which is the propagation problem from Section 28.1.1,
returned in a new form.

**The declaration is part of the interface.** A method declaring `throws
IOException` has committed to it. If a subclass or an implementation of an
interface wants to throw something else, it cannot — and interfaces designed
before an implementation's needs were known constrain it permanently. `Runnable.
run()` declares nothing, so no `Runnable` may throw a checked exception, which is
why every lambda that does any I/O needs a wrapper.

**The pressure is toward the wrong behavior.** This is the one that killed it.

Put yourself in front of a checked exception you cannot do anything about, at five
in the afternoon. You have three options: declare it and push the problem up to
somebody else, wrap it in a `RuntimeException`, or the one people actually take —

```java
try { ... } catch (IOException e) { }
```

Catch and ignore. Verified, in the sense that this compiles and prints nothing:

```
(that catch block printed nothing and lost everything)
```

The failure has now been converted into a wrong answer with no record anywhere
that anything went wrong. That is worse than every other option on the list — worse
than crashing, worse than propagating, worse than doing nothing at all.

And notice where the incentive came from. The language feature designed to make you
handle failures carefully is the thing that produced the empty catch block.

## The verdict

The consensus, arrived at over twenty years and effectively unanimous now:

**No language designed after Java has adopted checked exceptions.** C#, Kotlin,
Scala, Swift, Go, Rust all declined, and several of their designers have written
about specifically choosing not to.

**Java's own libraries have moved away.** The newer APIs use unchecked exceptions
where the older ones used checked ones. `java.time` throws
`DateTimeParseException`, unchecked, where `SimpleDateFormat` threw a checked
`ParseException`. Streams and lambdas cannot carry checked exceptions at all,
which effectively settled the question for new code.

**Most Java codebases wrap.** The dominant practice is to catch a checked
exception near where it arises, wrap it in an unchecked one with the cause, and
handle it much further up.

The honest summary: the diagnosis was right — failure *is* part of a method's
contract and *should* be visible — and the remedy was too rigid, because it made
"I cannot handle this here" impossible to express except by lying.

## What to do

Practical guidance for code you write today.

**Prefer unchecked exceptions for your own types.** `RuntimeException` for
anything a caller is unlikely to handle locally. This is what most current Java
does.

**Use checked exceptions when the caller genuinely has a decision to make**, and
that decision is at the immediate call site — a parse that might reasonably fall
back to a default, a resource that might reasonably be retried. These are rarer
than the library suggests.

**When you must catch a checked exception you cannot handle, wrap and rethrow with
the cause:**

```java
catch (IOException e) {
    throw new UncheckedIOException(e);
}
```

`UncheckedIOException` exists in the JDK for exactly this, which is itself a
comment on the feature.

**Never catch and ignore.** If there is genuinely nothing you can do, log it and
write a comment saying why. An empty catch block is the single most reliable
indicator in all of Java of a bug that has not happened yet.

**Do not declare `throws Exception`.** It tells callers nothing and forces them to
catch everything. Declare the specific types or none.

## Reading old code

You will meet a great deal of code written under the original rules, and it is
worth being able to read it charitably. Long `throws` clauses, defensive catch
blocks, and wrapper exception hierarchies were the reasonable response to a
language feature that was taken seriously. They are not incompetence; they are
what the design asked for.

Next: the question of where a failure should actually be dealt with.
