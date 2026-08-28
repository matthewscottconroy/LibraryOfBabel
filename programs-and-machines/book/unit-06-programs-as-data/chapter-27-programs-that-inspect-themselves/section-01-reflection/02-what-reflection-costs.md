# What Reflection Costs

The last lesson made every guarantee in Chapter 19 look negotiable. A private field
read from outside. A private method called. A `final` field assigned to.

Which ought to provoke an uncomfortable question, and you should let it: if all of
that is available, why is anybody still writing ordinary code?

There are four answers. Only the first is a number, and it is comfortably the least
important of the four.

## Speed

Twenty million method calls — ten million pairs of a mutator and an accessor —
timed directly and through `Method.invoke` after warm-up. Guess the ratio before
you look.

```
direct    1 ms   reflective   92 ms
direct    1 ms   reflective   64 ms
direct    2 ms   reflective   81 ms
```

Somewhere between forty and sixty times slower.

The reason is worth having, because it explains why the gap cannot be closed by a
cleverer JVM. A direct call is the two indirections and a jump from Chapter 21, and
the JIT deletes even that once the call site proves monomorphic. A reflective call
can never get there: the target is a `Method` object that is not known until run
time, the arguments have to be boxed into an `Object[]`, the return value has to be
boxed on the way back, and the access permission has to be re-checked. None of that
is overhead that a smarter compiler could notice was unnecessary. It is the work.

Now here is the part that contradicts the folklore. Looking a method *up* is cheap:

```
1,000,000 getMethod lookups: 20 ms
```

Twenty nanoseconds each, because modern JDKs cache the results. So the cost lives
almost entirely in `invoke`, not in finding the method — though you should still
hoist a `Method` object out of a loop, on general principle.

Whether forty times slower actually matters is Chapter 18's question, and the
answer depends entirely on where it sits. A framework doing reflective work once at
startup — scanning classes, wiring objects together — pays the cost once and nobody
ever notices. A framework doing it once per request, or once per element of a large
collection, has a genuine problem. That is precisely why serialization libraries
generate bytecode at run time instead of reflecting over every field.

## Safety

The bigger cost, and it will never show up in a benchmark.

```java
m = obj.getClass().getMethod("proccess", String.class);   // typo
```

Look at that string. Now consider: that code compiles. The compiler has no opinion
about it whatsoever.

It fails when it runs, with `NoSuchMethodException` — possibly in production,
possibly only on the one code path nobody covered with a test.

Everything the compiler normally does for you is suspended here. Rename a method in
your IDE and every ordinary call site updates automatically; the string inside a
reflective call does not, and nothing warns you. Change a parameter type, same
story. Delete a field that only a serializer ever reads, and the program compiles
perfectly and dies at run time.

This is the real argument against casual reflection, and it is Chapter 17's
argument for generics running backwards. The whole value of static types is that
mistakes surface at the earliest possible moment. Reflection moves them to the
latest possible moment.

## Tooling

A consequence of the above, and it deserves its own heading because people are
caught by it repeatedly.

**Find usages does not find reflective usages.** A method called only through
reflection looks unused to every tool that examines it. Sooner or later somebody
tidying up will delete it, entirely reasonably, and the failure will appear
somewhere unrelated.

**Dead code elimination and obfuscation break** for the same reason. Tools that
strip unreachable code — which matters enormously on Android and in native images —
cannot see a reference that exists only as a string. Which is why every such tool
needs a configuration file telling it what to keep.

**Ahead-of-time compilation** is the sharpest form of the problem. A GraalVM native
image has to know every reachable class at build time, and reflection defeats that
analysis by construction. The entire Java framework ecosystem has spent years now
moving work out of run-time reflection and into compile-time code generation,
largely because of this.

## Encapsulation

Now back to those three demonstrations from the opening, and what they actually
imply.

**Every invariant a class maintains can be broken from outside by anybody willing
to type `setAccessible(true)`.** Not worked around. Not made awkward. Broken, from
outside, by one method call.

That is not a bug in Java. It is a trade the language made deliberately. But it
does mean the guarantees of Chapters 19 and 20 hold **against code that plays by
the rules** — which is all code except reflection, and all the code you should be
writing.

Java 9's module system restored some of it. A module declares which packages are
`open` for reflection, and everything else genuinely cannot be reached. The JDK
closed its own internals this way, and the software that broke as a result was
software that had been reaching into implementation details for years and getting
away with it.

## So when is reflection right?

The rule, stated plainly: **reflection is for code that must work with classes it
has never seen.**

**Frameworks and libraries.** JUnit finding your `@Test` methods. Jackson mapping
your fields to JSON. Spring examining your constructors. A plugin loader. Not one
of these can name your classes, for the excellent reason that your classes did not
exist when they were written.

**Tools.** Debuggers, profilers, IDEs, class-file analyzers. Their subject matter
is code itself.

**Loading by configuration.** A driver named in a properties file; an
implementation chosen at deployment time. `ServiceLoader` is the standard mechanism
and is better than raw `Class.forName`, because what it does is declared rather
than implicit.

And the matching rule for everything else: **if you know the class at compile time,
call the method.**

Reflection reached for to dodge a cast, or to work around a `private` you could
have made package-private, or to pick one of several methods by name, is a design
problem wearing a disguise. The honest alternatives are nearly always an interface,
a functional parameter from Chapter 26, or an enum with per-constant behavior from
Chapter 22.

There is one defensible exception in ordinary application code, and it is testing.
Reaching into a private field to check state, or to inject a stub, is common and
reasonable. Even there, notice what it might be telling you: a class that can
*only* be tested reflectively is usually saying something about its design that is
worth hearing.

Next: the largest thing reflection cannot see.
