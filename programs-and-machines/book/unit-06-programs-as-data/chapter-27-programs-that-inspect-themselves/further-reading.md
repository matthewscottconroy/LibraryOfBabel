# Further Reading

**Joshua Bloch, *Effective Java*, third edition.** Item 65, "Prefer interfaces to
reflection", is Section 27.1.2's argument in two pages and with better examples.
Items 26 through 33 cover generics and erasure — particularly Item 27 on
eliminating unchecked warnings, which is the practical form of the heap-pollution
discussion, and Item 28 on why lists should be preferred to arrays, which turns
entirely on erasure.

**Gilad Bracha and David Ungar, "Mirrors: Design Principles for Meta-level
Facilities of Object-Oriented Programming Languages" (2004).** Short and worth
reading for the argument that reflective capability should be a separate object
that can be withheld. It will make Java 9's modules make sense as a design rather
than as an inconvenience.

**Gilad Bracha, "Generics in the Java Programming Language" (2004).** The tutorial
written when generics shipped. Section 6 onward is the erasure material, stated by
one of the people who chose it, including the migration-compatibility argument in
its original form.

**Brian Cantwell Smith, "Reflection and Semantics in a Procedural Language"
(1982).** The origin. Demanding, and it will change what you think reflection could
be — Java's version is a small corner of what the thesis describes.

**The `java.lang.reflect` package documentation.** Unusually good as reference
material, and the class-level documentation for `Method`, `Field` and
`AccessibleObject` states the access rules precisely. Read `AccessibleObject`
before you rely on `setAccessible` in anything that must run under a module system.

**JEP 396 and the "strong encapsulation" JEPs.** The record of the JDK closing its
own internals to reflection, and the arguments on both sides. A good case study in
how a platform changes something it should not have allowed in the first place.

**Naftalin and Wadler, *Java Generics and Collections*.** More than most people
need, and the right book if erasure's restrictions start biting in real code. The
chapters on wildcards and on the array/list interaction are the ones to read.
