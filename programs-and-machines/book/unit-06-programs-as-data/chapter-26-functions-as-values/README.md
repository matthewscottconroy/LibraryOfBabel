# Functions as Values

In Chapter 25's language, a procedure was a value. `Procedure(params, body)` was a
record you could store in a map, pass around, and hand to `apply` — a thing, not a
piece of syntax.

Java has this too. It has had it since 2014, and this chapter is about what it
means.

The idea in one sentence: **a piece of behavior can be a value**, so a method can
take one as an argument, return one as a result, or store one in a field, exactly
as it does with an `int` or a `String`.

That sounds modest and it is not. A method that takes a piece of behavior is a
method whose *algorithm* is fixed and whose *detail* is supplied by the caller —
which is a new axis of abstraction. Chapter 11 abstracted over values by naming a
process. This abstracts over the process itself.

Section 26.1 builds it up. Functional interfaces — an interface with one method,
which is what a Java lambda actually is. Lambda expressions and the closures
Chapter 25 said our language lacked. Then higher-order methods: taking behavior,
returning it, and composing it.

Section 26.2 is the vocabulary that comes with it. **Map**, **filter** and
**reduce** are three operations that between them replace most loops you have
written, and they are worth learning as concepts before meeting Java's syntax for
them. Then streams, which is that syntax, and which have real depth — laziness,
short-circuiting, and collectors. Then an honest chapter on when not to use any of
it, with measurements.

This is also where several forward references land. Chapter 14's `() ->` in the
test harness, Chapter 15's stream aside, Chapter 17's `removeIf` and
`Integer::sum`, Chapter 22's promise that anonymous classes have a shorter form —
all of them were this.

One thing to hold on to as the syntax accumulates. Everything in this chapter is a
method call on an object that has one method. The arrows and colons are notation;
underneath, Chapter 22's interfaces are doing all of the work, and Chapter 21's
dynamic dispatch is what makes the call go to the right place. If a piece of
syntax stops making sense, expand it back to an anonymous class and it will.
