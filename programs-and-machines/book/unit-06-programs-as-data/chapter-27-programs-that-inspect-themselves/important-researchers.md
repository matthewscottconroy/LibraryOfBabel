# Important Researchers

**Brian Cantwell Smith** (born 1950) introduced reflection to programming
languages in his 1982 doctoral thesis, with a language called 3-Lisp in which a
program could examine and modify its own evaluation as it ran. The idea was
considerably more radical than what Java has: Smith's programs could reach into
the interpreter's state — the equivalent of Chapter 25's `Env` and the pending
`eval` calls — and change how the remaining computation would proceed. Java's
reflection is the tamed version, reading structure rather than altering execution,
and the taming is what made it practical.

**Pattie Maes** (born 1961) formalized computational reflection in the late 1980s
and coined the distinction between **introspection**, a program examining itself,
and **intercession**, a program modifying itself. Java offers the first and only a
trace of the second — `setAccessible` and dynamic proxies. Maes later became known
for work on software agents and wearable computing, which is a reminder that
careers do not stay in one place.

**Gregor Kiczales** (born 1958) took reflection in a different direction with the
metaobject protocol — the idea that a language's own implementation should be an
object system you can extend, so that a program can change how method dispatch or
object layout work. That work led to aspect-oriented programming and AspectJ,
whose descendants are visible in every framework that transparently adds logging
or transactions to a method you wrote. Spring's proxies are this idea, considerably
simplified.

**Gilad Bracha** (born 1963) and **David Ungar** wrote the paper that named the
design principle Java's reflection follows: mirrors should be separate objects, so
that reflective capability can be withheld from code that should not have it. Java
does this imperfectly — `getClass()` is on `Object`, so every object can reflect —
and Java 9's modules are a partial retrofit of the principle. Bracha was also one
of the designers of Java generics and co-authored the Java Language Specification,
which makes him responsible for both halves of this chapter.

**Philip Wadler** (born 1956) returns from Chapter 26 for the other half. He
co-authored GJ, the proposal that became Java's generics, and with it the
erasure-based implementation. The decision was made under a hard constraint —
existing compiled code had to keep working — and the paper is unusually clear that
erasure was the price of adoption rather than the preferred design. Every
restriction in Section 27.2.1 traces back to that trade.

**Martin Odersky** (born 1958) co-authored GJ with Wadler and wrote the compiler
that became `javac`. He went on to design Scala, in large part to explore what a
JVM language could look like without Java's compatibility constraints — and
Scala's type system is what someone who had lived with erasure would build given a
free hand. It is a useful thing to look at after this chapter, because it shows
which of Java's limits are essential and which are historical.
