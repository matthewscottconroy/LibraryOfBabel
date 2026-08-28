# Annotations

You have been writing `@Override` since Chapter 20 on the strength of a promise
that it catches a specific bug. It is time to say what it actually is, and the
answer is smaller than expected: it has no members, it is thrown away before the
class file exists, and it changes nothing about how your program runs.

It is also the same construct that lets a test framework find your tests.

An **annotation** attaches information to a piece of code for another program to
read. It changes nothing by itself. `@Override` does not alter what a method does;
it tells the compiler to check something.

That is the whole idea, and everything else is a consequence of *which* program
reads it and *when*.

## Declaring one

```java
@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.METHOD)
@interface Test {
    String name() default "";
}
```

`@interface` declares an annotation type. Its members look like methods and are
really named parameters, each optionally with a default.

Two annotations on the declaration, and they are the important part.

**`@Target`** says where it may be written — method, field, class, parameter,
local variable. Getting this right turns a misplaced annotation into a compile
error.

**`@Retention`** says how long it survives — and this is the one that will cost
you an afternoon if you get it wrong. There are three answers:

| policy | survives to | example |
|---|---|---|
| `SOURCE` | discarded by the compiler | `@Override`, `@SuppressWarnings` |
| `CLASS` | in the class file, not loaded | rare |
| `RUNTIME` | readable by reflection | `@Test`, `@Column`, `@Deprecated` |

That table is the practical content of this lesson, and the middle row is a trap
with your name on it.

An annotation you intend to read reflectively **must** be `RUNTIME`. Forget it and
you get the most common annotation bug there is: everything compiles cleanly,
nothing is ever found, and there is no error message anywhere to tell you why. Your
framework silently does nothing, and the thing you would need to see is the absence
of one word.

## Reading them

```java
for (Field f : c.getDeclaredFields()) {
    Column col = f.getAnnotation(Column.class);
    if (col != null)
        System.out.printf("%s -> column %s nullable=%s%n",
            f.getName(), col.value(), col.nullable());
}
```

Verified, on a class whose fields carried `@Column("acct_id")` and
`@Column(value = "balance_cents", nullable = false)`:

```
id       -> column acct_id         nullable=true
cents    -> column balance_cents   nullable=false
```

The third field had no annotation and was skipped. That pattern — enumerate,
check for the annotation, act on its members — is essentially all of how an
object-relational mapper works.

One naming detail, which answers a question you may have been quietly carrying.
`value` is special: an annotation with a single member called `value` may be
written `@Column("acct_id")` rather than `@Column(value = "acct_id")`.

That is the entire reason so many annotations you meet have a member with such an
unhelpful name. It was never meant to be read.

## A test runner

Fifteen lines, and it is JUnit's core idea in full:

```java
int pass = 0, fail = 0;
Suite s = new Suite();
for (Method m : Suite.class.getDeclaredMethods()) {
    Test t = m.getAnnotation(Test.class);
    if (t == null) continue;
    m.setAccessible(true);
    try {
        m.invoke(s);
        System.out.println("PASS  " + t.name());
        pass++;
    } catch (InvocationTargetException e) {
        System.out.println("FAIL  " + t.name() + " : " + e.getCause());
        fail++;
    }
}
```

Verified, on a class with two annotated methods and one unannotated:

```
PASS  addition works
FAIL  this one fails : java.lang.AssertionError: expected 5
1 passed, 1 failed
```

The unannotated method was not run, though it would have thrown if it had been.

Now put that beside the hand-rolled test harness from Chapter 14, where every
single test had to be listed by hand in a `main` method — and where the way you
lost a test was by writing it and forgetting to add it to the list.

The annotation deletes the list. A new test is a new annotated method, found
automatically, and nothing else in the program has to change to accommodate it.
That is what this mechanism actually buys, and it is why every testing framework
written since about 2005 works this way.

`e.getCause()` is doing necessary work. `invoke` wraps whatever the method threw in
`InvocationTargetException`; report that and the user learns nothing, report the
cause and they see their own assertion failure.

## @Override, finally

Chapter 20 told you to write it on everything and Chapter 21 gave the reason.
Here is what it is.

```java
@Target(ElementType.METHOD)
@Retention(RetentionPolicy.SOURCE)
public @interface Override { }
```

No members. `SOURCE` retention, so it is gone before the class file exists — no
runtime cost, nothing to read, no reflection involved.

Its entire effect is a rule in the compiler: *if a method marked `@Override` does
not override or implement a supertype method, report an error.*

That is the third kind of annotation reader, after "nobody" and "the running
program": **the compiler itself**. `@SuppressWarnings` and `@FunctionalInterface`
are the same — instructions to the tool that processes the source, discarded
afterwards.

So `@Override` is not a modifier and it does not change dispatch; overriding
happens whether you write it or not. It is an assertion, checked, and it costs one
line to turn Chapter 20's silent `equals(Point)` bug into a compile error.

## Annotation processing

The fourth reader, and the one with the most momentum behind it.

An **annotation processor** runs during compilation, sees the annotated code, and
generates more source. Lombok generates getters. Dagger generates
dependency-injection code. Micronaut and Quarkus generate at build time what
Spring traditionally computed by reflection at startup.

That last shift is the interesting one, and Section 27.1.2 named the pressure
behind it: reflection is slow at startup, defeats ahead-of-time compilation, and
is invisible to dead-code analysis. Moving the work to compile time recovers all
three, and the annotation is doing the same job either way — carrying information
for another program to read.

## Using them well

**Do not invent one where a type would do.** An annotation is untyped extra
information. If you find yourself annotating classes to say what kind of thing
they are, an interface or an enum says it in a way the compiler checks.

**Use `RUNTIME` only when something reads it at run time.** The default is `CLASS`,
which is almost never what you want, and `SOURCE` is right for anything the
compiler or a linter consumes.

**Set `@Target`.** Without it an annotation may be written anywhere, and a
misplaced one fails silently.

**Remember they are declarative.** An annotation cannot execute; it can only be
found. Something else must do the work, and if nothing looks for your annotation
it does nothing at all — which is the failure mode to check first when a framework
appears to ignore you.

## Where this leaves the unit

Unit VI began with the claim that a program is text, text is data, and a program
can read a program. Four chapters later:

Chapter 24 turned text into a tree. Chapter 25 gave the tree meaning by walking
it, and a data structure became a program because an evaluator agreed to treat it
as one. Chapter 26 found the same idea inside Java, where a piece of behavior is a
value. Chapter 27 turned it on Java itself — a program reading its own structure
and acting on what it finds.

That is the same idea four times, and it is worth naming the through-line
explicitly: **the boundary between code and data is a choice, not a fact.** It is
drawn differently by a compiler, an interpreter, a serializer, and a test runner,
and knowing that it can be drawn differently is most of what separates using tools
from building them.

Unit VII leaves the program's inside and asks what happens at the edges — files,
errors, other machines, and the return of state as the central difficulty.
