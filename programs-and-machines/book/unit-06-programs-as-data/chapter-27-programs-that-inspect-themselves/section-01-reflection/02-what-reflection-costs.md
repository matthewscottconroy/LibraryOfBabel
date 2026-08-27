# What Reflection Costs

Four costs. The first is measurable and the other three are worse.

## Speed

Twenty million method calls — ten million pairs of a mutator and an accessor —
timed directly and through `Method.invoke`, after warm-up:

```
direct    1 ms   reflective   92 ms
direct    1 ms   reflective   64 ms
direct    2 ms   reflective   81 ms
```

Roughly forty to sixty times slower.

The reason is that a direct call is Chapter 21's two indirections and a jump,
inlined away entirely by the JIT once the site proves monomorphic. A reflective
call cannot be: the target is a `Method` object known only at run time, the
arguments must be boxed into an `Object[]`, the return value must be boxed, and
access must be re-checked.

The looking-up is cheaper than people assume:

```
1,000,000 getMethod lookups: 20 ms
```

Twenty nanoseconds each, because modern JDKs cache the results. So the cost is
concentrated in `invoke`, not in finding the method — but the standard advice
still applies, because a `Method` object is worth hoisting out of a loop anyway.

Whether forty times matters is Chapter 18's question. A framework doing reflective
work once at startup — scanning classes, wiring objects — pays it once and nobody
notices. A framework doing it per request, or per element of a large collection,
has a real problem, and this is why serialization libraries generate bytecode at
run time rather than reflecting on every field.

## Safety

The larger cost, and it does not show up in a benchmark.

```java
m = obj.getClass().getMethod("proccess", String.class);   // typo
```

That compiles. It fails when it runs, with `NoSuchMethodException`, possibly in
production, possibly only on the code path nobody tests.

Everything the compiler does for you is suspended. Rename a method in your IDE and
every ordinary call site is updated; the string in a reflective call is not, and
nothing warns you. Change a parameter type and the same. Delete a field that only
a serializer reads and the program compiles cleanly and fails at run time.

This is the real argument against casual reflection, and it is Chapter 17's
argument for generics running in reverse: the value of static types is that
mistakes are found at the earliest possible moment, and reflection moves them to
the latest.

## Tooling

A consequence of the above that deserves separate mention.

**Find usages** does not find reflective ones. A method called only through
reflection looks unused, and someone will delete it.

**Dead code elimination** and **obfuscation** break for the same reason. Tools that
strip unreachable code — which matters greatly on Android and in native images —
cannot see reflective references, which is why such tools need configuration files
listing what to keep.

**Ahead-of-time compilation** is the sharpest version. GraalVM native images must
know every reachable class at build time, and reflection defeats that analysis. The
entire ecosystem of Java frameworks has spent years moving work from run-time
reflection to compile-time code generation, largely for this reason.

## Encapsulation

Section 27.1.1 read a private field, called a private method, and assigned to a
`final` one. Every invariant a class maintains can be broken from outside by
someone willing to type `setAccessible(true)`.

That is not a bug in Java; it is the trade the language made. But it means that
the guarantees of Chapters 19 and 20 hold **against code that plays by the rules**,
which is all code except reflection and all the code you should be writing.

Java 9's modules restored some of it. A module lists which packages are `open` for
reflection, and the rest genuinely cannot be reached. The JDK closed its own
internals this way, and the software that broke was software that had been
reaching into implementation details for years.

## When reflection is right

The rule, stated plainly: **reflection is for code that must work with classes it
has never seen.**

**Frameworks and libraries.** JUnit finding `@Test` methods. Jackson mapping
fields to JSON. Spring examining constructors. A plugin loader. None of these can
name your classes, because your classes did not exist when they were written.

**Tools.** Debuggers, profilers, IDEs, class-file analyzers. Their subject matter
is code.

**Loading by configuration.** A driver named in a properties file, an
implementation chosen at deployment. `ServiceLoader` is the standard mechanism and
is preferable to raw `Class.forName` because it is declared rather than implicit.

And the corresponding rule for everything else: **if you know the class at compile
time, call the method.** Reflection used to avoid a cast, to work around a
`private` you could have made package-private, or to call one of several methods
by name is a design problem being papered over. The alternatives are almost always
an interface, a functional parameter from Chapter 26, or an enum with per-constant
behavior from Chapter 22.

The one honest exception in application code is testing: reaching into a private
field to check state, or to inject a stub, is common and defensible. Even there,
a class that can only be tested reflectively is usually telling you something
about its design.

Next: the largest thing reflection cannot see.
