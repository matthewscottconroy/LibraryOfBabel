# Boxes Around Primitives

`List<int>` does not compile. It is one of the first genuinely arbitrary-looking
refusals a Java beginner meets.

It is not arbitrary, and it is not small: the same decision explains autoboxing,
the traps that come with it, why Chapter 26's stream measurement differs tenfold,
and why one package in the standard library contains forty near-identical
interfaces.

Three lessons on a wrinkle in Java's type system and the trouble it causes.

Java has two kinds of value: primitives, which are the fixed-width boxes of Unit
I, and objects, which live on the heap and are reached by reference. The
collections of Chapter 17 hold objects only. So there must be a way to make an
object out of an `int`, and there is: the **wrapper classes**.

The first lesson is what they are and why they exist. The second is **autoboxing**,
which converts between the two automatically and hides the conversion imperfectly.
The third is `null`, which arrives here because unboxing it is one of the ways it
bites, and which deserves a lesson of its own.
