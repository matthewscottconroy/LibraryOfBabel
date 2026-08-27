# Reading an Error

Java tells you a great deal when a program fails. Most beginners read the first
line, feel bad, and start changing things. The information is right there.

## A real one

```java
public class Boom {
    public static void main(String[] args) {
        int[] a = {1, 2, 3};
        System.out.println("about to fail");
        System.out.println(sumTo(a, 3));
    }
    static int sumTo(int[] a, int n) {
        int total = 0;
        for (int i = 0; i <= n; i++) total += a[i];
        return total;
    }
}
```

Running it:

```
about to fail
Exception in thread "main" java.lang.ArrayIndexOutOfBoundsException: Index 3 out of bounds for length 3
	at Boom.sumTo(Boom.java:9)
	at Boom.main(Boom.java:5)
```

Four pieces of information. Take them in turn.

**`java.lang.ArrayIndexOutOfBoundsException`** — the *kind* of failure. An index
outside an array's bounds. That already tells you the defect is arithmetic about
a range, which is a small part of the program.

**`Index 3 out of bounds for length 3`** — the *specifics*. The index was 3 and
the length was 3. Valid indices are 0 through 2, so 3 is one too many. Chapter
9's off-by-one, named and quantified.

**`at Boom.sumTo(Boom.java:9)`** — *where*. Line 9, in `sumTo`. That is
`for (int i = 0; i <= n; i++) total += a[i];` — and now the `<=` is visible as the
culprit.

**`at Boom.main(Boom.java:5)`** — *how execution got there*. `main`, at line 5,
called `sumTo`.

The bug is `<=` where `<` was meant. The message contained the exception type, the
two numbers that did not fit, the line, and the call path. Read properly, it is
close to a diagnosis.

## The stack trace

Those `at` lines are the **call stack** — the chain of calls in progress when the
failure happened. Chapter 12 builds this properly; for now, the reading rule:

**Top line is where it broke. Lines below are how it got there. Read top-down.**

In a real program the trace may be forty lines, most of it library and framework
code. The useful trick: **find the topmost line that names a file you wrote.**
Frames above it are usually a library correctly rejecting something you handed it,
and the interesting question is what you handed it, which is in your frame.

## The other common one

```
Exception in thread "main" java.lang.NullPointerException: Cannot invoke "String.length()" because "<local1>" is null
	at Npe.main(Npe.java:4)
```

Since Java 14, null pointer messages are unusually good — this one names the
method being called and says which value was null. Before that you got only the
line number, and if the line contained three method calls you were guessing.

The phrasing repays attention: *cannot invoke `String.length()` because the value
is null*. The failure is not "something was null somewhere". It is "you asked a
null for its length". Chapter 16 discusses null itself and why it exists.

## Compile-time versus run-time, again

Chapter 5 made the distinction; here is why it pays.

A **compile error** comes from `javac`, before anything runs, and names a file and
line in your source. Nothing executed.

A **run-time error** comes from the JVM, after the program started. The output
before it — `about to fail`, in our example — tells you the program got that far,
which is itself evidence.

Knowing which you have tells you what kind of mistake to look for. Compile errors
are about what you *wrote*: a name that does not exist, a type that does not fit,
a brace that does not close. Run-time errors are about what the program *met*: a
value out of range, a null, a file that was not there.

## The habit

When something fails, before touching the code:

1. Read the whole message, including the exception type.
2. Note any values it reports. They are usually the crux.
3. Find the topmost frame in your own code, and go to that line.
4. Ask what would have to be true for the failure to happen there.

That last question is the pivot from symptom to cause. `Index 3 out of bounds for
length 3` happens if `i` reached 3, which happens if the condition permitted it,
which is the `<=`.

Four steps, and they usually finish the job before any debugging technique is
needed. The reason to be explicit about them is that the natural response to an
error message is to skim it, and the information density is high enough that
skimming loses most of it.

Next: what to do when the message does not tell you enough.
